// use glfw::{Action, Context, Key};
// // use imgui::*;

fn main() {
    //     let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    //     let (mut window, events) = glfw
    //         .create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
    //         .expect("Failed to create GLFW window.");

    //     window.set_key_polling(true);
    //     window.make_current();

    let mut ctx = quartz::Context::default();
    // let x = quartz::geometry::Rect::new(0.0, 0.0, 1.0, 1.0);

    //     println!("FOOOOO {:?}", x);

    //     while !window.should_close() {
    //         glfw.poll_events();
    //         for (_, event) in glfw::flush_messages(&events) {
    //             handle_window_event(&mut window, event);
    //         }

    //         // Start of frame
    quartz::begin();
    quartz::text("Bla bla");

    if quartz::button(&mut ctx, "Click me!") {
        println!("Click!");
    }

    quartz::text("Bla bla");
    quartz::end();
    //         // Render frame!
    //     }
}

// fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
//     match event {
//         glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _)
//         | glfw::WindowEvent::Key(Key::Q, _, Action::Press, _) => window.set_should_close(true),
//         _ => {}
//     }
// }