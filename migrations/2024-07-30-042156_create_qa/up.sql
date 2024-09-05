-- Your SQL goes here
CREATE TYPE JOB_TYPE AS ENUM ('', 'tee', 'opml');

CREATE TABLE "opml_answers"(
	"req_id" VARCHAR NOT NULL PRIMARY KEY,
	"node_id" VARCHAR NOT NULL,
	"model" VARCHAR NOT NULL,
	"prompt" VARCHAR NOT NULL,
	"answer" VARCHAR NOT NULL,
	"state_root" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL
);

CREATE TABLE "opml_questions"(
	"req_id" VARCHAR NOT NULL PRIMARY KEY,
	"model" VARCHAR NOT NULL,
	"prompt" VARCHAR NOT NULL,
	"callback" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL
);

CREATE TABLE "questions"(
	"request_id" VARCHAR NOT NULL PRIMARY KEY,
	"message_id" VARCHAR NOT NULL,
	"message" VARCHAR NOT NULL,
	"conversation_id" VARCHAR NOT NULL,
	"model" VARCHAR NOT NULL,
	"callback_url" VARCHAR NOT NULL,
	"job_type" VARCHAR NOT NULL,
	"status" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL
);

CREATE TABLE "answers"(
	"request_id" VARCHAR NOT NULL PRIMARY KEY,
	"node_id" VARCHAR NOT NULL,
	"model" VARCHAR NOT NULL,
	"prompt" VARCHAR NOT NULL,
	"answer" VARCHAR NOT NULL,
	"attestation" VARCHAR NOT NULL,
	"attest_signature" VARCHAR NOT NULL,
	"elapsed" INT4 NOT NULL,
	"job_type" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL
);

CREATE TABLE "job_request"(
	"id" VARCHAR NOT NULL PRIMARY KEY,
	"job" JSON NOT NULL,
	"job_type" VARCHAR NOT NULL,
	"status" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL
);

CREATE TABLE "job_result"(
	"id" VARCHAR NOT NULL PRIMARY KEY,
	"job_id" VARCHAR NOT NULL,
	"operator" VARCHAR NOT NULL,
	"result" JSON NOT NULL,
	"signature" VARCHAR NOT NULL,
	"job_type" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL
);


CREATE TABLE "operator"(
	"id" VARCHAR NOT NULL PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"address" VARCHAR NOT NULL,
	"start" VARCHAR NOT NULL,
	"end" VARCHAR NOT NULL,
	"operator_type" VARCHAR NOT NULL,
	"status" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL
);