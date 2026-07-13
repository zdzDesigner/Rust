use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    pattern: String,
    path: String,
}

fn main() {
    let args = Args::parse();
    println!("args.path:{}, args.pattern:{}", args.path, args.pattern);
}
