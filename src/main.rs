use std::process::Command;
use std::env;
use std::fs;
use std::io::{self, Write};
use glob::glob;
use std::path::Path;

fn main() {
    let _current_working_dir = env::current_dir().expect("Failed to get current working directory");

    let mut files: Vec<String> = Vec::new();
    for entry in glob("*.mkv").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => files.push(path.display().to_string()),
            Err(e) => println!("{:?}", e),
        }
    }

    if files.is_empty() {
        println!("No valid .mkv files found in the current directory.");
        return;
    }

    print!("Do you wish original .mkv files to be deleted when finished? [y/N]: ");
    io::stdout().flush().unwrap();
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read line");
    let answer = answer.trim().to_lowercase();

    let delete_sources = match answer.as_str() {
        "n" | "" => false,
        "y" => true,
        _ => {
            println!("Aborting. Enter a valid answer [y/n]");
            return;
        }
    };

    println!("\n\nStart converting ...................");

    for file in &files {
        let file_path = Path::new(&file);
        let output_file = file_path.with_extension("mp4");

        match Command::new("ffmpeg")
            .arg("-i")
            .arg(file_path)
            .arg("-codec")
            .arg("copy")
            .arg(&output_file)
            .status()
        {
            Ok(status) if status.success() => {
                println!("\n\nConverting Done!\n");
                if delete_sources {
                    match fs::remove_file(file_path) {
                        Ok(_) => println!("\n\nDeleted\n"),
                        Err(e) => println!("Failed to delete file: {}", e),
                    }
                }
            }
            _ => {
                println!("ffmpeg not found, attempting to install...");
                match Command::new("sudo")
                    .arg("apt")
                    .arg("install")
                    .arg("ffmpeg")
                    .status()
                {
                    Ok(status) if status.success() => {
                        println!("ffmpeg installed, retrying conversion...");
                        match Command::new("ffmpeg")
                            .arg("-i")
                            .arg(file_path)
                            .arg("-codec")
                            .arg("copy")
                            .arg(&output_file)
                            .status()
                        {
                            Ok(status) if status.success() => {
                                println!("\n\nConverting Done!\n");
                                if delete_sources {
                                    match fs::remove_file(file_path) {
                                        Ok(_) => println!("\n\nDeleted\n"),
                                        Err(e) => println!("Failed to delete file: {}", e),
                                    }
                                }
                            }
                            _ => println!("\n\nUnable to convert! Try installing ffmpeg if you don't have it already\n\n"),
                        }
                    }
                    _ => println!("\n\nUnable to install ffmpeg\n\n"),
                }
            }
        }
    }

    println!("\n\nThank you :D ...................");
}
