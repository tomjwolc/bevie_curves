use std::time::Duration;

use super::*;

pub struct LifetimePlugin;

impl Plugin for LifetimePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                .with_system(remove_dead)
            ).add_system_set(
                SystemSet::on_exit(AppState::InGame)
                .with_system(remove_remaining)
            )
        ;
    }
}

#[derive(Component)]
pub struct Lifetime {
    pub creation: Duration,
    pub lifespan: Duration
}

fn remove_dead(
    mut commands: Commands,
    lifetime_entities_query: Query<(&Lifetime, Entity)>,
    time: Res<Time>
) {
    for (lifetime, entity) in lifetime_entities_query.iter() {
        if time.elapsed() - lifetime.creation > lifetime.lifespan {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn remove_remaining(
    mut commands: Commands,
    lifetime_entities_query: Query<Entity, With<Lifetime>>
) {
    for entity in lifetime_entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}