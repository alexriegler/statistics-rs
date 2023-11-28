enum Color {
    // Black = 0,
    Red = 1,
    Green,
}

fn print_color_msg(color: Color, msg: &str) {
    let esc = "\x1b";
    let color = color as u32;
    println!("{esc}[3{color}m{msg}{esc}[m");
}

pub fn print_fail_msg(msg: &str) {
    print_color_msg(Color::Red, msg);
}

pub fn print_pass_msg(msg: &str) {
    print_color_msg(Color::Green, msg);
}
