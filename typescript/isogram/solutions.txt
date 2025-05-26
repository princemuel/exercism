export function isIsogram(text: string): boolean {
    text = text.toLowerCase().replace(/[\s-]+/g, "");
    return text.length === new Set([...text]).size;
}
