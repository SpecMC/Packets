from pyparsing import *

from .base import identifier, literal
from .types import type


enum_field = identifier + Optional("=" + Word(nums)("enum_field_value*"))

enum = (
    Keyword("enum")
    + identifier("enum_name*")
    + "("
    + type
    + ")"
    + "{"
    + ZeroOrMore(enum_field)("enum_fields*")
    + "}"
)("enum*")
