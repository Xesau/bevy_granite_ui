use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum EditorColor {
    MenuBar,
    MenuBarButtonText,
    MenuBarButtonHover,
    MenuBarButtonHoverText,
    TabBar,
    Background,
    Text,
    TabHover,
    TabActive,
    Heading,
    HeadingText,
    Button,
    InputField,
    InputFieldText,
}

#[derive(Resource)]
#[allow(dead_code)]
pub struct UiColors {
    pub editor_colors: HashMap<EditorColor, Color>,
}

impl Default for UiColors {
    fn default() -> Self {
        let mut editor_colors = HashMap::new();
        editor_colors.insert(EditorColor::MenuBar, Srgba::hex("#3B3B3B").unwrap().into());
        editor_colors.insert(EditorColor::MenuBarButtonText, Color::WHITE);
        editor_colors.insert(EditorColor::MenuBarButtonHover, Srgba::hex("#2C2C2C").unwrap().into());
        editor_colors.insert(EditorColor::MenuBarButtonHoverText, Color::WHITE);
        editor_colors.insert(EditorColor::TabBar, Srgba::hex("#2B2B2B").unwrap().into());
        editor_colors.insert(EditorColor::Background, Srgba::hex("#1B1B1B").unwrap().into());
        editor_colors.insert(EditorColor::Text, Color::WHITE);
        editor_colors.insert(EditorColor::TabHover, Srgba::hex("#353535").unwrap().into());
        editor_colors.insert(EditorColor::TabActive, Srgba::hex("#1B1B1B").unwrap().into());
        editor_colors.insert(EditorColor::Heading, Color::WHITE);
        editor_colors.insert(EditorColor::HeadingText, Color::BLACK);
        editor_colors.insert(EditorColor::Button, Srgba::hex("#0C0C0C").unwrap().into());
        editor_colors.insert(EditorColor::InputField, Color::WHITE);
        editor_colors.insert(EditorColor::InputFieldText, Color::BLACK);
        Self { editor_colors }
    }
}

#[derive(Component, Clone, Copy)]
pub struct EditorTextColor(pub EditorColor, pub Option<EditorColor>, pub Option<EditorColor>);

#[derive(Component, Clone, Copy)]
pub struct EditorBackgroundColor(pub EditorColor, pub Option<EditorColor>, pub Option<EditorColor>);

/// Update the colors of the text and background of the elements that have the EditorTextColor and EditorBackgroundColor components.
/// This system is run when the UiColors resource is changed.
pub fn update_colors(
    mut text_colors: Query<(&mut TextColor, &EditorTextColor, Option<&Interaction>), Or<(Changed<EditorTextColor>, Changed<Interaction>)>>,
    mut background_colors: Query<(&mut BackgroundColor, &EditorBackgroundColor, Option<&Interaction>), Or<(Changed<EditorBackgroundColor>, Changed<Interaction>)>>,
    ui_colors: Res<UiColors>
) {
    for (mut text_color, editor_text_color, interaction) in text_colors.iter_mut() {
        let color = match (interaction, editor_text_color.1, editor_text_color.2) {
            (Some(Interaction::Pressed), _, Some(clicked_color)) => clicked_color,
            (Some(Interaction::Pressed), Some(hover_color), None) => hover_color,
            (Some(Interaction::Hovered), Some(hover_color), _) => hover_color,
            (_, _, _) => editor_text_color.0,
        };
        text_color.0 = ui_colors.editor_colors[&color];
    }
    for (mut background_color, editor_background_color, interaction) in background_colors.iter_mut() {
        let color = match (interaction, editor_background_color.1, editor_background_color.2) {
            (Some(Interaction::Pressed), _, Some(clicked_color)) => clicked_color,
            (Some(Interaction::Pressed), Some(hover_color), None) => hover_color,
            (Some(Interaction::Hovered), Some(hover_color), _) => hover_color,
            (_, _, _) => editor_background_color.0,
        };
        background_color.0 = ui_colors.editor_colors[&color];
    }
}

/// Add the colors to the elements that have the EditorTextColor and EditorBackgroundColor components.
/// This system is run when the EditorTextColor and EditorBackgroundColor components are added.
pub fn add_colors(
    mut commands: Commands,
    ui_colors: Res<UiColors>,
    text_colors: Query<(Entity, &EditorTextColor), Changed<EditorTextColor>>,
    background_colors: Query<(Entity, &EditorBackgroundColor), Changed<EditorBackgroundColor>>,
) {
    for (entity, editor_text_color) in text_colors.iter() {
        commands.entity(entity).insert(TextColor(ui_colors.editor_colors[&editor_text_color.0]));
    }
    for (entity, editor_background_color) in background_colors.iter() {
        commands.entity(entity).insert(BackgroundColor(ui_colors.editor_colors[&editor_background_color.0]));
    }
}