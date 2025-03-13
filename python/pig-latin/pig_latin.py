import re

RE = re.compile("^(x(?!r)|y(?!t)|[^aeiouqxy]*(?:qu?)?)(.+)$")


def translate(phrase: str):
    return " ".join(map(translate_word, phrase.split()))


def translate_word(word: str):
    return RE.sub(lambda match: "{1}{0}ay".format(*match.groups()), word)
