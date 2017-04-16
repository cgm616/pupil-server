module Threshold exposing (..)

import Html exposing (Html, button, div, text, p, input, label)
import Html.Attributes exposing (..)
import Html.Events exposing (..)


main =
    Html.beginnerProgram { model = model, view = view, update = update }



-- MODEL


type Model
    = Choice
    | Existing
    | Register
    | LoggedIn


model : Model
model =
    Choice



-- UPDATE


update msg model =
    case msg of
        ChangeView view ->
            view


type Msg model
    = ChangeView model



-- VIEW


view model =
    case model of
        Choice ->
            div [ class "columns" ]
                [ button
                    [ class "column is- button is-primary is-medium", onClick (ChangeView Existing) ]
                    [ text "Login" ]
                , button
                    [ class "column is-half button is-danger is-medium", onClick (ChangeView Register) ]
                    [ text "Register" ]
                ]

        Existing ->
            div []
                [ div
                    [ class "field" ]
                    [ label [ class "label " ] [ text "Username" ]
                    , p [ class "control" ]
                        [ input [ class "input", type_ "text", placeholder "Username" ] [] ]
                    ]
                , div
                    [ class "field" ]
                    [ label [ class "label" ] [ text "Password" ]
                    , p [ class "control" ]
                        [ input [ class "input", type_ "password", placeholder "Password" ] [] ]
                    ]
                , div
                    [ class "field" ]
                    [ p [ class "control" ]
                        [ button
                            [ class "is-primary" ]
                            [ text "Login" ]
                        ]
                    ]
                ]

        Register ->
            div [] []

        LoggedIn ->
            div [] []
