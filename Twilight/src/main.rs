#[macro_use] extern crate anyhow;

mod types;
mod options;
mod twilight;
mod handler;

#[tokio::main(worker_threads=8)]
async fn main() -> anyhow::Result<()> {
  let iopts = options::get_ioptions()
                .map_err(|e| anyhow!("Failed to parse Dhall config {e}"))?;
  if let Err(err) = twilight::run(iopts).await {
    panic!("Twilight died {err}")
  }
  Ok(())
}
