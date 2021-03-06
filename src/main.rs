#![feature(iterator_fold_self)]
#![feature(in_band_lifetimes)]
#![feature(const_fn)]
#![feature(test)]

use memlib::logger::MinimalLogger;
use memlib::memory;

use log::*;
use anyhow::*;
use std::error::Error;
use memlib::memory::handle_interfaces::driver_handle::{DriverProcessHandle};
use memory::Handle;
use win_key_codes::VK_INSERT;
use msgbox::IconType;
use winutil::{HWND, get_windows};
use std::time::Duration;
use window_overlay::window::OverlayWindow;

mod sdk;
mod hacks;
mod config;
mod gui;

pub const PROCESS_NAME: &str = "ModernWarfare.exe";
pub const CHEAT_TICKRATE: u64 = 90;

const LOG_LEVEL: LevelFilter = LevelFilter::Debug;

fn run() -> Result<()> {
    // Initialize the logger
    MinimalLogger::init(LOG_LEVEL)?;

    // Create a handle to the game
    let handle = loop {
        match DriverProcessHandle::attach(PROCESS_NAME) {
            Ok(driver) => break Handle::from_interface(driver),
            Err(e) => {
                error!("{:?}", e);
                std::thread::sleep(Duration::from_secs(1));
            },
        }
    };

    sdk::init(handle)?;

    // Run the hack loop
    hacks::hack_main()?;

    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => {
            info!("Exiting cheat");
            0
        }
        Err(err) => {
            error!("{}", err);
            msgbox::create("Error", &err.to_string(), IconType::Error);
            1
        }
    })
}


fn find_cod_window(cod_pid: u32) -> Option<HWND> {
    get_windows().into_iter()
        .filter(|window| window.pid == cod_pid)
        .filter(|window| {
            if let Some(title) = &window.title {
                if title == "MSCTFIME UI" || title == "IME" {
                    return false;
                }
            }
            true
        })
        .map(|w| w.hwnd)
        .next()
}