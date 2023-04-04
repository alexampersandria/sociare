// @generated automatically by Diesel CLI.

diesel::table! {
    debts (id) {
        id -> Varchar,
        group_id -> Varchar,
        from_id -> Varchar,
        to_id -> Varchar,
        amount -> Int8,
        created_at -> Int8,
    }
}

diesel::table! {
    groups (id) {
        id -> Varchar,
        name -> Varchar,
        emoji -> Varchar,
        currency -> Varchar,
        created_at -> Int8,
    }
}

diesel::table! {
    messages (id) {
        id -> Varchar,
        group_id -> Varchar,
        user_id -> Varchar,
        content -> Text,
        created_at -> Int8,
        deleted -> Bool,
    }
}

diesel::table! {
    receipts (id) {
        id -> Varchar,
        group_id -> Varchar,
        user_id -> Varchar,
        amount -> Int8,
        info -> Text,
        created_at -> Int8,
        deleted -> Bool,
    }
}

diesel::table! {
    transactions (id) {
        id -> Varchar,
        group_id -> Varchar,
        from_id -> Varchar,
        to_id -> Varchar,
        debt_id -> Varchar,
        amount -> Int8,
        method -> Text,
        confirmed -> Bool,
        deleted -> Bool,
        created_at -> Int8,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        password -> Varchar,
        created_at -> Int8,
    }
}

diesel::table! {
    users_groups (id) {
        id -> Varchar,
        user_id -> Varchar,
        group_id -> Varchar,
        nickname -> Varchar,
        is_admin -> Bool,
        active -> Bool,
        created_at -> Int8,
    }
}

diesel::joinable!(debts -> groups (group_id));
diesel::joinable!(messages -> groups (group_id));
diesel::joinable!(messages -> users (user_id));
diesel::joinable!(receipts -> groups (group_id));
diesel::joinable!(receipts -> users (user_id));
diesel::joinable!(transactions -> debts (debt_id));
diesel::joinable!(transactions -> groups (group_id));
diesel::joinable!(users_groups -> groups (group_id));
diesel::joinable!(users_groups -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
  debts,
  groups,
  messages,
  receipts,
  transactions,
  users,
  users_groups,
);
