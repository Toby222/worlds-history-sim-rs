#[cfg(feature = "globe_view")]
use {
    crate::components::panning::Pan2d,
    bevy::{
        core_pipeline::{core_2d::Camera2d, core_3d::Camera3d},
        ecs::query::{With, Without},
        render::camera::Camera,
    },
};
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

#[cfg(not(feature = "globe_view"))]
iterable_enum!(ToolbarButton {
    GenerateWorld,
    SaveWorld,
    LoadWorld,
    Overlays,
    ToggleBiomes,
});
#[cfg(feature = "globe_view")]
iterable_enum!(ToolbarButton {
    GenerateWorld,
    SaveWorld,
    LoadWorld,
    Overlays,
    ToggleBiomes,
    GlobeView,
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
                #[cfg(feature = "globe_view")]
                ToolbarButton::GlobeView => {
                    let mut camera_3d = world
                        .query_filtered::<&mut Camera, (With<Camera3d>, Without<Camera2d>)>()
                        .single_mut(world);
                    camera_3d.is_active = !camera_3d.is_active;
                    let (mut camera_2d, mut pancam) = world
                        .query_filtered::<(&mut Camera, &mut Pan2d), (With<Camera2d>, Without<Camera3d>)>()
                        .single_mut(world);
                    camera_2d.is_active = !camera_2d.is_active;
                    pancam.enabled = camera_2d.is_active;
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
            #[cfg(feature = "globe_view")]
            ToolbarButton::GlobeView => "Toggle globe",
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
