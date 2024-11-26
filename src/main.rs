use bevy::{
    audio::Volume, 
    prelude::*, 
    sprite::MaterialMesh2dBundle, 
    window::{
        WindowMode, WindowResized, WindowResolution
    }
};

/*
    to build:
    cargo build --target wasm32-unknown-unknown --release
    wasm-bindgen --out-dir .\out\ --target web .\target\wasm32-unknown-unknown\release\game_off_2024.wasm
*/

/*
- [ ] add bubble particles
*/

const WINDOW_RESOLUTION: Vec2 = Vec2::new(859.0, 483.0);

const FOREGROUND_TITLE_SCREEN: &str = "TitleScreen.png";
const FOREGROUND_IMAGE_PATH: &str = "FrontPanel_noarrows.png";
const FOREGROUND_NO_BUTTONS_IMAGE_PATH: &str = "FrontPanel_nobuttons.png";
const FOREGROUND_IMAGE_SIZE: Vec2 = Vec2::new(3641.0, 2048.0);
const FOREGROUND_ASPECT_RATIO: f32 = FOREGROUND_IMAGE_SIZE.x / FOREGROUND_IMAGE_SIZE.y;

const FOREGROUND_TOP_MONITOR_IMAGE_SIZE: Vec2 = Vec2::new(1101.0, 1351.0);
const FOREGROUND_TOP_MONITOR_IMAGE_POS: Vec2 = Vec2::new(-1090.0, 180.0);
const FOREGROUND_BOTTOM_MONITOR_IMAGE_SIZE: Vec2 = Vec2::new(1101.0, 495.0);
const FOREGROUND_BOTTOM_MONITOR_IMAGE_POS: Vec2 = Vec2::new(-1090.0, -790.0);
const FOREGROUND_INFO_PAGELEFT_POS: Vec2 = Vec2::new(FOREGROUND_TOP_MONITOR_IMAGE_POS.x - 450.0, 725.0);
const FOREGROUND_INFO_PAGERIGHT_POS: Vec2 = Vec2::new(FOREGROUND_TOP_MONITOR_IMAGE_POS.x + 450.0, 725.0);
const FOREGROUND_INFO_BUTTON_RAD: f32 = 50.0;

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

const BACKGROUND_IMAGE_PATH: &str = "Background_empty_smallest.png";
const BACKGROUND_IMAGE_SIZE: Vec2 = Vec2::new(16378.0, 2048.0);
const BACKGROUND_PADDING_SIZE: f32 = 5.0;
const BACKGROUND_START_POS: Vec2 = Vec2::new(0.0, FOREGROUND_PORTHOLE_CENTER_POS.y);
const BACKGROUND_MIN_X: f32 = FOREGROUND_PORTHOLE_CENTER_POS.x - BACKGROUND_IMAGE_SIZE.x * BACKGROUND_SCALE / 2.0 + FOREGROUND_PORTHOLE_RAD + BACKGROUND_PADDING_SIZE;
const BACKGROUND_MAX_X: f32 = FOREGROUND_PORTHOLE_CENTER_POS.x + BACKGROUND_IMAGE_SIZE.x * BACKGROUND_SCALE / 2.0 - FOREGROUND_PORTHOLE_RAD - BACKGROUND_PADDING_SIZE;
const BACKGROUND_SCROLL_MAX_SPEED: f32 = 1200.0;
const BACKGROUND_SCROLL_ACCEL: f32 = 10000.0;
const BACKGROUND_SCALE: f32 = (FOREGROUND_PORTHOLE_RAD + BACKGROUND_PADDING_SIZE) * 2.0 / BACKGROUND_IMAGE_SIZE.y;
const BACKGROUND_OVERLAY_COLOR: Color = Color::Srgba(Srgba { red: 0.0, green: 0.25, blue: 0.37109375, alpha: 0.8 });
const BACKGROUND_COLOR: Color = Color::Srgba(Srgba { red: 0.2421875, green: 0.375, blue: 0.3671875, alpha: 1.0 });

const PROGRESS_BAR_BORDER_SIZE: Vec2 = Vec2::new(FOREGROUND_IMAGE_SIZE.x * 0.8, 150.0);
const PROGRESS_BAR_INTERNAL_SIZE: Vec2 = Vec2::new(FOREGROUND_IMAGE_SIZE.x * 0.8 - 50.0, 100.0);
const PROGRESS_BAR_TOTAL_UNITS: usize = LILGUYS_COUNT * 4 + 8 + 2;
const PROGRESS_BAR_BORDER_COLOR: Color = Color::Srgba(Srgba { red: 0.99609375, green: 0.65234375, blue: 0.16796875, alpha: 1.0, });
const PROGRESS_BAR_INTERNAL_COLOR: Color = Color::Srgba(Srgba { red: 0.0, green: 0.40234375, blue: 0.44921875, alpha: 1.0, });

const WIN_SCREEN_IMAGE_PATH: &str = "WinBox.png";
const LOSE_SCREEN_IMAGE_PATH: &str = "LoseBox.png";
const NEXT_MISSION_SCREEN_IMAGE_PATH: &str = "NextMissionBox.png";
const MESSAGE_BOX_IMAGE_SIZE: Vec2 = Vec2::new(1455.0, 1458.0);
const MESSAGE_CONTINUE_BUTTON_POS: Vec2 = Vec2::new(-125.0, -550.0);
const MESSAGE_CONTINUE_BUTTON_SIZE: Vec2 = Vec2::new(750.0, 150.0);
const MESSAGE_EXIT_BUTTON_POS: Vec2 = Vec2::new(370.0, -550.0);
const MESSAGE_EXIT_BUTTON_SIZE: Vec2 = Vec2::new(150.0, 150.0);

const CURSOR_IMAGE_PATH: &str = "Cursor.png";
const CURSOR_IMAGE_OFFSET: Vec2 = Vec2::new(20.0, -30.0);
const CURSOR_IMAGE_SIZE: Vec2 = Vec2::new(131.5, 157.0);

const Z_POS_CURSOR: f32 = 13.0;
const Z_POS_MESSAGE_BOX: f32 = 12.0;
const Z_POS_MESSAGE_OVERLAY: f32 = 11.0;
const Z_POS_FACEPLATE: f32 = 10.0;
const Z_POS_MONITORS: f32 = 9.0;
const Z_POS_SELECTED_OVERLAY: f32 = 8.0;
const Z_POS_BACKGROUND: f32 = 0.0;
const Z_POS_LIL_GUYS: f32 = 1.0;

const MUSIC_AUDIO_PATH: &str = "audio/BackingTrack.downsampled.wav";
const CLICK_SOUND_PATH: &str = "audio/ClickSound.wav";
const CLICK_SOUND_VOLUME: f32 = 0.6;

struct LilGuyInfo {
    spawn_pos: Vec2,
    bg_image_size: Vec2,
    bg_click_area: ClickArea,
    bg_click_offset: Vec2,
    zoom_image_size: Vec2,
    bg_image_path: &'static str,
    zoom_image_path: &'static str,
    info_monitor_image_path: &'static str,
    mission_monitor_image_path: &'static str,
}

const LILGUYS_COUNT: usize = 19;
const LILGUYS_BESTIARY: [LilGuyInfo; LILGUYS_COUNT] = [
    LilGuyInfo {
        spawn_pos: Vec2::new(3678.0, 407.0),
        bg_image_size: Vec2::new(213.0, 147.0),
        bg_click_area: ClickArea::Circular(96.0),
        bg_click_offset: Vec2::ZERO,
        zoom_image_size: Vec2::new(1293.0, 1003.0),
        bg_image_path: "lilguys_back/Abogus.png",
        zoom_image_path: "lilguys_zoomed/Abogus.png",
        info_monitor_image_path: "info_monitors/Abogus.png",
        mission_monitor_image_path: "mission_monitors/Abogus.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(5888.0, 428.0),
        bg_image_size: Vec2::new(301.0, 438.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(200.0, 400.0)),
        bg_click_offset: Vec2::ZERO,
        zoom_image_size: Vec2::new(970.0, 1404.0),
        bg_image_path: "lilguys_back/Biblet.png",
        zoom_image_path: "lilguys_zoomed/Biblet.png",
        info_monitor_image_path: "info_monitors/Biblet.png",
        mission_monitor_image_path: "mission_monitors/Biblet.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-1138.0, 461.0),
        bg_image_size: Vec2::new(373.0, 458.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(300.0, 400.0)),
        bg_click_offset: Vec2::ZERO,
        zoom_image_size: Vec2::new(1046.0, 1288.0),
        bg_image_path: "lilguys_back/Bloober.png",
        zoom_image_path: "lilguys_zoomed/Bloober.png",
        info_monitor_image_path: "info_monitors/Bloober.png",
        mission_monitor_image_path: "mission_monitors/Bloober.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-7325.0, -613.0),
        bg_image_size: Vec2::new(372.0, 289.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(300.0, 225.0)),
        bg_click_offset: Vec2::ZERO,
        zoom_image_size: Vec2::new(1488.0, 868.0),
        bg_image_path: "lilguys_back/ChetTimbo.png",
        zoom_image_path: "lilguys_zoomed/ChetTimbo.png",
        info_monitor_image_path: "info_monitors/ChetTimbo.png",
        mission_monitor_image_path: "mission_monitors/ChetTimbo.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(2763.0, -494.0),
        bg_image_size: Vec2::new(385.0, 373.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(300.0, 300.0)),
        bg_click_offset: Vec2::ZERO,
        zoom_image_size: Vec2::new(1232.0, 1020.0),
        bg_image_path: "lilguys_back/Feetta.png",
        zoom_image_path: "lilguys_zoomed/Feetta.png",
        info_monitor_image_path: "info_monitors/Feetta.png",
        mission_monitor_image_path: "mission_monitors/Feetta.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(7242.0, 615.0),
        bg_image_size: Vec2::new(526.0, 625.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(475.0, 575.0)),
        bg_click_offset: Vec2::ZERO,
        zoom_image_size: Vec2::new(1187.0, 1402.0),
        bg_image_path: "lilguys_back/Gloober.png",
        zoom_image_path: "lilguys_zoomed/Gloober.png",
        info_monitor_image_path: "info_monitors/Gloober.png",
        mission_monitor_image_path: "mission_monitors/Gloober.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-2176.0, -42.0),
        bg_image_size: Vec2::new(254.0, 274.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(200.0, 225.0)),
        bg_click_offset: Vec2::ZERO,
        zoom_image_size: Vec2::new(1108.0, 1374.0),
        bg_image_path: "lilguys_back/Golyp.png",
        zoom_image_path: "lilguys_zoomed/Golyp.png",
        info_monitor_image_path: "info_monitors/Golyp.png",
        mission_monitor_image_path: "mission_monitors/Golyp.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(37.0, -298.0),
        bg_image_size: Vec2::new(1558.0, 802.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(1500.0, 700.0)),
        bg_click_offset: Vec2::ZERO,
        zoom_image_size: Vec2::new(1108.0, 1374.0),
        bg_image_path: "lilguys_back/Jerry.png",
        zoom_image_path: "lilguys_zoomed/Jerry.png",
        info_monitor_image_path: "info_monitors/Jerry.png",
        mission_monitor_image_path: "mission_monitors/Jerry.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(627.0, 250.0),
        bg_image_size: Vec2::new(519.0, 363.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(450.0, 300.0)),
        bg_click_offset: Vec2::ZERO,
        zoom_image_size: Vec2::new(1411.0, 995.0),
        bg_image_path: "lilguys_back/Jurpils.png",
        zoom_image_path: "lilguys_zoomed/Jurpils.png",
        info_monitor_image_path: "info_monitors/Jurpils.png",
        mission_monitor_image_path: "mission_monitors/Jurpils.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-4166.0, 469.0),
        bg_image_size: Vec2::new(285.0, 302.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(225.0, 250.0)),
        bg_click_offset: Vec2::ZERO,
        zoom_image_size: Vec2::new(1320.0, 839.0),
        bg_image_path: "lilguys_back/Keif.png",
        zoom_image_path: "lilguys_zoomed/Keif.png",
        info_monitor_image_path: "info_monitors/Keif.png",
        mission_monitor_image_path: "mission_monitors/Keif.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-6910.0, -240.0),
        bg_image_size: Vec2::new(348.0, 463.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(300.0, 425.0)),
        bg_click_offset: Vec2::ZERO,
        zoom_image_size: Vec2::new(1077.0, 1327.0),
        bg_image_path: "lilguys_back/Nyada.png",
        zoom_image_path: "lilguys_zoomed/Nyada.png",
        info_monitor_image_path: "info_monitors/Nyada.png",
        mission_monitor_image_path: "mission_monitors/Nyada.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(6466.0, -337.0),
        bg_image_size: Vec2::new(246.0, 326.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(200.0, 350.0)),
        bg_click_offset: Vec2::ZERO,
        zoom_image_size: Vec2::new(1277.0, 1073.0),
        bg_image_path: "lilguys_back/Ooples.png",
        zoom_image_path: "lilguys_zoomed/Ooples.png",
        info_monitor_image_path: "info_monitors/Ooples.png",
        mission_monitor_image_path: "mission_monitors/Ooples.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-3355.0, -46.0),
        bg_image_size: Vec2::new(955.0, 830.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(900.0, 775.0)),
        bg_click_offset: Vec2::ZERO,
        zoom_image_size: Vec2::new(1344.0, 1077.0),
        bg_image_path: "lilguys_back/Patootoo.png",
        zoom_image_path: "lilguys_zoomed/Patootoo.png",
        info_monitor_image_path: "info_monitors/Patootoo.png",
        mission_monitor_image_path: "mission_monitors/Patootoo.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-5710.0, -49.0),
        bg_image_size: Vec2::new(858.0, 1437.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(800.0, 800.0)),
        bg_click_offset: Vec2::new(0.0, 300.0),
        zoom_image_size: Vec2::new(867.0, 1452.0),
        bg_image_path: "lilguys_back/Qwoud.png",
        zoom_image_path: "lilguys_zoomed/Qwoud.png",
        info_monitor_image_path: "info_monitors/Qwoud.png",
        mission_monitor_image_path: "mission_monitors/Qwoud.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-5860.0, -506.0),
        bg_image_size: Vec2::new(195.0, 245.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(150.0, 200.0)),
        bg_click_offset: Vec2::ZERO,
        zoom_image_size: Vec2::new(836.0, 1358.0),
        bg_image_path: "lilguys_back/Snarfblat.png",
        zoom_image_path: "lilguys_zoomed/Snarfblat.png",
        info_monitor_image_path: "info_monitors/Snarfblat.png",
        mission_monitor_image_path: "mission_monitors/Snarfblat.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(4638.0, -415.0),
        bg_image_size: Vec2::new(721.0, 838.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(650.0, 775.0)),
        bg_click_offset: Vec2::ZERO,
        zoom_image_size: Vec2::new(1022.0, 1346.0),
        bg_image_path: "lilguys_back/Squapple.png",
        zoom_image_path: "lilguys_zoomed/Squapple.png",
        info_monitor_image_path: "info_monitors/Squapple.png",
        mission_monitor_image_path: "mission_monitors/Squapple.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(-7375.0, 590.0),
        bg_image_size: Vec2::new(338.0, 212.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(275.0, 175.0)),
        bg_click_offset: Vec2::ZERO,
        zoom_image_size: Vec2::new(1348.0, 974.0),
        bg_image_path: "lilguys_back/Thit.png",
        zoom_image_path: "lilguys_zoomed/Thit.png",
        info_monitor_image_path: "info_monitors/Thit.png",
        mission_monitor_image_path: "mission_monitors/Thit.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(1842.0, -217.0),
        bg_image_size: Vec2::new(411.0, 468.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(375.0, 400.0)),
        bg_click_offset: Vec2::ZERO,
        zoom_image_size: Vec2::new(659.0, 1351.0),
        bg_image_path: "lilguys_back/Toober.png",
        zoom_image_path: "lilguys_zoomed/Toober.png",
        info_monitor_image_path: "info_monitors/Toober.png",
        mission_monitor_image_path: "mission_monitors/Toober.png",
    },
    LilGuyInfo {
        spawn_pos: Vec2::new(7621.0, -416.0),
        bg_image_size: Vec2::new(170.0, 258.0),
        bg_click_area: ClickArea::Rectangular(Vec2::new(125.0, 200.0)),
        bg_click_offset: Vec2::ZERO,
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
    GameWin,
}

fn main() {
    App::new()
    .add_event::<LilGuySelected>()
    .add_event::<LilGuyDeselected>()
    .add_event::<LilGuySubmitted>()
    .add_event::<ChangeInfoPage>()
    .add_event::<ButtonClicked>()
    .insert_resource(UiActions::default())
    .insert_resource(StopScrolling::default())
    .insert_resource(LilGuySelection::default())
    .insert_resource(ImageHandles::default())
    .insert_resource(AudioHandles::default())
    .insert_resource(TargetLilGuy::default())
    .insert_resource(CurrentInfoPage::default())
    .insert_resource(CorrectSubmissions::default())
    .add_plugins(
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
        )
    )
    .insert_state(GameState::default())
    .add_systems(OnEnter(GameState::Loading), (
        spawn_camera,
        start_load_images,
        start_load_audio,
        spawn_progress_bar,
    ))
    .add_systems(Update, (
        monitor_loading,
        update_progress_bar,
    ).run_if(in_state(GameState::Loading)))
    .add_systems(OnExit(GameState::Loading), (
        remove_progress_bar,
        start_playing_music,
    ))
    .add_systems(OnEnter(GameState::Title), (
        remove_game_items,
        spawn_title_screen,
        spawn_start_button,
    ))
    .add_systems(OnExit(GameState::Title), (
        remove_title_screen,
        remove_start_button,
        spawn_exit_button,
        spawn_faceplate,
        spawn_background,
        spawn_border_blocks,
        spawn_scroll_buttons,
        spawn_back_button,
        spawn_send_button,
        spawn_info_buttons,
        spawn_monitors,
        spawn_cursor,
        reset_submissions,
    ))
    .add_systems(OnExit(GameState::Title), (
        spawn_lilguys,
    ).after(spawn_background))
    .add_systems(OnEnter(GameState::Game), (
        deselect_lilguy,
        choose_target_lilguy,
    ))
    .add_systems(OnEnter(GameState::Game), (
        update_mission_monitor
    ).after(choose_target_lilguy))
    .add_systems(Update, (
        scroll_background,
        handle_change_page,
    ).before(check_button_clicked).run_if(in_state(GameState::Game)))
    .add_systems(PostUpdate, (
        update_cursor,
        handle_scrolling,
        update_faceplate,
    ).after(check_button_clicked).run_if(in_state(GameState::Game)))
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
        remove_message_box,
        remove_message_buttons,
    ))
    .add_systems(OnEnter(GameState::GameWin), (
        spawn_winscreen,
        spawn_end_message_buttons,
    ))
    .add_systems(OnExit(GameState::GameWin), (
        remove_message_box,
        remove_message_buttons,
        remove_lilguys,
        reset_background_position,
        reset_submissions,
    ))
    .add_systems(OnExit(GameState::GameWin), (
        spawn_lilguys,
    ).after(remove_lilguys))
    .add_systems(OnEnter(GameState::GameOver), (
        spawn_losescreen,
        spawn_end_message_buttons,
    ))
    .add_systems(OnExit(GameState::GameOver), (
        remove_message_box,
        remove_message_buttons,
        remove_lilguys,
        reset_background_position,
        reset_submissions,
    ))
    .add_systems(OnExit(GameState::GameOver), (
        spawn_lilguys,
    ).after(remove_lilguys))
    .add_systems(Update, (
        resize_foreground,
        deselect_on_esc,
        check_button_clicked,
        toggle_fullscreen,
    ))
    .add_systems(PostUpdate, (
        // debug_draw_buttons,
        handle_lilguy_selected,
        handle_lilguy_deselected,
        handle_lilguy_submitted,
        handle_button_clicked,
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

fn update_cursor(
    mut cursor: Query<(&mut Transform, &mut Visibility), With<CursorImage>>,
    mut windows: Query<&mut Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
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

fn start_playing_music(
    audio: Res<AudioHandles>,
    mut commands: Commands
) {
    let Some(music_handle) = &audio.bg_music else { return; };
    commands.spawn(
        AudioBundle {
            source: music_handle.clone(),
            settings: PlaybackSettings::LOOP,
            ..default()
        }
    );
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

fn deselect_lilguy(
    mut deselect: EventWriter<LilGuyDeselected>,
) {
    deselect.send(LilGuyDeselected);
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

fn remove_message_buttons(
    buttons: Query<Entity, With<MessageButton>>,
    mut commands: Commands,
) {
    for button in &buttons {
        commands.entity(button).despawn();
    }
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

fn remove_message_box(
    message_boxes: Query<Entity, With<MessageBox>>,
    overlays: Query<Entity, With<MessageOverlay>>,
    mut commands: Commands,
) {
    for entity in &message_boxes {
        commands.entity(entity).despawn();
    }
    for entity in &overlays {
        commands.entity(entity).despawn();
    }
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
            material: materials.add(ColorMaterial::from_color(PROGRESS_BAR_BORDER_COLOR)),
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
                material: materials.add(ColorMaterial::from_color(PROGRESS_BAR_INTERNAL_COLOR)),
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
            active_on: ActiveStates::Title,
        },
        GameItem,
    ));
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

#[derive(Resource, Default)]
struct AudioHandles {
    bg_music: Option<Handle<AudioSource>>,
    click: Option<Handle<AudioSource>>,
}

fn start_load_audio(
    asset_server: Res<AssetServer>,
    mut audio_handles: ResMut<AudioHandles>,
) {
    audio_handles.bg_music = Some(asset_server.load(MUSIC_AUDIO_PATH));
    audio_handles.click = Some(asset_server.load(CLICK_SOUND_PATH));
}

#[derive(Resource, Default)]
struct ImageHandles {
    title_screen: Option<Handle<Image>>,
    foreground: Option<Handle<Image>>,
    foreground_no_buttons: Option<Handle<Image>>,
    background: Option<Handle<Image>>,
    win_screen: Option<Handle<Image>>,
    lose_screen: Option<Handle<Image>>,
    next_mission_screen: Option<Handle<Image>>,
    cursor: Option<Handle<Image>>,
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
    handles.foreground_no_buttons = Some(assets.load(FOREGROUND_NO_BUTTONS_IMAGE_PATH));
    handles.background = Some(assets.load(BACKGROUND_IMAGE_PATH));
    handles.win_screen = Some(assets.load(WIN_SCREEN_IMAGE_PATH));
    handles.lose_screen = Some(assets.load(LOSE_SCREEN_IMAGE_PATH));
    handles.next_mission_screen = Some(assets.load(NEXT_MISSION_SCREEN_IMAGE_PATH));
    handles.cursor = Some(assets.load(CURSOR_IMAGE_PATH));

    for i in 0..LILGUYS_COUNT {
        handles.lilguys_back[i] = Some(assets.load(LILGUYS_BESTIARY[i].bg_image_path));
        handles.lilguys_zoomed[i] = Some(assets.load(LILGUYS_BESTIARY[i].zoom_image_path));
        handles.lilguys_info_monitor[i] = Some(assets.load(LILGUYS_BESTIARY[i].info_monitor_image_path));
        handles.lilguys_mission_monitor[i] = Some(assets.load(LILGUYS_BESTIARY[i].mission_monitor_image_path));
    }
}

fn monitor_loading(
    images: Res<ImageHandles>,
    audio: Res<AudioHandles>,
    assets: Res<AssetServer>,
    mut game_state: ResMut<NextState<GameState>>,
    mut progress: Query<&mut ProgressBar>,
) {
    let mut current_progress = 0;

    if asset_is_loaded(&assets, &audio.bg_music) {
        current_progress += 1;
    }
    if asset_is_loaded(&assets, &audio.click) {
        current_progress += 1;
    }
    if asset_is_loaded(&assets, &images.title_screen) {
        current_progress += 1;
    }
    if asset_is_loaded(&assets, &images.foreground) {
        current_progress += 1;
    }
    if asset_is_loaded(&assets, &images.foreground_no_buttons) {
        current_progress += 1;
    }
    if asset_is_loaded(&assets, &images.background) {
        current_progress += 1;
    }
    if asset_is_loaded(&assets, &images.win_screen) {
        current_progress += 1;
    }
    if asset_is_loaded(&assets, &images.lose_screen) {
        current_progress += 1;
    }
    if asset_is_loaded(&assets, &images.cursor) {
        current_progress += 1;
    }
    if asset_is_loaded(&assets, &images.next_mission_screen) {
        current_progress += 1;
    }
    for i in 0..LILGUYS_COUNT {
        if asset_is_loaded(&assets, &images.lilguys_back[i]) {
            current_progress += 1;
        }
        if asset_is_loaded(&assets, &images.lilguys_zoomed[i]) {
            current_progress += 1;
        }
        if asset_is_loaded(&assets, &images.lilguys_info_monitor[i]) {
            current_progress += 1;
        }
        if asset_is_loaded(&assets, &images.lilguys_mission_monitor[i]) {
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

fn asset_is_loaded<T: Asset>(asset_server: &AssetServer, image_handle: &Option<Handle<T>>) -> bool {
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
    let material = materials.add(ColorMaterial::from_color(BACKGROUND_COLOR));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            material: material.clone(),
            transform: Transform::from_translation(Vec3::new(FOREGROUND_IMAGE_SIZE.x, 0.0, Z_POS_MONITORS)),
            ..default()
        },
        GameItem,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            material,
            transform: Transform::from_translation(Vec3::new(-FOREGROUND_IMAGE_SIZE.x, 0.0, Z_POS_MONITORS)),
            ..default()
        },
        GameItem,
    ));
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
    active_on: ActiveStates,
}

#[derive(Clone, Copy)]
enum ClickArea {
    Circular(f32),
    Rectangular(Vec2)
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ActionTypes {
    ReturnToTitle,
    ScrollLeft,
    ScrollRight,
    ZoomLilguy(u8),
    UnZoomLilguy,
    SendToLab,
    StartGame,
    StartNextLevel,
    InfoPageLeft,
    InfoPageRight,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum ActiveStates {
    None = 0,
    Title = 1,
    Game = 2,
    Message = 4,
}

#[derive(Component)]
struct ExitButton;

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
            action: ActionTypes::ReturnToTitle,
            behavior: ClickBehaviors::SingleClick,
            active_on: ActiveStates::Game,
        },
        ExitButton,
        GameItem,
    ));
}

#[derive(Component)]
struct GameItem;

fn remove_game_items(
    items: Query<Entity, With<GameItem>>,
    mut commands: Commands
) {
    for item in &items {
        commands.entity(item).despawn_recursive();
    }
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
            active_on: ActiveStates::Game,
        },
        GameItem,
    ));
}

#[derive(Resource, Default)]
struct CurrentInfoPage {
    page_idx: usize,
}

#[derive(Event)]
enum ChangeInfoPage {
    PageLeft,
    PageRight,
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

fn spawn_info_buttons(
    mut commands: Commands,
) {
    commands.spawn_batch(
    [
        (
            SpatialBundle {
                transform: Transform::from_translation(FOREGROUND_INFO_PAGELEFT_POS.extend(0.0)),
                ..default()
            },
            Clickable {
                area: ClickArea::Circular(FOREGROUND_INFO_BUTTON_RAD),
                action: ActionTypes::InfoPageLeft,
                behavior: ClickBehaviors::SingleClick,
                active_on: ActiveStates::Game,
            },
            GameItem,
        ),
        (
            SpatialBundle {
                transform: Transform::from_translation(FOREGROUND_INFO_PAGERIGHT_POS.extend(0.0)),
                ..default()
            },
            Clickable {
                area: ClickArea::Circular(FOREGROUND_INFO_BUTTON_RAD),
                action: ActionTypes::InfoPageRight,
                behavior: ClickBehaviors::SingleClick,
                active_on: ActiveStates::Game,
            },
            GameItem,
        ),
    ]);
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
            active_on: ActiveStates::Game,
        },
        GameItem,
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

fn remove_lilguys(
    lilguys: Query<Entity, With<LilGuy>>,
    mut commands: Commands,
) {
    for lilguy in &lilguys {
        commands.entity(lilguy).despawn_recursive();
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
            active_on: ActiveStates::Game,
        },
        GameItem,
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
            active_on: ActiveStates::Game,
        },
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

fn scroll_background(
    input: Res<ButtonInput<KeyCode>>,
    mut ui_actions: ResMut<UiActions>,
    mut on_click: EventWriter<ButtonClicked>,
) {
    const LEFT_KEYS: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
    const RIGHT_KEYS: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];

    ui_actions.scrolling_left = input.any_pressed(LEFT_KEYS);
    ui_actions.scrolling_right = input.any_pressed(RIGHT_KEYS);

    if input.any_just_pressed(LEFT_KEYS) || input.any_just_pressed(RIGHT_KEYS) {
        on_click.send_default();
    }
}

#[derive(Event)]
struct LilGuySelected {
    lilguy_id: u8
}

#[derive(Event, Default)]
struct ButtonClicked;

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
    mut on_info_page_changed: EventWriter<ChangeInfoPage>,
    current_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut on_click: EventWriter<ButtonClicked>,
) {
    if input.just_released(MouseButton::Left) {
        reset_ui_actions(&mut ui_actions);
        return;
    }
    if !input.pressed(MouseButton::Left) {
        return;
    }
    let just_clicked = input.just_pressed(MouseButton::Left);

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

    let current_active_state = 
        match current_state.get() {
            GameState::Loading => ActiveStates::None,
            GameState::Title => ActiveStates::Title,
            GameState::Game => ActiveStates::Game,
            GameState::GameWin | GameState::NextLevel | GameState::GameOver => ActiveStates::Message
        };

    for (btn_transform, btn) in &btns {

        if (current_active_state as u8 & btn.active_on as u8) == 0 {
            continue;
        }
        
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
        
        let button_clicked = 
            match btn.action {
                ActionTypes::ReturnToTitle => if button_pressed { next_game_state.set(GameState::Title); true } else { false },
                ActionTypes::ScrollLeft => if selection.zoomed_lilguy_id.is_none() { ui_actions.scrolling_left = button_pressed; button_pressed } else { false },
                ActionTypes::ScrollRight => if selection.zoomed_lilguy_id.is_none() { ui_actions.scrolling_right = button_pressed; button_pressed } else { false },
                ActionTypes::ZoomLilguy(id) => if button_pressed && cursor_in_porthole { _ = on_lilguy_selected.send(LilGuySelected { lilguy_id: id }); true } else { false },
                ActionTypes::UnZoomLilguy => 
                    if button_pressed {
                        if selection.zoomed_lilguy_id.is_some() {
                            _ = on_lilguy_deselected.send(LilGuyDeselected);
                            true
                        } else { 
                            false 
                        }
                    } else {
                        false
                    },
                ActionTypes::SendToLab => 
                    if button_pressed {
                        if let Some(lilguy) = selection.zoomed_lilguy_id {
                            _ = on_lilguy_submitted.send(LilGuySubmitted {
                                lilguy_id_guess: lilguy
                            });
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    },
                ActionTypes::StartGame => if button_pressed { next_game_state.set(GameState::Game); true } else { false },
                ActionTypes::StartNextLevel => if button_pressed { next_game_state.set(GameState::Game); true } else { false },
                ActionTypes::InfoPageLeft => if button_pressed { _ = on_info_page_changed.send(ChangeInfoPage::PageLeft); true } else { false },
                ActionTypes::InfoPageRight => if button_pressed { _ = on_info_page_changed.send(ChangeInfoPage::PageRight); true } else { false },
            };
        if button_clicked && just_clicked {
            on_click.send_default();
        }
    }
}

fn reset_ui_actions(
    actions: &mut UiActions
) {
    actions.scrolling_left = false;
    actions.scrolling_right = false;
}

#[derive(Resource, Default)]
struct UiActions {
    scrolling_left: bool,
    scrolling_right: bool,
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
    zoomed_lilguy_id: Option<u8>,
}

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
        stop_scrolling.value = true;
    }
}

fn handle_button_clicked(
    mut clicked: EventReader<ButtonClicked>,
    audio: Res<AudioHandles>,
    mut commands: Commands,
) {
    if clicked.is_empty() {
        return;
    }
    clicked.clear();

    let Some(click) = &audio.click else { return; };
    commands.spawn(AudioBundle {
        source: click.clone(),
        settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(CLICK_SOUND_VOLUME)),
        ..default()
    });
}

#[derive(Event)]
struct LilGuyDeselected;

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
    stop_scrolling.value = false;
    for overlay in &overlays {
        commands.entity(overlay).despawn();
    }
}

// fn debug_draw_buttons(
//     mut gizmos: Gizmos,
//     buttons: Query<(&GlobalTransform, &Clickable)>,
// ) {
//     for (button_pos, button) in &buttons {
//         // it's assumed that everything except the progress bar is scaled squarely
//         let (scale, _, _) = button_pos.to_scale_rotation_translation();
//         match button.area {
//             ClickArea::Circular(rads) => { gizmos.circle_2d(button_pos.translation().xy(), rads * scale.x, RED); },
//             ClickArea::Rectangular(area) => { gizmos.rect_2d(button_pos.translation().xy(), Rot2::IDENTITY, area * scale.x, RED); },
//         }
//     }
// }