use egui_glfw as egui_backend;

use egui_backend::egui::{vec2, Pos2, Rect};
use egui_glfw::glfw::{Context, fail_on_errors};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
// const PIC_WIDTH: i32 = 320;
// const PIC_HEIGHT: i32 = 192;

mod triangle;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (mut window, events) = glfw
        .create_window(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "Egui in GLFW!",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_all_polling(true);
    window.make_current();
    // glfw.set_swap_interval(glfw::SwapInterval::Adaptive);
    glfw.set_swap_interval(glfw::SwapInterval::None);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut painter = egui_backend::Painter::new(&mut window);
    let egui_ctx = egui::Context::default();

    let (width, height) = window.get_framebuffer_size();
    let native_pixels_per_point = window.get_content_scale().0;

    let mut egui_input_state = egui_backend::EguiInputState::new(egui::RawInput {
        screen_rect: Some(Rect::from_min_size(
            Pos2::new(0f32, 0f32),
            vec2(width as f32, height as f32) / native_pixels_per_point,
        )),
        ..Default::default()
    });

    egui_input_state.input.time = Some(0.01);
    
    let triangle = triangle::Triangle::new();
    let slider = &mut 0.0;

    // Main rendering loop
    while !window.should_close() {
        glfw.poll_events();

        egui_ctx.begin_frame(egui_input_state.input.take());

        unsafe {
            gl::ClearColor(0.455, 0.302, 0.663, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Clear(gl::DEPTH_TEST);
        }

        triangle.draw();

        egui::Window::new("Egui with GLFW").show(&egui_ctx, |ui| {
            ui.label("A simple sine wave plotted onto a GL texture then blitted to an egui managed Image.");
            let btn_m = &mut ui.button("-");
            let btn_p = &mut ui.button("+");

            ui.add(egui::Slider::new(slider, 0.0..=100.0).text("My value"));

            if btn_m.clicked() && *slider > 0.0 {
                *slider -= 1.0;
            }

            if btn_p.clicked() && *slider < 100.0 {
                *slider += 1.0;
            }
        });

        let egui::FullOutput {
            platform_output,
            textures_delta,
            shapes, .. } = egui_ctx.end_frame();

        //Handle cut, copy text from egui
        if !platform_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(&mut egui_input_state, platform_output.copied_text);
        }

        //Note: passing a bg_color to paint_jobs will clear any previously drawn stuff.
        //Use this only if egui is being used for all drawing and you aren't mixing your own Open GL
        //drawing calls with it.
        //Since we are custom drawing an OpenGL Triangle we don't need egui to clear the background.

        let clipped_shapes = egui_ctx.tessellate(shapes, native_pixels_per_point);
        painter.paint_and_update_textures(native_pixels_per_point, &clipped_shapes, &textures_delta);

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Close => window.set_should_close(true),
                _ => {
                    egui_backend::handle_event(event, &mut egui_input_state);
                }
            }
        }
        
        window.swap_buffers();
    }
}