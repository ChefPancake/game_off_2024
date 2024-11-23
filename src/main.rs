use bevy::{
    color::palettes::css::{RED, WHITE_SMOKE}, prelude::*, sprite::MaterialMesh2dBundle, window::WindowResized
};

/*
- [ ] make the lilguys in the background children of it, then remove the BACKGROUND component from them
    - they'll move w the background for free
- [ ] get end-game states going
- [ ] add a loading bar to the loading screen
    - just overlaid rects should work
- [ ] add bubble particles
- [x] Fix clicking behavior
- [ ] Add paging through info screens
*/

const FOREGROUND_TITLE_SCREEN: &str = "TitleScreen.png";
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

const BACKGROUND_IMAGE_PATH: &str = "Background_empty.png";
const BACKGROUND_IMAGE_SIZE: Vec2 = Vec2::new(16378.0, 2048.0);
const BACKGROUND_PADDING_SIZE: f32 = 5.0;
const BACKGROUND_START_POS: Vec2 = Vec2::new(0.0, FOREGROUND_PORTHOLE_CENTER_POS.y);
const BACKGROUND_MIN_X: f32 = FOREGROUND_PORTHOLE_CENTER_POS.x - BACKGROUND_IMAGE_SIZE.x * BACKGROUND_SCALE / 2.0 + FOREGROUND_PORTHOLE_RAD + BACKGROUND_PADDING_SIZE;
const BACKGROUND_MAX_X: f32 = FOREGROUND_PORTHOLE_CENTER_POS.x + BACKGROUND_IMAGE_SIZE.x * BACKGROUND_SCALE / 2.0 - FOREGROUND_PORTHOLE_RAD - BACKGROUND_PADDING_SIZE;
const BACKGROUND_SCROLL_MAX_SPEED: f32 = 1200.0;
const BACKGROUND_SCROLL_ACCEL: f32 = 10000.0;
const BACKGROUND_SCALE: f32 = (FOREGROUND_PORTHOLE_RAD + BACKGROUND_PADDING_SIZE) * 2.0 / BACKGROUND_IMAGE_SIZE.y;

const PROGRESS_BAR_BORDER_SIZE: Vec2 = Vec2::new(FOREGROUND_IMAGE_SIZE.x * 0.8, 150.0);
const PROGRESS_BAR_INTERNAL_SIZE: Vec2 = Vec2::new(FOREGROUND_IMAGE_SIZE.x * 0.8 - 50.0, 100.0);
const PROGRESS_BAR_TOTAL_UNITS: usize = LILGUYS_COUNT * 4 + 6;

const WIN_SCREEN_IMAGE_PATH: &str = "WinBox.png";
const LOSE_SCREEN_IMAGE_PATH: &str = "LoseBox.png";
const NEXT_MISSION_SCREEN_IMAGE_PATH: &str = "NextMissionBox.png";
const MESSAGE_BOX_IMAGE_SIZE: Vec2 = Vec2::new(1455.0, 1458.0);

const Z_POS_FACEPLATE: f32 = 10.0;
const Z_POS_MONITORS: f32 = 9.0;
const Z_POS_BACKGROUND: f32 = 0.0;
const Z_POS_LIL_GUYS: f32 = 1.0;
const Z_POS_MESSAGE_BOX: f32 = 11.0;

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

const LILGUYS_COUNT: usize = 19;
const LILGUYS_BESTIARY: [LilGuyInfo; LILGUYS_COUNT] = [
    LilGuyInfo {
        spawn_pos: Vec2::new(3000.0, 0.0),
        bg_image_size: Vec2::new(213.0, 147.0),
        bg_click_area: ClickArea::Circular(96.0),
        zoom_image_size: Vec2::new(1293.0, 1003.0),
        bg_image_path: "lilguys_back/Abogus.png",
        zoom_image_path: "lilguys_zoomed/Abogus.png",
        info_monitor_image_path: "info_monitors/Abogus.png",
        mission_monitor_image_path: "mission_monitors/Abogus.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(2500.0, 300.0),
        bg_image_size: Vec2::new(301.0, 438.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(200.0, 400.0)),
        zoom_image_size: Vec2::new(970.0, 1404.0),
        bg_image_path: "lilguys_back/Biblet.png",
        zoom_image_path: "lilguys_zoomed/Biblet.png",
        info_monitor_image_path: "info_monitors/Biblet.png",
        mission_monitor_image_path: "mission_monitors/Biblet.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(2000.0, 100.0),
        bg_image_size: Vec2::new(373.0, 458.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(300.0, 400.0)),
        zoom_image_size: Vec2::new(1046.0, 1288.0),
        bg_image_path: "lilguys_back/Bloober.png",
        zoom_image_path: "lilguys_zoomed/Bloober.png",
        info_monitor_image_path: "info_monitors/Bloober.png",
        mission_monitor_image_path: "mission_monitors/Bloober.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(1500.0, -500.0),
        bg_image_size: Vec2::new(372.0, 289.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(300.0, 225.0)),
        zoom_image_size: Vec2::new(1488.0, 868.0),
        bg_image_path: "lilguys_back/ChetTimbo.png",
        zoom_image_path: "lilguys_zoomed/ChetTimbo.png",
        info_monitor_image_path: "info_monitors/ChetTimbo.png",
        mission_monitor_image_path: "mission_monitors/ChetTimbo.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(1000.0, 150.0),
        bg_image_size: Vec2::new(385.0, 373.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(300.0, 300.0)),
        zoom_image_size: Vec2::new(1232.0, 1020.0),
        bg_image_path: "lilguys_back/Feetta.png",
        zoom_image_path: "lilguys_zoomed/Feetta.png",
        info_monitor_image_path: "info_monitors/Feetta.png",
        mission_monitor_image_path: "mission_monitors/Feetta.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(500.0, -200.0),
        bg_image_size: Vec2::new(526.0, 625.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(475.0, 575.0)),
        zoom_image_size: Vec2::new(1187.0, 1402.0),
        bg_image_path: "lilguys_back/Gloober.png",
        zoom_image_path: "lilguys_zoomed/Gloober.png",
        info_monitor_image_path: "info_monitors/Gloober.png",
        mission_monitor_image_path: "mission_monitors/Gloober.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(000.0, 500.0),
        bg_image_size: Vec2::new(254.0, 274.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(200.0, 225.0)),
        zoom_image_size: Vec2::new(1108.0, 1374.0),
        bg_image_path: "lilguys_back/Golyp.png",
        zoom_image_path: "lilguys_zoomed/Golyp.png",
        info_monitor_image_path: "info_monitors/Golyp.png",
        mission_monitor_image_path: "mission_monitors/Golyp.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-1000.0, 500.0),
        bg_image_size: Vec2::new(1558.0, 802.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(1500.0, 700.0)),
        zoom_image_size: Vec2::new(1108.0, 1374.0),
        bg_image_path: "lilguys_back/Jerry.png",
        zoom_image_path: "lilguys_zoomed/Jerry.png",
        info_monitor_image_path: "info_monitors/Jerry.png",
        mission_monitor_image_path: "mission_monitors/Jerry.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-1500.0, 100.0),
        bg_image_size: Vec2::new(519.0, 363.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(450.0, 300.0)),
        zoom_image_size: Vec2::new(1411.0, 995.0),
        bg_image_path: "lilguys_back/Jurpils.png",
        zoom_image_path: "lilguys_zoomed/Jurpils.png",
        info_monitor_image_path: "info_monitors/Jurpils.png",
        mission_monitor_image_path: "mission_monitors/Jurpils.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-2000.0, 100.0),
        bg_image_size: Vec2::new(285.0, 302.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(225.0, 250.0)),
        zoom_image_size: Vec2::new(1320.0, 839.0),
        bg_image_path: "lilguys_back/Keif.png",
        zoom_image_path: "lilguys_zoomed/Keif.png",
        info_monitor_image_path: "info_monitors/Keif.png",
        mission_monitor_image_path: "mission_monitors/Keif.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-2500.0, 100.0),
        bg_image_size: Vec2::new(348.0, 463.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(300.0, 425.0)),
        zoom_image_size: Vec2::new(1077.0, 1327.0),
        bg_image_path: "lilguys_back/Nyada.png",
        zoom_image_path: "lilguys_zoomed/Nyada.png",
        info_monitor_image_path: "info_monitors/Nyada.png",
        mission_monitor_image_path: "mission_monitors/Nyada.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-3000.0, 100.0),
        bg_image_size: Vec2::new(246.0, 326.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(200.0, 350.0)),
        zoom_image_size: Vec2::new(1277.0, 1073.0),
        bg_image_path: "lilguys_back/Ooples.png",
        zoom_image_path: "lilguys_zoomed/Ooples.png",
        info_monitor_image_path: "info_monitors/Ooples.png",
        mission_monitor_image_path: "mission_monitors/Ooples.png",
    },

    LilGuyInfo {
        spawn_pos: Vec2::new(-3500.0, 100.0),
        bg_image_size: Vec2::new(955.0, 830.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(900.0, 775.0)),
        zoom_image_size: Vec2::new(1344.0, 1077.0),
        bg_image_path: "lilguys_back/Patootoo.png",
        zoom_image_path: "lilguys_zoomed/Patootoo.png",
        info_monitor_image_path: "info_monitors/Patootoo.png",
        mission_monitor_image_path: "mission_monitors/Patootoo.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-4000.0, 100.0),
        bg_image_size: Vec2::new(858.0, 1437.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(800.0, 1375.0)),
        zoom_image_size: Vec2::new(867.0, 1452.0),
        bg_image_path: "lilguys_back/Qwoud.png",
        zoom_image_path: "lilguys_zoomed/Qwoud.png",
        info_monitor_image_path: "info_monitors/Qwoud.png",
        mission_monitor_image_path: "mission_monitors/Qwoud.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-4500.0, 100.0),
        bg_image_size: Vec2::new(195.0, 245.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(150.0, 200.0)),
        zoom_image_size: Vec2::new(836.0, 1358.0),
        bg_image_path: "lilguys_back/Snarfblat.png",
        zoom_image_path: "lilguys_zoomed/Snarfblat.png",
        info_monitor_image_path: "info_monitors/Snarfblat.png",
        mission_monitor_image_path: "mission_monitors/Snarfblat.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-5000.0, 100.0),
        bg_image_size: Vec2::new(721.0, 838.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(650.0, 775.0)),
        zoom_image_size: Vec2::new(1022.0, 1346.0),
        bg_image_path: "lilguys_back/Squapple.png",
        zoom_image_path: "lilguys_zoomed/Squapple.png",
        info_monitor_image_path: "info_monitors/Squapple.png",
        mission_monitor_image_path: "mission_monitors/Squapple.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-5500.0, 100.0),
        bg_image_size: Vec2::new(338.0, 212.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(275.0, 175.0)),
        zoom_image_size: Vec2::new(1348.0, 974.0),
        bg_image_path: "lilguys_back/Thit.png",
        zoom_image_path: "lilguys_zoomed/Thit.png",
        info_monitor_image_path: "info_monitors/Thit.png",
        mission_monitor_image_path: "mission_monitors/Thit.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-6000.0, 100.0),
        bg_image_size: Vec2::new(411.0, 468.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(375.0, 400.0)),
        zoom_image_size: Vec2::new(659.0, 1351.0),
        bg_image_path: "lilguys_back/Toober.png",
        zoom_image_path: "lilguys_zoomed/Toober.png",
        info_monitor_image_path: "info_monitors/Toober.png",
        mission_monitor_image_path: "mission_monitors/Toober.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-6500.0, 100.0),
        bg_image_size: Vec2::new(170.0, 258.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(125.0, 200.0)),
        zoom_image_size: Vec2::new(1311.0, 837.0),
        bg_image_path: "lilguys_back/Unkie.png",
        zoom_image_path: "lilguys_zoomed/Unkie.png",
        info_monitor_image_path: "info_monitors/Unkie.png",
        mission_monitor_image_path: "mission_monitors/Unkie.png",
    },  
];


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Loading,
    Title,
    Game,
    NextLevel,
    GameOver,
}

fn main() {
    App::new()
    .add_event::<LilGuySelected>()
    .add_event::<LilGuyDeselected>()
    .add_event::<LilGuySubmitted>()
    .insert_resource(UiActions::default())
    .insert_resource(StopScrolling::default())
    .insert_resource(LilGuySelection::default())
    .insert_resource(ImageHandles::default())
    .insert_resource(TargetLilGuy::default())
    .add_plugins(DefaultPlugins)
    .insert_state(GameState::default())
    .add_systems(OnEnter(GameState::Loading), (
        spawn_camera,
        start_load_images,
        spawn_progress_bar,
    ))
    .add_systems(Update, (
        monitor_loading,
        update_progress_bar,
    ).run_if(in_state(GameState::Loading)))
    .add_systems(OnExit(GameState::Loading), (
        remove_progress_bar,
    ))
    .add_systems(OnEnter(GameState::Title), (
        spawn_exit_button,
        spawn_title_screen,
        spawn_start_button,
    ))
    .add_systems(OnExit(GameState::Title), (
        remove_title_screen,
        remove_start_button,
        spawn_faceplate,
        spawn_background,
        spawn_border_blocks,
        spawn_scroll_buttons,
        spawn_back_button,
        spawn_send_button,
    ))
    .add_systems(OnExit(GameState::Title), (
        spawn_lilguys,
    ).after(spawn_background))
    .add_systems(OnEnter(GameState::Game), (
        choose_target_lilguy,
    ))
    .add_systems(OnEnter(GameState::Game), (
        spawn_monitors,
    ).after(choose_target_lilguy))
    .add_systems(Update, (
        scroll_background,
    ).run_if(in_state(GameState::Game)))
    .add_systems(PostUpdate, (
        handle_scrolling,
    ).run_if(in_state(GameState::Game)))
    .add_systems(OnEnter(GameState::NextLevel), (
        remove_target_lilguy,
        spawn_winscreen,
    ))
    .add_systems(OnExit(GameState::NextLevel), (
        remove_message_box,
        // re-enable whatever's needed
    ))
    .add_systems(OnEnter(GameState::GameOver), (
        spawn_losescreen,
    ))
    .add_systems(OnExit(GameState::GameOver), (
        remove_message_box,
    ))
    .add_systems(Update, (
        resize_foreground,
        close_on_esc,
        check_button_clicked,
    ))
    .add_systems(PostUpdate, (
        debug_draw_buttons,
        handle_exiting,
        handle_lilguy_selected,
        handle_lilguy_deselected,
        handle_lilguy_submitted,
    ))
    .run();
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
    ));
}

fn remove_message_box(
    message_boxes: Query<Entity, With<MessageBox>>,
    mut commands: Commands,
) {
    for entity in &message_boxes {
        commands.entity(entity).despawn();
    }
}

fn remove_target_lilguy(
    lilguys: Query<(Entity, &LilGuy)>,
    target_lilguy: Res<TargetLilGuy>,
    selected_lilguy: Res<LilGuySelection>,
    mut commands: Commands,
    mut lilguy_deselected: EventWriter<LilGuyDeselected>
) {
    let Some(target_lilguy_id) = target_lilguy.target_lilguy_id else { return; };
    let Some(zoomed_lil_guy_entity) = selected_lilguy.zoomed_lilguy_entity else { return; };
    for (entity, lilguy) in &lilguys {
        if lilguy.lilguy_id == target_lilguy_id {
            commands.entity(entity).despawn();
            lilguy_deselected.send(LilGuyDeselected { lilguy_entity: zoomed_lil_guy_entity });
        }
    }
}

#[derive(Component)]
struct ProgressBar {
    progress_units: usize,
    total_units: usize,
}

fn spawn_progress_bar(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(PROGRESS_BAR_BORDER_SIZE.x, PROGRESS_BAR_BORDER_SIZE.y)).into(),
            material: materials.add(ColorMaterial::from_color(WHITE_SMOKE)),
            ..default()
        },
        ProgressBar {
            progress_units: 0,
            total_units: PROGRESS_BAR_TOTAL_UNITS,
        }
    ))
    .with_children(|cmd| {
        cmd.spawn(
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(PROGRESS_BAR_INTERNAL_SIZE.x, PROGRESS_BAR_INTERNAL_SIZE.y)).into(),
                material: materials.add(ColorMaterial::from_color(RED)),
                transform: Transform::from_translation(Vec2::ZERO.extend(Z_POS_FACEPLATE)),
                ..default()
            },
        );
    });
}

fn update_progress_bar(
    bar_parents: Query<(Entity, &ProgressBar)>,
    mut bar_children: Query<(&Parent, &mut Transform)>,
) {
    const LEFTMOST_EDGE: f32 = PROGRESS_BAR_INTERNAL_SIZE.x / -2.0;
    for (parent_entity, bar) in &bar_parents {
        for (bar_parent, mut bar_trans) in &mut bar_children {
            if **bar_parent == parent_entity {
                let new_width = PROGRESS_BAR_INTERNAL_SIZE.x * bar.progress_units as f32 / bar.total_units as f32;
                let new_x_pos = LEFTMOST_EDGE + new_width / 2.0;
                let new_scale = new_width / PROGRESS_BAR_INTERNAL_SIZE.x;
                bar_trans.translation.x = new_x_pos;
                bar_trans.scale.x = new_scale;
            }
        }
    }
}

fn remove_progress_bar(
    bars: Query<Entity, With<ProgressBar>>,
    mut commands: Commands,
) {
    for bar in &bars {
        commands.entity(bar).despawn_recursive();
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
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(FOREGROUND_BOTTOM_MONITOR_IMAGE_POS.extend(0.0)),
            ..default()
        },
        Clickable {
            area: ClickArea::Rectangular(FOREGROUND_BOTTOM_MONITOR_IMAGE_SIZE),
            action: ActionTypes::StartGame,
            behavior: ClickBehaviors::SingleClick,
        }
    ));
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

fn remove_title_screen(
    query: Query<Entity, With<TitleScreen>>,
    mut commands: Commands
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

#[derive(Resource, Default)]
struct TargetLilGuy {
    target_lilguy_id: Option<u8>,
}

#[derive(Event)]
struct LilGuySubmitted {
    lilguy_id_guess: u8,
}

fn choose_target_lilguy(
    mut target: ResMut<TargetLilGuy>,
) {
    let selected_lilguy: u8 = rand::random::<u8>() % (LILGUYS_COUNT as u8);
    target.target_lilguy_id = Some(selected_lilguy);
}

fn handle_lilguy_submitted(
    mut lilguy_submitted: EventReader<LilGuySubmitted>,
    target: Res<TargetLilGuy>,
    mut game_state: ResMut<NextState<GameState>>
) {
    let Some(target_id) = target.target_lilguy_id else { return; };

    for event in lilguy_submitted.read() {
        if event.lilguy_id_guess == target_id {
            game_state.set(GameState::NextLevel);
        } else {
            game_state.set(GameState::GameOver);
        }
    }
}

#[derive(Resource, Default)]
struct ImageHandles {
    title_screen: Option<Handle<Image>>,
    foreground: Option<Handle<Image>>,
    background: Option<Handle<Image>>,
    win_screen: Option<Handle<Image>>,
    lose_screen: Option<Handle<Image>>,
    next_mission_screen: Option<Handle<Image>>,
    lilguys_back: [Option<Handle<Image>>; LILGUYS_COUNT],
    lilguys_zoomed: [Option<Handle<Image>>; LILGUYS_COUNT],
    lilguys_info_monitor: [Option<Handle<Image>>; LILGUYS_COUNT],
    lilguys_mission_monitor: [Option<Handle<Image>>; LILGUYS_COUNT],
}

fn start_load_images(
    assets: Res<AssetServer>,
    mut handles: ResMut<ImageHandles>,
) {
    handles.title_screen = Some(assets.load(FOREGROUND_TITLE_SCREEN));
    handles.foreground = Some(assets.load(FOREGROUND_IMAGE_PATH));
    handles.background = Some(assets.load(BACKGROUND_IMAGE_PATH));
    handles.win_screen = Some(assets.load(WIN_SCREEN_IMAGE_PATH));
    handles.lose_screen = Some(assets.load(LOSE_SCREEN_IMAGE_PATH));
    handles.next_mission_screen = Some(assets.load(NEXT_MISSION_SCREEN_IMAGE_PATH));

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
    mut progress: Query<&mut ProgressBar>,
) {
    let mut current_progress = 0;

    if image_is_loaded(&assets, &handles.title_screen) {
        current_progress += 1;
    }
    if image_is_loaded(&assets, &handles.foreground) {
        current_progress += 1;
    }
    if image_is_loaded(&assets, &handles.background) {
        current_progress += 1;
    }
    if image_is_loaded(&assets, &handles.win_screen) {
        current_progress += 1;
    }
    if image_is_loaded(&assets, &handles.lose_screen) {
        current_progress += 1;
    }
    if image_is_loaded(&assets, &handles.next_mission_screen) {
        current_progress += 1;
    }
    for i in 0..LILGUYS_COUNT {
        if image_is_loaded(&assets, &handles.lilguys_back[i]) {
            current_progress += 1;
        }
        if image_is_loaded(&assets, &handles.lilguys_zoomed[i]) {
            current_progress += 1;
        }
        if image_is_loaded(&assets, &handles.lilguys_info_monitor[i]) {
            current_progress += 1;
        }
        if image_is_loaded(&assets, &handles.lilguys_mission_monitor[i]) {
            current_progress += 1;
        }
    }
    for mut progress in &mut progress {
        progress.progress_units = current_progress;
    }
    if current_progress >= PROGRESS_BAR_TOTAL_UNITS {
        game_state.set(GameState::Title);
    }
}

fn image_is_loaded(asset_server: &AssetServer, image_handle: &Option<Handle<Image>>) -> bool {
    use bevy::asset::LoadState;
    let Some(handle) = image_handle else { panic!("Image does not have a handle") };
    return asset_server.get_load_state(handle).is_some_and(|val| val == LoadState::Loaded);
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

#[derive(Clone, Copy, Eq, PartialEq)]
enum ClickBehaviors {
    SingleClick,
    ClickAndHold,
}

#[derive(Component, Clone)]
struct Clickable {
    area: ClickArea,
    action: ActionTypes,
    behavior: ClickBehaviors,
}

#[derive(Clone, Copy)]
enum ClickArea {
    Circular(f32),
    Rectangular(Vec2)
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ActionTypes {
    Exit,
    ScrollLeft,
    ScrollRight,
    ZoomLilguy(u8),
    UnZoomLilguy,
    SendToLab,
    StartGame,
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
            behavior: ClickBehaviors::SingleClick,
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
            behavior: ClickBehaviors::SingleClick,
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
            behavior: ClickBehaviors::SingleClick,
        }
    ));
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
                },
                Clickable {
                    area: lilguy.bg_click_area,
                    action: ActionTypes::ZoomLilguy(i as u8),
                    behavior: ClickBehaviors::SingleClick,
                }
            ));
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
    ));
}

fn spawn_monitors(
    images: Res<ImageHandles>,
    target_lilguy: Res<TargetLilGuy>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let Some(target_lilguy_id) = target_lilguy.target_lilguy_id else { return; };
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

    let Some(bottom_image) = &images.lilguys_mission_monitor[target_lilguy_id as usize] else {return; };
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
            behavior: ClickBehaviors::ClickAndHold,
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
            behavior: ClickBehaviors::ClickAndHold,
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
    mut on_lilguy_submitted: EventWriter<LilGuySubmitted>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if input.just_released(MouseButton::Left) {
        reset_ui_actions(&mut ui_actions);
        return;
    }
    if !input.pressed(MouseButton::Left) {
        return;
    }
    let just_clicked = input.just_pressed(MouseButton::Left);

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
        
        let (scale, _, _) = btn_transform.to_scale_rotation_translation();

        let button_pressed =
            match btn.behavior {
                ClickBehaviors::SingleClick => just_clicked,
                ClickBehaviors::ClickAndHold => true,
            }
            &&
            match btn.area {
                ClickArea::Circular(radius) => 
                    btn_transform.translation().xy().distance_squared(cursor_pos) < (radius * radius * scale.x * scale.y),
                ClickArea::Rectangular(area) => 
                    (btn_transform.translation().x - cursor_pos.x).abs() < scale.x * area.x / 2.0
                    && (btn_transform.translation().y - cursor_pos.y).abs() < scale.y * area.y / 2.0,
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
                    if let Some(lilguy) = selection.zoomed_lilguy_id {
                        _ = on_lilguy_submitted.send(LilGuySubmitted {
                            lilguy_id_guess: lilguy
                        });
                        // check if the selection matches the target
                    }
                },
            ActionTypes::StartGame => if button_pressed { game_state.set(GameState::Game) },
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
struct ZoomedInLilGuy;

#[derive(Resource, Default, Debug)]
struct LilGuySelection {
    zoomed_lilguy_entity: Option<Entity>,
    zoomed_lilguy_id: Option<u8>,
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
            ZoomedInLilGuy,
        ));
        
        lilguy_selection.zoomed_lilguy_entity = Some(cmd.id());
        lilguy_selection.zoomed_lilguy_id = Some(event.lilguy_id);
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
        selection.zoomed_lilguy_id = None;
        stop_scrolling.value = false;
    }
}

fn debug_draw_buttons(
    mut gizmos: Gizmos,
    buttons: Query<(&GlobalTransform, &Clickable)>,
) {
    for (button_pos, button) in &buttons {
        // it's assumed that everything except the progress bar is scaled squarely
        let (scale, _, _) = button_pos.to_scale_rotation_translation();
        match button.area {
            ClickArea::Circular(rads) => { gizmos.circle_2d(button_pos.translation().xy(), rads * scale.x, RED); },
            ClickArea::Rectangular(area) => { gizmos.rect_2d(button_pos.translation().xy(), Rot2::IDENTITY, area * scale.x, RED); },
        }
    }
}