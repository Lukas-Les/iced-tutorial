use iced::executor;
use iced::{Application, Command, Element, Settings, Theme, Renderer};
use iced::widget::{Button, row};

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello;

impl Application for Hello {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
        (Hello, Command::none())
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let files_button: iced::widget::Button<'_, Message, Theme, Renderer> = Button::new("Select files").on_press(Message::ButtonPressed);
        let dir_button: iced::widget::Button<'_, Message, Theme, Renderer> = Button::new("Select directory").on_press(Message::ButtonPressed);
        let content = row![files_button, dir_button];
        content.into()
    }
}

#[derive(Clone, Debug)]
enum Message {
    ButtonPressed,
}
