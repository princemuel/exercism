export class Squares {
    #n: number;
    constructor(count: number) {
        this.#n = count;
    }

    get sumOfSquares(): number {
        return (this.sum * (2 * this.#n + 1)) / 3;
    }

    get squareOfSum(): number {
        return this.sum ** 2;
    }

    get difference(): number {
        return Math.abs(this.squareOfSum - this.sumOfSquares);
    }

    get sum(): number {
        return (this.#n * (this.#n + 1)) / 2;
    }
}
