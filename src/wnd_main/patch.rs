use std::error::Error;

use winsafe as w;
use winsafe::co;

pub fn patch_installation(install_dir: &str) -> Result<(), Box<dyn Error>> {
	let css_path = build_css_path(install_dir);
	let orig_contents = read_css_contents(&css_path)?;

	println!("{}", orig_contents);

	Ok(())
}

fn build_css_path(install_dir: &str) -> String {
	const INNER_CSS: &str = "resources\\app\\out\\vs\\workbench\\workbench.desktop.main.css";

	let mut css_path = String::with_capacity(install_dir.len() + 1 + INNER_CSS.len());
	css_path.push_str(install_dir);
	if !css_path.ends_with("\\") {
		css_path.push('\\');
	}
	css_path.push_str(INNER_CSS);

	css_path
}

fn read_css_contents(css_path: &str) -> Result<String, Box<dyn Error>> {
	let file_exists = w::GetFileAttributes(css_path).is_ok();
	if !file_exists {
		return Err(format!("File does not exist: {}", css_path).into());
	}

	let hfile = w::HFILE::CreateFile(css_path, co::GENERIC::READ,
		co::FILE_SHARE::READ, None, co::DISPOSITION::OPEN_EXISTING,
		co::FILE_ATTRIBUTE::NORMAL, None)?;

	let contents = String::from_utf8(
		hfile.ReadFile(hfile.GetFileSizeEx()? as _, None)?
	).unwrap();

	hfile.CloseHandle()?;
	Ok(contents)
}
