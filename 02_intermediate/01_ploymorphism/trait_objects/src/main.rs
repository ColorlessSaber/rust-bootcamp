trait Vehicle: Paint {
    fn park(&self);
    fn get_default_color() -> String {
        "black".to_owned()
    }
}

trait Paint {
    fn paint(&self, color: String) {
        println!("painting object: {}", color);
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16
}

struct Car {
    info: VehicleInfo
}

impl Vehicle for Car {
    fn park(&self) {
        println!("parking car!");
    }
}

impl Paint for Car {}

struct Truck {
    info: VehicleInfo
}

impl Truck {
    fn unload(&self) {
        println!("unloading truck.")
    }
}

impl Vehicle for Truck {
    fn park(&self) {
        println!("parking truck!");
    }
}

impl Paint for Truck {}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("painting house: {}", color);
    }
}

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Honda".to_owned(),
            model: "Civic".to_owned(),
            year: 1995
        }
    };
    let house = House {};
    let object = create_paintable_object(true);

    // The dyn is used to allow the vector to have mix-match of objects that implement of the Paint trait
    let paintable_objects: Vec<&dyn Paint> = vec![&car, &house];

    paint_red(&car);
    paint_red(&house);
    paint_red(object.as_ref()); // The .as_ref() concerts the Box smart point to a reference

    paint_vehicle_red(&car);
}

fn paint_red(object: &dyn Paint) { // The &dyn is needed so the function accepts a trait object.
    object.paint("red".to_owned());
}

fn paint_vehicle_red<T>(object: &T) where T: Vehicle {
    object.paint("red".to_owned());
}

// If you plan to return different objects depending on the input, you need to use
// trait objects. Below is how to implement a trait object using Box smart pointer and dyn.
fn create_paintable_object(vehicle: bool) -> Box<dyn Paint> {
    if vehicle {
        Box::new(Car {
            info: VehicleInfo {
                make: "Honda".to_owned(),
                model: "Civic".to_owned(),
                year: 1995
            }
        })
    } else {
        Box::new(House {})
    }
}