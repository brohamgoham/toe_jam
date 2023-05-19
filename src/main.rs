use bevy::{
    asset::AssetPathId,
    prelude::*,
    render::camera::ScalingMode,
    text, window::WindowMode, a11y::AccessibilityNode,
    a11y::{
        accesskit::{NodeBuilder, Role},
    }, input::mouse::{MouseScrollUnit, MouseWheel},
};
use bevy_text::{Text, TextSection, TextStyle};

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct ScrollingList{
    position: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                resizable: true,
                title: "MoHam ToeJam".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_startup_systems((setup, spawn_player))
        .add_system(move_player)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera_bundle = Camera2dBundle::default();

    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn(camera_bundle);
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        }
    )
        .with_children(|parent| {
           // left vertical fill (border)
           parent
           .spawn(NodeBundle {
               style: Style {
                   size: Size::width(Val::Px(200.0)),
                   border: UiRect::all(Val::Px(2.0)),
                   ..default()
               },
               background_color: Color::rgb(0.65, 0.65, 0.65).into(),
               ..default()
           })
           .with_children(|parent| {
               // left vertical fill (content)
               parent
                   .spawn(NodeBundle {
                       style: Style {
                           size: Size::width(Val::Percent(100.0)),
                           ..default()
                       },
                       background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                       ..default()
                   })
                   .with_children(|parent| {
                       // text
                       parent.spawn((
                           TextBundle::from_section(
                               "Text Example",
                               TextStyle {
                                   font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                   font_size: 30.0,
                                   color: Color::WHITE,
                               },
                           )
                           .with_style(Style {
                               margin: UiRect::all(Val::Px(5.0)),
                               ..default()
                           }),
                           // Because this is a distinct label widget and
                           // not button/list item text, this is necessary
                           // for accessibility to treat the text accordingly.
                           Label,
                       ));
                   });
           });
       // right vertical fill
       parent
           .spawn(NodeBundle {
               style: Style {
                   flex_direction: FlexDirection::Column,
                   justify_content: JustifyContent::Center,
                   align_items: AlignItems::Center,
                   size: Size::width(Val::Px(200.0)),
                   ..default()
               },
               background_color: Color::rgb(0.15, 0.15, 0.15).into(),
               ..default()
           })
           .with_children(|parent| {
               // Title
               parent.spawn((
                   TextBundle::from_section(
                       "Scrolling list",
                       TextStyle {
                           font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                           font_size: 25.,
                           color: Color::WHITE,
                       },
                   )
                   .with_style(Style {
                       size: Size::height(Val::Px(25.)),
                       ..default()
                   }),
                   Label,
               ));
               // List with hidden overflow
               parent
                   .spawn(NodeBundle {
                       style: Style {
                           flex_direction: FlexDirection::Column,
                           align_self: AlignSelf::Stretch,
                           size: Size::height(Val::Percent(50.0)),
                           overflow: Overflow::Hidden,
                           ..default()
                       },
                       background_color: Color::rgb(0.10, 0.10, 0.10).into(),
                       ..default()
                   })
                   .with_children(|parent| {
                       // Moving panel
                       parent
                           .spawn((
                               NodeBundle {
                                   style: Style {
                                       flex_direction: FlexDirection::Column,
                                       max_size: Size::UNDEFINED,
                                       align_items: AlignItems::Center,
                                       ..default()
                                   },
                                   ..default()
                               },
                               ScrollingList::default(),
                               AccessibilityNode(NodeBuilder::new(Role::List)),
                           ))
                           .with_children(|parent| {
                               // List items
                               for i in 0..30 {
                                   parent.spawn((
                                       TextBundle::from_section(
                                           format!("Item {i}"),
                                           TextStyle {
                                               font: asset_server
                                                   .load("fonts/FiraSans-Bold.ttf"),
                                               font_size: 20.,
                                               color: Color::WHITE,
                                           },
                                       ),
                                       Label,
                                       AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                   ));
                               }
                           });
                   });
           });
       parent
           .spawn(NodeBundle {
               style: Style {
                   size: Size::new(Val::Px(200.0), Val::Px(200.0)),
                   position_type: PositionType::Absolute,
                   position: UiRect {
                       left: Val::Px(210.0),
                       bottom: Val::Px(10.0),
                       ..default()
                   },
                   border: UiRect::all(Val::Px(20.0)),
                   ..default()
               },
               background_color: Color::rgb(0.4, 0.4, 1.0).into(),
               ..default()
           })
           .with_children(|parent| {
               parent.spawn(NodeBundle {
                   style: Style {
                       size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                       ..default()
                   },
                   background_color: Color::rgb(0.8, 0.8, 1.0).into(),
                   ..default()
               });
           });
       // render order test: reddest in the back, whitest in the front (flex center)
       parent
           .spawn(NodeBundle {
               style: Style {
                   size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                   position_type: PositionType::Absolute,
                   align_items: AlignItems::Center,
                   justify_content: JustifyContent::Center,
                   ..default()
               },
               ..default()
           })
           .with_children(|parent| {
               parent
                   .spawn(NodeBundle {
                       style: Style {
                           size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                           ..default()
                       },
                       background_color: Color::rgb(1.0, 0.0, 0.0).into(),
                       ..default()
                   })
                   .with_children(|parent| {
                       parent.spawn(NodeBundle {
                           style: Style {
                               // Take the size of the parent node.
                               size: Size::all(Val::Percent(100.)),
                               position_type: PositionType::Absolute,
                               position: UiRect {
                                   left: Val::Px(20.0),
                                   bottom: Val::Px(20.0),
                                   ..default()
                               },
                               ..default()
                           },
                           background_color: Color::rgb(1.0, 0.3, 0.3).into(),
                           ..default()
                       });
                       parent.spawn(NodeBundle {
                           style: Style {
                               size: Size::all(Val::Percent(100.)),
                               position_type: PositionType::Absolute,
                               position: UiRect {
                                   left: Val::Px(40.0),
                                   bottom: Val::Px(40.0),
                                   ..default()
                               },
                               ..default()
                           },
                           background_color: Color::rgb(1.0, 0.5, 0.5).into(),
                           ..default()
                       });
                       parent.spawn(NodeBundle {
                           style: Style {
                               size: Size::all(Val::Percent(100.)),
                               position_type: PositionType::Absolute,
                               position: UiRect {
                                   left: Val::Px(60.0),
                                   bottom: Val::Px(60.0),
                                   ..default()
                               },
                               ..default()
                           },
                           background_color: Color::rgb(1.0, 0.7, 0.7).into(),
                           ..default()
                       });
                       // alpha test
                       parent.spawn(NodeBundle {
                           style: Style {
                               size: Size::all(Val::Percent(100.)),
                               position_type: PositionType::Absolute,
                               position: UiRect {
                                   left: Val::Px(80.0),
                                   bottom: Val::Px(80.0),
                                   ..default()
                               },
                               ..default()
                           },
                           background_color: Color::rgba(1.0, 0.9, 0.9, 0.4).into(),
                           ..default()
                       });
                   });
           });
       // bevy logo (flex center)
       parent
           .spawn(NodeBundle {
               style: Style {
                   size: Size::width(Val::Percent(100.)),
                   position_type: PositionType::Absolute,
                   justify_content: JustifyContent::Center,
                   align_items: AlignItems::FlexStart,
                   ..default()
               },
               ..default()
           })
           .with_children(|parent| {
               // bevy logo (image)
               parent
                   .spawn(ImageBundle {
                       style: Style {
                           size: Size::width(Val::Px(500.0)),
                           ..default()
                       },
                       ..default()
                   })
                   .with_children(|parent| {
                       // alt text
                       parent
                           .spawn(TextBundle::from_section("Bevy logo", TextStyle::default()));
                   });
           });
   });
}


fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        },
    ));
}

fn move_player(keys: Res<Input<KeyCode>>, mut player_query: Query<&mut Transform, With<Player>>) {
    let mut direction = Vec2::ZERO;
    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        direction.y += 1.;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction.y -= 1.;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1.;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.;
    }
    if direction == Vec2::ZERO {
        return;
    }

    let move_speed = 0.13;
    let move_delta = (direction * move_speed).extend(0.);

    for mut transform in player_query.iter_mut() {
        transform.translation += move_delta;
    }
}

fn title(mut commands: Commands) {
    commands.spawn(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Welcome to Toe Jame".to_string(),
                    style: TextStyle {
                        font: Handle::default(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                }
            ],
            alignment: TextAlignment::Center,
            linebreak_behaviour: bevy_text::BreakLineOn::AnyCharacter,
        },
        ..Default::default()
    });
}

fn mouse_scrolling(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.0);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.0,
                MouseScrollUnit::Pixel => mouse_wheel_event.y
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.0);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}