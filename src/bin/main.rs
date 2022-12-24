use iced::{executor, window, Font};
use iced::widget::{button, column, container, progress_bar, text, Column};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
    Theme,
};
use macropad_configurator::{hid_manager, macro_parser, macropad};

const ROBOTO: Font = iced::Font::External {
    name: "Roboto",
    bytes: include_bytes!("../../assets/fonts/Roboto-Regular.ttf"),
};

const ROBOTO_BOLD: Font = iced::Font::External {
    name: "Roboto Bold",
    bytes: include_bytes!("../../assets/fonts/Roboto-Bold.ttf"),
};

pub fn main() -> iced::Result {
    Configurator::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug)]
struct Configurator {
    state: State,
    theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    HidMessage(hid_manager::Message),
    HidEvent(hid_manager::Event),
    MacroLoaded(macro_parser::Macro),
    MacroSaved,
    CommandSent(macropad_protocol::data_protocol::DataCommand, [u8; 64]),
    CommandReceived(macropad_protocol::data_protocol::DataCommand, [u8; 64]),
    CommandErrored,
}

impl Application for Configurator {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Configurator, Command<Message>) {
        (
            Configurator {
                state: State::Disconnected,
                theme: {
                    match dark_light::detect() {
                        dark_light::Mode::Dark => Theme::Dark,
                        dark_light::Mode::Light => Theme::Light,
                    }
                },
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Macropad Configurator")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::HidMessage(_) => {}
            Message::HidEvent(hid_manager::Event::Connected(connection)) => {
                self.state = State::Connected(connection, Page::MainPage);
            }
            Message::HidEvent(hid_manager::Event::Disconnected) => {
                self.state = State::Disconnected;
            }
            Message::HidEvent(_) => {}
            Message::MacroLoaded(_) => {}
            Message::MacroSaved => {}
            Message::CommandSent(_, _) => {}
            Message::CommandReceived(_, _) => {}
            Message::CommandErrored => {}
        };

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        hid_manager::connect().map(Message::HidEvent)
    }

    fn view(&self) -> Element<Message> {
        match self.state {
            State::Disconnected => {
                let message = column![
                    text("Disconnected")
                        .font(ROBOTO)
                        .size(60)
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                    text("Connect your macropad to get started")
                        .font(ROBOTO)
                        .size(30)
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                ];

                container(message)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .padding(20)
                    .into()
            },
            State::Connected(_, Page::MainPage) => {
                let message = column![
                    // text("Main Page")
                    //     .font(ROBOTO)
                    //     .size(60)
                    //     .width(Length::Fill)
                    //     .horizontal_alignment(iced::alignment::Horizontal::Center),
                    // text("This is the main page")
                    //     .font(ROBOTO)
                    //     .size(30)
                    //     .width(Length::Fill)
                    //     .horizontal_alignment(iced::alignment::Horizontal::Center),
                    
                    macropad::macropad_led([iced::Color::from_rgb(1.0, 0.0, 0.5); 4])
                        // .width(Length::Fill)
                        // .height(Length::Fill)
                        // .center_x()
                        // .center_y()
                        // .padding(20)
                ];

                container(message)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .padding(20)
                    .into()
            },
            State::Connected(_, Page::ModifyKey(_)) => todo!(),
            State::Connected(_, Page::RecordMacro(_)) => todo!(),
            State::Connected(_, Page::ModifyLeds) => todo!(),
        }
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

#[derive(Debug)]
enum Page {
    MainPage,
    ModifyKey(usize),
    RecordMacro(usize),
    ModifyLeds,
}

#[derive(Debug)]
enum State {
    Disconnected,
    Connected(hid_manager::Connection, Page),
}