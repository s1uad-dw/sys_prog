use gtk::prelude::*;
use gtk::{Window, WindowType};
use cairo::Format;
use cairo::{Context, ImageSurface};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Parabola");
    window.set_default_size(400, 400);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.connect_draw(|win, ctx| {
        let (width, height) = win.get_size();
        let surface = ImageSurface::create(Format::ARgb32, width, height).unwrap();
        let cr = Context::new(&surface);

        // Set the line width and color
        cr.set_line_width(2.0);
        cr.set_source_rgb(0.0, 0.0, 0.0);

        // Draw the parabola
        let a = 0.01;
        let b = 0.0;
        let c = 0.0;
        let x0 = width as f64 / 2.0;
        let y0 = height as f64 / 2.0;
        cr.move_to(0.0, y0);
        for x in 0..width {
            let y = a * ((x as f64 - x0) * (x as f64 - x0)) + b * (x as f64 - x0) + c + y0;
            cr.line_to(x as f64, y);
        }
        cr.expect("REASON").stroke().expect("pizda");

        // Draw the surface to the context
        ctx.set_source_surface(&surface, 0.0, 0.0);
        ctx.paint();

        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}