module Threshold exposing (..)

import Html exposing (Html, button, div, text, p, input, label)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import String


main =
    Html.program { init = init, subscriptions = subscriptions, view = view, update = update }



-- MODEL


type alias Model =
    { currentView : ViewOption
    , password : String
    , verifyPassword : String
    , username : String
    , email : String
    , name : String
    , notice : Maybe String
    , loading : Bool
    }


type ViewOption
    = Choice
    | Existing
    | Register
    | LoggedIn


init : ( Model, Cmd Msg )
init =
    ( Model Choice "" "" "" "" "" Nothing False, Cmd.none )



-- UPDATE


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ChangeView newView ->
            ( { model | currentView = newView }, Cmd.none )

        UpdatePassword newPassword ->
            ( { model | password = newPassword }, Cmd.none )

        UpdateVerifyPassword newVerifyPassword ->
            ( { model | verifyPassword = newVerifyPassword }, Cmd.none )

        UpdateUsername newUsername ->
            ( { model | username = newUsername }, Cmd.none )

        UpdateEmail newEmail ->
            ( { model | email = newEmail }, Cmd.none )

        UpdateName newName ->
            ( { model | name = newName }, Cmd.none )

        Submit ->
            ( { model | loading = True }, Cmd.none )

        Response newNotice ->
            ( { model | notice = Just newNotice }, Cmd.none )

        Cancel ->
            init


type Msg
    = ChangeView ViewOption
    | UpdatePassword String
    | UpdateVerifyPassword String
    | UpdateUsername String
    | UpdateEmail String
    | UpdateName String
    | Submit
    | Cancel
    | Response String



-- VIEW


view : Model -> Html Msg
view model =
    case model.currentView of
        Choice ->
            div [ class "field is-grouped" ]
                [ div []
                    [ buttonCons "Login" [ "is-primary", "is-medium" ] False (ChangeView Existing) ]
                , div []
                    [ buttonCons "Register" [ "is-danger", "is-medium" ] False (ChangeView Register) ]
                ]

        Existing ->
            div []
                [ inputCons "text" "Username" "Username" [ "field" ] UpdateUsername
                , inputCons "password" "Password" "Password" [ "field" ] UpdatePassword
                , div [ class "field is-grouped" ]
                    [ buttonCons
                        "Submit"
                        (if model.loading then
                            [ "is-primary", "is-loading" ]
                         else
                            [ "is-primary" ]
                        )
                        False
                        Submit
                    , buttonCons
                        "Cancel"
                        [ "is-danger" ]
                        (if model.loading then
                            True
                         else
                            False
                        )
                        Cancel
                    ]
                ]

        Register ->
            div []
                [ inputCons "text" "Full Name" "Full Name" [ "field" ] UpdateName
                , div [ class "field is-grouped" ]
                    [ p [ class "control is-expanded" ]
                        [ label [ class "label" ] [ text "Email" ]
                        , input [ class "input", type_ "text", placeholder "Email", onInput UpdateEmail ] []
                        ]
                    , p [ class "control is-expanded" ]
                        [ label [ class "label" ] [ text "Username" ]
                        , input [ class "input", type_ "text", placeholder "Email", onInput UpdateUsername ] []
                        ]
                    ]
                , div [ class "field is-grouped" ]
                    [ p [ class "control is-expanded" ]
                        [ label [ class "label" ] [ text "Password" ]
                        , input [ class "input", type_ "password", placeholder "Password", onInput UpdatePassword ] []
                        ]
                    , p [ class "control is-expanded" ]
                        [ label [ class "label" ] [ text "Verify Password" ]
                        , input [ class "input", type_ "password", placeholder "Verify Password", onInput UpdateVerifyPassword ] []
                        ]
                    ]
                , div [ class "field is-grouped" ]
                    [ buttonCons
                        "Submit"
                        (if model.loading then
                            [ "is-primary", "is-loading" ]
                         else
                            [ "is-primary" ]
                        )
                        False
                        Submit
                    , buttonCons
                        "Cancel"
                        [ "is-danger" ]
                        (if model.loading then
                            True
                         else
                            False
                        )
                        Cancel
                    ]
                ]

        LoggedIn ->
            div [] []


inputCons : String -> String -> String -> List String -> (String -> Msg) -> Html Msg
inputCons kind_ name placeholder_ class_ msg =
    div [ class (String.join " " class_) ]
        [ p [ class "control" ]
            [ label [ class "label" ] [ text name ]
            , input [ class "input", type_ kind_, placeholder placeholder_, onInput msg ] []
            ]
        ]


buttonCons : String -> List String -> Bool -> Msg -> Html Msg
buttonCons text_ class_ disabled_ msg =
    p [ class "control" ]
        [ button [ class ((String.join " " class_) ++ " button"), onClick msg, disabled disabled_ ]
            [ text text_ ]
        ]



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none
