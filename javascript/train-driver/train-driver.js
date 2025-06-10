// @ts-check
//
// The line above enables type checking for this file. Various IDEs interpret
// the @ts-check directive. It will give you helpful autocompletion when
// implementing this exercise.

/**
 * Return each wagon's id in form of an array.
 *
 * @param {number[]} ids
 * @returns {number[]} wagon ids
 */
export const getListOfWagons = (...ids) => ids;

/**
 * Reorder the array of wagons by moving the first 2 wagons to the end of the array.
 *
 * @param {number[]} ids
 * @returns {number[]} reorderd list of wagons
 */
export const fixListOfWagons = (ids) => [...ids.slice(2), ...ids.slice(0, 2)];

/**
 * Fixes the array of wagons by inserting an array of wagons after the first element in eachWagonsID.
 *
 * @param {number[]} ids
 * @param {number[]} missingWagons
 * @returns {number[]} corrected list of wagons
 */
export function correctListOfWagons(ids, missingWagons) {
    return [...ids.slice(0, 1), ...missingWagons, ...ids.slice(1)];
}

/**
 * Extend route information by adding another object
 *
 * @param {Record<string, string>} information
 * @param {Record<string, string>} additional
 * @returns {Record<string, string>} extended route information
 */
export const extendRouteInformation = (information, additional) => ({
    ...information,
    ...additional,
});

/**
 * Separate arrival time from the route information object
 *
 * @param {Record<string, string>} information
 * @returns {[string, Record<string, string>]} array with arrival time and object without arrival time
 */
export function separateTimeOfArrival(information) {
    const { timeOfArrival, ...rest } = information;
    return [timeOfArrival, rest];
}
