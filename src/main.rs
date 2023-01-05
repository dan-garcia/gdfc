// Author:  Daniel Garcia
// date:    26DEC22
// ver:     0.1.1-alpha
// DESC:    To be used to copy save files for Grim Dawn from Documents location to
//          Steam remote location and vice versa.
// DEPS:    steamlocate 0.*
//          steamid-ng  1.*
//          dirs_next   2.*

// https://docs.rs/steamlocate/latest/steamlocate/
extern crate steamlocate;
// https://docs.rs/steamid-ng/latest/steamid_ng/
extern crate steamid_ng;
// https://docs.rs/dirs-next/latest/dirs_next/
extern crate dirs_next;
use std::path::PathBuf;
use steamlocate::{SteamDir, SteamApp};
use steamid_ng::SteamID;
use dirs_next::document_dir;

//  TODO:   fix formatting to match expected rust formatting
//          both for fns and top of file

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    //check if Steam install exists
    if get_steam_dir().is_none()
    {
        panic!("No Steam install found")
    }
    let mut steam_dir : SteamDir = get_steam_dir().unwrap();

    //check if Grim Dawn is installed
    if get_grim_dawn_dir(&mut steam_dir).is_none()
    {
        panic!("No Grim Dawn install directory found");
    }

    // FIXME need to take in steam3 (or steamid or steam2) as an arg
    // and parse appropriately to pass into SteamID::from_*
    // as well as change to unwrap_or() or unwrap_or_*()
    let steam3_id : SteamID = SteamID::from_steam3("[U:1:6620387]").unwrap();
    let steam_account_id : u32 = SteamID::account_id(&steam3_id);
    dbg!(&steam_account_id);

    // assemble path for remote save directory
    let steam_install_dir : &PathBuf = &steam_dir.path;
    let mut steam_remote_save_dir : PathBuf = steam_install_dir.to_path_buf();
    steam_remote_save_dir.push("userdata\\");
    steam_remote_save_dir.push(steam_account_id.to_string());
    steam_remote_save_dir.push("219990\\remote\\save");
    dbg!(&steam_remote_save_dir);

    // assemble path for windows steam non-remote save directory
    if get_docs_dir().is_none()
    {
        panic!("No default \"Documents\" directory found");
    }
    let mut steam_mydoc_save_dir : PathBuf = PathBuf::new();
    steam_mydoc_save_dir.push(get_docs_dir().unwrap().as_path());
    steam_mydoc_save_dir.push("My Games\\Grim Dawn\\save");
    dbg!(&steam_mydoc_save_dir);

    Ok(())

}

fn get_docs_dir() -> Option<PathBuf>
{
    match document_dir()
    {
        Some(dir) => return Some(dir),
        None => return None
    }
}

fn get_steam_dir() -> Option<SteamDir>
{
    match SteamDir::locate()
    {
        Some(dir) => return Some(dir),
        None => return None
    }
}

// we need a mutable ref here because steamlocate::SteamDir.app()
// takes a &mut
fn get_grim_dawn_dir(sdir : &mut SteamDir) -> Option<SteamApp>
{
    match sdir.app(&219990)
    {
        Some(app) => return Some(app.to_owned()),
        None => panic!("No Grim Dawn install directory found")
    }
}

fn copy_mydoc_to_steam()
{

}

fn copy_steam_to_mydoc()
{

}