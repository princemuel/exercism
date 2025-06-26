export class Matrix {
    #matrix: number[][];
    #columns?: number[][];
    /**
     * Constructs a Matrix from a string representation.
     * The string should contain rows of numbers
     * separated by spaces, with each row on a new line.
     *
     * @param source - A string representation of the matrix.
     */
    constructor(source: string) {
        this.#matrix = source
            .split("\n")
            .map((row) => row.trim().split(/\s+/).map(Number));
    }

    get rows(): number[][] {
        return this.#matrix;
    }

    get columns(): number[][] {
        return (this.#columns ??= this.#computeColumns());
    }

    /**
     * Computes the columns of the matrix.
     * This method is called lazily when the columns are accessed.
     *
     * @returns An array of arrays, where each inner array represents a column.
     */
    #computeColumns(): number[][] {
        if (this.#matrix.length === 0) return [];

        const numCols = this.#matrix[0].length;
        const columns: number[][] = Array.from({ length: numCols }, () => []);

        for (const row of this.#matrix) {
            for (const [idx, value] of row.entries()) columns[idx].push(value);
        }

        return columns;
    }
}
