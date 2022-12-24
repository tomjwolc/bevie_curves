use bevy::prelude::*;
use std::fs::File;
use std::io::prelude::*;

#[path="../rocks_plugin.rs"]
mod rocks_plugin;

#[derive(Resource)]
struct BorderPoints(Vec<Vec2>);

#[derive(Resource)]
struct Start(Vec2);

#[derive(Resource)]
struct Anchors(Vec<Vec2>);

#[derive(Resource)]
struct End(Vec2);

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

    let mut file = match choice {
        0 => {
            let mut file = File::create(format!("src/bin/levels/{}.json", file_name)).unwrap();
            let _ = file.write_all("\
            {\n    \
                \"border_points\": [],\n    \
                \"start\": [],\n    \
                \"anchors\": [],\n    \
                \"end\": []\n\
            }".as_bytes());

            file
        },
        1 => {
            let file = File::open(format!("src/bin/levels/{}.json", file_name)).unwrap();

            // Read as json here!!!
            
            file
        },
        2 => {
            let _ = std::fs::remove_file(format!("src/bin/levels/{}.json", file_name));
            return;
        },
        _ => { return; }
    };

    let _ = file.write_all("\
    {\n    \
        \"border_points\": [],\n    \
        \"start\": [],\n    \
        \"anchors\": [],\n    \
        \"end\": []\n\
    }".as_bytes());

    App::new()
        .insert_resource(BorderPoints(Vec::new()))
        .insert_resource(Start(Vec2::new(-100.0, 0.0)))
        .insert_resource(Anchors(Vec::new()))
        .insert_resource(End(Vec2::new(100.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bevy-ier curves [Level Editor]".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_startup_system(setup)
        .run()
    ;
}

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}

fn get_file_name(choice: usize) -> String {
    print!("Enter the name of the file: ");
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