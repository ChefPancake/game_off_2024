use bevy::audio::Volume;
use bevy::prelude::*;
use crate::clickable::*;
use crate::core::*;
use crate::data::*;
use crate::handles::*;

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ImageHandles::default())
        .insert_resource(AudioHandles::default())
        .add_systems(OnExit(GameState::Loading), (
            start_playing_music,
        ))
        .add_systems(PostUpdate, (
            handle_button_clicked,
        ));
    }
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