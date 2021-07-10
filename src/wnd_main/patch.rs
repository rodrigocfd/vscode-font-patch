use std::error::Error;
use winsafe::{self as w, co};

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

pub fn patch_installation(install_dir: &str) -> Result<(), Box<dyn Error>> {
	let css_path = css::build_path(install_dir);
	let orig_contents = css::read_contents(&css_path)?;
	let new_contents = css::apply_patch(&orig_contents)?;
	css::write_contents(&css_path, &new_contents)?;

	let dlp_path = dlp::build_path(install_dir);
	dlp::fix_theme_json(&dlp_path)
}

mod css {
	use std::error::Error;
	use winsafe::{self as w, co};

	pub fn build_path(install_dir: &str) -> String {
		const INNER_CSS: &str = "resources\\app\\out\\vs\\workbench\\workbench.desktop.main.css";

		let mut css_path = String::with_capacity(install_dir.len() + 1 + INNER_CSS.len());
		css_path.push_str(install_dir);
		if !css_path.ends_with("\\") {
			css_path.push('\\');
		}
		css_path.push_str(INNER_CSS);

		css_path
	}

	pub fn read_contents(css_path: &str) -> Result<String, Box<dyn Error>> {
		let mapin = w::MappedFile::open(css_path, w::MappedFileAccess::Read)?;
		let contents = String::from_utf8(
			mapin.as_slice().to_vec(),
		)?;
		Ok(contents)
	}

	pub fn apply_patch(orig_contents: &str) -> Result<String, Box<dyn Error>> {
		const END_OF_COMMS: &str = "-*/";
		const MAGIC_PATCH: &str = "*{text-shadow:transparent 0px 0px 0px, rgba(0, 0, 0, 0.5) 0px 0px 0px !important;}";

		let mut idx_start_code = match orig_contents.find(END_OF_COMMS) {
			Some(idx) => idx,
			None => return Err("End of comments not found.".into()),
		};

		idx_start_code += END_OF_COMMS.len();

		if MAGIC_PATCH == &orig_contents[idx_start_code..(idx_start_code + MAGIC_PATCH.len())] {
			return Err("Installation already patched, nothing to do.".into());
		}

		let mut new_contents = String::with_capacity(orig_contents.len() + MAGIC_PATCH.len());
		new_contents.push_str(&orig_contents[..idx_start_code]);
		new_contents.push_str(MAGIC_PATCH);
		new_contents.push_str(&orig_contents[idx_start_code..]);

		Ok(new_contents)
	}

	pub fn write_contents(css_path: &str, new_contents: &str) -> Result<(), Box<dyn Error>> {
		let (hfile, _) = w::HFILE::CreateFile(css_path, co::GENERIC::READ | co::GENERIC::WRITE,
			co::FILE_SHARE::NONE, None, co::DISPOSITION::TRUNCATE_EXISTING,
			co::FILE_ATTRIBUTE::NORMAL, None)?;
		defer! { hfile.CloseHandle().unwrap(); }

		hfile.WriteFile(new_contents.as_bytes(), None)?;
		Ok(())
	}
}

mod dlp {
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
			co::FILE_SHARE::NONE, None, co::DISPOSITION::TRUNCATE_EXISTING,
			co::FILE_ATTRIBUTE::NORMAL, None)?;
		defer! { hfile.CloseHandle().unwrap(); }

		hfile.WriteFile(new_contents.as_bytes(), None)?;
		Ok(())
	}
}
