use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ClientArgs {
    #[arg(short, long, required=true)]
    pub addr: String,

    #[arg(short, long, required=true)]
    pub name: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ServerArgs {
    #[arg(short, long, required=true)]
    pub addr: String,
}
