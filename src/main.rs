use clap::Parser;
use color_eyre::{eyre::Result, Report};
use iris_cli::utils::logger::config_logger;
use iris_cli::{cli, task_procces};

#[tokio::main]
async fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let arg_cli = cli::Cli::parse();

    config_logger(arg_cli.verbose, env_logger::Target::Stdout).expect("Error config logger");

    log::info!("Starting translation-cli");

    task_procces::start_procces(&arg_cli).await?;
    Ok(())
}

/*
match deelp::actions::send_petition(&client_deepl, &message).await {
    Ok(a) => {
        let json_strutc:Value = serde_json::from_slice(&a.bytes().await.unwrap()).unwrap();

        println!("{json_strutc}");
        let map = json_strutc.as_object().unwrap();
        for entry in map {
            println!("{} - {}",entry.0,entry.1)
        };
    }
    Err(e) => handle_error_petition_log(&e),
};
 */
