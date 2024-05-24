from pyparsing import *


identifier = pyparsing_common.identifier("identifier*")
number = pyparsing_common.number
string = "\"" + ZeroOrMore(CharsNotIn("\"")) + "\""
literal = (number | string)("literal*")
