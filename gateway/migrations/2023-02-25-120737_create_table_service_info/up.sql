-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "service_info" (
    "id" uuid DEFAULT gen_random_uuid (),
    "name" TEXT NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id"),
    UNIQUE ("name")
);