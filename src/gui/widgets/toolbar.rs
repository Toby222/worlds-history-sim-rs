use {
    crate::{
        gui::{
            open_window,
            windows::{SaveLoad, WorldOverlaySelection, WorldViewSelection},
            WidgetId,
            WidgetSystem,
        },
        macros::iterable_enum,
        resources::{GenerateWorldProgressChannel, GenerateWorldTask, OpenedWindows},
    },
    bevy::{
        ecs::{
            change_detection::Mut,
            system::{SystemParam, SystemState},
            world::World,
        },
        log::debug,
    },
    bevy_egui::egui::{Layout, Ui},
    planet::WorldManager,
    std::marker::PhantomData,
};

iterable_enum!(ToolbarButton {
    GenerateWorld,
    SaveLoad,
    Views,
    Overlays,
});

impl ToolbarButton {
    fn clicked(self, world: &mut World) {
        world.resource_scope(|world, mut world_manager: Mut<WorldManager>| {
            match self {
                ToolbarButton::GenerateWorld => {
                    let progress_sender = world.resource::<GenerateWorldProgressChannel>().sender();
                    let generate_world_task = &mut world.resource_mut::<GenerateWorldTask>();
                    if generate_world_task.0.is_some() {
                        debug!("Already generating new world")
                    } else {
                        generate_world_task.0 =
                            Some(world_manager.new_world_async(None, progress_sender))
                    }
                },
                ToolbarButton::SaveLoad => {
                    open_window::<SaveLoad>(&mut world.resource_mut::<OpenedWindows>());
                },
                ToolbarButton::Views => {
                    open_window::<WorldViewSelection>(&mut world.resource_mut::<OpenedWindows>());
                },
                ToolbarButton::Overlays => {
                    open_window::<WorldOverlaySelection>(
                        &mut world.resource_mut::<OpenedWindows>(),
                    );
                },
            };
        });
    }
}

impl From<ToolbarButton> for &'static str {
    fn from(button: ToolbarButton) -> Self {
        (&button).into()
    }
}

impl From<&ToolbarButton> for &'static str {
    fn from(button: &ToolbarButton) -> Self {
        match button {
            ToolbarButton::Views => "Change view",
            ToolbarButton::Overlays => "Overlays",
            ToolbarButton::GenerateWorld => "Generate new world",
            ToolbarButton::SaveLoad => "Save/Load",
        }
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
pub struct ToolbarWidget<'w, 's> {
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
