use bevy::{prelude::*, winit::WinitSettings, sprite::MaterialMesh2dBundle};
use bevy_prototype_lyon::prelude::ShapePlugin;
use rocks_plugin::RockBundle;
use std::fs::File;
use std::io::prelude::*;
use serde_json;
use serde::{Serialize, Deserialize};

#[path="../rocks_plugin.rs"]
mod rocks_plugin;
use rocks_plugin::*;

#[derive(Resource)]
struct CursorPos(Vec2);

#[derive(Serialize, Deserialize, Resource)]
struct FileInfo {
    border_points: Vec<[f32; 2]>,
    anchors: Vec<[f32; 2]>,
    end: [f32; 2]
}

#[derive(Resource)]
struct FileRef(File);

fn main() {
    let choice = get_choice(
        "What would you like to do?", 
            vec![
                "Create a level",
                "Modify a level",
                "Delete a level"
            ]
    );

    let file_name = get_file_name(choice);

    if &file_name[..] == "" { return; }

    let mut file_info = FileInfo {
        border_points: Vec::new(),
        anchors: Vec::new(),
        end: [0.0, 200.0]
    };

    let file_path = format!("src/bin/levels/{}.json", file_name);

    let file = match choice {
        0 => {
            let mut file = File::create(file_path).unwrap();
            let _ = file.write(serde_json::to_string_pretty(&file_info).unwrap().as_bytes());

            file
        },
        1 => {
            let mut file = File::open(file_path).unwrap();
            let mut contents = String::new();
            let _ = file.read_to_string(&mut contents);

            file_info = serde_json::from_str(&contents).unwrap();

            file
        },
        2 => {
            let _ = std::fs::remove_file(file_path);
            return;
        },
        _ => { return; }
    };

    App::new()
    .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(file_info)
        .insert_resource(FileRef(file))
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(CursorPos(Vec2::ZERO))

        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bevy-ier curves [Level Editor]".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(ShapePlugin)
        // .add_plugin(RocksPlugin)
        .add_startup_system(setup)
        .add_system(add_point_on_click)
        .add_system(set_cursor_pos)
        .add_system(save_to_file)
        .add_system(add_point_undo)
        .run()
    ;
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    file_info: Res<FileInfo>
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::default().into()).into(),
        material: materials.add(ColorMaterial::from(Color::GREEN)),
        transform: Transform
            ::from_xyz(0.0, 0.0, 3.0)
            .with_scale(Vec3::new(10.0, 10.0, 10.0)),
        ..default()
    });

    if file_info.border_points.len() > 1 {
        commands.spawn(RockBundle::new(
            file_info.border_points.iter().map(|[x, y]| Vec2::new(*x, *y)).collect()
        ));
    }
}

fn add_point_on_click(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    mut file_info: ResMut<FileInfo>,
    cursor_pos: Res<CursorPos>,
    rock_entity_query: Query<Entity, With<PolygonPoints>>
) {
    if buttons.just_pressed(MouseButton::Left) {
        file_info.border_points.push([cursor_pos.0.x, cursor_pos.0.y]);

        if file_info.border_points.len() > 1 {
            if let Ok(rock_entity) = rock_entity_query.get_single() {
                commands.entity(rock_entity).despawn();
            }

            commands.spawn(RockBundle::new(
                file_info.border_points.iter().map(|[x, y]| Vec2::new(*x, *y)).collect()
            ));
        }
    }
}

fn add_point_undo(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut file_info: ResMut<FileInfo>,
    rock_entity_query: Query<Entity, With<PolygonPoints>>
) {
    if keys.pressed(KeyCode::Z) && keys.pressed(KeyCode::LWin) {
        file_info.border_points.pop();

        if let Ok(rock_entity) = rock_entity_query.get_single() {
            commands.entity(rock_entity).despawn();
        }

        if file_info.border_points.len() > 1 {
            commands.spawn(RockBundle::new(
                file_info.border_points.iter().map(|[x, y]| Vec2::new(*x, *y)).collect()
            ));
        }
    }
}

fn save_to_file(
    mut file: ResMut<FileRef>,
    file_info: Res<FileInfo>,
    keys: Res<Input<KeyCode>>

) {
    if keys.pressed(KeyCode::S) && keys.pressed(KeyCode::LWin) {
        let _ = file.0.rewind();
        let _ = file.0.write(serde_json::to_string_pretty(&*file_info).unwrap().as_bytes());
        let _ = file.0.sync_all();
    }
}

fn set_cursor_pos(
    mut cursor_pos: ResMut<CursorPos>,
    camera_transform_query: Query<&Transform, With<Camera>>,
    windows: Res<Windows>
) {
    let window = windows.get_primary().unwrap();
    let camera_pos = camera_transform_query.single().translation;

    if let Some(cursor_position) = window.cursor_position() {
        cursor_pos.0.x = cursor_position.x - window.width() / 2.0 + camera_pos.x;
        cursor_pos.0.y = cursor_position.y - window.height() / 2.0 + camera_pos.y;
    }
}

fn get_file_name(choice: usize) -> String {
    print!("\nEnter the name of the file: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let mut file_name = String::new();
    let _ = std::io::stdin().read_line(&mut file_name);
    file_name.pop();

    while  
        file_name.len() != 0 &&
        match choice {
            0 => { File::open(format!("src/bin/levels/{}.json", file_name)).is_ok() },
            1 | 2 => { File::open(format!("src/bin/levels/{}.json", file_name)).is_err() },
            _ => false
        } 
    {
        print!("\nTry again: ");
        std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
        file_name = String::new();
        let _ = std::io::stdin().read_line(&mut file_name);
        file_name.pop();
    }

    file_name
}

fn get_choice(prompt: &str, choices: Vec<&str>) -> usize {
    println!(
        "{}{}", 
        prompt, 
        choices.iter().enumerate().fold(
            String::new(), 
            |accum, (i, choice)| format!("{}\n   {}: {}", accum, i, choice)
        )
    );
    
    let mut index = String::new();
    let _ = std::io::stdin().read_line(&mut index);
    index.pop();

    let while_conditional = |str: &String|  -> bool {
        if let Ok(i) = str.parse::<usize>() {
            i > choices.len()
        } else {
            true
        }
    };

    while while_conditional(&index) {
        print!("\nTry again: ");
        index = String::new();
        let _ = std::io::stdin().read_line(&mut index);
        index.pop();
    };

    index.parse::<usize>().unwrap()
}