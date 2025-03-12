def convert(number: int):
    sounds = [
        "Pling" if number % 3 == 0 else "",
        "Plang" if number % 5 == 0 else "",
        "Plong" if number % 7 == 0 else "",
    ]

    match "".join(sounds):
        case "":
            return str(number)
        case sound:
            return sound

    # return ''.join( v for k,v in { 3: "Pling", 5: "Plang", 7: "Plong" }.items() if number % k == 0 ) or str(number)
