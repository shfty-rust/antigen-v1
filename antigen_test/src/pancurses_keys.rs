pub struct PancursesInput(pancurses::Input);

impl From<pancurses::Input> for PancursesInput {
    fn from(input: pancurses::Input) -> Self {
        PancursesInput(input)
    }
}

impl antigen::IntoKey for PancursesInput {
    fn into_key(self) -> antigen::Key {
        let PancursesInput(input) = self;
        match input {
            pancurses::Input::Character(char) => match char {
                ' ' => antigen::Key::Space,
                '!' => antigen::Key::ExclamationMark,
                '"' => antigen::Key::DoubleQuote,
                '#' => antigen::Key::Number,
                '$' => antigen::Key::Dollar,
                '%' => antigen::Key::Percent,
                '&' => antigen::Key::Ampersand,
                '\'' => antigen::Key::SingleQuote,
                '(' => antigen::Key::OpenBracket,
                ')' => antigen::Key::CloseBracket,
                '*' => antigen::Key::Asterisk,
                '+' => antigen::Key::Plus,
                ',' => antigen::Key::Comma,
                '-' => antigen::Key::Hyphen,
                '.' => antigen::Key::Period,
                '/' => antigen::Key::Slash,
                '0' => antigen::Key::N0,
                '1' => antigen::Key::N1,
                '2' => antigen::Key::N2,
                '3' => antigen::Key::N3,
                '4' => antigen::Key::N4,
                '5' => antigen::Key::N5,
                '6' => antigen::Key::N6,
                '7' => antigen::Key::N7,
                '8' => antigen::Key::N8,
                '9' => antigen::Key::N9,
                ':' => antigen::Key::Colon,
                ';' => antigen::Key::Semicolon,
                '<' => antigen::Key::LessThan,
                '=' => antigen::Key::Equals,
                '>' => antigen::Key::GreaterThan,
                '?' => antigen::Key::QuestionMark,
                '@' => antigen::Key::At,
                'A' => antigen::Key::A,
                'B' => antigen::Key::B,
                'C' => antigen::Key::C,
                'D' => antigen::Key::D,
                'E' => antigen::Key::E,
                'F' => antigen::Key::F,
                'G' => antigen::Key::G,
                'H' => antigen::Key::H,
                'I' => antigen::Key::I,
                'J' => antigen::Key::J,
                'K' => antigen::Key::K,
                'L' => antigen::Key::L,
                'M' => antigen::Key::M,
                'N' => antigen::Key::N,
                'O' => antigen::Key::O,
                'P' => antigen::Key::P,
                'Q' => antigen::Key::Q,
                'R' => antigen::Key::R,
                'S' => antigen::Key::S,
                'T' => antigen::Key::T,
                'U' => antigen::Key::U,
                'V' => antigen::Key::V,
                'W' => antigen::Key::W,
                'X' => antigen::Key::X,
                'Y' => antigen::Key::Y,
                'Z' => antigen::Key::Z,
                '[' => antigen::Key::OpeningBracket,
                '\\' => antigen::Key::Backslash,
                ']' => antigen::Key::ClosingBracket,
                '^' => antigen::Key::Caret,
                '_' => antigen::Key::Underscore,
                '`' => antigen::Key::Grave,
                'a' => antigen::Key::A,
                'b' => antigen::Key::B,
                'c' => antigen::Key::C,
                'd' => antigen::Key::D,
                'e' => antigen::Key::E,
                'f' => antigen::Key::F,
                'g' => antigen::Key::G,
                'h' => antigen::Key::H,
                'i' => antigen::Key::I,
                'j' => antigen::Key::J,
                'k' => antigen::Key::K,
                'l' => antigen::Key::L,
                'm' => antigen::Key::M,
                'n' => antigen::Key::N,
                'o' => antigen::Key::O,
                'p' => antigen::Key::P,
                'q' => antigen::Key::Q,
                'r' => antigen::Key::R,
                's' => antigen::Key::S,
                't' => antigen::Key::T,
                'u' => antigen::Key::U,
                'v' => antigen::Key::V,
                'w' => antigen::Key::W,
                'x' => antigen::Key::X,
                'y' => antigen::Key::Y,
                'z' => antigen::Key::Z,
                '{' => antigen::Key::OpeningBrace,
                '|' => antigen::Key::VerticalBar,
                '}' => antigen::Key::ClosingBrace,
                '~' => antigen::Key::Tilde,
                '\u{8}' => antigen::Key::Backspace,
                '\u{1b}' => antigen::Key::Escape,
                '\u{ed2c}' => antigen::Key::Menu,
                '\u{ed2d}' => antigen::Key::Menu,
                '\u{ed35}' => antigen::Key::ScrollLock,
                '\u{ed30}' => antigen::Key::Pause,
                '\u{ed12}' => antigen::Key::Enter,
                '\u{ecc5}' => antigen::Key::Center,
                '\u{ec18}' => antigen::Key::F16,
                '\u{ec19}' => antigen::Key::F17,
                '\u{ec1a}' => antigen::Key::F18,
                '\u{ec1b}' => antigen::Key::F19,
                '\u{ec1c}' => antigen::Key::F20,
                '\u{ec1d}' => antigen::Key::F21,
                '\u{ec1e}' => antigen::Key::F22,
                '\u{ec1f}' => antigen::Key::F23,
                '\u{ec20}' => antigen::Key::F24,
                '\t' => antigen::Key::Tab,
                '\n' => antigen::Key::Enter,
                _ => unsupported_input(input),
            },
            pancurses::Input::KeyBreak => antigen::Key::Break,
            pancurses::Input::KeyDown => antigen::Key::Down,
            pancurses::Input::KeyUp => antigen::Key::Up,
            pancurses::Input::KeyLeft => antigen::Key::Left,
            pancurses::Input::KeyRight => antigen::Key::Right,
            pancurses::Input::KeyHome => antigen::Key::Home,
            pancurses::Input::KeyBackspace => antigen::Key::Backspace,
            pancurses::Input::KeyF0 => antigen::Key::F0,
            pancurses::Input::KeyF1 => antigen::Key::F1,
            pancurses::Input::KeyF2 => antigen::Key::F2,
            pancurses::Input::KeyF3 => antigen::Key::F3,
            pancurses::Input::KeyF4 => antigen::Key::F4,
            pancurses::Input::KeyF5 => antigen::Key::F5,
            pancurses::Input::KeyF6 => antigen::Key::F6,
            pancurses::Input::KeyF7 => antigen::Key::F7,
            pancurses::Input::KeyF8 => antigen::Key::F8,
            pancurses::Input::KeyF9 => antigen::Key::F9,
            pancurses::Input::KeyF10 => antigen::Key::F10,
            pancurses::Input::KeyF11 => antigen::Key::F11,
            pancurses::Input::KeyF12 => antigen::Key::F12,
            pancurses::Input::KeyF13 => antigen::Key::F13,
            pancurses::Input::KeyF14 => antigen::Key::F14,
            pancurses::Input::KeyF15 => antigen::Key::F15,
            pancurses::Input::KeyDL => antigen::Key::DeleteLine,
            pancurses::Input::KeyIL => antigen::Key::InsertLine,
            pancurses::Input::KeyDC => antigen::Key::Delete,
            pancurses::Input::KeyIC => antigen::Key::Insert,
            pancurses::Input::KeyEIC => antigen::Key::ExitInsertCharacter,
            pancurses::Input::KeyClear => antigen::Key::Clear,
            pancurses::Input::KeyEOS => antigen::Key::EndOfScreen,
            pancurses::Input::KeyEOL => antigen::Key::EndOfLine,
            pancurses::Input::KeySF => antigen::Key::ScrollForward,
            pancurses::Input::KeySR => antigen::Key::ScrollReverse,
            pancurses::Input::KeyNPage => antigen::Key::PageDown,
            pancurses::Input::KeyPPage => antigen::Key::PageUp,
            pancurses::Input::KeySTab => antigen::Key::SetTab,
            pancurses::Input::KeyCTab => antigen::Key::ClearTab,
            pancurses::Input::KeyCATab => antigen::Key::ClearAllTabs,
            pancurses::Input::KeyEnter => antigen::Key::Enter,
            pancurses::Input::KeySReset => antigen::Key::SoftReset,
            pancurses::Input::KeyReset => antigen::Key::Reset,
            pancurses::Input::KeyPrint => antigen::Key::Print,
            pancurses::Input::KeyLL => antigen::Key::LastLine,
            pancurses::Input::KeyAbort => antigen::Key::Abort,
            pancurses::Input::KeySHelp => antigen::Key::Help,
            pancurses::Input::KeyLHelp => antigen::Key::LHelp,
            pancurses::Input::KeyBTab => antigen::Key::BackTab,
            pancurses::Input::KeyBeg => antigen::Key::Beginning,
            pancurses::Input::KeyCancel => antigen::Key::Cancel,
            pancurses::Input::KeyClose => antigen::Key::Close,
            pancurses::Input::KeyCommand => antigen::Key::Command,
            pancurses::Input::KeyCopy => antigen::Key::Copy,
            pancurses::Input::KeyCreate => antigen::Key::Create,
            pancurses::Input::KeyEnd => antigen::Key::End,
            pancurses::Input::KeyExit => antigen::Key::Exit,
            pancurses::Input::KeyFind => antigen::Key::Find,
            pancurses::Input::KeyHelp => antigen::Key::Help,
            pancurses::Input::KeyMark => antigen::Key::Mark,
            pancurses::Input::KeyMessage => antigen::Key::Message,
            pancurses::Input::KeyMove => antigen::Key::Move,
            pancurses::Input::KeyNext => antigen::Key::Next,
            pancurses::Input::KeyOpen => antigen::Key::Open,
            pancurses::Input::KeyOptions => antigen::Key::Options,
            pancurses::Input::KeyPrevious => antigen::Key::Previous,
            pancurses::Input::KeyRedo => antigen::Key::Redo,
            pancurses::Input::KeyReference => antigen::Key::Reference,
            pancurses::Input::KeyRefresh => antigen::Key::Refresh,
            pancurses::Input::KeyReplace => antigen::Key::Replace,
            pancurses::Input::KeyRestart => antigen::Key::Restart,
            pancurses::Input::KeyResume => antigen::Key::Resume,
            pancurses::Input::KeySave => antigen::Key::Save,
            pancurses::Input::KeySBeg => antigen::Key::Beginning,
            pancurses::Input::KeySCancel => antigen::Key::Cancel,
            pancurses::Input::KeySCommand => antigen::Key::Command,
            pancurses::Input::KeySCopy => antigen::Key::Copy,
            pancurses::Input::KeySCreate => antigen::Key::Create,
            pancurses::Input::KeySDC => antigen::Key::Delete,
            pancurses::Input::KeySDL => antigen::Key::DeleteLine,
            pancurses::Input::KeySelect => antigen::Key::Select,
            pancurses::Input::KeySEnd => antigen::Key::End,
            pancurses::Input::KeySEOL => antigen::Key::EndOfLine,
            pancurses::Input::KeySExit => antigen::Key::Exit,
            pancurses::Input::KeySFind => antigen::Key::Find,
            pancurses::Input::KeySHome => antigen::Key::Home,
            pancurses::Input::KeySIC => antigen::Key::Insert,
            pancurses::Input::KeySLeft => antigen::Key::Left,
            pancurses::Input::KeySMessage => antigen::Key::Message,
            pancurses::Input::KeySMove => antigen::Key::Move,
            pancurses::Input::KeySNext => antigen::Key::Next,
            pancurses::Input::KeySOptions => antigen::Key::Options,
            pancurses::Input::KeySPrevious => antigen::Key::Previous,
            pancurses::Input::KeySPrint => antigen::Key::Print,
            pancurses::Input::KeySRedo => antigen::Key::Redo,
            pancurses::Input::KeySReplace => antigen::Key::Replace,
            pancurses::Input::KeySRight => antigen::Key::Right,
            pancurses::Input::KeySResume => antigen::Key::Resume,
            pancurses::Input::KeySSave => antigen::Key::Save,
            pancurses::Input::KeySSuspend => antigen::Key::Suspend,
            pancurses::Input::KeySUndo => antigen::Key::Undo,
            pancurses::Input::KeySuspend => antigen::Key::Suspend,
            pancurses::Input::KeyUndo => antigen::Key::Undo,
            pancurses::Input::KeyA1 => unsupported_input(input),
            pancurses::Input::KeyA3 => unsupported_input(input),
            pancurses::Input::KeyB2 => unsupported_input(input),
            pancurses::Input::KeyC1 => unsupported_input(input),
            pancurses::Input::KeyC3 => unsupported_input(input),
            pancurses::Input::KeyResize => antigen::Key::Unknown,
            pancurses::Input::KeyEvent => antigen::Key::Unknown,
            pancurses::Input::KeyMouse => antigen::Key::Unknown,
            pancurses::Input::KeyCodeYes => antigen::Key::Unknown,
            pancurses::Input::Unknown(_) => unsupported_input(input),
        }
    }
}

fn unsupported_input(input: pancurses::Input) -> antigen::Key {
    eprintln!("Unsupported pancurses input type: {:?}", input);
    antigen::Key::Unknown
}