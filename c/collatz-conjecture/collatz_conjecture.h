#ifndef COLLATZ_CONJECTURE_H
#define COLLATZ_CONJECTURE_H

/**
 * Error value returned when input is invalid (zero, negative, or causes overflow)
 */
#define ERROR_VALUE (-1)

/**
 * Calculate the number of steps to reach 1 using the Collatz Conjecture rules:
 * - If n is even: n = n / 2
 * - If n is odd: n = 3n + 1
 * - Repeat until n = 1
 *
 * @param start The positive integer to start the sequence from
 * @return Number of steps to reach 1, or ERROR_VALUE if input is invalid
 */
int steps(int start);

#endif /* COLLATZ_CONJECTURE_H */
