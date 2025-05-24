-- Schema: CREATE TABLE "gigasecond" ("moment" TEXT, "result" TEXT);
-- Task: update the gigasecond table and set the result based on the moment.
-- The result should be the moment plus 1 billion seconds.
UPDATE gigasecond
SET result = strftime(
        '%Y-%m-%dT%H:%M:%S',
        datetime(moment, '+1000000000 seconds')
    )
WHERE moment IS NOT NULL;
