import sys
from pyparsing import *

from parsers import *


if len(sys.argv) != 2:
    print("Usage: python main.py <file>")
    sys.exit(1)

result = ZeroOrMore(enum).parseFile(sys.argv[1])
print(result.dump())
