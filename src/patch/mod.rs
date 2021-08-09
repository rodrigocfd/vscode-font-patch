use std::error::Error;
use winsafe::{self as w, co};

mod font;
mod theme;

pub fn is_vscode_running() -> Result<bool, Box<dyn Error>> {
	let hpl = w::HPROCESSLIST::CreateToolhelp32Snapshot(co::TH32CS::SNAPPROCESS, None)?;
	defer! { hpl.CloseHandle().unwrap(); }

	let mut pe = w::PROCESSENTRY32::default();
	let mut found = false;

	if hpl.Process32First(&mut pe)? {
		loop {
			if pe.szExeFile() == "Code.exe" {
				found = true;
				break;
			}
			if !hpl.Process32Next(&mut pe)? {
				break;
			}
		}
	}

	Ok(found)
}

pub fn patch_installation(
	install_dir: &str,
	patch_font: bool, patch_theme: bool) -> Result<(), Box<dyn Error>>
{
	if patch_font {
		let css_path = font::build_path(install_dir);
		let orig_contents = font::read_contents(&css_path)?;
		let new_contents = font::apply_patch(&orig_contents)?;
		font::write_contents(&css_path, &new_contents)?;
	}

	if patch_theme {
		let dlp_path = theme::build_path(install_dir);
		theme::fix_theme_json(&dlp_path)?;
	}

	Ok(())
}
