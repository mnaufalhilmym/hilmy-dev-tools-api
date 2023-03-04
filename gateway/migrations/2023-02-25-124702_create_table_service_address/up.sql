-- Your SQL goes here
CREATE TYPE "service_address_status" AS ENUM ('accessible', 'inaccessible');

CREATE TABLE IF NOT EXISTS "service_address" (
    "id" uuid DEFAULT gen_random_uuid (),
    "service_id" uuid NOT NULL,
    "address" TEXT NOT NULL,
    "status" service_address_status NOT NULL,
    "last_used_at" TIMESTAMP NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("id"),
    CONSTRAINT "fk_service_info" FOREIGN KEY ("service_id") REFERENCES "service_info" ("id") ON DELETE CASCADE
);