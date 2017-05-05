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
    { currentView : ViewOption
    , createModel : CreateModel
    }


type alias CreateModel =
    { message : String
    , location : String
    }


type ViewOption
    = Create
    | Existing
    | Profile
    | Settings


empty : Model
empty =
    Model Existing (CreateModel "" "")


init : ( Model, Cmd Msg )
init =
    ( empty, Cmd.none )



-- UPDATE


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ChangeView newView ->
            ( { model | currentView = newView }, Cmd.none )

        UpdateMessage newMessage ->
            let
                createModel =
                    model.createModel
            in
                ( { model | createModel = { createModel | message = newMessage } }, Cmd.none )

        UpdateLocation newLocation ->
            let
                createModel =
                    model.createModel
            in
                ( { model | createModel = { createModel | location = newLocation } }, Cmd.none )


type Msg
    = ChangeView ViewOption
    | UpdateMessage String
    | UpdateLocation String



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
                            [ li [] [ a [ onClick (ChangeView Create) ] [ text "Create Appointment" ] ]
                            , li [] [ a [ onClick (ChangeView Existing) ] [ text "Your Appointments" ] ]
                            , li [] [ a [ onClick (ChangeView Profile) ] [ text "Profile" ] ]
                            , li [] [ a [ onClick (ChangeView Settings) ] [ text "Settings" ] ]
                            ]
                        ]
                    ]
                ]
            ]
        , (case model.currentView of
            Create ->
                create model

            Existing ->
                existing model

            Profile ->
                profile model

            Settings ->
                settings model
          )
        ]


create : Model -> Html Msg
create model =
    div [ class "container" ]
        [ div [ class "field" ]
            [ label [ class "label" ] [ text "Subject" ]
            , p [ class "control" ]
                [ span [ class "select" ]
                    [ select []
                        [ option [] [ text "Select Subject" ]
                        , option [] [ text "Math" ]
                        , option [] [ text "Science" ]
                        ]
                    ]
                ]
            ]
        , div [ class "field" ]
            [ inputCons
                "text"
                "Message"
                "Info that you think the tutor should know"
                []
                False
                model.createModel.message
                UpdateMessage
            ]
        ]


existing : Model -> Html Msg
existing model =
    div [] []


profile : Model -> Html Msg
profile model =
    div [] []


settings : Model -> Html Msg
settings model =
    div [] []


inputCons : String -> String -> String -> List String -> Bool -> String -> (String -> Msg) -> Html Msg
inputCons kind_ name placeholder_ class_ disabled_ value_ msg =
    p [ class ((String.join " " class_) ++ " control") ]
        [ label [ class "label" ] [ text name ]
        , input [ class "input", type_ kind_, placeholder placeholder_, onInput msg, disabled disabled_, value value_ ] []
        ]



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none
