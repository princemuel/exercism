# Pizza Order

Welcome to Pizza Order on Exercism's JavaScript Track.
If you need help running the tests or submitting your code, check out `HELP.md`.
If you get stuck on the exercise, check out `HINTS.md`, but try and solve it without using those first :)

## Introduction

Recursion is a powerful concept in programming that involves a function calling itself.
It can be a bit tricky to grasp at first, but once you understand the fundamentals, it becomes a valuable tool in solving complex problems.
In this tutorial, we'll explore recursion in JavaScript with easy-to-understand examples.

## What is Recursion?

Recursion occurs when a function calls itself, either directly or indirectly.
It's similar to a loop, but it involves breaking a problem down into smaller, more manageable sub-problems.

### Example 1: Countdown

Let's start with a simple example: a countdown function.

```javascript
function countdown(num) {
  // Base case
  if (num <= 0) {
    console.log("Blastoff!");
    return;
  }

  // Recursive case
  console.log(num);
  countdown(num - 1);
}

// Call the function
countdown(5);
```

In this example:

- **Base case**: When `num` becomes less than or equal to 0, the function prints "Blastoff!" and stops calling itself.
- **Recursive case**: The function prints the current `num` and calls itself with `num - 1`.

### Example 2: Factorial

Now, let's look at a classic example of recursion: calculating the factorial of a number.

```javascript
function factorial(n) {
  // Base case
  if (n === 0 || n === 1) {
    return 1;
  }

  // Recursive case
  return n * factorial(n - 1);
}

// Test the function
console.log(factorial(5)); // Output: 120
```

In this example:

- **Base case**: When `n` is 0 or 1, the function returns 1.
- **Recursive case**: The function multiplies `n` by the factorial of `n - 1`.

## Key Concepts

### Base Case

Every recursive function must have at least one base case, a condition where the function stops calling itself.
Without a base case, the recursion would continue indefinitely, leading to a stack overflow.

### Recursive Case

The recursive case defines how the function calls itself with a smaller or simpler version of the problem.

## Pros and Cons of Recursion

**Pros:**

- Elegant solution for certain problems.
- Mimics the mathematical induction concept.

**Cons:**

- Can be less efficient than iterative solutions.
- May lead to stack overflow for deep recursion.

## Conclusion

Recursion is a valuable technique that can simplify complex problems by breaking them into smaller, more manageable sub-problems.
Understanding base cases and recursive cases is crucial for implementing effective recursive solutions in JavaScript.

**Learn More:**

- [MDN: Recursion in JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Functions#recursion)
- [Eloquent JavaScript: Chapter 3 - Functions](https://eloquentjavascript.net/03_functions.html)

## Instructions

You run a pizza shop, and offer three types of pizzas:

- Margherita: $7
- Caprese: $9
- Formaggio: $10

If customers want, they can add an unlimited number of extra options: either "ExtraSauce" for $1 or "ExtraToppings" for $2.

Your task is to write code that assists the customer in figuring out the cost to them.

## Calculate the price of a pizza

Provided the pizza's name as the first argument, and an unlimited number of added options, calculate the price of the pizza in dollars.

```js
pizzaPrice("Margherita");
// => 7

pizzaPrice("Caprese", "ExtraSauce", "ExtraToppings");
// => 12

pizzaPrice("Caprese", "ExtraToppings", "ExtraToppings", "ExtraToppings", "ExtraToppings");
// => 17
```

## Calculate the total price of an order

Your function is called with a list of `PizzaOrder`s and should return the total price of the order in dollars.
Each `PizzaOrder` has a `pizza` property - the pizza's name, and an `extras` property - the list of extra options.

```js
const margherita = new PizzaOrder("Margherita");
const caprese = new PizzaOrder("Caprese", "ExtraToppings");
orderPrice([margherita, caprese]);
// => 18
```

You'll realize that you can't write this using recursion, as one test with a tremendous amount of orders will raise a `Maximum call stack size exceeded`.
No worries, this is intentional - try implementing this function using an imperative loop!
You have many options, such as, but not limited to using `reduce` or a `for` loop.

<!-- prettier-ignore-start -->
~~~~exercism/advanced
When the JavaScript interpreter is running the JavaScript code, it will keep track of which functions it has entered (started to call) on a data structure called "a stack".
When the function returns (ends), it is removed from the stack.

However, this stack has a limited size.
The most common mistake made is a recursive function that never ends.
Each call is placed on the stack, but before it returns, another call is placed on the stack.

```javascript
function kaboom() {
  kaboom()
}

kaboom()
// => RangeError: Maximum call stack size exceeded
```

The stacktrace of this error shows the same line over and over, which makes sense, because the function calls itself.
Whilst it has no real practical application in most cases, you can find out how tall that stack can get.

```javascript
let calls = 0;
function kaboom() {
  calls +=1 ;
  kaboom()
}

kaboom()
// => RangeError: Maximum call stack size exceeded

console.log(calls)
// => a number, generally higher than 10.000
```

There are only two viable solutions to a call stack error caused by a synchronous recursive function:
- ensure the functions return before the stack limit is reached, usually by adding or fixing a base case.
- rewrite the recursive function to an imperative loop, which will execute the body of the loop, without having to enter a function, thus without increasing the stack.
~~~~
<!-- prettier-ignore-end -->

## Source

### Created by

- @SleeplessByte
- @safwansamsudeen

### Contributed to by

- @Cool-Katt
