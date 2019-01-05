use super::types::Frame;
use conrod_core::{widget, Positionable, Sizeable, Widget};
use glium::Surface;
use std::sync::mpsc::channel;

widget_ids!(struct Ids {frame});

struct ActualImageDisplay {
    width: u32,
    height: u32,
    rx: std::sync::mpsc::Receiver<Vec<u8>>,

    image_map: conrod_core::image::Map<glium::texture::Texture2d>,
    image_id: conrod_core::image::Id,
    widget_ids: Ids,

    events_loop: glium::glutin::EventsLoop,
    display: glium::Display,
    renderer: conrod_glium::Renderer,
    ui: conrod_core::Ui,
}

pub struct ImageDisplay {
    tx: std::sync::mpsc::Sender<Vec<u8>>,
}

fn texture_from_raw_vec(
    raw_vec: Vec<u8>,
    width: u32,
    height: u32,
    display: &glium::Display,
) -> glium::texture::Texture2d {
    let rgba_image: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::from_raw(width, height, raw_vec).unwrap();
    let image_dimensions = rgba_image.dimensions();
    let raw_image =
        glium::texture::RawImage2d::from_raw_rgb_reversed(&rgba_image.into_raw(), image_dimensions);
    glium::texture::Texture2d::new(display, raw_image).unwrap()
}

impl ImageDisplay {
    pub fn new<S: Into<String>>(width: usize, height: usize, window_name: S) -> ImageDisplay {
        let window_name = window_name.into();
        let (tx, rx) = channel();
        std::thread::spawn(move || {
            let displayer = ActualImageDisplay::new(rx, width as u32, height as u32, window_name);
            displayer.run();
        });
        ImageDisplay { tx }
    }

    pub fn update(&mut self, frame: &Frame) -> Result<(), ()> {
        self.tx.send(frame.to_flat_vec()).map_err(|_| ())?;
        Ok(())
    }
}

impl ActualImageDisplay {
    fn run(mut self) {
        'main: loop {
            let mut events = Vec::new();
            self.events_loop.poll_events(|event| events.push(event));
            for event in events {
                if let glium::glutin::Event::WindowEvent { event, .. } = event {
                    match event {
                        glium::glutin::WindowEvent::CloseRequested => break 'main,
                        glium::glutin::WindowEvent::Resized(_) => self.ui.needs_redraw(),
                        _ => (),
                    }
                }
            }

            // Instantiate the widgets.
            {
                let ui = &mut self.ui.set_widgets();
                widget::Image::new(self.image_id)
                    .w_h(self.width as f64, self.height as f64)
                    .middle()
                    .set(self.widget_ids.frame, ui);
            }

            match self.rx.recv_timeout(std::time::Duration::from_millis(16)) {
                Ok(raw_vec) => {
                    self.image_map.replace(
                        self.image_id,
                        texture_from_raw_vec(raw_vec, self.width, self.height, &self.display),
                    );
                    self.ui.needs_redraw();
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => (),
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => return,
            }

            // Render the `Ui` and then display it on the screen.
            if let Some(primitives) = self.ui.draw_if_changed() {
                self.renderer
                    .fill(&self.display, primitives, &self.image_map);
                let mut target = self.display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                self.renderer
                    .draw(&self.display, &mut target, &self.image_map)
                    .unwrap();
                target.finish().unwrap();
            }
        }
    }

    fn new<S>(
        rx: std::sync::mpsc::Receiver<Vec<u8>>,
        width: u32,
        height: u32,
        window_name: S,
    ) -> ActualImageDisplay
    where
        S: Into<String>,
    {
        let events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title(window_name)
            .with_dimensions((width, height).into());
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        // construct our `Ui`.
        let mut ui = conrod_core::UiBuilder::new([width as f64, height as f64]).build();

        // A type used for converting `conrod_core::render::Primitives` into `Command`s that can be used
        // for drawing to the glium `Surface`.
        let renderer = conrod_glium::Renderer::new(&display).unwrap();

        let mut image_map = conrod_core::image::Map::new();
        let mut raw_vec = Vec::new();
        raw_vec.resize((width * height) as usize * 3, 0);
        let image_id = image_map.insert(texture_from_raw_vec(raw_vec, width, height, &display));

        ActualImageDisplay {
            width,
            height,
            rx,

            image_map,
            image_id,
            widget_ids: Ids::new(ui.widget_id_generator()),

            events_loop,
            display,
            renderer,
            ui,
        }
    }
}
