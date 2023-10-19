# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/blueprint/space_view_maximized.fbs".

# You can extend this class by creating a "SpaceViewMaximizedExt" class in "space_view_maximized_ext.py".

from __future__ import annotations

from typing import Any, Sequence, Union

import numpy as np
import numpy.typing as npt
from attrs import define, field

from .._baseclasses import BaseBatch, BaseExtensionType
from .._converters import (
    to_np_uint8,
)

__all__ = [
    "SpaceViewMaximized",
    "SpaceViewMaximizedArrayLike",
    "SpaceViewMaximizedBatch",
    "SpaceViewMaximizedLike",
    "SpaceViewMaximizedType",
]


@define(init=False)
class SpaceViewMaximized:
    """
    **Blueprint**: Whether a space view is maximized.

    Unstable. Used for the ongoing blueprint experimentations.
    """

    def __init__(self: Any, id: npt.ArrayLike | None = None):
        """Create a new instance of the SpaceViewMaximized blueprint."""

        # You can define your own __init__ function as a member of SpaceViewMaximizedExt in space_view_maximized_ext.py
        self.__attrs_init__(id=id)

    id: npt.NDArray[np.uint8] | None = field(default=None, converter=to_np_uint8)

    def __array__(self, dtype: npt.DTypeLike = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of SpaceViewMaximizedExt in space_view_maximized_ext.py
        return np.asarray(self.id, dtype=dtype)


SpaceViewMaximizedLike = SpaceViewMaximized
SpaceViewMaximizedArrayLike = Union[
    SpaceViewMaximized,
    Sequence[SpaceViewMaximizedLike],
]


class SpaceViewMaximizedType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.blueprint.SpaceViewMaximized"


class SpaceViewMaximizedBatch(BaseBatch[SpaceViewMaximizedArrayLike]):
    _ARROW_TYPE = SpaceViewMaximizedType()
