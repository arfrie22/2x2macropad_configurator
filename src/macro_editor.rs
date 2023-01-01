use std::borrow::{Borrow, BorrowMut};
use std::cell::{RefCell, Ref, RefMut};
use std::ops::Sub;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use iced::{mouse, Color, Vector, Size};
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{
    self, Canvas, Cursor, Frame, Geometry, Path, Stroke,
};
use iced::{Element, Length, Point, Rectangle, Theme};

use crate::font::{ROBOTO, Icon, ICON_FONT};
use crate::macro_parser::{MacroFrame, Macro, self};


const CLOSE_BUTTON_PADDING: f32 = 1.0;
pub const CLOSE_BUTTON_OFFSET: Vector = Vector::new(ACTION_SIZE.width - ACTION_SIZE.height - CLOSE_BUTTON_PADDING, CLOSE_BUTTON_PADDING);
pub const CLOSE_BUTTON_SIZE: Size = Size::new(ACTION_SIZE.height - (2.0 * CLOSE_BUTTON_PADDING), ACTION_SIZE.height - (2.0 * CLOSE_BUTTON_PADDING));

#[derive(Debug, Clone)]
pub struct Index {
    pub index: usize,
    pub parents: Vec<usize>,
}

impl Index {
    pub fn get_level(&self) -> usize {
        self.parents.len()
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

    pub fn add_to_macro(&self, frame: MacroFrame, actions: &mut Vec<Action>) {
        if self.parents.is_empty() {
            let mut parents = self.parents.clone();
            match actions[parents.remove(0)].get_action().borrow_mut() {
                ActionWrapper::Loop(actions, _) => {
                    let index = Index { index: self.index, parents };
                    index.add_to_macro(frame, actions)
                }
                _ => unreachable!(),
            }
        } else {
            actions.insert(self.index, Action::from(frame));
        }
    }

    pub fn remove_from_macro(&self, actions: &mut Vec<Action>) -> MacroFrame {
        if self.parents.is_empty() {
            let mut parents = self.parents.clone();
            match actions[parents.remove(0)].get_action().borrow_mut() {
                ActionWrapper::Loop(actions, _) => {
                    let index = Index { index: self.index, parents };
                    index.remove_from_macro(actions)
                }
                _ => unreachable!(),
            }
        } else {
            actions.remove(self.index).into()
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
pub enum Message {
    MoveFrame(Index, Index),
    RemoveFrame(Index),
    AddFrame(MacroFrame, Index),
    SelectFrame(Option<Index>),
    ReleaseGrab,
    DragStart,
    Scroll(Vector),
}

#[derive(Default, Debug)]
pub struct State {
    cache: canvas::Cache,
    scroll_offset: Vector,
    size_bounds: Rectangle,
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
        let max_x = ACTION_SIZE.width + (LOOP_PADDING * (Action::max_width(self.actions) - 1) as f32);

        max_x.sub(bounds.width).max(0.0)
    }

    fn get_max_y_offset(&self, bounds: &Rectangle) -> f32 {
        let max_y = (Action::max_length(self.actions) as f32 * (ACTION_SIZE.height + ACTION_PADDING)) - ACTION_PADDING;
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
        let cursor_position =
            if let Some(position) = cursor.position_in(&bounds) {
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
                            },
                            mouse::ScrollDelta::Pixels { x, y } => {
                                self.state.scroll_offset + Vector::new(x, -y)
                                
                            },
                        };

                        let scroll_offset = Vector::new(scroll_offset.x.max(0.0).min(self.get_max_x_offset(&bounds)), scroll_offset.y.max(0.0).min(self.get_max_y_offset(&bounds)));
                        Some(Message::Scroll(scroll_offset))
                    },
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        match *state {
                            (None, _) => {
                                if let Some((action, offset)) = Action::get_offset(self.actions, self.state.scroll_offset, cursor_position) {
                                    if action.on_close_button(offset) {
                                        *state = (None, None);
                                        println!("Close button pressed");
                                        return (event::Status::Captured, Some(Message::RemoveFrame(action.index_from(self.actions).unwrap())));
                                    }

                                    *state = (Some(Drag {
                                        action,
                                        drag_offset: offset,
                                        moved: false,
                                        to: cursor_position,
                                    }), None);

                                    return (event::Status::Captured, None);
                                }

                                if state.1.take().is_some() {
                                    return (event::Status::Captured, Some(Message::SelectFrame(None)));
                                }
                                None
                            },
                            _ => None,
                        }
                    }
                    mouse::Event::CursorMoved { .. } => {
                        if let Some(cursor_position) = cursor.position_in(&bounds) {
                            match state.0.take() {
                                Some(Drag { action, drag_offset, moved, to, .. }) => {
                                    let mut message = None;
                                    if !moved {
                                        if cursor_position != to {
                                            message = Some(Message::DragStart);
                                            *state = (Some(Drag { action, drag_offset, moved: true, to: cursor_position }), None); 
                                            return (event::Status::Captured, message);   
                                        }
                                    } else {
                                        // for action in self.actions {
                                        //     let index = action.try_lock().unwrap().index.try_lock().unwrap().clone();
                                        //     let action_index = index.get_root_action_index();

                                        //     // TODO: Check if current selected index is in the list actions
                                        //     for action in actions.iter() {
                                        //         if action.try_lock().unwrap().index.try_lock().unwrap().clone().get_root_action_index() == action_index {
                                        //             continue;
                                        //         }
                                        //     }

                                            

                                        //     let top_left = index.get_top_left(self.state.scroll_offset) - Vector::new(-LOOP_PADDING, -ACTION_SIZE.height / 2.0);
                                        //     let bounds = Rectangle::new(top_left, Size::new(ACTION_SIZE.width + LOOP_PADDING * 2.0, ACTION_SIZE.height));
                                        //     if bounds.contains(cursor_position) {
                                        //         message = Some(Message::MoveFrame(original_index, index));
                                        //         break;
                                        //     }
                                        // }
                                    }

                                    *state = (Some(Drag { action, drag_offset, moved, to: cursor_position }), None); 
                                    return (event::Status::Captured, message);
                                }
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        match state.0.take() {
                            Some(Drag { action, moved, to, .. }) => {
                                if !moved {
                                    let index = action.index_from(self.actions).unwrap();
                                    *state = (None, Some(action));
                                    return (event::Status::Captured, Some(Message::SelectFrame(Some(index))));
                                }
                                // for action in self.actions {
                                //     if let Some(offset) = action.index.get_offset(self.state.scroll_offset, cursor_position) {
                                //         if Index::on_close_button(offset) {
                                //             return (event::Status::Captured, Some(Message::RemoveFrame(action.index.clone())));
                                //         }

                                //         return (event::Status::Captured, Some(Message::MoveFrame(from, action.index.clone())));
                                //     }
                                // }

                                return (event::Status::Captured, Some(Message::ReleaseGrab));
                            }
                            None => None,
                        }
                    }
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
        let content =
            self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
                Action::draw_all(self.actions, frame, theme, self.state.scroll_offset, &state.1, &state.0);

                frame.stroke(
                    &Path::rectangle(Point::ORIGIN, frame.size()),
                    Stroke::default().with_width(2.0),
                );

                frame.fill_text(canvas::Text {
                    content: "Click to add a new curve".to_string(),
                    position: Point::ORIGIN,
                    color: Color::from_rgb8(0xFF, 0x00, 0x00),
                    size: 30.0,
                    font: ROBOTO,
                    horizontal_alignment: iced::alignment::Horizontal::Left,
                    vertical_alignment: iced::alignment::Vertical::Top,
                });
            });


            if let Some(drag) = state.0.as_ref() {
                let drag_action = drag.draw(&theme, bounds, cursor);
                let placeholder = {
                    let mut frame = Frame::new(bounds.size());

                    let index = drag.action.index_from(self.actions).unwrap();

                    let position = Point::new(
                        LOOP_PADDING * index.parents.len() as f32,
                        {
                            let mut y = index.index;
                            for parent in index.parents {
                                y += parent + 1;
                            }
                            y as f32 * (ACTION_SIZE.height + ACTION_PADDING)
                        },
                    ) - self.state.scroll_offset;
                    
                    frame.stroke(
                        &Path::rectangle(position, Size::new(ACTION_SIZE.width + (LOOP_PADDING * (drag.action.calculate_width() - 1) as f32) , ((ACTION_SIZE.height + ACTION_PADDING) * drag.action.calculate_length() as f32) - ACTION_PADDING)),
                        Stroke::default().with_color(Color::from_rgb8(0x00, 0xFF, 0xFF)).with_width(3.0),
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
                    if let Some((action, offset)) = Action::get_offset(self.actions, self.state.scroll_offset, cursor_position) {
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
            },
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
    KeyPress(usbd_human_interface_device::page::Keyboard, Option<Duration>),
    ConsumerPress(usbd_human_interface_device::page::Consumer, Option<Duration>),
    String(String, Option<Duration>),
    Chord(Vec<usbd_human_interface_device::page::Keyboard>, Option<Duration>),
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
            },
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
            macro_parser::ActionType::ConsumerPress(key, delay) => ActionWrapper::ConsumerPress(key, delay),
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
                ActionWrapper::KeyPress(key, delay) => macro_parser::ActionType::KeyPress(key, delay),
                ActionWrapper::ConsumerPress(key, delay) => macro_parser::ActionType::ConsumerPress(key, delay),
                ActionWrapper::String(string, delay) => macro_parser::ActionType::String(string, delay),
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
pub struct Action (Rc<RefCell<MacroAction>>);
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
            length = length.max(action.calculate_length());
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

    pub fn get_offset(actions: &[Action], scroll_offset: Vector, point: Point) -> Option<(Action, Vector)> {
        let mut index = 0;
        for action in actions {
            match action.get_action() {
                ActionWrapper::Loop(actions, _) => {
                    let top_left = Point::new(0.0, index as f32 * (ACTION_SIZE.height + ACTION_PADDING)) - scroll_offset;
                    let bounds = Rectangle::new(top_left, ACTION_SIZE);
                    if bounds.contains(point) {
                        return Some((action.clone(), point - top_left))
                    }
                    
                    index += 1;
                    if let Some(value) = Action::get_offset(actions.as_slice(), scroll_offset - Vector::new(LOOP_PADDING, index as f32 * (ACTION_SIZE.height + ACTION_PADDING)), point) {
                        return Some(value)
                    }

                    index += action.calculate_length() - 2;
    
                    let bottom_top_left = Point::new(0.0, index as f32 * (ACTION_SIZE.height + ACTION_PADDING)) - scroll_offset;
                    let bounds = Rectangle::new(bottom_top_left, ACTION_SIZE);
                    if bounds.contains(point) {
                        return Some((action.clone(), point - top_left))
                    }

                    index += action.calculate_length();
                },

                _ => {
                    let top_left = Point::new(0.0, index as f32 * (ACTION_SIZE.height + ACTION_PADDING)) - scroll_offset;
                    let bounds = Rectangle::new(top_left, ACTION_SIZE);
                    if bounds.contains(point) {
                        return Some((action.clone(), point - top_left))
                    }

                    index += action.calculate_length();
                }
            }
        }

        None
    }

    fn index_from_recurse(&self, actions: &[Action], parents: Vec<usize>) -> Result<Index, ()> {
        let mut index = 0;
        for action in actions {
            if *self == *action {
                return Ok(Index { index, parents })
            }

            match action.get_action() {
                ActionWrapper::Loop(actions, _) => {
                    let mut parents = parents.clone();
                    parents.push(index);
                    if let Ok(index) = self.index_from_recurse(actions.as_slice(), parents) {
                        return Ok(index)
                    }
                },

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
        let close_button_bounds = Rectangle::new(
            Point::ORIGIN + CLOSE_BUTTON_OFFSET,
            CLOSE_BUTTON_SIZE
        );

        close_button_bounds.contains(Point::ORIGIN + offset)
    }


    fn draw_all(actions: &[Action], frame: &mut Frame, theme: &Theme, scroll_offset: Vector, selected: &Option<Action>, drag: &Option<Drag>) {
        let mut index = 0;
        for action in actions {
            let position = Point::new(0.0, index as f32 * (ACTION_SIZE.height + ACTION_PADDING)) - scroll_offset;
            action.draw(frame, theme, position, selected, drag);

            index += action.calculate_length();
        }
    }

    fn draw_base(&self, frame: &mut Frame, theme: &Theme, position: Point, selected: bool, closeable: bool) {
        // TODO: COLORS
        let color = if selected {
            // theme.selected
            Color::from_rgb8(0x00, 0xFF, 0xFF)
        } else {
            // theme.background
            Color::from_rgb8(0x00, 0x00, 0xFF)
        };
        frame.fill_rectangle(position, ACTION_SIZE, color);

        // TODO: Will be buggy until iced supports vectorial text https://github.com/iced-rs/iced/pull/1610
        if closeable {
            frame.fill_text(canvas::Text {
                content: Icon::Close.into(),
                position: position + CLOSE_BUTTON_OFFSET,
                color: Color::from_rgb8(0xFF, 0x00, 0x00),
                size: CLOSE_BUTTON_SIZE.width,
                font: ICON_FONT,
                horizontal_alignment: iced::alignment::Horizontal::Left,
                vertical_alignment: iced::alignment::Vertical::Top,
            });
        }
    }

    fn draw(&self, frame: &mut Frame, theme: &Theme, position: Point, selected: &Option<Action>, drag: &Option<Drag>) {
        if let Some(drag) = drag {
            if drag.action == *self {
                return
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
                self.draw_base(frame, theme, position, selected_bool, true);
            },
            ActionWrapper::SetLed(_) => {
                self.draw_base(frame, theme, position, selected_bool, true);
            },
            ActionWrapper::ClearLed => {
                self.draw_base(frame, theme, position, selected_bool, true);
            },
            ActionWrapper::KeyDown(_) => {
                self.draw_base(frame, theme, position, selected_bool, true);
            },
            ActionWrapper::KeyUp(_) => {
                self.draw_base(frame, theme, position, selected_bool, true);
            },
            ActionWrapper::KeyPress(_, _) => {
                self.draw_base(frame, theme, position, selected_bool, true);
            },
            ActionWrapper::ConsumerPress(_, _) => {
                self.draw_base(frame, theme, position, selected_bool, true);
            },
            ActionWrapper::String(_, _) => {
                self.draw_base(frame, theme, position, selected_bool, true);
            },
            ActionWrapper::Chord(_, _) => {
                self.draw_base(frame, theme, position, selected_bool, true);
            },
            ActionWrapper::Loop(actions, _) => {
                self.draw_base(frame, theme, position, selected_bool, true);
                let mut index = 1;
                for action in actions {
                    action.draw(frame, theme, position + Vector::new(LOOP_PADDING, index as f32 * (ACTION_SIZE.height + ACTION_PADDING)), selected, drag);
                    index += action.calculate_length();
                }

                self.draw_base(frame, theme, position + Vector::new(0.0, index as f32 * (ACTION_SIZE.height + ACTION_PADDING)), selected_bool, false);
            },
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
}
