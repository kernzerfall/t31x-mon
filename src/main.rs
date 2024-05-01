mod args;

const KEYRING_SERVICE_NAME: &str = "home-temperature-statusbar";

extern crate pretty_env_logger;

#[macro_use]
extern crate log;

use std::time::Duration;

use anyhow::anyhow;
use clap::*;
use tapo::responses::ChildDeviceResult::*;
use tapo::responses::T31XResult;
use tapo::ApiClient;

use keyring::Entry;

use args::CliArgs;
use inquire::*;
use tapo::HubHandler;
use tokio::main;
use tokio::time::sleep;

fn set_creds(user: &str, pass: &str) -> Result<(), anyhow::Error> {
    let entry = Entry::new(KEYRING_SERVICE_NAME, user)?;
    entry.set_password(pass)?;
    info!("Keyring entry for {user} was set");
    Ok(())
}

fn ask_creds() -> Result<String, InquireError> {
    inquire::Password::new("Tapo password > ")
        .with_custom_confirmation_message("Confirm       > ")
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_help_message(
            "Enter the password of your Tapo account. \
             It will be saved to your keyring.",
        )
        .prompt()
}

fn get_pass(user: &str) -> Result<String, anyhow::Error> {
    let entry = Entry::new(KEYRING_SERVICE_NAME, user)?;

    match entry.get_password() {
        Err(keyring::Error::NoEntry) => {
            let pass = ask_creds()?;
            set_creds(user, &pass)?;
            Ok(pass)
        }
        Err(keyring::Error::Ambiguous(_)) => {
            error!("{KEYRING_SERVICE_NAME}, {user} is ambiguous in your keyring");
            Err(anyhow!("K::AMBIG"))
        }
        Ok(res) => Ok(res),
        Err(e) => {
            error!("Unknown keyring error: {e}");
            Err(anyhow!("K::UNKN"))
        }
    }
}

async fn setup(user: &str, pass: &str, ip: &str) -> Result<HubHandler, tapo::Error> {
    ApiClient::new(user, pass)?.h100(ip).await
}

async fn get_print_temp_data(
    hub: &HubHandler,
    device_id: &Option<String>,
) -> Result<(), anyhow::Error> {
    let children = hub.get_child_device_list().await?;
    debug!("Children data\n{children:#?}");

    let temperature_data = children
        .iter()
        .filter_map(|c| match c {
            T315(di) | T310(di) => Some(di),
            _ => None,
        })
        .collect::<Vec<&Box<T31XResult>>>();

    if temperature_data.is_empty() {
        error!("No temperature sensors found");
        println!("NO_SENS");
        return Ok(());
    }

    if temperature_data.len() == 1 {
        info!("Unambiguous data; only 1 sensor found; using it.");
        let tdata = temperature_data.first().unwrap();
        println!(
            "\u{f07d0} {:02.1}\u{f0504} {:02}\u{e373}",
            tdata.current_temperature, tdata.current_humidity
        );
    }

    info!("Multiple temperature devices; filtering data by {device_id:?}");
    if let Some(device_id) = device_id {
        match temperature_data
            .iter()
            .find(|td| td.device_id == *device_id)
        {
            Some(td) => {
                println!(
                    "\u{f07d0} {:02.1}\u{f0504} {:02}\u{e373}",
                    td.current_temperature, td.current_humidity
                );
            }
            None => {
                error!("Could not find temperature data for given device id.");
            }
        };
    }
    // println!("{:#?}", temperature_data);
    Ok(())
}

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    pretty_env_logger::formatted_timed_builder()
        .filter(Some("home_temperature_statusbar"), args.log_level)
        .init();

    info!("hello");

    let pass = get_pass(args.user.as_str())?;
    let hub = setup(args.user.as_str(), pass.as_str(), args.ip.as_str()).await?;

    let device_info = hub.get_device_info().await?;
    debug!("Device Info\n{device_info:#?}");
    info!(
        "Connected to {} named {}",
        device_info.r#type, device_info.nickname
    );

    loop {
        info!("Fetching");
        get_print_temp_data(&hub, &args.device).await?;
        info!("Sleeping for {}s", args.interval);
        sleep(Duration::from_secs(args.interval)).await;
    }
}
