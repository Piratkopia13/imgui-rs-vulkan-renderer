mod common;

use common::*;
use imgui::*;
use simple_logger::SimpleLogger;
use std::error::Error;

const APP_NAME: &str = "long list";

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init()?;

    let mut stable_str = String::new();
    let mut callback_str = String::new();

    System::new(APP_NAME)?.run((), move |_, ui, _| {
        if let Some(_window) = ui
            .window("Input text callbacks")
            .size([500.0, 300.0], Condition::FirstUseEver)
            .begin()
        {
            if ui.input_text("input stable", &mut stable_str).build() {
                dbg!(&stable_str);
            }

            let mut per_frame_buf = String::new();
            ui.input_text("input per frame", &mut per_frame_buf).build();

            if ui.is_item_deactivated_after_edit() {
                dbg!(&per_frame_buf);
            }

            struct CB;
            impl imgui::InputTextCallbackHandler for CB {
                fn on_history(
                    &mut self,
                    _dir: imgui::HistoryDirection,
                    _data: imgui::TextCallbackData,
                ) {
                }
            }
            let changed = ui
                .input_text("input callback", &mut callback_str)
                .callback(InputTextCallback::HISTORY, CB)
                .enter_returns_true(true)
                .build();

            if changed {
                println!("{:?}", callback_str);
            }
        }
    })?;

    Ok(())
}
