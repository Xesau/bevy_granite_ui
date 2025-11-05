use bevy::{
    camera::{
        Viewport,
        visibility::{Layer, RenderLayers},
    },
    prelude::*,
};

pub mod colors;
pub mod elements;
pub mod font;
pub mod icons;
pub mod shortcuts;

use elements::*;

use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

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
    SelectTab(usize),
    CloseTab(usize),
    FileNew,
    FileOpen,
    FileSave,
    FileSaveAs,
    FileClose,
    FileExit,
}

#[derive(Component, Clone)]
pub struct ClickAction(pub UiEvent);

#[derive(Debug, Clone, Copy)]
pub enum ToolButtonAction {
    Pointer,
    Move,
    Rotate,
    Scale,
    AddEntity,
    ImportFile,
}

#[derive(Resource)]
pub struct EditorRenderLayer(Layer);

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
            // Add default UI elements
            .add_systems(Startup, setup_ui.after(icons::load_tool_button_icons)) // Load icons before UI is - setup
            // Render layer
            .insert_resource(EditorRenderLayer(self.editor_render_layer))
            .add_systems(Update, add_render_layer)
            // Fonts
            .add_systems(Startup, font::load_font)
            // Icons
            .add_systems(Startup, icons::load_tool_button_icons)
            // Colors
            .insert_resource(colors::UiColors::default())
            .add_systems(Update, colors::update_colors)
            .add_systems(Update, colors::add_colors)
            .add_systems(Update, update_camera_viewport)
            // Reactive elements
            .add_systems(Update, reactive_tab)
            .add_systems(Update, reactive_tool_button)
            .add_systems(Update, reactive_status_bar)
            .add_systems(Update, reactive_menu_bar_button)
            .add_systems(Update, reactive_fps_counter)
            .add_systems(Update, reactive_camera_preview)
            .add_systems(Update, menu_bar_dropdown_visibility)
            .add_systems(Update, handle_ui_events)
            .add_message::<UiEvent>()
            // Update UI elements
            .add_systems(Update, update_fps_counter)
            .add_plugins(shortcuts::ShortcutsPlugin)
            .add_systems(Update, font::update_text_font);
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
                    (
                        MenuBarDropdownRoot,
                        children![
                            (
                                MenuBarButton {
                                    text: "File".to_string(),
                                    shortcut_text: None,
                                    is_dropdown: true,
                                    is_in_submenu: false,
                                },
                                ClickAction(UiEvent::OpenMenu {
                                    id: "file".to_string()
                                })
                            ),
                            (
                                MenuBarDropdown {
                                    id: "file".to_string()
                                },
                                children![
                                    (
                                        MenuBarButton {
                                            text: "New".to_string(),
                                            shortcut_text: shortcuts.get_shortcut(UiEvent::FileNew).map(|shortcut| shortcut.to_string()),
                                            is_in_submenu: true,
                                            is_dropdown: false,
                                        },
                                        ClickAction(UiEvent::FileNew)
                                    ),
                                    (
                                        MenuBarButton {
                                            text: "Open".to_string(),
                                            shortcut_text: shortcuts.get_shortcut(UiEvent::FileOpen).map(|shortcut| shortcut.to_string()),
                                            is_in_submenu: true,
                                            is_dropdown: false,
                                        },
                                        ClickAction(UiEvent::FileOpen)
                                    ),
                                    (
                                        MenuBarButton {
                                            text: "Save".to_string(),
                                            shortcut_text: shortcuts.get_shortcut(UiEvent::FileSave).map(|shortcut| shortcut.to_string()),
                                            is_in_submenu: true,
                                            is_dropdown: false,
                                        },
                                        ClickAction(UiEvent::FileSave)
                                    ),
                                    (
                                        MenuBarButton {
                                            text: "Save As".to_string(),
                                            shortcut_text: shortcuts.get_shortcut(UiEvent::FileSaveAs).map(|shortcut| shortcut.to_string()),
                                            is_in_submenu: true,
                                            is_dropdown: false,
                                        },
                                        ClickAction(UiEvent::FileSaveAs)
                                    ),
                                    (
                                        MenuBarButton {
                                            text: "Close".to_string(),
                                            shortcut_text: shortcuts.get_shortcut(UiEvent::FileClose).map(|shortcut| shortcut.to_string()),
                                            is_in_submenu: true,
                                            is_dropdown: false,
                                        },
                                        ClickAction(UiEvent::FileClose)
                                    ),
                                    (   MenuBarButton {
                                            text: "Exit".to_string(),
                                            shortcut_text: shortcuts.get_shortcut(UiEvent::FileExit).map(|shortcut| shortcut.to_string()),
                                            is_in_submenu: true,
                                            is_dropdown: false,
                                        },
                                        ClickAction(UiEvent::FileExit)
                                    )
                                ]
                            )
                        ]
                    ),
                    (
                        MenuBarDropdownRoot,
                        children![
                            (
                                MenuBarButton {
                                    text: "Edit".to_string(),
                                    shortcut_text: None,
                                    is_dropdown: false,
                                    is_in_submenu: false,
                                },
                                ClickAction(UiEvent::OpenMenu {
                                    id: "edit".to_string()
                                })
                            )
                        ]
                    ),
                    (
                        MenuBarDropdownRoot,
                        children![
                            (
                                MenuBarButton {
                                    text: "View".to_string(),
                                    shortcut_text: None,
                                    is_dropdown: false,
                                    is_in_submenu: false,
                                },
                                ClickAction(UiEvent::OpenMenu {
                                    id: "view".to_string()
                                })
                            )
                        ]
                    ),
                    (
                        MenuBarDropdownRoot,
                        children![
                            (
                                MenuBarButton {
                                    text: "Camera".to_string(),
                                    shortcut_text: None,
                                    is_dropdown: false,
                                    is_in_submenu: false,
                                },
                                ClickAction(UiEvent::OpenMenu {
                                    id: "camera".to_string()
                                })
                            )
                        ]
                    ),
                    (
                        MenuBarDropdownRoot,
                        children![
                            (
                                MenuBarButton {
                                    text: "Window".to_string(),
                                    shortcut_text: None,
                                    is_dropdown: false,
                                    is_in_submenu: false,
                                },
                                ClickAction(UiEvent::OpenMenu {
                                    id: "window".to_string()
                                })
                            )
                        ]
                    ),
                    (
                        MenuBarDropdownRoot,
                        children![
                            (
                                MenuBarButton {
                                    text: "Help".to_string(),
                                    shortcut_text: None,
                                    is_dropdown: false,
                                    is_in_submenu: false,
                                },
                                ClickAction(UiEvent::OpenMenu {
                                    id: "help".to_string()
                                })
                            )
                        ]
                    ),
                ]
            ),
            (
                TabBar,
                children![
                    Tab {
                        name: "Tab 1".to_string(),
                        is_active: true,
                    },
                    Tab {
                        name: "Tab 2".to_string(),
                        is_active: false,
                    },
                    Tab {
                        name: "Tab 3".to_string(),
                        is_active: false,
                    }
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
                                    ToolButton {
                                        action: ToolButtonAction::Pointer,
                                        icon: tool_button_icons.pointer.clone(),
                                        is_active: true
                                    },
                                    ToolButtonSeparator,
                                    ToolButton {
                                        action: ToolButtonAction::Move,
                                        icon: tool_button_icons.move_.clone(),
                                        is_active: false
                                    },
                                    ToolButtonSeparator,
                                    ToolButton {
                                        action: ToolButtonAction::Rotate,
                                        icon: tool_button_icons.rotate.clone(),
                                        is_active: false
                                    },
                                    ToolButtonSeparator,
                                    ToolButton {
                                        action: ToolButtonAction::Scale,
                                        icon: tool_button_icons.scale.clone(),
                                        is_active: false
                                    },
                                ]
                            ),
                            (
                                ToolButtonGroup,
                                children![
                                    ToolButton {
                                        action: ToolButtonAction::AddEntity,
                                        icon: tool_button_icons.add_entity.clone(),
                                        is_active: false
                                    },
                                    ToolButtonSeparator,
                                    ToolButton {
                                        action: ToolButtonAction::ImportFile,
                                        icon: tool_button_icons.add_prefab.clone(),
                                        is_active: false
                                    },
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

fn update_fps_counter(
    mut fps_counter: Single<&mut FpsCounter>,
    diagnostics: Res<DiagnosticsStore>,
) {
    fps_counter.fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
        .map(|fps| fps as f32);
}

#[derive(Message, Clone)]
pub struct MenuBarButtonClicked {
    pub id: String,
}

fn menu_bar_dropdown_visibility(
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

fn handle_ui_events(
    query: Query<(&ClickAction, &Interaction), Changed<Interaction>>,
    mut ui_event_writer: MessageWriter<UiEvent>,
) {
    for (click_action, interaction) in query.iter() {
        if *interaction == Interaction::Pressed {
            ui_event_writer.write(click_action.0.clone());
        }
    }
}