import sys

from tokenize import tokenize
from parse import *


if len(sys.argv) != 2:
    print("Usage: python main.py <file>")
    exit(1)

with open(sys.argv[1], encoding="utf-8") as f:
    contents: str = f.read()


# Remove comments
contents: list[str] = contents.splitlines()
line = 0
while line < len(contents):
    comment = contents[line].find("//")
    if comment != -1:
        contents[line] = contents[line][:comment]

    line += 1
contents: str = "\n".join(contents)


# Tokenize
tokens: list[str] = tokenize(contents)


# Parse and generate code
generated: str = ""
while True:
    try:
        enum: ProtocolEnum = ProtocolEnum.parse(tokens)
        generated += "enum " + enum.name + "{"
        field_value: int = 0
        for i, (name, value) in enumerate(enum.fields.items()):
            if i != 0:
                generated += ","
            generated += name
            if field_value != value:
                generated += f"={value}"
                field_value = value
            field_value += 1
        generated += "}"
    except:
        if len(tokens) == 0:
            break

print(generated)
