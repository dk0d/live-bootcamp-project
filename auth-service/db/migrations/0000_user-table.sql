-- Migration: 0000_user-table (up)
-- Created at: 2026-02-15T20:36:50.164135+00:00
-- To snapshot: 0ee3edfc-30c1-4b9b-b77f-179fb20497f9

CREATE TABLE "user" (
  "email" TEXT PRIMARY KEY NOT NULL,
  "password_hash" TEXT NOT NULL,
  "two_factor" TEXT NOT NULL DEFAULT 'none'
);
--> +statement
COMMENT ON TABLE "user" IS 'User accounts';