use {
    crate::{
        gui::{open_window, update_textures, windows::Overlay, WidgetId, WidgetSystem},
        macros::iterable_enum,
        resources::OpenedWindows,
    },
    bevy::{
        asset::Assets,
        ecs::{
            change_detection::Mut,
            component::Component,
            system::{SystemParam, SystemState},
            world::World,
        },
        log::debug,
        render::texture::Image,
    },
    bevy_egui::egui::{Layout, Ui},
    planet::WorldManager,
    std::marker::PhantomData,
};

iterable_enum!(ToolbarButton {
    GenerateWorld,
    SaveWorld,
    LoadWorld,
    Overlays,
    ToggleBiomes,
});

impl ToolbarButton {
    fn clicked(self, world: &mut World) {
        world.resource_scope(|world, mut world_manager: Mut<'_, WorldManager>| {
            match self {
                ToolbarButton::GenerateWorld => {
                    if let Err(err) = world_manager.new_world() {
                        eprintln!("Failed to generate world: {}", err);
                    } else {
                        update_textures(&world_manager, &mut world.resource_mut::<Assets<Image>>());
                    }
                },
                ToolbarButton::SaveWorld => {
                    if let Err(err) = world_manager.save_world("planet.ron") {
                        eprintln!("Failed to save planet.ron: {}", err);
                    }
                },
                ToolbarButton::LoadWorld => {
                    let mut images = world.resource_mut::<Assets<Image>>();
                    if let Err(err) = world_manager.load_world("planet.ron", &mut images) {
                        eprintln!("Failed to load planet.ron: {}", err);
                    } else {
                        update_textures(&world_manager, &mut images);
                    }
                },
                ToolbarButton::Overlays => {
                    open_window::<Overlay>(&mut world.resource_mut::<OpenedWindows>());
                },
                ToolbarButton::ToggleBiomes => {
                    world_manager.render_settings.cycle_view();
                    update_textures(&world_manager, &mut world.resource_mut::<Assets<Image>>());
                },
            };
        });
    }
}

impl From<ToolbarButton> for &'static str {
    fn from(button: ToolbarButton) -> Self {
        match button {
            ToolbarButton::Overlays => "Overlays",
            ToolbarButton::ToggleBiomes => "Toggle biome view",
            ToolbarButton::GenerateWorld => "Generate new world",
            ToolbarButton::SaveWorld => "Save",
            ToolbarButton::LoadWorld => "Load",
        }
    }
}

impl From<&ToolbarButton> for &'static str {
    fn from(button: &ToolbarButton) -> Self {
        (*button).into()
    }
}

impl From<ToolbarButton> for String {
    fn from(button: ToolbarButton) -> Self {
        <&'static str>::from(button).into()
    }
}

impl From<&ToolbarButton> for String {
    fn from(button: &ToolbarButton) -> Self {
        <&'static str>::from(button).into()
    }
}

#[derive(SystemParam)]
pub(crate) struct ToolbarWidget<'w, 's> {
    #[system_param(ignore)]
    _phantom: PhantomData<(&'w (), &'s ())>,
}
impl WidgetSystem for ToolbarWidget<'_, '_> {
    fn render(world: &mut World, _state: &mut SystemState<Self>, ui: &mut Ui, _id: WidgetId) {
        ui.with_layout(
            Layout::left_to_right(bevy_egui::egui::Align::Center),
            |ui| {
                for button in ToolbarButton::ITEMS {
                    if ui.button(<&'static str>::from(button)).clicked() {
                        debug!("Pressed button: {:#?}", button);
                        button.clicked(world);
                    }
                }
            },
        );
    }
}
