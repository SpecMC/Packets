import sys
from pyparsing import *

from parsers import *


if len(sys.argv) != 2:
    print("Usage: python main.py <file>")
    sys.exit(1)

spec_file = open(sys.argv[1])
print(ZeroOrMore(enum).parseFile(spec_file))
