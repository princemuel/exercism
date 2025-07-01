// @ts-check

/**
 * Generates a random starship registry number.
 *
 * @returns {string} the generated registry number.
 */
export const randomShipRegistryNumber = () => `NCC-${Math.floor(Math.random() * 9000) + 1000}`;

/**
 * Generates a random stardate.
 *
 * @returns {number} a stardate between 41000 (inclusive) and 42000 (exclusive).
 */
export const randomStardate = () => Math.random() * 1000 + 41000;

/**
 * Generates a random planet class.
 *
 * @returns {string} a one-letter planet class.
 */
export function randomPlanetClass() {
    const classes = "DHJKLMNRTY";
    const randomIdx = Math.floor(Math.random() * classes.length);
    return classes[randomIdx];
}
