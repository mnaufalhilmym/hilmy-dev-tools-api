-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "account" (
    "id" uuid DEFAULT gen_random_uuid (),
    "email" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id"),
    UNIQUE ("email")
);