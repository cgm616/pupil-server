table! {
    users {
        id -> Integer,
        name -> VarChar,
        email -> VarChar,
        username -> VarChar,
        pass -> VarChar,
        conf -> Bool,
    }
}

table! {
    conf {
        id -> Integer,
        created -> Timestamptz,
        userid -> Integer,
        username -> VarChar,
        link -> VarChar,
        reset -> Bool,
    }
}
