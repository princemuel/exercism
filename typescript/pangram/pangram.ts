export const isPangram = (input: string): boolean =>
    new Set(input.toLowerCase().match(/[a-z]/g)).size === 26;
