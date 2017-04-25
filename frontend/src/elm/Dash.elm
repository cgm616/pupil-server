module Dash exposing (..)

import Student
import Tutor
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Decode
import Json.Encode
import String


main =
    Html.program { init = init, subscriptions = subscriptions, view = view, update = update }



-- MODEL


type alias Model =
    { currentView : ViewOption
    , studentModel : Student.Model
    , tutorModel : Tutor.Model
    }


type ViewOption
    = StudentView
    | TutorView
    | Choice


empty : Model
empty =
    Model StudentView Student.empty Tutor.empty


init : ( Model, Cmd Msg )
init =
    ( empty, Cmd.none )



-- UPDATE


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ChangeView newView ->
            ( { model | currentView = newView }, Cmd.none )

        StudentMsg subMsg ->
            let
                ( subModel, subCmd ) =
                    Student.update subMsg model.studentModel
            in
                ( { model | studentModel = subModel }, Cmd.none )

        TutorMsg subMsg ->
            let
                ( subModel, subCmd ) =
                    Tutor.update subMsg model.tutorModel
            in
                ( { model | tutorModel = subModel }, Cmd.none )

        Logout ->
            ( model, Cmd.none )


type Msg
    = ChangeView ViewOption
    | StudentMsg Student.Msg
    | TutorMsg Tutor.Msg
    | Logout



-- VIEW


view : Model -> Html Msg
view model =
    div []
        [ section [ class "hero" ]
            [ div [ class "hero-head" ]
                [ div [ class "container" ]
                    [ nav [ class "nav" ]
                        [ div [ class "nav-left" ]
                            [ a [ class "nav-item" ] [ img [ src "/static/img/logo.png" ] [] ] ]
                        , span [ class "nav-toggle" ] []
                        , div [ class "nav-right nav-menu" ]
                            [ div [ class "nav-item" ]
                                [ div
                                    [ class "field is-grouped", style [ ( " margin-bottom", "0" ) ] ]
                                    [ (case model.currentView of
                                        StudentView ->
                                            buttonCons "Switch to Tutor" [ "is-info" ] False (ChangeView TutorView)

                                        TutorView ->
                                            buttonCons "Switch to Student" [ "is-info" ] False (ChangeView StudentView)

                                        Choice ->
                                            div [] []
                                      )
                                    , buttonCons "Log Out" [ "is-danger" ] False Logout
                                    ]
                                ]
                            ]
                        ]
                    ]
                ]
            ]
        , (case model.currentView of
            StudentView ->
                Html.map StudentMsg (Student.view model.studentModel)

            TutorView ->
                Html.map TutorMsg (Tutor.view model.tutorModel)

            Choice ->
                div [] []
          )
        ]


buttonCons : String -> List String -> Bool -> Msg -> Html Msg
buttonCons text_ class_ disabled_ msg =
    p [ class "control is-expanded" ]
        [ button [ class ((String.join " " class_) ++ " button"), onClick msg, disabled disabled_ ]
            [ text text_ ]
        ]



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none
