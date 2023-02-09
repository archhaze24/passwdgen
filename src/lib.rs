use clap::{arg, ArgAction, Parser};
use rand::Rng;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "passwdgen - a pretty simple tool for password generation"
)]
pub struct Args {
    #[arg(short, long, help = "Length of generated passwords. Default is 16.")]
    length: Option<u32>,

    #[arg(short, long, help = "Quantity of generated passwords. Default is 1.")]
    quantity: Option<u32>,

    #[arg(long = "remove-similar", action = ArgAction::SetTrue, help = "Remove similar characters from passwords (i, l, 1, L, o, O, 0). Default is false.")]
    remove_similar: bool,

    #[arg(long = "remove-numbers", action = ArgAction::SetTrue, help = "Remove numbers from passwords. Default is false.")]
    remove_numbers: bool,

    #[arg(long = "remove-uppercase", action = ArgAction::SetTrue, help = "Remove uppercase characters from passwords. Default is false.")]
    remove_uppercase: bool,

    #[arg(long = "remove-lowercase", action = ArgAction::SetTrue, help = "Remove lowercase characters from passwords. Default is false.")]
    remove_lowercase: bool,

    #[arg(short = 's', long = "add-special-characters", action = ArgAction::SetTrue, help = "Add special characters (!\";#$%&'()*+,-./:;<=>?@[]^_`{|}~) to passwords. Default is false.")]
    add_special_characters: bool,
}

pub fn start(args: Args) {
    let quantity: u32 = args.quantity.unwrap_or(1);
    let length: u32 = args.length.unwrap_or(16);

    if args.remove_uppercase
        && args.remove_lowercase
        && args.remove_numbers
        && !args.add_special_characters
    {
        eprintln!("passwdgen: error - there is no characters to generate password from");
        std::process::exit(1);
    }

    let characters = compose_characters(&args);

    for _ in 0..quantity {
        let password = generate(&characters, length);

        println!("{password}");
    }
}

fn compose_characters(args: &Args) -> Vec<char> {
    let lowercase: &str = if !args.remove_lowercase {
        if args.remove_similar {
            "abcdefghjkmnpqrstuvwxyz"
        } else {
            "abcdefghijklmnopqrstuvwxyz"
        }
    } else {
        ""
    };

    let uppercase: &str = if !args.remove_uppercase {
        if args.remove_similar {
            "ABCDEFGHIJKMNPQRSTUVWXYZ"
        } else {
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        }
    } else {
        ""
    };

    let special: &str = if args.add_special_characters {
        "!\";#$%&'()*+,-./:;<=>?@[]^_`{|}~"
    } else {
        ""
    };

    let numbers: &str = if !args.remove_numbers {
        if args.remove_similar {
            "23456789"
        } else {
            "0123456789"
        }
    } else {
        ""
    };

    let characters = String::from("") + lowercase + uppercase + special + numbers;

    characters.chars().collect()
}

fn generate(characters: &Vec<char>, length: u32) -> String {
    let mut password = String::new();
    let mut rng = rand::thread_rng();

    for _ in 0..length {
        let index = rng.gen_range(0..characters.len());
        password.insert(password.len(), characters[index]);
    }

    password
}
