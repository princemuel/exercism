-- Schema: CREATE TABLE "grains" ("task" TEXT, "square" INT, "result" INT);
-- Task: update the grains table and set the result based on the task (and square fields).

UPDATE grains SET result = POWER(2, square - 1) WHERE task = 'single-square';
UPDATE grains SET result = POWER(2, 64) - 1 WHERE task = 'total';
