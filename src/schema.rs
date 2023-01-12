// @generated automatically by Diesel CLI.

diesel::table! {
    characters (id) {
        id -> Int4,
        vida -> Int4,
        mana -> Int4,
        danio -> Int4,
        oro -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        nombre -> Varchar,
        pass -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    characters,
    users,
);
