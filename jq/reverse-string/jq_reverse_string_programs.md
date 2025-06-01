# jq String Reversal Programs

Collection of different approaches to reverse a string using jq.

## 1. Explode/Reverse/Implode (Recommended)

```bash
echo '"hello world"' | jq 'explode | reverse | implode'
```

**As a reusable function:**

```jq
def reverse_string: explode | reverse | implode;
```

**Usage:**

```bash
echo '"Programming is fun"' | jq '. | explode | reverse | implode'
# Output: "nuf si gnimmargorP"
```

## 2. Split/Reverse/Join

```bash
echo '"hello"' | jq 'split("") | reverse | join("")'
```

## 3. Recursive Approach

```jq
def reverse_recursive:
  if length <= 1 then .
  else .[-1:] + (.[:-1] | reverse_recursive)
  end;
```

**Usage:**

```bash
echo '"hello"' | jq 'reverse_recursive'
```

## 4. Using Reduce (Building Backwards)

```bash
echo '"hello"' | jq 'reduce explode[] as $char (""; implode([$char]) + .)'
```

## 5. Manual Indexing with Map

```bash
echo '"hello"' | jq '. as $s | [range(length-1; -1; -1)] | map($s[.]) | join("")'
```

## 6. Using Foreach

```bash
echo '"hello"' | jq '[foreach explode[] as $c (null; $c)] | reverse | implode'
```

## 7. String Slicing with Range

```bash
echo '"programming"' | jq '[range(length)] | map(length - 1 - .) | map(input[.]) | join("")' --null-input --raw-input
```

## Performance Notes

- **Fastest:** `explode | reverse | implode` - Most idiomatic jq approach
- **Most Readable:** `split("") | reverse | join("")` - Clear intent
- **Educational:** Recursive approach - Good for functional programming practice
- **Avoid:** Deep recursion in jq isn't optimized

## Recommended Usage

For **Project Euler** and **Exercism** exercises, use the explode/reverse/implode method as it's the most idiomatic jq way to handle character-level string manipulation.

```bash
# Template for string reversal
echo '"your_string_here"' | jq 'explode | reverse | implode'
```
