macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("./assets/fuckyou.ico");
    res.compile().unwrap();

    for n in 1..=1000 {
        p!("Fuck AutoHotKeys {n} times!")
    }
}