use bevy::prelude::*;

struct ClickablePlugin;
impl Plugin for ClickablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_button_clicked);
    }
}

// HEY
// 
// we're going to consolidate all the different types of events that can be written into this
// then the core can handle it?
#[derive(Event)]
pub struct ClickableClicked {
    action_type: ActionTypes
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ClickBehaviors {
    SingleClick,
    ClickAndHold,
}

#[derive(Component, Clone)]
pub struct Clickable {
    pub area: ClickArea,
    pub action: ActionTypes,
    pub behavior: ClickBehaviors,
    pub active_on: ActiveStates,
}

#[derive(Clone, Copy)]
pub enum ClickArea {
    Circular(f32),
    Rectangular(Vec2)
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ActionTypes {
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
pub enum ActiveStates {
    None = 0,
    Title = 1,
    Game = 2,
    Message = 4,
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
                ActionTypes::ScrollLeft => if selection.zoomed_lilguy_id.is_none() { ui_actions.scrolling_left = button_pressed; true } else { false },
                ActionTypes::ScrollRight => if selection.zoomed_lilguy_id.is_none() { ui_actions.scrolling_right = button_pressed; true } else { false },
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