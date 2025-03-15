// @ts-check

/**
 * Calculates the total bird count.
 *
 * @param {number[]} birdsPerDay
 * @returns {number} total bird count
 */
export const totalBirdCount = (birdsPerDay) =>
  birdsPerDay.reduce((total, count) => total + count, 0);

/**
 * Calculates the total number of birds seen in a specific week.
 *
 * @param {number[]} birdsPerDay
 * @param {number} week
 * @returns {number} birds counted in the given week
 */
export const birdsInWeek = (birdsPerDay, week) =>
  totalBirdCount(birdsPerDay.slice((week - 1) * 7, week * 7));

/**
 * Fixes the counting mistake by increasing the bird count
 * by one for every second day.
 *
 * @param {number[]} birdsPerDay
 * @returns {number[]} corrected bird count data
 */
export const fixBirdCountLog = (birdsPerDay) => {
  for (let idx = 0; idx < birdsPerDay.length; idx += 2) {
    birdsPerDay[idx] += 1;
  }
  return birdsPerDay;
};
