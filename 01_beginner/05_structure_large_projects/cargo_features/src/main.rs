use draw::color;

fn main() {
    draw::draw_line(32, 32);

    let color = color::RGB {
        r: 247,
        g: 76,
        b: 0
    };

    color::draw_line(32, 32, &color);

    let square = draw::shapes::Rectangle {
        color,
        width: 32,
        height: 32
    };

    color::draw_line(32, 32, &square.color);
}
