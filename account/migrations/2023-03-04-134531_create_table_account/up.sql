-- Your SQL goes here
CREATE TYPE "account_role" AS ENUM ('user', 'admin');

CREATE TABLE IF NOT EXISTS "account" (
    "id" uuid DEFAULT gen_random_uuid (),
    "email" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    "role" account_role NOT NULL DEFAULT 'user',
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id"),
    UNIQUE ("email")
);