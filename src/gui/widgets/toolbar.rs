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
        gui::{open_window, windows::Overlay, WidgetId, WidgetSystem},
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
        render::{render_resource::Extent3d, texture::Image},
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
    Rainfall,
    Temperature,
    Overlays,
    ToggleBiomes,
    Contours,
});
#[cfg(feature = "globe_view")]
iterable_enum!(ToolbarButton {
    GenerateWorld,
    SaveWorld,
    LoadWorld,
    Rainfall,
    Temperature,
    Overlays,
    ToggleBiomes,
    Contours,
    GlobeView,
});
fn update_textures(world_manager: &WorldManager, images: &mut Mut<Assets<Image>>) {
    debug!("refreshing world texture");
    let map_image_handle = images.get_handle(
        world_manager
            .map_image_handle_id
            .expect("No map image handle"),
    );
    let map_image = images
        .get_mut(&map_image_handle)
        .expect("Map image handle pointing to non-existing image");
    map_image.resize(Extent3d {
        width:                 world_manager.world().width,
        height:                world_manager.world().height,
        depth_or_array_layers: 1,
    });
    map_image.data = world_manager.map_color_bytes();
}
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
                ToolbarButton::Rainfall => {
                    world_manager.toggle_rainfall();
                    update_textures(&world_manager, &mut world.resource_mut::<Assets<Image>>());
                },
                ToolbarButton::Temperature => {
                    world_manager.toggle_temperature();
                    update_textures(&world_manager, &mut world.resource_mut::<Assets<Image>>());
                },
                ToolbarButton::Overlays => {
                    open_window::<Overlay>(&mut world.resource_mut::<OpenedWindows>());
                },
                ToolbarButton::ToggleBiomes => {
                    world_manager.cycle_view();
                    update_textures(&world_manager, &mut world.resource_mut::<Assets<Image>>());
                },
                ToolbarButton::Contours => {
                    world_manager.toggle_contours();
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
            ToolbarButton::Rainfall => "Toggle rainfall",
            ToolbarButton::Temperature => "Toggle temperature",
            ToolbarButton::Contours => "Toggle contours",
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
    fn system(world: &mut World, _state: &mut SystemState<Self>, ui: &mut Ui, _id: WidgetId) {
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
