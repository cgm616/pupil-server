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
    div [ class "columns" ]
        [ div [ class "column is-half is-offset-one-quarter box has-shadow" ]
            [ case model.currentView of
                Choice ->
                    viewChoice model

                Existing ->
                    viewExisting model

                Register ->
                    viewRegister model

                LoggedIn ->
                    viewLoggedIn model
            ]
        ]


viewChoice model =
    div [ class "field is-grouped animate-fade-in", style [ ( "margin-bottom", "0" ) ] ]
        [ buttonCons "Login" [ "is-primary", "is-medium", "is-fullwidth" ] False (ChangeView Existing)
        , buttonCons "Register" [ "is-danger", "is-medium", "is-fullwidth" ] False (ChangeView Register)
        ]


viewExisting model =
    div [ class "animate-fade-in" ]
        [ div [ class "field" ]
            [ inputCons "text" "Username" "Username" [] UpdateUsername ]
        , div [ class "field" ]
            [ inputCons "password" "Password" "Password" [] UpdatePassword ]
        , div [ class "field is-grouped" ]
            [ buttonCons
                "Submit"
                (if model.loading then
                    [ "is-primary", "is-loading", "is-fullwidth" ]
                 else
                    [ "is-primary", "is-fullwidth" ]
                )
                False
                Submit
            , buttonCons
                "Cancel"
                [ "is-danger", "is-fullwidth" ]
                (if model.loading then
                    True
                 else
                    False
                )
                Cancel
            ]
        ]


viewRegister model =
    div [ class "animate-fade-in" ]
        [ div [ class "field" ]
            [ inputCons "text" "Full Name" "Full Name" [] UpdateName ]
        , div [ class "field is-grouped" ]
            [ inputCons "text" "Email" "Email" [ "is-expanded" ] UpdateEmail
            , inputCons "text" "Username" "Username" [ "is-expanded" ] UpdateUsername
            ]
        , div [ class "field is-grouped" ]
            [ inputCons "password" "Password" "Password" [ "is-expanded" ] UpdatePassword
            , inputCons "password" "Verify Password" "Verify Password" [ "is-expanded" ] UpdateVerifyPassword
            ]
        , div [ class "field is-grouped" ]
            [ buttonCons
                "Submit"
                (if model.loading then
                    [ "is-primary", "is-loading", "is-fullwidth" ]
                 else
                    [ "is-primary", "is-fullwidth" ]
                )
                False
                Submit
            , buttonCons
                "Cancel"
                [ "is-danger", "is-fullwidth" ]
                (if model.loading then
                    True
                 else
                    False
                )
                Cancel
            ]
        ]


viewLoggedIn model =
    div [ class "animate-fade-in" ] []


inputCons : String -> String -> String -> List String -> (String -> Msg) -> Html Msg
inputCons kind_ name placeholder_ class_ msg =
    p [ class ((String.join " " class_) ++ " control") ]
        [ label [ class "label" ] [ text name ]
        , input [ class "input", type_ kind_, placeholder placeholder_, onInput msg ] []
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
