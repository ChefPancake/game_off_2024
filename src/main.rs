use bevy::{
    input::mouse::MouseMotion, prelude::*, sprite::MaterialMesh2dBundle, window::{
        WindowMode, WindowResized, WindowResolution
    }
};

/*
    to build:
    cargo build --target wasm32-unknown-unknown --release
    wasm-bindgen --out-dir .\out\ --target web .\target\wasm32-unknown-unknown\release\game_off_2024.wasm
*/

mod data;
mod clickable;
mod handles;
mod core;
mod progress;
mod audio;

use data::*;
use clickable::*;
use handles::*;
use core::*;
use progress::*;
use audio::*;

fn main() {
    App::new()
    .insert_resource(LastInputType::Mouse)
    .insert_resource(StopScrolling::default())
    .insert_resource(TargetLilGuy::default())
    .insert_resource(CurrentInfoPage::default())
    .insert_resource(CorrectSubmissions::default())
    .add_plugins((
        DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "KEEP YOUR SEA CRITS".to_string(),
                    fit_canvas_to_parent: true,
                    resolution: WindowResolution::new(WINDOW_RESOLUTION.x, WINDOW_RESOLUTION.y),
                    ..default()
                }),
                ..default()
            }
        ).set(
            AssetPlugin {
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            }
        ),
        HandlesPlugin,
        ProgressPlugin,
        AudioPlugin,
        CorePlugin,
        ClickablePlugin,
    ))
    .add_systems(PreStartup, (
        spawn_camera,
    ))
    .add_systems(Update, (
        handle_loading_completed,
    ).run_if(in_state(GameState::Loading)))
    .add_systems(OnEnter(GameState::Title), (
        remove_entities::<GameItem>,
        spawn_title_screen,
        spawn_start_button,
    ))
    .add_systems(OnExit(GameState::Title), (
        remove_entities::<TitleScreen>,
        remove_start_button,
        setup_ui_buttons,
        spawn_faceplate,
        spawn_background,
        spawn_border_blocks,
        spawn_monitors,
        spawn_cursor,
        reset_submissions,
    ))
    .add_systems(OnExit(GameState::Title), (
        spawn_lilguys,
    ).after(spawn_background))
    .add_systems(OnEnter(GameState::Game), (
        publish_default_event::<LilGuyDeselected>,
        choose_target_lilguy,
    ))
    .add_systems(OnEnter(GameState::Game), (
        update_mission_monitor
    ).after(choose_target_lilguy))
    .add_systems(Update, (
        handle_scroll_key_input,
        handle_change_page,
    ).run_if(in_state(GameState::Game)))
    .add_systems(PostUpdate, (
        update_cursor,
        handle_scrolling,
        update_faceplate,
    ).run_if(in_state(GameState::Game)))
    .add_systems(OnExit(GameState::Game), (
        reset_cursor_vis,
    ))
    .add_systems(OnEnter(GameState::NextLevel), (
        remove_target_lilguy,
    ))
    .add_systems(OnEnter(GameState::NextLevel), (
        spawn_nextmissionscreen,
        spawn_nextlevel_message_buttons,
    ).after(remove_target_lilguy))
    .add_systems(OnExit(GameState::NextLevel), (
        remove_entities::<MessageBox>,
        remove_entities::<MessageOverlay>,
        remove_entities::<MessageButton>,
    ))
    .add_systems(OnEnter(GameState::GameWin), (
        spawn_winscreen,
        spawn_end_message_buttons,
    ))
    .add_systems(OnExit(GameState::GameWin), (
        remove_entities::<MessageBox>,
        remove_entities::<MessageOverlay>,
        remove_entities::<MessageButton>,
        remove_entities::<LilGuy>,
        reset_background_position,
        reset_submissions,
    ))
    .add_systems(OnExit(GameState::GameWin), (
        spawn_lilguys,
    ).after(remove_entities::<LilGuy>))
    .add_systems(OnEnter(GameState::GameOver), (
        spawn_losescreen,
        spawn_end_message_buttons,
    ))
    .add_systems(OnExit(GameState::GameOver), (
        remove_entities::<MessageBox>,
        remove_entities::<MessageOverlay>,
        remove_entities::<MessageButton>,
        remove_entities::<LilGuy>,
        reset_background_position,
        reset_submissions,
    ))
    .add_systems(OnExit(GameState::GameOver), (
        spawn_lilguys,
    ).after(remove_entities::<LilGuy>))
    .add_systems(Update, (
        resize_foreground,
        deselect_on_esc,
        toggle_fullscreen,
    ))
    .add_systems(PostUpdate, (
        handle_lilguy_selected,
        handle_lilguy_deselected,
        handle_lilguy_submitted,
        handle_mouse_motion,
    ))
    .run();
}

fn toggle_fullscreen(
    key_input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
) {
    let alts = [ KeyCode::AltLeft, KeyCode::AltRight ];
    if key_input.just_pressed(KeyCode::Enter) && key_input.any_pressed(alts) {
        for mut window in &mut windows {
            window.mode = 
                match window.mode {
                    WindowMode::BorderlessFullscreen => WindowMode::Windowed,
                    _ => WindowMode::BorderlessFullscreen,
                };
        }
    }
}

#[derive(Component)]
struct GameItem;

fn remove_entities<T: Component>(
    mut commands: Commands,
    items: Query<Entity, With<T>>,
) {
    for item in &items {
        commands.entity(item).despawn_recursive();
    }
}


#[derive(Component)]
struct CursorImage;

fn spawn_cursor(
    images: Res<ImageHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let Some(image) = &images.cursor else { return; };
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(CURSOR_IMAGE_SIZE.x, CURSOR_IMAGE_SIZE.y)).into(),
            material: materials.add(image.clone()),
            ..default()
        },
        CursorImage,
        GameItem,
    ));
}

#[derive(Resource, Eq, PartialEq)]
enum LastInputType {
    Mouse,
    Touch,
}

fn handle_mouse_motion(
    mut mouse_motion: EventReader<MouseMotion>,
    mut last_input: ResMut<LastInputType>,
) {
    if mouse_motion.is_empty() {
        return;
    }
    mouse_motion.clear();
    *last_input = LastInputType::Mouse;
}

fn update_cursor(
    mut last_input: ResMut<LastInputType>,
    touch_input: Res<Touches>,
    mut cursor: Query<(&mut Transform, &mut Visibility), With<CursorImage>>,
    mut windows: Query<&mut Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    // if there's any touch input, hide everything
    if touch_input.any_just_pressed() || *last_input == LastInputType::Touch {
        *last_input = LastInputType::Touch;
        for (_, mut cursor_vis) in &mut cursor {
            *cursor_vis = Visibility::Hidden;
        }
        return;
    }

    let Ok(mut window) = windows.get_single_mut() else { 
        return; 
    }; 
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok((camera, camera_transform)) = cameras.get_single() else {
        return;
    };
    let Some(cursor_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    let cursor_in_porthole =
        FOREGROUND_PORTHOLE_CENTER_POS.distance_squared(cursor_pos) < (FOREGROUND_PORTHOLE_RAD * FOREGROUND_PORTHOLE_RAD);

    for (mut cursor_trans, mut cursor_vis) in &mut cursor {
        window.cursor.visible = !cursor_in_porthole;
        *cursor_vis = if cursor_in_porthole { Visibility::Visible } else { Visibility::Hidden };
        cursor_trans.translation = (cursor_pos + CURSOR_IMAGE_OFFSET).extend(Z_POS_CURSOR);
    }
}

fn reset_cursor_vis(
    mut windows: Query<&mut Window>,
) {
    let Ok(mut window) = windows.get_single_mut() else { 
        return; 
    };
    window.cursor.visible = true;
}

#[derive(Resource, Default)]
struct CorrectSubmissions {
    ids: [Option<u8>; 2]
}

fn reset_submissions(
    mut submissions: ResMut<CorrectSubmissions>,
) {
    *submissions = default();
}

#[derive(Component)]
struct MessageBox;

fn spawn_winscreen(
    images: Res<ImageHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let Some(image) = &images.win_screen else { return; };
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(MESSAGE_BOX_IMAGE_SIZE.x, MESSAGE_BOX_IMAGE_SIZE.y)).into(),
            material: materials.add(image.clone()),
            transform: Transform::from_translation(Vec2::ZERO.extend(Z_POS_MESSAGE_BOX)),
            ..default()
        },
        MessageBox,
        GameItem,
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(FOREGROUND_IMAGE_SIZE.x, FOREGROUND_IMAGE_SIZE.y)).into(),
            material: materials.add(ColorMaterial::from_color(BACKGROUND_OVERLAY_COLOR)),
            transform: Transform::from_translation(Vec2::ZERO.extend(Z_POS_MESSAGE_OVERLAY)),
            ..default()
        },
        MessageOverlay,
        GameItem,
    ));
}

fn reset_background_position(
    mut bgs: Query<&mut Transform, With<Background>>,
) {
    for mut bg in &mut bgs {
        bg.translation.x = 0.0;
    }
}

fn publish_default_event<E: Event + Default>(
    mut event_writer: EventWriter<E>,
) {
    event_writer.send_default();
}

#[derive(Component)]
struct MessageButton;

fn spawn_end_message_buttons(
    mut commands: Commands
) {
    spawn_message_buttons(&mut commands, ActionTypes::StartGame);
}

fn spawn_nextlevel_message_buttons(
    mut commands: Commands
) {
    spawn_message_buttons(&mut commands, ActionTypes::StartNextLevel);
}

fn spawn_message_buttons(
    commands: &mut Commands,
    action: ActionTypes
) {
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(MESSAGE_CONTINUE_BUTTON_POS.extend(0.0)),
            ..default()
        },
        Clickable {
            area: ClickArea::Rectangular(MESSAGE_CONTINUE_BUTTON_SIZE),
            action,
            active_on: ActiveStates::Message,
            behavior: ClickBehaviors::SingleClick,
        },
        MessageButton,
        GameItem,
    ));
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(MESSAGE_EXIT_BUTTON_POS.extend(0.0)),
            ..default()
        },
        Clickable {
            area: ClickArea::Rectangular(MESSAGE_EXIT_BUTTON_SIZE),
            action: ActionTypes::ReturnToTitle,
            active_on: ActiveStates::Message,
            behavior: ClickBehaviors::SingleClick,
        },
        MessageButton,
        GameItem,
    ));
}

fn spawn_losescreen(
    images: Res<ImageHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let Some(image) = &images.lose_screen else { return; };
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(MESSAGE_BOX_IMAGE_SIZE.x, MESSAGE_BOX_IMAGE_SIZE.y)).into(),
            material: materials.add(image.clone()),
            transform: Transform::from_translation(Vec2::ZERO.extend(Z_POS_MESSAGE_BOX)),
            ..default()
        },
        MessageBox,
        GameItem,
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(FOREGROUND_IMAGE_SIZE.x, FOREGROUND_IMAGE_SIZE.y)).into(),
            material: materials.add(ColorMaterial::from_color(BACKGROUND_OVERLAY_COLOR)),
            transform: Transform::from_translation(Vec2::ZERO.extend(Z_POS_MESSAGE_OVERLAY)),
            ..default()
        },
        MessageOverlay,
        GameItem,
    ));
}

fn spawn_nextmissionscreen(
    images: Res<ImageHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let Some(image) = &images.next_mission_screen else { return; };
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(MESSAGE_BOX_IMAGE_SIZE.x, MESSAGE_BOX_IMAGE_SIZE.y)).into(),
            material: materials.add(image.clone()),
            transform: Transform::from_translation(Vec2::ZERO.extend(Z_POS_MESSAGE_BOX)),
            ..default()
        },
        MessageBox,
        GameItem,
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(FOREGROUND_IMAGE_SIZE.x, FOREGROUND_IMAGE_SIZE.y)).into(),
            material: materials.add(ColorMaterial::from_color(BACKGROUND_OVERLAY_COLOR)),
            transform: Transform::from_translation(Vec2::ZERO.extend(Z_POS_MESSAGE_OVERLAY)),
            ..default()
        },
        MessageOverlay,
        GameItem,
    ));
}

fn remove_target_lilguy(
    lilguys: Query<(Entity, &LilGuy)>,
    target_lilguy: Res<TargetLilGuy>,
    mut commands: Commands,
    mut lilguy_deselected: EventWriter<LilGuyDeselected>
) {
    let Some(target_lilguy_id) = target_lilguy.target_lilguy_id else { return; };
    for (entity, lilguy) in &lilguys {
        if lilguy.lilguy_id == target_lilguy_id {
            commands.entity(entity).despawn_recursive();
            lilguy_deselected.send(LilGuyDeselected);
        }
    }
}


fn handle_loading_completed(
    mut updates: EventReader<LoadingProgressUpdated>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for update in updates.read() {
        if update.completed == update.total {
            game_state.set(GameState::Title);
        }
    }
}

fn remove_start_button(
    buttons: Query<(Entity, &Clickable)>,
    mut commands: Commands
) {
    for (entity, btn) in &buttons {
        if btn.action == ActionTypes::StartGame {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_start_button(
    mut commands: Commands,
) {
    spawn_button(
        &mut commands,
        FOREGROUND_BOTTOM_MONITOR_IMAGE_POS,
        Clickable {
            area: ClickArea::Rectangular(FOREGROUND_BOTTOM_MONITOR_IMAGE_SIZE),
            action: ActionTypes::StartGame,
            behavior: ClickBehaviors::SingleClick,
            active_on: ActiveStates::Title,
        },
    );
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(BACKGROUND_COLOR),
                ..default()
            },
            ..default()
        },
    );
}

#[derive(Component)]
struct TitleScreen;

fn spawn_title_screen(
    images: Res<ImageHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let Some(title_image) = &images.title_screen else { return; };
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(FOREGROUND_IMAGE_SIZE.x, FOREGROUND_IMAGE_SIZE.y)).into(),
            material: materials.add(title_image.clone()),
            transform: Transform::from_translation(Vec2::ZERO.extend(Z_POS_FACEPLATE)),
            ..default()
        },
        TitleScreen,
    ));
}

#[derive(Resource, Default)]
struct TargetLilGuy {
    target_lilguy_id: Option<u8>,
}

fn choose_target_lilguy(
    submissions: Res<CorrectSubmissions>,
    mut target: ResMut<TargetLilGuy>,
) {
    loop {
        let selected_lilguy: u8 = rand::random::<u8>() % (LILGUYS_COUNT as u8);

        if let Some(id) = submissions.ids[0] {
            if id == selected_lilguy {
                continue;
            }
        }

        if let Some(id) = submissions.ids[1] {
            if id == selected_lilguy {
                continue;
            }
        }

        target.target_lilguy_id = Some(selected_lilguy);
        break;
    }
}

fn handle_lilguy_submitted(
    mut lilguy_submitted: EventReader<LilGuySubmitted>,
    target: Res<TargetLilGuy>,
    mut submissions: ResMut<CorrectSubmissions>,
    mut game_state: ResMut<NextState<GameState>>
) {
    let Some(target_id) = target.target_lilguy_id else { return; };

    for event in lilguy_submitted.read() {
        if event.lilguy_id_guess == target_id {
            if submissions.ids[0].is_none() {
                submissions.ids[0] = Some(target_id);
                game_state.set(GameState::NextLevel);
                continue;
            }
            if submissions.ids[1].is_none() {
                submissions.ids[1] = Some(target_id);
                game_state.set(GameState::NextLevel);
                continue;
            }
            game_state.set(GameState::GameWin);

        } else {
            game_state.set(GameState::GameOver);
        }
    }
}

// don't want to mess with masking stuff - just going to block off the sides
fn spawn_border_blocks(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let mesh = Rectangle::new(FOREGROUND_IMAGE_SIZE.x, FOREGROUND_IMAGE_SIZE.y);
    let material = materials.add(ColorMaterial::from_color(BACKGROUND_COLOR));

    commands.spawn_batch([
        (
            MaterialMesh2dBundle {
                mesh: meshes.add(mesh).into(),
                material: material.clone(),
                transform: Transform::from_translation(Vec3::new(FOREGROUND_IMAGE_SIZE.x, 0.0, Z_POS_MONITORS)),
                ..default()
            },
            GameItem,
        ),
        (
            MaterialMesh2dBundle {
                mesh: meshes.add(mesh).into(),
                material,
                transform: Transform::from_translation(Vec3::new(-FOREGROUND_IMAGE_SIZE.x, 0.0, Z_POS_MONITORS)),
                ..default()
            },
            GameItem,
        ),
    ]);
}

fn setup_ui_buttons(
    mut commands: Commands
) {
    spawn_button(
        &mut commands,
        FOREGROUND_CLOSE_BUTTON_POS,
        Clickable {
            area: ClickArea::Rectangular(FOREGROUND_SMALL_BUTTON_AREA),
            action: ActionTypes::ReturnToTitle,
            behavior: ClickBehaviors::SingleClick,
            active_on: ActiveStates::Game,
        },
    );
    spawn_button(
        &mut commands,
        FOREGROUND_INFO_PAGELEFT_POS,
        Clickable {
            area: ClickArea::Circular(FOREGROUND_INFO_BUTTON_RAD),
            action: ActionTypes::InfoPageLeft,
            behavior: ClickBehaviors::SingleClick,
            active_on: ActiveStates::Game,
        },
    );
    spawn_button(
        &mut commands,
        FOREGROUND_INFO_PAGERIGHT_POS,
        Clickable {
            area: ClickArea::Circular(FOREGROUND_INFO_BUTTON_RAD),
            action: ActionTypes::InfoPageRight,
            behavior: ClickBehaviors::SingleClick,
            active_on: ActiveStates::Game,
        },
    );
    spawn_button(
        &mut commands,
        FOREGROUND_BACK_BUTTON_POS,
        Clickable {
            area: ClickArea::Rectangular(FOREGROUND_BACK_BUTTON_AREA),
            action: ActionTypes::UnZoomLilguy,
            behavior: ClickBehaviors::SingleClick,
            active_on: ActiveStates::Game, 
        }
    );
    spawn_button(
        &mut commands,
        FOREGROUND_SEND_BUTTON_POS,
        Clickable {
            area: ClickArea::Rectangular(FOREGROUND_SEND_BUTTON_AREA),
            action: ActionTypes::SendToLab,
            behavior: ClickBehaviors::SingleClick,
            active_on: ActiveStates::Game,
        },
    );
    spawn_button(
        &mut commands,
        FOREGROUND_LEFT_BUTTON_POS,
        Clickable {
            area: ClickArea::Rectangular(FOREGROUND_SMALL_BUTTON_AREA),
            action: ActionTypes::ScrollLeft,
            behavior: ClickBehaviors::ClickAndHold,
            active_on: ActiveStates::Game,
        },
    );
    spawn_button(
        &mut commands,
        FOREGROUND_RIGHT_BUTTON_POS,
        Clickable {
            area: ClickArea::Rectangular(FOREGROUND_SMALL_BUTTON_AREA),
            action: ActionTypes::ScrollRight,
            behavior: ClickBehaviors::ClickAndHold,
            active_on: ActiveStates::Game,
        },
    );
}

fn spawn_button(
    commands: &mut Commands,
    pos: Vec2, 
    clickable: Clickable, 
) {
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(pos.extend(0.0)),
            ..default()
        },
        clickable,
        GameItem,
    ));
}

#[derive(Resource, Default)]
struct CurrentInfoPage {
    page_idx: usize,
}

fn handle_change_page(
    mut page_changed: EventReader<ChangeInfoPage>,
    images: Res<ImageHandles>,
    mut current_page: ResMut<CurrentInfoPage>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut info_page_images: Query<&mut Handle<ColorMaterial>, With<InfoMonitor>>,
) {
    for event in page_changed.read() {
        let page_adjustment = match event {
            ChangeInfoPage::PageLeft => LILGUYS_COUNT - 1,
            ChangeInfoPage::PageRight => LILGUYS_COUNT + 1,
        };
        current_page.page_idx = (current_page.page_idx + page_adjustment) % LILGUYS_COUNT;
        for mut image in &mut info_page_images {
            let Some(image_handle) = &images.lilguys_info_monitor[current_page.page_idx] else { continue; };
            let new_handle = materials.add(image_handle.clone());
            *image = new_handle;
        }
    }
}

#[derive(Component)]
struct LilGuy {
    lilguy_id: u8,
}

fn spawn_lilguys(
    background: Query<Entity, With<Background>>,
    images: Res<ImageHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let Ok(bg_entity) = background.get_single() else { return; };
    let mut bg_cmd = commands.entity(bg_entity);

    for i in 0..LILGUYS_COUNT {   
        let lilguy = &LILGUYS_BESTIARY[i];

        let Some(handle) = &images.lilguys_back[i] else { return; };
        let material = materials.add(handle.clone());
        let size = &lilguy.bg_image_size;
        let position = lilguy.spawn_pos;
        
        bg_cmd.with_children(|cmd| {
            cmd.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Rectangle::new(size.x, size.y)).into(),
                    material,
                    transform: Transform::from_translation(position.extend(Z_POS_LIL_GUYS)),
                    ..default()
                },
                LilGuy {
                    lilguy_id: i as u8
                }
            )).with_children(|cmd_child| {
                cmd_child.spawn((
                    SpatialBundle {
                        transform: Transform::from_translation(lilguy.bg_click_offset.extend(0.0)),
                        ..default()
                    },
                    Clickable {
                        area: lilguy.bg_click_area,
                        action: ActionTypes::ZoomLilguy(i as u8),
                        behavior: ClickBehaviors::SingleClick,
                        active_on: ActiveStates::Game,
                    }
                ));
            });
        });
    }
}

#[derive(Component)]
struct Faceplate;

fn spawn_faceplate(
    images: Res<ImageHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let Some(image) = &images.foreground else { return; };
    let material = materials.add(image.clone());

    commands.spawn((
        MaterialMesh2dBundle {
            material,
            mesh: meshes.add(Rectangle::new(FOREGROUND_IMAGE_SIZE.x, FOREGROUND_IMAGE_SIZE.y)).into(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, Z_POS_FACEPLATE)),
            ..default()
        },
        Faceplate,
        GameItem,
    ));
}

fn update_faceplate(
    mut on_select: EventReader<LilGuySelected>,
    mut on_deselect: EventReader<LilGuyDeselected>,
    mut on_submit: EventReader<LilGuyDeselected>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    images: Res<ImageHandles>,
    mut faceplates: Query<&mut Handle<ColorMaterial>, With<Faceplate>>,
) {
    let selected = !on_select.is_empty();
    on_select.clear();
    let deselected = !on_deselect.is_empty();
    on_deselect.clear();
    let submitted = !on_submit.is_empty();
    on_submit.clear();

    let removing_buttons = deselected || submitted;
    let adding_buttons = selected;
    if adding_buttons == removing_buttons {
        return;
    }

    let image = 
        if adding_buttons {
            &images.foreground
        } else {
            &images.foreground_no_buttons
        };

    let Some(image) = image else { return; };
    for mut faceplate in &mut faceplates {
        *faceplate = materials.add(image.clone());
    }
}

#[derive(Component)]
struct MissionMonitor;

#[derive(Component)]
struct InfoMonitor;

fn spawn_monitors(
    images: Res<ImageHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let Some(top_image) = &images.lilguys_info_monitor[0] else { return; };
    let top_material = materials.add(top_image.clone());

    commands.spawn((
        MaterialMesh2dBundle {
            material: top_material,
            mesh: meshes.add(Rectangle::new(FOREGROUND_TOP_MONITOR_IMAGE_SIZE.x, FOREGROUND_TOP_MONITOR_IMAGE_SIZE.y)).into(),
            transform: Transform::from_translation(FOREGROUND_TOP_MONITOR_IMAGE_POS.extend(Z_POS_MONITORS)),
            ..default()
        },
        InfoMonitor,
        GameItem,
    ));

    let Some(bottom_image) = &images.lilguys_mission_monitor[0] else { return; };
    let bottom_material = materials.add(bottom_image.clone());

    commands.spawn((
        MaterialMesh2dBundle {
            material: bottom_material,
            mesh: meshes.add(Rectangle::new(FOREGROUND_BOTTOM_MONITOR_IMAGE_SIZE.x, FOREGROUND_BOTTOM_MONITOR_IMAGE_SIZE.y)).into(),
            transform: Transform::from_translation(FOREGROUND_BOTTOM_MONITOR_IMAGE_POS.extend(Z_POS_MONITORS)),
            ..default()
        },
        MissionMonitor,
        GameItem,
    ));
}


fn update_mission_monitor(
    images: Res<ImageHandles>,
    target_lilguy: Res<TargetLilGuy>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mission_monitors: Query<&mut Handle<ColorMaterial>, With<MissionMonitor>>,
) {
    let Some(target_lilguy_id) = target_lilguy.target_lilguy_id else { return; };
    let Some(image) = &images.lilguys_mission_monitor[target_lilguy_id as usize] else { return; };

    for mut mission_monitor in &mut mission_monitors {
        *mission_monitor = materials.add(image.clone());
    }
}

#[derive(Component, Default, Debug)]
struct Background {
    vel_x: f32,
}

fn spawn_background(
    images: Res<ImageHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let Some(image) = &images.background else { return; };
    let material = materials.add(image.clone());
    let mesh = Rectangle::new(BACKGROUND_IMAGE_SIZE.x, BACKGROUND_IMAGE_SIZE.y);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            material,
            transform: Transform::from_translation(BACKGROUND_START_POS.extend(Z_POS_BACKGROUND)).with_scale(Vec3::new(BACKGROUND_SCALE, BACKGROUND_SCALE, 1.0)),
            ..default()
        },
        Background::default(),
        GameItem,
    ));
}

fn deselect_on_esc(
    key_input: Res<ButtonInput<KeyCode>>,
    mut deselect: EventWriter<LilGuyDeselected>,
    mut on_click: EventWriter<ButtonClicked>,
) {
    if key_input.just_pressed(KeyCode::Escape) {
        deselect.send(LilGuyDeselected);
        on_click.send_default();
    }
}

fn resize_foreground(
    mut on_window_resize: EventReader<WindowResized>,
    mut window: Query<&mut Window>,
) {
    for _ in on_window_resize.read() {
        if let Ok(mut window) = window.get_single_mut() {
            let window_width = window.physical_width() as f32;
            let window_height = window.physical_height() as f32;
            let window_ratio = window_width / window_height;
            if window_ratio == FOREGROUND_ASPECT_RATIO {
                continue;
            }

            let new_scale = 
                if window_ratio < FOREGROUND_ASPECT_RATIO { //x is the shorter side
                    window_width / FOREGROUND_IMAGE_SIZE.x
                } else { //y is the shorter side
                    window_height / FOREGROUND_IMAGE_SIZE.y
                };
            window.resolution.set_scale_factor(new_scale);
        }
    }
}

fn handle_scroll_key_input(
    input: Res<ButtonInput<KeyCode>>,
    mut scroll: EventWriter<ScrollDirections>,
    mut on_click: EventWriter<ButtonClicked>,
) {
    const LEFT_KEYS: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
    const RIGHT_KEYS: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];
    const ALL_KEYS: [KeyCode; 4] = [KeyCode::KeyA, KeyCode::ArrowLeft, KeyCode::KeyD, KeyCode::ArrowRight];

    if input.any_pressed(LEFT_KEYS) {
        scroll.send(ScrollDirections::ScrollLeft);
    }
    if input.any_pressed(RIGHT_KEYS) {
        scroll.send(ScrollDirections::ScrollRight);
    }
    if input.any_just_pressed(ALL_KEYS) {
        on_click.send_default();
    }
}

#[derive(Resource, Default)]
struct StopScrolling {
    pub value: bool
}

// TODO: clean up the math - make it framerate-insensitive
fn handle_scrolling(
    time: Res<Time>,
    mut scrolling: EventReader<ScrollDirections>,
    stop: Res<StopScrolling>,
    mut background: Query<(&mut Transform, &mut Background)>,
) {
    if (*stop).value { //overloading Res.value
        return;
    }

    let mut scrolling_left = false;
    let mut scrolling_right = false;

    for scroll in scrolling.read() {
        match scroll {
            ScrollDirections::ScrollLeft => scrolling_left = true,
            ScrollDirections::ScrollRight => scrolling_right = true,
        }
    }

    let mut dir_x = 0.0;
    if scrolling_right {
        dir_x -= 1.0;
    }
    if scrolling_left {
        dir_x += 1.0;
    }
    
    let delta_time = time.delta_seconds();
    for (mut bg_transform, mut bg) in &mut background {
        
        let mut releasing = false;
        if dir_x == 0.0 {
            releasing = true;
            if bg.vel_x != 0.0 {
                dir_x = bg.vel_x.signum() * -1.0;
            } else {
                continue;
            }
        }
        let new_vel = bg.vel_x + delta_time * BACKGROUND_SCROLL_ACCEL * dir_x;
        
        if new_vel > BACKGROUND_SCROLL_MAX_SPEED {
            bg.vel_x = BACKGROUND_SCROLL_MAX_SPEED;
            bg_transform.translation.x += delta_time * BACKGROUND_SCROLL_MAX_SPEED;
        } else if new_vel < -BACKGROUND_SCROLL_MAX_SPEED {
            bg.vel_x = -BACKGROUND_SCROLL_MAX_SPEED;
            bg_transform.translation.x += delta_time * -BACKGROUND_SCROLL_MAX_SPEED;
        } else {
            let del_vel = delta_time * BACKGROUND_SCROLL_ACCEL * dir_x;
            if releasing && del_vel.abs() > bg.vel_x.abs() && del_vel.signum() != bg.vel_x.signum() {
                bg.vel_x = 0.0;
            } else {
                bg.vel_x += del_vel;
            }
            let del_pos = delta_time * bg.vel_x + delta_time * delta_time * BACKGROUND_SCROLL_ACCEL * dir_x / 2.0;

            bg_transform.translation.x += del_pos;
        }
        if bg_transform.translation.x > BACKGROUND_MAX_X {
            bg_transform.translation.x = BACKGROUND_MAX_X;
            bg.vel_x = 0.0;
        } else if bg_transform.translation.x < BACKGROUND_MIN_X {
            bg_transform.translation.x = BACKGROUND_MIN_X;
            bg.vel_x = 0.0;
        }
    }
}

#[derive(Component)]
struct ZoomedInLilGuy;

#[derive(Component)]
struct SelectionOverlay;

#[derive(Component)]
struct MessageOverlay;

fn handle_lilguy_selected(
    mut events: EventReader<LilGuySelected>,
    images: Res<ImageHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut stop_scrolling: ResMut<StopScrolling>,
    mut commands: Commands,
    mut lilguy_selection: ResMut<LilGuySelection>,
) {
    if lilguy_selection.zoomed_lilguy_id.is_some() {
        return;
    }

    for event in events.read() {
        let lilguy_id = event.lilguy_id as usize;
        let Some(image) = &images.lilguys_zoomed[lilguy_id] else { return; };
        let material = materials.add(image.clone());
        let size = LILGUYS_BESTIARY[lilguy_id].zoom_image_size;

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(FOREGROUND_IMAGE_SIZE.x, FOREGROUND_IMAGE_SIZE.y)).into(),
                material: materials.add(ColorMaterial::from_color(BACKGROUND_OVERLAY_COLOR)),
                transform: Transform::from_translation(Vec2::ZERO.extend(Z_POS_SELECTED_OVERLAY)),
                ..default()
            },
            SelectionOverlay,
            GameItem,
        ));

        commands.spawn((
            MaterialMesh2dBundle {
                material,
                mesh: meshes.add(Rectangle::new(size.x, size.y)).into(),
                transform: Transform::from_translation(FOREGROUND_PORTHOLE_CENTER_POS.extend(Z_POS_MONITORS)),
                ..default()
            },
            ZoomedInLilGuy,
            GameItem,
        ));
        
        lilguy_selection.zoomed_lilguy_id = Some(event.lilguy_id);
        (*stop_scrolling).value = true;
    }
}

fn handle_lilguy_deselected(
    mut events: EventReader<LilGuyDeselected>,
    mut selection: ResMut<LilGuySelection>,
    mut stop_scrolling: ResMut<StopScrolling>,
    zoomed_lilguys: Query<Entity, With<ZoomedInLilGuy>>,
    overlays: Query<Entity, With<SelectionOverlay>>,
    mut commands: Commands
) {
    if events.is_empty() {
        return;
    }
    events.clear();
    
    for lilguy in &zoomed_lilguys {
        commands.entity(lilguy).despawn();
    }
    selection.zoomed_lilguy_id = None;
    (*stop_scrolling).value = false;
    for overlay in &overlays {
        commands.entity(overlay).despawn();
    }
}