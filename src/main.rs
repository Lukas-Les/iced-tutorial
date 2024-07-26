use std::fs::{File, read_dir};
use std::io::{self, Read};
use std::path::PathBuf;

use rfd::FileDialog;

use iced::widget::text_editor::Content;
use iced::widget::{text, text_editor, Button, Column, Container, Row, TextInput};
use iced::{alignment, Alignment, Element, Renderer, Sandbox, Settings, Theme};


pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello {
    content: text_editor::Content,
}

impl Sandbox for Hello {
    type Message = Message;

    fn new() -> Hello {
        Hello {
            content: text_editor::Content::with_text("The results will be desplayed here ^^"),
        }
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SelectFiles => {
                let target_paths = FileDialog::new()
                    .add_filter("text", &["txt", "html", "json", "csv"])
                    .pick_files();
                let res = process_paths_recursive(&target_paths, "bb");
                self.content = Content::with_text(res.concat().as_str());
            }
            Message::SelectDir => {
                let target_paths = FileDialog::new()
                    .pick_folders();
                let res = process_paths_recursive(&target_paths, "bb");
                self.content = Content::with_text(res.concat().as_str());

            }
            Message::ResetResult => {
                self.content = text_editor::Content::with_text("The results will be desplayed here");
            }
            Message::ActionPerformed(action) => {
                self.content.perform(action);
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let top_row = Row::new()
            .push(Button::new("Select Files").on_press(Message::SelectFiles))
            .push(Button::new("Select directory").on_press(Message::SelectDir))
            .push(Button::new("Reset").on_press(Message::ResetResult))
            .spacing(3);

        let top_row_container = Container::new(top_row).padding(3).center_x();
        let result_display = text_editor(&self.content).on_action(Message::ActionPerformed);
        let content = Column::new().push(top_row_container).push(result_display);
        content.into()
    }
}

#[derive(Clone, Debug)]
enum Message {
    SelectFiles,
    SelectDir,
    ResetResult,
    ActionPerformed(text_editor::Action),
}

fn option_vec_pathbuf_to_content(option_vec: &Option<Vec<PathBuf>>) -> text_editor::Content {
    match option_vec {
        Some(vec) => {
            let joined_paths: String = vec.iter()
                .map(|pathbuf| pathbuf.to_str().unwrap_or("Invalid UTF-8"))
                .collect::<Vec<&str>>()
                .join("\n");
            Content::with_text(&joined_paths)
        },
        None => Content::new()
    }
}


fn process_paths_recursive(paths: &Option<Vec<PathBuf>>, re_pattern: &str) -> Vec<String> {
    let mut results = Vec::new();

    if let Some(paths) = paths {
        for path in paths {
            let path = path.as_path();
            if path.is_dir() {
                println!("Processing directory: {}", path.display());
                if let Ok(entries) = read_dir(path) {
                    let mut sub_paths = Vec::new();
                    for entry in entries {
                        if let Ok(entry) = entry {
                            sub_paths.push(entry.path());
                        }
                    }
                    results.extend(process_paths_recursive(&Some(sub_paths), re_pattern));
                }
            } else {
                println!("Processing file: {}", path.display());
                if let Ok(mut file) = File::open(path) {
                    let mut buffer = String::new();
                    if file.read_to_string(&mut buffer).is_ok() {
                        // Perform regex matching here using `re_pattern` if necessary
                        let result = "bybiai ".to_string();
                        results.push(result);
                    } else {
                        println!("Failed to read file: {}", path.display());
                    }
                } else {
                    println!("Failed to open file: {}", path.display());
                }
            }
        }
    }
    results
}