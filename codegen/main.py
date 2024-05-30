import sys

from tokenize import tokenize
from parse import *


if len(sys.argv) != 2:
    print("Usage: python main.py <file>")
    exit(1)

with open(sys.argv[1], encoding="utf-8") as f:
    contents = f.read()


# Remove comments
contents = contents.splitlines()
line = 0
while line < len(contents):
    comment = contents[line].find("//")
    if comment != -1:
        contents[line] = contents[line][:comment]

    line += 1
contents = "\n".join(contents)


tokens = tokenize(contents)
# print(tokens)


# token = 0
# while token < len(tokens):
#     if tokens[token] in ["enum", "type", "packet"]:
#         print(*tokens[token : token + 2])
#     token += 1

print(ProtocolEnum.parse(tokens))
print(ProtocolEnum.parse(tokens))
print(ProtocolEnum.parse(tokens))
print(ProtocolEnum.parse(tokens))
print(ProtocolEnum.parse(tokens))
print(ProtocolEnum.parse(tokens))
print(ProtocolEnum.parse(tokens))
# print(contents)
