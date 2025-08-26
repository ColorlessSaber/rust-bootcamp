use orphan_rule::Point;
// **NOTE** The Orphan Rule states that in order to implement a trait on a given type, either the trait or the type must be in be defined within the current crate.
// one way to get round it is to create a wrapper around the object in question like below.
struct PointWrapper(Point);

impl PartialEq for PointWrapper {
    fn eq(&self, other: &Self) -> bool {
     self.0.x == other.0.x && self.0.y == other.0.y
    }
}

fn main() {
    let p1 = PointWrapper(Point { x: 5, y: 5 });
    let p2 = PointWrapper(Point { x: 5, y: 5 });

    println!("{}", p1 == p2);
}
