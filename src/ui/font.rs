use bevy::prelude::*;

use crate::ui::EditorUiElement;

#[derive(Resource, Deref, Clone)]
pub struct FontHandle(pub Handle<Font>);

pub fn load_font(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(FontHandle(asset_server.load("fonts/Inter.ttf")));
}

/// This system adds the font to the text elements that have the EditorUiElement component.
pub fn update_text_font(
    mut text_fonts: Query<&mut TextFont, (Added<Text>, With<EditorUiElement>)>,
    font_handle: Res<FontHandle>,
) {
    for mut text_font in text_fonts.iter_mut() {
        text_font.font = font_handle.0.clone();
    }
}