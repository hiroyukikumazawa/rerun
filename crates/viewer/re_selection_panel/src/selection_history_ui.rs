use egui::RichText;

use re_ui::{UICommand, UiExt as _};
use re_viewer_context::{Item, ItemCollection, SelectionHistory};
use re_viewport_blueprint::ViewportBlueprint;

// ---

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SelectionHistoryUi {}

impl SelectionHistoryUi {
    pub(crate) fn selection_ui(
        &self,
        ui: &mut egui::Ui,
        viewport: &ViewportBlueprint,
        history: &mut SelectionHistory,
    ) -> Option<ItemCollection> {
        let next = self.next_button_ui(ui, viewport, history);
        let prev = self.prev_button_ui(ui, viewport, history);
        prev.or(next)
    }

    fn prev_button_ui(
        &self,
        ui: &mut egui::Ui,
        viewport: &ViewportBlueprint,
        history: &mut SelectionHistory,
    ) -> Option<ItemCollection> {
        // undo selection
        if let Some(previous) = history.previous() {
            let response = ui
                .small_icon_button(&re_ui::icons::ARROW_LEFT)
                .on_hover_text(format!(
                    "Go to previous selection{}:\n\
                {}\n\
                \n\
                Right-click for more.",
                    UICommand::SelectionPrevious.format_shortcut_tooltip_suffix(ui.ctx()),
                    selection_to_string(viewport, &previous.selection),
                ));

            let mut return_current = false;
            response.context_menu(|ui| {
                // undo: newest on top, oldest on bottom
                let cur = history.current;
                for i in (0..history.current).rev() {
                    self.history_item_ui(viewport, ui, i, history);
                }
                return_current = cur != history.current;
            });
            if return_current {
                return history.current().map(|sel| sel.selection);
            }

            // TODO(cmc): using the keyboard shortcut should highlight the associated
            // button or something (but then again it, it'd make more sense to do that
            // at the egui level rather than specifically here).
            if response.clicked() {
                return history.select_previous();
            }
        } else {
            ui.add_enabled_ui(false, |ui| {
                ui.small_icon_button(&re_ui::icons::ARROW_LEFT)
                    .on_disabled_hover_text("No past selections found");
            });
        }

        None
    }

    fn next_button_ui(
        &self,
        ui: &mut egui::Ui,
        viewport: &ViewportBlueprint,
        history: &mut SelectionHistory,
    ) -> Option<ItemCollection> {
        // redo selection
        if let Some(next) = history.next() {
            let response = ui
                .small_icon_button(&re_ui::icons::ARROW_RIGHT)
                .on_hover_text(format!(
                    "Go to next selection{}:\n\
                {}\n\
                \n\
                Right-click for more.",
                    UICommand::SelectionNext.format_shortcut_tooltip_suffix(ui.ctx()),
                    selection_to_string(viewport, &next.selection),
                ));

            let mut return_current = false;
            response.context_menu(|ui| {
                // redo: oldest on top, most recent on bottom
                let cur = history.current;
                for i in (history.current + 1)..history.stack.len() {
                    self.history_item_ui(viewport, ui, i, history);
                }
                return_current = cur != history.current;
            });
            if return_current {
                return history.current().map(|sel| sel.selection);
            }

            // TODO(cmc): using the keyboard shortcut should highlight the associated
            // button or something (but then again it, it'd make more sense to do that
            // at the egui level rather than specifically here).
            if response.clicked() {
                return history.select_next();
            }
        } else {
            ui.add_enabled_ui(false, |ui| {
                ui.small_icon_button(&re_ui::icons::ARROW_RIGHT)
                    .on_disabled_hover_text("No future selections found");
            });
        }

        None
    }

    #[allow(clippy::unused_self)]
    fn history_item_ui(
        &self,
        viewport: &ViewportBlueprint,
        ui: &mut egui::Ui,
        index: usize,
        history: &mut SelectionHistory,
    ) {
        if let Some(sel) = history.stack.get(index) {
            ui.horizontal(|ui| {
                {
                    // borrow checker workaround
                    let sel = selection_to_string(viewport, sel);
                    if ui
                        .selectable_value(&mut history.current, index, sel)
                        .clicked()
                    {
                        ui.close_menu();
                    }
                }
                if sel.iter_items().count() == 1 {
                    if let Some(item) = sel.iter_items().next() {
                        item_kind_ui(ui, item);
                    }
                }
            });
        }
    }
}

// Different kinds of selections can share the same path in practice! We need to
// differentiate those in the UI to avoid confusion.
fn item_kind_ui(ui: &mut egui::Ui, sel: &Item) {
    ui.weak(RichText::new(format!("({})", sel.kind())));
}

fn selection_to_string(viewport: &ViewportBlueprint, selection: &ItemCollection) -> String {
    debug_assert!(
        !selection.is_empty(),
        "History should never contain empty selections."
    );
    if selection.len() == 1 {
        if let Some(item) = selection.iter_items().next() {
            item_to_string(viewport, item)
        } else {
            // All items got removed or weren't there to begin with.
            debug_assert!(
                selection.iter_space_context().next().is_some(),
                "History should never keep selections that have both an empty item & context list."
            );
            "<space context>".to_owned()
        }
    } else if let Some(kind) = selection.are_all_items_same_kind() {
        format!("{}x {}s", selection.len(), kind)
    } else {
        "<multiple selections>".to_owned()
    }
}

fn item_to_string(viewport: &ViewportBlueprint, item: &Item) -> String {
    match item {
        Item::AppId(app_id) => app_id.to_string(),
        Item::DataSource(data_source) => data_source.to_string(),
        Item::StoreId(store_id) => store_id.to_string(),
        Item::SpaceView(space_view_id) => {
            // TODO(#4678): unnamed space views should have their label formatted accordingly (subdued)
            if let Some(space_view) = viewport.view(space_view_id) {
                space_view.display_name_or_default().as_ref().to_owned()
            } else {
                "<removed space view>".to_owned()
            }
        }
        Item::InstancePath(instance_path) => instance_path.to_string(),
        Item::DataResult(space_view_id, instance_path) => {
            // TODO(#4678): unnamed space views should have their label formatted accordingly (subdued)
            let space_view_display_name = if let Some(space_view) = viewport.view(space_view_id) {
                space_view.display_name_or_default().as_ref().to_owned()
            } else {
                "<removed space view>".to_owned()
            };

            format!("{instance_path} in {space_view_display_name}")
        }
        Item::ComponentPath(path) => {
            format!("{}:{}", path.entity_path, path.component_name.short_name(),)
        }
        Item::Container(container_id) => {
            // TODO(#4678): unnamed container should have their label formatted accordingly (subdued)
            if let Some(container) = viewport.container(container_id) {
                container.display_name_or_default().as_ref().to_owned()
            } else {
                "<removed container>".to_owned()
            }
        }
    }
}
