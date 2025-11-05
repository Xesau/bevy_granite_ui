use bevy::prelude::*;

pub struct IconsPlugin;

impl Plugin for IconsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_tool_button_icons);
    }
}

#[derive(Resource)]
pub struct ToolButtonIcons {
    pub pointer: Handle<Image>,
    pub move_: Handle<Image>,
    pub rotate: Handle<Image>,
    pub scale: Handle<Image>,
    pub add_entity: Handle<Image>,
    pub add_prefab: Handle<Image>,
}

pub fn load_tool_button_icons(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.insert_resource(ToolButtonIcons {
        pointer: asset_server.load("toolbar_icons/Cursor.png"),
        move_: asset_server.load("toolbar_icons/Move.png"),
        rotate: asset_server.load("toolbar_icons/Rotate.png"),
        scale: asset_server.load("toolbar_icons/Scale.png"),
        add_entity: asset_server.load("toolbar_icons/Plus.png"),
        add_prefab: asset_server.load("toolbar_icons/Add Prefab.png"),
    });
}