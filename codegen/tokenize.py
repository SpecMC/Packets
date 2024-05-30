WHITESPACE = [" ", "\t", "\n", "\r"]
SPECIAL_CHARS = [
    "==",
    "!=",
    "||",
    "&&",
    "**",
    "(",
    ")",
    "{",
    "}",
    "[",
    "]",
    ",",
    "=",
    # "-", removed to not mess with negative numbers
    # removed because useless
    # ".",
    # ";",
    # ":",
    # "+",
    # "*",
    # "/",
    # "%",
    # "!",
    # "&",
    # "|",
    # "^",
    # "~",
]


def tokenize(input: str) -> list[str]:
    tokens = []
    current_token = ""
    char = 0

    while char < len(input):
        if input[char] in WHITESPACE + SPECIAL_CHARS:
            if current_token != "":
                tokens.append(current_token)
                current_token = ""
            for special_char in SPECIAL_CHARS:
                if (
                    char + len(special_char) <= len(input)
                    and input[char : char + len(special_char)] == special_char
                ):
                    tokens.append(special_char)
                    char += len(special_char) - 1
                    break
        else:
            current_token += input[char]
        char += 1

    return tokens
