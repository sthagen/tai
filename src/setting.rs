// TODO: Better argument parsing, maybe will use a library if arguments will become too much.

#[derive(Debug)]
pub struct Settings {
    program_name: String,
    pub image_file: String,
}
impl Settings {
    pub fn new(args: &mut std::env::Args) -> Option<Self> {
        let program_name = args.next().unwrap();
        let image_file: String;

        let args: Vec<String> = args.collect();

        if args.len() < 1 {
            print_usage(program_name);
            return None;
        }

        for arg in args.iter() {
            match arg.as_str() {
                "-h" | "--help" => {
                    print_usage(program_name);
                    return None;
                }
                _ => {
                    continue;
                }
            }
        }

        if args[args.len() - 1].starts_with("-") {
            return None;
        };

        image_file = args.into_iter().last().unwrap();

        Some(Self {
            program_name,
            image_file,
        })
    }
}
pub fn print_usage(program_name: String) {
    println!("USAGE: {} [OPTIONS] [IMAGE_FILE]", program_name);
    println!();
    println!("OPTIONS: ");
    println!("\t -h|--help\t Show this help message");
}
