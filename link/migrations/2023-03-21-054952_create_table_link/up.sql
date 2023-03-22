-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "link" (
    "id" uuid DEFAULT gen_random_uuid (),
    "title" TEXT NOT NULL,
    "short_url" TEXT NOT NULL,
    "long_url" TEXT NOT NULL,
    "visits" INTEGER NOT NULL DEFAULT 0,
    "created_by_id" uuid NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id"),
    UNIQUE ("short_url")
);