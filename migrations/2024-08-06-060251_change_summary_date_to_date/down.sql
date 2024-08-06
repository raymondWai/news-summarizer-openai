-- This file should undo anything in `up.sql`
ALTER TABLE summary
ALTER COLUMN date TYPE TIMESTAMP;
