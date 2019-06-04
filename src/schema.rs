table! {
    product (id) {
        id -> Int4,
        shop_id -> Uuid,
        name -> Varchar,
        price -> Nullable<Float8>,
        option_group -> Jsonb,
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
        account_password -> Text,
        email -> Varchar,
        valid_email -> Bool,
        phone -> Nullable<Varchar>,
        name -> Varchar,
        role -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    valid (id) {
        id -> Uuid,
        user_id -> Uuid,
        kind -> Varchar,
        kind_value -> Varchar,
        code -> Varchar,
        req -> Nullable<Varchar>,
        res -> Nullable<Varchar>,
        created_at -> Timestamp,
        valid_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(product, shop, user, valid,);
