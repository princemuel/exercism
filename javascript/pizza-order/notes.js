/// <reference path="./global.d.ts" />
//
// @ts-check

/**
 * Price mappings for different pizza types
 * @constant {Object.<string, number>}
 */
const PIZZA_PRICES = {
    Margherita: 7,
    Caprese: 9,
    Formaggio: 10,
};

/**
 * Price mappings for extra options
 * @constant {Object.<string, number>}
 */
const EXTRA_PRICES = {
    ExtraSauce: 1,
    ExtraToppings: 2,
};

/**
 * Calculates the total price of a pizza with extras using optimized recursion.
 *
 * This function demonstrates a space-efficient recursive approach that avoids
 * the O(n²) time complexity of array slicing by using index-based iteration.
 *
 * ## Algorithm Analysis:
 * - **Time Complexity**: O(n) where n is the number of extras
 * - **Space Complexity**: O(n) due to call stack depth
 * - **Recursion Depth**: Equal to the number of extras
 *
 * ## Key Optimizations:
 * 1. **No Array Copying**: Uses index parameter instead of slice()
 * 2. **Tail Recursion Compatible**: Could be optimized by engines that support it
 * 3. **Memory Efficient**: Only stores primitive values on each stack frame
 *
 * ## Recursion Pattern:
 * ```
 * pizzaPriceRecursiveOptimized(pizza, [extra1, extra2, extra3], 0)
 * ├── Process extra1 at index 0
 * └── pizzaPriceRecursiveOptimized(pizza, [extra1, extra2, extra3], 1)
 *     ├── Process extra2 at index 1
 *     └── pizzaPriceRecursiveOptimized(pizza, [extra1, extra2, extra3], 2)
 *         ├── Process extra3 at index 2
 *         └── pizzaPriceRecursiveOptimized(pizza, [extra1, extra2, extra3], 3)
 *             └── Base case: index >= length, return pizza price
 * ```
 *
 * ## Performance Comparison:
 * - **vs. slice() approach**: Eliminates O(n²) array copying overhead
 * - **vs. iterative**: Same time complexity, but uses O(n) stack space
 * - **vs. functional reduce**: More verbose but demonstrates recursion principles
 *
 * ## Stack Safety:
 * JavaScript engines typically support 10,000-15,000 recursive calls before
 * stack overflow. This function will handle that many extras safely.
 *
 * @param {Pizza} pizza - The base pizza type ("Margherita", "Caprese", or "Formaggio")
 * @param {Extra[]} extras - Array of extra options to add to the pizza
 * @param {number} [index=0] - Current position in the extras array (internal parameter)
 *
 * @returns {number} The total price of the pizza including all extras
 *
 * @throws {TypeError} If pizza is not a valid pizza type
 * @throws {TypeError} If any extra is not a valid extra type
 * @throws {RangeError} If recursion depth exceeds JavaScript's call stack limit
 *
 * @example
 * // Basic usage - single extra
 * pizzaPriceRecursiveOptimized("Margherita", ["ExtraSauce"]);
 * // Returns: 8 (7 + 1)
 *
 * @example
 * // Multiple extras
 * pizzaPriceRecursiveOptimized("Caprese", ["ExtraSauce", "ExtraToppings"]);
 * // Returns: 12 (9 + 1 + 2)
 *
 * @example
 * // No extras - base case immediately
 * pizzaPriceRecursiveOptimized("Formaggio", []);
 * // Returns: 10
 *
 * @example
 * // Duplicate extras allowed
 * pizzaPriceRecursiveOptimized("Margherita", ["ExtraToppings", "ExtraToppings"]);
 * // Returns: 11 (7 + 2 + 2)
 *
 * @example
 * // Internal index parameter (normally not used directly)
 * pizzaPriceRecursiveOptimized("Caprese", ["ExtraSauce", "ExtraToppings"], 1);
 * // Returns: 11 (9 + 2) - starts processing from index 1
 *
 * @since 1.0.0
 * @see {@link pizzaPriceIterative} For a more efficient iterative alternative
 * @see {@link pizzaPriceFunctional} For a functional programming approach
 */
export function pizzaPriceRecursiveOptimized(pizza, extras, index = 0) {
    // ========================================================================
    // BASE CASE: Recursion Termination Condition
    // ========================================================================

    /**
     * Base case: We've processed all extras in the array
     *
     * This is the crucial termination condition that prevents infinite recursion.
     * When index reaches or exceeds the array length, we've processed all extras
     * and can return the base pizza price.
     *
     * Mathematical proof of termination:
     * - Initial call: index = 0, extras.length = n
     * - Each recursive call: index increases by 1
     * - After n calls: index = n, condition (n >= n) is true
     * - Therefore: recursion terminates in exactly n+1 function calls
     */
    if (index >= extras.length) {
        return PIZZA_PRICES[pizza];
    }

    // ========================================================================
    // RECURSIVE CASE: Problem Decomposition
    // ========================================================================

    /**
     * Recursive case: Process current extra and recurse for remaining extras
     *
     * This follows the classic recursive pattern:
     * 1. Process one element (current extra at index)
     * 2. Recurse with a smaller problem (index + 1)
     * 3. Combine results (addition)
     *
     * The key insight: Instead of creating a new array (expensive), we simply
     * advance the index pointer (cheap). This transforms:
     * - Array copying: O(n) per call → O(n²) total
     * - Index increment: O(1) per call → O(n) total
     */
    const currentExtra = extras[index];
    const currentExtraPrice = EXTRA_PRICES[currentExtra];

    /**
     * Recursive call explanation:
     * - Same pizza: Pizza type doesn't change
     * - Same extras array: No copying needed
     * - index + 1: Move to next extra (problem size reduction)
     *
     * This creates a linear recursion chain where each call processes exactly
     * one extra and delegates the rest to the next recursive call.
     */
    const remainingExtrasPrice = pizzaPriceRecursiveOptimized(pizza, extras, index + 1);

    // Combine current extra price with the price of remaining extras
    return currentExtraPrice + remainingExtrasPrice;
}

/**
 * Public API wrapper that maintains the original function signature
 *
 * This wrapper hides the internal index parameter from the public API,
 * providing a clean interface while leveraging the optimized recursive
 * implementation internally.
 *
 * @param {Pizza} pizza - The base pizza type
 * @param {...Extra} extras - Variable number of extra options
 *
 * @returns {number} The total price of the pizza including all extras
 *
 * @example
 * // Public API usage (matches original Exercism signature)
 * pizzaPrice("Margherita", "ExtraSauce", "ExtraToppings");
 * // Returns: 10 (7 + 1 + 2)
 */
export function pizzaPrice(pizza, ...extras) {
    return pizzaPriceRecursiveOptimized(pizza, extras);
}

// ============================================================================
// DETAILED EXECUTION TRACE EXAMPLE
// ============================================================================

/**
 * Execution trace for pizzaPriceRecursiveOptimized("Caprese", ["ExtraSauce", "ExtraToppings"])
 *
 * Call Stack Visualization:
 *
 * ┌─ Call 1: pizzaPriceRecursiveOptimized("Caprese", ["ExtraSauce", "ExtraToppings"], 0)
 * │  ├─ index = 0, extras.length = 2
 * │  ├─ 0 >= 2? No → Continue to recursive case
 * │  ├─ currentExtra = "ExtraSauce"
 * │  ├─ currentExtraPrice = 1
 * │  ├─ Make recursive call with index = 1
 * │  │
 * │  └─ Call 2: pizzaPriceRecursiveOptimized("Caprese", ["ExtraSauce", "ExtraToppings"], 1)
 * │     ├─ index = 1, extras.length = 2
 * │     ├─ 1 >= 2? No → Continue to recursive case
 * │     ├─ currentExtra = "ExtraToppings"
 * │     ├─ currentExtraPrice = 2
 * │     ├─ Make recursive call with index = 2
 * │     │
 * │     └─ Call 3: pizzaPriceRecursiveOptimized("Caprese", ["ExtraSauce", "ExtraToppings"], 2)
 * │        ├─ index = 2, extras.length = 2
 * │        ├─ 2 >= 2? Yes → Base case reached!
 * │        └─ Return PIZZA_PRICES["Caprese"] = 9
 * │
 * │  ├─ Call 2 resumes: remainingExtrasPrice = 9
 * │  └─ Return 2 + 9 = 11
 * │
 * ├─ Call 1 resumes: remainingExtrasPrice = 11
 * └─ Return 1 + 11 = 12
 *
 * Final Result: 12
 *
 * Memory Usage Per Call:
 * - pizza: reference to string (8 bytes)
 * - extras: reference to array (8 bytes)
 * - index: number (8 bytes)
 * - currentExtra: reference to string (8 bytes)
 * - currentExtraPrice: number (8 bytes)
 * - remainingExtrasPrice: number (8 bytes)
 *
 * Total per stack frame: ~48 bytes
 * For n extras: ~48n bytes total stack usage
 */
