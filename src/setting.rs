// TODO: Better argument parsing, maybe will use a library if arguments will become too much.

#[derive(Debug)]
pub struct Settings {
    program_name: String,
    pub image_file: String,
    pub image_size: u32,
}
impl Settings {
    pub fn new(args: &mut std::env::Args) -> Option<Self> {
        let program_name = args.next().unwrap();
        let image_file: String;
        let mut image_size: u32 = 16;
        let args: Vec<String> = args.collect();

        if args.len() < 1 {
            println!("try -h | --help option to show help!");
            return None;
        }

        for mut _i in 0..args.len() {
            match args[_i].as_str() {
                "-h" | "--help" => {
                    print_usage(program_name);
                    return None;
                }
                "-s" | "--size" => {
                    if _i == args.len() - 1 {
                        print_usage(program_name);
                        return None;
                    };
                    image_size = args[_i + 1].parse().unwrap_or(image_size);
                    _i += 1
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
            image_size,
        })
    }
}
pub fn print_usage(program_name: String) {
    println!("USAGE: {} [OPTIONS] [IMAGE_FILE]", program_name);
    println!();
    println!("OPTIONS: ");
    println!("\t -h | --help\t Show this help message");
    println!("\t -s | --size\t Followed by a number to Resize the output (lower number means bigger output).");
}
