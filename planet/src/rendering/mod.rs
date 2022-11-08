#[cfg(feature = "render")]
use bevy::asset::HandleId;
use {crate::macros::iterable_enum, bevy::utils::HashSet};

iterable_enum!(WorldView { Biomes, Topography });
iterable_enum!(WorldOverlay {
    Temperature,
    Rainfall
});

#[cfg(feature = "render")]
#[derive(Debug, Default)]
pub struct WorldRenderSettings {
    pub map_image_handle_id: Option<HandleId>,

    visible_overlays: HashSet<WorldOverlay>,
    pub view:         WorldView,
}

#[cfg(feature = "render")]
impl WorldRenderSettings {
    pub fn overlay_visible(&self, overlay: &WorldOverlay) -> bool {
        self.visible_overlays.contains(overlay)
    }

    pub fn toggle_overlay(&mut self, overlay: &WorldOverlay) {
        if self.visible_overlays.contains(overlay) {
            assert!(
                self.visible_overlays.remove(overlay),
                "Failed to remove overlay [{overlay:#?}], that shouldn't happen."
            );
        } else {
            assert!(
                self.visible_overlays.insert(*overlay),
                "Failed to insert overlay [{overlay:#?}], that shouldn't happen."
            );
        }
    }
}

impl Default for WorldView {
    fn default() -> Self {
        WorldView::Biomes
    }
}
