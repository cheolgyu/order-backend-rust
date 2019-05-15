table! {
    user (id) {
        id -> Int4,
        account_id -> Varchar,
        account_password -> Varchar,
        email -> Varchar,
        name -> Varchar,
        role -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}
