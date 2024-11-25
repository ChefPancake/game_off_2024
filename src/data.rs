use bevy::prelude::*;
use crate::clickable::*;

pub const FOREGROUND_TITLE_SCREEN: &str = "TitleScreen.png";
pub const FOREGROUND_IMAGE_PATH: &str = "FrontPanel.png";
pub const FOREGROUND_IMAGE_SIZE: Vec2 = Vec2::new(3641.0, 2048.0);
pub const FOREGROUND_ASPECT_RATIO: f32 = FOREGROUND_IMAGE_SIZE.x / FOREGROUND_IMAGE_SIZE.y;

pub const FOREGROUND_TOP_MONITOR_IMAGE_SIZE: Vec2 = Vec2::new(1101.0, 1351.0);
pub const FOREGROUND_TOP_MONITOR_IMAGE_POS: Vec2 = Vec2::new(-1090.0, 180.0);
pub const FOREGROUND_BOTTOM_MONITOR_IMAGE_SIZE: Vec2 = Vec2::new(1101.0, 495.0);
pub const FOREGROUND_BOTTOM_MONITOR_IMAGE_POS: Vec2 = Vec2::new(-1090.0, -790.0);
pub const FOREGROUND_INFO_PAGELEFT_POS: Vec2 = Vec2::new(FOREGROUND_TOP_MONITOR_IMAGE_POS.x - 450.0, 725.0);
pub const FOREGROUND_INFO_PAGERIGHT_POS: Vec2 = Vec2::new(FOREGROUND_TOP_MONITOR_IMAGE_POS.x + 450.0, 725.0);
pub const FOREGROUND_INFO_BUTTON_RAD: f32 = 50.0;

pub const FOREGROUND_CLOSE_BUTTON_POS: Vec2 = Vec2::new(1650.0, 860.0);
pub const FOREGROUND_LEFT_BUTTON_POS: Vec2 = Vec2::new(-178.0, -900.0);
pub const FOREGROUND_BACK_BUTTON_POS: Vec2 = Vec2::new(140.0, -900.0);
pub const FOREGROUND_BACK_BUTTON_AREA: Vec2 = Vec2::new(400.0, 180.0);
pub const FOREGROUND_SEND_BUTTON_POS: Vec2 = Vec2::new(772.0, -900.0);
pub const FOREGROUND_SEND_BUTTON_AREA: Vec2 = Vec2::new(765.0, 180.0);
pub const FOREGROUND_RIGHT_BUTTON_POS: Vec2 = Vec2::new(1265.0, -900.0);
pub const FOREGROUND_SMALL_BUTTON_AREA: Vec2 = Vec2::new(180.0, 180.0);
pub const FOREGROUND_PORTHOLE_CENTER_POS: Vec2 = Vec2::new(530.0, 105.0);
pub const FOREGROUND_PORTHOLE_RAD: f32 = 780.0;

pub const BACKGROUND_IMAGE_PATH: &str = "Background_empty_smallest.png";
pub const BACKGROUND_IMAGE_SIZE: Vec2 = Vec2::new(16378.0, 2048.0);
pub const BACKGROUND_PADDING_SIZE: f32 = 5.0;
pub const BACKGROUND_START_POS: Vec2 = Vec2::new(0.0, FOREGROUND_PORTHOLE_CENTER_POS.y);
pub const BACKGROUND_MIN_X: f32 = FOREGROUND_PORTHOLE_CENTER_POS.x - BACKGROUND_IMAGE_SIZE.x * BACKGROUND_SCALE / 2.0 + FOREGROUND_PORTHOLE_RAD + BACKGROUND_PADDING_SIZE;
pub const BACKGROUND_MAX_X: f32 = FOREGROUND_PORTHOLE_CENTER_POS.x + BACKGROUND_IMAGE_SIZE.x * BACKGROUND_SCALE / 2.0 - FOREGROUND_PORTHOLE_RAD - BACKGROUND_PADDING_SIZE;
pub const BACKGROUND_SCROLL_MAX_SPEED: f32 = 1200.0;
pub const BACKGROUND_SCROLL_ACCEL: f32 = 10000.0;
pub const BACKGROUND_SCALE: f32 = (FOREGROUND_PORTHOLE_RAD + BACKGROUND_PADDING_SIZE) * 2.0 / BACKGROUND_IMAGE_SIZE.y;
pub const BACKGROUND_OVERLAY_COLOR: Color = Color::Srgba(Srgba { red: 0.0, green: 0.25, blue: 0.37109375, alpha: 0.8 });
pub const BACKGROUND_COLOR: Color = Color::Srgba(Srgba { red: 0.2421875, green: 0.375, blue: 0.3671875, alpha: 1.0 });

pub const PROGRESS_BAR_BORDER_SIZE: Vec2 = Vec2::new(FOREGROUND_IMAGE_SIZE.x * 0.8, 150.0);
pub const PROGRESS_BAR_INTERNAL_SIZE: Vec2 = Vec2::new(FOREGROUND_IMAGE_SIZE.x * 0.8 - 50.0, 100.0);
pub const PROGRESS_BAR_TOTAL_UNITS: usize = LILGUYS_COUNT * 4 + 7 + 2;
pub const PROGRESS_BAR_BORDER_COLOR: Color = Color::Srgba(Srgba { red: 0.99609375, green: 0.65234375, blue: 0.16796875, alpha: 1.0, });
pub const PROGRESS_BAR_INTERNAL_COLOR: Color = Color::Srgba(Srgba { red: 0.0, green: 0.40234375, blue: 0.44921875, alpha: 1.0, });

pub const WIN_SCREEN_IMAGE_PATH: &str = "WinBox.png";
pub const LOSE_SCREEN_IMAGE_PATH: &str = "LoseBox.png";
pub const NEXT_MISSION_SCREEN_IMAGE_PATH: &str = "NextMissionBox.png";
pub const MESSAGE_BOX_IMAGE_SIZE: Vec2 = Vec2::new(1455.0, 1458.0);
pub const MESSAGE_CONTINUE_BUTTON_POS: Vec2 = Vec2::new(-125.0, -550.0);
pub const MESSAGE_CONTINUE_BUTTON_SIZE: Vec2 = Vec2::new(750.0, 150.0);
pub const MESSAGE_EXIT_BUTTON_POS: Vec2 = Vec2::new(370.0, -550.0);
pub const MESSAGE_EXIT_BUTTON_SIZE: Vec2 = Vec2::new(150.0, 150.0);

pub const CURSOR_IMAGE_PATH: &str = "Cursor.png";
pub const CURSOR_IMAGE_OFFSET: Vec2 = Vec2::new(20.0, -30.0);
pub const CURSOR_IMAGE_SIZE: Vec2 = Vec2::new(131.5, 157.0);

pub const Z_POS_CURSOR: f32 = 13.0;
pub const Z_POS_MESSAGE_BOX: f32 = 12.0;
pub const Z_POS_MESSAGE_OVERLAY: f32 = 11.0;
pub const Z_POS_FACEPLATE: f32 = 10.0;
pub const Z_POS_MONITORS: f32 = 9.0;
pub const Z_POS_SELECTED_OVERLAY: f32 = 8.0;
pub const Z_POS_BACKGROUND: f32 = 0.0;
pub const Z_POS_LIL_GUYS: f32 = 1.0;

pub const MUSIC_AUDIO_PATH: &str = "audio/BackingTrack.downsampled.wav";
pub const CLICK_SOUND_PATH: &str = "audio/ClickSound.wav";
pub const CLICK_SOUND_VOLUME: f32 = 0.6;

pub struct LilGuyInfo {
    pub spawn_pos: Vec2,
    pub bg_image_size: Vec2,
    pub bg_click_area: ClickArea,
    pub bg_click_offset: Vec2,
    pub zoom_image_size: Vec2,
    pub bg_image_path: &'static str,
    pub zoom_image_path: &'static str,
    pub info_monitor_image_path: &'static str,
    pub mission_monitor_image_path: &'static str,
}

pub const LILGUYS_COUNT: usize = 19;
pub const LILGUYS_BESTIARY: [LilGuyInfo; LILGUYS_COUNT] = [
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
