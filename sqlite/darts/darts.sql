-- Schema: CREATE TABLE "darts" ("x" REAL, "y" REAL, score INTEGER);
-- Task: update the darts table and set the score based on the x and y values.
UPDATE darts
SET score = CASE
        WHEN sqrt(x * x + y * y) <= 1 THEN 10 -- Inner circle (bullseye)
        WHEN sqrt(x * x + y * y) <= 5 THEN 5 -- Middle circle
        WHEN sqrt(x * x + y * y) <= 10 THEN 1 -- Outer circle
        ELSE 0 -- Outside target
    END;
