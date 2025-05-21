-- Schema: CREATE TABLE "leap" ( "year" INT, "is_leap" BOOL);
-- Task: update the leap table and set the is_leap based on the year field.
UPDATE leap
SET is_leap = NOT year % 4
    AND (
        year % 100
        OR NOT year % 400
    )
