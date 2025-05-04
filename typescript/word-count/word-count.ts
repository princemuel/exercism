export function count(text: string): Map<string, number> {
    const words = text.toLowerCase().match(/[\w']+/g) ?? [];
    const map = new Map<string, number>();

    for (let word of words) {
        word = word.replace(/^'+|'+$/g, "");
        if (!word) continue;
        const n = map.get(word) ?? 0;
        map.set(word, n + 1);
    }

    return map;
}
