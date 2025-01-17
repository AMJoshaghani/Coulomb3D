/*
        In this file, all front-ends and back-ends come together to form the App. for math and algebra
    you need to check the algebra.rs file and all physical calculations are in physics.rs. All those
    workers are placed as a mod to this file, and are indeed included in math/mod.rs, and therefore
    they are added here.
 */
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};
mod math;
use math::*;

// Global Variables
const WINDOW: (i32, i32) = (1280, 720); // Customizing window dimensions
#[derive(Debug)]
struct Charge {
    charge: f64, // measured in Coulomb
    position: (f32, f32, f32), // in meters
    color: (f32, f32, f32, f32), // rgba
}
// Error dialog definitions:
struct ErrorDialog {
    rect: Rect,          // Position and size of the dialog
    message: String,     // Error message
    ok_button: Rect,     // "OK" button position and size
    is_open: bool,       // Whether the dialog is open
}

impl ErrorDialog {
    fn new() -> Self {
        let dialog_width = 400.0;
        let dialog_height = 200.0;
        let screen_center_x = screen_width() / 2.0;
        let screen_center_y = screen_height() / 2.0;

        Self {
            rect: Rect::new(
                screen_center_x - dialog_width / 2.0,
                screen_center_y - dialog_height / 2.0,
                dialog_width,
                dialog_height,
            ),
            message: String::new(),
            ok_button: Rect::new(
                screen_center_x - 50.0,
                screen_center_y + 50.0,
                100.0,
                40.0,
            ),
            is_open: false,
        }
    }

    fn show(&mut self, message: &str) {
        self.message = message.to_string();
        self.is_open = true;
    }

    fn update(&mut self) -> bool {
        if self.is_open {
            if is_mouse_button_pressed(MouseButton::Left) {
                let mouse_pos = mouse_position();
                if self.ok_button.contains(mouse_pos.into()) {
                    // Close the dialog when "OK" is clicked
                    self.is_open = false;
                    return true; // Dialog was closed
                }
            }
        }
        false // Dialog is still open
    }

    fn draw(&self) {
        if self.is_open {
            // Draw the dialog background
            draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, LIGHTGRAY);

            // Draw the dialog border
            draw_rectangle_lines(self.rect.x, self.rect.y, self.rect.w, self.rect.h, 2.0, DARKGRAY);

            // Draw the error message
            let text_size = measure_text(&self.message, None, 24, 1.0);
            let text_x = self.rect.x + (self.rect.w - text_size.width) / 5.0 + 80.;
            let text_y = self.rect.y + 50.0;
            draw_multiline_text(&self.message, text_x, text_y, 20.0, Some(1.), BLACK);

            // Draw the "OK" button
            draw_rectangle(self.ok_button.x, self.ok_button.y, self.ok_button.w, self.ok_button.h, BLUE);
            draw_text(
                "OK",
                self.ok_button.x + 35.0,
                self.ok_button.y + 25.0,
                24.0,
                WHITE,
            );
        }
    }
}
// window properties
fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Coulomb3D"),
        fullscreen: false,
        window_resizable: false,
        window_width: WINDOW.0,
        window_height: WINDOW.1,
        sample_count: 8,
        // high_dpi: true,
        ..Default::default()
    }
}

// main UI function
#[macroquad::main(window_conf)]
async fn main() {
    const FONT_BYTES: &[u8] = include_bytes!("static/Lato-Light.ttf");
    // let font = load_ttf_font_from_bytes(FONT_BYTES).unwrap();
    // let font = load_file("src/static/Lato-Light.ttf").await.unwrap(); // custom font

    // Camera parameters
    let mut camera_yaw = 0.0; // Horizontal rotation
    let mut camera_pitch = 0.0; // Vertical rotation
    let mut camera_distance = 10.0; // Distance from the target
    let target = Vec3::new(0.0, 3.0, 0.0); // Camera target (e.g., a character)

    // Mouse drag state
    let mut is_dragging = false;
    let mut last_mouse_pos = Vec2::ZERO;

    // charge settings
    let mut charges: Vec<Charge> = Vec::new();
    let mut reference_string: String = String::from("(0,0,0)");
    let mut reference = (0.0, 0.0, 0.0);
    let mut charge_value: String = String::from("0.00001");
    let mut charge_position: String = String::from("(1,1,1)");

    // results
    let mut phi: String; // Electric potential
    let mut p: String; // Electric Dipole Moment
    let mut e: String; // Electric Field

    // error dialog
    let mut error_dialog = ErrorDialog::new();
    let mut show_error = false;
    let mut err_text: String = String::new();

    // style configurations
    let window_style = root_ui()
        .style_builder()
        .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
        .build();

    let label_style = root_ui()
        .style_builder()
        .font(&FONT_BYTES)
        .unwrap()
        // .text_color(DARKGRAY)
        .font_size(13)
        .build();

    let button_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
        .margin(RectOffset::new(16.0, 16.0, -8.0, -8.0))
        .color(LIGHTGRAY)
        .font(&FONT_BYTES)
        .unwrap()
        .text_color(BLACK)
        .font_size(12)
        .build();

    let ui_skin = Skin {
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);

    // main program loop
    loop {
        clear_background(LIGHTGRAY);

        // Do the calculations
        if !charges.is_empty() {
            // Doing the calculations on every frame, todo!("optimize")
            let _p = electric_dipole_moment(&charges, &reference);
            let _e = electric_field(&charges, &reference);
            let _phi = electric_potential(&charges, &reference);
            //
            phi = String::from(&format!("Phi =\n{:}", _phi));
            p = String::from(&format!("P =\n{:}\n|p| = {:}", _p, _p.magnitude()));
            e = String::from(&format!("E =\n{:.}\n|E| = {:.}", _e, _e.magnitude()));
        } else {
            // If no charge was present in configuration:
            phi = String::from("Not Calculated");
            p = String::from("Not Calculated");
            e = String::from("Not Calculated");
        }

        // Camera
        // Handle mouse input for dragging
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position().into();
            if !is_dragging {
                // Start dragging
                is_dragging = true;
                last_mouse_pos = mouse_pos;
            } else {
                // Calculate mouse delta
                let delta = mouse_pos - last_mouse_pos;
                last_mouse_pos = mouse_pos;

                // Update camera yaw and pitch based on mouse delta
                camera_yaw -= delta.x * 0.01; // Adjust sensitivity as needed
                camera_pitch -= delta.y * 0.01; // Adjust sensitivity as needed

                // Clamp pitch to avoid flipping the camera
                camera_pitch = camera_pitch.clamp(-1.5, 1.5);
            }
        } else {
            // Stop dragging
            is_dragging = false;
        }

        // Handle mouse wheel for zooming
        let (_, mouse_wheel_y) = mouse_wheel();
        camera_distance -= mouse_wheel_y * 0.5; // Adjust zoom speed as needed
        camera_distance = camera_distance.clamp(2.0, 20.0); // Clamp distance to a reasonable range

        // Calculate camera position
        let camera_position = Vec3::new(
            camera_distance * camera_yaw.cos() * camera_pitch.cos(),
            camera_distance * camera_pitch.sin(),
            camera_distance * camera_yaw.sin() * camera_pitch.cos(),
        ) + target;

        // Set up the camera
        set_camera(&Camera3D {
            position: camera_position,
            target,
            up: Vec3::new(0.0, 1.0, 0.0),
            ..Default::default()
        });

        // Objects
        draw_grid(20, 1., WHITE, WHITE); // fixed-on-view grid
        draw_sphere(vec3(reference.0, reference.1, reference.2), 0.1, None, BLACK);
        // Drawing charges on the grid:
        for charge in &mut charges {
            let p: (f32, f32, f32) = charge.position;
            let c: (f32, f32, f32, f32) = charge.color;
            draw_sphere(
                vec3(p.0, p.1, p.2),
                charge_to_radius(charge.charge) as f32,
                None,
                Color::new(c.0, c.1, c.2, c.3)
            );
        }
        for i in 0..charges.len() {
            for j in (i + 1)..charges.len() {
                let p1: (f32, f32, f32) = charges[i].position;
                let p2: (f32, f32, f32) = charges[j].position;
                draw_line_3d(
                    vec3(p1.0, p1.1, p1.2),
                    vec3(p2.0, p2.1, p2.2),
                    Color::new(1.0, 0.3, 0.5, 0.9)
                );
            }
        }

        // Widgets
        root_ui().label(vec2(1065., 20.), "Settings & Configurations");
        root_ui().window(
            hash!(),
            vec2(screen_width() / 2.0 + WINDOW.0 as f32 / 3.5 - 50.0, 40.),
            vec2(WINDOW.0 as f32 / 5. + 50.0, WINDOW.1 as f32 / 5. + 100.0),
            |ui| {
                ui.input_text(hash!(), ": r' (Reference point)", &mut reference_string);
                if ui.button(None, "Set"){
                    if let Err(e) = string_to_tuple(&reference_string){
                        err_text = String::from(e);
                        show_error = true;
                    } else {
                        reference = string_to_tuple(&reference_string).unwrap();
                    };
                }
                ui.separator(); ui.separator();
                ui.label(None, "** Add Charges to Configuration:");
                ui.input_text(hash!(), "Charge value (in C)", &mut charge_value);
                ui.input_text(hash!(), "Charge position", &mut charge_position);
                if ui.button(None, "add charge") {
                    let p = string_to_tuple(&charge_position);
                    if !charge_value.parse::<f32>().is_ok() {
                        err_text = String::from("Charge value is not a number");
                        show_error = true;
                    } else if let Err(e) = p {
                        err_text = String::from(e);
                        show_error = true;
                    } else if !is_position_unique(&charge_position, &charges) {
                        err_text = String::from("Position is not unique");
                        show_error = true;
                    } else {
                        let position = p.unwrap();
                        charges.push(Charge { charge: charge_value.parse::<f64>().unwrap(), position, color: generate_random_rgba() })
                    }
                };
                ui.separator(); ui.separator();
                ui.label(None, "** Charges:");

                // List of charges:
                let mut i = 1;
                for charge in &charges {
                    ui.label(None, &format!("{i}- c: {:.?}, p: {:?}", charge.charge, charge.position));
                    i += 1;
                }
                if charges.is_empty(){
                    ui.separator();
                    ui.label(None, "No charge has been defined yet.");
                    ui.separator();
                } else {
                    if ui.button(None, "Reset") {
                        charges.clear(); // removes all charges
                    };
                }
            },
        );
        root_ui().label(vec2(75., 20.), "Calculations & Properties");
        root_ui().window(
            hash!(),
            vec2(20., 40.),
            vec2(WINDOW.0 as f32 / 5., WINDOW.1 as f32 / 2.),
            |ui| {
                ui.label(None, "** Electrical Dipole Moment (N/C):");
                ui.editbox(hash!(), vec2(WINDOW.0 as f32 / 5. - 6., 90.), &mut p);
                ui.separator(); ui.separator();
                ui.label(None, "** Electric Potential (V):");
                ui.editbox(hash!(), vec2(WINDOW.0 as f32 / 5. - 6., 50.), &mut phi);
                ui.separator(); ui.separator();
                ui.label(None,  "** Electric Field (C.m):");
                ui.editbox(hash!(), vec2(WINDOW.0 as f32 / 5. - 6., 90.), &mut e);
                ui.separator(); ui.separator();
            },
        );

        // Screen Properties
        set_default_camera();
        root_ui().label(vec2((WINDOW.0 / 2) as f32 - 60f32, 20.0), "Coulomb3D (v0.0.1)");


        // Show the error dialog if needed
        if show_error {
            error_dialog.show(&format!("{}:\nplease check your input.", err_text));
            show_error = false; // Reset the flag
        }

        // Update and draw the error dialog
        if error_dialog.update() {}
        error_dialog.draw();


        next_frame().await
    }
}
