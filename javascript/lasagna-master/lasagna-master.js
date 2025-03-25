/// <reference path="./global.d.ts" />
// @ts-check

/**
 *
 * @param {number | undefined} remainingTime
 * @returns {string} the cooking status of the lasagna
 */
export const cookingStatus = (remainingTime) => {
    if (Number.isNaN(Number(remainingTime)))
        return "You forgot to set the timer.";
    if (remainingTime === 0) return "Lasagna is done.";
    return "Not done, please wait.";
};

/**
 *
 *  @param {Array<string>} layers
 *  @param {number} averagePrepTimePerLayer
 *  @returns {number | undefined}
 */

export const preparationTime = (layers, averagePrepTimePerLayer = 2) => {
    return layers.length * averagePrepTimePerLayer;
};

/**
 *
 *  @param {Array<string>} layers
 *  @returns {number | undefined}
 */

export const quantities = (layers) => {
    const test = ["noodles", "sauce"];

    const estimate = Object.create({ noodles: 0, sauce: 0 });
    for (const layer of layers) {
        if (test.includes(layer)) {
            const count = estimate[layer] ?? 0;
            estimate[layer] = count + 1;
        }
    }
    estimate.noodles *= 50;
    estimate.sauce *= 0.2;
    return estimate;
};

/**
 *
 *  @param {Array<string>} a
 *  @param {Array<string>} b
 *  @returns void
 */

export const addSecretIngredient = (a, b) => {
    b[b.length] = a[a.length - 1]
};


/**
 * @param {Record<string, number>} recipeObj
 * @param {number} portions
 *
 */

export const scaleRecipe = (recipeObj, portions) => {
    return Object.fromEntries(
        Object.entries(recipeObj).map(([k, v]) => [k, (v * portions) / 2]))
}
