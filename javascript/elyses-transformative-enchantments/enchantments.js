// @ts-check

/**
 * Double every card in the deck.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} deck with every card doubled
 */
export const seeingDouble = (deck) => deck.map((card) => card * 2);

/**
 *  Creates triplicates of every 3 found in the deck.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} deck with triplicate 3s
 */
export const threeOfEachThree = (deck) =>
    deck.flatMap((card) => (card === 3 ? [3, 3, 3] : card));

/**
 * Extracts the middle two cards from a deck.
 * Assumes a deck is always 10 cards.
 *
 * @param {number[]} deck of 10 cards
 *
 * @returns {number[]} deck with only two middle cards
 */
export const middleTwo = (deck) => {
    const middleIndex = Math.floor(deck.length / 2);
    return deck.slice(middleIndex - 1, middleIndex + 1);
};

/**
 * Moves the outside two cards to the middle.
 *
 * @param {number[]} deck with even number of cards
 *
 * @returns {number[]} transformed deck
 */

export const sandwichTrick = (deck) => {
    const first = deck[0];
    const last = deck[deck.length - 1];
    const middle = deck.slice(1, -1);
    const middleIndex = Math.floor(middle.length / 2);
    return [
        ...middle.slice(0, middleIndex),
        last,
        first,
        ...middle.slice(middleIndex),
    ];
};

/**
 * Removes every card from the deck except 2s.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} deck with only 2s
 */
export const twoIsSpecial = (deck) => deck.filter((card) => card === 2);

/**
 * Returns a perfectly order deck from lowest to highest.
 *
 * @param {number[]} deck shuffled deck
 *
 * @returns {number[]} ordered deck
 */
export const perfectlyOrdered = (deck) => deck.sort((a, b) => a - b);

/**
 * Reorders the deck so that the top card ends up at the bottom.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} reordered deck
 */
export const reorder = (deck) => deck.reverse();
