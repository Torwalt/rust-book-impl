use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum DepartmentType {
    Sales,
    Engineering,
    HumanResources,
}

#[derive(Debug)]
pub struct Department {
    pub t: DepartmentType,
    pub staff: Vec<Employee>,
}

#[derive(Debug)]
pub struct Employee {
    pub name: String,
}

impl Employee {
    pub fn new(n: &str) -> Self {
        Self {
            name: String::from(n),
        }
    }
}

pub struct Company {
    departments: HashMap<DepartmentType, Department>,
}

impl Company {
    pub fn new() -> Self {
        Self {
            departments: HashMap::new(),
        }
    }

    pub fn add_to_department(&mut self, e: Employee, d: DepartmentType) {
        let maybe_dep = self.departments.get_mut(&d);
        match maybe_dep {
            Some(dep) => {
                dep.staff.push(e);
            }
            None => {
                let new_dep = Department {
                    t: d.clone(),
                    staff: vec![e],
                };
                self.departments.insert(d, new_dep);
            }
        }
    }

    pub fn list_department(&self, d: &DepartmentType) -> &[Employee] {
        let maybe_dep = self.departments.get(d);
        match maybe_dep {
            Some(dep) => return &dep.staff,
            None => return &[],
        }
    }

    pub fn list_company(&self) -> Vec<&Department> {
        return self.departments.values().collect::<Vec<&Department>>();
    }
}
