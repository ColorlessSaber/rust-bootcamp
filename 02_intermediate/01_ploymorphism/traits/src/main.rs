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
    let car = Car {
        info: VehicleInfo {
            make: "Honda".to_owned(),
            model: "Civic".to_owned(),
            year: 2015,
        }
    };

    let house = House {};
    let object = create_paintable_object();

    paint_red(&car);
    paint_red(&house);
    paint_green(&object);

    paint_vehicle_yellow(&car);
    //paint_vehicle_yellow(&house); // error for house don't implement park trait
    //paint_vehicle_yellow(&object); // error for object don't implement park trait
}

// Below are two ways to set trait bounds to an input
fn paint_red<T: Paint>(object: &T) { // The T: Paint allows for any generic type that has the trait paint
    object.paint("red".to_owned());
}

fn paint_green(object: &impl Paint) { // Using impl is syntax suger for the setup above.
    object.paint("green".to_owned());
}

fn paint_vehicle_yellow<T>(object: &T) where T: Paint + Park { // T can be anything but must have the traits Paint and Park
    object.paint("yellow".to_owned());
}

// traits can also be used as output. What is returned must implement that trait in question.
// only works with one concrete type.
fn create_paintable_object() -> impl Paint {
    House {}
}