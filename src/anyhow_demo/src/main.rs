use anyhow::Result;
mod config;
mod setup;
mod launcher;

fn main() -> Result<()> {
    launcher::launch_app()
}
