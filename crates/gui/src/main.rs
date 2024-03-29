mod gui;

fn main() {

    println!("Launching GUI");

    let egui_options = eframe::NativeOptions {
   
        viewport: egui::ViewportBuilder::default().with_inner_size([/* width */ 400.0, /* height */ 500.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "iPod Tone file player",
        egui_options,
        Box::new(|cc | {
            // This gives us image support
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // Enable light theme
            let _ = &cc.egui_ctx.set_visuals(egui::Visuals::light());

            return Box::<gui::MyApp>::new(gui::MyApp::default());
        }),
    );

    println!("Terminating GUI");


}