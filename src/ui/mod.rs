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

#[derive(Message, Clone, Copy)]
pub enum UiEvent {
    SelectTab(usize),
    CloseTab(usize),
    SelectTool(ToolButtonAction),
}

#[derive(Component, Clone, Copy)]
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
            .add_message::<UiEvent>()
            // Update UI elements
            .add_systems(Update, update_fps_counter)
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
        children![
            (
                MenuBar,
                children![
                    (MenuBarButton {
                        text: "File".to_string(),
                    },),
                    (MenuBarButton {
                        text: "Edit".to_string(),
                    },),
                    (MenuBarButton {
                        text: "View".to_string(),
                    },),
                    (MenuBarButton {
                        text: "Camera".to_string(),
                    },),
                    (MenuBarButton {
                        text: "Window".to_string(),
                    },),
                    (MenuBarButton {
                        text: "Help".to_string(),
                    },),
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
