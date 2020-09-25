#![forbid(unsafe_code)]

#[macro_use]
extern crate log;

// use clap::Parser;
// use failure::{Fallible, ResultExt};
use log::LevelFilter;
// use std::error::Error;
use zbus::{Connection, interface};
use std::error::Error;
use std::process::Command;

// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     /// Verbose mode (-v, -vv, -vvv, etc.)
//     #[arg(short, long)]
//     verbose: Vec<()>
// }


struct Bootc1 {
    count: u64
}

#[interface(name = "org.bootc1")]
impl Bootc1 {
    fn status(&self) -> String {
        info!("Running: 'bootc status'");
        let mut cmd = Command::new("bootc");
        cmd.arg("status");
        let Ok(output) = cmd.output() else {
            error!("failed to execute process");
            return "{}".to_string();
        };
        match String::from_utf8(output.stdout) {
            Ok(r) => r,
            Err(e) => {
                error!("non utf8 output: {e}");
                "{}".to_string()
            }
        }
    }

    fn update(&self) -> String {
        info!("Running: 'bootc update'");
        let mut cmd = Command::new("bootc");
        cmd.arg("update").arg("--check");
        let Ok(output) = cmd.output() else {
            error!("failed to execute process");
            return "{}".to_string();
        };
        match String::from_utf8(output.stdout) {
            Ok(r) => r,
            Err(e) => {
                error!("non utf8 output: {e}");
                "{}".to_string()
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let opt = Opt::from_args();

    // let level = match opt.verbose {
    //     0 => LevelFilter::Info,
    //     1 => LevelFilter::Debug,
    //     _ => LevelFilter::Trace,
    // };
    let level = LevelFilter::Info;

    env_logger::Builder::new()
        .filter(None, level)
        .format_timestamp(None)
        .init();

    let user = false;
    let connection = if user {
        Connection::session().await?
    } else {
        Connection::system().await?
    };

    let state = Bootc1 { count: 0 };

    connection.object_server().at("/org/bootc1", state).await?;

    connection.request_name("org.bootc").await?;

    info!("Waiting for clients");

    loop {
        std::future::pending::<()>().await;
    }
}
