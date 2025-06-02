def rebase(input_base, digits, output_base):
    # check arguments
    if input_base < 2:
        raise ValueError("input base must be >= 2")
    for d in digits:
        if not 0 <= d < input_base:
            raise ValueError("all digits must satisfy 0 <= d < input base")
    if output_base < 2:
        raise ValueError("output base must be >= 2")

    if output_base == 10:
        return to_dec(input_base, digits)
    elif input_base == 10:
        return from_dec(output_base, digits)
    return from_dec(output_base, to_dec(input_base, digits))

def to_dec(input_base, digits):
    sum = 0
    index = len(digits)-1
    for d in digits:
        sum += d * (input_base ** index)
        index -= 1
    return [int(d) for d in str(sum)]

def from_dec(output_base, digits):
    num = int(''.join(map(str, digits)))
    if num == 0:
        return [0]
    remainders = []
    while (num != 0):
        remainders.append(num % output_base)
        num = int(num/output_base)
    remainders.reverse()
    return remainders
