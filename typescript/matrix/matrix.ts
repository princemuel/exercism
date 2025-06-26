export class Matrix {
    #rows: number[][];
    #columns?: number[][];
    /**
     * Constructs a Matrix from a string representation.
     * The string should contain rows of numbers
     * separated by spaces, with each row on a new line.
     *
     * @param source - A string representation of the matrix.
     */
    constructor(source: string) {
        this.#rows = source
            .split("\n")
            .map((row) => row.trim().split(/\s+/).map(Number));
    }

    get rows(): number[][] {
        return this.#rows;
    }

    get columns(): number[][] {
        return (this.#columns ??= this.#compute_cols());
    }

    #compute_cols(): number[][] {
        if (this.#rows.length === 0) return [];
        const numCols = this.#rows[0].length;

        const cols: number[][] = Array.from({ length: numCols }, () => []);

        for (let rowIdx = 0; rowIdx < this.#rows.length; rowIdx++) {
            for (let colIdx = 0; colIdx < numCols; colIdx++) {
                cols[colIdx].push(this.#rows[rowIdx][colIdx]);
            }
        }

        return cols;
    }
}
