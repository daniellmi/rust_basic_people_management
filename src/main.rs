use std::io::{stdin};

struct Person {
    id: u8,
    name: String,
    age: u8,
}

fn main() {
    let mut people: Vec<Person> = Vec::new();
    let mut option = String::new();
    let mut cod: u8 = 0;

    loop {
        option.clear();

        println!(
            "inform a number \n 1 - Add person\n 2 - See all the people\n 3 - Modify person \n 4- Delete person \n 0 - Leave program"
        );
        stdin().read_line(&mut option).expect("error");

        let number: i32 = option.trim().parse().expect("error");

        if number == 0 {
            break;
        }

        match number {
            1 => {
                let mut name_t: String = String::new();
                let mut age: String = String::new();

                println!("type the person name: ");
                stdin().read_line(&mut name_t).expect("error");

                println!("type the person age: ");
                stdin().read_line(&mut age).expect("");

                let age_parsed: u8 = age.trim().parse().expect("error");

                people.push(Person {
                    id: cod,
                    name: name_t,
                    age: age_parsed,
                });
            }
            2 => {
                println!("All the people: ");
                for p in &people {
                    println!("id: {}, name: {} & age: {} ", p.id, p.name, p.age);
                }
            }
            3 => {
                println!("Update person: inform the id");
                let mut id = String::new();
                stdin().read_line(&mut id).expect("error");

                let id_parsed: u8 = id.trim().parse().expect("error");

                if let Some(p) = people.iter_mut().find(|p| p.id == id_parsed) {
                    println!("What to dou want to modify?\n 1 - name\n 2 - Age");
                    let mut opt: String = String::new();
                    stdin().read_line(&mut opt).expect("error");

                    let opt_parsed: u8 = opt.trim().parse().expect("error");

                    match opt_parsed {
                        1 => {
                            println!("type the new name: ");

                            let mut new_name = String::new();

                            stdin().read_line(&mut new_name).expect("error");
                            p.name = new_name;
                        }
                        2 => {
                            println!("type the new age: ");
                            let mut new_age = String::new();
                            stdin().read_line(&mut new_age).expect("error");
                            let new_age_parsed: u8 = new_age.trim().parse().expect("error");

                            p.age = new_age_parsed;
                        }
                        _ => println!("value dos not exist"),
                    }
                    println!("Name: {} and age: {}", p.name, p.age);
                } else {
                    println!("not found");
                }
            }
            4 => {
                println!("Delete person: inform the id");

                let mut d_id: String = String::new();

                stdin().read_line(&mut d_id).expect("error");

                let d_id_parsed: u8 = d_id.trim().parse().expect("error");

                // position func returns the index
                if let Some(index) = people.iter_mut().position(|p| p.id == d_id_parsed) {
                    // removes the array element by the index
                    people.swap_remove(index);

                    println!("person removed");
                    for x in &people {
                        println!("id: {} and name: {}", x.id, x.name);
                    }
                } else {
                    println!("no one was found");
                }
            }
            _ => println!("error"),
        }
        cod += 1;
    }
}
