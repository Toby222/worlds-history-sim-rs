use {
    crate::{
        components::panning::Pan2d,
        gui::{WidgetId, WidgetSystem},
        macros::iterable_enum,
    },
    bevy::{
        ecs::{
            component::Component,
            system::{SystemParam, SystemState},
            world::World,
        },
        log::debug,
        prelude::{Assets, Camera, Camera2d, Camera3d, Image, Mut, With, Without},
        render::render_resource::Extent3d,
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
    PlanetView,
    Contours,
});
#[cfg(feature = "globe_view")]
iterable_enum!(ToolbarButton {
    GenerateWorld,
    SaveWorld,
    LoadWorld,
    Rainfall,
    Temperature,
    PlanetView,
    Contours,
    GlobeView,
});
fn update_textures(world: &mut World) {
    debug!("refreshing world texture");
    world.resource_scope(|world, world_manager: Mut<WorldManager>| {
        let mut images = world.resource_mut::<Assets<Image>>();

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
    });
}
impl ToolbarButton {
    fn clicked(self, world: &mut World) {
        match self {
            ToolbarButton::GenerateWorld => {
                world.resource_scope(|world, mut world_manager: Mut<'_, WorldManager>| {
                    match world_manager.new_world() {
                        Err(err) => {
                            eprintln!("Failed to generate world: {}", err);
                        },
                        Ok(_) => {
                            update_textures(world);
                        },
                    }
                })
            },
            ToolbarButton::SaveWorld => {
                if let Err(err) = world.resource::<WorldManager>().save_world("planet.ron") {
                    eprintln!("Failed to save planet.ron: {}", err);
                }
            },
            ToolbarButton::LoadWorld => {
                world.resource_scope(|world, mut images: Mut<'_, Assets<Image>>| {
                    if let Err(err) = world
                        .resource_mut::<WorldManager>()
                        .load_world("planet.ron", &mut images)
                    {
                        eprintln!("Failed to save planet.ron: {}", err);
                    } else {
                        update_textures(world);
                    }
                });
            },
            ToolbarButton::Rainfall => {
                world.resource_mut::<WorldManager>().toggle_rainfall();
                update_textures(world);
            },
            ToolbarButton::Temperature => {
                world.resource_mut::<WorldManager>().toggle_temperature();
                update_textures(world);
            },
            ToolbarButton::PlanetView => {
                world.resource_mut::<WorldManager>().cycle_view();
                update_textures(world);
            },
            ToolbarButton::Contours => {
                world.resource_mut::<WorldManager>().toggle_contours();
                update_textures(world);
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
    }
}

impl From<ToolbarButton> for &'static str {
    fn from(button: ToolbarButton) -> Self {
        match button {
            ToolbarButton::Rainfall => "Toggle rainfall",
            ToolbarButton::Temperature => "Toggle temperature",
            ToolbarButton::Contours => "Toggle contours",
            ToolbarButton::PlanetView => "Cycle view",
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
