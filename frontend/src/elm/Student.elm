module Student exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Decode
import Json.Encode
import String


-- MODEL


type alias Model =
    { currentView : ViewOption }


type ViewOption
    = Choice


empty : Model
empty =
    Model Choice


init : ( Model, Cmd Msg )
init =
    ( empty, Cmd.none )



-- UPDATE


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ChangeView newView ->
            ( { model | currentView = newView }, Cmd.none )


type Msg
    = ChangeView ViewOption



-- VIEW


view : Model -> Html Msg
view model =
    div []
        [ section
            [ class "hero is-info" ]
            [ div [ class "hero-body" ]
                [ div [ class "container" ]
                    [ h1 [ class "title is-1" ]
                        [ text "Welcome! You are currently a"
                        , strong [] [ text " Student " ]
                        ]
                    , h3 [ class "subtitle is-3" ] [ text "Get started learning today!" ]
                    ]
                ]
            , div [ class "hero-foot" ]
                [ div [ class "container" ]
                    [ div [ class "tabs is-boxed" ]
                        [ ul []
                            [ li [] [ a [] [ text " Create Appointment " ] ]
                            , li [] [ a [] [ text " Your Appointments " ] ]
                            , li [] [ a [] [ text " Profile " ] ]
                            , li [] [ a [] [ text " Settings " ] ]
                            ]
                        ]
                    ]
                ]
            ]
        ]



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none
