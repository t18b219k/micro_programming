use eframe::egui::mutex::RwLock;
use eframe::egui::{Response, Ui};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Hexadecimal input.
///
///
pub struct HexInput<'a> {
    tgt: &'a mut u8,
    key: usize,
}
impl<'a> HexInput<'a> {
    pub fn new(target: &'a mut u8, key: usize) -> Self {
        Self { tgt: target, key }
    }
}

static KEY: once_cell::sync::Lazy<AtomicUsize> = once_cell::sync::Lazy::new(|| AtomicUsize::new(0));
static BUFFER: once_cell::sync::Lazy<RwLock<String>> =
    once_cell::sync::Lazy::new(|| RwLock::new(String::new()));

impl<'a> eframe::egui::Widget for HexInput<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut text_buffer = if self.key == KEY.load(Ordering::Relaxed) {
            let buffer = BUFFER.read().clone();
            if buffer != self.tgt.to_string() {
                format!("{:02X}", self.tgt)
            } else {
                buffer
            }
        } else {
            format!("{:02X}", self.tgt)
        };

        let text_edit = eframe::egui::TextEdit::singleline(&mut text_buffer).desired_width(16.0);

        let response = ui.add(text_edit);

        if response.has_focus() {
            if let Ok(v) = u8::from_str_radix(&text_buffer, 16) {
                *self.tgt = v;
                *BUFFER.write() = text_buffer;
                KEY.store(self.key, Ordering::SeqCst);
            }
        }
        response
    }
}
