// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/components/disconnected_space.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Component**: Specifies that the entity path at which this is logged is disconnected from its parent.
///
/// This is useful for specifying that a subgraph is independent of the rest of the scene.
///
/// If a transform or pinhole is logged on the same path, this component will be ignored.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct DisconnectedSpace(
    /// Whether the entity path at which this is logged is disconnected from its parent.
    pub bool,
);

impl From<bool> for DisconnectedSpace {
    #[inline]
    fn from(is_disconnected: bool) -> Self {
        Self(is_disconnected)
    }
}

impl From<DisconnectedSpace> for bool {
    #[inline]
    fn from(value: DisconnectedSpace) -> Self {
        value.0
    }
}

impl<'a> From<DisconnectedSpace> for ::std::borrow::Cow<'a, DisconnectedSpace> {
    #[inline]
    fn from(value: DisconnectedSpace) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a DisconnectedSpace> for ::std::borrow::Cow<'a, DisconnectedSpace> {
    #[inline]
    fn from(value: &'a DisconnectedSpace) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl ::re_types_core::Loggable for DisconnectedSpace {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.components.DisconnectedSpace".into()
    }

    #[allow(clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use arrow2::datatypes::*;
        DataType::Boolean
    }

    #[allow(clippy::wildcard_imports)]
    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| {
                        let Self(data0) = datum.into_owned();
                        data0
                    });
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            BooleanArray::new(
                Self::arrow_datatype(),
                data0.into_iter().map(|v| v.unwrap_or_default()).collect(),
                data0_bitmap,
            )
            .boxed()
        })
    }

    #[allow(clippy::wildcard_imports)]
    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok(arrow_data
            .as_any()
            .downcast_ref::<BooleanArray>()
            .ok_or_else(|| {
                DeserializationError::datatype_mismatch(
                    DataType::Boolean,
                    arrow_data.data_type().clone(),
                )
            })
            .with_context("rerun.components.DisconnectedSpace#is_disconnected")?
            .into_iter()
            .map(|v| v.ok_or_else(DeserializationError::missing_data))
            .map(|res| res.map(|v| Some(Self(v))))
            .collect::<DeserializationResult<Vec<Option<_>>>>()
            .with_context("rerun.components.DisconnectedSpace#is_disconnected")
            .with_context("rerun.components.DisconnectedSpace")?)
    }
}
