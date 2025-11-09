use bevy::{
    camera::{
        Viewport,
        visibility::{Layer, RenderLayers},
    }, prelude::*
};

pub mod colors;
pub mod elements;
pub mod font;
pub mod icons;
pub mod shortcuts;
pub mod fullscreen;

use elements::*;

use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use crate::ui::fullscreen::FullscreenState;

pub struct UiPlugin {
    pub editor_render_layer: Layer,
}

impl Default for UiPlugin {
    fn default() -> Self {
        Self {
            editor_render_layer: 999,
        }
    }
}

#[derive(Message, Hash, Eq, PartialEq, Clone, Debug)]
pub enum UiEvent {
    OpenMenu { id: String },
    CloseMenus,
    FileNew,
    FileOpen,
    FileSave,
    FileSaveAs,
    FileClose,
    FileExit,
    ShowHelp,
    Undo,
    Redo,
    SelectTool(Tool),
    SelectTab(usize),
    CloseTab(usize),
    ToggleFullscreen,
    NextTab,
    PreviousTab,
}

#[derive(Component, Clone)]
pub struct ClickAction(pub UiEvent);

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
pub enum Tool {
    #[default]
    Pointer,
    Move,
    Rotate,
    Scale,
    AddEntity,
    ImportFile,
}

#[derive(Resource)]
pub struct EditorRenderLayer(Layer);

#[derive(Resource, Default, Clone, Copy)]
pub struct CurrentTab(pub Option<usize>);


impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Dependencies:
        // If FrameTimeDiagnosticsPlugin is not added yet, add it
        if app
            .get_added_plugins::<FrameTimeDiagnosticsPlugin>()
            .is_empty()
        {
            app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        }

        // If Bevy UI is not added yet, add it
        if app.get_added_plugins::<bevy::ui::UiPlugin>().is_empty() {
            app.add_plugins(bevy::ui::UiPlugin::default());
        }

        app
            .add_plugins(fullscreen::FullscreenPlugin)
            .add_plugins(shortcuts::ShortcutsPlugin)
            .add_plugins(colors::ColorsPlugin)
            .add_plugins(elements::ElementsPlugin)
            .add_plugins(font::FontPlugin)
            .add_plugins(icons::IconsPlugin)
            .add_message::<UiEvent>()

            .insert_resource(CurrentTab(Some(0)))
            .insert_resource(EditorRenderLayer(self.editor_render_layer))

            // Add default UI elements
            // Make sure icons are loaded before UI is set up to avoid uninitialized Icons resource
            .add_systems(Startup, setup_ui.after(icons::load_tool_button_icons))
            // Render layer
            .add_systems(Update, add_render_layer)
            // Camera
            .add_systems(Update, update_camera_viewport)
            // UI events
            .add_systems(Update, handle_click_action)
            .add_systems(Update, handle_close_app)
            .add_systems(Update, handle_toggle_fullscreen)
            .add_systems(Update, handle_tab_events)
            // Update UI elements
            .add_systems(Update, update_fps_counter)
            .add_systems(Update, update_menu_dropdown_visibility)
            .add_systems(Update, update_selected_tool_button)
            .add_systems(Update, update_current_tab.run_if(resource_changed::<CurrentTab>))
        ;
    }
}

/// This system adds the editor render layer to the elements that have the EditorUiElement component and don't have a render layer.
/// This is used to ensure that the editor ui elements are rendered on the correct layer and thus rendered by the correct camera.
fn add_render_layer(
    mut commands: Commands,
    editor_render_layer: Res<EditorRenderLayer>,
    editor_ui_elements: Query<Entity, (With<EditorUiElement>, Without<RenderLayers>)>,
) {
    for entity in editor_ui_elements.iter() {
        commands
            .entity(entity)
            .insert(RenderLayers::layer(editor_render_layer.0));
    }
}

#[derive(Component, Default, Clone, Copy)]
pub struct EditorUiElement;

#[derive(Component)]
pub struct EditorUiCamera;

/// Sets up the default UI elements.
fn setup_ui(
    mut commands: Commands,
    render_layer: Res<EditorRenderLayer>,
    tool_button_icons: Res<icons::ToolButtonIcons>,
    shortcuts: Res<shortcuts::Shortcuts>,
) {
    commands.spawn((
        EditorUiCamera,
        Camera2d::default(),
        RenderLayers::layer(render_layer.0),
        Camera {
            order: 999,
            ..default()
        },
    ));

    commands.spawn((
        EditorUi,
        Button,
        ClickAction(UiEvent::CloseMenus),
        children![
            (
                MenuBar,
                children![
                    elements::menu_bar_dropdown!("File".to_string(), "file",
                        [
                            MenuBarButton::new("New".to_string(), UiEvent::FileNew, &shortcuts),
                            MenuBarButton::new("Open".to_string(), UiEvent::FileOpen, &shortcuts),
                            MenuBarButton::new("Save".to_string(), UiEvent::FileSave, &shortcuts),
                            MenuBarButton::new("Save As".to_string(), UiEvent::FileSaveAs, &shortcuts),
                            MenuBarButton::new("Close".to_string(), UiEvent::FileClose, &shortcuts),
                            MenuBarButton::new("Exit".to_string(), UiEvent::FileExit, &shortcuts),
                        ]
                    ),
                    elements::menu_bar_dropdown!("Edit".to_string(), "edit",
                        [
                            MenuBarButton::new("Undo".to_string(), UiEvent::Undo, &shortcuts),
                            MenuBarButton::new("Redo".to_string(), UiEvent::Redo, &shortcuts),
                        ]
                    ),
                    elements::menu_bar_dropdown!("View".to_string(), "view",
                        [
                    	    MenuBarButton::new("Toggle Fullscreen".to_string(), UiEvent::ToggleFullscreen, &shortcuts),
                            MenuBarButton::new("Next Tab".to_string(), UiEvent::NextTab, &shortcuts),
                            MenuBarButton::new("Previous Tab".to_string(), UiEvent::PreviousTab, &shortcuts),
                        ]
                    ),
                    elements::menu_bar_dropdown!("Camera".to_string(), "camera",
                        [

                        ]
                    ),
                    elements::menu_bar_dropdown!("Help".to_string(), "help",
                        [
                            MenuBarButton::new("Show Help".to_string(), UiEvent::ShowHelp, &shortcuts),
                        ]
                    ),
                ]
            ),
            (
                TabBar,
                children![
                    Tab::new(0, "Tab 1".to_string(), true),
                    Tab::new(1, "Tab 2".to_string(), false),
                    Tab::new(2, "Tab 3".to_string(), false),
                ]
            ),
            (
                ToolBar,
                children![
                    (
                        ToolButtons,
                        children![
                            (
                                ToolButtonGroup,
                                children![
                                    ToolButton::new(Tool::Pointer, true, tool_button_icons.pointer.clone()),
                                    ToolButtonSeparator,
                                    ToolButton::new(Tool::Move, false, tool_button_icons.move_.clone()),
                                    ToolButtonSeparator,
                                    ToolButton::new(Tool::Rotate, false, tool_button_icons.rotate.clone()),
                                    ToolButtonSeparator,
                                    ToolButton::new(Tool::Scale, false, tool_button_icons.scale.clone()),
                                ]
                            ),
                            (
                                ToolButtonGroup,
                                children![
                                    ToolButton::new(Tool::AddEntity, false, tool_button_icons.add_entity.clone()),
                                    ToolButtonSeparator,
                                    ToolButton::new(Tool::ImportFile, false, tool_button_icons.add_prefab.clone()),
                                ]
                            )
                        ]
                    ),
                    (FpsCounter { fps: None })
                ]
            ),
            (CameraPreview,),
            StatusBar {
                text: "Some status".to_string(),
            }
        ],
    ));
}

/// Updates the camera viewport of the other cameras other than the EditorUiCamera
/// to match the screen coordinates of the CameraPreview element in the UI.
fn update_camera_viewport(
    mut other_cameras: Query<&mut Camera, Without<EditorUiCamera>>,
    camera_preview_position: Single<
        (&UiGlobalTransform, &ComputedNode),
        (With<CameraPreview>, Changed<UiGlobalTransform>),
    >,
    // _: Single<&Camera, Added<EditorUiCamera>>,
) {
    let center_x = camera_preview_position.0.translation.x;
    let center_y = camera_preview_position.0.translation.y;
    let width = camera_preview_position.1.unrounded_size.x;
    let height = camera_preview_position.1.unrounded_size.y;
    let top_left_x = center_x - width / 2.0;
    let top_left_y = center_y - height / 2.0;

    for mut camera in other_cameras.iter_mut() {
        camera.viewport = Some(Viewport {
            physical_position: UVec2::new(top_left_x as u32, top_left_y as u32),
            physical_size: UVec2::new(width as u32, height as u32),
            ..default()
        });
    }
}

/// Updates FpsCounter component's fps field based on the FrameTimeDiagnosticsPlugin.
fn update_fps_counter(
    mut fps_counter: Single<&mut FpsCounter>,
    diagnostics: Res<DiagnosticsStore>,
) {
    fps_counter.fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
        .map(|fps| fps as f32);
}

/// Updates ToolButton component's is_active field based on the selected tool.
fn update_selected_tool_button(
    mut tool_buttons: Query<&mut ToolButton>,
    mut tool_button_clicked_reader: MessageReader<UiEvent>,
) {
    for event in tool_button_clicked_reader.read() {
        if let UiEvent::SelectTool(tool) = event {
            for mut tool_button in tool_buttons.iter_mut() {
                tool_button.is_active = tool_button.action == *tool;
            }
        }
    }
}

/// Updates MenuBarDropdown component's visibility based on the clicked button.
fn update_menu_dropdown_visibility(
    mut menu_bar_dropdown: Query<(&MenuBarDropdown, &mut Visibility)>,
    mut menu_bar_button_clicked_reader: MessageReader<UiEvent>,
) {
    // Update dropdown visibility based on the clicked button
    for event in menu_bar_button_clicked_reader.read() {
        match event {
            UiEvent::OpenMenu { id } => {
                for (menu_bar_dropdown, mut visibility) in menu_bar_dropdown.iter_mut() {
                    *visibility = if menu_bar_dropdown.id == *id {
                        Visibility::Visible
                    } else {
                        Visibility::Hidden
                    };
                }
            }
            // All other events close the menus
            _ => {
                for (_, mut visibility) in menu_bar_dropdown.iter_mut() {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}

fn handle_click_action(
    query: Query<(&ClickAction, &Interaction), Changed<Interaction>>,
    mut ui_event_writer: MessageWriter<UiEvent>,
) {
    for (click_action, interaction) in query.iter() {
        if *interaction == Interaction::Pressed {
            ui_event_writer.write(click_action.0.clone());
        }
    }
}

pub fn handle_close_app(
    mut ui_event_reader: MessageReader<UiEvent>,
    mut app_exit_writer: MessageWriter<AppExit>,
) {
    for event in ui_event_reader.read() {
        if let UiEvent::FileExit = event {
            app_exit_writer.write(AppExit::Success);
        }
    }
}

fn handle_toggle_fullscreen(
    mut ui_event_reader: MessageReader<UiEvent>,
    fullscreen_state: Res<State<FullscreenState>>,
    mut next_state: ResMut<NextState<FullscreenState>>,
) {
    for event in ui_event_reader.read() {
        if let UiEvent::ToggleFullscreen = event {
            next_state.set(if fullscreen_state.get() == &FullscreenState::Normal {
                FullscreenState::Fullscreen
            } else {
                FullscreenState::Normal
            });
        }
    }
}

// Handle next, previous and select tab
fn handle_tab_events(
    mut ui_event_reader: MessageReader<UiEvent>,
    mut current_tab: ResMut<CurrentTab>,
) {
    let tab_count = 3;
    for event in ui_event_reader.read() {
        if let UiEvent::NextTab = event {
            // Wrap around to the first tab if the current tab is the last tab
            current_tab.0 = if tab_count == 0 { None } else { Some((current_tab.0.unwrap_or(0) + 1) % tab_count) };
        }
        if let UiEvent::PreviousTab = event {
            // Wrap around to the last tab if the current tab is the first tab
            current_tab.0 = if tab_count == 0 { None } else { Some((tab_count + current_tab.0.unwrap_or(0) - 1) % tab_count) };
        }
        if let UiEvent::SelectTab(index) = event {
            // Clamp the index to the range of the tab count
            current_tab.0 = Some((*index).max(0).min(tab_count - 1));
        }
    }
}

/// Updates Tab component's is_active field based on the CurrentTab resource.
fn update_current_tab(
    current_tab: Res<CurrentTab>,
    tabs: Query<(Entity, &Tab)>,
    mut commands: Commands,
) {
    for (entity, tab) in tabs.iter() {
        let new_active=  Some(tab.index) == current_tab.0;
        if new_active != tab.is_active {
            commands.entity(entity).insert(Tab {
                is_active: new_active,
                ..((*tab).clone())
            });
        }
    }
}