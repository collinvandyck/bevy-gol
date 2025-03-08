use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, setup_camera);
        app.add_systems(Startup, add_people);
        app.add_systems(Update, ((update_people, greet_people).chain()));
    }
}

#[derive(Component)]
struct Person;

#[derive(Component, derive_more::Display)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name(String::from("Collin"))));
    commands.spawn((Person, Name(String::from("Charity"))));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {name}");
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if &name.0 == &"Collin" {
            name.0.push_str(".");
        }
    }
}
