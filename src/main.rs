use bevy::{
    color::palettes::css::RED, prelude::*, sprite::MaterialMesh2dBundle, window::WindowResized
};

const FOREGROUND_IMAGE_PATH: &str = "FrontPanel.png";
const FOREGROUND_IMAGE_SIZE: Vec2 = Vec2::new(3641.0, 2048.0);
const FOREGROUND_ASPECT_RATIO: f32 = FOREGROUND_IMAGE_SIZE.x / FOREGROUND_IMAGE_SIZE.y;

const FOREGROUND_TOP_MONITOR_IMAGE_SIZE: Vec2 = Vec2::new(1101.0, 1351.0);
const FOREGROUND_TOP_MONITOR_IMAGE_POS: Vec2 = Vec2::new(-1090.0, 180.0);
const FOREGROUND_BOTTOM_MONITOR_IMAGE_SIZE: Vec2 = Vec2::new(1101.0, 495.0);
const FOREGROUND_BOTTOM_MONITOR_IMAGE_POS: Vec2 = Vec2::new(-1090.0, -790.0);

const FOREGROUND_CLOSE_BUTTON_POS: Vec2 = Vec2::new(1650.0, 860.0);
const FOREGROUND_LEFT_BUTTON_POS: Vec2 = Vec2::new(-178.0, -900.0);
const FOREGROUND_BACK_BUTTON_POS: Vec2 = Vec2::new(140.0, -900.0);
const FOREGROUND_BACK_BUTTON_AREA: Vec2 = Vec2::new(400.0, 180.0);
const FOREGROUND_SEND_BUTTON_POS: Vec2 = Vec2::new(772.0, -900.0);
const FOREGROUND_SEND_BUTTON_AREA: Vec2 = Vec2::new(765.0, 180.0);
const FOREGROUND_RIGHT_BUTTON_POS: Vec2 = Vec2::new(1265.0, -900.0);
const FOREGROUND_SMALL_BUTTON_AREA: Vec2 = Vec2::new(180.0, 180.0);
const FOREGROUND_PORTHOLE_CENTER_POS: Vec2 = Vec2::new(530.0, 105.0);
const FOREGROUND_PORTHOLE_RAD: f32 = 780.0;


const BACKGROUND_IMAGE_PATH: &str = "Roughback.png";
const BACKGROUND_IMAGE_SIZE: Vec2 = Vec2::new(16378.0, 2048.0);
const BACKGROUND_PADDING_SIZE: f32 = 5.0;
const BACKGROUND_START_POS: Vec2 = Vec2::new(0.0, FOREGROUND_PORTHOLE_CENTER_POS.y);
const BACKGROUND_MIN_X: f32 = FOREGROUND_PORTHOLE_CENTER_POS.x - BACKGROUND_IMAGE_SIZE.x * BACKGROUND_SCALE / 2.0 + FOREGROUND_PORTHOLE_RAD + BACKGROUND_PADDING_SIZE;
const BACKGROUND_MAX_X: f32 = FOREGROUND_PORTHOLE_CENTER_POS.x + BACKGROUND_IMAGE_SIZE.x * BACKGROUND_SCALE / 2.0 - FOREGROUND_PORTHOLE_RAD - BACKGROUND_PADDING_SIZE;
const BACKGROUND_SCROLL_MAX_SPEED: f32 = 1200.0;
const BACKGROUND_SCROLL_ACCEL: f32 = 10000.0;
const BACKGROUND_SCALE: f32 = (FOREGROUND_PORTHOLE_RAD + BACKGROUND_PADDING_SIZE) * 2.0 / BACKGROUND_IMAGE_SIZE.y;

const Z_POS_FACEPLATE: f32 = 10.0;
const Z_POS_MONITORS: f32 = 9.0;
const Z_POS_BACKGROUND: f32 = 0.0;
const Z_POS_LIL_GUYS: f32 = 1.0;

struct LilGuyInfo {
    spawn_pos: Vec2,
    bg_image_size: Vec2,
    bg_click_area: ClickArea,
    zoom_image_size: Vec2,
    bg_image_path: &'static str,
    zoom_image_path: &'static str,
    info_monitor_image_path: &'static str,
    mission_monitor_image_path: &'static str,
}

const LILGUYS_COUNT: usize = 1;
const LILGUYS_BESTIARY: [LilGuyInfo; LILGUYS_COUNT] = [
    LilGuyInfo {
        spawn_pos: Vec2::new(3000.0, 0.0),
        bg_image_size: Vec2::new(272.0, 192.0),
        bg_click_area: ClickArea::Circular(96.0),
        zoom_image_size: Vec2::new(377.0, 294.0),
        bg_image_path: "lilguys_back/Layer 3.png",
        zoom_image_path: "lilguys_zoomed/Layer 3.png",
        info_monitor_image_path: "info_monitors/TopScreen_Lollip.png",
        mission_monitor_image_path: "mission_monitors/BottomScreen_Lollip.png",
    },
];

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Loading,
    Game,
    //NextLevel,
    //GameOver,
}

fn main() {
    App::new()
    .add_event::<LilGuySelected>()
    .add_event::<LilGuyDeselected>()
    .insert_resource(UiActions::default())
    .insert_resource(StopScrolling::default())
    .insert_resource(LilGuySelection::default())
    .insert_resource(ImageHandles::default())
    .add_plugins(DefaultPlugins)
    .insert_state(GameState::default())
    .add_systems(OnEnter(GameState::Loading), (
        start_load_images,
    ))
    .add_systems(Update, (
        monitor_loading,
    ).run_if(in_state(GameState::Loading)))
    .add_systems(OnEnter(GameState::Game), (
        spawn_camera,
        spawn_faceplate,
        spawn_background,
        spawn_border_blocks,
        spawn_monitors,
        spawn_exit_button,
        spawn_scroll_buttons,
        spawn_back_button,
        spawn_send_button,
        spawn_lilguys,
    ))
    .add_systems(Update, (
        resize_foreground,
        close_on_esc,
    ))
    .add_systems(Update, (
        resize_foreground,
        check_button_clicked,
        scroll_background,
    ).run_if(in_state(GameState::Game)))
    .add_systems(PostUpdate, (
        handle_exiting,
        handle_scrolling,
        handle_lilguy_selected,
        handle_lilguy_deselected,
        debug_draw_buttons,
    ).run_if(in_state(GameState::Game)))
    .run();
}

#[derive(Resource, Default)]
struct ImageHandles {
    foreground: Option<Handle<Image>>,
    background: Option<Handle<Image>>,
    lilguys_back: [Option<Handle<Image>>; LILGUYS_COUNT],
    lilguys_zoomed: [Option<Handle<Image>>; LILGUYS_COUNT],
    lilguys_info_monitor: [Option<Handle<Image>>; LILGUYS_COUNT],
    lilguys_mission_monitor: [Option<Handle<Image>>; LILGUYS_COUNT],
}

fn start_load_images(
    assets: Res<AssetServer>,
    mut handles: ResMut<ImageHandles>,
) {
    handles.foreground = Some(assets.load(FOREGROUND_IMAGE_PATH));
    handles.background = Some(assets.load(BACKGROUND_IMAGE_PATH));

    for i in 0..LILGUYS_COUNT {
        handles.lilguys_back[i] = Some(assets.load(LILGUYS_BESTIARY[i].bg_image_path));
        handles.lilguys_zoomed[i] = Some(assets.load(LILGUYS_BESTIARY[i].zoom_image_path));
        handles.lilguys_info_monitor[i] = Some(assets.load(LILGUYS_BESTIARY[i].info_monitor_image_path));
        handles.lilguys_mission_monitor[i] = Some(assets.load(LILGUYS_BESTIARY[i].mission_monitor_image_path));   
    }
}

fn monitor_loading(
    handles: Res<ImageHandles>,
    assets: Res<AssetServer>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if image_is_not_loaded(&assets, &handles.foreground) {
        return;
    }
    if image_is_not_loaded(&assets, &handles.background) {
        return;
    }
    for i in 0..LILGUYS_COUNT {
        if image_is_not_loaded(&assets, &handles.lilguys_back[i]) {
            return;
        }
        if image_is_not_loaded(&assets, &handles.lilguys_zoomed[i]) {
            return;
        }
        if image_is_not_loaded(&assets, &handles.lilguys_info_monitor[i]) {
            return;
        }
        if image_is_not_loaded(&assets, &handles.lilguys_mission_monitor[i]) {
            return;
        }
    }
    game_state.set(GameState::Game);
}

fn image_is_not_loaded(asset_server: &AssetServer, image_handle: &Option<Handle<Image>>) -> bool {
    use bevy::asset::LoadState;
    let Some(handle) = image_handle else { panic!("Image does not have a handle") };
    return asset_server.get_load_state(handle).is_some_and(|val| val != LoadState::Loaded);
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            ..default()
        },
    );
}

// don't want to mess with masking stuff - just going to block off the sides
fn spawn_border_blocks(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let mesh = Rectangle::new(FOREGROUND_IMAGE_SIZE.x, FOREGROUND_IMAGE_SIZE.y);
    let bg_color = Color::srgb_u8(0x3e, 0x60, 0x5e);
    let material = materials.add(ColorMaterial::from_color(bg_color));

    commands.spawn(
        MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            material: material.clone(),
            transform: Transform::from_translation(Vec3::new(FOREGROUND_IMAGE_SIZE.x, 0.0, Z_POS_MONITORS)),
            ..default()
        }
    );

    commands.spawn(
        MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            material,
            transform: Transform::from_translation(Vec3::new(-FOREGROUND_IMAGE_SIZE.x, 0.0, Z_POS_MONITORS)),
            ..default()
        }
    );
}

#[derive(Component, Clone)]
struct Clickable {
    area: ClickArea,
    action: ActionTypes,
}

#[derive(Clone, Copy)]
enum ClickArea {
    Circular(f32),
    Rectangular(Vec2)
}

#[derive(Clone, Copy)]
enum ActionTypes {
    Exit,
    ScrollLeft,
    ScrollRight,
    ZoomLilguy(u8),
    UnZoomLilguy,
    SendToLab,
}

fn spawn_exit_button(
    mut commands: Commands,
) {
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(FOREGROUND_CLOSE_BUTTON_POS.extend(0.0)),
            ..default()
        },
        Clickable {
            area: ClickArea::Rectangular(FOREGROUND_SMALL_BUTTON_AREA),
            action: ActionTypes::Exit,
        }
    ));
}

fn spawn_back_button(
    mut commands: Commands,
) {
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(FOREGROUND_BACK_BUTTON_POS.extend(0.0)),
            ..default()
        },
        Clickable {
            area: ClickArea::Rectangular(FOREGROUND_BACK_BUTTON_AREA),
            action: ActionTypes::UnZoomLilguy,
        }
    ));
}

fn spawn_send_button(
    mut commands: Commands,
) {
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(FOREGROUND_SEND_BUTTON_POS.extend(0.0)),
            ..default()
        },
        Clickable {
            area: ClickArea::Rectangular(FOREGROUND_SEND_BUTTON_AREA),
            action: ActionTypes::SendToLab,
        }
    ));
}

#[derive(Component)]
struct LilGuy {
    lilguy_id: u8
}

fn spawn_lilguys(
    images: Res<ImageHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for i in 0..LILGUYS_COUNT {   
        let lilguy = &LILGUYS_BESTIARY[i];

        let Some(handle) = &images.lilguys_back[i] else { return; };
        let material = materials.add(handle.clone());
        let size = &lilguy.bg_image_size;
        let position = lilguy.spawn_pos;
        
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(size.x, size.y)).into(),
                material,
                transform: Transform::from_translation(position.extend(Z_POS_LIL_GUYS)),
                ..default()
            },
            LilGuy {
                lilguy_id: i as u8,
            },
            Background::default(),
            Clickable {
                area: lilguy.bg_click_area,
                action: ActionTypes::ZoomLilguy(i as u8),
            }
        ));
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
    ));
}

fn spawn_monitors(
    images: Res<ImageHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let Some(top_image) = &images.lilguys_info_monitor[0] else { return; };
    let top_material = materials.add(top_image.clone());

    commands.spawn(
        MaterialMesh2dBundle {
            material: top_material,
            mesh: meshes.add(Rectangle::new(FOREGROUND_TOP_MONITOR_IMAGE_SIZE.x, FOREGROUND_TOP_MONITOR_IMAGE_SIZE.y)).into(),
            transform: Transform::from_translation(FOREGROUND_TOP_MONITOR_IMAGE_POS.extend(Z_POS_MONITORS)),
            ..default()
        }
    );

    let Some(bottom_image) = &images.lilguys_mission_monitor[0] else {return; };
    let bottom_material = materials.add(bottom_image.clone());

    commands.spawn(
        MaterialMesh2dBundle {
            material: bottom_material,
            mesh: meshes.add(Rectangle::new(FOREGROUND_BOTTOM_MONITOR_IMAGE_SIZE.x, FOREGROUND_BOTTOM_MONITOR_IMAGE_SIZE.y)).into(),
            transform: Transform::from_translation(FOREGROUND_BOTTOM_MONITOR_IMAGE_POS.extend(Z_POS_MONITORS)),
            ..default()
        }
    );
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
        Background::default()
    ));
}

fn spawn_scroll_buttons(
    mut commands: Commands
) {
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(FOREGROUND_LEFT_BUTTON_POS.extend(Z_POS_FACEPLATE)),
            ..default()
        },
        Clickable {
            area: ClickArea::Rectangular(FOREGROUND_SMALL_BUTTON_AREA),
            action: ActionTypes::ScrollLeft,
        },
    ));

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(FOREGROUND_RIGHT_BUTTON_POS.extend(Z_POS_FACEPLATE)),
            ..default()
        },
        Clickable {
            area: ClickArea::Rectangular(FOREGROUND_SMALL_BUTTON_AREA),
            action: ActionTypes::ScrollRight,
        }
    ));
}

fn close_on_esc(
    key_input: Res<ButtonInput<KeyCode>>,
    mut ui_actions: ResMut<UiActions>,
) {
    if key_input.just_pressed(KeyCode::Escape) {
        ui_actions.exiting = true;
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

fn scroll_background(
    input: Res<ButtonInput<KeyCode>>,
    mut ui_actions: ResMut<UiActions>,
) {
    const LEFT_KEYS: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
    const RIGHT_KEYS: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];

    ui_actions.scrolling_left = input.any_pressed(LEFT_KEYS);
    ui_actions.scrolling_right = input.any_pressed(RIGHT_KEYS);
}

#[derive(Event)]
struct LilGuySelected {
    lilguy_id: u8
}

fn check_button_clicked(
    input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    btns: Query<(&GlobalTransform, &Clickable)>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    selection: Res<LilGuySelection>,
    mut ui_actions: ResMut<UiActions>,
    mut on_lilguy_selected: EventWriter<LilGuySelected>,
    mut on_lilguy_deselected: EventWriter<LilGuyDeselected>,
) {
    if input.just_released(MouseButton::Left) {
        reset_ui_actions(&mut ui_actions);
        return;
    }
    if !input.pressed(MouseButton::Left) {
        return;
    }

    for (btn_transform, btn) in &btns {
        let Ok(window) = windows.get_single() else { 
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
        
        let button_pressed =
            match btn.area {
                ClickArea::Circular(radius) => 
                    btn_transform.translation().xy().distance_squared(cursor_pos) < (radius * radius),
                ClickArea::Rectangular(area) => 
                    (btn_transform.translation().x - cursor_pos.x).abs() < area.x / 2.0
                    && (btn_transform.translation().y - cursor_pos.y).abs() < area.y / 2.0,
            };

        let cursor_in_porthole =
            FOREGROUND_PORTHOLE_CENTER_POS.distance_squared(cursor_pos) < (FOREGROUND_PORTHOLE_RAD * FOREGROUND_PORTHOLE_RAD);
        
        match btn.action {
            ActionTypes::Exit => ui_actions.exiting = button_pressed,
            ActionTypes::ScrollLeft => if selection.zoomed_lilguy_entity.is_none() { ui_actions.scrolling_left = button_pressed },
            ActionTypes::ScrollRight => if selection.zoomed_lilguy_entity.is_none() { ui_actions.scrolling_right = button_pressed },
            ActionTypes::ZoomLilguy(id) => if button_pressed && cursor_in_porthole { _ = on_lilguy_selected.send(LilGuySelected { lilguy_id: id }) },
            ActionTypes::UnZoomLilguy => 
                if button_pressed {
                    if let Some(lilguy) = selection.zoomed_lilguy_entity {
                        _ = on_lilguy_deselected.send(LilGuyDeselected {
                            lilguy_entity: lilguy
                        });
                    }
                },
            ActionTypes::SendToLab => 
                if button_pressed {
                    if let Some(lilguy) = selection.zoomed_lilguy_entity {
                        // check if the selection matches the target
                    }
                },
        };
    }
}

fn reset_ui_actions(
    actions: &mut UiActions
) {
    actions.exiting = false;
    actions.scrolling_left = false;
    actions.scrolling_right = false;
}

#[derive(Resource, Default)]
struct UiActions {
    exiting: bool,
    scrolling_left: bool,
    scrolling_right: bool,
}

fn handle_exiting(
    ui_actions: Res<UiActions>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if ui_actions.exiting {
        app_exit_events.send(AppExit::Success);
    }
}

#[derive(Resource, Default)]
struct StopScrolling {
    value: bool
}

// TODO: clean up the math - make it framerate-insensitive
fn handle_scrolling(
    time: Res<Time>,
    ui_actions: Res<UiActions>,
    stop: Res<StopScrolling>,
    mut background: Query<(&mut Transform, &mut Background)>,
) {
    if stop.value {
        return;
    }

    let mut dir_x = 0.0;
    if ui_actions.scrolling_right {
        dir_x -= 1.0;
    }
    if ui_actions.scrolling_left {
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
struct ZoomedInLilGuy {
    lilguy_id: u8
}

#[derive(Resource, Default, Debug)]
struct LilGuySelection {
    zoomed_lilguy_entity: Option<Entity>,
}

fn handle_lilguy_selected(
    mut events: EventReader<LilGuySelected>,
    images: Res<ImageHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut stop_scrolling: ResMut<StopScrolling>,
    mut commands: Commands,
    mut lilguy_selection: ResMut<LilGuySelection>,

) {
    if lilguy_selection.zoomed_lilguy_entity.is_some() {
        return;
    }

    for event in events.read() {
        let lilguy_id = event.lilguy_id as usize;
        let Some(image) = &images.lilguys_zoomed[lilguy_id] else { return; };
        let material = materials.add(image.clone());
        let size = LILGUYS_BESTIARY[lilguy_id].zoom_image_size;

        let cmd = commands.spawn((
            MaterialMesh2dBundle {
                material,
                mesh: meshes.add(Rectangle::new(size.x, size.y)).into(),
                transform: Transform::from_translation(FOREGROUND_PORTHOLE_CENTER_POS.extend(Z_POS_MONITORS)),
                ..default()
            },
            ZoomedInLilGuy {
                lilguy_id: event.lilguy_id
            }
        ));
        
        lilguy_selection.zoomed_lilguy_entity = Some(cmd.id());
        stop_scrolling.value = true;
    }
}

#[derive(Event)]
struct LilGuyDeselected {
    lilguy_entity: Entity
}

fn handle_lilguy_deselected(
    mut events: EventReader<LilGuyDeselected>,
    mut selection: ResMut<LilGuySelection>,
    mut stop_scrolling: ResMut<StopScrolling>,
    mut commands: Commands
) {
    let had_events = !events.is_empty();
    for event in events.read() {
        commands.entity(event.lilguy_entity).despawn();
    }
    if had_events {
        selection.zoomed_lilguy_entity = None;
        stop_scrolling.value = false;
    }
}

fn debug_draw_buttons(
    mut gizmos: Gizmos,
    buttons: Query<(&GlobalTransform, &Clickable)>,
) {
    for (button_pos, button) in &buttons {
        match button.area {
            ClickArea::Circular(rads) => { gizmos.circle_2d(button_pos.translation().xy(), rads, RED); },
            ClickArea::Rectangular(area) => { gizmos.rect_2d(button_pos.translation().xy(), Rot2::IDENTITY, area, RED); },
        }
    }
}