export class ValueError extends Error {
    constructor() {
        super("Bank account error");
    }
}

export class BankAccount {
    #balance = 0;
    #open = false;
    #locked = false;
    constructor() {}

    open(): void {
        if (this.#open) throw new ValueError();
        this.#open = true;
        this.#balance = 0;
    }

    close(): void {
        if (!this.#open) throw new ValueError();
        this.#open = false;
    }

    deposit(amount: number): void {
        this.#transact(() => {
            if (!this.#open || amount < 0) throw new ValueError();
            this.#balance += amount;
        });
    }

    withdraw(amount: number): void {
        this.#transact(() => {
            if (!this.#open || amount < 0 || amount > this.#balance)
                throw new ValueError();
            this.#balance -= amount;
        });
    }

    get balance(): number {
        if (!this.#open) throw new ValueError();
        return this.#balance;
    }

    #transact(operation: () => void): void {
        if (this.#locked) throw new ValueError();
        this.#locked = true;
        try {
            operation();
        } finally {
            this.#locked = false;
        }
    }
}
