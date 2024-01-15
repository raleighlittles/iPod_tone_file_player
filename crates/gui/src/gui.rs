use ipod_tone_player_utils::sound_helpers::play_sound_aloud;

#[derive(Debug, PartialEq)]


/// Boilerplate for egui
pub struct MyApp {

    tone_file_contents : String,
}

impl Default for MyApp {

    fn default() -> Self {
        Self {
            tone_file_contents : "".to_string()
        }
    }
}



impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {

            // ui.heading("iPod Tone file player");

            ui.vertical( |vert_ui| {
                vert_ui.label("Format: <Frequency in Hertz> <Duration in milliseconds>");
                vert_ui.text_edit_multiline(&mut self.tone_file_contents);

            }); // end vertical UI

            ui.end_row();



            let play_btn_icon = egui::include_image!("../assets/circle-play-regular.svg");

            if ui.add(egui::Button::image_and_text(play_btn_icon, "Play")).clicked() {

                // Don't move this code into the shared section; we only want to call it once the text has been finalized, not every time the
                // GUI updates
                let (_, users_sounds) = ipod_tone_player_utils::sound_helpers::parse_sounds_from_tone_file(self.tone_file_contents.clone());
                
                println!("Play button clicked. Playing sound aloud");
                    
                play_sound_aloud(&users_sounds);


            } // end Play button


            let download_to_file_icon = egui::include_image!("../assets/download-solid.svg");

            if ui.add(egui::Button::image_and_text(download_to_file_icon, "Save to file")).clicked() {

                // Don't move this code into the shared section; we only want to call it once the text has been finalized, not every time the
                // GUI updates
                let (tone_name, users_sounds) = ipod_tone_player_utils::sound_helpers::parse_sounds_from_tone_file(self.tone_file_contents.clone());

                // TODO Give the user the ability to choose where the want to save the file to
                let tone_filename = tone_name + ".wav";

                println!("Saving tone to file '{}'", tone_filename);

                ipod_tone_player_utils::sound_helpers::write_audio_to_wav(&users_sounds, &tone_filename);
            }



            ui.end_row();



        }); // end `show()`

    }

}