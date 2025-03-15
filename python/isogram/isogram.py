def is_isogram(s: str):
    s = s.lower().replace(" ", "").replace("-", "")
    return len(s) == len(set(s))
