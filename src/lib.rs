#![feature(proc_macro_hygiene)]
#![feature(with_options)]
#![feature(const_mut_refs)]
#![feature(exclusive_range_pattern)]

pub mod common;
mod hazard_manager;
mod hitbox_visualizer;
mod training;

#[cfg(test)]
mod test;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate num_derive;

use crate::common::*;
use crate::menu::set_menu_from_url;

use skyline::libc::mkdir;
use std::fs;
use skyline::nro::{self, NroInfo};

use owo_colors::OwoColorize;

fn nro_main(nro: &NroInfo<'_>) {
    if nro.module.isLoaded {
        return;
    }

    if nro.name == "common" {
        skyline::install_hooks!(
            training::shield::handle_sub_guard_cont,
            training::directional_influence::handle_correct_damage_vector_common,
            training::sdi::process_hit_stop_delay,
            training::tech::handle_change_status,
        );
    }
}

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr();
    };
}

#[cfg(not(test))]
#[skyline::main(name = "training_modpack")]
pub fn main() {
    macro_rules! log {
        ($($arg:tt)*) => {
            print!("{}", "[Training Modpack] ".green());
            println!($($arg)*);
        };
    }

    log!("Initialized.");
    hitbox_visualizer::hitbox_visualization();
    hazard_manager::hazard_manager();
    training::training_mods();
    nro::add_hook(nro_main).unwrap();

    mkdir(c_str!("sd:/TrainingModpack/"), 777);

    let ovl_path = "sd:/switch/.overlays/ovlTrainingModpack.ovl";
    if !fs::metadata(ovl_path).is_err() {
        log!("Removing ovlTrainingModpack.ovl...");
        fs::remove_file(ovl_path).unwrap();
    }

    log!("Performing version check...");
    release::version_check();

    
    let menu_conf_path = "sd:/TrainingModpack/training_modpack_menu.conf";
    if !fs::metadata(menu_conf_path).is_err() {
        log!("[Training Modpack] Loading previous menu from training_modpack_menu.conf...");
        let menu_conf = fs::read(menu_conf_path).unwrap();
        set_menu_from_url(menu_conf);
    }
}
