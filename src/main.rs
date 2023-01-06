// Author:  Daniel Garcia
// ver:     0.1.1-alpha
// DESC:    Simple CLI tool to transfer Grim Dawn save files from Steam's remote 
//          save directory to the default save directory and vice-versa


//  TODO:   fix formatting to match expected rust formatting
//          both for fns and top of file

// https://docs.rs/steamlocate/latest/steamlocate/
extern crate steamlocate;
// https://docs.rs/steamid-ng/latest/steamid_ng/
extern crate steamid_ng;
// https://docs.rs/dirs-next/latest/dirs_next/
extern crate dirs_next;
// https://docs.rs/clap/latest/clap/
extern crate clap;

use std::path::PathBuf;
use steamlocate::{SteamDir, SteamApp};
use steamid_ng::SteamID;
use dirs_next::document_dir;
use clap::Parser;

// FIXME add in tags for proper arg flags
#[derive(Parser)]
// FIXME about and author are not being picked up here?
#[command(author, version, about, long_about = None)]
struct Args
{
    // SteamID3 as string
    steamid3 : String,
    #[arg(short, long)]
    debug : bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    // parse args w/clap
    let passed_args = Args::parse();

    // FIXME need proper validation of steamid3 passed from args
    // currently just defaults to a bogus ID
    let steam_id : SteamID = SteamID::from_steam3(&passed_args.steamid3)
                                    .unwrap_or(SteamID::from_steam3("[U:1:1234567]").unwrap());
    dbg!(&steam_id);
    let steam_account_id : u32 = SteamID::account_id(&steam_id);
    dbg!(&steam_account_id);

    // check if Steam install exists
    if get_steam_dir().is_none()
    {
        // FIXME we should handle this with more grace
        panic!("No Steam install found")
    }
    let mut steam_dir : SteamDir = get_steam_dir().unwrap();

    // check if Grim Dawn is installed
    if get_grim_dawn_dir(&mut steam_dir).is_none()
    {
        // FIXME we should handle this with more grace
        panic!("No Grim Dawn install directory found");
    }

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
        // FIXME we should handle this with more grace
        panic!("No default \"Documents\" directory found");
    }
    let mut steam_doc_save_dir : PathBuf = PathBuf::new();
    steam_doc_save_dir.push(get_docs_dir().unwrap().as_path());
    steam_doc_save_dir.push("My Games\\Grim Dawn\\save");
    dbg!(&steam_doc_save_dir);

    // check that dirs exist
    match steam_doc_save_dir.try_exists()
    {
        Ok(_) => (),
        // FIXME we should handle this with more grace
        Err(e) => panic!("Error: {}", e)
    }

    match steam_doc_save_dir.try_exists()
    {
        Ok(_) => (),
        // FIXME we should handle this with more grace
        Err(e) => panic!("Error: {}", e)
    }

    Ok(())

}

/// Returns Option\<std::path::PathBuf\> of default 'documents' directory.
/// 
/// https://docs.rs/dirs-next/latest/dirs_next/
fn get_docs_dir() -> Option<PathBuf>
{
    match document_dir()
    {
        Some(dir) => return Some(dir),
        None => return None
    }
}

/// Returns Option\<Steamlocate::SteamDir\> of Steam install directory.
/// 
/// https://docs.rs/steamlocate/latest/steamlocate/
fn get_steam_dir() -> Option<SteamDir>
{
    match SteamDir::locate()
    {
        Some(dir) => return Some(dir),
        None => return None
    }
}

/// Returns Option\<Steamlocate::SteamApp\> of Grim Dawn install directory.
/// 
/// https://docs.rs/steamlocate/latest/steamlocate/
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

// fn copy_docs_to_steam()
// {

// }

// fn copy_steam_to_docs()
// {

// }