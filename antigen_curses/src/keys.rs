pub struct CursesInput(pancurses::Input);

impl From<pancurses::Input> for CursesInput {
    fn from(input: pancurses::Input) -> Self {
        CursesInput(input)
    }
}

impl antigen::core::keyboard::IntoKey for CursesInput {
    fn into_key(self) -> antigen::core::keyboard::Key {
        let CursesInput(input) = self;
        match input {
            pancurses::Input::Character(char) => match char {
                ' ' => antigen::core::keyboard::Key::Space,
                '!' => antigen::core::keyboard::Key::ExclamationMark,
                '"' => antigen::core::keyboard::Key::DoubleQuote,
                '#' => antigen::core::keyboard::Key::Number,
                '$' => antigen::core::keyboard::Key::Dollar,
                '%' => antigen::core::keyboard::Key::Percent,
                '&' => antigen::core::keyboard::Key::Ampersand,
                '\'' => antigen::core::keyboard::Key::SingleQuote,
                '(' => antigen::core::keyboard::Key::OpenBracket,
                ')' => antigen::core::keyboard::Key::CloseBracket,
                '*' => antigen::core::keyboard::Key::Asterisk,
                '+' => antigen::core::keyboard::Key::Plus,
                ',' => antigen::core::keyboard::Key::Comma,
                '-' => antigen::core::keyboard::Key::Hyphen,
                '.' => antigen::core::keyboard::Key::Period,
                '/' => antigen::core::keyboard::Key::Slash,
                '0' => antigen::core::keyboard::Key::N0,
                '1' => antigen::core::keyboard::Key::N1,
                '2' => antigen::core::keyboard::Key::N2,
                '3' => antigen::core::keyboard::Key::N3,
                '4' => antigen::core::keyboard::Key::N4,
                '5' => antigen::core::keyboard::Key::N5,
                '6' => antigen::core::keyboard::Key::N6,
                '7' => antigen::core::keyboard::Key::N7,
                '8' => antigen::core::keyboard::Key::N8,
                '9' => antigen::core::keyboard::Key::N9,
                ':' => antigen::core::keyboard::Key::Colon,
                ';' => antigen::core::keyboard::Key::Semicolon,
                '<' => antigen::core::keyboard::Key::LessThan,
                '=' => antigen::core::keyboard::Key::Equals,
                '>' => antigen::core::keyboard::Key::GreaterThan,
                '?' => antigen::core::keyboard::Key::QuestionMark,
                '@' => antigen::core::keyboard::Key::At,
                'A' => antigen::core::keyboard::Key::A,
                'B' => antigen::core::keyboard::Key::B,
                'C' => antigen::core::keyboard::Key::C,
                'D' => antigen::core::keyboard::Key::D,
                'E' => antigen::core::keyboard::Key::E,
                'F' => antigen::core::keyboard::Key::F,
                'G' => antigen::core::keyboard::Key::G,
                'H' => antigen::core::keyboard::Key::H,
                'I' => antigen::core::keyboard::Key::I,
                'J' => antigen::core::keyboard::Key::J,
                'K' => antigen::core::keyboard::Key::K,
                'L' => antigen::core::keyboard::Key::L,
                'M' => antigen::core::keyboard::Key::M,
                'N' => antigen::core::keyboard::Key::N,
                'O' => antigen::core::keyboard::Key::O,
                'P' => antigen::core::keyboard::Key::P,
                'Q' => antigen::core::keyboard::Key::Q,
                'R' => antigen::core::keyboard::Key::R,
                'S' => antigen::core::keyboard::Key::S,
                'T' => antigen::core::keyboard::Key::T,
                'U' => antigen::core::keyboard::Key::U,
                'V' => antigen::core::keyboard::Key::V,
                'W' => antigen::core::keyboard::Key::W,
                'X' => antigen::core::keyboard::Key::X,
                'Y' => antigen::core::keyboard::Key::Y,
                'Z' => antigen::core::keyboard::Key::Z,
                '[' => antigen::core::keyboard::Key::OpeningBracket,
                '\\' => antigen::core::keyboard::Key::Backslash,
                ']' => antigen::core::keyboard::Key::ClosingBracket,
                '^' => antigen::core::keyboard::Key::Caret,
                '_' => antigen::core::keyboard::Key::Underscore,
                '`' => antigen::core::keyboard::Key::Grave,
                'a' => antigen::core::keyboard::Key::A,
                'b' => antigen::core::keyboard::Key::B,
                'c' => antigen::core::keyboard::Key::C,
                'd' => antigen::core::keyboard::Key::D,
                'e' => antigen::core::keyboard::Key::E,
                'f' => antigen::core::keyboard::Key::F,
                'g' => antigen::core::keyboard::Key::G,
                'h' => antigen::core::keyboard::Key::H,
                'i' => antigen::core::keyboard::Key::I,
                'j' => antigen::core::keyboard::Key::J,
                'k' => antigen::core::keyboard::Key::K,
                'l' => antigen::core::keyboard::Key::L,
                'm' => antigen::core::keyboard::Key::M,
                'n' => antigen::core::keyboard::Key::N,
                'o' => antigen::core::keyboard::Key::O,
                'p' => antigen::core::keyboard::Key::P,
                'q' => antigen::core::keyboard::Key::Q,
                'r' => antigen::core::keyboard::Key::R,
                's' => antigen::core::keyboard::Key::S,
                't' => antigen::core::keyboard::Key::T,
                'u' => antigen::core::keyboard::Key::U,
                'v' => antigen::core::keyboard::Key::V,
                'w' => antigen::core::keyboard::Key::W,
                'x' => antigen::core::keyboard::Key::X,
                'y' => antigen::core::keyboard::Key::Y,
                'z' => antigen::core::keyboard::Key::Z,
                '{' => antigen::core::keyboard::Key::OpeningBrace,
                '|' => antigen::core::keyboard::Key::VerticalBar,
                '}' => antigen::core::keyboard::Key::ClosingBrace,
                '~' => antigen::core::keyboard::Key::Tilde,
                '\u{8}' => antigen::core::keyboard::Key::Backspace,
                '\u{1b}' => antigen::core::keyboard::Key::Escape,
                '\u{ed2c}' => antigen::core::keyboard::Key::Menu,
                '\u{ed2d}' => antigen::core::keyboard::Key::Menu,
                '\u{ed35}' => antigen::core::keyboard::Key::ScrollLock,
                '\u{ed30}' => antigen::core::keyboard::Key::Pause,
                '\u{ed12}' => antigen::core::keyboard::Key::Enter,
                '\u{ecc5}' => antigen::core::keyboard::Key::Center,
                '\u{ec18}' => antigen::core::keyboard::Key::F16,
                '\u{ec19}' => antigen::core::keyboard::Key::F17,
                '\u{ec1a}' => antigen::core::keyboard::Key::F18,
                '\u{ec1b}' => antigen::core::keyboard::Key::F19,
                '\u{ec1c}' => antigen::core::keyboard::Key::F20,
                '\u{ec1d}' => antigen::core::keyboard::Key::F21,
                '\u{ec1e}' => antigen::core::keyboard::Key::F22,
                '\u{ec1f}' => antigen::core::keyboard::Key::F23,
                '\u{ec20}' => antigen::core::keyboard::Key::F24,
                '\t' => antigen::core::keyboard::Key::Tab,
                '\n' => antigen::core::keyboard::Key::Enter,
                _ => unsupported_input(input),
            },
            pancurses::Input::KeyBreak => antigen::core::keyboard::Key::Break,
            pancurses::Input::KeyDown => antigen::core::keyboard::Key::Down,
            pancurses::Input::KeyUp => antigen::core::keyboard::Key::Up,
            pancurses::Input::KeyLeft => antigen::core::keyboard::Key::Left,
            pancurses::Input::KeyRight => antigen::core::keyboard::Key::Right,
            pancurses::Input::KeyHome => antigen::core::keyboard::Key::Home,
            pancurses::Input::KeyBackspace => antigen::core::keyboard::Key::Backspace,
            pancurses::Input::KeyF0 => antigen::core::keyboard::Key::F0,
            pancurses::Input::KeyF1 => antigen::core::keyboard::Key::F1,
            pancurses::Input::KeyF2 => antigen::core::keyboard::Key::F2,
            pancurses::Input::KeyF3 => antigen::core::keyboard::Key::F3,
            pancurses::Input::KeyF4 => antigen::core::keyboard::Key::F4,
            pancurses::Input::KeyF5 => antigen::core::keyboard::Key::F5,
            pancurses::Input::KeyF6 => antigen::core::keyboard::Key::F6,
            pancurses::Input::KeyF7 => antigen::core::keyboard::Key::F7,
            pancurses::Input::KeyF8 => antigen::core::keyboard::Key::F8,
            pancurses::Input::KeyF9 => antigen::core::keyboard::Key::F9,
            pancurses::Input::KeyF10 => antigen::core::keyboard::Key::F10,
            pancurses::Input::KeyF11 => antigen::core::keyboard::Key::F11,
            pancurses::Input::KeyF12 => antigen::core::keyboard::Key::F12,
            pancurses::Input::KeyF13 => antigen::core::keyboard::Key::F13,
            pancurses::Input::KeyF14 => antigen::core::keyboard::Key::F14,
            pancurses::Input::KeyF15 => antigen::core::keyboard::Key::F15,
            pancurses::Input::KeyDL => antigen::core::keyboard::Key::DeleteLine,
            pancurses::Input::KeyIL => antigen::core::keyboard::Key::InsertLine,
            pancurses::Input::KeyDC => antigen::core::keyboard::Key::Delete,
            pancurses::Input::KeyIC => antigen::core::keyboard::Key::Insert,
            pancurses::Input::KeyEIC => antigen::core::keyboard::Key::ExitInsertCharacter,
            pancurses::Input::KeyClear => antigen::core::keyboard::Key::Clear,
            pancurses::Input::KeyEOS => antigen::core::keyboard::Key::EndOfScreen,
            pancurses::Input::KeyEOL => antigen::core::keyboard::Key::EndOfLine,
            pancurses::Input::KeySF => antigen::core::keyboard::Key::ScrollForward,
            pancurses::Input::KeySR => antigen::core::keyboard::Key::ScrollReverse,
            pancurses::Input::KeyNPage => antigen::core::keyboard::Key::PageDown,
            pancurses::Input::KeyPPage => antigen::core::keyboard::Key::PageUp,
            pancurses::Input::KeySTab => antigen::core::keyboard::Key::SetTab,
            pancurses::Input::KeyCTab => antigen::core::keyboard::Key::ClearTab,
            pancurses::Input::KeyCATab => antigen::core::keyboard::Key::ClearAllTabs,
            pancurses::Input::KeyEnter => antigen::core::keyboard::Key::Enter,
            pancurses::Input::KeySReset => antigen::core::keyboard::Key::SoftReset,
            pancurses::Input::KeyReset => antigen::core::keyboard::Key::Reset,
            pancurses::Input::KeyPrint => antigen::core::keyboard::Key::Print,
            pancurses::Input::KeyLL => antigen::core::keyboard::Key::LastLine,
            pancurses::Input::KeyAbort => antigen::core::keyboard::Key::Abort,
            pancurses::Input::KeySHelp => antigen::core::keyboard::Key::Help,
            pancurses::Input::KeyLHelp => antigen::core::keyboard::Key::LHelp,
            pancurses::Input::KeyBTab => antigen::core::keyboard::Key::BackTab,
            pancurses::Input::KeyBeg => antigen::core::keyboard::Key::Beginning,
            pancurses::Input::KeyCancel => antigen::core::keyboard::Key::Cancel,
            pancurses::Input::KeyClose => antigen::core::keyboard::Key::Close,
            pancurses::Input::KeyCommand => antigen::core::keyboard::Key::Command,
            pancurses::Input::KeyCopy => antigen::core::keyboard::Key::Copy,
            pancurses::Input::KeyCreate => antigen::core::keyboard::Key::Create,
            pancurses::Input::KeyEnd => antigen::core::keyboard::Key::End,
            pancurses::Input::KeyExit => antigen::core::keyboard::Key::Exit,
            pancurses::Input::KeyFind => antigen::core::keyboard::Key::Find,
            pancurses::Input::KeyHelp => antigen::core::keyboard::Key::Help,
            pancurses::Input::KeyMark => antigen::core::keyboard::Key::Mark,
            pancurses::Input::KeyMessage => antigen::core::keyboard::Key::Message,
            pancurses::Input::KeyMove => antigen::core::keyboard::Key::Move,
            pancurses::Input::KeyNext => antigen::core::keyboard::Key::Next,
            pancurses::Input::KeyOpen => antigen::core::keyboard::Key::Open,
            pancurses::Input::KeyOptions => antigen::core::keyboard::Key::Options,
            pancurses::Input::KeyPrevious => antigen::core::keyboard::Key::Previous,
            pancurses::Input::KeyRedo => antigen::core::keyboard::Key::Redo,
            pancurses::Input::KeyReference => antigen::core::keyboard::Key::Reference,
            pancurses::Input::KeyRefresh => antigen::core::keyboard::Key::Refresh,
            pancurses::Input::KeyReplace => antigen::core::keyboard::Key::Replace,
            pancurses::Input::KeyRestart => antigen::core::keyboard::Key::Restart,
            pancurses::Input::KeyResume => antigen::core::keyboard::Key::Resume,
            pancurses::Input::KeySave => antigen::core::keyboard::Key::Save,
            pancurses::Input::KeySBeg => antigen::core::keyboard::Key::Beginning,
            pancurses::Input::KeySCancel => antigen::core::keyboard::Key::Cancel,
            pancurses::Input::KeySCommand => antigen::core::keyboard::Key::Command,
            pancurses::Input::KeySCopy => antigen::core::keyboard::Key::Copy,
            pancurses::Input::KeySCreate => antigen::core::keyboard::Key::Create,
            pancurses::Input::KeySDC => antigen::core::keyboard::Key::Delete,
            pancurses::Input::KeySDL => antigen::core::keyboard::Key::DeleteLine,
            pancurses::Input::KeySelect => antigen::core::keyboard::Key::Select,
            pancurses::Input::KeySEnd => antigen::core::keyboard::Key::End,
            pancurses::Input::KeySEOL => antigen::core::keyboard::Key::EndOfLine,
            pancurses::Input::KeySExit => antigen::core::keyboard::Key::Exit,
            pancurses::Input::KeySFind => antigen::core::keyboard::Key::Find,
            pancurses::Input::KeySHome => antigen::core::keyboard::Key::Home,
            pancurses::Input::KeySIC => antigen::core::keyboard::Key::Insert,
            pancurses::Input::KeySLeft => antigen::core::keyboard::Key::Left,
            pancurses::Input::KeySMessage => antigen::core::keyboard::Key::Message,
            pancurses::Input::KeySMove => antigen::core::keyboard::Key::Move,
            pancurses::Input::KeySNext => antigen::core::keyboard::Key::Next,
            pancurses::Input::KeySOptions => antigen::core::keyboard::Key::Options,
            pancurses::Input::KeySPrevious => antigen::core::keyboard::Key::Previous,
            pancurses::Input::KeySPrint => antigen::core::keyboard::Key::Print,
            pancurses::Input::KeySRedo => antigen::core::keyboard::Key::Redo,
            pancurses::Input::KeySReplace => antigen::core::keyboard::Key::Replace,
            pancurses::Input::KeySRight => antigen::core::keyboard::Key::Right,
            pancurses::Input::KeySResume => antigen::core::keyboard::Key::Resume,
            pancurses::Input::KeySSave => antigen::core::keyboard::Key::Save,
            pancurses::Input::KeySSuspend => antigen::core::keyboard::Key::Suspend,
            pancurses::Input::KeySUndo => antigen::core::keyboard::Key::Undo,
            pancurses::Input::KeySuspend => antigen::core::keyboard::Key::Suspend,
            pancurses::Input::KeyUndo => antigen::core::keyboard::Key::Undo,
            pancurses::Input::KeyA1 => unsupported_input(input),
            pancurses::Input::KeyA3 => unsupported_input(input),
            pancurses::Input::KeyB2 => unsupported_input(input),
            pancurses::Input::KeyC1 => unsupported_input(input),
            pancurses::Input::KeyC3 => unsupported_input(input),
            pancurses::Input::KeyResize => antigen::core::keyboard::Key::Unknown,
            pancurses::Input::KeyEvent => antigen::core::keyboard::Key::Unknown,
            pancurses::Input::KeyMouse => antigen::core::keyboard::Key::Unknown,
            pancurses::Input::KeyCodeYes => antigen::core::keyboard::Key::Unknown,
            pancurses::Input::Unknown(_) => unsupported_input(input),
        }
    }
}

fn unsupported_input(input: pancurses::Input) -> antigen::core::keyboard::Key {
    eprintln!("Unsupported pancurses input type: {:?}", input);
    antigen::core::keyboard::Key::Unknown
}
