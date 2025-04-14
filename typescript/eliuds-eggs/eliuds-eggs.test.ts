import { describe, expect, it } from "@jest/globals"
import { eggCount } from "./eliuds-eggs.ts"

describe("EliudsEggs", () => {
    it("0 eggs", () => {
        const expected = 0
        const actual = eggCount(0)
        expect(actual).toEqual(expected)
    })

    it("1 egg", () => {
        const expected = 1
        const actual = eggCount(16)
        expect(actual).toEqual(expected)
    })

    it("4 eggs", () => {
        const expected = 4
        const actual = eggCount(89)
        expect(actual).toEqual(expected)
    })

    it("13 eggs", () => {
        const expected = 13
        const actual = eggCount(2000000000)
        expect(actual).toEqual(expected)
    })
})
