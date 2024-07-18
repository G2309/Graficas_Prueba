mod color;

use color::Color;

fn main() {
    // Prueba de la estructura Color
    let color1 = Color::new(255, 0, 0);
    let color2 = Color::from_hex(0x00FF00);
    let color3 = color1.add(&color2);
    let color4 = color3.multiply(0.5);

    color1.print(); // Color (r: 255, g: 0, b: 0)
    color2.print(); // Color (r: 0, g: 255, b: 0)
    color3.print(); // Color (r: 255, g: 255, b: 0)
    color4.print(); // Color (r: 127, g: 127, b: 0)

    println!("Color 1 Hex: {:X}", color1.to_hex());

}

