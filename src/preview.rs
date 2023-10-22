use sdl2::event::Event;
use std::sync::mpsc;
use std::thread;

pub struct Preview {
    thread: thread::JoinHandle<()>,
    tx: mpsc::Sender<Vec<u8>>,
}

impl Preview {
    pub fn submit_image(self: &Preview, image: &Vec<u8>) -> Result<(), mpsc::SendError<Vec<u8>>> {
        self.tx.send(image.clone())
    }
    pub fn wait(self: Preview) {
        self.thread.join().unwrap();
    }
}

pub fn open_window(width: usize, height: usize) -> Result<Preview, String> {
    let (tx, rx) = mpsc::channel();
    let child = thread::spawn(move || {
        sdl_thread(rx, width as u32, height as u32);
    });
    // TODO: Check window opened properly before returning.
    Ok(Preview { thread: child, tx })
}

fn sdl_thread(rx: mpsc::Receiver<Vec<u8>>, width: u32, height: u32) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Render preview", width, height)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().software().build().unwrap();

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture(
            sdl2::pixels::PixelFormatEnum::RGBA32,
            sdl2::render::TextureAccess::Static,
            width,
            height,
        )
        .unwrap();

    'mainloop: loop {
        // TODO: Throttle loop.
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }
        match rx.try_recv() {
            Ok(image) => {
                texture.update(None, &image, 4 * width as usize).unwrap();
                canvas.copy(&texture, None, None).unwrap();
                canvas.present();
            }
            _ => {}
        }
    }
}
