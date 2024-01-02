use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    number: u32,

    #[arg(long, default_value_t = 0)]
    first: u32,

    #[arg(long, default_value_t = 1)]
    second: u32
}

fn main() {
    let args = Args::parse();
    
    let mut t1 = args.first;
    let mut t2 = args.second;

    println!("{}", t1);
    for _ in 0..args.number-1 {
        println!("{}", t2);
        let next = t1 + t2;
        t1 = t2;
        t2 = next;
    }
}
