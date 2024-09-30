// @generated automatically by Diesel CLI.

diesel::table! {
    job_request (id) {
        id -> Varchar,
        user -> Varchar,
        job -> Json,
        clock -> Json,
        job_type -> Varchar,
        status -> Varchar,
        tag -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    job_result (id) {
        id -> Varchar,
        job_id -> Varchar,
        operator -> Varchar,
        result -> Json,
        vrf -> Json,
        verify_id -> Varchar,
        tag -> Varchar,
        clock -> Json,
        signature -> Varchar,
        job_type -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    operator (id) {
        id -> Varchar,
        name -> Varchar,
        address -> Varchar,
        start -> Varchar,
        end -> Varchar,
        operator_type -> Varchar,
        status -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    project (id) {
        id -> Varchar,
        name -> Varchar,
        address -> Varchar,
        token -> Varchar,
        status -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user (id) {
        id -> Varchar,
        name -> Varchar,
        address -> Varchar,
        verify_id -> Varchar,
        status -> Varchar,
        tag -> Varchar,
        count -> Int4,
        created_at -> Timestamp,
    }
}

diesel::joinable!(job_result -> job_request (job_id));

diesel::allow_tables_to_appear_in_same_query!(
    job_request,
    job_result,
    operator,
    project,
    user,
);
