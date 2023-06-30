# NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa

__all__ = ["ClassId", "ClassIdArray", "ClassIdArrayLike", "ClassIdLike", "ClassIdType"]


## --- ClassId --- ##


@dataclass
class ClassId:
    """A 16-bit ID representing a type of semantic class."""

    id: int

    def __array__(self) -> npt.ArrayLike:
        return np.asarray(self.id)


ClassIdLike = Union[ClassId, int]

ClassIdArrayLike = Union[
    ClassIdLike,
    Sequence[ClassIdLike],
    npt.NDArray[np.uint8],
    npt.NDArray[np.uint16],
    npt.NDArray[np.uint32],
    npt.NDArray[np.uint64],
]


# --- Arrow support ---

from .class_id_ext import ClassIdArrayExt  # noqa: E402


class ClassIdType(pa.ExtensionType):  # type: ignore[misc]
    def __init__(self: type[pa.ExtensionType]) -> None:
        pa.ExtensionType.__init__(self, pa.uint16(), "rerun.class_id")

    def __arrow_ext_serialize__(self: type[pa.ExtensionType]) -> bytes:
        # since we don't have a parameterized type, we don't need extra metadata to be deserialized
        return b""

    @classmethod
    def __arrow_ext_deserialize__(
        cls: type[pa.ExtensionType], storage_type: Any, serialized: Any
    ) -> type[pa.ExtensionType]:
        # return an instance of this subclass given the serialized metadata.
        return ClassIdType()

    def __arrow_ext_class__(self: type[pa.ExtensionType]) -> type[pa.ExtensionArray]:
        return ClassIdArray


# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(ClassIdType())


class ClassIdArray(pa.ExtensionArray, ClassIdArrayExt):  # type: ignore[misc]
    @staticmethod
    def from_similar(data: ClassIdArrayLike | None) -> pa.Array:
        if data is None:
            return ClassIdType().wrap_array(pa.array([], type=ClassIdType().storage_type))
        else:
            return ClassIdArrayExt._from_similar(
                data,
                mono=ClassId,
                mono_aliases=ClassIdLike,
                many=ClassIdArray,
                many_aliases=ClassIdArrayLike,
                arrow=ClassIdType,
            )
