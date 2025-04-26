export function isIsogram(text: string): boolean {
    return !text.match(/([a-z]).*\1/i);
}
