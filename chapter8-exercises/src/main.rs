use crate::{pig::to_pig_latin, stats::median_and_mode};

mod department;
mod pig;
mod stats;

fn main() {
    println!("Hello, world!");

    let (median, mode): (usize, usize) = median_and_mode(&vec![1, 9, 7, 4, 1, 6, 7, 1]);
    println!("Median is {}, mode is {}", median, mode);

    let piggy = to_pig_latin(String::from("My Name, is Alex."));
    println!("As piglatin: {}", piggy);

    let mut comp = department::Company::new();
    comp.add_to_department(
        department::Employee::new("Alex"),
        department::DepartmentType::Engineering,
    );
    comp.add_to_department(
        department::Employee::new("Shmalex"),
        department::DepartmentType::Engineering,
    );
    comp.add_to_department(
        department::Employee::new("Annika"),
        department::DepartmentType::HumanResources,
    );
    comp.add_to_department(
        department::Employee::new("Sale-ormoon"),
        department::DepartmentType::Sales,
    );

    let engineering_dep = comp.list_department(&department::DepartmentType::Engineering);
    println!("{:?}", engineering_dep);

    let departments = comp.list_company();
    for d in departments {
        println!("Department: {:?}", d.t);
        for e in &d.staff {
            println!("Employee: {}", e.name);
        }
    }
}
