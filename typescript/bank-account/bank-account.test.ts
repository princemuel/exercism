import { describe, expect, it } from "@jest/globals"
import { BankAccount, ValueError } from "./bank-account.ts"

describe("Bank Account", () => {
    it("newly opened account has zero balance", () => {
        const account = new BankAccount()
        account.open()
        expect(account.balance).toEqual(0)
    })

    it("can deposit money", () => {
        const account = new BankAccount()
        account.open()
        account.deposit(100)
        expect(account.balance).toEqual(100)
    })

    it("can deposit money sequentially", () => {
        const account = new BankAccount()
        account.open()
        account.deposit(100)
        account.deposit(50)
        expect(account.balance).toEqual(150)
    })

    it("can withdraw money", () => {
        const account = new BankAccount()
        account.open()
        account.deposit(100)
        account.withdraw(50)
        expect(account.balance).toEqual(50)
    })

    it("can withdraw money sequentially", () => {
        const account = new BankAccount()
        account.open()
        account.deposit(100)
        account.withdraw(20)
        account.withdraw(80)
        expect(account.balance).toEqual(0)
    })

    it("checking balance of closed account throws error", () => {
        const account = new BankAccount()
        account.open()
        account.close()
        expect(() => account.balance).toThrow(ValueError)
    })

    it("deposit into closed account throws error", () => {
        const account = new BankAccount()
        account.open()
        account.close()
        expect(() => {
            account.deposit(50)
        }).toThrow(ValueError)
    })

    it("withdraw from closed account throws error", () => {
        const account = new BankAccount()
        account.open()
        account.close()
        expect(() => {
            account.withdraw(50)
        }).toThrow(ValueError)
    })

    it("close already closed account throws error", () => {
        const account = new BankAccount()
        expect(() => {
            account.close()
        }).toThrow(ValueError)
    })

    it("open already opened account throws error", () => {
        const account = new BankAccount()
        account.open()
        expect(() => {
            account.open()
        }).toThrow(ValueError)
    })

    it("reopened account does not retain balance", () => {
        const account = new BankAccount()
        account.open()
        account.deposit(50)
        account.close()
        account.open()
        expect(account.balance).toEqual(0)
    })

    it("cannot withdraw more than deposited", () => {
        const account = new BankAccount()
        account.open()
        account.deposit(25)
        expect(() => {
            account.withdraw(50)
        }).toThrow(ValueError)
    })

    it("cannot withdraw negative amount", () => {
        const account = new BankAccount()
        account.open()
        account.deposit(100)
        expect(() => {
            account.withdraw(-50)
        }).toThrow(ValueError)
    })

    it("cannot deposit negative amount", () => {
        const account = new BankAccount()
        account.open()
        expect(() => {
            account.deposit(-50)
        }).toThrow(ValueError)
    })

    it("changing balance directly throws error", () => {
        const account = new BankAccount()
        account.open()
        expect(() => {
            // @ts-expect-error This is supposed to be a read-only property
            account.balance = 100
        }).toThrow(Error)
    })
})
