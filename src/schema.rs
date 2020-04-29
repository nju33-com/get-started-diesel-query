table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        nickname -> Nullable<Varchar>,
        age -> Int4,
        active -> Bool,
        created_at -> Timestamp,
    }
}
