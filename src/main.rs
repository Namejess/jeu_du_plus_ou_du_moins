use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Bienvenue dans le jeu du plus ou du moins !");
    println!("Le but est de trouver un nombre entre 1 et 100");

    let nombre_aleatoire = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Veuillez entrer un nombre entre 1 et 100 :");

        let mut nombre_a_trouver = String::new();

        io::stdin()
            .read_line(&mut nombre_a_trouver)
            .expect("Erreur de lecture");

        let nombre_a_trouver: u32 = match nombre_a_trouver.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Vous avez entré : {}", nombre_a_trouver);

        match nombre_a_trouver.cmp(&nombre_aleatoire) {
            Ordering::Less => println!("Trop petit !"),
            Ordering::Greater => println!("Trop grand !"),
            Ordering::Equal => {
                println!("Bravo !");
                break;
            }
        }
    }
}
