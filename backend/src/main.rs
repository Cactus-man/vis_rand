#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub use dice::{roll_threaded, DieKind};
use serde::Deserialize;

fn main() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        //.menu(tauri::Menu::os_default(&context.package_info().name))
        .invoke_handler(tauri::generate_handler![roll])
        .run(context)
        .expect("error while running tauri application");
}

#[derive(Deserialize)]
#[serde(tag = "variant")]
pub enum FrontDie {
    #[serde(rename = "simple")]
    Simple { sides: usize },
    #[serde(rename = "weighted")]
    Weighted { sides: Vec<(usize, u16)> },
}

#[derive(Deserialize)]
pub struct Panel {
    die: FrontDie,
    amount: u8,
}

#[tauri::command]
fn roll(rolls: usize, dice: Vec<Panel>) -> Vec<(usize, usize)> {
    let dice: Vec<(DieKind, u8)> = dice
        .into_iter()
        .map(|Panel { die, amount }| (die, amount))
        .map(|(die, n)| {
            let die = match die {
                FrontDie::Simple { sides } => DieKind::simple(sides),
                FrontDie::Weighted { sides } => DieKind::weighted(&sides),
            };
            (die, n)
        })
        .collect();

    let mut rolls: Vec<_> = roll_threaded(dice, rolls).into_iter().collect();
    rolls.sort_unstable_by_key(|&(d, _)| d);
    return rolls;
}
