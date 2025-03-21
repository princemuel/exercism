def square_root(n: int):
    if n < 0:
        return None

    if n == 0 or n == 1:
        return n

    x = n // 2

    while True:
        # Calculate next approximation
        next_x = (x + n // x) // 2

        if next_x >= x:
            break

        x = next_x

    if x * x == n:
        return x

    return None
