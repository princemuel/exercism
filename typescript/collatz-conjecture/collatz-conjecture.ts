type Thunk<T> = () => T | Thunk<T>;

export function trampoline<T, U extends unknown[]>(
    fn: (...args: U) => T | Thunk<T>,
): (...args: U) => T {
    return (...args: U): T => {
        let result: T | Thunk<T> = fn(...args);
        while (typeof result === "function") result = (result as Thunk<T>)();
        return result;
    };
}

function collatz(n: number, count: number = 0): Thunk<number> {
    if (n <= 0 || !Number.isInteger(n)) {
        throw new TypeError("Only positive integers are allowed");
    }

    if (n === 1) return () => count;

    const next = n % 2 ? 3 * n + 1 : n / 2;
    return () => collatz(next, count + 1);
}

export const steps = trampoline(collatz);
