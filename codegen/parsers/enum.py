from pyparsing import *

from .base import identifier
from .types import type


enum_field = identifier + Optional("=" + identifier | )

enum = (
    Keyword("enum")
    + identifier
    + "("
    + type
    + ")"
    + "{"
    + ZeroOrMore(Word(alphanums) + Word(alphanums))
    + "}"
)
