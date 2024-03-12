use bevy::prelude::*;
use rand::{distributions::Alphanumeric, Rng};

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (update_people, greet_people).chain());
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    println!("Adding people");
    commands.spawn((Person, Name("Yogo".to_string())));
    commands.spawn((Person, Name("Sheena".to_string())));
    commands.spawn((Person, Name("Jialan".to_string())));
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    let mut rng = rand::thread_rng();
    for mut name in &mut query {
        let random_name: String = std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(10) // You can change the length of the name here
            .collect();
        name.0 = random_name;
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        print_every_person_name(query);
    }
}

fn print_every_person_name(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello, {}!", name.0);
    }
}
