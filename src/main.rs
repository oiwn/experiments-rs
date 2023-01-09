use clap::{Parser, Subcommand};

mod actix_actor;
mod tiny_actor;
mod tokio_actor;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: CliCommands,
}

#[derive(Subcommand)]
enum CliCommands {
    TokioActor,
    TinyActor,
    ActixActor,
}

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    let cli = Cli::parse();

    match cli.command {
        // Tokio actor
        CliCommands::TokioActor => {
            log::info!("Tokio actor...");
            let rt = tokio::runtime::Runtime::new().unwrap();

            rt.block_on(async {
                println!("now running on a worker thread");

                tokio_actor::run().await;
            });
        }
        // Tiny actor
        CliCommands::TinyActor => {
            log::info!("Tiny actor...");
            let rt = tokio::runtime::Runtime::new().unwrap();

            rt.block_on(async {
                println!("now running on a worker thread");
                let result = tiny_actor::run().await;
                match result {
                    Ok(data) => println!("Finished: {:?}", data),
                    Err(err) => eprintln!("Error: {:?}", err),
                }
            });
        }
        // Actix actor
        CliCommands::ActixActor => {
            log::info!("Actix actor...");
            let system = actix::System::new();
            system.block_on(async { actix_actor::run().await });
            system.run().unwrap();
            actix::System::current().stop();
        }
    }
}
