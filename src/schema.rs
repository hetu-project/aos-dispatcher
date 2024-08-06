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
    questions (request_id) {
        request_id -> Varchar,
        message_id -> Varchar,
        message -> Varchar,
        conversation_id -> Varchar,
        model -> Varchar,
        callback_url -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    opml_answers,
    opml_questions,
    questions,
);
