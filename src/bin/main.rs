use std::time::{Duration, Instant};

use iced::{executor, window, Font};
use iced::widget::{button, column, container, progress_bar, text, Column};
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
    Theme,
};
use macropad_configurator::led_effects::LedRunner;
use macropad_configurator::macro_parser::LedConfig;
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
    ButtonPressed(usize),
    ButtonMainPage,
    ButtonLedPage,
    LedUpdate(Instant),
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
                self.state = State::Connected(connection, Page::ModifyLeds(LedRunner::default()));
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
            Message::ButtonPressed(i) => {
                self.state = State::Connected(
                    match &self.state {
                        State::Connected(connection, _) => connection.clone(),
                        _ => unreachable!(),
                    },
                    Page::ModifyKey(i),
                );
            }
            Message::ButtonMainPage => {
                self.state = State::Connected(
                    match &self.state {
                        State::Connected(connection, _) => connection.clone(),
                        _ => unreachable!(),
                    },
                    Page::MainPage,
                );
            }
            Message::ButtonLedPage => {
                self.state = State::Connected(
                    match &self.state {
                        State::Connected(connection, _) => connection.clone(),
                        _ => unreachable!(),
                    },
                    Page::ModifyLeds(LedRunner::default()),
                );
            }
            Message::LedUpdate(_) => {
                if let State::Connected(con, Page::ModifyLeds(led_runner)) = &mut self.state {
                    led_runner.update(&con.get_macropad().led_config);
                }
            }
        };

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            hid_manager::connect().map(Message::HidEvent),
            iced::time::every(Duration::from_millis(16))
                .map(Message::LedUpdate),
        ])
        // time::every(Duration::from_millis(1000 / self.speed as u64))
        //         .map(Message::Tick)
        // hid_manager::connect().map(Message::HidEvent)
    }

    fn view(&self) -> Element<Message> {
        match &self.state {
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
                    text("Main Page")
                        .font(ROBOTO)
                        .size(60)
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                    text("This is the main page")
                        .font(ROBOTO)
                        .size(30)
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                    
                    // macropad::macropad_led([iced::Color::from_rgb(1.0, 0.0, 0.5); 4])
                    macropad::macropad_button().on_press(Message::ButtonPressed)
                        // .width(Length::Fill)
                        // .height(Length::Fill)
                        // .center_x()
                        // .center_y()
                        // .padding(20)
                ];

                container(message)
                    // .width(Length::Shrink)
                    // .height(Length::Shrink)
                    .center_x()
                    .center_y()
                    .padding(100)
                    .into()
            },
            State::Connected(con, Page::ModifyLeds(runner)) => {
                let message = column![
                    text("Main Page")
                        .font(ROBOTO)
                        .size(60)
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                    text("This is the main page")
                        .font(ROBOTO)
                        .size(30)
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                    
                    macropad::macropad_led(runner.get_leds(&con.get_macropad().led_config))
                ];

                container(message)
                    // .width(Length::Shrink)
                    // .height(Length::Shrink)
                    .center_x()
                    .center_y()
                    .padding(100)
                    .into()
            },
            State::Connected(_, Page::ModifyKey(_)) => {
                let message = column![
                    text("Modify Key")
                        .font(ROBOTO)
                        .size(60)
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                    text("This is the modify key page")
                        .font(ROBOTO)
                        .size(30)
                        .width(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                    button("Back")
                        .on_press(Message::ButtonMainPage)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .padding(20)
                ];

                container(message)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .padding(20)
                    .into()
            },
            State::Connected(_, Page::RecordMacro(_)) => todo!(),
        }
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

#[derive(Debug)]
enum Page {
    MainPage,
    ModifyLeds(LedRunner),
    ModifyKey(usize),
    RecordMacro(usize),
}

#[derive(Debug)]
enum State {
    Disconnected,
    Connected(hid_manager::Connection, Page),
}