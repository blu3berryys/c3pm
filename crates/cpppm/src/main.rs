use cli::cli;
use depman::git_switch;

#[tokio::main]
async fn main() -> Result<(), String> {
    git_switch("aaa", "./owo").expect("fuck");
    
    cli()
}