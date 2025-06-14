-- Schema: CREATE TABLE "rna-transcription" ("dna" TEXT, "result" TEXT);
-- Task: update the rna-transcription table and set the result based on the dna field.
UPDATE "rna-transcription"
set result = upper(
        replace(
            replace(
                replace(replace(dna, 'G', 'c'), 'C', 'g'),
                'T',
                'a'
            ),
            'A',
            'u'
        )
    )
