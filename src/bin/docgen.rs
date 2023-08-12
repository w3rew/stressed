use stressed::args::Args;

fn main() -> std::io::Result<()> {
    let out = std::env::args().nth(1).expect("Target file is not given");

    let md = clap_markdown::help_markdown::<Args>();

    std::fs::write(out, md)?;

    Ok(())
}
