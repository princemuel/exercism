import string


def is_pangram(sentence: str) -> bool:
    # return set(string.ascii_lowercase) <= set(sentence.lower())
    return set(string.ascii_lowercase).issubset(sentence.lower())
