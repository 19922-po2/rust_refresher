use enigo::{Enigo, MouseButton, MouseControllable};
use rdev::{Event, EventType, Key, listen};
use screenshots::Screen;
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread,
    time::Duration,
};

static BM_COUNT: AtomicUsize = AtomicUsize::new(0);
static MYSTIC_COUNT: AtomicUsize = AtomicUsize::new(0);

fn get_primary_screen() -> Screen {
    let screens = Screen::all().unwrap();
    screens
        .into_iter()
        .find(|s| s.display_info.x == 0 && s.display_info.y == 0) // primary monitor
        .unwrap_or_else(|| Screen::all().unwrap()[0].clone()) // fallback
}

fn get_screen_resolution() -> (u32, u32) {
    let screen = get_primary_screen();
    let image = screen.capture().unwrap();
    (image.width(), image.height())
}

/* fn get_screen_resolution() -> (u32, u32) {
    let screen = Screen::all().unwrap()[0].clone();
    let image = screen.capture().unwrap();
    (image.width(), image.height())
} */

fn left_click_at(enigo: &mut Enigo, x: i32, y: i32) {
    enigo.mouse_move_to(x, y);
    thread::sleep(Duration::from_secs(1));
    enigo.mouse_click(MouseButton::Left);
}

fn move_mouse_to(enigo: &mut Enigo, x: i32, y: i32) {
    enigo.mouse_move_to(x, y);
}

/* fn double_click_at(enigo: &mut Enigo, x: i32, y: i32) {
    enigo.mouse_move_to(x, y);
    enigo.mouse_click(MouseButton::Left);
    thread::sleep(Duration::from_millis(250));
    enigo.mouse_click(MouseButton::Left);
}
 */
fn scroll_down(enigo: &mut Enigo, amount: i32) {
    enigo.mouse_scroll_y(amount); // negative for down
}

fn screenshot_pixel(x: u32, y: u32) -> (u8, u8, u8) {
    //let screen = Screen::all().unwrap()[0].clone();
    let screen = get_primary_screen();
    let image = screen.capture().unwrap();
    let buffer = image.as_raw();
    let idx = ((y * image.width() + x) * 4) as usize;
    (
        buffer[idx],     // R
        buffer[idx + 1], // G
        buffer[idx + 2], // B
    )
}
/* fn screenshot_pixel(x: u32, y: u32) -> (u8, u8, u8) {
    let screen = Screen::all().unwrap()[0].clone();
    let image = screen.capture().unwrap();
    let pixel = image.get_pixel(x, y); // returns &Rgba<u8>
    let channels = pixel.0;
    // Try RGBA first
    (channels[2], channels[1], channels[0])
} */

fn is_pixel_color(x: u32, y: u32, expected: (u8, u8, u8)) -> bool {
    screenshot_pixel(x, y) == expected
}

fn refresh(enigo: &mut Enigo) {
    thread::sleep(Duration::from_secs(1));
    left_click_at(enigo, 447, 955);
    thread::sleep(Duration::from_secs(2));
    left_click_at(enigo, 1114, 672);
    thread::sleep(Duration::from_secs(1));
    move_mouse_to(enigo, 1285, 227);
    thread::sleep(Duration::from_secs(1));
}

fn buy(enigo: &mut Enigo, item: Option<(i32, i32)>) {
    if let Some((x, y)) = item {
        left_click_at(enigo, x, y);
        thread::sleep(Duration::from_secs(1));
        left_click_at(enigo, 1150, 743);
        thread::sleep(Duration::from_secs(1));
    } else {
        println!("No items to buy");
    }
}

fn check_bm_1_to_4() -> Option<(i32, i32)> {
    if is_pixel_color(894, 199, (177, 91, 33)) {
        println!("BM found at (894, 199)");
        BM_COUNT.fetch_add(1, Ordering::SeqCst);
        return Some((1696, 249));
    }
    if is_pixel_color(894, 199, (250, 65, 22)) {
        println!("Mystic found at (894, 199)");
        MYSTIC_COUNT.fetch_add(1, Ordering::SeqCst);
        return Some((1696, 249));
    }
    if is_pixel_color(895, 403, (174, 86, 30)) {
        println!("BM found at (895, 403)");
        BM_COUNT.fetch_add(1, Ordering::SeqCst);
        return Some((1696, 460));
    }
    if is_pixel_color(895, 403, (250, 65, 22)) {
        println!("Mystic found at (895, 403)");
        MYSTIC_COUNT.fetch_add(1, Ordering::SeqCst);
        return Some((1696, 460));
    }
    if is_pixel_color(895, 609, (178, 93, 34)) {
        println!("BM found at (895, 609)");
        BM_COUNT.fetch_add(1, Ordering::SeqCst);
        return Some((1696, 666));
    }
    if is_pixel_color(895, 609, (250, 65, 22)) {
        println!("Mystic found at (895, 609)");
        MYSTIC_COUNT.fetch_add(1, Ordering::SeqCst);
        return Some((1696, 666));
    }
    if is_pixel_color(895, 816, (179, 95, 36)) {
        println!("BM found at (895, 816)");
        BM_COUNT.fetch_add(1, Ordering::SeqCst);
        return Some((1696, 870));
    }
    if is_pixel_color(895, 816, (250, 61, 21)) {
        println!("Mystic found at (895, 816)");
        MYSTIC_COUNT.fetch_add(1, Ordering::SeqCst);
        return Some((1696, 870));
    }

    //println!("No BMs or Mystics found in rows 1 to 4");
    None
}

fn check_bm_5_to_6() -> Option<(i32, i32)> {
    if is_pixel_color(893, 714, (180, 100, 40)) {
        println!("BM found at (893, 714)");
        BM_COUNT.fetch_add(1, Ordering::SeqCst);
        return Some((1696, 770));
    }
    if is_pixel_color(893, 714, (250, 65, 22)) {
        println!("Mystic found at (893, 714)");
        MYSTIC_COUNT.fetch_add(1, Ordering::SeqCst);
        return Some((1696, 770));
    }
    if is_pixel_color(893, 920, (181, 103, 42)) {
        println!("BM found at (893, 920)");
        BM_COUNT.fetch_add(1, Ordering::SeqCst);
        return Some((1696, 970));
    }
    if is_pixel_color(893, 920, (252, 62, 21)) {
        println!("Mystic found at (893, 920)");
        MYSTIC_COUNT.fetch_add(1, Ordering::SeqCst);
        return Some((1696, 970));
    }

    //println!("No BMs or Mystics found in rows 5 to 6");
    None
}

fn refresh_loop() {
    let stop = Arc::new(AtomicBool::new(false));
    let stop_clone = stop.clone();

    // Keyboard listener in separate thread
    std::thread::spawn(move || {
        let callback = move |event: Event| {
            if let EventType::KeyPress(Key::Escape) = event.event_type {
                stop_clone.store(true, Ordering::SeqCst);
            }
        };
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error);
        }
    });

    let mut enigo = Enigo::new();

    while !stop.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_secs(1));

        if let Some(item) = check_bm_1_to_4() {
            println!("BM or Mystic found in rows 1 to 4");
            buy(&mut enigo, Some(item));
        }

        thread::sleep(Duration::from_secs(1));
        scroll_down(&mut enigo, 30);
        thread::sleep(Duration::from_secs(1));

        if let Some(item) = check_bm_5_to_6() {
            println!("BM or Mystic found in rows 5 to 6");
            buy(&mut enigo, Some(item));
        } else {
            refresh(&mut enigo);
        }
        println!(
            "Current totals → BMs: {}, Mystics: {}",
            BM_COUNT.load(Ordering::SeqCst),
            MYSTIC_COUNT.load(Ordering::SeqCst)
        );
    }

    println!("Refresh loop stopped by user.");
}

fn main() {
    thread::sleep(Duration::from_secs(2));
    let (w, h) = get_screen_resolution();
    println!("Screen resolution: {}x{}", w, h);

    refresh_loop();
    /* check_bm_1_to_4(); */
    /* let mut enigo = Enigo::new();
    move_mouse_to(&mut enigo, 894, 199);
    let (r, g, b) = screenshot_pixel(894, 199);
    println!("Pixel at (894, 199): ({},{},{})", r, g, b); */
}
