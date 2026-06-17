/*
In this file, all front-ends and back-ends come together to form the App. For math and algebra
you need to check the algebra.rs file and all physical calculations are in physics.rs. All those
workers are placed as a mod to this file, and are indeed included in math/mod.rs, and therefore
they are added here.
*/
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};
mod math;
use math::*;

// Global Variables
const WINDOW: (i32, i32) = (1280, 720);

#[derive(Debug)]
struct Charge {
    charge: f64,
    position: (f32, f32, f32),
    color: (f32, f32, f32, f32),
}

// ── Error dialog ─────────────────────────────────────────────────────────────

struct ErrorDialog {
    rect: Rect,
    message: String,
    ok_button: Rect,
    is_open: bool,
}

impl ErrorDialog {
    fn new() -> Self {
        let dialog_width  = 400.0;
        let dialog_height = 200.0;
        let cx = screen_width()  / 2.0;
        let cy = screen_height() / 2.0;
        Self {
            rect: Rect::new(cx - dialog_width / 2.0, cy - dialog_height / 2.0, dialog_width, dialog_height),
            message: String::new(),
            ok_button: Rect::new(cx - 50.0, cy + 50.0, 100.0, 40.0),
            is_open: false,
        }
    }

    fn show(&mut self, message: &str) {
        self.message = message.to_string();
        self.is_open = true;
    }

    fn update(&mut self) -> bool {
        if self.is_open && is_mouse_button_pressed(MouseButton::Left) {
            let mp = mouse_position();
            if self.ok_button.contains(mp.into()) {
                self.is_open = false;
                return true;
            }
        }
        false
    }

    fn draw(&self) {
        if !self.is_open { return; }
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, LIGHTGRAY);
        draw_rectangle_lines(self.rect.x, self.rect.y, self.rect.w, self.rect.h, 2.0, DARKGRAY);

        let text_size = measure_text(&self.message, None, 24, 1.0);
        let tx = self.rect.x + (self.rect.w - text_size.width) / 5.0 + 80.0;
        let ty = self.rect.y + 50.0;
        draw_multiline_text(&self.message, tx, ty, 20.0, Some(1.0), BLACK);

        draw_rectangle(self.ok_button.x, self.ok_button.y, self.ok_button.w, self.ok_button.h, BLUE);
        draw_text("OK", self.ok_button.x + 35.0, self.ok_button.y + 25.0, 24.0, WHITE);
    }
}

// ── Bounding cube wireframe ───────────────────────────────────────────────────

/// Draw the 12 edges of an axis-aligned cube centred at the origin with half-size `b`.
fn draw_bounds_cube(b: f32, col: Color) {
    let c = [
        vec3(-b, -b, -b), vec3( b, -b, -b), vec3( b,  b, -b), vec3(-b,  b, -b),
        vec3(-b, -b,  b), vec3( b, -b,  b), vec3( b,  b,  b), vec3(-b,  b,  b),
    ];
    let edges: [(usize, usize); 12] = [
        (0,1),(1,2),(2,3),(3,0), // bottom face
        (4,5),(5,6),(6,7),(7,4), // top face
        (0,4),(1,5),(2,6),(3,7), // verticals
    ];
    for (i, j) in edges {
        draw_line_3d(c[i], c[j], col);
    }
}

// ── Window config ─────────────────────────────────────────────────────────────

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Coulomb3D"),
        fullscreen: false,
        window_resizable: false,
        window_width:  WINDOW.0,
        window_height: WINDOW.1,
        sample_count: 8,
        ..Default::default()
    }
}

// ── Main ──────────────────────────────────────────────────────────────────────

#[macroquad::main(window_conf)]
async fn main() {
    const FONT_BYTES: &[u8] = include_bytes!("static/Lato-Light.ttf");

    // Camera parameters
    let mut camera_yaw:      f32 = 0.0;
    let mut camera_pitch:    f32 = 0.0;
    let mut camera_distance: f32 = 10.0;
    let target = Vec3::new(0.0, 3.0, 0.0);

    let mut is_dragging   = false;
    let mut last_mouse_pos = Vec2::ZERO;

    // Charge configuration
    let mut charges: Vec<Charge> = Vec::new();
    let mut reference_string = String::from("(0,0,0)");
    let mut reference: (f32, f32, f32) = (0.0, 0.0, 0.0);
    let mut charge_value    = String::from("0.00001");
    let mut charge_position = String::from("(1,1,1)");

    // Calculation results
    let mut phi: String;
    let mut p:   String;
    let mut e:   String;

    // Error dialog
    let mut error_dialog = ErrorDialog::new();
    let mut show_error   = false;
    let mut err_text     = String::new();

    // ── Field line state ──────────────────────────────────────────────────────
    // `show_field_lines` toggles the visualisation.
    // `field_lines_dirty` signals that lines must be recomputed next frame.
    let mut show_field_lines: bool = false;
    let mut field_lines: Vec<Vec<(f32, f32, f32)>> = Vec::new();
    let mut field_line_bounds: f32 = 0.0;
    let mut field_lines_dirty: bool = false;

    // ── UI skin ───────────────────────────────────────────────────────────────
    let window_style = root_ui()
        .style_builder()
        .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
        .build();

    let label_style = root_ui()
        .style_builder()
        .font(&FONT_BYTES).unwrap()
        .font_size(13)
        .build();

    let button_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
        .margin(RectOffset::new(16.0, 16.0, -8.0, -8.0))
        .color(LIGHTGRAY)
        .font(&FONT_BYTES).unwrap()
        .text_color(BLACK)
        .font_size(12)
        .build();

    let ui_skin = Skin { window_style, button_style, label_style, ..root_ui().default_skin() };
    root_ui().push_skin(&ui_skin);

    // ── Main loop ─────────────────────────────────────────────────────────────
    loop {
        clear_background(LIGHTGRAY);

        // ── Recompute field lines if the configuration changed ────────────────
        // done once per dirty frame.
        if field_lines_dirty && show_field_lines {
            if charges.is_empty() {
                field_lines.clear();
                field_line_bounds = 0.0;
            } else {
                let (lines, bounds) = generate_field_lines(&charges);
                field_lines      = lines;
                field_line_bounds = bounds;
            }
            field_lines_dirty = false;
        }

        // ── Electrostatic calculations ────────────────────────────────────────
        if !charges.is_empty() {
            let mm   = multipole_moments(&charges, &reference);
            let _e   = electric_field(&charges, &reference);
            let _phi = electric_potential(&charges, &reference);

            let (px, py, pz) = mm.dipole.components();
            let (ex, ey, ez) = _e.components();
            let q = &mm.quadrupole;

            phi = format!("Phi = {:.4e} V", _phi);

            // Multipole panel: monopole + dipole + quadrupole upper triangle
            // (lower triangle mirrors it; Qzz = -Qxx - Qyy by tracelessness)
            p = format!(
                "Q   = {:.4e} C\n\n\
                 p   = ({:.3e},\n      {:.3e},\n      {:.3e}) C·m\n\
                 |p| = {:.4e} C·m\n\n\
                 Quadrupole tensor [C·m²]\n\
                 (traceless: Qxx+Qyy+Qzz=0)\n\
                 Qxx={:.4e}\n\
                 Qyy={:.4e}\n\
                 Qzz={:.4e}\n\
                 Qxy={:.4e}\n\
                 Qxz={:.4e}\n\
                 Qyz={:.4e}",
                mm.monopole,
                px, py, pz, mm.dipole.magnitude(),
                q[0][0], q[1][1], q[2][2],
                q[0][1], q[0][2], q[1][2],
            );

            e = format!(
                "E = ({:.3e},\n     {:.3e},\n     {:.3e}) N/C\n\
                 |E| = {:.4e} N/C",
                ex, ey, ez, _e.magnitude()
            );
        } else {
            phi = String::from("Not Calculated");
            p   = String::from("Not Calculated");
            e   = String::from("Not Calculated");
        }

        // ── Camera drag ───────────────────────────────────────────────────────
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos: Vec2 = mouse_position().into();
            if !is_dragging {
                is_dragging    = true;
                last_mouse_pos = mouse_pos;
            } else {
                let delta  = mouse_pos - last_mouse_pos;
                last_mouse_pos = mouse_pos;
                camera_yaw   -= delta.x * 0.01;
                camera_pitch -= delta.y * 0.01;
                camera_pitch  = camera_pitch.clamp(-1.5, 1.5);
            }
        } else {
            is_dragging = false;
        }

        let (_, wheel_y) = mouse_wheel();
        camera_distance -= wheel_y * 0.5;
        camera_distance  = camera_distance.clamp(2.0, 20.0);

        let camera_position = Vec3::new(
            camera_distance * camera_yaw.cos() * camera_pitch.cos(),
            camera_distance * camera_pitch.sin(),
            camera_distance * camera_yaw.sin() * camera_pitch.cos(),
        ) + target;

        set_camera(&Camera3D {
            position: camera_position,
            target,
            up: Vec3::new(0.0, 1.0, 0.0),
            ..Default::default()
        });

        // ── 3-D objects ───────────────────────────────────────────────────────
        draw_grid(20, 1.0, WHITE, WHITE);
        draw_sphere(vec3(reference.0, reference.1, reference.2), 0.1, None, BLACK);

        for charge in &charges {
            let (px, py, pz) = charge.position;
            let (cr, cg, cb, ca) = charge.color;
            draw_sphere(
                vec3(px, py, pz),
                charge_to_radius(charge.charge) as f32,
                None,
                Color::new(cr, cg, cb, ca),
            );
        }

        // Lines between charges
        for i in 0..charges.len() {
            for j in (i + 1)..charges.len() {
                let (ax, ay, az) = charges[i].position;
                let (bx, by, bz) = charges[j].position;
                draw_line_3d(vec3(ax, ay, az), vec3(bx, by, bz), Color::new(1.0, 0.3, 0.5, 0.9));
            }
        }

        // ── Field lines ───────────────────────────────────────────────────────
        if show_field_lines && !field_lines.is_empty() {
            // Faint bounding cube so the user sees the computation volume
            draw_bounds_cube(field_line_bounds, Color::new(0.35, 0.35, 1.0, 0.18));

            // Draw each polyline segment by segment
            for line in &field_lines {
                for seg in line.windows(2) {
                    let (ax, ay, az) = seg[0];
                    let (bx, by, bz) = seg[1];
                    draw_line_3d(
                        vec3(ax, ay, az),
                        vec3(bx, by, bz),
                        Color::new(0.05, 0.88, 0.3, 0.82),
                    );
                }
            }
        }

        // ── UI ────────────────────────────────────────────────────────────────
        root_ui().label(vec2(1065., 20.), "Settings & Configurations");
        root_ui().window(
            hash!(),
            vec2(screen_width() / 2.0 + WINDOW.0 as f32 / 3.5 - 50.0, 40.0),
            vec2(WINDOW.0 as f32 / 5.0 + 50.0, WINDOW.1 as f32 / 5.0 + 180.0),
            |ui| {
                // Reference point
                ui.input_text(hash!(), ": r' (Reference point)", &mut reference_string);
                if ui.button(None, "Set") {
                    match string_to_tuple(&reference_string) {
                        Err(e) => { err_text = String::from(e); show_error = true; }
                        Ok(v)  => reference = v,
                    }
                }

                ui.separator(); ui.separator();
                ui.label(None, "** Add Charges to Configuration:");
                ui.input_text(hash!(), "Charge value (in C)", &mut charge_value);
                ui.input_text(hash!(), "Charge position",     &mut charge_position);

                if ui.button(None, "add charge") {
                    let p = string_to_tuple(&charge_position);
                    if charge_value.parse::<f32>().is_err() {
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
                        charges.push(Charge {
                            charge:   charge_value.parse::<f64>().unwrap(),
                            position,
                            color:    generate_random_rgba(),
                        });
                        // Mark field lines for recomputation
                        field_lines_dirty = true;
                    }
                }

                ui.separator(); ui.separator();
                ui.label(None, "** Charges:");

                let mut idx = 1;
                for charge in &charges {
                    ui.label(None, &format!("{idx}- c: {:.?}, p: {:?}", charge.charge, charge.position));
                    idx += 1;
                }

                if charges.is_empty() {
                    ui.separator();
                    ui.label(None, "No charge has been defined yet.");
                    ui.separator();
                } else if ui.button(None, "Reset") {
                    charges.clear();
                    field_lines.clear();
                    field_line_bounds = 0.0;
                    field_lines_dirty = false;
                }

                // ── Field line toggle ─────────────────────────────────────────
                ui.separator(); ui.separator();
                ui.label(None, "** Field Lines (E):");
                let fl_label = if show_field_lines { "[ ON] Hide Field Lines" }
                else               { "[OFF] Show Field Lines" };
                if ui.button(None, fl_label) {
                    show_field_lines  = !show_field_lines;
                    field_lines_dirty = true; // recompute (or clear) next frame
                    if !show_field_lines {
                        field_lines.clear();
                    }
                }
            },
        );

        root_ui().label(vec2(75., 20.), "Calculations & Properties");
        root_ui().window(
            hash!(),
            vec2(20., 40.),
            vec2(WINDOW.0 as f32 / 5.0, WINDOW.1 as f32 / 1.35),
            |ui| {
                ui.label(None, "** Multipole Moments:");
                ui.editbox(hash!(), vec2(WINDOW.0 as f32 / 5.0 - 6., 220.), &mut p);
                ui.separator(); ui.separator();
                ui.label(None, "** Electric Potential (V):");
                ui.editbox(hash!(), vec2(WINDOW.0 as f32 / 5.0 - 6., 50.), &mut phi);
                ui.separator(); ui.separator();
                ui.label(None, "** Electric Field (N/C):");
                ui.editbox(hash!(), vec2(WINDOW.0 as f32 / 5.0 - 6., 90.), &mut e);
                ui.separator(); ui.separator();
            },
        );

        // ── Overlays ──────────────────────────────────────────────────────────
        set_default_camera();
        root_ui().label(vec2((WINDOW.0 / 2) as f32 - 60.0, 20.0), "Coulomb3D (v0.1.1)");

        if show_error {
            error_dialog.show(&format!("{}:\nplease check your input.", err_text));
            show_error = false;
        }
        if error_dialog.update() {}
        error_dialog.draw();

        next_frame().await
    }
}