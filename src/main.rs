use image::{ImageBuffer, Rgb, RgbImage};
use rand::Rng;
use std::fs::Metadata;
use std::path::Path;
use std::{env, fs, process};

fn main() {
    let all_arguments: Vec<String> = env::args().collect();
    let godot_project;
    if all_arguments.len() >= 2 {
        godot_project = all_arguments[1].clone();
    } else {
        godot_project = match env::current_dir() {
            Ok(t) => t.to_string_lossy().to_string(),
            Err(_) => {
                println!("Cannot get current directory.");
                process::exit(1);
            }
        };
    }
    let godot_project = godot_project.trim_end_matches('/').to_string() + "/";

    if !Path::new(&godot_project).is_dir() {
        println!("{} isn't proper directory.", all_arguments[1]);
        process::exit(1);
    }
    if !Path::new(&(godot_project.to_string() + "/project.godot")).exists() {
        println!("{} isn't proper Godot project.", all_arguments[1]);
        process::exit(1);
    }

    let mut folders_to_check: Vec<String> = Vec::new();
    let mut next_folder: String;
    let mut current_folder: String;

    folders_to_check.push(godot_project);

    while !folders_to_check.is_empty() {
        current_folder = folders_to_check.pop().unwrap();

        let read_dir = match fs::read_dir(&current_folder) {
            Ok(t) => t,
            _ => continue,
        };
        for entry in read_dir {
            let entry_data = match entry {
                Ok(t) => t,
                Err(_) => continue, //Permissions denied
            };
            let metadata: Metadata = match entry_data.metadata() {
                Ok(t) => t,
                Err(_) => continue, //Permissions denied
            };
            if metadata.is_dir() {
                let folder_name: String = match entry_data.file_name().into_string() {
                    Ok(t) => t,
                    Err(_) => continue, // Permission Denied
                };
                if folder_name == ".git" || folder_name == ".import" {
                    continue;
                }
                next_folder = "".to_string() + &*current_folder.to_string() + &*folder_name + "/";
                folders_to_check.push(next_folder);
            } else if metadata.is_file() {
                let file_name: String = match entry_data.file_name().into_string() {
                    Ok(t) => t,
                    Err(_) => continue, // Permission Denied
                };

                let file_name: String = "".to_string() + &*current_folder + &*file_name;

                if !file_name.ends_with(".png") && !file_name.ends_with(".jpg") {
                    continue;
                }
                let mut image_file: RgbImage = ImageBuffer::new(1, 1);
                // let old_img = match image::open(&file_name) {
                //     Ok(t) => t,
                //     Err(_) => {
                //         println!("Failed to open file {}", &file_name);
                //         continue;
                //     }
                // };
                // let image_dimensions = old_img.dimensions();
                // image_file = ImageBuffer::new(image_dimensions.0, image_dimensions.1);

                match fs::remove_file(&file_name) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Failed to remove {}", file_name);
                        continue;
                    }
                }

                let color = Rgb::from([
                    rand::thread_rng().gen_range(0u8, 255u8),
                    rand::thread_rng().gen_range(0u8, 255u8),
                    rand::thread_rng().gen_range(0u8, 255u8),
                ]);

                for pixel in image_file.pixels_mut() {
                    *pixel = color;
                }

                match image_file.save(&file_name) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Failed to save file {}", file_name);
                        continue;
                    }
                };
            }
        }
    }
}
