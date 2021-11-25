use winsafe::{prelude::*, self as w, co};

mod font;
mod theme;

pub fn is_vscode_running() -> w::ErrResult<bool> {
	let hpl = w::HPROCESSLIST::CreateToolhelp32Snapshot(co::TH32CS::SNAPPROCESS, None)?;
	let mut pe = w::PROCESSENTRY32::default();
	let mut found = false;

	for pe in hpl.iter(&mut pe) {
		if pe?.szExeFile() == "Code.exe" {
			found = true;
			break;
		}
	}

	hpl.CloseHandle()?;
	Ok(found)
}

pub fn patch_installation(
	install_dir: &str,
	patch_font: bool, patch_theme: bool) -> w::ErrResult<()>
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
