fn sleep_for(secs: f32) {
    let dur = if let Ok(dur) = std::time::Duration::try_from_secs_f32(secs) {
        dur
    } else {
        std::time::Duration::from_millis(500)
    };

    std::thread::sleep(dur);
    println!("slept for {:?}", dur);
}

fn main() {
    sleep_for(-10.0);
    sleep_for(0.8);
}
