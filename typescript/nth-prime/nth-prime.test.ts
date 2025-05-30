import { describe, expect, it } from "@jest/globals"
import { nth } from "./nth-prime.ts"

describe("Prime", () => {
    it("first", () => {
        expect(nth(1)).toEqual(2)
    })

    it("second", () => {
        expect(nth(2)).toEqual(3)
    })

    it("sixth", () => {
        expect(nth(6)).toEqual(13)
    })

    it("big prime", () => {
        expect(nth(10001)).toEqual(104743)
    })

    it("weird case", () => {
        expect(() => nth(0)).toThrow("Prime is not possible")
    })
})
