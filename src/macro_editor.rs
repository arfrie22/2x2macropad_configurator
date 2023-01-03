use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell, RefMut};
use std::ops::Sub;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{self, Canvas, Cursor, Frame, Geometry, Path, Stroke};
use iced::{mouse, Color, Size, Vector};
use iced::{Element, Length, Point, Rectangle, Theme};

use crate::font::{Icon, ICON_FONT, ROBOTO, ROBOTO_BOLD};
use crate::macro_parser::{self, Macro, MacroFrame};
use crate::type_wrapper::{ConsumerWrapper, KeyboardWrapper};

const CLOSE_BUTTON_PADDING: f32 = 1.0;
const CLOSE_BUTTON_OFFSET: Vector = Vector::new(
    ACTION_SIZE.width - ACTION_SIZE.height - CLOSE_BUTTON_PADDING,
    CLOSE_BUTTON_PADDING,
);
const CLOSE_BUTTON_SIZE: Size = Size::new(
    ACTION_SIZE.height - (2.0 * CLOSE_BUTTON_PADDING),
    ACTION_SIZE.height - (2.0 * CLOSE_BUTTON_PADDING),
);

const DELAY_OFFSET: Vector = Vector::new(
    ACTION_SIZE.width - 10.0 - CLOSE_BUTTON_SIZE.width,
    ACTION_SIZE.height - 5.0,
);
const DELAY_SIZE: f32 = 15.0;

const TITLE_OFFSET: Vector = Vector::new(10.0, ACTION_SIZE.height / 2.0);
const TITLE_SIZE: f32 = 20.0;

const ARGUMENT_SIZE: f32 = 15.0;
const ARGUMENT_TEXT_SIZE: f32 = 10.0;

const MOVE_THRESHOLD: f32 = 5.0;

#[derive(Debug, Clone)]
pub struct Index {
    pub index: usize,
    pub parents: Vec<usize>,
}

impl Index {
    pub fn get_level(&self) -> usize {
        self.parents.len()
    }

    fn get_action_recurse(index: Index, root: Action) -> Action {
        if let ActionWrapper::Loop(actions, _) = root.get_action() {
            if !index.parents.is_empty() {
                let mut parents = index.parents.clone();
                let parent_index = parents.remove(0);
                let index = Index {
                    index: index.index,
                    parents,
                };
                Index::get_action_recurse(index, actions[parent_index].clone())
            } else {
                actions[index.index].clone()
            }
        } else {
            unreachable!()
        }
    }

    pub fn get_action(&self, actions: &[Action]) -> Action {
        if !self.parents.is_empty() {
            let mut parents = self.parents.clone();
            let parent_index = parents.remove(0);
            Index::get_action_recurse(
                Index {
                    index: self.index,
                    parents,
                },
                actions[parent_index].clone(),
            )
        } else {
            actions[self.index].clone()
        }
    }

    // pub fn get_top_left(&self, scroll_offset: Vector) -> Point {
    //     match self {
    //         Index::ImaginaryIndex { action_index, parent_index } => {
    //             let x = parent_index.try_lock().unwrap().get_level() as f32 * LOOP_PADDING;
    //             let y = *action_index as f32 * (ACTION_SIZE.height + ACTION_PADDING);

    //             Point::new(0.0, y) - scroll_offset
    //         },
    //         Index::Index { action_index, .. } => {
    //             let y = *action_index as f32 * (ACTION_SIZE.height + ACTION_PADDING);

    //             Point::new(0.0, y) - scroll_offset
    //         },
    //         Index::Nested { action_index, .. } => {
    //             let x = self.get_level() as f32 * LOOP_PADDING;
    //             let y = *action_index as f32 * (ACTION_SIZE.height + ACTION_PADDING);

    //             Point::new(x, y) - scroll_offset
    //         },
    //     }
    // }

    // pub fn get_offset(&self, scroll_offset: Vector, point: Point) -> Option<Vector> {
    //     let top_left = self.get_top_left(scroll_offset);
    //     let bounds = Rectangle::new(top_left, ACTION_SIZE);

    //     if bounds.contains(point) {
    //         Some(point - top_left)
    //     } else {
    //         None
    //     }
    // }

    // pub fn on_close_button(offset: Vector) -> bool {
    //     let close_button_bounds = Rectangle::new(
    //         Point::ORIGIN + Self::CLOSE_BUTTON_OFFSET,
    //         Self::CLOSE_BUTTON_SIZE
    //     );

    //     close_button_bounds.contains(Point::ORIGIN + offset)
    // }

    fn avoid_collisions(&self, new_index: &mut Index) {
        if !new_index.parents.is_empty() {
            if !self.parents.is_empty() {
                for (i, parent) in new_index.parents.iter_mut().enumerate() {
                    if self.parents.len() > i {
                        if self.parents[i] < *parent {
                            *parent -= 1;
                            break;
                        }
                    } else {
                        if self.index < *parent {
                            *parent -= 1;
                        }

                        break;
                    }
                }
            } else if self.index < *new_index.parents.last().unwrap() {
                *new_index.parents.last_mut().unwrap() -= 1;
            }
        }
    }

    fn add_to_macro_recurse(index: Index, action: Action, root: Action) {
        if let ActionWrapper::Loop(mut actions, count) = root.get_action() {
            if !index.parents.is_empty() {
                let mut parents = index.parents.clone();
                let mut parent_index = parents.remove(0);
                let index = Index {
                    index: index.index,
                    parents,
                };
                Index::add_to_macro_recurse(index, action, actions[parent_index].clone());
            } else {
                let mut actions = actions.clone();
                actions.insert(index.index, action);
                root.set_action(ActionWrapper::Loop(actions, count));
            }
        } else {
            unreachable!()
        }
    }

    pub fn add_to_macro(&self, action: Action, actions: &mut Vec<Action>) {
        if !self.parents.is_empty() {
            let mut parents = self.parents.clone();
            let parent_index = parents.remove(0);
            Index::add_to_macro_recurse(
                Index {
                    index: self.index,
                    parents,
                },
                action,
                actions[parent_index].clone(),
            );
        } else {
            actions.insert(self.index, action);
        }
    }

    fn remove_from_macro_recurse(index: Index, root: Action) -> Action {
        if let ActionWrapper::Loop(mut actions, count) = root.get_action() {
            if !index.parents.is_empty() {
                let mut parents = index.parents.clone();
                let mut parent_index = parents.remove(0);
                let index = Index {
                    index: index.index,
                    parents,
                };
                Index::remove_from_macro_recurse(index, actions[parent_index].clone())
            } else {
                let mut actions = actions.clone();
                let action = actions.remove(index.index);
                root.set_action(ActionWrapper::Loop(actions, count));
                action
            }
        } else {
            unreachable!()
        }
    }

    pub fn remove_from_macro(&self, actions: &mut Vec<Action>) -> Action {
        if !self.parents.is_empty() {
            let mut parents = self.parents.clone();
            let parent_index = parents.remove(0);
            Index::remove_from_macro_recurse(
                Index {
                    index: self.index,
                    parents,
                },
                actions[parent_index].clone(),
            )
        } else {
            actions.remove(self.index)
        }
    }

    pub fn move_in_macro(&self, new_index: Index, actions: &mut Vec<Action>) {
        let frame = self.remove_from_macro(actions);
        new_index.add_to_macro(frame, actions);
    }
}

impl PartialEq for Index {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.parents == other.parents
    }
}

#[derive(Debug, Clone)]
pub enum ActionOptions {
    Empty,
    SetLed((u8, u8, u8)),
    ClearLed,
    KeyDown(usbd_human_interface_device::page::Keyboard),
    KeyUp(usbd_human_interface_device::page::Keyboard),
    KeyPress(
        usbd_human_interface_device::page::Keyboard,
        Option<Duration>,
    ),
    ConsumerPress(
        usbd_human_interface_device::page::Consumer,
        Option<Duration>,
    ),
    String(String, Option<Duration>),
    Chord(
        Vec<usbd_human_interface_device::page::Keyboard>,
        Option<Duration>,
    ),
    Loop(u8),
}

#[derive(Debug, Clone)]
pub struct SelectedAction {
    pub index: Index,
    pub action_options: ActionOptions,
    pub delay: Option<Duration>,
}

impl SelectedAction {
    pub fn new(action: &Action, index: Index) -> Self {
        let action_options = match action.get_action() {
            ActionWrapper::SetLed(color) => ActionOptions::SetLed(color),
            ActionWrapper::ClearLed => ActionOptions::ClearLed,
            ActionWrapper::KeyDown(key) => ActionOptions::KeyDown(key),
            ActionWrapper::KeyUp(key) => ActionOptions::KeyUp(key),
            ActionWrapper::KeyPress(key, delay) => ActionOptions::KeyPress(key, delay),
            ActionWrapper::ConsumerPress(key, delay) => ActionOptions::ConsumerPress(key, delay),
            ActionWrapper::String(string, delay) => ActionOptions::String(string, delay),
            ActionWrapper::Chord(keys, delay) => ActionOptions::Chord(keys, delay),
            ActionWrapper::Loop(_, count) => ActionOptions::Loop(count),
            ActionWrapper::Empty => ActionOptions::Empty,
        };

        Self {
            index,
            action_options,
            delay: action.get_delay(),
        }
    }

    pub fn from_action(action: &Action, actions: &[Action]) -> Self {
        Self::new(action, action.index_from(actions).unwrap())
    }

    pub fn from_index(index: &Index, actions: &[Action]) -> Self {
        Self::new(&index.get_action(actions), index.clone())
    }

    pub fn update_action(&self, actions: &[Action]) {
        let action = self.index.get_action(actions);

        match &self.action_options {
            ActionOptions::Empty => {
                if let ActionWrapper::Empty = action.get_action() {
                    action.set_action(ActionWrapper::Empty);
                } else {
                    unreachable!()
                }
            }
            ActionOptions::SetLed(color) => {
                if let ActionWrapper::SetLed(_) = action.get_action() {
                    action.set_action(ActionWrapper::SetLed(*color));
                } else {
                    unreachable!()
                }
            }
            ActionOptions::ClearLed => {
                if let ActionWrapper::ClearLed = action.get_action() {
                    action.set_action(ActionWrapper::ClearLed);
                } else {
                    unreachable!()
                }
            }
            ActionOptions::KeyDown(key) => {
                if let ActionWrapper::KeyDown(_) = action.get_action() {
                    action.set_action(ActionWrapper::KeyDown(*key));
                } else {
                    unreachable!()
                }
            }
            ActionOptions::KeyUp(key) => {
                if let ActionWrapper::KeyUp(_) = action.get_action() {
                    action.set_action(ActionWrapper::KeyUp(*key));
                } else {
                    unreachable!()
                }
            }
            ActionOptions::KeyPress(key, delay) => {
                if let ActionWrapper::KeyPress(_, _) = action.get_action() {
                    action.set_action(ActionWrapper::KeyPress(*key, *delay));
                } else {
                    unreachable!()
                }
            }
            ActionOptions::ConsumerPress(key, delay) => {
                if let ActionWrapper::ConsumerPress(_, _) = action.get_action() {
                    action.set_action(ActionWrapper::ConsumerPress(*key, *delay));
                } else {
                    unreachable!()
                }
            }
            ActionOptions::String(string, delay) => {
                if let ActionWrapper::String(_, _) = action.get_action() {
                    action.set_action(ActionWrapper::String(string.clone(), *delay));
                } else {
                    unreachable!()
                }
            }
            ActionOptions::Chord(keys, delay) => {
                if let ActionWrapper::Chord(_, _) = action.get_action() {
                    action.set_action(ActionWrapper::Chord(keys.clone(), *delay));
                } else {
                    unreachable!()
                }
            }
            ActionOptions::Loop(count) => {
                if let ActionWrapper::Loop(actions, _) = action.get_action() {
                    action.set_action(ActionWrapper::Loop(actions.clone(), *count));
                } else {
                    unreachable!()
                }
            }
        }

        action.set_delay(self.delay);
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    MoveFrame(Index, Index),
    RemoveFrame(Index),
    AddFrame(MacroFrame, Index),
    SelectFrame(Option<SelectedAction>),
    ReleaseGrab,
    DragStart,
    Scroll(Vector),
}

#[derive(Default, Debug)]
pub struct State {
    cache: canvas::Cache,
    scroll_offset: Vector,
}

const ACTION_SIZE: Size = Size::new(600.0, 50.0);
const ACTION_PADDING: f32 = 10.0;
const LOOP_PADDING: f32 = 50.0;

impl State {
    pub fn view<'a>(&'a self, actions: &'a [Action]) -> Element<'a, Message> {
        Canvas::new(Editor {
            state: self,
            actions,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear()
    }

    pub fn scroll_to(&mut self, offset: Vector) {
        self.scroll_offset = offset;
    }

    pub fn reset_scroll(&mut self) {
        self.scroll_offset = Vector::new(0.0, 0.0);
    }

    pub fn scroll_to_bottom(&mut self, actions: &[Action], bounds: &Rectangle) {
        let max_y = (actions.len() as f32 * (ACTION_SIZE.height + ACTION_PADDING)) - ACTION_PADDING;
        self.scroll_offset = Vector::new(0.0, max_y.sub(bounds.height).max(0.0));
    }
}

#[derive(Debug, Clone)]
struct Drag {
    action: Action,
    drag_offset: Vector,
    moved: bool,
    to: Point,
    moving: Option<Index>,
}

impl Drag {
    fn draw(&self, theme: &Theme, bounds: Rectangle, cursor: Cursor) -> Geometry {
        let mut frame = Frame::new(bounds.size());

        if let Some(cursor_position) = cursor.position_in(&bounds) {
            self.action.draw(
                &mut frame,
                theme,
                cursor_position - self.drag_offset,
                &None,
                &None,
            );
        }

        frame.into_geometry()
    }
}

struct Editor<'a> {
    state: &'a State,
    actions: &'a [Action],
}

impl<'a> Editor<'a> {
    fn get_max_x_offset(&self, bounds: &Rectangle) -> f32 {
        let max_x =
            ACTION_SIZE.width + (LOOP_PADDING * (Action::max_width(self.actions) - 1) as f32);

        max_x.sub(bounds.width).max(0.0)
    }

    fn get_max_y_offset(&self, bounds: &Rectangle) -> f32 {
        let max_y = (Action::max_length(self.actions) as f32
            * (ACTION_SIZE.height + ACTION_PADDING))
            - ACTION_PADDING;
        max_y.sub(bounds.height).max(0.0)
    }
}

impl<'a> canvas::Program<Message> for Editor<'a> {
    type State = (Option<Drag>, Option<Action>);

    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        let cursor_position = if let Some(position) = cursor.position_in(&bounds) {
            position
        } else {
            return (event::Status::Ignored, None);
        };

        match event {
            Event::Mouse(mouse_event) => {
                let message = match mouse_event {
                    mouse::Event::WheelScrolled { delta } => {
                        let scroll_offset = match delta {
                            mouse::ScrollDelta::Lines { x, y } => {
                                self.state.scroll_offset + Vector::new(x * 20.0, -y * 20.0)
                            }
                            mouse::ScrollDelta::Pixels { x, y } => {
                                self.state.scroll_offset + Vector::new(x, -y)
                            }
                        };

                        let scroll_offset = Vector::new(
                            scroll_offset.x.max(0.0).min(self.get_max_x_offset(&bounds)),
                            scroll_offset.y.max(0.0).min(self.get_max_y_offset(&bounds)),
                        );
                        Some(Message::Scroll(scroll_offset))
                    }
                    mouse::Event::ButtonPressed(mouse::Button::Left) => match *state {
                        (None, _) => {
                            if let Some((action, offset)) = Action::get_offset(
                                self.actions,
                                self.state.scroll_offset,
                                cursor_position,
                            ) {
                                if action.on_close_button(offset) {
                                    *state = (None, None);
                                    return (
                                        event::Status::Captured,
                                        Some(Message::RemoveFrame(
                                            action.index_from(self.actions).unwrap(),
                                        )),
                                    );
                                }

                                *state = (
                                    Some(Drag {
                                        action,
                                        drag_offset: offset,
                                        moved: false,
                                        to: cursor_position,
                                        moving: None,
                                    }),
                                    None,
                                );

                                return (event::Status::Captured, None);
                            }

                            if state.1.take().is_some() {
                                return (event::Status::Captured, Some(Message::SelectFrame(None)));
                            }
                            None
                        }
                        _ => None,
                    },
                    mouse::Event::CursorMoved { .. } => {
                        if let Some(cursor_position) = cursor.position_in(&bounds) {
                            match state.0.take() {
                                Some(Drag {
                                    action,
                                    drag_offset,
                                    moved,
                                    mut to,
                                    mut moving,
                                    ..
                                }) => {
                                    if !moved {
                                        if cursor_position.distance(to) > MOVE_THRESHOLD {
                                            *state = (
                                                Some(Drag {
                                                    action,
                                                    drag_offset,
                                                    moved: true,
                                                    to: cursor_position,
                                                    moving: None,
                                                }),
                                                None,
                                            );
                                            return (
                                                event::Status::Captured,
                                                Some(Message::DragStart),
                                            );
                                        }
                                    } else {
                                        to = cursor_position;
                                        if let Some(index) = moving.take() {
                                            if action.index_from(self.actions).unwrap() != index {
                                                moving = Some(index);
                                            }
                                        } else {
                                            if let Some(mut index) = Action::get_drag_index(
                                                &action,
                                                self.actions,
                                                self.actions,
                                                self.state.scroll_offset,
                                                cursor_position,
                                            ) {
                                                let from_index =
                                                    action.index_from(self.actions).unwrap();
                                                from_index.avoid_collisions(&mut index);

                                                let message =
                                                    Message::MoveFrame(from_index, index.clone());
                                                *state = (
                                                    Some(Drag {
                                                        action,
                                                        drag_offset,
                                                        moved,
                                                        to: cursor_position,
                                                        moving: Some(index),
                                                    }),
                                                    None,
                                                );
                                                return (event::Status::Captured, Some(message));
                                            }
                                        }
                                    }

                                    *state = (
                                        Some(Drag {
                                            action,
                                            drag_offset,
                                            moved,
                                            to,
                                            moving,
                                        }),
                                        None,
                                    );

                                    None
                                }
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => match state.0.take() {
                        Some(Drag { action, moved, .. }) => {
                            if !moved {
                                let selected =
                                    Some(SelectedAction::from_action(&action, self.actions));
                                *state = (None, Some(action));
                                return (
                                    event::Status::Captured,
                                    Some(Message::SelectFrame(selected)),
                                );
                            }

                            return (event::Status::Captured, Some(Message::ReleaseGrab));
                        }
                        None => None,
                    },
                    _ => None,
                };

                (event::Status::Captured, message)
            }
            _ => (event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        theme: &Theme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry> {
        let content = self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
            Action::draw_all(
                self.actions,
                frame,
                theme,
                self.state.scroll_offset,
                &state.1,
                &state.0,
            );

            frame.stroke(
                &Path::rectangle(Point::ORIGIN, frame.size()),
                Stroke::default().with_width(2.0),
            );
        });

        if let Some(drag) = state.0.as_ref() {
            let drag_action = drag.draw(&theme, bounds, cursor);
            let placeholder = {
                let mut frame = Frame::new(bounds.size());

                let index = drag.action.index_from(self.actions).unwrap();

                let position = Point::new(
                    LOOP_PADDING * index.parents.len() as f32,
                    drag.action
                        .get_count_to(self.actions, Vec::new(), 0)
                        .unwrap() as f32
                        * (ACTION_SIZE.height + ACTION_PADDING),
                ) - self.state.scroll_offset;

                frame.stroke(
                    &Path::rectangle(
                        position,
                        Size::new(
                            ACTION_SIZE.width
                                + (LOOP_PADDING * (drag.action.calculate_width() - 1) as f32),
                            ((ACTION_SIZE.height + ACTION_PADDING)
                                * drag.action.calculate_length() as f32)
                                - ACTION_PADDING,
                        ),
                    ),
                    Stroke::default()
                        .with_color(Color::from_rgb8(0x00, 0xFF, 0xFF))
                        .with_width(3.0),
                );

                frame.into_geometry()
            };
            vec![content, drag_action, placeholder]
        } else {
            vec![content]
        }
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> mouse::Interaction {
        match &state.0 {
            Some(_) => mouse::Interaction::Grabbing,
            None => {
                if let Some(cursor_position) = cursor.position_in(&bounds) {
                    if let Some((action, offset)) =
                        Action::get_offset(self.actions, self.state.scroll_offset, cursor_position)
                    {
                        if action.on_close_button(offset) {
                            mouse::Interaction::Pointer
                        } else {
                            mouse::Interaction::Grab
                        }
                    } else {
                        mouse::Interaction::default()
                    }
                } else {
                    mouse::Interaction::default()
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ActionWrapper {
    Empty,
    SetLed((u8, u8, u8)),
    ClearLed,
    KeyDown(usbd_human_interface_device::page::Keyboard),
    KeyUp(usbd_human_interface_device::page::Keyboard),
    KeyPress(
        usbd_human_interface_device::page::Keyboard,
        Option<Duration>,
    ),
    ConsumerPress(
        usbd_human_interface_device::page::Consumer,
        Option<Duration>,
    ),
    String(String, Option<Duration>),
    Chord(
        Vec<usbd_human_interface_device::page::Keyboard>,
        Option<Duration>,
    ),
    Loop(Vec<Action>, u8),
}

impl ActionWrapper {
    fn calculate_length(&self) -> usize {
        match self {
            ActionWrapper::Loop(actions, _) => {
                let mut length = 2;
                for action in actions {
                    length += action.calculate_length();
                }
                length
            }
            _ => 1,
        }
    }

    fn calculate_width(&self) -> usize {
        match self {
            ActionWrapper::Loop(actions, _) => {
                if actions.is_empty() {
                    1
                } else {
                    let mut width = 1;
                    for action in actions {
                        width = width.max(action.calculate_width());
                    }

                    width + 1
                }
            }
            _ => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MacroAction {
    action: ActionWrapper,
    delay: Option<Duration>,
}

impl MacroAction {
    fn calculate_length(&self) -> usize {
        self.action.calculate_length()
    }

    fn calculate_width(&self) -> usize {
        self.action.calculate_width()
    }
}

impl From<MacroFrame> for MacroAction {
    fn from(frame: MacroFrame) -> Self {
        let action = match frame.action {
            macro_parser::ActionType::Empty => ActionWrapper::Empty,
            macro_parser::ActionType::SetLed(color) => ActionWrapper::SetLed(color),
            macro_parser::ActionType::ClearLed => ActionWrapper::ClearLed,
            macro_parser::ActionType::KeyDown(key) => ActionWrapper::KeyDown(key),
            macro_parser::ActionType::KeyUp(key) => ActionWrapper::KeyUp(key),
            macro_parser::ActionType::KeyPress(key, delay) => ActionWrapper::KeyPress(key, delay),
            macro_parser::ActionType::ConsumerPress(key, delay) => {
                ActionWrapper::ConsumerPress(key, delay)
            }
            macro_parser::ActionType::String(string, delay) => ActionWrapper::String(string, delay),
            macro_parser::ActionType::Chord(keys, delay) => ActionWrapper::Chord(keys, delay),
            macro_parser::ActionType::Loop(frames, loop_count) => {
                let mut actions = Vec::new();
                for frame in frames {
                    actions.push(Action::from(frame));
                }
                ActionWrapper::Loop(actions, loop_count)
            }
        };
        let length = action.calculate_length();
        MacroAction {
            action,
            delay: frame.delay,
        }
    }
}

impl From<MacroAction> for MacroFrame {
    fn from(action: MacroAction) -> Self {
        MacroFrame {
            action: match action.action {
                ActionWrapper::Empty => macro_parser::ActionType::Empty,
                ActionWrapper::SetLed(color) => macro_parser::ActionType::SetLed(color),
                ActionWrapper::ClearLed => macro_parser::ActionType::ClearLed,
                ActionWrapper::KeyDown(key) => macro_parser::ActionType::KeyDown(key),
                ActionWrapper::KeyUp(key) => macro_parser::ActionType::KeyUp(key),
                ActionWrapper::KeyPress(key, delay) => {
                    macro_parser::ActionType::KeyPress(key, delay)
                }
                ActionWrapper::ConsumerPress(key, delay) => {
                    macro_parser::ActionType::ConsumerPress(key, delay)
                }
                ActionWrapper::String(string, delay) => {
                    macro_parser::ActionType::String(string, delay)
                }
                ActionWrapper::Chord(keys, delay) => macro_parser::ActionType::Chord(keys, delay),
                ActionWrapper::Loop(actions, loop_count) => {
                    let mut frames = Vec::new();
                    for action in actions {
                        frames.push(action.into());
                    }
                    macro_parser::ActionType::Loop(frames, loop_count)
                }
            },
            delay: action.delay,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Action(Rc<RefCell<MacroAction>>);
impl Action {
    pub fn new(action: MacroAction) -> Self {
        Action(Rc::new(RefCell::new(action)))
    }

    fn calculate_length(&self) -> usize {
        self.0.try_borrow().unwrap().calculate_length()
    }

    fn calculate_width(&self) -> usize {
        self.0.try_borrow().unwrap().calculate_width()
    }

    fn max_length(actions: &[Action]) -> usize {
        let mut length = 0;
        for action in actions {
            length += action.calculate_length();
        }
        length
    }

    fn max_width(actions: &[Action]) -> usize {
        let mut width = 0;
        for action in actions {
            width = width.max(action.calculate_width());
        }
        width
    }

    pub fn get_offset(
        actions: &[Action],
        scroll_offset: Vector,
        point: Point,
    ) -> Option<(Action, Vector)> {
        let mut index = 0;
        for action in actions {
            match action.get_action() {
                ActionWrapper::Loop(actions, _) => {
                    let top_left =
                        Point::new(0.0, index as f32 * (ACTION_SIZE.height + ACTION_PADDING))
                            - scroll_offset;
                    let bounds = Rectangle::new(top_left, ACTION_SIZE);
                    if bounds.contains(point) {
                        return Some((action.clone(), point - top_left));
                    }

                    index += 1;
                    if let Some(value) = Action::get_offset(
                        actions.as_slice(),
                        scroll_offset
                            - Vector::new(
                                LOOP_PADDING,
                                index as f32 * (ACTION_SIZE.height + ACTION_PADDING),
                            ),
                        point,
                    ) {
                        return Some(value);
                    }

                    index += action.calculate_length() - 2;

                    let bottom_top_left =
                        Point::new(0.0, index as f32 * (ACTION_SIZE.height + ACTION_PADDING))
                            - scroll_offset;
                    let bounds = Rectangle::new(bottom_top_left, ACTION_SIZE);
                    if bounds.contains(point) {
                        return Some((action.clone(), point - top_left));
                    }

                    index += 1;
                }

                _ => {
                    let top_left =
                        Point::new(0.0, index as f32 * (ACTION_SIZE.height + ACTION_PADDING))
                            - scroll_offset;
                    let bounds = Rectangle::new(top_left, ACTION_SIZE);
                    if bounds.contains(point) {
                        return Some((action.clone(), point - top_left));
                    }

                    index += action.calculate_length();
                }
            }
        }

        None
    }

    pub fn get_drag_index(
        drag_action: &Action,
        actions: &[Action],
        all_actions: &[Action],
        scroll_offset: Vector,
        point: Point,
    ) -> Option<Index> {
        let mut index = 0;
        for action in actions {
            if drag_action.contains(action) {
                index += action.calculate_length();
                continue;
            }
            match action.get_action() {
                ActionWrapper::Loop(loop_actions, _) => {
                    let loop_index = action.index_from(all_actions).unwrap();
                    let top_left = Point::new(
                        -LOOP_PADDING,
                        (index as f32 - 0.5) * (ACTION_SIZE.height + ACTION_PADDING),
                    ) - scroll_offset;
                    let bounds = Rectangle::new(
                        top_left,
                        Size::new(
                            ACTION_SIZE.width + (LOOP_PADDING * 2.0),
                            ACTION_SIZE.height + ACTION_PADDING,
                        ),
                    );
                    if bounds.contains(point) {
                        return Some(loop_index);
                    }

                    index += 1;

                    if !loop_actions.is_empty() {
                        if let Some(value) = Action::get_drag_index(
                            drag_action,
                            loop_actions.as_slice(),
                            actions,
                            scroll_offset
                                - Vector::new(
                                    LOOP_PADDING,
                                    index as f32 * (ACTION_SIZE.height + ACTION_PADDING),
                                ),
                            point,
                        ) {
                            return Some(value);
                        }

                        index += action.calculate_length() - 2;
                    }

                    let top_left = Point::new(
                        -LOOP_PADDING,
                        (index as f32 - 0.5) * (ACTION_SIZE.height + ACTION_PADDING),
                    ) - scroll_offset;
                    let bounds = Rectangle::new(
                        top_left,
                        Size::new(
                            ACTION_SIZE.width + (LOOP_PADDING * 2.0),
                            ACTION_SIZE.height + ACTION_PADDING,
                        ),
                    );
                    if bounds.contains(point) {
                        let mut parents = loop_index.parents.clone();
                        let drag_index = drag_action.index_from(all_actions).unwrap();
                        parents.push(loop_index.index);
                        let index = Index {
                            index: if drag_index.parents == parents {
                                loop_actions.len() - 1
                            } else {
                                loop_actions.len()
                            },
                            parents,
                        };
                        if index != drag_index {
                            return Some(index);
                        }
                    }

                    index += 1;
                }

                _ => {
                    let top_left = Point::new(
                        -LOOP_PADDING,
                        (index as f32 - 0.5) * (ACTION_SIZE.height + ACTION_PADDING),
                    ) - scroll_offset;
                    let bounds = Rectangle::new(
                        top_left,
                        Size::new(
                            ACTION_SIZE.width + (LOOP_PADDING * 2.0),
                            ACTION_SIZE.height + ACTION_PADDING,
                        ),
                    );
                    if bounds.contains(point) {
                        return Some(action.index_from(all_actions).unwrap());
                    }

                    index += action.calculate_length();
                }
            }
        }

        let top_left = Point::new(
            -LOOP_PADDING,
            (index as f32 - 0.5) * (ACTION_SIZE.height + ACTION_PADDING),
        ) - scroll_offset;
        let bounds = Rectangle::new(
            top_left,
            Size::new(
                ACTION_SIZE.width + (LOOP_PADDING * 2.0),
                ACTION_SIZE.height + ACTION_PADDING,
            ),
        );
        if bounds.contains(point) {
            let last = actions.last().unwrap();
            if !drag_action.contains(last) {
                let index = last.index_from(all_actions).unwrap();
                return Some(Index {
                    index: if index.parents == drag_action.index_from(all_actions).unwrap().parents
                    {
                        index.index
                    } else {
                        index.index + 1
                    },
                    parents: index.parents,
                });
            }
        }

        None
    }

    pub fn get_count_to(
        &self,
        actions: &[Action],
        parents: Vec<usize>,
        index: usize,
    ) -> Result<usize, ()> {
        let mut index = index;
        for action in actions {
            if *self == *action {
                return Ok(index);
            }

            match action.get_action() {
                ActionWrapper::Loop(actions, _) => {
                    index += 1;
                    let mut parents = parents.clone();
                    parents.push(index);
                    if let Ok(index) = self.get_count_to(actions.as_slice(), parents, index) {
                        return Ok(index);
                    } else {
                        index += action.calculate_length() - 1;
                    }
                }

                _ => index += 1,
            }
        }

        Err(())
    }

    fn index_from_recurse(&self, actions: &[Action], parents: Vec<usize>) -> Result<Index, ()> {
        let mut index = 0;
        for action in actions {
            if *self == *action {
                return Ok(Index { index, parents });
            }

            match action.get_action() {
                ActionWrapper::Loop(actions, _) => {
                    let mut parents = parents.clone();
                    parents.push(index);
                    if let Ok(index) = self.index_from_recurse(actions.as_slice(), parents) {
                        return Ok(index);
                    }
                }

                _ => (),
            }

            index += 1;
        }

        Err(())
    }

    pub fn index_from(&self, actions: &[Action]) -> Result<Index, ()> {
        self.index_from_recurse(actions, Vec::new())
    }

    pub fn get_action(&self) -> ActionWrapper {
        self.0.try_borrow().unwrap().action.clone()
    }

    pub fn get_delay(&self) -> Option<Duration> {
        self.0.try_borrow().unwrap().delay.clone()
    }

    pub fn set_delay(&self, delay: Option<Duration>) {
        self.0.try_borrow_mut().unwrap().delay = delay;
    }

    pub fn set_action(&self, action: ActionWrapper) {
        self.0.try_borrow_mut().unwrap().action = action;
    }

    pub fn on_close_button(&self, offset: Vector) -> bool {
        let close_button_bounds =
            Rectangle::new(Point::ORIGIN + CLOSE_BUTTON_OFFSET, CLOSE_BUTTON_SIZE);

        close_button_bounds.contains(Point::ORIGIN + offset)
    }

    fn draw_all(
        actions: &[Action],
        frame: &mut Frame,
        theme: &Theme,
        scroll_offset: Vector,
        selected: &Option<Action>,
        drag: &Option<Drag>,
    ) {
        let mut index = 0;
        for action in actions {
            let position = Point::new(0.0, index as f32 * (ACTION_SIZE.height + ACTION_PADDING))
                - scroll_offset;
            action.draw(frame, theme, position, selected, drag);

            index += action.calculate_length();
        }
    }

    fn draw_base(&self, frame: &mut Frame, theme: &Theme, position: Point, selected: bool) {
        let color = if selected {
            theme.extended_palette().background.strong.color
        } else {
            theme.extended_palette().background.weak.color
        };
        frame.fill_rectangle(position, ACTION_SIZE, color);
    }

    fn draw_close_button(&self, frame: &mut Frame, theme: &Theme, position: Point) {
        // TODO: Will be buggy until iced supports vectorial text https://github.com/iced-rs/iced/pull/1610
        frame.fill_text(canvas::Text {
            content: Icon::Close.into(),
            position: position + CLOSE_BUTTON_OFFSET,
            color: theme.palette().danger,
            size: CLOSE_BUTTON_SIZE.width,
            font: ICON_FONT,
            horizontal_alignment: iced::alignment::Horizontal::Left,
            vertical_alignment: iced::alignment::Vertical::Top,
        });
    }

    fn draw_delay(
        &self,
        frame: &mut Frame,
        theme: &Theme,
        position: Point,
        delay: Option<Duration>,
    ) {
        let delay = if let Some(delay) = delay {
            format!("{}ms", delay.as_millis())
        } else {
            "Default delay".to_string()
        };

        frame.fill_text(canvas::Text {
            content: delay,
            position: position + DELAY_OFFSET,
            color: theme.palette().text,
            size: DELAY_SIZE,
            font: ROBOTO,
            horizontal_alignment: iced::alignment::Horizontal::Right,
            vertical_alignment: iced::alignment::Vertical::Bottom,
        });
    }

    fn draw_action_title(&self, frame: &mut Frame, theme: &Theme, position: Point, title: String) {
        frame.fill_text(canvas::Text {
            content: title,
            position: position + TITLE_OFFSET,
            color: theme.palette().text,
            size: TITLE_SIZE,
            font: ROBOTO_BOLD,
            horizontal_alignment: iced::alignment::Horizontal::Left,
            vertical_alignment: iced::alignment::Vertical::Center,
        });
    }

    fn draw_arguments(
        &self,
        frame: &mut Frame,
        theme: &Theme,
        position: Point,
        selected: bool,
        arguments: Arguments,
    ) {
        for argument in arguments.args {
            frame.fill_rectangle(
                position
                    + if let ArgumentType::Boolean(_, _) = &argument.arg_type {
                        Vector::new(argument.offset + (argument.width - ARGUMENT_SIZE) / 2.0, ((ACTION_SIZE.height - ARGUMENT_SIZE) / 2.0) + (ARGUMENT_SIZE / 2.0))
                    } else {
                        Vector::new(argument.offset, (ACTION_SIZE.height - ARGUMENT_SIZE) / 2.0)
                    },
                Size::new(
                    if let ArgumentType::Boolean(_, _) = &argument.arg_type {
                        ARGUMENT_SIZE
                    } else {
                        argument.width
                    },
                    ARGUMENT_SIZE,
                ),
                if selected {
                    theme.extended_palette().primary.strong.color
                } else {
                    theme.extended_palette().primary.weak.color
                },
            );

            if let Some((offset, content)) = argument.pre_text {
                frame.fill_text(canvas::Text {
                    content,
                    position: position + Vector::new(offset + 2.5, ACTION_SIZE.height / 2.0),
                    color: theme.palette().text,
                    size: ARGUMENT_SIZE,
                    font: ROBOTO,
                    horizontal_alignment: iced::alignment::Horizontal::Left,
                    vertical_alignment: iced::alignment::Vertical::Center,
                });
            }

            if let ArgumentType::Color(color) = argument.arg_type {
                frame.fill_rectangle(
                    position
                        + Vector::new(
                            argument.offset + 2.5,
                            ((ACTION_SIZE.height - ARGUMENT_SIZE) / 2.0) + 2.5,
                        ),
                    Size::new(argument.width - 5.0, ARGUMENT_SIZE - 5.0),
                    color,
                );
            } else if let ArgumentType::Boolean(boolean, content) = argument.arg_type {
                frame.fill_rectangle(
                    position
                        + Vector::new((argument.offset + (argument.width - ARGUMENT_SIZE) / 2.0) + 2.5, ((ACTION_SIZE.height - ARGUMENT_SIZE) / 2.0) + (ARGUMENT_SIZE / 2.0) + 2.5),
                    Size::new(ARGUMENT_SIZE - 5.0, ARGUMENT_SIZE - 5.0),
                    if boolean {
                        theme.palette().success
                    } else {
                        theme.palette().danger
                    },
                );

                frame.fill_text(canvas::Text {
                    content,
                    position: position
                        + Vector::new(
                            argument.offset + 2.5 + (argument.width / 2.0),
                            (ACTION_SIZE.height - ARGUMENT_SIZE) / 2.0,
                        ),
                    color: theme.palette().text,
                    size: ARGUMENT_SIZE,
                    font: ROBOTO,
                    horizontal_alignment: iced::alignment::Horizontal::Center,
                    vertical_alignment: iced::alignment::Vertical::Center,
                });
            } else {
                let content = match argument.arg_type {
                    ArgumentType::String(string) => string,
                    ArgumentType::Number(num) => num.to_string(),
                    ArgumentType::Color(_) => unreachable!(),
                    ArgumentType::Boolean(_, _) => unreachable!(),
                };

                frame.fill_text(canvas::Text {
                    content,
                    position: position
                        + Vector::new(
                            argument.offset + 2.5 + (argument.width / 2.0),
                            ACTION_SIZE.height / 2.0,
                        ),
                    color: theme.palette().text,
                    size: ARGUMENT_SIZE,
                    font: ROBOTO,
                    horizontal_alignment: iced::alignment::Horizontal::Center,
                    vertical_alignment: iced::alignment::Vertical::Center,
                });
            }

            if let Some(content) = argument.post_text {
                frame.fill_text(canvas::Text {
                    content,
                    position: position
                        + Vector::new(
                            argument.offset + argument.width + 2.5,
                            ACTION_SIZE.height / 2.0,
                        ),
                    color: theme.palette().text,
                    size: ARGUMENT_SIZE,
                    font: ROBOTO,
                    horizontal_alignment: iced::alignment::Horizontal::Left,
                    vertical_alignment: iced::alignment::Vertical::Center,
                });
            }
        }
    }

    fn draw(
        &self,
        frame: &mut Frame,
        theme: &Theme,
        position: Point,
        selected: &Option<Action>,
        drag: &Option<Drag>,
    ) {
        if let Some(drag) = drag {
            if drag.action == *self {
                return;
            }
        }

        let selected_bool = if let Some(selected) = selected {
            if *selected == *self {
                true
            } else {
                false
            }
        } else {
            false
        };

        match self.get_action() {
            ActionWrapper::Empty => {
                self.draw_base(frame, theme, position, selected_bool);
                self.draw_close_button(frame, theme, position);
                self.draw_delay(frame, theme, position, self.get_delay());
                self.draw_action_title(frame, theme, position, "Delay".to_string());
            }
            ActionWrapper::SetLed((r, g, b)) => {
                self.draw_base(frame, theme, position, selected_bool);
                self.draw_close_button(frame, theme, position);
                self.draw_delay(frame, theme, position, self.get_delay());
                self.draw_action_title(frame, theme, position, "Set LED Color".to_string());
                self.draw_arguments(
                    frame,
                    theme,
                    position,
                    selected_bool,
                    Arguments::after_title("Set LED Color".to_string())
                        .with_color(Color::from_rgb8(r, g, b)),
                );
            }
            ActionWrapper::ClearLed => {
                self.draw_base(frame, theme, position, selected_bool);
                self.draw_close_button(frame, theme, position);
                self.draw_delay(frame, theme, position, self.get_delay());
                self.draw_action_title(frame, theme, position, "Clear LED".to_string());
            }
            ActionWrapper::KeyDown(key) => {
                self.draw_base(frame, theme, position, selected_bool);
                self.draw_close_button(frame, theme, position);
                self.draw_delay(frame, theme, position, self.get_delay());
                self.draw_action_title(frame, theme, position, "Hold Key".to_string());
                self.draw_arguments(
                    frame,
                    theme,
                    position,
                    selected_bool,
                    Arguments::after_title("Hold Key".to_string()).with_key(key),
                );
            }
            ActionWrapper::KeyUp(key) => {
                self.draw_base(frame, theme, position, selected_bool);
                self.draw_close_button(frame, theme, position);
                self.draw_delay(frame, theme, position, self.get_delay());
                self.draw_action_title(frame, theme, position, "Release Key".to_string());
                self.draw_arguments(
                    frame,
                    theme,
                    position,
                    selected_bool,
                    Arguments::after_title("Release Key".to_string()).with_key(key),
                );
            }
            ActionWrapper::KeyPress(key, delay) => {
                self.draw_base(frame, theme, position, selected_bool);
                self.draw_close_button(frame, theme, position);
                self.draw_delay(frame, theme, position, self.get_delay());
                self.draw_action_title(frame, theme, position, "Press Key".to_string());
                self.draw_arguments(
                    frame,
                    theme,
                    position,
                    selected_bool,
                    Arguments::after_title("Press Key".to_string())
                        .with_key(key)
                        .with_delay(delay),
                );
            }
            ActionWrapper::ConsumerPress(key, delay) => {
                self.draw_base(frame, theme, position, selected_bool);
                self.draw_close_button(frame, theme, position);
                self.draw_delay(frame, theme, position, self.get_delay());
                self.draw_action_title(frame, theme, position, "Press Consumer Key".to_string());
                self.draw_arguments(
                    frame,
                    theme,
                    position,
                    selected_bool,
                    Arguments::after_title("Press Consumer Key".to_string())
                        .with_consumer(key)
                        .with_delay(delay),
                );
            }
            ActionWrapper::String(string, delay) => {
                self.draw_base(frame, theme, position, selected_bool);
                self.draw_close_button(frame, theme, position);
                self.draw_delay(frame, theme, position, self.get_delay());
                self.draw_action_title(frame, theme, position, "Type String".to_string());
                self.draw_arguments(
                    frame,
                    theme,
                    position,
                    selected_bool,
                    Arguments::after_title("Type String".to_string())
                        .with_string(string)
                        .with_delay(delay),
                );
            }
            ActionWrapper::Chord(keys, delay) => {
                self.draw_base(frame, theme, position, selected_bool);
                self.draw_close_button(frame, theme, position);
                self.draw_delay(frame, theme, position, self.get_delay());
                self.draw_action_title(frame, theme, position, "Chord".to_string());
                self.draw_arguments(
                    frame,
                    theme,
                    position,
                    selected_bool,
                    Arguments::after_title("Chord".to_string())
                        .with_chord(keys)
                        .with_delay(delay),
                );
            }
            ActionWrapper::Loop(actions, loop_count) => {
                self.draw_base(frame, theme, position, selected_bool);
                self.draw_close_button(frame, theme, position);
                self.draw_action_title(frame, theme, position, "Begin Loop".to_string());
                self.draw_arguments(
                    frame,
                    theme,
                    position,
                    selected_bool,
                    Arguments::after_title("Begin Loop".to_string())
                        .with_labeled_number("Loop Count:", loop_count as f32),
                );

                let mut index = 1;
                for action in actions {
                    action.draw(
                        frame,
                        theme,
                        position
                            + Vector::new(
                                LOOP_PADDING,
                                index as f32 * (ACTION_SIZE.height + ACTION_PADDING),
                            ),
                        selected,
                        drag,
                    );
                    index += action.calculate_length();
                }

                let position = position
                    + Vector::new(0.0, index as f32 * (ACTION_SIZE.height + ACTION_PADDING));
                self.draw_base(frame, theme, position, selected_bool);
                self.draw_delay(frame, theme, position, self.get_delay());
                self.draw_action_title(frame, theme, position, "End Loop".to_string());
            }
        }
    }
}

impl From<MacroAction> for Action {
    fn from(action: MacroAction) -> Self {
        Action::new(action)
    }
}

impl From<Action> for MacroAction {
    fn from(action: Action) -> Self {
        action.0.try_borrow().unwrap().clone()
    }
}

impl From<MacroFrame> for Action {
    fn from(frame: MacroFrame) -> Self {
        Action::from(MacroAction::from(frame))
    }
}

impl From<Action> for MacroFrame {
    fn from(action: Action) -> Self {
        MacroFrame::from(MacroAction::from(action))
    }
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Action {
    pub fn contains(&self, action: &Action) -> bool {
        if self == action {
            return true;
        }

        match self.get_action() {
            ActionWrapper::Loop(actions, _) => {
                for loop_action in actions {
                    if loop_action.contains(action) {
                        return true;
                    }
                }
            }
            _ => {}
        }

        false
    }

    pub fn from_macro(macro_data: &crate::macro_parser::Macro) -> Vec<Action> {
        return Action::from_frames(&macro_data.frames, Vec::new());
    }

    fn from_frames(frames: &[MacroFrame], parents: Vec<Arc<Mutex<Index>>>) -> Vec<Action> {
        let mut actions = Vec::new();

        for frame in frames {
            actions.push(Action::from(MacroAction::from(frame.clone())));
        }

        actions
    }

    pub fn to_macro(actions: &[Action]) -> crate::macro_parser::Macro {
        let mut frames = Vec::new();

        for action in actions {
            frames.push(MacroFrame::from(action.clone()));
        }

        crate::macro_parser::Macro { frames }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum ArgumentType {
    String(String),
    Number(f32),
    Color(Color),
    Boolean(bool, String),
}

#[derive(Clone, Debug, PartialEq)]
struct Argument {
    pub arg_type: ArgumentType,
    pub width: f32,
    pub pre_text: Option<(f32, String)>,
    pub offset: f32,
    pub post_text: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
struct Arguments {
    pub args: Vec<Argument>,
    offset: f32,
}

impl Arguments {
    pub fn after_title(title: String) -> Self {
        Arguments {
            args: Vec::new(),
            // TODO: Use real text measurement
            offset: (title.len() as f32 * TITLE_SIZE / 2.3) + (2.0 * TITLE_OFFSET.x),
        }
    }

    pub fn with_spacer(mut self, width: f32) -> Self {
        self.offset += width;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.args.push(Argument {
            arg_type: ArgumentType::Color(color),
            width: 20.0,
            pre_text: Some((self.offset, "Color:".to_string())),
            offset: self.offset + Arguments::offset_from_text("Color:".to_string()),
            post_text: None,
        });

        self.offset += 20.0 + Arguments::offset_from_text("Color:".to_string()) + TITLE_OFFSET.x;

        self
    }

    pub fn with_delay(mut self, delay: Option<Duration>) -> Self {
        self.args.push(Argument {
            arg_type: if let Some(delay) = delay.as_ref() {
                ArgumentType::Number(delay.as_millis() as f32)
            } else {
                ArgumentType::String("Default".to_string())
            },
            width: 40.0,
            pre_text: Some((self.offset, "Delay:".to_string())),
            offset: self.offset + Arguments::offset_from_text("Delay:".to_string()),
            post_text: if delay.is_some() {
                Some("ms".to_string())
            } else {
                None
            },
        });

        self.offset += 40.0 + Arguments::offset_from_text("Delay:".to_string()) + TITLE_OFFSET.x;

        if delay.is_some() {
            self.offset += Arguments::offset_from_text("ms".to_string());
        }

        self
    }

    pub fn with_key(mut self, key: usbd_human_interface_device::page::Keyboard) -> Self {
        let content = KeyboardWrapper::from(key).to_string();
        let content = if content.len() > 7 {
            format!("{}...", &content[0..4])
        } else {
            content
        };

        self.args.push(Argument {
            arg_type: ArgumentType::String(content),
            width: 50.0,
            pre_text: Some((self.offset, "Key:".to_string())),
            offset: self.offset + Arguments::offset_from_text("Key:".to_string()),
            post_text: None,
        });

        self.offset += 50.0 + Arguments::offset_from_text("Key:".to_string()) + TITLE_OFFSET.x;

        self
    }

    pub fn with_consumer(mut self, key: usbd_human_interface_device::page::Consumer) -> Self {
        let content = ConsumerWrapper::from(key).to_string();
        let content = if content.len() > 7 {
            format!("{}...", &content[0..4])
        } else {
            content
        };

        self.args.push(Argument {
            arg_type: ArgumentType::String(content),
            width: 50.0,
            pre_text: Some((self.offset, "Consumer:".to_string())),
            offset: self.offset + Arguments::offset_from_text("Consumer:".to_string()),
            post_text: None,
        });

        self.offset += 50.0 + Arguments::offset_from_text("Consumer:".to_string()) + TITLE_OFFSET.x;

        self
    }

    pub fn with_string(mut self, content: String) -> Self {
        let content = if content.len() > 15 {
            format!("{}...", &content[0..12])
        } else {
            content
        };

        // TODO: Replace \n and \t with down and right arrow respectively
        self.args.push(Argument {
            arg_type: ArgumentType::String(content),
            width: 100.0,
            pre_text: Some((self.offset, "String:".to_string())),
            offset: self.offset + Arguments::offset_from_text("String:".to_string()),
            post_text: None,
        });

        self.offset += 100.0 + Arguments::offset_from_text("String:".to_string()) + TITLE_OFFSET.x;

        self
    }

    pub fn with_chord(mut self, keys: Vec<usbd_human_interface_device::page::Keyboard>) -> Self {
        let content = KeyboardWrapper::get_chord_string(&keys);
        let content = if content.len() > 7 {
            format!("{}...", &content[0..4])
        } else {
            content
        };

        // Should have booleans for Ctrl, Alt, Shift, and GUI (need new argument type) (should be success if true and danger if false) (should be square box like color argument type)
        // TODO: Replace \n and \t with down and right arrow respectively
        self.args.push(Argument {
            arg_type: ArgumentType::String(content),
            width: 50.0,
            pre_text: Some((self.offset, "Chord:".to_string())),
            offset: self.offset + Arguments::offset_from_text("Chord:".to_string()),
            post_text: None,
        });

        self.offset += 50.0 + Arguments::offset_from_text("Chord:".to_string()) + TITLE_OFFSET.x;

        self.with_labeled_boolean("CTRL", true)
            .with_labeled_boolean("SHIFT", false)
            .with_labeled_boolean("ALT", true)
            .with_labeled_boolean("GUI", true)
    }

    pub fn with_labeled_number(mut self, label: &str, number: f32) -> Self {
        self.args.push(Argument {
            arg_type: ArgumentType::Number(number),
            width: 40.0,
            pre_text: Some((self.offset, label.to_owned())),
            offset: self.offset + Arguments::offset_from_text(label.to_owned()),
            post_text: None,
        });

        self.offset += 40.0 + Arguments::offset_from_text(label.to_owned()) + TITLE_OFFSET.x;

        self
    }

    pub fn with_labeled_boolean(mut self, label: &str, boolean: bool) -> Self {
        self.args.push(Argument {
            arg_type: ArgumentType::Boolean(boolean, label.to_owned()),
            width: Arguments::offset_from_text(label.to_owned()),
            pre_text: None,
            offset: self.offset,
            post_text: None,
        });

        self.offset += Arguments::offset_from_text(label.to_owned()).max(ARGUMENT_SIZE) + TITLE_OFFSET.x;

        self
    }

    fn offset_from_text(text: String) -> f32 {
        // TODO: Use real text measurement
        (text.len() + 2) as f32 * ARGUMENT_TEXT_SIZE / 2.0
    }
}
