use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::core::*;
use crate::handles::*;
use crate::data::*;

pub struct ProgressPlugin;

impl Plugin for ProgressPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Loading), (
            spawn_progress_bar,
        ))
        .add_systems(Update, (
            update_progress_bar,
        ).run_if(in_state(GameState::Loading)))
        .add_systems(OnExit(GameState::Loading), (
            remove_progress_bar,
        ));
    }
}


#[derive(Component)]
struct ProgressBar;

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
        ProgressBar,
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
    mut updates: EventReader<LoadingProgressUpdated>,
    bar_parents: Query<Entity, With<ProgressBar>>,
    mut bar_children: Query<(&Parent, &mut Transform)>,
) {
    if updates.is_empty() {
        return;
    }
    let mut first_update: Option<LoadingProgressUpdated> = None;
    for update in updates.read() {
        first_update = Some(update.clone());
        break;
    }
    updates.clear();
    let first_update = first_update.unwrap();

    const LEFTMOST_EDGE: f32 = PROGRESS_BAR_INTERNAL_SIZE.x / -2.0;
    for parent_entity in &bar_parents {
        for (bar_parent, mut bar_trans) in &mut bar_children {
            if **bar_parent == parent_entity {
                let new_width = PROGRESS_BAR_INTERNAL_SIZE.x * first_update.completed as f32 / first_update.total as f32;
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