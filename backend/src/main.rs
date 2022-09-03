#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub use dice::{roll_threaded, DieKind};
use serde::{Deserialize, Serialize};

fn main() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        //.menu(tauri::Menu::os_default(&context.package_info().name))
        .invoke_handler(tauri::generate_handler![roll])
        .run(context)
        .expect("error while running tauri application");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SideData {
    dots: u16,
    occurrences: usize,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "variant")]
pub enum InnerData {
    #[serde(rename = "simple")]
    Simple { sides: usize },
    #[serde(rename = "weighted")]
    Weighted { sides: Vec<SideData> },
}

#[derive(Debug, Deserialize)]
pub struct DieData {
    rolls: u8,
    #[serde(flatten)]
    inner: InnerData,
}

#[tauri::command]
fn roll(total: usize, dice: Vec<DieData>) -> Vec<SideData> {
    let dice: Vec<(DieKind, u8)> = dice
        .into_iter()
        .map(|DieData { inner, rolls }| (inner, rolls))
        .map(|(inner, n)| {
            let die = match inner {
                InnerData::Simple { sides } => DieKind::simple(sides),
                InnerData::Weighted { sides } => {
                    let sides: Vec<_> =
                        sides.into_iter().map(|s| (s.occurrences, s.dots)).collect();
                    DieKind::weighted(&sides)
                }
            };
            (die, n)
        })
        .collect();

    let mut rolls: Vec<_> = roll_threaded(dice, total)
        .into_iter()
        .map(|(dots, occurrences)| SideData {
            dots: u16::try_from(dots).unwrap(),
            occurrences,
        })
        .collect();
    rolls.sort_unstable_by_key(|side| side.dots);
    return rolls;
}
