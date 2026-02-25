-- Migration: 0000_init (up)
-- Created at: 2026-02-25T03:45:42.393210+00:00
-- To snapshot: a089836b-f849-4275-800a-ea0800f56ecc

CREATE TABLE "user" (
  "email" TEXT PRIMARY KEY NOT NULL,
  "password_hash" TEXT NOT NULL,
  "two_factor" TEXT NOT NULL DEFAULT 'none'
);
--> +statement
COMMENT ON TABLE "user" IS 'User accounts';
--> +statement
CREATE TABLE "two_factor" (
  "email" TEXT PRIMARY KEY NOT NULL,
  "id" UUID NOT NULL DEFAULT uuidv7(),
  "code" TEXT NOT NULL
);
--> +statement
COMMENT ON TABLE "two_factor" IS '2FA Codes';