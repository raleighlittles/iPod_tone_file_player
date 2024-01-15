#[derive(Debug, PartialEq)]
enum PlaybackDirection {
    PiezoBeeper,
    DefaultSoundDevice
}


/// Boilerplate for egui
pub struct MyApp {

    tone_file_contents : String,

    playback_radio_btn : PlaybackDirection

}

impl Default for MyApp {

    fn default() -> Self {
        Self {
            tone_file_contents : "".to_string(),
            playback_radio_btn : PlaybackDirection::DefaultSoundDevice
        }
    }
}



impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("iPod Tone file player");

            ui.vertical( |vert_ui| {
                vert_ui.label("Enter Tone file text");
                vert_ui.text_edit_multiline(&mut self.tone_file_contents);

            }); // end vertical UI

            ui.end_row();

            ui.horizontal( |horiz_ui| {
                horiz_ui.radio_value(&mut self.playback_radio_btn, PlaybackDirection::PiezoBeeper, "Motherboard piezo beeper");
                horiz_ui.radio_value(&mut self.playback_radio_btn, PlaybackDirection::DefaultSoundDevice, "Default sound device (speakers/headphones");
            }); // end horizontal UI

            ui.end_row();

            let play_btn_icon = egui::include_image!("../assets/circle-play-regular.svg");

            if ui.add(egui::Button::image_and_text(play_btn_icon, "Play")).clicked() {
                
                println!("Play button clicked");
            }

            ui.end_row();



        }); // end `show()`

    }

}