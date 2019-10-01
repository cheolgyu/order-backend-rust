table! {
    fcm (id) {
        id -> Int4,
        to -> Varchar,
        order_id -> Int4,
        order_detail_id -> Int4,
        shop_notification_id -> Int4,
        order_detail_state -> Int4,
        trigger -> Varchar,
        req -> Jsonb,
        resp -> Jsonb,
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
        html_type -> Varchar,
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
        default -> Int4,
        options -> Array<Int4>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    order (id) {
        id -> Int4,
        shop_id -> Uuid,
        state -> Int4,
        price -> Float8,
        products -> Jsonb,
        sw_token -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    order_detail (id) {
        id -> Int4,
        order_id -> Int4,
        shop_id -> Uuid,
        state -> Int4,
        txt -> Jsonb,
        req_session_id -> Jsonb,
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
        opt_group -> Array<Int4>,
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
        notification_key -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    shop_notification (id) {
        id -> Int4,
        shop_id -> Uuid,
        interval -> Int4,
        content -> Varchar,
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
    user_device (id) {
        id -> Int4,
        user_id -> Uuid,
        name -> Varchar,
        sw_token -> Varchar,
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
    fcm,
    option,
    option_group,
    order,
    order_detail,
    product,
    shop,
    shop_notification,
    user,
    user_device,
    valid,
);
