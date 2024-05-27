fn main() {
    let mut on_heap = String::from("This string will be stored on the heap.");
    take_ownership(&mut on_heap);
    println!("{on_heap}");

    let slice = &on_heap[0..5];
    println!("{slice}");

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let user = User {
        name: String::from("alex"),
        age: 29,
    };
    let mut user = dbg!(user);
    println!("{}, {}", user.age, user.name);
    user.print_name();
    user.set_name(String::from("schmalex"));
    dbg!(user);

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn take_ownership(data: &mut String) {
    data.push_str("other");
    println!("{data}");
}

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

impl User {
    fn print_name(&self) {
        println!("{0}", self.name,)
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
