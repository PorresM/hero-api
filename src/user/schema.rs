table! {
    user (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        roles -> Array<Text>,
        created -> Timestamp,
        modified -> Timestamp,
    }
}
