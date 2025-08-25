trait Vehicle: Paint { // Paint is a supertrait to Vehicle
    fn park(&self);
    fn get_default_color() -> String {
        "black".to_owned()
    }
}

trait Paint {
    fn paint(&self, color: String) {
        println!("Paint color: {}", color);
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

impl Paint for Car {}

impl Vehicle for Car {
    fn park(&self) {
        println!("Parking car");
    }
}

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Honda".to_owned(),
            model: "Civic".to_owned(),
            year: 2015,
        }
    };
    paint_vehicle_yellow(&car);
}

fn paint_vehicle_yellow<T>(object: &T) where T: Vehicle { // T can be anything but must have the traits Paint and Park
    object.paint("yellow".to_owned());
}