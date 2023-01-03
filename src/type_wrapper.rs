use usbd_human_interface_device::page::{Keyboard, Consumer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardWrapper (Keyboard);

impl std::fmt::Display for KeyboardWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            Keyboard::NoEventIndicated => write!(f, "None"),
            Keyboard::ErrorRollOver => write!(f, "Error Roll Over"),
            Keyboard::POSTFail => write!(f, "POST Fail"),
            Keyboard::ErrorUndefine => write!(f, "Error Undefine"),
            Keyboard::A => write!(f, "A"),
            Keyboard::B => write!(f, "B"),
            Keyboard::C => write!(f, "C"),
            Keyboard::D => write!(f, "D"),
            Keyboard::E => write!(f, "E"),
            Keyboard::F => write!(f, "F"),
            Keyboard::G => write!(f, "G"),
            Keyboard::H => write!(f, "H"),
            Keyboard::I => write!(f, "I"),
            Keyboard::J => write!(f, "J"),
            Keyboard::K => write!(f, "K"),
            Keyboard::L => write!(f, "L"),
            Keyboard::M => write!(f, "M"),
            Keyboard::N => write!(f, "N"),
            Keyboard::O => write!(f, "O"),
            Keyboard::P => write!(f, "P"),
            Keyboard::Q => write!(f, "Q"),
            Keyboard::R => write!(f, "R"),
            Keyboard::S => write!(f, "S"),
            Keyboard::T => write!(f, "T"),
            Keyboard::U => write!(f, "U"),
            Keyboard::V => write!(f, "V"),
            Keyboard::W => write!(f, "W"),
            Keyboard::X => write!(f, "X"),
            Keyboard::Y => write!(f, "Y"),
            Keyboard::Z => write!(f, "Z"),
            Keyboard::Keyboard1 => write!(f, "1"),
            Keyboard::Keyboard2 => write!(f, "2"),
            Keyboard::Keyboard3 => write!(f, "3"),
            Keyboard::Keyboard4 => write!(f, "4"),
            Keyboard::Keyboard5 => write!(f, "5"),
            Keyboard::Keyboard6 => write!(f, "6"),
            Keyboard::Keyboard7 => write!(f, "7"),
            Keyboard::Keyboard8 => write!(f, "8"),
            Keyboard::Keyboard9 => write!(f, "9"),
            Keyboard::Keyboard0 => write!(f, "0"),
            Keyboard::ReturnEnter => write!(f, "Return"),
            Keyboard::Escape => write!(f, "Escape"),
            Keyboard::DeleteBackspace => write!(f, "Backspace"),
            Keyboard::Tab => write!(f, "Tab"),
            Keyboard::Space => write!(f, "Space"),
            Keyboard::Minus => write!(f, "Minus"),
            Keyboard::Equal => write!(f, "Equal"),
            Keyboard::LeftBrace => write!(f, "Left Brace"),
            Keyboard::RightBrace => write!(f, "Right Brace"),
            Keyboard::Backslash => write!(f, "Backslash"),
            Keyboard::NonUSHash => write!(f, "Non U SHash"),
            Keyboard::Semicolon => write!(f, "Semicolon"),
            Keyboard::Apostrophe => write!(f, "Apostrophe"),
            Keyboard::Grave => write!(f, "Grave"),
            Keyboard::Comma => write!(f, "Comma"),
            Keyboard::Dot => write!(f, "Dot"),
            Keyboard::ForwardSlash => write!(f, "Forward Slash"),
            Keyboard::CapsLock => write!(f, "Caps Lock"),
            Keyboard::F1 => write!(f, "F1"),
            Keyboard::F2 => write!(f, "F2"),
            Keyboard::F3 => write!(f, "F3"),
            Keyboard::F4 => write!(f, "F4"),
            Keyboard::F5 => write!(f, "F5"),
            Keyboard::F6 => write!(f, "F6"),
            Keyboard::F7 => write!(f, "F7"),
            Keyboard::F8 => write!(f, "F8"),
            Keyboard::F9 => write!(f, "F9"),
            Keyboard::F10 => write!(f, "F10"),
            Keyboard::F11 => write!(f, "F11"),
            Keyboard::F12 => write!(f, "F12"),
            Keyboard::PrintScreen => write!(f, "Print Screen"),
            Keyboard::ScrollLock => write!(f, "Scroll Lock"),
            Keyboard::Pause => write!(f, "Pause"),
            Keyboard::Insert => write!(f, "Insert"),
            Keyboard::Home => write!(f, "Home"),
            Keyboard::PageUp => write!(f, "Page Up"),
            Keyboard::DeleteForward => write!(f, "Delete"),
            Keyboard::End => write!(f, "End"),
            Keyboard::PageDown => write!(f, "Page Down"),
            Keyboard::RightArrow => write!(f, "Right Arrow"),
            Keyboard::LeftArrow => write!(f, "Left Arrow"),
            Keyboard::DownArrow => write!(f, "Down Arrow"),
            Keyboard::UpArrow => write!(f, "Up Arrow"),
            Keyboard::KeypadNumLockAndClear => write!(f, "Keypad Num Lock And Clear"),
            Keyboard::KeypadDivide => write!(f, "Keypad Divide"),
            Keyboard::KeypadMultiply => write!(f, "Keypad Multiply"),
            Keyboard::KeypadSubtract => write!(f, "Keypad Subtract"),
            Keyboard::KeypadAdd => write!(f, "Keypad Add"),
            Keyboard::KeypadEnter => write!(f, "Keypad Enter"),
            Keyboard::Keypad1 => write!(f, "Keypad 1"),
            Keyboard::Keypad2 => write!(f, "Keypad 2"),
            Keyboard::Keypad3 => write!(f, "Keypad 3"),
            Keyboard::Keypad4 => write!(f, "Keypad 4"),
            Keyboard::Keypad5 => write!(f, "Keypad 5"),
            Keyboard::Keypad6 => write!(f, "Keypad 6"),
            Keyboard::Keypad7 => write!(f, "Keypad 7"),
            Keyboard::Keypad8 => write!(f, "Keypad 8"),
            Keyboard::Keypad9 => write!(f, "Keypad 9"),
            Keyboard::Keypad0 => write!(f, "Keypad 0"),
            Keyboard::KeypadDot => write!(f, "Keypad Dot"),
            Keyboard::NonUSBackslash => write!(f, "Non US Backslash"),
            Keyboard::Application => write!(f, "Application"),
            Keyboard::Power => write!(f, "Power"),
            Keyboard::KeypadEqual => write!(f, "Keypad Equal"),
            Keyboard::F13 => write!(f, "F13"),
            Keyboard::F14 => write!(f, "F14"),
            Keyboard::F15 => write!(f, "F15"),
            Keyboard::F16 => write!(f, "F16"),
            Keyboard::F17 => write!(f, "F17"),
            Keyboard::F18 => write!(f, "F18"),
            Keyboard::F19 => write!(f, "F19"),
            Keyboard::F20 => write!(f, "F20"),
            Keyboard::F21 => write!(f, "F21"),
            Keyboard::F22 => write!(f, "F22"),
            Keyboard::F23 => write!(f, "F23"),
            Keyboard::F24 => write!(f, "F24"),
            Keyboard::Execute => write!(f, "Execute"),
            Keyboard::Help => write!(f, "Help"),
            Keyboard::Menu => write!(f, "Menu"),
            Keyboard::Select => write!(f, "Select"),
            Keyboard::Stop => write!(f, "Stop"),
            Keyboard::Again => write!(f, "Again"),
            Keyboard::Undo => write!(f, "Undo"),
            Keyboard::Cut => write!(f, "Cut"),
            Keyboard::Copy => write!(f, "Copy"),
            Keyboard::Paste => write!(f, "Paste"),
            Keyboard::Find => write!(f, "Find"),
            Keyboard::Mute => write!(f, "Mute"),
            Keyboard::VolumeUp => write!(f, "Volume Up"),
            Keyboard::VolumeDown => write!(f, "Volume Down"),
            Keyboard::LockingCapsLock => write!(f, "Locking Caps Lock"),
            Keyboard::LockingNumLock => write!(f, "Locking Num Lock"),
            Keyboard::LockingScrollLock => write!(f, "Locking Scroll Lock"),
            Keyboard::KeypadComma => write!(f, "Keypad Comma"),
            Keyboard::KeypadEqualSign => write!(f, "Keypad Equal Sign"),
            Keyboard::Kanji1 => write!(f, "Kanji 1"),
            Keyboard::Kanji2 => write!(f, "Kanji 2"),
            Keyboard::Kanji3 => write!(f, "Kanji 3"),
            Keyboard::Kanji4 => write!(f, "Kanji 4"),
            Keyboard::Kanji5 => write!(f, "Kanji 5"),
            Keyboard::Kanji6 => write!(f, "Kanji 6"),
            Keyboard::Kanji7 => write!(f, "Kanji 7"),
            Keyboard::Kanji8 => write!(f, "Kanji 8"),
            Keyboard::Kanji9 => write!(f, "Kanji 9"),
            Keyboard::LANG1 => write!(f, "LANG 1"),
            Keyboard::LANG2 => write!(f, "LANG 2"),
            Keyboard::LANG3 => write!(f, "LANG 3"),
            Keyboard::LANG4 => write!(f, "LANG 4"),
            Keyboard::LANG5 => write!(f, "LANG 5"),
            Keyboard::LANG6 => write!(f, "LANG 6"),
            Keyboard::LANG7 => write!(f, "LANG 7"),
            Keyboard::LANG8 => write!(f, "LANG 8"),
            Keyboard::LANG9 => write!(f, "LANG 9"),
            Keyboard::AlternateErase => write!(f, "Alternate Erase"),
            Keyboard::SysReqAttention => write!(f, "Sys Req Attention"),
            Keyboard::Cancel => write!(f, "Cancel"),
            Keyboard::Clear => write!(f, "Clear"),
            Keyboard::Prior => write!(f, "Prior"),
            Keyboard::Return => write!(f, "Return"),
            Keyboard::Separator => write!(f, "Separator"),
            Keyboard::Out => write!(f, "Out"),
            Keyboard::Oper => write!(f, "Oper"),
            Keyboard::ClearAgain => write!(f, "Clear Again"),
            Keyboard::CrSelProps => write!(f, "Cr Sel Props"),
            Keyboard::ExSel => write!(f, "Ex Sel"),
            Keyboard::LeftControl => write!(f, "Left Control"),
            Keyboard::LeftShift => write!(f, "Left Shift"),
            Keyboard::LeftAlt => write!(f, "Left Alt"),
            Keyboard::LeftGUI => write!(f, "Left GUI"),
            Keyboard::RightControl => write!(f, "Right Control"),
            Keyboard::RightShift => write!(f, "Right Shift"),
            Keyboard::RightAlt => write!(f, "Right Alt"),
            Keyboard::RightGUI => write!(f, "Right GUI"),
        }
    }
}

impl From<Keyboard> for KeyboardWrapper {
    fn from(key: Keyboard) -> KeyboardWrapper {
        KeyboardWrapper(key)
    }
}

impl From<KeyboardWrapper> for Keyboard {
    fn from(wrapper: KeyboardWrapper) -> Keyboard {
        wrapper.0
    }
}

impl KeyboardWrapper {
    pub fn get_chord_string(keys: &Vec<Keyboard> ) -> String {
        let mut chord = String::new();
        if keys.contains(&Keyboard::LeftControl) || keys.contains(&Keyboard::RightControl) {
            chord.push_str("Ctrl + ");
        }

        if keys.contains(&Keyboard::LeftAlt) || keys.contains(&Keyboard::RightAlt) {
            chord.push_str("Alt + ");
        }

        let caps = if keys.contains(&Keyboard::LeftShift) || keys.contains(&Keyboard::RightShift) {
            chord.push_str("Shift + ");
            true
        } else {
            false
        };

        if keys.contains(&Keyboard::LeftGUI) || keys.contains(&Keyboard::RightGUI) {
            chord.push_str("GUI + ");
        }
        
        for key in keys {
            if let Ok(c) = KeyboardWrapper::from(*key).get_char(caps) {
                chord.push(c);
            }
        }
        chord
    }

    pub fn get_char(&self, caps: bool) -> Result<char, ()> {
        Ok(if caps {
            match self.0 {
                Keyboard::A => 'A',
                Keyboard::B => 'B',
                Keyboard::C => 'C',
                Keyboard::D => 'D',
                Keyboard::E => 'E',
                Keyboard::F => 'F',
                Keyboard::G => 'G',
                Keyboard::H => 'H',
                Keyboard::I => 'I',
                Keyboard::J => 'J',
                Keyboard::K => 'K',
                Keyboard::L => 'L',
                Keyboard::M => 'M',
                Keyboard::N => 'N',
                Keyboard::O => 'O',
                Keyboard::P => 'P',
                Keyboard::Q => 'Q',
                Keyboard::R => 'R',
                Keyboard::S => 'S',
                Keyboard::T => 'T',
                Keyboard::U => 'U',
                Keyboard::V => 'V',
                Keyboard::W => 'W',
                Keyboard::X => 'X',
                Keyboard::Y => 'Y',
                Keyboard::Z => 'Z',
                Keyboard::Grave => '~',
                Keyboard::Keyboard1 => '!',
                Keyboard::Keyboard2 => '@',
                Keyboard::Keyboard3 => '#',
                Keyboard::Keyboard4 => '$',
                Keyboard::Keyboard5 => '%',
                Keyboard::Keyboard6 => '^',
                Keyboard::Keyboard7 => '&',
                Keyboard::Keyboard8 => '*',
                Keyboard::Keyboard9 => '(',
                Keyboard::Keyboard0 => ')',
                Keyboard::Minus => '_',
                Keyboard::Equal => '+',
                Keyboard::LeftBrace => '{',
                Keyboard::RightBrace => '}',
                Keyboard::Backslash => '|',
                Keyboard::Semicolon => ':',
                Keyboard::Apostrophe => '"',
                Keyboard::Return => '\n',
                Keyboard::Comma => '<',
                Keyboard::Separator => '>',
                Keyboard::Dot => '?',
                Keyboard::Space => ' ',
                Keyboard::Tab => '\t',
                
                _ => return Err(()),
            }
        } else {
            match self.0 {
                Keyboard::A => 'a',
                Keyboard::B => 'b',
                Keyboard::C => 'c',
                Keyboard::D => 'd',
                Keyboard::E => 'e',
                Keyboard::F => 'f',
                Keyboard::G => 'g',
                Keyboard::H => 'h',
                Keyboard::I => 'i',
                Keyboard::J => 'j',
                Keyboard::K => 'k',
                Keyboard::L => 'l',
                Keyboard::M => 'm',
                Keyboard::N => 'n',
                Keyboard::O => 'o',
                Keyboard::P => 'p',
                Keyboard::Q => 'q',
                Keyboard::R => 'r',
                Keyboard::S => 's',
                Keyboard::T => 't',
                Keyboard::U => 'u',
                Keyboard::V => 'v',
                Keyboard::W => 'w',
                Keyboard::X => 'x',
                Keyboard::Y => 'y',
                Keyboard::Z => 'z',
                Keyboard::Grave => '`',
                Keyboard::Keyboard1 => '1',
                Keyboard::Keyboard2 => '2',
                Keyboard::Keyboard3 => '3',
                Keyboard::Keyboard4 => '4',
                Keyboard::Keyboard5 => '5',
                Keyboard::Keyboard6 => '6',
                Keyboard::Keyboard7 => '7',
                Keyboard::Keyboard8 => '8',
                Keyboard::Keyboard9 => '9',
                Keyboard::Keyboard0 => '0',
                Keyboard::Minus => '-',
                Keyboard::Equal => '=',
                Keyboard::LeftBrace => '[',
                Keyboard::RightBrace => ']',
                Keyboard::Backslash => '\\',
                Keyboard::Semicolon => ';',
                Keyboard::Apostrophe => '\'',
                Keyboard::Return => '\n',
                Keyboard::Comma => ',',
                Keyboard::Separator => '.',
                Keyboard::Dot => '/',
                Keyboard::Space => ' ',
                Keyboard::Tab => '\t',
                
                _ => return Err(()),
            }
        })
    }

    pub fn from_char(char: char) -> (Keyboard, Option<bool>) {
        match char {
            'A' => (Keyboard::A, Some(true)),
            'B' => (Keyboard::B, Some(true)),
            'C' => (Keyboard::C, Some(true)),
            'D' => (Keyboard::D, Some(true)),
            'E' => (Keyboard::E, Some(true)),
            'F' => (Keyboard::F, Some(true)),
            'G' => (Keyboard::G, Some(true)),
            'H' => (Keyboard::H, Some(true)),
            'I' => (Keyboard::I, Some(true)),
            'J' => (Keyboard::J, Some(true)),
            'K' => (Keyboard::K, Some(true)),
            'L' => (Keyboard::L, Some(true)),
            'M' => (Keyboard::M, Some(true)),
            'N' => (Keyboard::N, Some(true)),
            'O' => (Keyboard::O, Some(true)),
            'P' => (Keyboard::P, Some(true)),
            'Q' => (Keyboard::Q, Some(true)),
            'R' => (Keyboard::R, Some(true)),
            'S' => (Keyboard::S, Some(true)),
            'T' => (Keyboard::T, Some(true)),
            'U' => (Keyboard::U, Some(true)),
            'V' => (Keyboard::V, Some(true)),
            'W' => (Keyboard::W, Some(true)),
            'X' => (Keyboard::X, Some(true)),
            'Y' => (Keyboard::Y, Some(true)),
            'Z' => (Keyboard::Z, Some(true)),
            '~' => (Keyboard::Grave, Some(true)),
            '!' => (Keyboard::Keyboard1, Some(true)),
            '@' => (Keyboard::Keyboard2, Some(true)),
            '#' => (Keyboard::Keyboard3, Some(true)),
            '$' => (Keyboard::Keyboard4, Some(true)),
            '%' => (Keyboard::Keyboard5, Some(true)),
            '^' => (Keyboard::Keyboard6, Some(true)),
            '&' => (Keyboard::Keyboard7, Some(true)),
            '*' => (Keyboard::Keyboard8, Some(true)),
            '(' => (Keyboard::Keyboard9, Some(true)),
            ')' => (Keyboard::Keyboard0, Some(true)),
            '_' => (Keyboard::Minus, Some(true)),
            '+' => (Keyboard::Equal, Some(true)),
            '{' => (Keyboard::LeftBrace, Some(true)),
            '}' => (Keyboard::RightBrace, Some(true)),
            '|' => (Keyboard::Backslash, Some(true)),
            ':' => (Keyboard::Semicolon, Some(true)),
            '"' => (Keyboard::Apostrophe, Some(true)),
            '<' => (Keyboard::Comma, Some(true)),
            '>' => (Keyboard::Separator, Some(true)),
            '\n' => (Keyboard::Return, None),
            '?' => (Keyboard::Dot, None),
            ' ' => (Keyboard::Space, None),
            '\t' => (Keyboard::Tab, None),

            'a' => (Keyboard::A, Some(false)),
            'b' => (Keyboard::B, Some(false)),
            'c' => (Keyboard::C, Some(false)),
            'd' => (Keyboard::D, Some(false)),
            'e' => (Keyboard::E, Some(false)),
            'f' => (Keyboard::F, Some(false)),
            'g' => (Keyboard::G, Some(false)),
            'h' => (Keyboard::H, Some(false)),
            'i' => (Keyboard::I, Some(false)),
            'j' => (Keyboard::J, Some(false)),
            'k' => (Keyboard::K, Some(false)),
            'l' => (Keyboard::L, Some(false)),
            'm' => (Keyboard::M, Some(false)),
            'n' => (Keyboard::N, Some(false)),
            'o' => (Keyboard::O, Some(false)),
            'p' => (Keyboard::P, Some(false)),
            'q' => (Keyboard::Q, Some(false)),
            'r' => (Keyboard::R, Some(false)),
            's' => (Keyboard::S, Some(false)),
            't' => (Keyboard::T, Some(false)),
            'u' => (Keyboard::U, Some(false)),
            'v' => (Keyboard::V, Some(false)),
            'w' => (Keyboard::W, Some(false)),
            'x' => (Keyboard::X, Some(false)),
            'y' => (Keyboard::Y, Some(false)),
            'z' => (Keyboard::Z, Some(false)),
            '`' => (Keyboard::Grave, Some(false)),
            '1' => (Keyboard::Keyboard1, Some(false)),
            '2' => (Keyboard::Keyboard2, Some(false)),
            '3' => (Keyboard::Keyboard3, Some(false)),
            '4' => (Keyboard::Keyboard4, Some(false)),
            '5' => (Keyboard::Keyboard5, Some(false)),
            '6' => (Keyboard::Keyboard6, Some(false)),
            '7' => (Keyboard::Keyboard7, Some(false)),
            '8' => (Keyboard::Keyboard8, Some(false)),
            '9' => (Keyboard::Keyboard9, Some(false)),
            '0' => (Keyboard::Keyboard0, Some(false)),
            '-' => (Keyboard::Minus, Some(false)),
            '=' => (Keyboard::Equal, Some(false)),
            '[' => (Keyboard::LeftBrace, Some(false)),
            ']' => (Keyboard::RightBrace, Some(false)),
            '\\' => (Keyboard::Backslash, Some(false)),
            ';' => (Keyboard::Semicolon, Some(false)),
            '\'' => (Keyboard::Apostrophe, Some(false)),
            ',' => (Keyboard::Comma, Some(false)),
            '.' => (Keyboard::Separator, Some(false)),
            '/' => (Keyboard::Dot, Some(false)),
            
            _ => unreachable!(),
        }
    }

    pub const KEYS: [KeyboardWrapper; 110] = [
        KeyboardWrapper(Keyboard::NoEventIndicated),
        KeyboardWrapper(Keyboard::A),
        KeyboardWrapper(Keyboard::B),
        KeyboardWrapper(Keyboard::C),
        KeyboardWrapper(Keyboard::D),
        KeyboardWrapper(Keyboard::E),
        KeyboardWrapper(Keyboard::F),
        KeyboardWrapper(Keyboard::G),
        KeyboardWrapper(Keyboard::H),
        KeyboardWrapper(Keyboard::I),
        KeyboardWrapper(Keyboard::J),
        KeyboardWrapper(Keyboard::K),
        KeyboardWrapper(Keyboard::L),
        KeyboardWrapper(Keyboard::M),
        KeyboardWrapper(Keyboard::N),
        KeyboardWrapper(Keyboard::O),
        KeyboardWrapper(Keyboard::P),
        KeyboardWrapper(Keyboard::Q),
        KeyboardWrapper(Keyboard::R),
        KeyboardWrapper(Keyboard::S),
        KeyboardWrapper(Keyboard::T),
        KeyboardWrapper(Keyboard::U),
        KeyboardWrapper(Keyboard::V),
        KeyboardWrapper(Keyboard::W),
        KeyboardWrapper(Keyboard::X),
        KeyboardWrapper(Keyboard::Y),
        KeyboardWrapper(Keyboard::Z),
        KeyboardWrapper(Keyboard::ReturnEnter),
        KeyboardWrapper(Keyboard::Escape),
        KeyboardWrapper(Keyboard::DeleteBackspace),
        KeyboardWrapper(Keyboard::Tab),
        KeyboardWrapper(Keyboard::Space),
        KeyboardWrapper(Keyboard::Minus),
        KeyboardWrapper(Keyboard::Equal),
        KeyboardWrapper(Keyboard::LeftBrace),
        KeyboardWrapper(Keyboard::RightBrace),
        KeyboardWrapper(Keyboard::Backslash),
        KeyboardWrapper(Keyboard::Semicolon),
        KeyboardWrapper(Keyboard::Apostrophe),
        KeyboardWrapper(Keyboard::Grave),
        KeyboardWrapper(Keyboard::Comma),
        KeyboardWrapper(Keyboard::Dot),
        KeyboardWrapper(Keyboard::ForwardSlash),
        KeyboardWrapper(Keyboard::CapsLock),
        KeyboardWrapper(Keyboard::F1),
        KeyboardWrapper(Keyboard::F2),
        KeyboardWrapper(Keyboard::F3),
        KeyboardWrapper(Keyboard::F4),
        KeyboardWrapper(Keyboard::F5),
        KeyboardWrapper(Keyboard::F6),
        KeyboardWrapper(Keyboard::F7),
        KeyboardWrapper(Keyboard::F8),
        KeyboardWrapper(Keyboard::F9),
        KeyboardWrapper(Keyboard::F10),
        KeyboardWrapper(Keyboard::F11),
        KeyboardWrapper(Keyboard::F12),
        KeyboardWrapper(Keyboard::PrintScreen),
        KeyboardWrapper(Keyboard::ScrollLock),
        KeyboardWrapper(Keyboard::Pause),
        KeyboardWrapper(Keyboard::Insert),
        KeyboardWrapper(Keyboard::Home),
        KeyboardWrapper(Keyboard::PageUp),
        KeyboardWrapper(Keyboard::DeleteForward),
        KeyboardWrapper(Keyboard::End),
        KeyboardWrapper(Keyboard::PageDown),
        KeyboardWrapper(Keyboard::RightArrow),
        KeyboardWrapper(Keyboard::LeftArrow),
        KeyboardWrapper(Keyboard::DownArrow),
        KeyboardWrapper(Keyboard::UpArrow),
        KeyboardWrapper(Keyboard::KeypadNumLockAndClear),
        KeyboardWrapper(Keyboard::KeypadDivide),
        KeyboardWrapper(Keyboard::KeypadMultiply),
        KeyboardWrapper(Keyboard::KeypadSubtract),
        KeyboardWrapper(Keyboard::KeypadAdd),
        KeyboardWrapper(Keyboard::KeypadEnter),
        KeyboardWrapper(Keyboard::Keypad1),
        KeyboardWrapper(Keyboard::Keypad2),
        KeyboardWrapper(Keyboard::Keypad3),
        KeyboardWrapper(Keyboard::Keypad4),
        KeyboardWrapper(Keyboard::Keypad5),
        KeyboardWrapper(Keyboard::Keypad6),
        KeyboardWrapper(Keyboard::Keypad7),
        KeyboardWrapper(Keyboard::Keypad8),
        KeyboardWrapper(Keyboard::Keypad9),
        KeyboardWrapper(Keyboard::Keypad0),
        KeyboardWrapper(Keyboard::KeypadDot),
        KeyboardWrapper(Keyboard::KeypadEqual),
        KeyboardWrapper(Keyboard::F13),
        KeyboardWrapper(Keyboard::F14),
        KeyboardWrapper(Keyboard::F15),
        KeyboardWrapper(Keyboard::F16),
        KeyboardWrapper(Keyboard::F17),
        KeyboardWrapper(Keyboard::F18),
        KeyboardWrapper(Keyboard::F19),
        KeyboardWrapper(Keyboard::F20),
        KeyboardWrapper(Keyboard::F21),
        KeyboardWrapper(Keyboard::F22),
        KeyboardWrapper(Keyboard::F23),
        KeyboardWrapper(Keyboard::F24),
        // KeyboardWrapper(Keyboard::Execute),
        // KeyboardWrapper(Keyboard::Help),
        // KeyboardWrapper(Keyboard::Menu),
        // KeyboardWrapper(Keyboard::Select),
        // KeyboardWrapper(Keyboard::Stop),
        // KeyboardWrapper(Keyboard::Again),
        // KeyboardWrapper(Keyboard::Undo),
        // KeyboardWrapper(Keyboard::Cut),
        // KeyboardWrapper(Keyboard::Copy),
        // KeyboardWrapper(Keyboard::Paste),
        // KeyboardWrapper(Keyboard::Find),
        // KeyboardWrapper(Keyboard::Mute),
        KeyboardWrapper(Keyboard::VolumeUp),
        KeyboardWrapper(Keyboard::VolumeDown),
        // KeyboardWrapper(Keyboard::LockingCapsLock),
        // KeyboardWrapper(Keyboard::LockingNumLock),
        // KeyboardWrapper(Keyboard::LockingScrollLock),
        KeyboardWrapper(Keyboard::KeypadComma),
        // KeyboardWrapper(Keyboard::KeypadEqualSign),
        // KeyboardWrapper(Keyboard::Kanji1),
        // KeyboardWrapper(Keyboard::Kanji2),
        // KeyboardWrapper(Keyboard::Kanji3),
        // KeyboardWrapper(Keyboard::Kanji4),
        // KeyboardWrapper(Keyboard::Kanji5),
        // KeyboardWrapper(Keyboard::Kanji6),
        // KeyboardWrapper(Keyboard::Kanji7),
        // KeyboardWrapper(Keyboard::Kanji8),
        // KeyboardWrapper(Keyboard::Kanji9),
        // KeyboardWrapper(Keyboard::LANG1),
        // KeyboardWrapper(Keyboard::LANG2),
        // KeyboardWrapper(Keyboard::LANG3),
        // KeyboardWrapper(Keyboard::LANG4),
        // KeyboardWrapper(Keyboard::LANG5),
        // KeyboardWrapper(Keyboard::LANG6),
        // KeyboardWrapper(Keyboard::LANG7),
        // KeyboardWrapper(Keyboard::LANG8),
        // KeyboardWrapper(Keyboard::LANG9),
        // KeyboardWrapper(Keyboard::AlternateErase),
        // KeyboardWrapper(Keyboard::SysReqAttention),
        // KeyboardWrapper(Keyboard::Cancel),
        // KeyboardWrapper(Keyboard::Clear),
        // KeyboardWrapper(Keyboard::Prior),
        // KeyboardWrapper(Keyboard::Return),
        // KeyboardWrapper(Keyboard::Separator),
        // KeyboardWrapper(Keyboard::Out),
        // KeyboardWrapper(Keyboard::Oper),
        // KeyboardWrapper(Keyboard::ClearAgain),
        // KeyboardWrapper(Keyboard::CrSelProps),
        // KeyboardWrapper(Keyboard::ExSel),
        KeyboardWrapper(Keyboard::LeftControl),
        KeyboardWrapper(Keyboard::LeftShift),
        KeyboardWrapper(Keyboard::LeftAlt),
        KeyboardWrapper(Keyboard::LeftGUI),
        KeyboardWrapper(Keyboard::RightControl),
        KeyboardWrapper(Keyboard::RightShift),
        KeyboardWrapper(Keyboard::RightAlt),
        KeyboardWrapper(Keyboard::RightGUI),
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConsumerWrapper(Consumer);

impl std::fmt::Display for ConsumerWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            Consumer::Unassigned => write!(f, "Unassigned"),
            Consumer::ConsumerControl => write!(f, "ConsumerControl"),
            Consumer::NumericKeyPad => write!(f, "NumericKeyPad"),
            Consumer::ProgrammableButtons => write!(f, "ProgrammableButtons"),
            Consumer::Microphone => write!(f, "Microphone"),
            Consumer::Headphone => write!(f, "Headphone"),
            Consumer::GraphicEqualizer => write!(f, "GraphicEqualizer"),
            Consumer::Plus10 => write!(f, "Plus10"),
            Consumer::Plus100 => write!(f, "Plus100"),
            Consumer::AmPm => write!(f, "AmPm"),
            Consumer::Power => write!(f, "Power"),
            Consumer::Reset => write!(f, "Reset"),
            Consumer::Sleep => write!(f, "Sleep"),
            Consumer::SleepAfter => write!(f, "SleepAfter"),
            Consumer::SleepMode => write!(f, "SleepMode"),
            Consumer::Illumination => write!(f, "Illumination"),
            Consumer::FunctionButtons => write!(f, "FunctionButtons"),
            Consumer::Menu => write!(f, "Menu"),
            Consumer::MenuPick => write!(f, "MenuPick"),
            Consumer::MenuUp => write!(f, "MenuUp"),
            Consumer::MenuDown => write!(f, "MenuDown"),
            Consumer::MenuLeft => write!(f, "MenuLeft"),
            Consumer::MenuRight => write!(f, "MenuRight"),
            Consumer::MenuEscape => write!(f, "MenuEscape"),
            Consumer::MenuValueIncrease => write!(f, "MenuValueIncrease"),
            Consumer::MenuValueDecrease => write!(f, "MenuValueDecrease"),
            Consumer::DataOnScreen => write!(f, "DataOnScreen"),
            Consumer::ClosedCaption => write!(f, "ClosedCaption"),
            Consumer::ClosedCaptionSelect => write!(f, "ClosedCaptionSelect"),
            Consumer::VcrTv => write!(f, "VcrTv"),
            Consumer::BroadcastMode => write!(f, "BroadcastMode"),
            Consumer::Snapshot => write!(f, "Snapshot"),
            Consumer::Still => write!(f, "Still"),
            Consumer::Selection => write!(f, "Selection"),
            Consumer::AssignSelection => write!(f, "AssignSelection"),
            Consumer::ModeStep => write!(f, "ModeStep"),
            Consumer::RecallLast => write!(f, "RecallLast"),
            Consumer::EnterChannel => write!(f, "EnterChannel"),
            Consumer::OrderMovie => write!(f, "OrderMovie"),
            Consumer::Channel => write!(f, "Channel"),
            Consumer::MediaSelection => write!(f, "MediaSelection"),
            Consumer::MediaSelectComputer => write!(f, "MediaSelectComputer"),
            Consumer::MediaSelectTV => write!(f, "MediaSelectTV"),
            Consumer::MediaSelectWWW => write!(f, "MediaSelectWWW"),
            Consumer::MediaSelectDVD => write!(f, "MediaSelectDVD"),
            Consumer::MediaSelectTelephone => write!(f, "MediaSelectTelephone"),
            Consumer::MediaSelectProgramGuide => write!(f, "MediaSelectProgramGuide"),
            Consumer::MediaSelectVideoPhone => write!(f, "MediaSelectVideoPhone"),
            Consumer::MediaSelectGames => write!(f, "MediaSelectGames"),
            Consumer::MediaSelectMessages => write!(f, "MediaSelectMessages"),
            Consumer::MediaSelectCD => write!(f, "MediaSelectCD"),
            Consumer::MediaSelectVCR => write!(f, "MediaSelectVCR"),
            Consumer::MediaSelectTuner => write!(f, "MediaSelectTuner"),
            Consumer::Quit => write!(f, "Quit"),
            Consumer::Help => write!(f, "Help"),
            Consumer::MediaSelectTape => write!(f, "MediaSelectTape"),
            Consumer::MediaSelectCable => write!(f, "MediaSelectCable"),
            Consumer::MediaSelectSatellite => write!(f, "MediaSelectSatellite"),
            Consumer::MediaSelectSecurity => write!(f, "MediaSelectSecurity"),
            Consumer::MediaSelectHome => write!(f, "MediaSelectHome"),
            Consumer::MediaSelectCall => write!(f, "MediaSelectCall"),
            Consumer::ChannelIncrement => write!(f, "ChannelIncrement"),
            Consumer::ChannelDecrement => write!(f, "ChannelDecrement"),
            Consumer::MediaSelectSAP => write!(f, "MediaSelectSAP"),
            Consumer::VCRPlus => write!(f, "VCRPlus"),
            Consumer::Once => write!(f, "Once"),
            Consumer::Daily => write!(f, "Daily"),
            Consumer::Weekly => write!(f, "Weekly"),
            Consumer::Monthly => write!(f, "Monthly"),
            Consumer::Play => write!(f, "Play"),
            Consumer::Pause => write!(f, "Pause"),
            Consumer::Record => write!(f, "Record"),
            Consumer::FastForward => write!(f, "Fast Forward"),
            Consumer::Rewind => write!(f, "Rewind"),
            Consumer::ScanNextTrack => write!(f, "ScanNextTrack"),
            Consumer::ScanPreviousTrack => write!(f, "ScanPreviousTrack"),
            Consumer::Stop => write!(f, "Stop"),
            Consumer::Eject => write!(f, "Eject"),
            Consumer::RandomPlay => write!(f, "RandomPlay"),
            Consumer::SelectDisc => write!(f, "SelectDisc"),
            Consumer::EnterDisc => write!(f, "EnterDisc"),
            Consumer::Repeat => write!(f, "Repeat"),
            Consumer::Tracking => write!(f, "Tracking"),
            Consumer::TrackNormal => write!(f, "TrackNormal"),
            Consumer::SlowTracking => write!(f, "SlowTracking"),
            Consumer::FrameForward => write!(f, "FrameForward"),
            Consumer::FrameBack => write!(f, "FrameBack"),
            Consumer::Mark => write!(f, "Mark"),
            Consumer::ClearMark => write!(f, "ClearMark"),
            Consumer::RepeatFromMark => write!(f, "RepeatFromMark"),
            Consumer::ReturnToMark => write!(f, "ReturnToMark"),
            Consumer::SearchMarkForward => write!(f, "SearchMarkForward"),
            Consumer::SearchMarkBackwards => write!(f, "SearchMarkBackwards"),
            Consumer::CounterReset => write!(f, "CounterReset"),
            Consumer::ShowCounter => write!(f, "ShowCounter"),
            Consumer::TrackingIncrement => write!(f, "TrackingIncrement"),
            Consumer::TrackingDecrement => write!(f, "TrackingDecrement"),
            Consumer::StopEject => write!(f, "StopEject"),
            Consumer::PlayPause => write!(f, "Play / Pause"),
            Consumer::PlaySkip => write!(f, "PlaySkip"),
            Consumer::Volume => write!(f, "Volume"),
            Consumer::Balance => write!(f, "Balance"),
            Consumer::Mute => write!(f, "Mute"),
            Consumer::Bass => write!(f, "Bass"),
            Consumer::Treble => write!(f, "Treble"),
            Consumer::BassBoost => write!(f, "BassBoost"),
            Consumer::SurroundMode => write!(f, "SurroundMode"),
            Consumer::Loudness => write!(f, "Loudness"),
            Consumer::MPX => write!(f, "MPX"),
            Consumer::VolumeIncrement => write!(f, "Volume Increment"),
            Consumer::VolumeDecrement => write!(f, "Volume Decrement"),
            Consumer::SpeedSelect => write!(f, "SpeedSelect"),
            Consumer::PlaybackSpeed => write!(f, "PlaybackSpeed"),
            Consumer::StandardPlay => write!(f, "StandardPlay"),
            Consumer::LongPlay => write!(f, "LongPlay"),
            Consumer::ExtendedPlay => write!(f, "ExtendedPlay"),
            Consumer::Slow => write!(f, "Slow"),
            Consumer::FanEnable => write!(f, "FanEnable"),
            Consumer::FanSpeed => write!(f, "FanSpeed"),
            Consumer::LightEnable => write!(f, "LightEnable"),
            Consumer::LightIlluminationLevel => write!(f, "LightIlluminationLevel"),
            Consumer::ClimateControlEnable => write!(f, "ClimateControlEnable"),
            Consumer::RoomTemperature => write!(f, "RoomTemperature"),
            Consumer::SecurityEnable => write!(f, "SecurityEnable"),
            Consumer::FireAlarm => write!(f, "FireAlarm"),
            Consumer::PoliceAlarm => write!(f, "PoliceAlarm"),
            Consumer::Proximity => write!(f, "Proximity"),
            Consumer::Motion => write!(f, "Motion"),
            Consumer::DuressAlarm => write!(f, "DuressAlarm"),
            Consumer::HoldupAlarm => write!(f, "HoldupAlarm"),
            Consumer::MedicalAlarm => write!(f, "MedicalAlarm"),
            Consumer::BalanceRight => write!(f, "BalanceRight"),
            Consumer::BalanceLeft => write!(f, "BalanceLeft"),
            Consumer::BassIncrement => write!(f, "BassIncrement"),
            Consumer::BassDecrement => write!(f, "BassDecrement"),
            Consumer::TrebleIncrement => write!(f, "TrebleIncrement"),
            Consumer::TrebleDecrement => write!(f, "TrebleDecrement"),
            Consumer::SpeakerSystem => write!(f, "SpeakerSystem"),
            Consumer::ChannelLeft => write!(f, "ChannelLeft"),
            Consumer::ChannelRight => write!(f, "ChannelRight"),
            Consumer::ChannelCenter => write!(f, "ChannelCenter"),
            Consumer::ChannelFront => write!(f, "ChannelFront"),
            Consumer::ChannelCenterFront => write!(f, "ChannelCenterFront"),
            Consumer::ChannelSide => write!(f, "ChannelSide"),
            Consumer::ChannelSurround => write!(f, "ChannelSurround"),
            Consumer::ChannelLowFrequencyEnhancement => write!(f, "ChannelLowFrequencyEnhancement"),
            Consumer::ChannelTop => write!(f, "ChannelTop"),
            Consumer::ChannelUnknown => write!(f, "ChannelUnknown"),
            Consumer::SubChannel => write!(f, "SubChannel"),
            Consumer::SubChannelIncrement => write!(f, "SubChannelIncrement"),
            Consumer::SubChannelDecrement => write!(f, "SubChannelDecrement"),
            Consumer::AlternateAudioIncrement => write!(f, "AlternateAudioIncrement"),
            Consumer::AlternateAudioDecrement => write!(f, "AlternateAudioDecrement"),
            Consumer::ApplicationLaunchButtons => write!(f, "ApplicationLaunchButtons"),
            Consumer::ALLaunchButtonConfigurationTool => write!(f, "ALLaunchButtonConfigurationTool"),
            Consumer::ALProgrammableButtonConfiguration => write!(f, "ALProgrammableButtonConfiguration"),
            Consumer::ALConsumerControlConfiguration => write!(f, "ALConsumerControlConfiguration"),
            Consumer::ALWordProcessor => write!(f, "ALWordProcessor"),
            Consumer::ALTextEditor => write!(f, "ALTextEditor"),
            Consumer::ALSpreadsheet => write!(f, "ALSpreadsheet"),
            Consumer::ALGraphicsEditor => write!(f, "ALGraphicsEditor"),
            Consumer::ALPresentationApp => write!(f, "ALPresentationApp"),
            Consumer::ALDatabaseApp => write!(f, "ALDatabaseApp"),
            Consumer::ALEmailReader => write!(f, "ALEmailReader"),
            Consumer::ALNewsreader => write!(f, "ALNewsreader"),
            Consumer::ALVoicemail => write!(f, "ALVoicemail"),
            Consumer::ALContactsAddressBook => write!(f, "ALContactsAddressBook"),
            Consumer::ALCalendarSchedule => write!(f, "ALCalendarSchedule"),
            Consumer::ALTaskProjectManager => write!(f, "ALTaskProjectManager"),
            Consumer::ALLogJournalTimecard => write!(f, "ALLogJournalTimecard"),
            Consumer::ALCheckbookFinance => write!(f, "ALCheckbookFinance"),
            Consumer::ALCalculator => write!(f, "ALCalculator"),
            Consumer::ALAvCapturePlayback => write!(f, "ALAvCapturePlayback"),
            Consumer::ALLocalMachineBrowser => write!(f, "ALLocalMachineBrowser"),
            Consumer::ALLanWanBrowser => write!(f, "ALLanWanBrowser"),
            Consumer::ALInternetBrowser => write!(f, "ALInternetBrowser"),
            Consumer::ALRemoteNetworkingISPConnect => write!(f, "ALRemoteNetworkingISPConnect"),
            Consumer::ALNetworkConference => write!(f, "ALNetworkConference"),
            Consumer::ALNetworkChat => write!(f, "ALNetworkChat"),
            Consumer::ALTelephonyDialer => write!(f, "ALTelephonyDialer"),
            Consumer::ALLogon => write!(f, "ALLogon"),
            Consumer::ALLogoff => write!(f, "ALLogoff"),
            Consumer::ALLogonLogoff => write!(f, "ALLogonLogoff"),
            Consumer::ALTerminalLockScreensaver => write!(f, "ALTerminalLockScreensaver"),
            Consumer::ALControlPanel => write!(f, "ALControlPanel"),
            Consumer::ALCommandLineProcessorRun => write!(f, "ALCommandLineProcessorRun"),
            Consumer::ALProcessTaskManager => write!(f, "ALProcessTaskManager"),
            Consumer::ALSelectTaskApplication => write!(f, "ALSelectTaskApplication"),
            Consumer::ALNextTaskApplication => write!(f, "ALNextTaskApplication"),
            Consumer::ALPreviousTaskApplication => write!(f, "ALPreviousTaskApplication"),
            Consumer::ALPreemptiveHaltTaskApplication => write!(f, "ALPreemptiveHaltTaskApplication"),
            Consumer::ALIntegratedHelpCenter => write!(f, "ALIntegratedHelpCenter"),
            Consumer::ALDocuments => write!(f, "ALDocuments"),
            Consumer::ALThesaurus => write!(f, "ALThesaurus"),
            Consumer::ALDictionary => write!(f, "ALDictionary"),
            Consumer::ALDesktop => write!(f, "ALDesktop"),
            Consumer::ALSpellCheck => write!(f, "ALSpellCheck"),
            Consumer::ALGrammarCheck => write!(f, "ALGrammarCheck"),
            Consumer::ALWirelessStatus => write!(f, "ALWirelessStatus"),
            Consumer::ALKeyboardLayout => write!(f, "ALKeyboardLayout"),
            Consumer::ALVirusProtection => write!(f, "ALVirusProtection"),
            Consumer::ALEncryption => write!(f, "ALEncryption"),
            Consumer::ALScreenSaver => write!(f, "ALScreenSaver"),
            Consumer::ALAlarms => write!(f, "ALAlarms"),
            Consumer::ALClock => write!(f, "ALClock"),
            Consumer::ALFileBrowser => write!(f, "ALFileBrowser"),
            Consumer::ALPowerStatus => write!(f, "ALPowerStatus"),
            Consumer::ALImageBrowser => write!(f, "ALImageBrowser"),
            Consumer::ALAudioBrowser => write!(f, "ALAudioBrowser"),
            Consumer::ALMovieBrowser => write!(f, "ALMovieBrowser"),
            Consumer::ALDigitalRightsManager => write!(f, "ALDigitalRightsManager"),
            Consumer::ALDigitalWallet => write!(f, "ALDigitalWallet"),
            Consumer::ALInstantMessaging => write!(f, "ALInstantMessaging"),
            Consumer::ALOemFeaturesTipsTutorialBrowser => write!(f, "ALOemFeaturesTipsTutorialBrowser"),
            Consumer::ALOemHelp => write!(f, "ALOemHelp"),
            Consumer::ALOnlineCommunity => write!(f, "ALOnlineCommunity"),
            Consumer::ALEntertainmentContentBrowser => write!(f, "ALEntertainmentContentBrowser"),
            Consumer::ALOnlineShoppingBrowser => write!(f, "ALOnlineShoppingBrowser"),
            Consumer::ALSmartCardInformationHelp => write!(f, "ALSmartCardInformationHelp"),
            Consumer::ALMarketMonitorFinanceBrowser => write!(f, "ALMarketMonitorFinanceBrowser"),
            Consumer::ALCustomizedCorporateNewsBrowser => write!(f, "ALCustomizedCorporateNewsBrowser"),
            Consumer::ALOnlineActivityBrowser => write!(f, "ALOnlineActivityBrowser"),
            Consumer::ALResearchSearchBrowser => write!(f, "ALResearchSearchBrowser"),
            Consumer::ALAudioPlayer => write!(f, "ALAudioPlayer"),
            Consumer::GenericGUIApplicationControls => write!(f, "GenericGUIApplicationControls"),
            Consumer::ACNew => write!(f, "ACNew"),
            Consumer::ACOpen => write!(f, "ACOpen"),
            Consumer::ACClose => write!(f, "ACClose"),
            Consumer::ACExit => write!(f, "ACExit"),
            Consumer::ACMaximize => write!(f, "ACMaximize"),
            Consumer::ACMinimize => write!(f, "ACMinimize"),
            Consumer::ACSave => write!(f, "ACSave"),
            Consumer::ACPrint => write!(f, "ACPrint"),
            Consumer::ACProperties => write!(f, "ACProperties"),
            Consumer::ACUndo => write!(f, "ACUndo"),
            Consumer::ACCopy => write!(f, "ACCopy"),
            Consumer::ACCut => write!(f, "ACCut"),
            Consumer::ACPaste => write!(f, "ACPaste"),
            Consumer::ACSelectAll => write!(f, "ACSelectAll"),
            Consumer::ACFind => write!(f, "ACFind"),
            Consumer::ACFindAndReplace => write!(f, "ACFindAndReplace"),
            Consumer::ACSearch => write!(f, "ACSearch"),
            Consumer::ACGoTo => write!(f, "ACGoTo"),
            Consumer::ACHome => write!(f, "ACHome"),
            Consumer::ACBack => write!(f, "ACBack"),
            Consumer::ACForward => write!(f, "ACForward"),
            Consumer::ACStop => write!(f, "ACStop"),
            Consumer::ACRefresh => write!(f, "ACRefresh"),
            Consumer::ACPreviousLink => write!(f, "ACPreviousLink"),
            Consumer::ACNextLink => write!(f, "ACNextLink"),
            Consumer::ACBookmarks => write!(f, "ACBookmarks"),
            Consumer::ACHistory => write!(f, "ACHistory"),
            Consumer::ACSubscriptions => write!(f, "ACSubscriptions"),
            Consumer::ACZoomIn => write!(f, "ACZoomIn"),
            Consumer::ACZoomOut => write!(f, "ACZoomOut"),
            Consumer::ACZoom => write!(f, "ACZoom"),
            Consumer::ACFullScreenView => write!(f, "ACFullScreenView"),
            Consumer::ACNormalView => write!(f, "ACNormalView"),
            Consumer::ACViewToggle => write!(f, "ACViewToggle"),
            Consumer::ACScrollUp => write!(f, "ACScrollUp"),
            Consumer::ACScrollDown => write!(f, "ACScrollDown"),
            Consumer::ACScroll => write!(f, "ACScroll"),
            Consumer::ACPanLeft => write!(f, "ACPanLeft"),
            Consumer::ACPanRight => write!(f, "ACPanRight"),
            Consumer::ACPan => write!(f, "ACPan"),
            Consumer::ACNewWindow => write!(f, "ACNewWindow"),
            Consumer::ACTileHorizontally => write!(f, "ACTileHorizontally"),
            Consumer::ACTileVertically => write!(f, "ACTileVertically"),
            Consumer::ACFormat => write!(f, "ACFormat"),
            Consumer::ACEdit => write!(f, "ACEdit"),
            Consumer::ACBold => write!(f, "ACBold"),
            Consumer::ACItalics => write!(f, "ACItalics"),
            Consumer::ACUnderline => write!(f, "ACUnderline"),
            Consumer::ACStrikethrough => write!(f, "ACStrikethrough"),
            Consumer::ACSubscript => write!(f, "ACSubscript"),
            Consumer::ACSuperscript => write!(f, "ACSuperscript"),
            Consumer::ACAllCaps => write!(f, "ACAllCaps"),
            Consumer::ACRotate => write!(f, "ACRotate"),
            Consumer::ACResize => write!(f, "ACResize"),
            Consumer::ACFlipHorizontal => write!(f, "ACFlipHorizontal"),
            Consumer::ACFlipVertical => write!(f, "ACFlipVertical"),
            Consumer::ACMirrorHorizontal => write!(f, "ACMirrorHorizontal"),
            Consumer::ACMirrorVertical => write!(f, "ACMirrorVertical"),
            Consumer::ACFontSelect => write!(f, "ACFontSelect"),
            Consumer::ACFontColor => write!(f, "ACFontColor"),
            Consumer::ACFontSize => write!(f, "ACFontSize"),
            Consumer::ACJustifyLeft => write!(f, "ACJustifyLeft"),
            Consumer::ACJustifyCenterH => write!(f, "ACJustifyCenterH"),
            Consumer::ACJustifyRight => write!(f, "ACJustifyRight"),
            Consumer::ACJustifyBlockH => write!(f, "ACJustifyBlockH"),
            Consumer::ACJustifyTop => write!(f, "ACJustifyTop"),
            Consumer::ACJustifyCenterV => write!(f, "ACJustifyCenterV"),
            Consumer::ACJustifyBottom => write!(f, "ACJustifyBottom"),
            Consumer::ACJustifyBlockV => write!(f, "ACJustifyBlockV"),
            Consumer::ACIndentDecrease => write!(f, "ACIndentDecrease"),
            Consumer::ACIndentIncrease => write!(f, "ACIndentIncrease"),
            Consumer::ACNumberedList => write!(f, "ACNumberedList"),
            Consumer::ACRestartNumbering => write!(f, "ACRestartNumbering"),
            Consumer::ACBulletedList => write!(f, "ACBulletedList"),
            Consumer::ACPromote => write!(f, "ACPromote"),
            Consumer::ACDemote => write!(f, "ACDemote"),
            Consumer::ACYes => write!(f, "ACYes"),
            Consumer::ACNo => write!(f, "ACNo"),
            Consumer::ACCancel => write!(f, "ACCancel"),
            Consumer::ACCatalog => write!(f, "ACCatalog"),
            Consumer::ACBuyCheckout => write!(f, "ACBuyCheckout"),
            Consumer::ACAddToCart => write!(f, "ACAddToCart"),
            Consumer::ACExpand => write!(f, "ACExpand"),
            Consumer::ACExpandAll => write!(f, "ACExpandAll"),
            Consumer::ACCollapse => write!(f, "ACCollapse"),
            Consumer::ACCollapseAll => write!(f, "ACCollapseAll"),
            Consumer::ACPrintPreview => write!(f, "ACPrintPreview"),
            Consumer::ACPasteSpecial => write!(f, "ACPasteSpecial"),
            Consumer::ACInsertMode => write!(f, "ACInsertMode"),
            Consumer::ACDelete => write!(f, "ACDelete"),
            Consumer::ACLock => write!(f, "ACLock"),
            Consumer::ACUnlock => write!(f, "ACUnlock"),
            Consumer::ACProtect => write!(f, "ACProtect"),
            Consumer::ACUnprotect => write!(f, "ACUnprotect"),
            Consumer::ACAttachComment => write!(f, "ACAttachComment"),
            Consumer::ACDeleteComment => write!(f, "ACDeleteComment"),
            Consumer::ACViewComment => write!(f, "ACViewComment"),
            Consumer::ACSelectWord => write!(f, "ACSelectWord"),
            Consumer::ACSelectSentence => write!(f, "ACSelectSentence"),
            Consumer::ACSelectParagraph => write!(f, "ACSelectParagraph"),
            Consumer::ACSelectColumn => write!(f, "ACSelectColumn"),
            Consumer::ACSelectRow => write!(f, "ACSelectRow"),
            Consumer::ACSelectTable => write!(f, "ACSelectTable"),
            Consumer::ACSelectObject => write!(f, "ACSelectObject"),
            Consumer::ACRedoRepeat => write!(f, "ACRedoRepeat"),
            Consumer::ACSort => write!(f, "ACSort"),
            Consumer::ACSortAscending => write!(f, "ACSortAscending"),
            Consumer::ACSortDescending => write!(f, "ACSortDescending"),
            Consumer::ACFilter => write!(f, "ACFilter"),
            Consumer::ACSetClock => write!(f, "ACSetClock"),
            Consumer::ACViewClock => write!(f, "ACViewClock"),
            Consumer::ACSelectTimeZone => write!(f, "ACSelectTimeZone"),
            Consumer::ACEditTimeZones => write!(f, "ACEditTimeZones"),
            Consumer::ACSetAlarm => write!(f, "ACSetAlarm"),
            Consumer::ACClearAlarm => write!(f, "ACClearAlarm"),
            Consumer::ACSnoozeAlarm => write!(f, "ACSnoozeAlarm"),
            Consumer::ACResetAlarm => write!(f, "ACResetAlarm"),
            Consumer::ACSynchronize => write!(f, "ACSynchronize"),
            Consumer::ACSendReceive => write!(f, "ACSendReceive"),
            Consumer::ACSendTo => write!(f, "ACSendTo"),
            Consumer::ACReply => write!(f, "ACReply"),
            Consumer::ACReplyAll => write!(f, "ACReplyAll"),
            Consumer::ACForwardMsg => write!(f, "ACForwardMsg"),
            Consumer::ACSend => write!(f, "ACSend"),
            Consumer::ACAttachFile => write!(f, "ACAttachFile"),
            Consumer::ACUpload => write!(f, "ACUpload"),
            Consumer::ACDownloadSaveTargetAs => write!(f, "ACDownloadSaveTargetAs"),
            Consumer::ACSetBorders => write!(f, "ACSetBorders"),
            Consumer::ACInsertRow => write!(f, "ACInsertRow"),
            Consumer::ACInsertColumn => write!(f, "ACInsertColumn"),
            Consumer::ACInsertFile => write!(f, "ACInsertFile"),
            Consumer::ACInsertPicture => write!(f, "ACInsertPicture"),
            Consumer::ACInsertObject => write!(f, "ACInsertObject"),
            Consumer::ACInsertSymbol => write!(f, "ACInsertSymbol"),
            Consumer::ACSaveAndClose => write!(f, "ACSaveAndClose"),
            Consumer::ACRename => write!(f, "ACRename"),
            Consumer::ACMerge => write!(f, "ACMerge"),
            Consumer::ACSplit => write!(f, "ACSplit"),
            Consumer::ACDistributeHorizontally => write!(f, "ACDistributeHorizontally"),
            Consumer::ACDistributeVertically => write!(f, "ACDistributeVertically"),
        }
    }
}

impl ConsumerWrapper {
    pub const KEYS: [ConsumerWrapper; 7] = [
        ConsumerWrapper(Consumer::VolumeDecrement),
        ConsumerWrapper(Consumer::VolumeIncrement),
        ConsumerWrapper(Consumer::Play),
        ConsumerWrapper(Consumer::Pause),
        ConsumerWrapper(Consumer::PlayPause),
        ConsumerWrapper(Consumer::FastForward),
        ConsumerWrapper(Consumer::Rewind),
    ];
}

impl From<Consumer> for ConsumerWrapper {
    fn from(consumer: Consumer) -> ConsumerWrapper {
        ConsumerWrapper(consumer)
    }
}

impl From<ConsumerWrapper> for Consumer {
    fn from(wrapper: ConsumerWrapper) -> Consumer {
        wrapper.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub gui: bool,

    pub string: String,
}

impl Chord {
}

impl From<Vec<KeyboardWrapper>> for Chord {
    fn from(keys: Vec<KeyboardWrapper>) -> Chord {
        keys.into_iter().map(KeyboardWrapper::from).collect::<Vec<KeyboardWrapper>>().into()
        
    }
}

impl From<Chord> for Vec<Keyboard> {
    fn from(chord: Chord) -> Self {
        let mut keys = Vec::new();
        if chord.ctrl {
            keys.push(Keyboard::LeftControl);
        }
        if chord.alt {
            keys.push(Keyboard::LeftAlt);
        }
        if chord.shift {
            keys.push(Keyboard::LeftShift);
        }
        if chord.gui {
            keys.push(Keyboard::LeftGUI);
        }

        for key in chord.string.chars() {
            match key {
                'A' => keys.push(Keyboard::A),
                'B' => keys.push(Keyboard::B),
                'C' => keys.push(Keyboard::C),
                'D' => keys.push(Keyboard::D),
                'E' => keys.push(Keyboard::E),
                'F' => keys.push(Keyboard::F),
                'G' => keys.push(Keyboard::G),
                'H' => keys.push(Keyboard::H),
                'I' => keys.push(Keyboard::I),
                'J' => keys.push(Keyboard::J),
                'K' => keys.push(Keyboard::K),
                'L' => keys.push(Keyboard::L),
                'M' => keys.push(Keyboard::M),
                'N' => keys.push(Keyboard::N),
                'O' => keys.push(Keyboard::O),
                'P' => keys.push(Keyboard::P),
                'Q' => keys.push(Keyboard::Q),
                'R' => keys.push(Keyboard::R),
                'S' => keys.push(Keyboard::S),
                'T' => keys.push(Keyboard::T),
                'U' => keys.push(Keyboard::U),
                'V' => keys.push(Keyboard::V),
                'W' => keys.push(Keyboard::W),
                'X' => keys.push(Keyboard::X),
                'Y' => keys.push(Keyboard::Y),
                'Z' => keys.push(Keyboard::Z),
                '1' => keys.push(Keyboard::Keyboard1),
                '2' => keys.push(Keyboard::Keyboard2),
                '3' => keys.push(Keyboard::Keyboard3),
                '4' => keys.push(Keyboard::Keyboard4),
                '5' => keys.push(Keyboard::Keyboard5),
                '6' => keys.push(Keyboard::Keyboard6),
                '7' => keys.push(Keyboard::Keyboard7),
                '8' => keys.push(Keyboard::Keyboard8),
                '9' => keys.push(Keyboard::Keyboard9),
                '0' => keys.push(Keyboard::Keyboard0),
                '⏎' => keys.push(Keyboard::ReturnEnter),
                '⎋' => keys.push(Keyboard::Escape),
                '⇥' => keys.push(Keyboard::Tab),
                ' ' => keys.push(Keyboard::Space),
                '-' => keys.push(Keyboard::Minus),
                '=' => keys.push(Keyboard::Equal),
                '[' => keys.push(Keyboard::LeftBrace),
                ']' => keys.push(Keyboard::RightBrace),
                '\\' => keys.push(Keyboard::Backslash),
                '#' => keys.push(Keyboard::NonUSHash),
                ';' => keys.push(Keyboard::Semicolon),
                '\'' => keys.push(Keyboard::Apostrophe),
                '`' => keys.push(Keyboard::Grave),
                ',' => keys.push(Keyboard::Comma),
                '.' => keys.push(Keyboard::Dot),
                '/' => keys.push(Keyboard::ForwardSlash),
                '→' => keys.push(Keyboard::RightArrow),
                '←' => keys.push(Keyboard::LeftArrow),
                '↓' => keys.push(Keyboard::DownArrow),
                '↑' => keys.push(Keyboard::UpArrow),

                _ => unimplemented!(),
            }
        }

        keys
    }
}

impl From<Chord> for Vec<KeyboardWrapper> {
    fn from(chord: Chord) -> Self {
        <Vec<Keyboard>>::from(chord).into_iter().map(KeyboardWrapper::from).collect::<Vec<KeyboardWrapper>>()
    }
}

impl From<Vec<Keyboard>> for Chord {
    fn from(keys: Vec<Keyboard>) -> Chord {
        let mut ctrl = false;
        let mut alt = false;
        let mut shift = false;
        let mut gui = false;
        let mut string = String::new();

        for key in keys {
            match key {
                Keyboard::A => {
                    if !string.contains("A") {
                        string.push('A');
                    }
                },
                Keyboard::B => {
                    if !string.contains("B") {
                        string.push('B');
                    }
                },
                Keyboard::C => {
                    if !string.contains("C") {
                        string.push('C');
                    }
                },
                Keyboard::D => {
                    if !string.contains("D") {
                        string.push('D');
                    }
                },
                Keyboard::E => {
                    if !string.contains("E") {
                        string.push('E');
                    }
                },
                Keyboard::F => {
                    if !string.contains("F") {
                        string.push('F');
                    }
                },
                Keyboard::G => {
                    if !string.contains("G") {
                        string.push('G');
                    }
                },
                Keyboard::H => {
                    if !string.contains("H") {
                        string.push('H');
                    }
                },
                Keyboard::I => {
                    if !string.contains("I") {
                        string.push('I');
                    }
                },
                Keyboard::J => {
                    if !string.contains("J") {
                        string.push('J');
                    }
                },
                Keyboard::K => {
                    if !string.contains("K") {
                        string.push('K');
                    }
                },
                Keyboard::L => {
                    if !string.contains("L") {
                        string.push('L');
                    }
                },
                Keyboard::M => {
                    if !string.contains("M") {
                        string.push('M');
                    }
                },
                Keyboard::N => {
                    if !string.contains("N") {
                        string.push('N');
                    }
                },
                Keyboard::O => {
                    if !string.contains("O") {
                        string.push('O');
                    }
                },
                Keyboard::P => {
                    if !string.contains("P") {
                        string.push('P');
                    }
                },
                Keyboard::Q => {
                    if !string.contains("Q") {
                        string.push('Q');
                    }
                },
                Keyboard::R => {
                    if !string.contains("R") {
                        string.push('R');
                    }
                },
                Keyboard::S => {
                    if !string.contains("S") {
                        string.push('S');
                    }
                },
                Keyboard::T => {
                    if !string.contains("T") {
                        string.push('T');
                    }
                },
                Keyboard::U => {
                    if !string.contains("U") {
                        string.push('U');
                    }
                },
                Keyboard::V => {
                    if !string.contains("V") {
                        string.push('V');
                    }
                },
                Keyboard::W => {
                    if !string.contains("W") {
                        string.push('W');
                    }
                },
                Keyboard::X => {
                    if !string.contains("X") {
                        string.push('X');
                    }
                },
                Keyboard::Y => {
                    if !string.contains("Y") {
                        string.push('Y');
                    }
                },
                Keyboard::Z => {
                    if !string.contains("Z") {
                        string.push('Z');
                    }
                },
                Keyboard::Keyboard1 => {
                    if !string.contains("1") {
                        string.push('1');
                    }
                },
                Keyboard::Keyboard2 => {
                    if !string.contains("2") {
                        string.push('2');
                    }
                },
                Keyboard::Keyboard3 => {
                    if !string.contains("3") {
                        string.push('3');
                    }
                },
                Keyboard::Keyboard4 => {
                    if !string.contains("4") {
                        string.push('4');
                    }
                },
                Keyboard::Keyboard5 => {
                    if !string.contains("5") {
                        string.push('5');
                    }
                },
                Keyboard::Keyboard6 => {
                    if !string.contains("6") {
                        string.push('6');
                    }
                },
                Keyboard::Keyboard7 => {
                    if !string.contains("7") {
                        string.push('7');
                    }
                },
                Keyboard::Keyboard8 => {
                    if !string.contains("8") {
                        string.push('8');
                    }
                },
                Keyboard::Keyboard9 => {
                    if !string.contains("9") {
                        string.push('9');
                    }
                },
                Keyboard::Keyboard0 => {
                    if !string.contains("0") {
                        string.push('0');
                    }
                },
                Keyboard::ReturnEnter => {
                    if !string.contains("⏎") {
                        string.push('⏎');
                    }
                },
                Keyboard::Escape => {
                    if !string.contains("⎋") {
                        string.push('⎋');
                    }
                },
                Keyboard::Tab => {
                    if !string.contains("⇥") {
                        string.push('⇥');
                    }
                },
                Keyboard::Space => {
                    if !string.contains(" ") {
                        string.push(' ');
                    }
                },
                Keyboard::Minus => {
                    if !string.contains("-") {
                        string.push('-');
                    }
                },
                Keyboard::Equal => {
                    if !string.contains("=") {
                        string.push('=');
                    }
                },
                Keyboard::LeftBrace => {
                    if !string.contains("[") {
                        string.push('[');
                    }
                },
                Keyboard::RightBrace => {
                    if !string.contains("]") {
                        string.push(']');
                    }
                },
                Keyboard::Backslash => {
                    if !string.contains("\\") {
                        string.push('\\');
                    }
                },
                Keyboard::NonUSHash => {
                    if !string.contains("#") {
                        string.push('#');
                    }
                },
                Keyboard::Semicolon => {
                    if !string.contains(";") {
                        string.push(';');
                    }
                },
                Keyboard::Apostrophe => {
                    if !string.contains("'") {
                        string.push('\'');
                    }
                },
                Keyboard::Grave => {
                    if !string.contains("`") {
                        string.push('`');
                    }
                },
                Keyboard::Comma => {
                    if !string.contains(",") {
                        string.push(',');
                    }
                },
                Keyboard::Dot => {
                    if !string.contains(".") {
                        string.push('.');
                    }
                },
                Keyboard::ForwardSlash => {
                    if !string.contains("/") {
                        string.push('/');
                    }
                },
                Keyboard::RightArrow => {
                    if !string.contains("→") {
                        string.push('→');
                    }
                },
                Keyboard::LeftArrow => {
                    if !string.contains("←") {
                        string.push('←');
                    }
                },
                Keyboard::DownArrow => {
                    if !string.contains("↓") {
                        string.push('↓');
                    }
                },
                Keyboard::UpArrow => {
                    if !string.contains("↑") {
                        string.push('↑');
                    }
                },
                Keyboard::LeftControl => {
                    ctrl = true;
                },
                Keyboard::LeftShift => {
                    shift = true;
                },
                Keyboard::LeftAlt => {
                    alt = true;
                },
                Keyboard::LeftGUI => {
                    gui = true;
                },
                Keyboard::RightControl => {
                    ctrl = true;
                },
                Keyboard::RightShift => {
                    shift = true;
                },
                Keyboard::RightAlt => {
                    alt = true;
                },
                Keyboard::RightGUI => {
                    gui = true;
                },

                _ => unimplemented!(),
            }
        }

        Chord {
            ctrl,
            alt,
            shift,
            gui,
            string,
        }
    }
}