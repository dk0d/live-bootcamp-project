-- Migration: 0001_remove-2fa-for-redis (down)
-- Created at: 2026-02-25T03:46:00.316780+00:00
-- This migration reverses the changes made by the up migration.

CREATE TABLE "two_factor" (
  "email" TEXT PRIMARY KEY NOT NULL,
  "id" UUID NOT NULL DEFAULT uuidv7(),
  "code" TEXT NOT NULL
);
--> +statement
COMMENT ON TABLE "two_factor" IS '2FA Codes';