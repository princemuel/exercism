export class BankAccount {
    #balance = 0;
    #open = false;
    #locked = false;
    constructor() {}

    open() {
        if (this.#open) throw new ValueError();
        this.#open = true;
        this.#balance = 0;
    }

    close() {
        if (!this.#open) throw new ValueError();
        this.#open = false;
    }

    /**
     * @param {number} amount
     */
    deposit(amount) {
        this.#transact(() => {
            if (amount < 0) throw new ValueError();
            this.#balance += amount;
        });
    }

    /**
     * @param {number} amount
     */
    withdraw(amount) {
        this.#transact(() => {
            if (amount < 0 || amount > this.#balance) throw new ValueError();
            this.#balance -= amount;
        });
    }

    get balance() {
        if (!this.#open) throw new ValueError();
        return this.#balance;
    }

    /**
     * @param {() => void} operation
     */
    #transact(operation) {
        if (!this.#open || this.#locked) throw new ValueError();
        this.#locked = true;
        try {
            operation();
        } finally {
            this.#locked = false;
        }
    }
}

export class ValueError extends Error {
    constructor(message = "Bank account error") {
        super(message);
    }
}
