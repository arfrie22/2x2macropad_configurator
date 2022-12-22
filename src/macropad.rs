
    use iced::touch;
    use iced::widget::canvas;
    use iced::widget::canvas::event::{self, Event};
    use iced::widget::canvas::{
        Cache, Canvas, Cursor, Frame, Geometry, Path, Text,
    };
    use iced::{
        alignment, mouse, Color, Element, Length, Point, Rectangle, Size,
        Theme, Vector,
    };
    use std::future::Future;
    use std::ops::RangeInclusive;
    use std::time::{Duration, Instant};

    use crate::macro_parser::Macropad;

    pub struct Pad {
        state: State,
        info_box: Option<InfoBox>,
        
    }

    #[derive(Debug, Clone)]
    pub enum Message {
        UpdateMacropad(&'static Macropad),
        HoverKey(Option<usize>),
        ClickKey(usize),
        ReleaseKey(usize),
    }

    #[derive(Debug, Clone)]
    pub struct InfoBox {
        pub content: String,
        pub x: f32,
        pub y: f32,
    }

    impl Default for Pad {
        fn default() -> Self {
            Self {
                state: State::default(),
                info_box: None,
            }
        }
    }

    impl Pad {
        pub fn update(&mut self, message: Message) {
            // match message {
            //     Message::Populate(cell) => {
            //         self.state.populate(cell);
            //         self.life_cache.clear();

            //         self.preset = Preset::Custom;
            //     }
            //     Message::Unpopulate(cell) => {
            //         self.state.unpopulate(&cell);
            //         self.life_cache.clear();

            //         self.preset = Preset::Custom;
            //     }
            //     Message::Translated(translation) => {
            //         self.translation = translation;

            //         self.life_cache.clear();
            //         self.grid_cache.clear();
            //     }
            //     Message::Scaled(scaling, translation) => {
            //         self.scaling = scaling;

            //         if let Some(translation) = translation {
            //             self.translation = translation;
            //         }

            //         self.life_cache.clear();
            //         self.grid_cache.clear();
            //     }
            //     Message::Ticked {
            //         result: Ok(life),
            //         tick_duration,
            //     } => {
            //         self.state.update(life);
            //         self.life_cache.clear();

            //         self.last_tick_duration = tick_duration;
            //     }
            //     Message::Ticked {
            //         result: Err(error), ..
            //     } => {
            //         dbg!(error);
            //     }
            // }
        }

        pub fn view(&self) -> Element<Message> {
            Canvas::new(self)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }

        pub fn clear(&mut self) {
            // self.state = State::default();
            // self.preset = Preset::Custom;

            // self.life_cache.clear();
        }

        // fn visible_region(&self, size: Size) -> Region {
        //     let width = size.width / self.scaling;
        //     let height = size.height / self.scaling;

        //     Region {
        //         x: -self.translation.x - width / 2.0,
        //         y: -self.translation.y - height / 2.0,
        //         width,
        //         height,
        //     }
        // }

        // fn project(&self, position: Point, size: Size) -> Point {
        //     let region = self.visible_region(size);

        //     Point::new(
        //         position.x / self.scaling + region.x,
        //         position.y / self.scaling + region.y,
        //     )
        // }
    }

    impl canvas::Program<Message> for Pad {
        type State = Interaction;

        fn update(
            &self,
            interaction: &mut Interaction,
            event: Event,
            bounds: Rectangle,
            cursor: Cursor,
        ) -> (event::Status, Option<Message>) {
            if let Event::Mouse(mouse::Event::ButtonReleased(_)) = event {
                *interaction = Interaction::None;
            }

            let cursor_position =
                if let Some(position) = cursor.position_in(&bounds) {
                    position
                } else {
                    return (event::Status::Ignored, None);
                };

            let cell = Cell::at(self.project(cursor_position, bounds.size()));
            let is_populated = self.state.contains(&cell);

            let (populate, unpopulate) = if is_populated {
                (None, Some(Message::Unpopulate(cell)))
            } else {
                (Some(Message::Populate(cell)), None)
            };

            match event {
                Event::Touch(touch::Event::FingerMoved { .. }) => {
                    let message = {
                        *interaction = if is_populated {
                            Interaction::Erasing
                        } else {
                            Interaction::Drawing
                        };

                        populate.or(unpopulate)
                    };

                    (event::Status::Captured, message)
                }
                Event::Mouse(mouse_event) => match mouse_event {
                    mouse::Event::ButtonPressed(button) => {
                        let message = match button {
                            mouse::Button::Left => {
                                *interaction = if is_populated {
                                    Interaction::Erasing
                                } else {
                                    Interaction::Drawing
                                };

                                populate.or(unpopulate)
                            }
                            mouse::Button::Right => {
                                *interaction = Interaction::Panning {
                                    translation: self.translation,
                                    start: cursor_position,
                                };

                                None
                            }
                            _ => None,
                        };

                        (event::Status::Captured, message)
                    }
                    mouse::Event::CursorMoved { .. } => {
                        let message = match *interaction {
                            Interaction::Drawing => populate,
                            Interaction::Erasing => unpopulate,
                            Interaction::Panning { translation, start } => {
                                Some(Message::Translated(
                                    translation
                                        + (cursor_position - start)
                                            * (1.0 / self.scaling),
                                ))
                            }
                            _ => None,
                        };

                        let event_status = match interaction {
                            Interaction::None => event::Status::Ignored,
                            _ => event::Status::Captured,
                        };

                        (event_status, message)
                    }
                    mouse::Event::WheelScrolled { delta } => match delta {
                        mouse::ScrollDelta::Lines { y, .. }
                        | mouse::ScrollDelta::Pixels { y, .. } => {
                            if y < 0.0 && self.scaling > Self::MIN_SCALING
                                || y > 0.0 && self.scaling < Self::MAX_SCALING
                            {
                                let old_scaling = self.scaling;

                                let scaling = (self.scaling * (1.0 + y / 30.0))
                                    .max(Self::MIN_SCALING)
                                    .min(Self::MAX_SCALING);

                                let translation =
                                    if let Some(cursor_to_center) =
                                        cursor.position_from(bounds.center())
                                    {
                                        let factor = scaling - old_scaling;

                                        Some(
                                            self.translation
                                                - Vector::new(
                                                    cursor_to_center.x * factor
                                                        / (old_scaling
                                                            * old_scaling),
                                                    cursor_to_center.y * factor
                                                        / (old_scaling
                                                            * old_scaling),
                                                ),
                                        )
                                    } else {
                                        None
                                    };

                                (
                                    event::Status::Captured,
                                    Some(Message::Scaled(scaling, translation)),
                                )
                            } else {
                                (event::Status::Captured, None)
                            }
                        }
                    },
                    _ => (event::Status::Ignored, None),
                },
                _ => (event::Status::Ignored, None),
            }
        }

        fn draw(
            &self,
            _interaction: &Interaction,
            _theme: &Theme,
            bounds: Rectangle,
            cursor: Cursor,
        ) -> Vec<Geometry> {
            let center = Vector::new(bounds.width / 2.0, bounds.height / 2.0);

            let life = self.life_cache.draw(bounds.size(), |frame| {
                let background = Path::rectangle(Point::ORIGIN, frame.size());
                frame.fill(&background, Color::from_rgb8(0x40, 0x44, 0x4B));

                frame.with_save(|frame| {
                    frame.translate(center);
                    frame.scale(self.scaling);
                    frame.translate(self.translation);
                    frame.scale(Cell::SIZE as f32);

                    let region = self.visible_region(frame.size());

                    for cell in region.cull(self.state.cells()) {
                        frame.fill_rectangle(
                            Point::new(cell.j as f32, cell.i as f32),
                            Size::UNIT,
                            Color::WHITE,
                        );
                    }
                });
            });

            let overlay = {
                let mut frame = Frame::new(bounds.size());

                let hovered_cell =
                    cursor.position_in(&bounds).map(|position| {
                        Cell::at(self.project(position, frame.size()))
                    });

                if let Some(cell) = hovered_cell {
                    frame.with_save(|frame| {
                        frame.translate(center);
                        frame.scale(self.scaling);
                        frame.translate(self.translation);
                        frame.scale(Cell::SIZE as f32);

                        frame.fill_rectangle(
                            Point::new(cell.j as f32, cell.i as f32),
                            Size::UNIT,
                            Color {
                                a: 0.5,
                                ..Color::BLACK
                            },
                        );
                    });
                }

                let text = Text {
                    color: Color::WHITE,
                    size: 14.0,
                    position: Point::new(frame.width(), frame.height()),
                    horizontal_alignment: alignment::Horizontal::Right,
                    vertical_alignment: alignment::Vertical::Bottom,
                    ..Text::default()
                };

                if let Some(cell) = hovered_cell {
                    frame.fill_text(Text {
                        content: format!("({}, {})", cell.j, cell.i),
                        position: text.position - Vector::new(0.0, 16.0),
                        ..text
                    });
                }

                let cell_count = self.state.cell_count();

                frame.fill_text(Text {
                    content: format!(
                        "{} cell{} @ {:?} ({})",
                        cell_count,
                        if cell_count == 1 { "" } else { "s" },
                        self.last_tick_duration,
                        self.last_queued_ticks
                    ),
                    ..text
                });

                frame.into_geometry()
            };

            if self.scaling < 0.2 || !self.show_lines {
                vec![life, overlay]
            } else {
                let grid = self.grid_cache.draw(bounds.size(), |frame| {
                    frame.translate(center);
                    frame.scale(self.scaling);
                    frame.translate(self.translation);
                    frame.scale(Cell::SIZE as f32);

                    let region = self.visible_region(frame.size());
                    let rows = region.rows();
                    let columns = region.columns();
                    let (total_rows, total_columns) =
                        (rows.clone().count(), columns.clone().count());
                    let width = 2.0 / Cell::SIZE as f32;
                    let color = Color::from_rgb8(70, 74, 83);

                    frame.translate(Vector::new(-width / 2.0, -width / 2.0));

                    for row in region.rows() {
                        frame.fill_rectangle(
                            Point::new(*columns.start() as f32, row as f32),
                            Size::new(total_columns as f32, width),
                            color,
                        );
                    }

                    for column in region.columns() {
                        frame.fill_rectangle(
                            Point::new(column as f32, *rows.start() as f32),
                            Size::new(width, total_rows as f32),
                            color,
                        );
                    }
                });

                vec![life, grid, overlay]
            }
        }

        fn mouse_interaction(
            &self,
            interaction: &Interaction,
            bounds: Rectangle,
            cursor: Cursor,
        ) -> mouse::Interaction {
            match interaction {
                Interaction::Drawing => mouse::Interaction::Crosshair,
                Interaction::Erasing => mouse::Interaction::Crosshair,
                Interaction::Panning { .. } => mouse::Interaction::Grabbing,
                Interaction::None if cursor.is_over(&bounds) => {
                    mouse::Interaction::Crosshair
                }
                _ => mouse::Interaction::default(),
            }
        }
    }

    #[derive(Default)]
    struct State {
        life: Life,
        births: FxHashSet<Cell>,
        is_ticking: bool,
    }

    impl State {
        pub fn with_life(life: Life) -> Self {
            Self {
                life,
                ..Self::default()
            }
        }

        fn cell_count(&self) -> usize {
            self.life.len() + self.births.len()
        }

        fn contains(&self, cell: &Cell) -> bool {
            self.life.contains(cell) || self.births.contains(cell)
        }

        fn cells(&self) -> impl Iterator<Item = &Cell> {
            self.life.iter().chain(self.births.iter())
        }

        fn populate(&mut self, cell: Cell) {
            if self.is_ticking {
                self.births.insert(cell);
            } else {
                self.life.populate(cell);
            }
        }

        fn unpopulate(&mut self, cell: &Cell) {
            if self.is_ticking {
                let _ = self.births.remove(cell);
            } else {
                self.life.unpopulate(cell);
            }
        }

        fn update(&mut self, mut life: Life) {
            self.births.drain().for_each(|cell| life.populate(cell));

            self.life = life;
            self.is_ticking = false;
        }

        fn tick(
            &mut self,
            amount: usize,
        ) -> Option<impl Future<Output = Result<Life, TickError>>> {
            if self.is_ticking {
                return None;
            }

            self.is_ticking = true;

            let mut life = self.life.clone();

            Some(async move {
                tokio::task::spawn_blocking(move || {
                    for _ in 0..amount {
                        life.tick();
                    }

                    life
                })
                .await
                .map_err(|_| TickError::JoinFailed)
            })
        }
    }

    #[derive(Clone, Default)]
    pub struct Life {
        cells: FxHashSet<Cell>,
    }

    impl Life {
        fn len(&self) -> usize {
            self.cells.len()
        }

        fn contains(&self, cell: &Cell) -> bool {
            self.cells.contains(cell)
        }

        fn populate(&mut self, cell: Cell) {
            self.cells.insert(cell);
        }

        fn unpopulate(&mut self, cell: &Cell) {
            let _ = self.cells.remove(cell);
        }

        fn tick(&mut self) {
            let mut adjacent_life = FxHashMap::default();

            for cell in &self.cells {
                let _ = adjacent_life.entry(*cell).or_insert(0);

                for neighbor in Cell::neighbors(*cell) {
                    let amount = adjacent_life.entry(neighbor).or_insert(0);

                    *amount += 1;
                }
            }

            for (cell, amount) in adjacent_life.iter() {
                match amount {
                    2 => {}
                    3 => {
                        let _ = self.cells.insert(*cell);
                    }
                    _ => {
                        let _ = self.cells.remove(cell);
                    }
                }
            }
        }

        pub fn iter(&self) -> impl Iterator<Item = &Cell> {
            self.cells.iter()
        }
    }

    impl std::iter::FromIterator<Cell> for Life {
        fn from_iter<I: IntoIterator<Item = Cell>>(iter: I) -> Self {
            Life {
                cells: iter.into_iter().collect(),
            }
        }
    }

    impl std::fmt::Debug for Life {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Life")
                .field("cells", &self.cells.len())
                .finish()
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Cell {
        i: isize,
        j: isize,
    }

    impl Cell {
        const SIZE: usize = 20;

        fn at(position: Point) -> Cell {
            let i = (position.y / Cell::SIZE as f32).ceil() as isize;
            let j = (position.x / Cell::SIZE as f32).ceil() as isize;

            Cell {
                i: i.saturating_sub(1),
                j: j.saturating_sub(1),
            }
        }

        fn cluster(cell: Cell) -> impl Iterator<Item = Cell> {
            use itertools::Itertools;

            let rows = cell.i.saturating_sub(1)..=cell.i.saturating_add(1);
            let columns = cell.j.saturating_sub(1)..=cell.j.saturating_add(1);

            rows.cartesian_product(columns).map(|(i, j)| Cell { i, j })
        }

        fn neighbors(cell: Cell) -> impl Iterator<Item = Cell> {
            Cell::cluster(cell).filter(move |candidate| *candidate != cell)
        }
    }

    pub struct Region {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    }

    impl Region {
        fn rows(&self) -> RangeInclusive<isize> {
            let first_row = (self.y / Cell::SIZE as f32).floor() as isize;

            let visible_rows =
                (self.height / Cell::SIZE as f32).ceil() as isize;

            first_row..=first_row + visible_rows
        }

        fn columns(&self) -> RangeInclusive<isize> {
            let first_column = (self.x / Cell::SIZE as f32).floor() as isize;

            let visible_columns =
                (self.width / Cell::SIZE as f32).ceil() as isize;

            first_column..=first_column + visible_columns
        }

        fn cull<'a>(
            &self,
            cells: impl Iterator<Item = &'a Cell>,
        ) -> impl Iterator<Item = &'a Cell> {
            let rows = self.rows();
            let columns = self.columns();

            cells.filter(move |cell| {
                rows.contains(&cell.i) && columns.contains(&cell.j)
            })
        }
    }

    pub enum Interaction {
        None,
        Drawing,
        Erasing,
        Panning { translation: Vector, start: Point },
    }

    impl Default for Interaction {
        fn default() -> Self {
            Self::None
        }
    }