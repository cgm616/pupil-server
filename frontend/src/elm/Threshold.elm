module Threshold exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Decode
import Json.Encode
import String
import Navigation
import Validate exposing (..)


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
    , validation : Validation
    }


type ViewOption
    = Button
    | Choice
    | Existing
    | Register
    | LoggedIn


type alias Validation =
    { name : Maybe ( Bool, String )
    , email : Maybe ( Bool, String )
    , username : Maybe ( Bool, String )
    , password : Maybe ( Bool, String )
    , verifyPassword : Maybe ( Bool, String )
    }


type Field
    = Name
    | Email
    | Username
    | Password
    | VerifyPassword


empty : Model
empty =
    Model
        Button
        ""
        ""
        ""
        ""
        ""
        Nothing
        False
        emptyValidation


emptyValidation =
    Validation Nothing Nothing Nothing Nothing Nothing


init : ( Model, Cmd Msg )
init =
    ( empty, Cmd.none )


validateModel : Model -> ( Model, List Field )
validateModel model =
    let
        list =
            (Validate.all
                [ .name >> ifBlank Name
                , .email >> ifInvalidEmail Email
                , .username >> ifBlank Username
                ]
                model
            )
                ++ (if String.length model.password < 9 then
                        [ Password, VerifyPassword ]
                    else
                        []
                   )
                ++ (if model.verifyPassword == model.password then
                        []
                    else
                        [ VerifyPassword ]
                   )
    in
        ( { model
            | validation =
                Validation
                    (if List.member Name list then
                        Just ( False, "A name must be given." )
                     else
                        Just ( True, "" )
                    )
                    (if List.member Email list then
                        Just ( False, "A valid email must be given." )
                     else
                        Just ( True, "" )
                    )
                    (if List.member Username list then
                        Just ( False, "A username must be given." )
                     else
                        Nothing
                    )
                    (if List.member Password list then
                        Just ( False, "A password longer than 8 characters must be given." )
                     else
                        Just ( True, "" )
                    )
                    (if List.member VerifyPassword list then
                        Just ( False, "The password verification does not match." )
                     else
                        Just ( True, "" )
                    )
          }
        , list
        )


noticeGen : Validation -> ViewOption -> Maybe String
noticeGen { name, email, username, password, verifyPassword } currentView =
    case currentView of
        Existing ->
            Just
                (String.join
                    " "
                    [ Tuple.second (maybeDestruct username)
                    , Tuple.second (maybeDestruct password)
                    ]
                )

        Register ->
            Just
                (String.join
                    " "
                    [ Tuple.second (maybeDestruct name)
                    , Tuple.second (maybeDestruct email)
                    , Tuple.second (maybeDestruct username)
                    , Tuple.second (maybeDestruct password)
                    , Tuple.second (maybeDestruct verifyPassword)
                    ]
                )

        _ ->
            Just ""


maybeDestruct : Maybe ( Bool, String ) -> ( Bool, String )
maybeDestruct choice =
    case choice of
        Nothing ->
            ( False, "" )

        Just something ->
            something



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
            case model.currentView of
                Existing ->
                    let
                        tuple =
                            validateModel model
                    in
                        if
                            tuple
                                |> Tuple.second
                                |> List.filter (\n -> ((n == Username) || (n == Password)))
                                |> List.isEmpty
                        then
                            ( { model | loading = True, notice = Nothing }, submitLogin model )
                        else
                            let
                                newModel =
                                    Tuple.first tuple
                            in
                                ( { newModel | notice = noticeGen newModel.validation newModel.currentView }, Cmd.none )

                Register ->
                    let
                        tuple =
                            validateModel model
                    in
                        if
                            tuple
                                |> Tuple.second
                                |> List.isEmpty
                        then
                            ( { model | loading = True, notice = Nothing }, submitRegister model )
                        else
                            let
                                newModel =
                                    Tuple.first tuple
                            in
                                ( { newModel | notice = noticeGen newModel.validation newModel.currentView }, Cmd.none )

                _ ->
                    ( { model | loading = True }, Cmd.none )

        Response (Ok response) ->
            ( model, Navigation.load response )

        Response (Err error) ->
            ( { empty
                | currentView = model.currentView
                , notice =
                    (case error of
                        Http.BadUrl url ->
                            Just "For some reason the request failed. Please relead and try again."

                        Http.Timeout ->
                            Just "The server is currently overloaded, please try again later."

                        Http.NetworkError ->
                            Just "Check network connection."

                        Http.BadStatus response ->
                            Just response.body

                        Http.BadPayload string response ->
                            Just "Server response was unexpected. Please try again later."
                    )
              }
            , Cmd.none
            )

        Cancel ->
            ( { empty | currentView = Choice }, Cmd.none )


type Msg
    = ChangeView ViewOption
    | UpdatePassword String
    | UpdateVerifyPassword String
    | UpdateUsername String
    | UpdateEmail String
    | UpdateName String
    | Submit
    | Cancel
    | Response (Result Http.Error String)



-- VIEW


view : Model -> Html Msg
view model =
    div []
        [ section [ class "hero is-fullheight is-info" ]
            [ div [ class "hero-head" ]
                [ div [ class "container" ]
                    [ nav [ class "nav" ]
                        [ div [ class "nav-left" ]
                            [ a [ class "nav-item" ] [ img [ src "/static/img/logo.png" ] [] ] ]
                        , span [ class "nav-toggle" ] []
                        , div [ class "nav-right nav-menu" ]
                            [ a [ class "nav-item", href "#what" ] [ text "What" ]
                            , a [ class "nav-item", href "#why" ] [ text "Why" ]
                            , a [ class "nav-item", href "#how" ] [ text "How" ]
                            , a [ class "nav-item", href "#about" ] [ text "About" ]
                            , div [ class "nav-item" ] [ viewSplit model ]
                            ]
                        ]
                    ]
                ]
            , div [ class "hero-body has-text-centered" ]
                [ div [ class "container" ]
                    [ h1 [ class "title is-2" ]
                        [ text "Learn with "
                        , strong [] [ text "Pupil" ]
                        ]
                    , h3 [ class "title is-2" ]
                        [ strong [] [ text "Peer tutoring " ]
                        , text "built to be accessable and useful for "
                        , strong [] [ text "all" ]
                        , text "."
                        ]
                    ]
                ]
            , div [ class "hero-foot has-text-centered" ]
                [ div [ class "container" ]
                    [ span [ id "point-down" ] []
                    , h5 [ class "title is-5" ] [ text "Scroll down" ]
                    ]
                ]
            ]
        , viewSection Left
            "what"
            "What is Pupil?"
            (p [] [ text "Pupil is a new way to augment learning outside of a classroom environment." ])
        , viewSection Right
            "why"
            "Why use Pupil?"
            (p [] [ text "Pupil is a new way to augment learning outside of a classroom environment." ])
        , viewSection Left
            "how"
            "How does Pupil work?"
            (p [] [ text "Pupil is a new way to augment learning outside of a classroom environment." ])
        , viewSection Right
            "about"
            "Who built Pupil?"
            (p [] [ text "Pupil is a new way to augment learning outside of a classroom environment." ])
        ]


viewSplit model =
    case model.currentView of
        Button ->
            viewButton model

        Choice ->
            div []
                [ viewButton model
                , viewModal (viewChoice model) model
                ]

        Existing ->
            div []
                [ viewButton model
                , viewModal (viewExisting model) model
                ]

        Register ->
            div []
                [ viewButton model
                , viewModal (viewRegister model) model
                ]

        LoggedIn ->
            viewLoggedIn model


viewButton model =
    div [ class "field is-grouped", style [ ( "margin-bottom", "0" ) ] ]
        [ buttonCons "Login" [ "is-warning" ] False (ChangeView Existing)
        , buttonCons "Get Started" [ "is-primary" ] False (ChangeView Choice)
        ]


viewChoice model =
    div [ class "has-text-centered animate-fade-in" ]
        [ p [ class "title", style [ ( "color", "#4a4a4a" ) ] ] [ text "Register to get started" ]
        , p [ class "subtitle", style [ ( "color", "#4a4a4a" ) ] ] [ text "Login if you already have an account" ]
        , div [ class "field is-grouped animate-fade-in", style [ ( "margin-bottom", "0" ) ] ]
            [ buttonCons "Login" [ "is-primary", "is-medium", "is-fullwidth" ] False (ChangeView Existing)
            , buttonCons "Register" [ "is-danger", "is-medium", "is-fullwidth" ] False (ChangeView Register)
            ]
        ]


viewExisting model =
    div [ class "animate-fade-in" ]
        [ div [ class "field" ]
            [ inputCons
                "text"
                "Username"
                "Username"
                []
                (case model.validation.username of
                    Nothing ->
                        []

                    Just ( False, _ ) ->
                        [ "is-danger" ]

                    Just ( True, _ ) ->
                        [ "is-success" ]
                )
                model.loading
                model.username
                UpdateUsername
            ]
        , div [ class "field" ]
            [ inputCons "password"
                "Password"
                "Password"
                []
                (case model.validation.password of
                    Nothing ->
                        []

                    Just ( False, _ ) ->
                        [ "is-danger" ]

                    Just ( True, _ ) ->
                        [ "is-success" ]
                )
                model.loading
                model.password
                UpdatePassword
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
            , buttonCons "Cancel" [ "is-danger", "is-fullwidth" ] model.loading Cancel
            ]
        , (case model.notice of
            Nothing ->
                div [] []

            Just message ->
                div [ class "notification is-warning" ]
                    [ text message ]
          )
        ]


viewRegister model =
    div [ class "animate-fade-in" ]
        [ div [ class "field" ]
            [ inputCons "text"
                "Full Name"
                "Full Name"
                []
                (case model.validation.name of
                    Nothing ->
                        []

                    Just ( False, _ ) ->
                        [ "is-danger" ]

                    Just ( True, _ ) ->
                        [ "is-success" ]
                )
                model.loading
                model.name
                UpdateName
            ]
        , div [ class "field is-grouped" ]
            [ inputCons "text"
                "Email"
                "Email"
                [ "is-expanded" ]
                (case model.validation.email of
                    Nothing ->
                        []

                    Just ( False, _ ) ->
                        [ "is-danger" ]

                    Just ( True, _ ) ->
                        [ "is-success" ]
                )
                model.loading
                model.email
                UpdateEmail
            , inputCons "text"
                "Username"
                "Username"
                [ "is-expanded" ]
                (case model.validation.username of
                    Nothing ->
                        []

                    Just ( False, _ ) ->
                        [ "is-danger" ]

                    Just ( True, _ ) ->
                        [ "is-success" ]
                )
                model.loading
                model.username
                UpdateUsername
            ]
        , div [ class "field is-grouped" ]
            [ inputCons "password"
                "Password"
                "Password"
                [ "is-expanded" ]
                (case model.validation.password of
                    Nothing ->
                        []

                    Just ( False, _ ) ->
                        [ "is-danger" ]

                    Just ( True, _ ) ->
                        [ "is-success" ]
                )
                model.loading
                model.password
                UpdatePassword
            , inputCons "password"
                "Verify Password"
                "Verify Password"
                [ "is-expanded" ]
                (case model.validation.verifyPassword of
                    Nothing ->
                        []

                    Just ( False, _ ) ->
                        [ "is-danger" ]

                    Just ( True, _ ) ->
                        [ "is-success" ]
                )
                model.loading
                model.verifyPassword
                UpdateVerifyPassword
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
            , buttonCons "Cancel" [ "is-danger", "is-fullwidth" ] model.loading Cancel
            ]
        , (case model.notice of
            Nothing ->
                div [] []

            Just message ->
                div [ class "notification is-warning" ]
                    [ text message ]
          )
        ]


viewLoggedIn model =
    div [] []


viewModal function model =
    div [ class "modal is-active" ]
        [ div [ class "modal-background", onClick (ChangeView Button) ] []
        , div [ class "modal-content" ]
            [ div [ class "box" ] [ function ] ]
        , div [ class "modal-close", onClick (ChangeView Button) ] []
        ]


type Side
    = Left
    | Right


viewSection contentSide link_ title_ content_ =
    case contentSide of
        Left ->
            section
                [ class "hero is-medium is-dark" ]
                [ a [ name link_ ] []
                , div [ class "hero-body" ]
                    [ div [ class "container" ]
                        [ div [ class "columns" ]
                            [ div [ class "column is-6" ]
                                [ h3 [ class "title" ]
                                    [ text title_ ]
                                , hr [] []
                                , content_
                                ]
                            , div [ class "box column is-5 is-offset-1" ]
                                [ figure [ class "image is-16by9" ]
                                    [ img [ src "http://placehold.it/640x360" ] [] ]
                                ]
                            ]
                        ]
                    ]
                ]

        Right ->
            section
                [ class "hero is-medium is-info" ]
                [ a [ name link_ ] []
                , div [ class "hero-body" ]
                    [ div [ class "container" ]
                        [ div [ class "columns" ]
                            [ div [ class "box column is-5" ]
                                [ figure [ class "image is-16by9" ] [ img [ src "http://placehold.it/640x360" ] [] ] ]
                            , div [ class "column is-6 is-offset-1" ]
                                [ h3 [ class "title" ]
                                    [ text title_ ]
                                , hr [] []
                                , content_
                                ]
                            ]
                        ]
                    ]
                ]


inputCons : String -> String -> String -> List String -> List String -> Bool -> String -> (String -> Msg) -> Html Msg
inputCons kind_ name placeholder_ class_ inputClass_ disabled_ value_ msg =
    p [ class ((String.join " " class_) ++ " control") ]
        [ label [ class "label" ] [ text name ]
        , input [ class ((String.join " " inputClass_) ++ " input"), type_ kind_, placeholder placeholder_, onInput msg, disabled disabled_, value value_ ] []
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



-- HTTP


submitLogin model =
    Http.send
        Response
        (Http.post
            "/login"
            (encodeLogin model.username model.password)
            (Json.Decode.string)
        )


encodeLogin username password =
    Http.jsonBody
        (Json.Encode.object
            [ ( "username", Json.Encode.string username )
            , ( "password", Json.Encode.string password )
            ]
        )


submitRegister model =
    Http.send
        Response
        (Http.post
            "/register"
            (encodeRegister model.name model.email model.username model.password)
            (Json.Decode.string)
        )


encodeRegister name email username password =
    Http.jsonBody
        (Json.Encode.object
            [ ( "name", Json.Encode.string name )
            , ( "email", Json.Encode.string email )
            , ( "username", Json.Encode.string username )
            , ( "password", Json.Encode.string password )
            ]
        )
