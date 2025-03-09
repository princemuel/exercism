// @ts-check

/**
 * The fast attack is available when the knight is sleeping
 *
 * @param {boolean} knightIsAwake
 *
 * @return {boolean} Whether or not you can execute a fast attack.
 */
export const canExecuteFastAttack = (knightIsAwake) => !knightIsAwake;

/**
 * A useful spy captures information, which they can't do if everyone's asleep.
 *
 * @param {boolean} knightIsAwake
 * @param {boolean} archerIsAwake
 * @param {boolean} prisonerIsAwake
 *
 * @returns {boolean} Whether or not you can spy on someone.
 */
export const canSpy = (knightIsAwake, archerIsAwake, prisonerIsAwake) =>
  knightIsAwake || archerIsAwake || prisonerIsAwake;

/**
 * You'll get caught by the archer if you signal while they're awake.
 *
 * @param {boolean} archerIsAwake
 * @param {boolean} prisonerIsAwake
 *
 * @returns {boolean} Whether or not you can send a signal to the prisoner.
 */
export const canSignalPrisoner = (archerIsAwake, prisonerIsAwake) =>
  prisonerIsAwake && !archerIsAwake;

/**
 * The final stage in the plan: freeing Annalyn's best friend.
 *
 * @param {boolean} knightIsAwake
 * @param {boolean} archerIsAwake
 * @param {boolean} prisonerIsAwake
 * @param {boolean} petDogIsPresent
 *
 * @returns {boolean} Whether or not you can free Annalyn's friend.
 */
export const canFreePrisoner = (
  knightIsAwake,
  archerIsAwake,
  prisonerIsAwake,
  petDogIsPresent
) => {
  return (
    (petDogIsPresent && !archerIsAwake) ||
    (!petDogIsPresent && prisonerIsAwake && !archerIsAwake && !knightIsAwake)
  );
};
