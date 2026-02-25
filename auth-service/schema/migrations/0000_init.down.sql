-- Migration: 0000_init (down)
-- Created at: 2026-02-25T03:45:42.393313+00:00
-- This migration reverses the changes made by the up migration.

DROP TABLE "two_factor";
--> +statement
DROP TABLE "user";