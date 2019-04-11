extern crate clap;
extern crate inflector;
extern crate rand;

use clap::{App, Arg};
use inflector::Inflector;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
     let matches = App::new("Indie Name Generator")
          .author("Rafael Sierra <rafaeljsg14 at gmail>")
          .about("Get your own Indie name for your gaming company!")
          .arg(Arg::with_name("adjectives")
               .short("a")
               .long("adjectives")
               .value_name("FILE")
               .help("Text file with list of adjectives to use")
               .takes_value(true)
               .required(true))
          .arg(Arg::with_name("nouns")
               .short("n")
               .long("nouns")
               .value_name("FILE")
               .help("Text file with list of nouns to use")
               .takes_value(true)
               .required(true))
          .arg(Arg::with_name("separator")
               .short("s")
               .long("separator")
               .value_name("SEPARATOR")
               .help("Separator to use")
               .takes_value(true)
               .default_value(" "))
          .get_matches();

     //
     // Handling adjectives file
     //
     let adjectives_path = Path::new(matches.value_of("adjectives").unwrap());
     let adjectives_display = adjectives_path.display();
     let mut adjectives_str = String::new();
     {
          let mut adjectives_file = match File::open(&adjectives_path) {
               Err(why) => panic!("Failed opening {}: {}", adjectives_display, why),
               Ok(file) => file,
          };

          match adjectives_file.read_to_string(&mut adjectives_str) {
               Err(why) => panic!("{}", why),
               Ok(_) => (),
          };
     }
     let adjectives: Vec<&str> = adjectives_str.split("\n").collect();

     //
     // Handling nouns file
     //
     let nouns_path = Path::new(matches.value_of("nouns").unwrap());
     let nouns_display = nouns_path.display();
     let mut nouns_str = String::new();
     {
          let mut nouns_file = match File::open(&nouns_path) {
               Err(why) => panic!("Failed opening {}: {}", nouns_display, why),
               Ok(file) => file,
          };

          match nouns_file.read_to_string(&mut nouns_str) {
               Err(why) => panic!("{}", why),
               Ok(_) => (),
          };
     }
     let nouns: Vec<&str> = nouns_str.split("\n").collect();

     //
     // Using advanced AI and Machine Learning we create the indie gaming company name
     //
     let mut rng = thread_rng();
     let noun = match nouns.choose(&mut rng) {
          Some(name) => name,
          None => "",
     };
     let adjective = match adjectives.choose(&mut rng) {
          Some(name) => name,
          None => "",
     };

     // Now print the result
     println!(
          "{}{}{}",
          adjective.to_title_case(),
          matches.value_of("separator").unwrap(),
          noun.to_title_case()
     );
}
