// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/series_line.fbs".

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

/// **Archetype**: Define the style properties for a line series in a chart.
///
/// This archetype only provides styling information and should be logged as static
/// when possible. The underlying data needs to be logged to the same entity-path using
/// [`archetypes::Scalar`][crate::archetypes::Scalar].
///
/// ## Example
///
/// ### Line series
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_series_line_style").spawn()?;
///
///     // Set up plot styling:
///     // They are logged static as they don't change over time and apply to all timelines.
///     // Log two lines series under a shared root so that they show in the same plot by default.
///     rec.log_static(
///         "trig/sin",
///         &rerun::SeriesLine::new()
///             .with_color([255, 0, 0])
///             .with_name("sin(0.01t)")
///             .with_width(2.0),
///     )?;
///     rec.log_static(
///         "trig/cos",
///         &rerun::SeriesLine::new()
///             .with_color([0, 255, 0])
///             .with_name("cos(0.01t)")
///             .with_width(4.0),
///     )?;
///
///     for t in 0..((std::f32::consts::TAU * 2.0 * 100.0) as i64) {
///         rec.set_time_sequence("step", t);
///
///         // Log two time series under a shared root so that they show in the same plot by default.
///         rec.log("trig/sin", &rerun::Scalar::new((t as f64 / 100.0).sin()))?;
///         rec.log("trig/cos", &rerun::Scalar::new((t as f64 / 100.0).cos()))?;
///     }
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/series_line_style/d2616d98b1e46bdb85849b8669154fdf058e3453/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/series_line_style/d2616d98b1e46bdb85849b8669154fdf058e3453/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/series_line_style/d2616d98b1e46bdb85849b8669154fdf058e3453/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/series_line_style/d2616d98b1e46bdb85849b8669154fdf058e3453/1200w.png">
///   <img src="https://static.rerun.io/series_line_style/d2616d98b1e46bdb85849b8669154fdf058e3453/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug)]
pub struct SeriesLine {
    /// Color for the corresponding series.
    pub color: Option<crate::components::Color>,

    /// Stroke width for the corresponding series.
    pub width: Option<crate::components::StrokeWidth>,

    /// Display name of the series.
    ///
    /// Used in the legend.
    pub name: Option<crate::components::Name>,

    /// Configures the zoom-dependent scalar aggregation.
    ///
    /// This is done only if steps on the X axis go below a single pixel,
    /// i.e. a single pixel covers more than one tick worth of data. It can greatly improve performance
    /// (and readability) in such situations as it prevents overdraw.
    pub aggregation_policy: Option<crate::components::AggregationPolicy>,
}

impl ::re_types_core::SizeBytes for SeriesLine {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.color.heap_size_bytes()
            + self.width.heap_size_bytes()
            + self.name.heap_size_bytes()
            + self.aggregation_policy.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Option<crate::components::Color>>::is_pod()
            && <Option<crate::components::StrokeWidth>>::is_pod()
            && <Option<crate::components::Name>>::is_pod()
            && <Option<crate::components::AggregationPolicy>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.SeriesLineIndicator".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 4usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Color".into(),
            "rerun.components.StrokeWidth".into(),
            "rerun.components.Name".into(),
            "rerun.components.AggregationPolicy".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.SeriesLineIndicator".into(),
            "rerun.components.Color".into(),
            "rerun.components.StrokeWidth".into(),
            "rerun.components.Name".into(),
            "rerun.components.AggregationPolicy".into(),
        ]
    });

impl SeriesLine {
    /// The total number of components in the archetype: 0 required, 1 recommended, 4 optional
    pub const NUM_COMPONENTS: usize = 5usize;
}

/// Indicator component for the [`SeriesLine`] [`::re_types_core::Archetype`]
pub type SeriesLineIndicator = ::re_types_core::GenericIndicatorComponent<SeriesLine>;

impl ::re_types_core::Archetype for SeriesLine {
    type Indicator = SeriesLineIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.SeriesLine".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Series line"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: SeriesLineIndicator = SeriesLineIndicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow2_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let color = if let Some(array) = arrays_by_name.get("rerun.components.Color") {
            <crate::components::Color>::from_arrow2_opt(&**array)
                .with_context("rerun.archetypes.SeriesLine#color")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let width = if let Some(array) = arrays_by_name.get("rerun.components.StrokeWidth") {
            <crate::components::StrokeWidth>::from_arrow2_opt(&**array)
                .with_context("rerun.archetypes.SeriesLine#width")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let name = if let Some(array) = arrays_by_name.get("rerun.components.Name") {
            <crate::components::Name>::from_arrow2_opt(&**array)
                .with_context("rerun.archetypes.SeriesLine#name")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let aggregation_policy =
            if let Some(array) = arrays_by_name.get("rerun.components.AggregationPolicy") {
                <crate::components::AggregationPolicy>::from_arrow2_opt(&**array)
                    .with_context("rerun.archetypes.SeriesLine#aggregation_policy")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        Ok(Self {
            color,
            width,
            name,
            aggregation_policy,
        })
    }
}

impl ::re_types_core::AsComponents for SeriesLine {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            self.color
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.width
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.name
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.aggregation_policy
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for SeriesLine {}

impl SeriesLine {
    /// Create a new `SeriesLine`.
    #[inline]
    pub fn new() -> Self {
        Self {
            color: None,
            width: None,
            name: None,
            aggregation_policy: None,
        }
    }

    /// Color for the corresponding series.
    #[inline]
    pub fn with_color(mut self, color: impl Into<crate::components::Color>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Stroke width for the corresponding series.
    #[inline]
    pub fn with_width(mut self, width: impl Into<crate::components::StrokeWidth>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Display name of the series.
    ///
    /// Used in the legend.
    #[inline]
    pub fn with_name(mut self, name: impl Into<crate::components::Name>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Configures the zoom-dependent scalar aggregation.
    ///
    /// This is done only if steps on the X axis go below a single pixel,
    /// i.e. a single pixel covers more than one tick worth of data. It can greatly improve performance
    /// (and readability) in such situations as it prevents overdraw.
    #[inline]
    pub fn with_aggregation_policy(
        mut self,
        aggregation_policy: impl Into<crate::components::AggregationPolicy>,
    ) -> Self {
        self.aggregation_policy = Some(aggregation_policy.into());
        self
    }
}
