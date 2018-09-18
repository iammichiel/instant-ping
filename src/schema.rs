table! {
    certificates (id_certificate) {
        id_certificate -> Varchar,
        domain -> Varchar,
        last_check -> Nullable<Datetime>,
        dashboard_id -> Varchar,
    }
}

table! {
    dashboards (id_dashboard) {
        id_dashboard -> Varchar,
        last_access -> Nullable<Datetime>,
    }
}

joinable!(certificates -> dashboards (dashboard_id));

allow_tables_to_appear_in_same_query!(
    certificates,
    dashboards,
);
