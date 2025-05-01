export function parse(phrase: string): string {
    return phrase
        .split(/[^A-Za-z]+/)
        .flatMap((word) => {
            if (!word) return [];
            if (word === word.toUpperCase()) return [word[0]];
            return [word[0], ...(word.slice(1).match(/[A-Z]/g) || [])];
        })
        .map((letter) => letter.toUpperCase())
        .join("");
}
