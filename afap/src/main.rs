use bevy::MinimalPlugins;
use bevy::app::App;
use bevy::time::Time;
use bevy::time::Timer;
use bevy::time::TimerMode;
use bevy_ecs::query::With;
use bevy_ecs::system::Commands;
use bevy_ecs::system::Query;
use bevy_ecs::system::Res;
use bevy_ecs::system::ResMut;
use bevy_ecs::system::Resource;
use bevy_ecs::component::Component;

// ***************************************************************************
// ECS
// ***************************************************************************
// Entities are unique "things" that are assigned groups of Components,
// which are then processed using Systems.

// ***************************************************************************
// Resources
// ***************************************************************************

#[derive(Resource)]
struct GreetTimer(Timer);

// ***************************************************************************
// Startup Systems
// ***************************************************************************

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}


// ***************************************************************************
// Systems
// ***************************************************************************

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    } else {
        println!("hello {}!", time.delta().as_secs_f64());
    }
}

// ***************************************************************************
// Components
// ***************************************************************************

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);


// ***************************************************************************
// Main
// ***************************************************************************

fn main() {
    App::new()
    .add_plugins(MinimalPlugins)
    .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
    .add_startup_system(add_people)  // runs once before all other systems
    .add_system(greet_people)
    .run();
}
