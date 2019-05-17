table! {
    option (id) {
        id -> Uuid,
        name -> Varchar,
        value_type -> Varchar,
        option_titles -> Nullable<Jsonb>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    option_title (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    product (id) {
        id -> Uuid,
        shop_id -> Uuid,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    shop (id) {
        id -> Uuid,
        ceo_id -> Uuid,
        name -> Varchar,
        products -> Nullable<Jsonb>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    user (id) {
        id -> Uuid,
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

allow_tables_to_appear_in_same_query!(
    option,
    option_title,
    product,
    shop,
    user,
);
