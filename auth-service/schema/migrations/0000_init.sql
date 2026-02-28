-- Migration: 0000_init (up)
-- Created at: 2026-02-28T18:07:48.725522+00:00
-- To snapshot: 3fc41e3f-d9b5-4124-933d-f4d4688f448d

CREATE TABLE "user" (
  "email" TEXT PRIMARY KEY NOT NULL,
  "password_hash" TEXT NOT NULL,
  "two_factor" TEXT NOT NULL DEFAULT 'none'
);
--> +statement
COMMENT ON TABLE "user" IS 'User accounts';