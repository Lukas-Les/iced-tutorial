use iced::{Sandbox, Command, Element, Settings, Theme, Renderer};
use iced::widget::{Button, row, Text};


pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello {
    target_files: Vec<Option<String>>,
    result: Option<String>,
}

impl Sandbox for Hello {
    type Message = Message;

    fn new() -> Hello {
        Hello {
            target_files: Vec::new(),
            result: None
        }
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::LEFT_PRESSED => {
                self.result = Some("pressed".to_string());
            }
            Message::RIGHT_PRESSED => {
                self.result = None;
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let left_button: iced::widget::Button<'_, Message, Theme, Renderer> = Button::new("LEFT").on_press(Message::LEFT_PRESSED);
        let right_button: iced::widget::Button<'_, Message, Theme, Renderer> = Button::new("RIGHT").on_press(Message::RIGHT_PRESSED);
        let result_text = self.result.clone().unwrap_or_default();
        let content = row![
            left_button, 
            Text::new(result_text),
            right_button,
        ];
        content.into()
    }
}

#[derive(Clone, Debug)]
enum Message {
    LEFT_PRESSED,
    RIGHT_PRESSED,
}
