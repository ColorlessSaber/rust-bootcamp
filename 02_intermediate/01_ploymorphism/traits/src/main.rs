trait Park {
    fn park(&self); // left empty is it can be implemented
}

trait Paint {
    fn paint(&self, color: String) { // Added default implementation that can be overwritten
        println!("Paint object {}", color);
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16,
}

struct Car {
    info: VehicleInfo,
}

impl Park for Car {
    fn park(&self) {
        println!("parking car!");
    }
}

impl Paint for Car {} // no errors because it has default implementation

struct Truck {
    info: VehicleInfo,
}

impl Truck {
    fn unload(&self) {
        println!("unloading truck.")
    }
}

impl Park for Truck {
    fn park(&self) {
        println!("parking truck!");
    }
}

impl Paint for Truck {}

struct House {}
impl Paint for House {
    fn paint(&self, color: String) { // overrides the default implementation
        println!("paint house {}", color);
    }
}

fn main() {
    println!("Hello, world!");
}
