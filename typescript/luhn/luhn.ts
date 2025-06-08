export function valid(input: string): boolean {
    const sanitized = input.replace(/\s/g, "");
    if (sanitized.length < 2 || !/^\d+$/.test(sanitized)) return false;

    let total = 0;
    const length = sanitized.length;
    const parity = length % 2;

    // Process directly with string character codes
    for (let idx = 0; idx < length - 1; idx++) {
        // Convert char to digit: charCode - 48 (ASCII '0')
        const digit = sanitized.charCodeAt(idx) - 48;

        if (idx % 2 !== parity) total += digit;
        else {
            const doubled = digit * 2;
            total += doubled > 9 ? doubled - 9 : doubled;
        }
    }

    const actualCheckDigit = sanitized.charCodeAt(length - 1) - 48;
    const expectedCheckDigit = (10 - (total % 10)) % 10;

    return actualCheckDigit === expectedCheckDigit;
}
