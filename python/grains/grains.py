def square(n: int):
    if n < 1 or n > 64:
        raise ValueError("square must be between 1 and 64")
    return 1 << (n - 1)


def total():
    return (1 << 64) - 1
