
fn main() {
    let tone_filename: String = std::env::args()
        .nth(1)
        .expect("Missing first argument - tone file");

    let tone_filepath = std::path::Path::new(&tone_filename);

    if !tone_filepath.exists() {
        panic!("Tone file path '{}' doesn't exist", tone_filename);
    }

    let tone_file_text: String = std::fs::read_to_string(tone_filepath).unwrap();

    let tone_sounds = ipod_tone_player_utils::sound_helpers::parse_sounds_from_tone_file(tone_file_text);

    let output_direction_arg : String = std::env::args().nth(2).expect("Missing second argument! Either '--file' to output to a wav file or '--sound' to play the sound directly.");

    if output_direction_arg == "--file" {
        let output_filename: String = tone_filename.clone() + ".wav";

        //println!("Writing audio to '{}'", output_filename);

        let _ = ipod_tone_player_utils::sound_helpers::write_audio_to_wav(tone_sounds, &output_filename);
    } else if output_direction_arg == "--sound" {
        ipod_tone_player_utils::sound_helpers::play_sound_aloud(tone_sounds);
    } else {
        panic!("'{}' is not a known option for this program", output_direction_arg);
    }
}
