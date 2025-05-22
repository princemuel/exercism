export const toRna = (strand: string): string => {
    const complements = new Map([
        ["G", "C"],
        ["C", "G"],
        ["T", "A"],
        ["A", "U"],
    ]);

    return [...strand]
        .map((nucleotide) => {
            const rna = complements.get(nucleotide);
            if (!rna) throw new Error("Invalid input DNA.");
            return rna;
        })
        .join("");
};
