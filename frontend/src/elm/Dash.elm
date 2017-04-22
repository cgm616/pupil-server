module Dash exposing (..)

import Student
import Tutor
import Html exposing (Html, button, div, text, p, input, label)
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
    = StudentView Student.Model
    | TutorView Tutor.Model
    | Choice


empty : Model
empty =
    Model Choice Student.empty Tutor.empty


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
    div [ class "coolstuff" ]
        [ p [] [ text "Hi!" ] ]



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none
