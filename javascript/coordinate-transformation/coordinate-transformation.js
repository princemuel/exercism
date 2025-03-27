// @ts-check
//
// The line above enables type checking for this file. Various IDEs interpret
// the @ts-check directive. It will give you helpful autocompletion when
// implementing this exercise.

/**
 * Create a function that returns a function making use of a closure to
 * perform a repeatable 2d translation of a coordinate pair.
 *
 * @param {number} dx the translate x component
 * @param {number} dy the translate y component
 *
 * @returns {function} a function which takes an x, y parameter, returns the
 *  translated coordinate pair in the form [x, y]
 */
export const translate2d =
    (dx, dy) => (/** @type {number} */ a, /** @type {number} */ b) =>
        [dx + a, dy + b];

/**
 * Create a function that returns a function making use of a closure to
 * perform a repeatable 2d scale of a coordinate pair.
 *
 * @param {number} sx the amount to scale the x component
 * @param {number} sy the amount to scale the y component
 *
 * @returns {function} a function which takes an x, y parameter, returns the
 *  scaled coordinate pair in the form [x, y]
 */
export const scale2d =
    (sx, sy) => (/** @type {number} */ a, /** @type {number} */ b) =>
        [sx * a, sy * b];

/**
 * Create a composition function that returns a function that combines two
 * functions to perform a repeatable transformation
 *
* @template T
* @param {(x: T, y: T) => [T, T]} f The first function to apply.
* @param {(x: T, y: T) => [T, T]} g The second function to apply.
* @returns {(x: T, y: T) => [T, T]} a function which takes an x, y parameter,
   and returns the  transformed coordinate pair in the form [x, y]
 */
export const composeTransform = (f, g) => (x, y) => g(...f(x, y));

/**
 * Return a function that memoizes the last result.  If the arguments are the same as the last call,
 * then memoized result returned.
 *
 * @param {function} f the transformation function to memoize, assumes takes two arguments 'x' and 'y'
 *
 * @returns {function} a function which takes x and y arguments, and will either return the saved result
 *  if the arguments are the same on subsequent calls, or compute a new result if they are different.
 */
export const memoizeTransform = (f) => {
    let a, b, c;
    return (/** @type {number} */ x, /** @type {number} */ y) => {
        if (x === a && y === b) return c;
        const result = f(x, y);
        [a, b, c] = [x, y, result];
        return result;
    };
};
