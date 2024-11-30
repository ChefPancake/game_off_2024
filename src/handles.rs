use crate::data::*;
use crate::core::GameState;
use bevy::prelude::*;

pub struct HandlesPlugin;
impl Plugin for HandlesPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<LoadingProgressUpdated>()
        .insert_resource(ImageHandles::default())
        .insert_resource(AudioHandles::default())
        .add_systems(OnEnter(GameState::Loading), (
            start_load_audio,
            start_load_images,
        ))
        .add_systems(Update, (
            monitor_loading,
        ));
    }
}

#[derive(Event, Copy, Clone)]
pub struct LoadingProgressUpdated {
    pub total: usize,
    pub completed: usize,
}

#[derive(Resource, Default)]
pub struct ImageHandles {
    pub title_screen: Option<Handle<Image>>,
    pub foreground: Option<Handle<Image>>,
    pub foreground_no_buttons: Option<Handle<Image>>,
    pub background: Option<Handle<Image>>,
    pub win_screen: Option<Handle<Image>>,
    pub lose_screen: Option<Handle<Image>>,
    pub next_mission_screen: Option<Handle<Image>>,
    pub cursor: Option<Handle<Image>>,
    pub lilguys_back: [Option<Handle<Image>>; LILGUYS_COUNT],
    pub lilguys_zoomed: [Option<Handle<Image>>; LILGUYS_COUNT],
    pub lilguys_info_monitor: [Option<Handle<Image>>; LILGUYS_COUNT],
    pub lilguys_mission_monitor: [Option<Handle<Image>>; LILGUYS_COUNT],
}


#[derive(Resource, Default)]
pub struct AudioHandles {
    pub bg_music: Option<Handle<AudioSource>>,
    pub click: Option<Handle<AudioSource>>,
}


fn start_load_audio(
    asset_server: Res<AssetServer>,
    mut audio_handles: ResMut<AudioHandles>,
) {
    audio_handles.bg_music = Some(asset_server.load(MUSIC_AUDIO_PATH));
    audio_handles.click = Some(asset_server.load(CLICK_SOUND_PATH));
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
    mut progress: EventWriter<LoadingProgressUpdated>,
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

    progress.send(LoadingProgressUpdated { total: PROGRESS_BAR_TOTAL_UNITS, completed: current_progress });
}

fn asset_is_loaded<T: Asset>(asset_server: &AssetServer, image_handle: &Option<Handle<T>>) -> bool {
    use bevy::asset::LoadState;
    let Some(handle) = image_handle else { panic!("Image does not have a handle") };
    return asset_server.get_load_state(handle).is_some_and(|val| val == LoadState::Loaded);
}