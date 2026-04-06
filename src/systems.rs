use bevy::prelude::*;

use crate::{
    components::{Name, Person},
    resources::GreetTimer,
};

pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

pub fn hello_world() {
    println!("hello world!");
}

pub fn update_people(mut query: Query<&mut crate::components::Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break;
        }
    }
}

pub fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&crate::components::Name, With<Person>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for name in &query {
        println!("hello {}!", name.0);
    }
}
