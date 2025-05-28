/**
 * Checks if a phrase contains every letter of the alphabet (pangram)
 * Uses bit manipulation for O(1) space and fast lookups
 * @param {string} phrase - The phrase to check
 * @returns {boolean} - True if phrase is a pangram, false otherwise
 */
export function isPangram(phrase) {
    // Bitmask: each of 26 bits represents a letter (a=bit0, b=bit1, ..., z=bit25)
    let bitMask = 0;

    for (const char of phrase) {
        // Get character code, default to 0 if undefined (handles edge cases)
        const code = char.codePointAt(0) || 0;

        // Check if character is a letter: A-Z (65-90) or a-z (97-122)
        if ((code >= 65 && code <= 90) || (code >= 97 && code <= 122)) {
            // Convert both cases to same bit position:
            // & 0x1f extracts lower 5 bits, making A(65) and a(97) both map to 1
            // Subtract 1 to get 0-based index (A/a->0, B/b->1, ..., Z/z->25)
            bitMask |= 1 << ((code & 0x1f) - 1);

            // Early exit: 0x3ffffff = 26 ones in binary (all letters found)
            if (bitMask === 0x3ffffff) return true;
        }
    }

    return false;
}
