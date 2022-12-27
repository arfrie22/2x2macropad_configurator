use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use iced::subscription::{events, events_with};
use iced::widget::{
    button, column, container, pane_grid, pick_list, progress_bar, row, slider, text, text_input,
    Column, Container, Row, Space, Text,
};
use iced::{alignment, executor, window, Color, Font, Padding, Size};
use iced::{Alignment, Application, Command, Element, Length, Settings, Subscription, Theme};
use iced_aw::style::TabBarStyles;
use iced_aw::{color_picker, ColorPicker, TabLabel, Tabs};
use iced_native::widget::space;
use macropad_configurator::hid_manager::Connection;
use macropad_configurator::led_effects::LedRunner;
use macropad_configurator::macro_parser::LedConfig;
use macropad_configurator::{hid_manager, macro_parser, macropad};
use macropad_protocol::data_protocol::LedEffect;
use num_enum::{FromPrimitive, IntoPrimitive};

const ROBOTO: Font = iced::Font::External {
    name: "Roboto",
    bytes: include_bytes!("../../assets/fonts/Roboto-Regular.ttf"),
};

const ROBOTO_BOLD: Font = iced::Font::External {
    name: "Roboto Bold",
    bytes: include_bytes!("../../assets/fonts/Roboto-Bold.ttf"),
};

const ICON_FONT: Font = iced::Font::External {
    name: "Icons",
    bytes: include_bytes!("../../assets/fonts/remixicon.ttf"),
};

const ACTION_DELAY: u64 = 200;

#[derive(Debug, Clone)]
enum Icon {
    Keyboard,
    Light,
    Gear,
    Sun,
    Moon,
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::Keyboard => '\u{ee72}',
            Icon::Light => '\u{eea6}',
            Icon::Gear => '\u{f0e5}',
            Icon::Sun => '\u{f1bc}',
            Icon::Moon => '\u{ef72}',
        }
    }
}

const HEADER_SIZE: u16 = 32;
const TAB_PADDING: u16 = 16;

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
    key_tab: KeyTab,
    led_tab: LedTab,
    settings_tab: SettingsTab,
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
    ButtonHovered(Option<usize>),
    ButtonClicked(bool),
    ReturnToMainPage,
    LedUpdate(Instant),
    UpdateTick(Instant),
    LedEffectChanged(LedEffect),
    LedPeriodChanged(f32),
    LedPeriodChangedText(String),
    LedBrightnessChanged(f32),
    LedBrightnessChangedText(String),
    PickColor,
    CancelColor,
    SubmitColor(Color),
    TabSelected(usize),
    PressTimeChangedText(String),
    HoldTimeChangedText(String),
    DefaultDelayChangedText(String),
    SwitchTheme,
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
                theme: match dark_light::detect() {
                    dark_light::Mode::Dark => Theme::Dark,
                    dark_light::Mode::Light => Theme::Light,
                },
                key_tab: KeyTab::default(),
                led_tab: LedTab::default(),
                settings_tab: SettingsTab::default(),
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
                self.key_tab = KeyTab::new(connection.get_macropad());
                self.led_tab = LedTab::new(connection.get_macropad(), LedRunner::default());
                self.settings_tab = SettingsTab::new(connection.get_macropad(), self.theme.clone());
                self.state = State::Connected(connection, Page::MainPage(0));
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
            Message::ButtonHovered(state) => {
                self.key_tab.selected_key = state;
            }
            Message::ButtonClicked(state) => {
                self.key_tab.clicked = state;
            }
            Message::ReturnToMainPage => {
                self.state = State::Connected(
                    match &mut self.state {
                        State::Connected(connection, _) => connection.clone(),
                        _ => unreachable!(),
                    },
                    Page::MainPage(0),
                );
            }
            Message::LedUpdate(_) => {
                if let State::Connected(con, Page::MainPage(id)) = &mut self.state {
                    if TabId::from(*id) == TabId::ModifyLed {
                        self.led_tab
                            .led_runner
                            .update(&con.get_macropad().lock().unwrap().led_config);
                    }
                }
            }
            Message::UpdateTick(_) => {
                if let State::Connected(con, _) = &mut self.state {
                    self.led_tab.run_actions(con.get_macropad(), con);
                    self.settings_tab.run_actions(con.get_macropad(), con);
                }
            }
            Message::LedEffectChanged(effect) => {
                self.led_tab
                    .queue_action(hid_manager::MacropadCommand::LedEffect(effect));
            }
            Message::LedPeriodChanged(period) => {
                self.led_tab.period_text = (period / 10.0).to_string();
                self.led_tab
                    .queue_action(hid_manager::MacropadCommand::LedEffectPeriod(period / 10.0));
            }
            Message::LedPeriodChangedText(text) => {
                self.led_tab.period_text = text.clone();

                if let Ok(period) = text.parse::<f32>() {
                    if (-5.0..=5.0).contains(&period) {
                        self.led_tab
                            .queue_action(hid_manager::MacropadCommand::LedEffectPeriod(period));
                    }
                }
            }
            Message::LedBrightnessChanged(brightness) => {
                self.led_tab.brightness_text = brightness.to_string();
                self.led_tab
                    .queue_action(hid_manager::MacropadCommand::LedBrightness(
                        brightness as u8,
                    ));
            }
            Message::LedBrightnessChangedText(text) => {
                self.led_tab.brightness_text = text.clone();

                if let Ok(brightness) = text.parse::<u8>() {
                    if (0..=255).contains(&brightness) {
                        self.led_tab
                            .queue_action(hid_manager::MacropadCommand::LedBrightness(brightness));
                    }
                }
            }
            Message::PickColor => {
                self.led_tab.show_picker = true;
            }
            Message::CancelColor => {
                self.led_tab.show_picker = false;
            }
            Message::SubmitColor(color) => {
                let c = color.into_rgba8();
                self.led_tab
                    .queue_action(hid_manager::MacropadCommand::LedBaseColor((
                        c[0], c[1], c[2],
                    )));
                self.led_tab.show_picker = false;
            }
            Message::TabSelected(i) => {
                self.state = State::Connected(
                    match &mut self.state {
                        State::Connected(connection, _) => {
                            match TabId::from(i) {
                                TabId::MainPage => {}
                                TabId::ModifyLed => {
                                    self.led_tab.led_runner.reset();
                                    self.led_tab.update_config(connection.get_macropad());
                                }
                                TabId::ModyifySettings => {
                                    self.settings_tab.update_config(connection.get_macropad());
                                }
                            }
                            connection.clone()
                        }
                        _ => unreachable!(),
                    },
                    Page::MainPage(i),
                );
            }
            Message::PressTimeChangedText(text) => {
                self.settings_tab.press_time_text = text.clone();

                if let Ok(speed) = text.parse::<f32>() {
                    let speed = (speed * 1000.0).trunc() / 1000.0;
                    if speed != 0.0 && (0.0..=u32::MAX as f32).contains(&speed) {
                        if text.contains(".") && text.split('.').last().unwrap().len() > 3 {
                            self.settings_tab.default_delay_text = format!("{:.3}", speed);
                        }
                        self.settings_tab
                            .queue_action(hid_manager::MacropadCommand::TapSpeed(
                                (speed * 1000.0) as u32,
                            ));
                    }
                }
            }
            Message::HoldTimeChangedText(text) => {
                self.settings_tab.hold_time_text = text.clone();

                if let Ok(speed) = text.parse::<f32>() {
                    let speed = (speed * 1000.0).trunc() / 1000.0;
                    if speed != 0.0 && (0.0..=u32::MAX as f32).contains(&speed) {
                        if text.contains(".") && text.split('.').last().unwrap().len() > 3 {
                            self.settings_tab.default_delay_text = format!("{:.3}", speed);
                        }
                        self.settings_tab
                            .queue_action(hid_manager::MacropadCommand::HoldSpeed(
                                (speed * 1000.0) as u32,
                            ));
                    }
                }
            }
            Message::DefaultDelayChangedText(text) => {
                self.settings_tab.default_delay_text = text.clone();

                if let Ok(speed) = text.parse::<f32>() {
                    let speed = (speed * 1000.0).trunc() / 1000.0;
                    if speed != 0.0 && (0.0..=u32::MAX as f32).contains(&speed) {
                        if text.contains(".") && text.split('.').last().unwrap().len() > 3 {
                            self.settings_tab.default_delay_text = format!("{:.3}", speed);
                        }
                        self.settings_tab
                            .queue_action(hid_manager::MacropadCommand::DefaultDelay(
                                (speed * 1000.0) as u32,
                            ));
                    }
                }
            }
            Message::SwitchTheme => {
                self.theme = match self.theme {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                    _ => Theme::Dark,
                };
                self.settings_tab.theme = self.theme.clone();
            }
        };

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            hid_manager::connect().map(Message::HidEvent),
            match &self.state {
                State::Connected(_, Page::MainPage(i)) => {
                    iced::time::every(Duration::from_millis(16)).map(Message::LedUpdate)
                }
                _ => Subscription::none(),
            },
            match &self.state {
                State::Connected(con, _) => {
                    iced::time::every(Duration::from_millis(50)).map(Message::UpdateTick)
                }
                _ => Subscription::none(),
            },
        ])
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
            }
            State::Connected(con, Page::MainPage(i)) => Tabs::new(*i, Message::TabSelected)
                .push(self.key_tab.tab_label(), self.key_tab.view())
                .push(self.led_tab.tab_label(), self.led_tab.view())
                .push(self.settings_tab.tab_label(), self.settings_tab.view())
                .tab_bar_style(TabBarStyles::Purple)
                .icon_font(ICON_FONT)
                .tab_bar_position(iced_aw::TabBarPosition::Bottom)
                .text_font(ROBOTO)
                .text_size(20)
                .into(),
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
                        .on_press(Message::ReturnToMainPage)
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
            }
            State::Connected(_, Page::RecordMacro(_)) => {
                // pane_grid()
                todo!()
            }
        }
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

#[derive(Debug)]
enum Page {
    MainPage(usize),
    // ModifyLeds(LedRunner),
    ModifyKey(usize),
    RecordMacro(usize),
}

#[repr(usize)]
#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, IntoPrimitive, FromPrimitive,
)]
enum TabId {
    #[num_enum(default)]
    MainPage = 0,
    ModifyLed = 1,
    ModyifySettings = 2,
}

#[derive(Debug)]
enum State {
    Disconnected,
    Connected(hid_manager::Connection, Page),
}

trait Tab {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(HEADER_SIZE))
            .push(self.content());

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .padding(TAB_PADDING)
            .into()
    }

    fn content(&self) -> Element<'_, Self::Message>;
}

#[derive(Debug)]
struct KeyTab {
    selected_key: Option<usize>,
    clicked: bool,
    key_configs: Vec<macro_parser::KeyConfig>,
}

impl KeyTab {
    fn new(macropad: Arc<Mutex<macro_parser::Macropad>>) -> Self {
        Self {
            selected_key: None,
            clicked: false,
            key_configs: macropad.lock().unwrap().key_configs.clone(),
        }
    }
}

impl Default for KeyTab {
    fn default() -> Self {
        Self {
            selected_key: None,
            clicked: false,
            key_configs: Vec::new(),
        }
    }
}

impl Tab for KeyTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Keys")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Keyboard.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let message = column![
            text("Select a key to modify")
                .font(ROBOTO)
                .size(60)
                .width(Length::Fill)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            macropad::macropad_button(self.selected_key, self.clicked).on_press(Message::ButtonPressed).on_hover(Message::ButtonHovered).on_click(Message::ButtonClicked),
        ];

        container(message)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(100)
            .into()
    }
}

#[derive(Debug)]
struct LedTab {
    config: macro_parser::LedConfig,
    led_runner: LedRunner,
    show_picker: bool,
    period_text: String,
    brightness_text: String,
    actions: HashMap<
        macropad_protocol::data_protocol::LedCommand,
        (bool, Instant, hid_manager::MacropadCommand),
    >,
}

impl LedTab {
    fn new(macropad: Arc<Mutex<macro_parser::Macropad>>, led_runner: LedRunner) -> Self {
        let config = macropad.lock().unwrap().led_config.clone();

        Self {
            config: config.clone(),
            led_runner,
            show_picker: false,
            period_text: config.effect_period.to_string(),
            brightness_text: config.brightness.to_string(),
            actions: HashMap::new(),
        }
    }

    fn update_config(&mut self, macropad: Arc<Mutex<macro_parser::Macropad>>) {
        self.config = macropad.lock().unwrap().led_config.clone();
    }

    fn run_actions(&mut self, macropad: Arc<Mutex<macro_parser::Macropad>>, con: &mut Connection) {
        for (command, (active, time_to_run, action)) in self.actions.iter_mut() {
            if *active && time_to_run.elapsed() > Duration::ZERO {
                *active = false;

                match action {
                    hid_manager::MacropadCommand::LedBaseColor(_) => {
                        macropad.lock().unwrap().led_config.base_color = self.config.base_color;
                    }
                    hid_manager::MacropadCommand::LedEffect(_) => {
                        macropad.lock().unwrap().led_config.effect = self.config.effect;
                    }
                    hid_manager::MacropadCommand::LedBrightness(_) => {
                        macropad.lock().unwrap().led_config.brightness = self.config.brightness;
                    }
                    hid_manager::MacropadCommand::LedEffectPeriod(_) => {
                        macropad.lock().unwrap().led_config.effect_period =
                            self.config.effect_period;
                    }
                    hid_manager::MacropadCommand::LedEffectOffset(_) => {
                        macropad.lock().unwrap().led_config.effect_offset =
                            self.config.effect_offset;
                    }
                    _ => unreachable!(),
                }

                con.send(hid_manager::Message::Set(action.clone()));
            }
        }
    }

    fn queue_action(&mut self, action: hid_manager::MacropadCommand) {
        match action {
            hid_manager::MacropadCommand::LedBaseColor(color) => {
                self.config.base_color = color;
                self.actions.insert(
                    macropad_protocol::data_protocol::LedCommand::BaseColor,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            hid_manager::MacropadCommand::LedEffect(effect) => {
                self.config.effect = effect;
                self.actions.insert(
                    macropad_protocol::data_protocol::LedCommand::Effect,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            hid_manager::MacropadCommand::LedBrightness(brightness) => {
                self.config.brightness = brightness;
                self.actions.insert(
                    macropad_protocol::data_protocol::LedCommand::Brightness,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            hid_manager::MacropadCommand::LedEffectPeriod(period) => {
                self.config.effect_period = period;
                self.actions.insert(
                    macropad_protocol::data_protocol::LedCommand::EffectPeriod,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            hid_manager::MacropadCommand::LedEffectOffset(offset) => {
                self.config.effect_offset = offset;
                self.actions.insert(
                    macropad_protocol::data_protocol::LedCommand::EffectOffset,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            _ => unreachable!(),
        }
    }
}

impl Default for LedTab {
    fn default() -> Self {
        Self {
            config: LedConfig::default(),
            led_runner: LedRunner::default(),
            show_picker: false,
            period_text: String::from(""),
            brightness_text: String::from(""),
            actions: HashMap::new(),
        }
    }
}

impl Tab for LedTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("LEDs")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Light.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let message = column![row![
            column![
                container(column![
                    text("Effect").font(ROBOTO).size(30),
                    pick_list(
                        &macropad_configurator::macropad_wrapper::EFFECTS[..],
                        Some(self.config.effect),
                        Message::LedEffectChanged
                    ),
                ])
                .padding(Padding {
                    top: 20,
                    right: 0,
                    bottom: 20,
                    left: 0,
                }),
                container(column![
                    text("Period").font(ROBOTO).size(30),
                    row![
                        slider(
                            -50.0..=50.0,
                            self.config.effect_period * 10.0,
                            Message::LedPeriodChanged
                        )
                        .width(Length::Units(200)),
                        Space::with_width(Length::Units(20)),
                        text_input(
                            self.config.effect_period.to_string().as_str(),
                            self.period_text.as_str(),
                            Message::LedPeriodChangedText
                        )
                        .font(ROBOTO)
                        .width(Length::Units(50)),
                    ],
                ])
                .padding(Padding {
                    top: 20,
                    right: 0,
                    bottom: 20,
                    left: 0,
                }),
                container(column![
                    text("Brightness").font(ROBOTO).size(30),
                    row![
                        slider(
                            0.0..=255.0,
                            self.config.brightness as f32,
                            Message::LedBrightnessChanged
                        )
                        .width(Length::Units(200)),
                        Space::with_width(Length::Units(20)),
                        text_input(
                            self.config.brightness.to_string().as_str(),
                            self.brightness_text.as_str(),
                            Message::LedBrightnessChangedText
                        )
                        .font(ROBOTO)
                        .width(Length::Units(50)),
                    ],
                ])
                .padding(Padding {
                    top: 20,
                    right: 0,
                    bottom: 20,
                    left: 0,
                }),
                container(column![
                    text("Base Color").font(ROBOTO).size(30),
                    ColorPicker::new(
                        self.show_picker,
                        Color::from_rgb8(
                            self.config.base_color.0,
                            self.config.base_color.1,
                            self.config.base_color.2
                        ),
                        button("Pick Color").on_press(Message::PickColor),
                        Message::CancelColor,
                        Message::SubmitColor,
                    )
                ])
                .padding(Padding {
                    top: 20,
                    right: 0,
                    bottom: 20,
                    left: 0,
                }),
            ],
            macropad::macropad_led(self.led_runner.get_leds(&self.config)),
        ],];

        container(message)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(100)
            .into()
    }
}

#[derive(Debug)]
struct SettingsTab {
    config: macro_parser::MacroConfig,
    theme: Theme,
    press_time_text: String,
    hold_time_text: String,
    default_delay_text: String,
    actions: HashMap<
        macropad_protocol::data_protocol::ConfigElements,
        (bool, Instant, hid_manager::MacropadCommand),
    >,
}

impl SettingsTab {
    fn new(macropad: Arc<Mutex<macro_parser::Macropad>>, theme: Theme) -> Self {
        let config = macropad.lock().unwrap().config.clone();

        Self {
            config: config.clone(),
            theme,
            press_time_text: (config.tap_speed / 1000).to_string(),
            hold_time_text: (config.hold_speed / 1000).to_string(),
            default_delay_text: (config.default_delay / 1000).to_string(),
            actions: HashMap::new(),
        }
    }

    fn update_config(&mut self, macropad: Arc<Mutex<macro_parser::Macropad>>) {
        self.config = macropad.lock().unwrap().config.clone();
    }

    fn run_actions(&mut self, macropad: Arc<Mutex<macro_parser::Macropad>>, con: &mut Connection) {
        for (command, (active, time_to_run, action)) in self.actions.iter_mut() {
            if *active && time_to_run.elapsed() > Duration::ZERO {
                *active = false;

                match action {
                    hid_manager::MacropadCommand::TapSpeed(_) => {
                        macropad.lock().unwrap().config.tap_speed = self.config.tap_speed;
                    }
                    hid_manager::MacropadCommand::HoldSpeed(_) => {
                        macropad.lock().unwrap().config.hold_speed = self.config.hold_speed;
                    }
                    hid_manager::MacropadCommand::DefaultDelay(_) => {
                        macropad.lock().unwrap().config.default_delay = self.config.default_delay;
                    }
                    _ => unreachable!(),
                }

                con.send(hid_manager::Message::Set(action.clone()));
            }
        }
    }

    fn queue_action(&mut self, action: hid_manager::MacropadCommand) {
        match action {
            hid_manager::MacropadCommand::TapSpeed(speed) => {
                self.config.tap_speed = speed;
                self.actions.insert(
                    macropad_protocol::data_protocol::ConfigElements::TapSpeed,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            hid_manager::MacropadCommand::HoldSpeed(speed) => {
                self.config.hold_speed = speed;
                self.actions.insert(
                    macropad_protocol::data_protocol::ConfigElements::HoldSpeed,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            hid_manager::MacropadCommand::DefaultDelay(delay) => {
                self.config.default_delay = delay;
                self.actions.insert(
                    macropad_protocol::data_protocol::ConfigElements::DefaultDelay,
                    (
                        true,
                        Instant::now() + Duration::from_millis(ACTION_DELAY),
                        action,
                    ),
                );
            }
            _ => unreachable!(),
        }
    }
}

impl Default for SettingsTab {
    fn default() -> Self {
        Self {
            config: macro_parser::MacroConfig::default(),
            theme: match dark_light::detect() {
                dark_light::Mode::Dark => Theme::Dark,
                dark_light::Mode::Light => Theme::Light,
            },
            press_time_text: String::from(""),
            hold_time_text: String::from(""),
            default_delay_text: String::from(""),
            actions: HashMap::new(),
        }
    }
}

impl Tab for SettingsTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Settings")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Gear.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let message = column![
            container(column![
                text("Theme").font(ROBOTO).size(30),
                button(
                    text(char::from(match self.theme {
                        Theme::Light => Icon::Sun,
                        Theme::Dark => Icon::Moon,
                        _ => Icon::Sun,
                    }))
                    .font(ICON_FONT)
                    .size(30)
                )
                .on_press(Message::SwitchTheme)
            ])
            .padding(Padding {
                top: 20,
                right: 0,
                bottom: 20,
                left: 0,
            }),
            container(column![
                text("Press Time (ms)").font(ROBOTO).size(30),
                text_input(
                    (self.config.tap_speed / 1000).to_string().as_str(),
                    self.press_time_text.as_str(),
                    Message::PressTimeChangedText
                )
                .font(ROBOTO)
                .width(Length::Units(50)),
            ])
            .padding(Padding {
                top: 20,
                right: 0,
                bottom: 20,
                left: 0,
            }),
            container(column![
                text("Hold Time (ms)").font(ROBOTO).size(30),
                text_input(
                    (self.config.hold_speed / 1000).to_string().as_str(),
                    self.hold_time_text.as_str(),
                    Message::HoldTimeChangedText
                )
                .font(ROBOTO)
                .width(Length::Units(50)),
            ])
            .padding(Padding {
                top: 20,
                right: 0,
                bottom: 20,
                left: 0,
            }),
            container(column![
                text("Default Delay (ms)").font(ROBOTO).size(30),
                text_input(
                    (self.config.default_delay / 1000).to_string().as_str(),
                    self.default_delay_text.as_str(),
                    Message::DefaultDelayChangedText
                )
                .font(ROBOTO)
                .width(Length::Units(50)),
            ])
            .padding(Padding {
                top: 20,
                right: 0,
                bottom: 20,
                left: 0,
            }),
        ];

        container(message)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(100)
            .into()
    }
}

// struct Pane {
//     id: usize,
//     // pub is_pinned: bool,
// }

// impl Pane {
//     fn new(id: usize) -> Self {
//         Self {
//             id,
//             // is_pinned: false,
//         }
//     }
// }

// fn view_content<'a>(
//     pane: pane_grid::Pane,
//     total_panes: usize,
//     is_pinned: bool,
//     size: Size,
// ) -> Element<'a, Message> {
//     let button = |label, message| {
//         button(
//             text(label)
//                 .width(Length::Fill)
//                 .horizontal_alignment(alignment::Horizontal::Center)
//                 .size(16),
//         )
//         .width(Length::Fill)
//         .padding(8)
//         .on_press(message)
//     };

//     let mut controls = column![
//         button(
//             "Split horizontally",
//             Message::Split(pane_grid::Axis::Horizontal, pane),
//         ),
//         button(
//             "Split vertically",
//             Message::Split(pane_grid::Axis::Vertical, pane),
//         )
//     ]
//     .spacing(5)
//     .max_width(150);

//     if total_panes > 1 && !is_pinned {
//         controls = controls.push(
//             button("Close", Message::Close(pane))
//                 .style(theme::Button::Destructive),
//         );
//     }

//     let content = column![
//         text(format!("{}x{}", size.width, size.height)).size(24),
//         controls,
//     ]
//     .width(Length::Fill)
//     .spacing(10)
//     .align_items(Alignment::Center);

//     container(scrollable(content))
//         .width(Length::Fill)
//         .height(Length::Fill)
//         .padding(5)
//         .center_y()
//         .into()
// }

// fn view_controls<'a>(
//     pane: pane_grid::Pane,
//     total_panes: usize,
//     is_pinned: bool,
//     is_maximized: bool,
// ) -> Element<'a, Message> {
//     let mut row = row![].spacing(5);

//     if total_panes > 1 {
//         let toggle = {
//             let (content, message) = if is_maximized {
//                 ("Restore", Message::Restore)
//             } else {
//                 ("Maximize", Message::Maximize(pane))
//             };
//             button(text(content).size(14))
//                 .style(theme::Button::Secondary)
//                 .padding(3)
//                 .on_press(message)
//         };

//         row = row.push(toggle);
//     }

//     let mut close = button(text("Close").size(14))
//         .style(theme::Button::Destructive)
//         .padding(3);

//     if total_panes > 1 && !is_pinned {
//         close = close.on_press(Message::Close(pane));
//     }

//     row.push(close).into()
// }

// mod style {
//     use iced::widget::container;
//     use iced::Theme;

//     pub fn title_bar_active(theme: &Theme) -> container::Appearance {
//         let palette = theme.extended_palette();

//         container::Appearance {
//             text_color: Some(palette.background.strong.text),
//             background: Some(palette.background.strong.color.into()),
//             ..Default::default()
//         }
//     }

//     pub fn title_bar_focused(theme: &Theme) -> container::Appearance {
//         let palette = theme.extended_palette();

//         container::Appearance {
//             text_color: Some(palette.primary.strong.text),
//             background: Some(palette.primary.strong.color.into()),
//             ..Default::default()
//         }
//     }

//     pub fn pane_active(theme: &Theme) -> container::Appearance {
//         let palette = theme.extended_palette();

//         container::Appearance {
//             background: Some(palette.background.weak.color.into()),
//             border_width: 2.0,
//             border_color: palette.background.strong.color,
//             ..Default::default()
//         }
//     }

//     pub fn pane_focused(theme: &Theme) -> container::Appearance {
//         let palette = theme.extended_palette();

//         container::Appearance {
//             background: Some(palette.background.weak.color.into()),
//             border_width: 2.0,
//             border_color: palette.primary.strong.color,
//             ..Default::default()
//         }
//     }
// }
