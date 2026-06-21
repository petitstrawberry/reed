//! Reed — a lightweight text editor built with ScarletUI.

use std::env;
use std::fs;

use scarlet_ui::ComponentElement;
use scarlet_ui::event::{KeyCode, KeyEvent};
use scarlet_ui::prelude::*;

#[derive(Clone)]
struct Reed {
    text: State<String>,
    selection: State<TextSelection>,
    scroll: State<TextViewScroll>,
    file_path: Option<String>,
}

impl Reed {
    fn new(file_path: Option<String>, initial_text: String) -> Self {
        Self {
            text: State::new(StateId::new(1), initial_text),
            selection: State::new(StateId::new(2), TextSelection::collapsed(0)),
            scroll: State::new(StateId::new(3), TextViewScroll::default()),
            file_path,
        }
    }

    fn title(&self) -> String {
        match &self.file_path {
            Some(path) => format!("Reed — {}", path),
            None => String::from("Reed — untitled"),
        }
    }

    fn content(&self) -> impl View + Clone + use<> {
        let text = self.text.clone();
        let path = self.file_path.clone();

        TextView::new(self.text.clone(), self.selection.clone())
            .scroll_state(self.scroll.clone())
            .placeholder("Start typing...")
            .font_size(14.0)
            .padding(16.0)
            .line_numbers(true)
            .current_line_highlight(true)
            .border_color(Color::rgba(0.0, 0.0, 0.0, 0.0))
            .focused_border_color(Color::rgba(0.0, 0.0, 0.0, 0.0))
            .on_key(move |event| {
                if let KeyEvent::Pressed {
                    keycode: KeyCode::Char('s'),
                    modifiers,
                } = event
                {
                    if modifiers.primary() {
                        if let Some(path) = &path {
                            let content = text.get();
                            let _ = fs::write(path, content.as_str());
                        }
                        return true;
                    }
                }
                false
            })
    }
}

impl View for Reed {
    fn create_element(&self) -> Box<dyn Element> {
        Box::new(ComponentElement::new_with_builder(
            self.clone(),
            |reed: &Reed| reed.content().create_element(),
        ))
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

impl Application for Reed {
    fn scenes(&self) -> impl Scene {
        WindowGroup::new(
            "main",
            Window::new(self.title(), self.content()).size(Size::new(800.0, 600.0)),
        )
    }
}

fn main() -> scarlet_ui::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).map(|s| s.to_string());

    let initial_text = match &file_path {
        Some(path) => fs::read_to_string(path)
            .unwrap_or_else(|_| format!("# Reed\n\nCould not open: {}\n\nStart typing...\n", path)),
        None => String::from("# Reed\n\nA lightweight text editor.\n\nStart typing...\n"),
    };

    let mut app = Reed::new(file_path, initial_text);
    app.run()
}
