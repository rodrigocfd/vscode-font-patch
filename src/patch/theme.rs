use std::error::Error;
use winsafe::{self as w, co};

pub fn build_path(install_dir: &str) -> String {
	const DLP_JSON: &str = "resources\\app\\extensions\\theme-defaults\\themes\\light_vs.json";

	let mut css_path = String::with_capacity(install_dir.len() + 1 + DLP_JSON.len());
	css_path.push_str(install_dir);
	if !css_path.ends_with("\\") {
		css_path.push('\\');
	}
	css_path.push_str(DLP_JSON);

	css_path
}

pub fn fix_theme_json(dlp_path: &str) -> Result<(), Box<dyn Error>> {
	let mut new_contents = String::default();
	let mut found = false;

	{
		let mapin = w::MappedFile::open(dlp_path, w::MappedFileAccess::Read)?;
		let contents = String::from_utf8(mapin.as_slice().to_vec())?;
		new_contents.reserve(contents.len() + 3);
		for line in contents.lines() {
			if line.trim() == "\"list.activeSelectionIconForeground\": \"#FFF\"" {
				found = true;
				new_contents.push_str("\t\t// \"list.activeSelectionIconForeground\": \"#FFF\""); // commented
			} else {
				new_contents.push_str(line);
			}
			new_contents.push('\n');
		}
	}

	if !found {
		return Err("Default Light+ theme setting not found, not patched.".into());
	}

	let (hfile, _) = w::HFILE::CreateFile(dlp_path, co::GENERIC::READ | co::GENERIC::WRITE,
		co::FILE_SHARE::NoValue, None, co::DISPOSITION::TRUNCATE_EXISTING,
		co::FILE_ATTRIBUTE::NORMAL, None)?;
	defer! { hfile.CloseHandle().unwrap(); }

	hfile.WriteFile(new_contents.as_bytes(), None)?;
	Ok(())
}
