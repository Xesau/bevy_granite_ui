use bevy::prelude::*;

pub struct FullscreenPlugin;

impl Plugin for FullscreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<FullscreenState>()
            .add_systems(Update, update_node_fullscreen_display)
        ;
    }
}

#[derive(States, Default, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum FullscreenState {
    #[default]
    Normal,
    Fullscreen,
}

#[derive(Component, Default, Clone, Copy)]
pub struct NodeFullscreenDisplay {
    pub normal_display: Display,
    pub fullscreen_display: Display,
}

impl NodeFullscreenDisplay {
    pub fn new(normal_display: Display, fullscreen_display: Display) -> Self {
        Self { normal_display, fullscreen_display }
    }
}

pub fn update_node_fullscreen_display(
    mut nodes: Query<(&NodeFullscreenDisplay, &mut Node)>,
    fullscreen_state: Res<State<FullscreenState>>,
) {
    for (node_fullscreen_display, mut node) in nodes.iter_mut() {
        node.display = match fullscreen_state.get() {
            FullscreenState::Normal => {
                node_fullscreen_display.normal_display
            }
            FullscreenState::Fullscreen => {
                node_fullscreen_display.fullscreen_display
            }
        };
    }
}