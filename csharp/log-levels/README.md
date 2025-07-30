# Log Levels

Welcome to Log Levels on Exercism's C# Track.
If you need help running the tests or submitting your code, check out `HELP.md`.
If you get stuck on the exercise, check out `HINTS.md`, but try and solve it without using those first :)

## Introduction

## Strings

A `string` in C# is an object that represents immutable text as a sequence of Unicode characters (letters, digits, punctuation, etc.). Double quotes are used to define a `string` instance:

```csharp
string fruit = "Apple";
```

### Methods

Strings are manipulated by calling the string's [built-in methods][docs-string-methods].
For example, to remove the whitespace from a string, you can use the [Trim method][tutorial-docs.microsoft.com-trim-white-space] like this:

```csharp
string name = " Jane "
name.Trim()
// => "Jane"
```

Once a string has been constructed, its value can never change.
Any methods that appear to modify a string will actually return a new string.

### Concatenation

Multiple strings can be concatenated (added) together. The simplest way to achieve this is by using the `+` operator.

```csharp
string name = "Jane";
"Hello " + name + "!";
// => "Hello Jane!"
```

### Formatting

For any string formatting more complex than simple concatenation, string interpolation is preferred. To use interpolation in a string, prefix it with the dollar (`$`) symbol. Then you can use curly braces (`{}`) to access any variables inside your string.

```csharp
string name = "Jane";
$"Hello {name}!";
// => "Hello Jane!"
```

[docs-string-methods]: https://docs.microsoft.com/en-us/dotnet/api/system.string
[tutorial-docs.microsoft.com-trim-white-space]: https://docs.microsoft.com/en-us/dotnet/csharp/how-to/modify-string-contents#trim-white-space

## Instructions

In this exercise you'll be processing log-lines.

Each log line is a string formatted as follows: `"[<LEVEL>]: <MESSAGE>"`.

There are three different log levels:

- `INFO`
- `WARNING`
- `ERROR`

You have three tasks, each of which will take a log line and ask you to do something with it.

## 1. Get message from a log line

Implement the (_static_) `LogLine.Message()` method to return a log line's message:

```csharp
LogLine.Message("[ERROR]: Invalid operation")
// => "Invalid operation"
```

Any leading or trailing white space should be removed:

```csharp
LogLine.Message("[WARNING]:  Disk almost full\r\n")
// => "Disk almost full"
```

## 2. Get log level from a log line

Implement the (_static_) `LogLine.LogLevel()` method to return a log line's log level, which should be returned in lowercase:

```csharp
LogLine.LogLevel("[ERROR]: Invalid operation")
// => "error"
```

## 3. Reformat a log line

Implement the (_static_) `LogLine.Reformat()` method that reformats the log line, putting the message first and the log level after it in parentheses:

```csharp
LogLine.Reformat("[INFO]: Operation completed")
// => "Operation completed (info)"
```

## Source

### Created by

- @ErikSchierboom

### Contributed to by

- @yzAlvin
