// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/components/range1d.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Component**: A 1D range, specifying a lower and upper bound.
#[derive(Clone, Debug, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(transparent)]
pub struct Range1D(pub crate::datatypes::Range1D);

impl ::re_types_core::SizeBytes for Range1D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::Range1D>::is_pod()
    }
}

impl<T: Into<crate::datatypes::Range1D>> From<T> for Range1D {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::datatypes::Range1D> for Range1D {
    #[inline]
    fn borrow(&self) -> &crate::datatypes::Range1D {
        &self.0
    }
}

impl std::ops::Deref for Range1D {
    type Target = crate::datatypes::Range1D;

    #[inline]
    fn deref(&self) -> &crate::datatypes::Range1D {
        &self.0
    }
}

impl std::ops::DerefMut for Range1D {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::datatypes::Range1D {
        &mut self.0
    }
}

::re_types_core::macros::impl_into_cow!(Range1D);

impl ::re_types_core::Loggable for Range1D {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        crate::datatypes::Range1D::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayRef>
    where
        Self: Clone + 'a,
    {
        crate::datatypes::Range1D::to_arrow_opt(data.into_iter().map(|datum| {
            datum.map(|datum| match datum.into() {
                ::std::borrow::Cow::Borrowed(datum) => ::std::borrow::Cow::Borrowed(&datum.0),
                ::std::borrow::Cow::Owned(datum) => ::std::borrow::Cow::Owned(datum.0),
            })
        }))
    }

    fn from_arrow2_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        crate::datatypes::Range1D::from_arrow2_opt(arrow_data)
            .map(|v| v.into_iter().map(|v| v.map(Self)).collect())
    }

    #[inline]
    fn from_arrow2(arrow_data: &dyn arrow2::array::Array) -> DeserializationResult<Vec<Self>>
    where
        Self: Sized,
    {
        crate::datatypes::Range1D::from_arrow2(arrow_data).map(bytemuck::cast_vec)
    }
}

impl ::re_types_core::Component for Range1D {
    #[inline]
    fn name() -> ComponentName {
        "rerun.components.Range1D".into()
    }
}
