use bevy::prelude::*;

pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_state(GameState::default())
        .insert_resource(LilGuySelection::default())
        .add_event::<LilGuySelected>()
        .add_event::<LilGuyDeselected>()
        .add_event::<LilGuySubmitted>()
        .add_event::<ChangeInfoPage>()
        .add_event::<ScrollDirections>();
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Title,
    Game,
    NextLevel,
    GameOver,
    GameWin,
}

#[derive(Event)]
pub enum ScrollDirections {
    ScrollLeft,
    ScrollRight,
}

#[derive(Event, Default)]
pub struct LilGuySelected {
    pub lilguy_id: u8
}

#[derive(Event, Default)]
pub struct LilGuySubmitted {
    pub lilguy_id_guess: u8,
}

#[derive(Event)]
pub enum ChangeInfoPage {
    PageLeft,
    PageRight,
}


#[derive(Event, Default)]
pub struct LilGuyDeselected;

#[derive(Resource, Default, Debug)]
pub struct LilGuySelection {
    pub zoomed_lilguy_id: Option<u8>,
}