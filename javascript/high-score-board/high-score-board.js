/// <reference path="./global.d.ts" />
// @ts-check

/**
 * Creates a new score board with an initial entry.
 *
 * @returns {Record<string, number>} new score board
 */
export const createScoreBoard = () => ({ "The Best Ever": 1_000_000 });

/**
 * Adds a player to a score board.
 *
 * @param {Record<string, number>} scoreBoard
 * @param {string} player
 * @param {number} score
 * @returns {Record<string, number>} updated score board
 */
export const addPlayer = (scoreBoard, player, score) => {
  scoreBoard[player] = score;
  return scoreBoard;
};

/**
 * Removes a player from a score board.
 *
 * @param {Record<string, number>} scoreBoard
 * @param {string} player
 * @returns {Record<string, number>} updated score board
 */
export const removePlayer = (scoreBoard, player) => {
  delete scoreBoard[player];
  return scoreBoard;
};

/**
 * Increases a player's score by the given amount.
 *
 * @param {Record<string, number>} scoreBoard
 * @param {string} player
 * @param {number} points
 * @returns {Record<string, number>} updated score board
 */
export const updateScore = (scoreBoard, player, points) => {
  scoreBoard[player] += points;
  return scoreBoard;
};

/**
 * Applies 100 bonus points to all players on the board.
 *
 * @param {Record<string, number>} scoreBoard
 * @returns {Record<string, number>} updated score board
 */
export const applyMondayBonus = (scoreBoard) => {
  for (const key in scoreBoard) {
    if (Object.prototype.hasOwnProperty.call(scoreBoard, key)) {
      scoreBoard[key] += 100;
    }
  }

  return scoreBoard;
};

/**
 * Normalizes a score with the provided normalization function.
 *
 * @param {Params} params the parameters for performing the normalization
 * @returns {number} normalized score
 */
export const normalizeScore = (params) => {
  return params.normalizeFunction(params.score);
};
