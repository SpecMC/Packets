# This doesn't do tokenization. The formatting must be strict.

import sys

if len(sys.argv) != 2:
    print("Usage: python main.py <file>")
    exit(1)

with open(sys.argv[1], encoding="utf-8") as f:
    contents = f.read().splitlines()


# Remove comments
line = 0
while line < len(contents):
    comment = contents[line].find("//")
    if comment != -1:
        contents[line] = contents[line][:comment]

    if contents[line] == "":
        contents.pop(line)
        line -= 1

    line += 1


contents = "\n".join(contents)
char = 0
while char < len(contents):
    if contents[char : char + 4] == "enum":
        char += 4
        print(contents[char+1:char + contents[char:].find("(")])
    char += 1

# print(contents)
