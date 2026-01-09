// @generated automatically by Diesel CLI.

diesel::table! {
    contributions (id) {
        id -> Integer,
        member_id -> Integer,
        amount_paid -> Double,
        month_period -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    member (id) {
        id -> Integer,
        f_name -> Text,
        m_name -> Text,
        l_name -> Text,
        email -> Text,
        password -> Text,
        address -> Text,
        age -> Integer,
        is_admin -> Bool,
    }
}

diesel::table! {
    settings (key) {
        key -> Text,
        value -> Text,
    }
}

diesel::joinable!(contributions -> member (member_id));

diesel::allow_tables_to_appear_in_same_query!(contributions, member, settings,);
