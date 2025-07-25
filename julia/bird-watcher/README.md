# Bird Watcher

Welcome to Bird Watcher on Exercism's Julia Track.
If you need help running the tests or submitting your code, check out `HELP.md`.
If you get stuck on the exercise, check out `HINTS.md`, but try and solve it without using those first :)

## Introduction

In the [`Vectors`][vectors] Concept, we said that "arrays are at the heart of the Julia language" and a vector is a 1-dimensional.

Given this, we could reasonably hope that the language provides many versatile and powerful ways to _do things_ with vectors, whatever that means.

## Functions expecting vector input

Some very simple functions take a vector input and return a scalar output.

```julia
v = [2, 3, 4]
length(v)  # => 3
sum(v)  # => 9
```

## Arithmetic

Suppose you have a numerical vector and want to subtract 0.5 from each value.

```julia-repl
julia> v = [1.2, 1.5, 1.7]
3-element Vector{Float64}:
 1.2
 1.5
 1.7

julia> v - 0.5
ERROR: MethodError: no method matching -(::Vector{Float64}, ::Float64)
```

That fails, so what about subtracting another vector?

```julia-repl
julia> v - [0.5, 0.5, 0.5]
3-element Vector{Float64}:
 0.7
 1.0
 1.2
```

Successful, but quite tedious and memory-hungry as the vectors get longer.

Fortunately, Julia has a "magic" dot to solve this problem very simply: `v .- 0.5` is all you need.

The next section explains why.

## Broadcasting

So, `v - 0.5` fails but `v .- 0.5` succeeds, and we need to understand what the dot is doing.

Two things, which combine to give the desired result.

### 1. Element-wise application

Firstly, adding a dot before any infix operator means "apply this operation to each element separately".

Similarly, adding a dot _after_ a function name "vectorizes" it, even if the function was written for scalar inputs.

```julia-repl
julia> sqrt.([1, 4, 9])
3-element Vector{Float64}:
 1.0
 2.0
 3.0
```

### 2. Singleton expansion

We saw in a previous example that we can subtract vectors of equal length, though please understand that `.-` is a _safer_ operator than `-` by making the element-wise intention clear.

```julia-repl
julia> v .- [0.5, 0.5, 0.5]
3-element Vector{Float64}:
 0.7
 1.0
 1.2
```

What about vectors of unequal length?

```julia-repl
julia> v .- [0.5, 0.5]
ERROR: DimensionMismatch: arrays could not be broadcast to a common size

julia> v .- [0.5,]
3-element Vector{Float64}:
 0.7
 1.0
 1.2
```

In general, unequal lengths are an error, _except_ when one has length 1 (technically, a "singleton" dimension).

Singletons like `[0.5,]` or just `0.5` are automatically expanded to the necessary length by repetition.
This is at the heart of `broadcasting`.

## Indexing

Selecting elements of a vector by index number has been discussed in previous Concepts.

```julia
a = collect('A':'Z')  # => 26-element Vector{Char}

# index with an integer
a[2]  # => 'B'

# index with a range
 a[12:2:18]  # => ['L', 'N', 'P, 'R']

 # index with another vector
 a[ [1, 3, 5] ]  # => ['A', 'C', 'E']
```

### Logical indexing

It is also possible to select elements that satisfy some logical expression (technically, a "predicate").
This usually requires broadcasting.

```julia-repl
julia> a[a .< 'D']
3-element Vector{Char}:
 'A': ASCII/Unicode U+0041 (category Lu: Letter, uppercase)
 'B': ASCII/Unicode U+0042 (category Lu: Letter, uppercase)
 'C': ASCII/Unicode U+0043 (category Lu: Letter, uppercase)
```

For more complex expression the dots tend to proliferate (but they are small and easy to type).

```julia-repl
julia> a[a .< 'D' .|| a .> 'W']
6-element Vector{Char}:
 'A': ASCII/Unicode U+0041 (category Lu: Letter, uppercase)
 'B': ASCII/Unicode U+0042 (category Lu: Letter, uppercase)
 'C': ASCII/Unicode U+0043 (category Lu: Letter, uppercase)
 'X': ASCII/Unicode U+0058 (category Lu: Letter, uppercase)
 'Y': ASCII/Unicode U+0059 (category Lu: Letter, uppercase)
 'Z': ASCII/Unicode U+005A (category Lu: Letter, uppercase)
```

A reminder that the "vector" can in fact be any appropriate ordered iterable, such as a range:

```julia-repl
julia> n = 3:10
3:10

julia> n[isodd.(n)]
4-element Vector{Int64}:
 3
 5
 7
 9
```

[vectors]: https://exercism.org/tracks/julia/concepts/vectors

## Instructions

You're an avid bird watcher who keeps track of how many birds have visited your garden in the last seven days.

You have six tasks, all dealing with the numbers of birds that visited your garden.

## 1. Check how many birds visited today

Implement the `today()` function to return how many birds visited your garden today.
The bird counts are ordered by day, with the first element being the count of the oldest day, and the last element being today's count.

```julia-repl
julia> birds_per_day = [2, 5, 0, 7, 4, 1]
julia> today(birds_per_day)
1
```

## 2. Increment today's count

Implement the `increment_todays_count()` function to increment today's count:

```julia-repl
julia> birds_per_day = [2, 5, 0, 7, 4, 1]
julia> increment_todays_count(birds_per_day)
[2, 5, 0, 7, 4, 2]
```

## 3. Check if there was a day with no visiting birds

Implement the `has_day_without_birds()` function that returns `true` if there was a day at which zero birds visited the garden; otherwise, return `false`:

```julia-repl
julia> birds_per_day = [2, 5, 0, 7, 4, 1]
julia> has_day_without_birds(birds_per_day)
true
```

## 4. Calculate the number of visiting birds for the first number of days

Implement the `count_for_first_days()` function that returns the number of birds that have visited your garden from the start of the week, but limit the count to the specified number of days from the start of the week.

```julia-repl
julia> birds_per_day = [2, 5, 0, 7, 4, 1]
julia> count_for_first_days(birds_per_day, 4)
14
```

## 5. Calculate the number of busy days

Some days are busier that others.
A busy day is one where five or more birds have visited your garden.
Implement the `busy_days()` function to return the number of busy days:

```julia-repl
julia> birds_per_day = [2, 5, 0, 7, 4, 1]
julia> busy_days(birds_per_day)
2
```

## 6. Calculate averages by day of the week

You decide to extend your records by keeping counts for multiple weeks.
In each case, the counts are arranged by day of the week, from Monday as the first entry to Sunday as the last.

Implement the `average_per_day()` function that returns the average for 2 weeks.

```julia-repl
julia> week1 = [7, 2, 9, 1, 3, 0, 10]
julia> week2 = [2, 6, 4, 1, 3, 8, 9]
julia> average_per_day(week1, week2)
[4.5, 4.0, 6.5, 1.0, 4.0, 3.0, 9.5]
```

## Source

### Created by

- @colinleach
