export class Squares {
    #n;

    constructor(/**  @type {number} n  */ n) {
        this.#n = n;
    }

    get difference() {
        return Math.abs(this.squareOfSum - this.sumOfSquares);
    }
    get sumOfSquares() {
        return (this.sum * (2 * this.n + 1)) / 3;
    }
    get squareOfSum() {
        return this.sum ** 2;
    }

    get sum() {
        return (this.n * (this.n + 1)) / 2;
    }
    get n() {
        return this.#n;
    }
}
