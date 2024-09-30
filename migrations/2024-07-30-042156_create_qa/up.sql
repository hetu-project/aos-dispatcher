-- Your SQL goes here
-- CREATE DATABASE dispatcher

-- CREATE TYPE JOB_TYPE AS ENUM ('', 'tee', 'opml');

CREATE TABLE "job_request"(
	"id" VARCHAR NOT NULL PRIMARY KEY,
	"user" VARCHAR NOT NULL,
	"job" JSON NOT NULL,
	"clock" JSON NOT NULL,
	"job_type" VARCHAR NOT NULL,
	"status" VARCHAR NOT NULL,
	"tag" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL
);

CREATE TABLE "job_result"(
	"id" VARCHAR NOT NULL PRIMARY KEY,
	"job_id" VARCHAR NOT NULL REFERENCES job_request(id),
	"operator" VARCHAR NOT NULL,
	"result" JSON NOT NULL,
	"vrf" JSON NOT NULL,
	"verify_id" VARCHAR NOT NULL,
	"tag" VARCHAR NOT NULL,
	"clock" JSON NOT NULL,
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

CREATE TABLE "project"(
	"id" VARCHAR NOT NULL PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"address" VARCHAR NOT NULL,
	"token" VARCHAR NOT NULL,
	"status" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL
);


CREATE TABLE "user"(
	"id" VARCHAR NOT NULL PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"address" VARCHAR NOT NULL,
	"verify_id" VARCHAR NOT NULL,
	"status" VARCHAR NOT NULL,
	"tag" VARCHAR NOT NULL,
	"count" INTEGER NOT NULL,
	"created_at" TIMESTAMP NOT NULL
);