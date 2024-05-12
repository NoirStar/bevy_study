use bevy::prelude::*;

fn hello_world() {
    println!("hello world!");
}

// 매크로를 사용하여 특정 트레잇을 자동 구현하도록 하는 지시어
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// Query 이것도 매크로, 제네릭 데이터 타입 ECS 시스템에서 특정 컴포넌트를 가진 엔티티들을 조회할 떄 사용됨
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Proctor" {
            name.0 = "Hume".to_string();
            break;
        }
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Proctor".to_string())));
    commands.spawn((Person, Name("Hume".to_string())));
    commands.spawn((Person, Name("Nieves".to_string())));
}


pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {

    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}
