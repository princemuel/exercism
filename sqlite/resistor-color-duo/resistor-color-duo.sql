-- Schema: CREATE TABLE "color_code" ("color1" TEXT, "color2" TEXT, "result" INT);
-- Task: update the color_code table and set the result based on the two colors.
WITH colors(name, value) AS (
    VALUES ('black', 0),
        ('brown', 1),
        ('red', 2),
        ('orange', 3),
        ('yellow', 4),
        ('green', 5),
        ('blue', 6),
        ('violet', 7),
        ('grey', 8),
        ('white', 9)
)
UPDATE color_code AS c
SET result = a.value || b.value
FROM colors AS a,
    colors AS b
WHERE a.name = c.color1
    AND b.name = c.color2;
