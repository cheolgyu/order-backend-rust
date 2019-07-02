table! {
    map_product_opt_group (id) {
        id -> Int4,
        shop_id -> Uuid,
        product_id -> Int4,
        opt_group -> Nullable<Array<Int4>>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    option (id) {
        id -> Int4,
        shop_id -> Uuid,
        name -> Varchar,
        price -> Float8,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    option_group (id) {
        id -> Int4,
        shop_id -> Uuid,
        name -> Varchar,
        options -> Array<Int4>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    product (id) {
        id -> Int4,
        shop_id -> Uuid,
        name -> Varchar,
        price -> Float8,
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

allow_tables_to_appear_in_same_query!(
    map_product_opt_group,
    option,
    option_group,
    product,
    shop,
    user,
    valid,
);
