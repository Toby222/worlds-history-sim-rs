use {
    crate::gui::{update_textures, WindowSystem},
    bevy::{
        asset::Assets,
        ecs::{
            change_detection::Mut,
            system::{SystemParam, SystemState},
            world::World,
        },
        render::texture::Image,
    },
    bevy_egui::egui::Ui,
    planet::{WorldManager, WorldRenderSettings, WorldView},
    std::marker::PhantomData,
};

#[derive(SystemParam)]
pub(crate) struct WorldViewSelection<'w, 's> {
    #[system_param(ignore)]
    _phantom: PhantomData<(&'w (), &'s ())>,
}

impl WindowSystem for WorldViewSelection<'_, '_> {
    fn draw_contents(world: &mut World, _state: &mut SystemState<Self>, ui: &mut Ui) {
        world.resource_scope(|world, mut render_settings: Mut<WorldRenderSettings>| {
            let current_selection = render_settings.view;
            for view in WorldView::iterator() {
                let view = *view;
                if ui
                    .selectable_label(view == current_selection, <&'static str>::from(view))
                    .clicked()
                    && render_settings.view != view
                {
                    render_settings.view = view;
                    world.resource_scope(|world, mut images: Mut<Assets<Image>>| {
                        update_textures(
                            world.resource::<WorldManager>(),
                            &render_settings,
                            &mut images,
                        );
                    });
                }
            }
        });
    }

    fn name() -> &'static str {
        "View Selection"
    }
}
