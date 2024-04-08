use std::collections::HashMap;

pub fn employee_db() {
    let mut db = HashMap::new();

    add_prompt("Add Sally to Engineering".to_string(), &mut db);
    add_prompt("Add Amir to Sales".to_string(), &mut db);
    add_prompt("Add Bashir to Sales".to_string(), &mut db);
    add_prompt("Add Allu to Engineering".to_string(), &mut db);

    let mut sorted_db = HashMap::new();
    for (d, e) in db {
        let mut sorted_employees = e.clone();
        sorted_employees.sort();
        sorted_db.insert(d, sorted_employees);
    }

    println!("{:?}", sorted_db);
}

fn add_prompt(s: String, db: &mut HashMap<String, Vec<String>>) {
    let words: Vec<&str> = s.split_whitespace().collect();

    let department = words[3].to_string();
    db.entry(department)
        .or_insert(vec![])
        .push(words[1].to_string());
}
