use crate::ui::{
    EditorUiElement,
    ToolButtonAction,
    colors::{EditorBackgroundColor, EditorColor, EditorTextColor},
};
use bevy::prelude::*;

/// This macro is used to create a reactive element.
/// It is currently not very performant because it despawns and re-inserts the entire entity every time the data changes.
macro_rules! reactive_element {
    ($name:ident, $system_name:ident, $bundle:expr) => {
        pub fn $system_name(
            mut commands: Commands,
            query: Query<(Entity, &$name), Changed<$name>>,
        ) {
            for (entity, entity_data) in query.iter() {
                let mut entity_commands = commands.entity(entity);
                entity_commands.despawn_children();

                let bundle = $bundle(entity_data);
                entity_commands.insert(bundle);
            }
        }
    };
}

#[derive(Component)]
#[require(EditorUiElement)]
#[require(Node {
    display: Display::Flex,
    flex_direction: FlexDirection::Column,
    height: Val::Percent(100.0),
    width: Val::Percent(100.0),
    left: Val::Px(0.0),
    top: Val::Px(0.0),
    position_type: PositionType::Absolute,
    ..default()
})]
pub struct EditorUi;

#[derive(Component)]
#[require(EditorUiElement)]
#[require(EditorBackgroundColor(EditorColor::MenuBar, None, None))]
#[require(Node {
    display: Display::Flex,
    height: Val::Px(39.0),
    width: Val::Percent(100.0),
    column_gap: Val::Px(2.0),
    padding: UiRect::new(Val::Px(15.0), Val::Px(15.0), Val::Px(5.0), Val::Px(5.0)),
    ..default()
})]
pub struct MenuBar;

#[derive(Component)]
pub struct MenuBarButton {
    pub text: String,
}

reactive_element!(
    MenuBarButton,
    reactive_menu_bar_button,
    |menu_bar_button: &MenuBarButton| {
        (
            EditorUiElement,
            Button,
            Node {
                display: Display::Flex,
                height: Val::Px(29.0),
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.0),
                padding: UiRect::new(Val::Px(15.0), Val::Px(15.0), Val::Px(6.0), Val::Px(6.0)),
                ..default()
            },
            BorderRadius::all(Val::Px(4.0)),
            EditorBackgroundColor(EditorColor::MenuBar, Some(EditorColor::MenuBarButtonHover), None),
            children![(
                EditorUiElement,
                Text::new(&menu_bar_button.text),
                EditorTextColor(EditorColor::Text, None, None),
                TextFont {
                    font_size: 13.0,
                    ..default()
                }
            )],
        )
    }
);

#[derive(Component)]
#[require(EditorUiElement)]
#[require(EditorBackgroundColor(EditorColor::TabBar, None, None))]
#[require(Node {
    display: Display::Flex,
    height: Val::Px(50.0),
    width: Val::Percent(100.0),
    column_gap: Val::Px(10.0),
    padding: UiRect::new(Val::Px(15.0), Val::Px(15.0), Val::Px(12.0), Val::Px(0.0)),
    ..default()
})]
pub struct TabBar;

#[derive(Component)]
pub struct Tab {
    pub name: String,
    pub is_active: bool,
}

reactive_element!(Tab, reactive_tab, |tab: &Tab| {
    (
        EditorUiElement,
        Button,
        Node {
            display: Display::Flex,
            height: Val::Px(38.0),
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.0),
            padding: UiRect::new(Val::Px(15.0), Val::Px(15.0), Val::Px(10.0), Val::Px(10.0)),
            ..default()
        },
        EditorBackgroundColor(if tab.is_active { EditorColor::TabActive } else { EditorColor::TabBar }, Some(if tab.is_active { EditorColor::TabActive } else { EditorColor::TabHover }), None),
        BorderRadius::new(Val::Px(4.0), Val::Px(4.0), Val::Px(0.0), Val::Px(0.0)),
        children![
            (
                EditorUiElement,
                Node {
                    display: Display::Flex,
                    height: Val::Px(18.0),
                    width: Val::Px(18.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                Text::new("o"),
                BackgroundColor(Srgba::hex("#378D09").unwrap().into()),
                BorderRadius::all(Val::Px(2.0)),
            ),
            (
                EditorUiElement,
                Text::new(&tab.name),
                EditorTextColor(EditorColor::Text, None, None),
                TextFont {
                    font_size: 13.0,
                    ..default()
                }
            ),
            (
                EditorUiElement,
                Button,
                Node {
                    display: if tab.is_active { Display::Flex } else { Display::None },
                    height: Val::Px(18.0),
                    width: Val::Px(18.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![
                    (
                        EditorUiElement,
                        Text::new("x"),
                        EditorTextColor(EditorColor::Text, None, None),
                        TextFont {
                            font_size: 13.0,
                            ..default()
                        }
                    )
                ]
            )
        ],
    )
});

#[derive(Component)]
#[require(EditorUiElement)]
#[require(Node {
    display: Display::Flex,
    padding: UiRect::new(Val::Px(15.0), Val::Px(15.0), Val::Px(6.0), Val::Px(6.0)),
    align_items: AlignItems::Center,
    justify_content: JustifyContent::SpaceBetween,
    ..default()
})]
#[require(EditorBackgroundColor(EditorColor::Background, None, None))]
pub struct ToolBar;

#[derive(Component)]
#[require(EditorUiElement)]
#[require(Node {
    display: Display::Flex,
    align_items: AlignItems::Center,
    column_gap: Val::Px(10.0),
    ..default()
})]
pub struct ToolButtons;

#[derive(Component)]
#[require(EditorUiElement)]
#[require(Node {
    display: Display::Flex,
    height: Val::Px(38.0),
    align_items: AlignItems::Center,
    column_gap: Val::Px(10.0),
    padding: UiRect::new(Val::Px(15.0), Val::Px(15.0), Val::Px(10.0), Val::Px(10.0)),
    ..default()
})]
#[require(BorderRadius::all(Val::Px(4.0)))]
#[require(EditorBackgroundColor(EditorColor::Button, None, None))]
pub struct ToolButtonGroup;

#[derive(Component)]
#[require(EditorUiElement)]
#[require(Node {
    display: Display::Block,
    height: Val::Px(14.0),
    width: Val::Px(1.0),
    ..default()
})]
#[require(EditorBackgroundColor(EditorColor::Background, None, None))]
pub struct ToolButtonSeparator;

#[derive(Component)]
pub struct ToolButton {
    pub action: ToolButtonAction,
    pub is_active: bool,
    pub icon: Handle<Image>,
}

reactive_element!(
    ToolButton,
    reactive_tool_button,
    |tool_button: &ToolButton| {
        (
            EditorUiElement,
            Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Px(25.0),
                height: Val::Px(25.0),
                ..default()
            },
            EditorBackgroundColor(if tool_button.is_active { EditorColor::Background } else { EditorColor::Button }, None, None),
            BorderRadius::all(Val::Px(3.0)),
            children![(
                EditorUiElement,
                Node {
                    width: Val::Px(20.0),
                    height: Val::Px(20.0),
                    ..default()
                },
                ImageNode {
                    image: tool_button.icon.clone(),
                    ..default()
                }
            )],
        )
    }
);

#[derive(Component)]
pub struct StatusBar {
    pub text: String,
}

reactive_element!(
    StatusBar,
    reactive_status_bar,
    |status_bar: &StatusBar| {
        (
            EditorUiElement,
            Node {
                display: Display::Flex,
                height: Val::Px(20.0),
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            EditorBackgroundColor(EditorColor::Background, None, None),
            children![
                (
                    EditorUiElement,
                    Text::new(format!("Status: {}", &status_bar.text)),
                    EditorTextColor(EditorColor::Text, None, None),
                    TextFont {
                        font_size: 13.0,
                        ..default()
                    }
                )
            ]
        )
    }
);

#[derive(Component)]
pub struct CameraPreview;

reactive_element!(CameraPreview, reactive_camera_preview, |_camera_preview: &CameraPreview| {
    (
        EditorUiElement,
        Node {
            display: Display::Flex,
            flex_grow: 1.0,
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    )
});

#[derive(Component)]
pub struct FpsCounter {
    pub fps: Option<f32>,
}

reactive_element!(
    FpsCounter,
    reactive_fps_counter,
    |fps_counter: &FpsCounter| {
        (
            EditorUiElement,
            Node {
                display: Display::Flex,
                height: Val::Px(20.0),
                justify_self: JustifySelf::End,
                align_items: AlignItems::Center,
                ..default()
            },
            Text::new(format!("FPS: {}", fps_counter.fps.map(|fps| (fps as u32).to_string()).unwrap_or("--".to_string()))),
            TextFont {
                font_size: 13.0,
                ..default()
            }
        )
    }
);