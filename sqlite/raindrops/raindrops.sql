-- Schema: CREATE TABLE "raindrops" ("number" INT, "sound" TEXT);
-- Task: update the raindrops table and set the sound based on the number.
-- Schema: CREATE TABLE "raindrops" ("number" INT, "sound" TEXT);
-- Task: update the raindrops table and set the sound based on the number.
UPDATE raindrops
SET sound = iif(number % 3, '', 'Pling') || iif(number % 5, '', 'Plang') || iif(number % 7, '', 'Plong');
UPDATE raindrops
SET sound = iif(sound == '', number, sound);
