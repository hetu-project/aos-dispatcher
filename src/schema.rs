// @generated automatically by Diesel CLI.

diesel::table! {
    answers (request_id) {
        request_id -> Varchar,
        node_id -> Varchar,
        model -> Varchar,
        prompt -> Varchar,
        answer -> Varchar,
        attestation -> Varchar,
        attest_signature -> Varchar,
        elapsed -> Int4,
        job_type -> Varchar,
        created_at -> Timestamp,
    }
}

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
    opml_answers (req_id) {
        req_id -> Varchar,
        node_id -> Varchar,
        model -> Varchar,
        prompt -> Varchar,
        answer -> Varchar,
        state_root -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    opml_questions (req_id) {
        req_id -> Varchar,
        model -> Varchar,
        prompt -> Varchar,
        callback -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    project (id) {
        id -> Varchar,
        name -> Varchar,
        address -> Varchar,
        status -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    questions (request_id) {
        request_id -> Varchar,
        message_id -> Varchar,
        message -> Varchar,
        conversation_id -> Varchar,
        model -> Varchar,
        callback_url -> Varchar,
        job_type -> Varchar,
        status -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user (id) {
        id -> Varchar,
        name -> Varchar,
        address -> Varchar,
        status -> Varchar,
        tag -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(job_result -> job_request (job_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    job_request,
    job_result,
    operator,
    opml_answers,
    opml_questions,
    project,
    questions,
    user,
);
