use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_uma)
        .run();
}

#[derive(Component)]
struct Uma;

#[derive(Component)]
struct Name(String);

fn add_uma(mut commands: Commands) {
    commands.spawn((Uma, Name("Dariz".to_string())));
    commands.spawn((Uma, Name("Calandagan".to_string())));
    commands.spawn((Uma, Name("Aventure".to_string())));    
    commands.spawn((Uma, Name("Sosie".to_string())));
    commands.spawn((Uma, Name("Quisisana".to_string())));
    commands.spawn((Uma, Name("Gezora".to_string())));
}




