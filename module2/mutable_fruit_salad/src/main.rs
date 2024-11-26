/*
Usage:  

cargo run -- fruits.csv
or
cargo run -- --fruits "apple, pear"

 */

 use clap::Parser;
 use fruit_salad_maker::{check_and_add_fruit, create_fruit_salad, fruit_salad_vec};
 
 #[derive(Parser)]
 #[clap(
     version = "1.0",
     author = "Your Name <your.email@example.com>",
     about = "Make a Fruit Salad"
 )]
 struct Opts {
     /// Fruits input as a string of comma separated values
     #[clap(short, long)]
     fruits: Option<String>,
     csvfile: Option<String>,
 }
 
 // Function that converts a csv file to a vector of strings
 fn csv_to_vec(csv: &str) -> Vec<String> {
     csv.split(',')
         .map(|s| s.trim().to_string())
         .collect()
 }
 fn display_fruit_salad(fruits: Vec<String>) {
     println!("Your fruit salad contains:");
     for fruit in fruits {
         println!("{}", fruit);
     }
 }
 
 fn main() {
    // immutable vector
    let fruit_salad_immutable = fruit_salad_vec();
    println!("Original immutable fruit salad: {:?}", fruit_salad_immutable);

    // To mutate the vector, we need to declare it as mutable:
    let mut fruit_salad_mutable = fruit_salad_vec();
    fruit_salad_mutable.push("figs");
    println!("Modified mutable fruit salad: {:?}", fruit_salad_mutable);
 
    // Clone immutable to mutable and add figs by changing ownership.
    let result = check_and_add_fruit(fruit_salad_immutable.clone());
    match result {
        Ok(mutated_fruit_salad) => println!("Added figs to the cloned fruit salad: {:?}", mutated_fruit_salad),
        Err(err) => println!("Error: {}", err),
    }
    println!("Original immutable fruit salad: {:?}", fruit_salad_immutable);

    let opts: Opts = Opts::parse();
 
    // Use fruits from CSV file or command-line input
    let fruit_list = match opts.csvfile {
        Some(filename) => {
            let fruits = std::fs::read_to_string(filename)
                .expect("Could not read file");
            csv_to_vec(&fruits)
        },
        None => {
            opts.fruits.unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        },
    };

    // display fruit salad
    let fruit_salad = create_fruit_salad(fruit_list);
    display_fruit_salad(fruit_salad);

}
