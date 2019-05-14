table! {
    user (id) {
        id -> Int4,
        uuid -> Uuid,
        account_id -> Varchar,
        account_password -> Varchar,
        name -> Varchar,
        role -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}
