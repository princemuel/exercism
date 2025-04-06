type Sides = [number, number, number];

export class Triangle {
    #sides: Sides;
    #isValid: boolean;

    constructor(...sides: Sides) {
        if (sides.length !== 3)
            throw new Error("Triangle must have exactly 3 sides");
        this.#sides = sides;
        this.#isValid = this.#computeValidity();
    }

    #computeValidity(): boolean {
        return (
            this.#sides.every((s) => s > 0) &&
            this.#sides.reduce((a, b) => a + b, 0) >
                2 * Math.max(...this.#sides)
        );
    }

    #isValidTriangle(): boolean {
        return this.#isValid;
    }

    get isEquilateral(): boolean {
        return this.#isValidTriangle() && new Set(this.#sides).size === 1;
    }

    get isIsosceles(): boolean {
        return this.#isValidTriangle() && new Set(this.#sides).size < 3;
    }

    get isScalene(): boolean {
        return this.#isValidTriangle() && new Set(this.#sides).size === 3;
    }
}
