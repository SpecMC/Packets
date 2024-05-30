import abc
from typing import Dict, Type


class Parse(metaclass=abc.ABCMeta):
    """
    Abstract base class for protocol elements that can be parsed from tokens.
    """

    @staticmethod
    @abc.abstractmethod
    def parse(tokens: list[str]) -> object:
        """
        Parse the given tokens and return an object.

        Args:
            tokens (list[str]): The tokens to parse. This method consumes tokens from the list.

        Returns:
            object: The parsed object.

        Raises:
            NotImplementedError: If the parse method is not implemented.
        """

        raise NotImplementedError


class ProtocolInteger(Parse):
    U8: int = 0
    U16: int = 1
    U32: int = 2
    U64: int = 3
    I8: int = 4
    I16: int = 5
    I32: int = 6
    I64: int = 7
    VARINT: int = 8
    VARLONG: int = 9

    def __init__(self, type: Type["ProtocolInteger"]) -> None:
        self.type = type

    @staticmethod
    def parse(tokens: list[str]) -> "ProtocolInteger":
        type_str: str = tokens.pop(0).upper()

        try:
            type: Type[ProtocolInteger] = getattr(ProtocolInteger, type_str)
        except AttributeError:
            raise ValueError(f"Invalid type: {type_str}")

        return ProtocolInteger(type)

    def __str__(self) -> str:
        for name, value in ProtocolInteger.__dict__.items():
            if value == self.type:
                return name
        raise ValueError(f"Invalid type: {self.type}")


class ProtocolEnum(Parse):
    name: str
    type: Type[ProtocolInteger]
    fields: Dict[str, int]

    def __init__(self) -> None:
        self.name = ""
        self.type = ProtocolInteger.VARINT
        self.fields = {}

    @staticmethod
    def parse(tokens: list[str]) -> "ProtocolEnum":
        self = ProtocolEnum()
        assert tokens.pop(0) == "enum", "Expected keyword enum"
        self.name = tokens.pop(0)
        assert tokens.pop(0) == "(", "Expected open parenthesis"
        self.type = ProtocolInteger.parse(tokens)
        assert tokens.pop(0) == ")", "Expected close parenthesis"
        assert tokens.pop(0) == "{", "Expected open brace"

        field_value: int = 0
        while True:
            if tokens[0] == "}":
                break
            field_name = tokens.pop(0)
            if tokens[0] == "=":
                tokens.pop(0)
                field_value = int(tokens.pop(0))

            self.fields[field_name] = field_value
            field_value += 1

        assert tokens.pop(0) == "}", "Expected close brace"

        return self

    def __str__(self) -> str:
        return f"enum {self.name}({self.type}): {self.fields}"
