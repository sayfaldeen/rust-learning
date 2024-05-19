use clap::Parser; // bring clap into scope

// Set up a struct that contains the parameters
#[derive(Parser, Debug)]
struct Params {
    #[clap(short='u', long="username", default_value = "Anonymous")]
    username: String,

    #[clap(short, long, default_value = "OK")]
    status: String,

    #[clap(short, long, default_value_t = 3)]
    n_times: u32,
}

fn main() {
    let args = Params::parse();
    println!("\n{:#?}", args);

    print!("\n");
    for _ in 0..args.n_times{
        println!("Hello, {}. I am glad to see that you are {}.", args.username, args.status);
    }

}
