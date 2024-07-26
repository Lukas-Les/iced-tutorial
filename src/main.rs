use iced::widget::text_input::{self, Value};
use iced::{alignment, Alignment, Command, Element, Renderer, Sandbox, Settings, Theme};
use iced::widget::{text, text_editor, Button, Column, Container, Row, TextInput};

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello {
    target_paths: Option<Vec<String>>,
    content: text_input::Value,
}

impl Sandbox for Hello {
    type Message = Message;

    fn new() -> Hello {
        Hello {
            target_paths: None,
            content: text_input::Value::new("The results will be desplayed here"),
        }
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SelectFiles => {
                self.target_paths = Some(vec!["a".to_string(), "b".to_string(), "new".to_string()]);
                self.content = text_input::Value::new("files");
            }
            Message::SelectDir => {
                self.target_paths = Some(vec!["d".to_string()]);
                self.content = text_input::Value::new("dir");
            }
            Message::ResetResult => {
                self.target_paths = None;
                self.content = text_input::Value::new("The results will be desplayed here");
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let top_row = Row::new()
            .push(Button::new("Select Files").on_press(Message::SelectFiles))
            .push(Button::new("Select directory").on_press(Message::SelectDir))
            .push(Button::new("Reset").on_press(Message::ResetResult))
            .spacing(3)
            .align_items(alignment::Alignment::Center);

        let top_row_container = Container::new(top_row)
            .padding(3)
            .center_x();

        let result_display = TextInput::new("placeholder", &self.content.to_string());
        let content = Column::new()
            .push(top_row_container)
            .push(result_display);
        content.into()
    }
}

#[derive(Clone, Debug)]
enum Message {
    SelectFiles,
    SelectDir,
    ResetResult,
}
