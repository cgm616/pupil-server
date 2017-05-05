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
    = Create
    | Existing
    | Profile
    | Settings


empty : Model
empty =
    Model Existing


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
            [ class "hero is-dark" ]
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
    div [] []


existing : Model -> Html Msg
existing model =
    div [] []


profile : Model -> Html Msg
profile model =
    div [] []


settings : Model -> Html Msg
settings model =
    div [] []



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none
