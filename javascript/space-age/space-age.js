const planets = new Map([
    ["mercury", 0.2408467],
    ["venus", 0.61519726],
    ["earth", 1.0],
    ["mars", 1.8808158],
    ["jupiter", 11.862615],
    ["saturn", 29.447498],
    ["uranus", 84.016846],
    ["neptune", 164.79132],
]);
const EARTH_YEAR_IN_SECS = 365.25 * 24.0 * 60.0 * 60.0;

/**
 * @param {string} planet
 * @param {number} seconds
 */
export function age(planet, seconds) {
    const orbital_period = planets.get(planet.toLowerCase()) ?? 1.0;
    return Number((seconds / (EARTH_YEAR_IN_SECS * orbital_period)).toFixed(2));
}
