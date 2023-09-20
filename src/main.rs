use image::DynamicImage;
use image::Luma;
use qrcode::QrCode;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureQuery, WindowCanvas};
use std::io::Cursor;
use std::time::Duration;

fn render(canvas: &mut WindowCanvas, texture: &Texture, text: &Texture) -> Result<(), String> {
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let position = Point::new(0, 0);
    // src position in the spritesheet
    let sprite = Rect::new(0, 0, 600, 600);
    // Treat the center of the screen as the (0, 0) coordinate
    let screen_position = position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, sprite.width(), sprite.height());

    canvas.copy(texture, sprite, screen_rect)?;

    let TextureQuery { width, height, .. } = text.query();
    let screen_rect = Rect::from_center(
        // Point::new((width / 2) as i32, (height / 2) as i32),
        Point::new(300, (height / 2) as i32),
        width,
        height,
    );
    canvas.copy(text, None, screen_rect)?;

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);

    let window = video_subsystem
        .window("rust-sdl2 demo", 600, 600)
        .position_centered()
        .borderless()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let encoded_text = "https://rustpad.io/yololo";

    // Encode some data into bits.
    let code = QrCode::new(encoded_text).expect("Could not create QR code");

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();
    // let svg_xml = code.render::<svg::Color>().build();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    let mut bytes = Vec::new();

    DynamicImage::ImageLuma8(image)
        .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
        .expect("Could not convert image");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture_bytes(&bytes)?;

    // Load a font
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context.load_font("/usr/share/fonts/TTF/DejaVuSans.ttf", 48)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    // render a surface, and convert it to a texture bound to the canvas
    let surface_text = font
        .render(encoded_text)
        .blended(Color::RGBA(0, 0, 0, 255))
        .map_err(|e| e.to_string())?;
    let text = texture_creator
        .create_texture_from_surface(&surface_text)
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        render(&mut canvas, &texture, &text)?;

        // canvas.copy(&texture, None, None).unwrap();
        // canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
