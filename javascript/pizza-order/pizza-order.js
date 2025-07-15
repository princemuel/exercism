/// <reference path="./global.d.ts" />
//
// @ts-check

const PIZZA_PRICES = {
    Margherita: 7,
    Caprese: 9,
    Formaggio: 10,
};

const EXTRA_PRICES = {
    ExtraSauce: 1,
    ExtraToppings: 2,
};

/**
 * Determine the price of the pizza given the pizza and optional extras
 *
 * @param {Pizza} pizza name of the pizza to be made
 * @param {Extra[]} extras list of extras
 *
 * @returns {number} the price of the pizza
 */
export const pizzaPrice = (pizza, ...extras) => pizzaPriceInternal(pizza, extras);

/**
 * Determine the price of the pizza given the pizza and optional extras
 *
 * @param {Pizza} pizza name of the pizza to be made
 * @param {Extra[]} extras list of extras
 * @param {number} [index=0] - Current position in the extras array **(internal parameter)**
 *
 * @returns {number} The total price of the pizza including all extras
 *
 * @throws {TypeError} If pizza is not a valid pizza type
 * @throws {TypeError} If any extra is not a valid extra type
 * @throws {RangeError} If recursion depth exceeds JavaScript's call stack limit
 */
function pizzaPriceInternal(pizza, extras, index = 0) {
    if (index >= extras.length) return PIZZA_PRICES[pizza];

    const extra = extras[index];
    const price = EXTRA_PRICES[extra];
    const remaining = pizzaPriceInternal(pizza, extras, index + 1);

    return price + remaining;
}
/**
 * Calculate the price of the total order, given individual orders
 *
 * (HINT: For this exercise, you can take a look at the supplied "global.d.ts" file
 * for a more info about the type definitions used)
 *
 * @param {PizzaOrder[]} pizzaOrders a list of pizza orders
 * @returns {number} the price of the total order
 */
export function orderPrice(pizzaOrders) {
    let total = 0;
    for (const order of pizzaOrders) total += pizzaPrice(order.pizza, ...order.extras);
    return total;
}
