import { describe, expect, it } from "@jest/globals"
import { steps } from "./collatz-conjecture.ts"

describe("CollatzConjecture", () => {
    it("zero steps for one", () => {
        const expected = 0
        expect(steps(1)).toBe(expected)
    })

    it("divide if even", () => {
        const expected = 4
        expect(steps(16)).toBe(expected)
    })

    it("even and odd steps", () => {
        const expected = 9
        expect(steps(12)).toBe(expected)
    })

    it("Large number of even and odd steps", () => {
        const expected = 152
        expect(steps(1000000)).toBe(expected)
    })

    it("zero is an error", () => {
        const expected = "Only positive integers are allowed"
        expect(() => {
            steps(0)
        }).toThrow(expected)
    })

    it("negative value is an error", () => {
        const expected = "Only positive integers are allowed"
        expect(() => {
            steps(-15)
        }).toThrow(expected)
    })

    it("non-integer value is an error", () => {
        const expected = "Only positive integers are allowed"
        expect(() => {
            steps(3.1415)
        }).toThrow(expected)
    })
})
