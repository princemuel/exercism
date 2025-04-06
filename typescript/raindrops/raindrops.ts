export const convert = (n: number): string => {
    const sounds = new Map([
        [3, "Pling"],
        [5, "Plang"],
        [7, "Plong"],
    ]);

    let result = "";
    for (const [k, v] of sounds) if (n % k === 0) result += v;
    return result || String(n);
};
