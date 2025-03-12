def convert(number: int):
    return "".join(
        value
        for key, value in {3: "Pling", 5: "Plang", 7: "Plong"}.items()
        if number % key == 0
    ) or str(number)
