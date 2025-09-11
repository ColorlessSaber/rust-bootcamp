#[derive(Debug)]
struct Student {
    name: String,
    gpa: f32,
}

fn main() {
    let students = vec![
        "Bogdan 3.1",
        "Wallace 2.3",
        "Lidiya 3.5",
        "Kyle 3.9",
        "Anatoliy 4.0",
    ];

    let good_students: Vec<Student> = students.iter()
        .map(|s| {
            let mut s = s.split(' '); // returns an iterator
            let name = s.next()?.to_owned();
            let gpa = s.next()?.parse::<f32>().ok()?;
            Some(Student { name, gpa })
        })
        .flatten()
        .filter(|s| s.gpa >= 3.5)
        .collect();

    // .iter(), .map(), .flatten(), .ok, etc are a combinator that will return an iterator of the strings in the student's vector.
    // combinators are small, pure functions which perform a specific task that could be chained together to perform complex operations.
    //**NOTE** The methods from line-16 up to line-23 are known as lazy. Meaning they won't iterator until next-method is called or a method that calls next-method is used, IE collect.
    // ------------------------------------------
    /* The old way
    let mut good_students = vec![];
    for s in students {
        let mut s = s.split(' '); // returns an iterator
        let name = s.next();
        let gpa = s.next();

        if let (Some(name), Some(gpa)) = (name, gpa) {
            let name = name.to_owned();
            let gpa = gpa.parse::<f32>(); // The ::<f32> is known as "turbo fish syntax."
            // The turbo fish syntax is used when a function defines a generic. However, its unclear what concrete type should substitute that generic.

            if let Ok(gpa) = gpa {
                if gpa >= 3.5 {
                    good_students.push(Student { name, gpa });
                }
            }
        }
    }
    */
    for s in good_students {
        println!("{:?}", s);
    }
}