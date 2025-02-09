// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/viewport_blueprint.fbs".

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

/// **Archetype**: The top-level description of the viewport.
#[derive(Clone, Debug, Default)]
pub struct ViewportBlueprint {
    /// The layout of the space-views
    pub root_container: Option<crate::blueprint::components::RootContainer>,

    /// Show one tab as maximized?
    pub maximized: Option<crate::blueprint::components::SpaceViewMaximized>,

    /// Whether the viewport layout is determined automatically.
    ///
    /// If `true`, the container layout will be reset whenever a new space view is added or removed.
    /// This defaults to `false` and is automatically set to `false` when there is user determined layout.
    pub auto_layout: Option<crate::blueprint::components::AutoLayout>,

    /// Whether or not space views should be created automatically.
    ///
    /// If `true`, the viewer will only add space views that it hasn't considered previously (as identified by `past_viewer_recommendations`)
    /// and which aren't deemed redundant to existing space views.
    /// This defaults to `false` and is automatically set to `false` when the user adds space views manually in the viewer.
    pub auto_space_views: Option<crate::blueprint::components::AutoSpaceViews>,

    /// Hashes of all recommended space views the viewer has already added and that should not be added again.
    ///
    /// This is an internal field and should not be set usually.
    /// If you want the viewer from stopping to add space views, you should set `auto_space_views` to `false`.
    ///
    /// The viewer uses this to determine whether it should keep adding space views.
    pub past_viewer_recommendations:
        Option<Vec<crate::blueprint::components::ViewerRecommendationHash>>,
}

impl ::re_types_core::SizeBytes for ViewportBlueprint {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.root_container.heap_size_bytes()
            + self.maximized.heap_size_bytes()
            + self.auto_layout.heap_size_bytes()
            + self.auto_space_views.heap_size_bytes()
            + self.past_viewer_recommendations.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Option<crate::blueprint::components::RootContainer>>::is_pod()
            && <Option<crate::blueprint::components::SpaceViewMaximized>>::is_pod()
            && <Option<crate::blueprint::components::AutoLayout>>::is_pod()
            && <Option<crate::blueprint::components::AutoSpaceViews>>::is_pod()
            && <Option<Vec<crate::blueprint::components::ViewerRecommendationHash>>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.blueprint.components.ViewportBlueprintIndicator".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.blueprint.components.RootContainer".into(),
            "rerun.blueprint.components.SpaceViewMaximized".into(),
            "rerun.blueprint.components.AutoLayout".into(),
            "rerun.blueprint.components.AutoSpaceViews".into(),
            "rerun.blueprint.components.ViewerRecommendationHash".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 6usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.blueprint.components.ViewportBlueprintIndicator".into(),
            "rerun.blueprint.components.RootContainer".into(),
            "rerun.blueprint.components.SpaceViewMaximized".into(),
            "rerun.blueprint.components.AutoLayout".into(),
            "rerun.blueprint.components.AutoSpaceViews".into(),
            "rerun.blueprint.components.ViewerRecommendationHash".into(),
        ]
    });

impl ViewportBlueprint {
    /// The total number of components in the archetype: 0 required, 1 recommended, 5 optional
    pub const NUM_COMPONENTS: usize = 6usize;
}

/// Indicator component for the [`ViewportBlueprint`] [`::re_types_core::Archetype`]
pub type ViewportBlueprintIndicator = ::re_types_core::GenericIndicatorComponent<ViewportBlueprint>;

impl ::re_types_core::Archetype for ViewportBlueprint {
    type Indicator = ViewportBlueprintIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.ViewportBlueprint".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Viewport blueprint"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: ViewportBlueprintIndicator = ViewportBlueprintIndicator::DEFAULT;
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
        let root_container =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.RootContainer") {
                <crate::blueprint::components::RootContainer>::from_arrow2_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.ViewportBlueprint#root_container")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let maximized = if let Some(array) =
            arrays_by_name.get("rerun.blueprint.components.SpaceViewMaximized")
        {
            <crate::blueprint::components::SpaceViewMaximized>::from_arrow2_opt(&**array)
                .with_context("rerun.blueprint.archetypes.ViewportBlueprint#maximized")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let auto_layout =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.AutoLayout") {
                <crate::blueprint::components::AutoLayout>::from_arrow2_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.ViewportBlueprint#auto_layout")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let auto_space_views =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.AutoSpaceViews") {
                <crate::blueprint::components::AutoSpaceViews>::from_arrow2_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.ViewportBlueprint#auto_space_views")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let past_viewer_recommendations = if let Some(array) =
            arrays_by_name.get("rerun.blueprint.components.ViewerRecommendationHash")
        {
            Some({
                <crate::blueprint::components::ViewerRecommendationHash>::from_arrow2_opt(&**array)
                    .with_context(
                        "rerun.blueprint.archetypes.ViewportBlueprint#past_viewer_recommendations",
                    )?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context(
                        "rerun.blueprint.archetypes.ViewportBlueprint#past_viewer_recommendations",
                    )?
            })
        } else {
            None
        };
        Ok(Self {
            root_container,
            maximized,
            auto_layout,
            auto_space_views,
            past_viewer_recommendations,
        })
    }
}

impl ::re_types_core::AsComponents for ViewportBlueprint {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            self.root_container
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.maximized
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.auto_layout
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.auto_space_views
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.past_viewer_recommendations
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for ViewportBlueprint {}

impl ViewportBlueprint {
    /// Create a new `ViewportBlueprint`.
    #[inline]
    pub fn new() -> Self {
        Self {
            root_container: None,
            maximized: None,
            auto_layout: None,
            auto_space_views: None,
            past_viewer_recommendations: None,
        }
    }

    /// The layout of the space-views
    #[inline]
    pub fn with_root_container(
        mut self,
        root_container: impl Into<crate::blueprint::components::RootContainer>,
    ) -> Self {
        self.root_container = Some(root_container.into());
        self
    }

    /// Show one tab as maximized?
    #[inline]
    pub fn with_maximized(
        mut self,
        maximized: impl Into<crate::blueprint::components::SpaceViewMaximized>,
    ) -> Self {
        self.maximized = Some(maximized.into());
        self
    }

    /// Whether the viewport layout is determined automatically.
    ///
    /// If `true`, the container layout will be reset whenever a new space view is added or removed.
    /// This defaults to `false` and is automatically set to `false` when there is user determined layout.
    #[inline]
    pub fn with_auto_layout(
        mut self,
        auto_layout: impl Into<crate::blueprint::components::AutoLayout>,
    ) -> Self {
        self.auto_layout = Some(auto_layout.into());
        self
    }

    /// Whether or not space views should be created automatically.
    ///
    /// If `true`, the viewer will only add space views that it hasn't considered previously (as identified by `past_viewer_recommendations`)
    /// and which aren't deemed redundant to existing space views.
    /// This defaults to `false` and is automatically set to `false` when the user adds space views manually in the viewer.
    #[inline]
    pub fn with_auto_space_views(
        mut self,
        auto_space_views: impl Into<crate::blueprint::components::AutoSpaceViews>,
    ) -> Self {
        self.auto_space_views = Some(auto_space_views.into());
        self
    }

    /// Hashes of all recommended space views the viewer has already added and that should not be added again.
    ///
    /// This is an internal field and should not be set usually.
    /// If you want the viewer from stopping to add space views, you should set `auto_space_views` to `false`.
    ///
    /// The viewer uses this to determine whether it should keep adding space views.
    #[inline]
    pub fn with_past_viewer_recommendations(
        mut self,
        past_viewer_recommendations: impl IntoIterator<
            Item = impl Into<crate::blueprint::components::ViewerRecommendationHash>,
        >,
    ) -> Self {
        self.past_viewer_recommendations = Some(
            past_viewer_recommendations
                .into_iter()
                .map(Into::into)
                .collect(),
        );
        self
    }
}
