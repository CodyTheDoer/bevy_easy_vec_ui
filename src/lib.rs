use bevy::prelude::*;

use std::time::Duration;
pub struct BevyEasyVecUiPlugin {
    font_path: String,
    camera_layer: isize,
    title_font_size: f32,
    title: String,
    data_font_size: f32,
    data_vec_left: Vec<String>,
    data_vec_right: Vec<String>,
}

impl BevyEasyVecUiPlugin {
    pub fn init(font_path: &str) -> Self {
        Self {
            font_path: String::from(font_path),
            camera_layer: -1,
            title_font_size: 42.0,
            title: String::from("Default Title: Not Set"),
            data_font_size: 12.0,
            data_vec_left: Vec::new(),
            data_vec_right: Vec::new(),
        }
    }

    pub fn camera_layer(mut self, layer: isize) -> Self {
        self.camera_layer = layer;
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = String::from(title);
        self
    }

    pub fn title_font_size(mut self, size: f32) -> Self {
        self.title_font_size = size;
        self
    }

    pub fn data_font_size(mut self, size: f32) -> Self {
        self.data_font_size = size;
        self
    }

    pub fn build(self) -> BevyEasyVecUiPlugin {
        BevyEasyVecUiPlugin {
            font_path: self.font_path,
            camera_layer: self.camera_layer,
            title_font_size: self.title_font_size,
            title: self.title,
            data_font_size: self.data_font_size,
            data_vec_left: self.data_vec_left,
            data_vec_right: self.data_vec_right,
        }
    }
}

impl Plugin for BevyEasyVecUiPlugin {
    fn build(&self, app: &mut App) {
        
        app.insert_resource(EasyVecUi {
            font_path: self.font_path.clone(),
            camera_layer: self.camera_layer.clone(),
            title_font_size: self.title_font_size.clone(),
            title: self.title.clone(),
            data_font_size: self.data_font_size.clone(),
            data_vec_left: self.data_vec_left.clone(),
            data_vec_right: self.data_vec_right.clone(),
        });
        app.insert_resource(EasyVecUiFonts::new());
        app.insert_resource(EasyVecUiUpdateTimer(Timer::new(Duration::from_millis(250), TimerMode::Repeating)));
        app.add_systems(Startup, setup_ui);
        app.add_systems(Update, ui_update_system);
    }
}

pub fn setup_ui(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    user_supplied: Res<EasyVecUi>,
    mut fonts: ResMut<EasyVecUiFonts>,
) {
    // Load and setup fonts
    let font = asset_server.load(&user_supplied.font_path);
    let title_display = TextStyle {
        font: font.clone(),
        font_size: user_supplied.title_font_size,
        ..default()
    };
    let data_display = TextStyle {
        font: font,
        font_size: user_supplied.data_font_size,
        ..default()
    };
    fonts.fonts.push(title_display);
    fonts.fonts.push(data_display);

    // Set up a 2D camera for the Ui
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            camera: Camera {
                order: user_supplied.camera_layer, // Render before the 3D scene
                ..default()
            },
            ..default()
        },
        EasyVecUiCamera,
    ));

    // Title: Create a screen-sized Ui node for the centered title
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                align_items: AlignItems::Center, // Align the title text to the center vertically
                justify_content: JustifyContent::Center, // Center the title text horizontally
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(10.0), // Height is 10% of the screen, to occupy the top area
                top: Val::Percent(0.0),     // Position it at the very top
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            &user_supplied.title,
                            fonts.fonts[0].clone(),
                        )],
                        ..default()
                    },
                    ..default()
                },
                EasyVecUiTitleText, // Tag the title text so it can be updated later
            ));
        });

    // HUD: Create a Ui node to display connected players
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                align_items: AlignItems::FlexStart,     // Align items from the top of the node
                flex_direction: FlexDirection::Column,  // Stack items vertically
                justify_content: JustifyContent::FlexStart, // Align top-left
                position_type: PositionType::Absolute,
                bottom: Val::Percent(0.0), // Position at the bottom of the screen
                left: Val::Percent(0.0),   // Align it to the left of the screen
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Tag this node so it can be dynamically updated
            parent.spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column, // Stack items vertically
                        ..default()
                    },
                    ..default()
                },
                EasyVecUiNodeLeft, // Tag the node for easy updates later
            ));
        });
    
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                align_items: AlignItems::FlexStart,     // Align items from the top of the node
                flex_direction: FlexDirection::Column,  // Stack items vertically
                justify_content: JustifyContent::FlexEnd, // Align top-Right
                position_type: PositionType::Absolute,
                bottom: Val::Percent(0.0), // Position at the bottom of the screen
                right: Val::Percent(0.0),   // Align it to the left of the screen
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Tag this node so it can be dynamically updated
            parent.spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column, // Stack items vertically
                        ..default()
                    },
                    ..default()
                },
                EasyVecUiNodeRight, // Tag the node for easy updates later
            ));
        });
}

pub fn ui_update_system(
    time: Res<Time>,
    mut timer: ResMut<EasyVecUiUpdateTimer>,
    commands: Commands,
    fonts: Res<EasyVecUiFonts>,
    query_left: Query<Entity, With<EasyVecUiNodeLeft>>,
    query_right: Query<Entity, With<EasyVecUiNodeRight>>,
    user_supplied: Res<EasyVecUi>,
) {
    // Check if the timer has finished
    if timer.0.tick(time.delta()).finished() {
        // Call the function to update the connected players Ui
        update_ui(user_supplied, query_left, query_right, commands, fonts);
    }
}

pub fn update_ui(
    user_supplied: Res<EasyVecUi>,
    query_left: Query<Entity, With<EasyVecUiNodeLeft>>,
    query_right: Query<Entity, With<EasyVecUiNodeRight>>,
    mut commands: Commands,
    fonts: Res<EasyVecUiFonts>,
) {    
    if let Ok(data_node_container_left) = query_left.get_single() {
        commands.entity(data_node_container_left).despawn_descendants();

        // Iterate over each player and create a row for each one
        for status in user_supplied.data_vec_left.iter() {
            // Spawn a new node for each player, representing a row
            commands.entity(data_node_container_left).with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        background_color: BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.25)), // Semi-transparent dark background
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row, // Arrange items horizontally within the row
                            align_items: AlignItems::Center,    // Center items vertically within the row
                            margin: UiRect::all(Val::Px(5.0)),  // Add some spacing between rows
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|row| {
                        // Player ID text
                        row.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    format!("{}", status),
                                    fonts.fonts[1].clone(),
                                )],
                                ..default()
                            },
                            style: Style {
                                margin: UiRect::right(Val::Px(10.0)), // Spacing between player ID and other fields
                                ..default()
                            },
                            ..default()
                        });
                    });
            });
        }
    }
    
    if let Ok(data_node_container_right) = query_right.get_single() {
        commands.entity(data_node_container_right).despawn_descendants();

        // Iterate over each player and create a row for each one
        for status in user_supplied.data_vec_right.iter() {
            // Spawn a new node for each player, representing a row
            commands.entity(data_node_container_right).with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        background_color: BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.25)), // Semi-transparent dark background
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row, // Arrange items horizontally within the row
                            align_items: AlignItems::Center,    // Center items vertically within the row
                            margin: UiRect::all(Val::Px(5.0)),  // Add some spacing between rows
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|row| {
                        // Player ID text
                        row.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    format!("{}", status),
                                    fonts.fonts[1].clone(),
                                )],
                                ..default()
                            },
                            style: Style {
                                margin: UiRect::right(Val::Px(10.0)), // Spacing between player ID and other fields
                                ..default()
                            },
                            ..default()
                        });
                    });
            });
        }
    }
}

#[derive(Asset, Component, TypePath)]
pub struct EasyVecUiCamera;

#[derive(Component)]
pub struct EasyVecUiNodeLeft;

#[derive(Component)]
pub struct EasyVecUiNodeRight;

#[derive(Clone, Resource)]
pub struct EasyVecUi {
    pub font_path: String,
    pub camera_layer: isize,
    pub title_font_size: f32,
    pub title: String,
    pub data_font_size: f32,
    pub data_vec_left: Vec<String>,
    pub data_vec_right: Vec<String>,
}

impl EasyVecUi {
    pub fn inject_vec_left(&mut self, vec: Vec<String>) {
        self.data_vec_left = vec;
    }

    pub fn inject_vec_right(&mut self, vec: Vec<String>) {
        self.data_vec_right = vec;
    }
}

#[derive(Resource)]
pub struct EasyVecUiFonts {
    pub fonts: Vec<TextStyle>,
}

impl EasyVecUiFonts {
    pub fn new() -> Self {
        let fonts: Vec<TextStyle> = Vec::new();
        EasyVecUiFonts {
            fonts,
        }
    }
}

#[derive(Component)]
pub struct EasyVecUiStatusText;

#[derive(Component)]
pub struct EasyVecUiTitleText;

#[derive(Resource)]
pub struct EasyVecUiUpdateTimer(pub Timer);