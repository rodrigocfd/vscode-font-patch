use winsafe::{prelude::*, self as w, co};

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

pub fn read_contents(css_path: &str) -> w::ErrResult<String> {
	let mapin = w::FileMapped::open(css_path, w::FileAccess::ExistingReadOnly)?;
	let contents = String::from_utf8(
		mapin.as_slice().to_vec(),
	)?;
	Ok(contents)
}

pub fn apply_patch(orig_contents: &str) -> w::ErrResult<String> {
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

pub fn write_contents(css_path: &str, new_contents: &str) -> w::ErrResult<()> {
	let (hfile, _) = w::HFILE::CreateFile(css_path,
		co::GENERIC::READ | co::GENERIC::WRITE,
		co::FILE_SHARE::NoValue, None,
		co::DISPOSITION::TRUNCATE_EXISTING,
		co::FILE_ATTRIBUTE::NORMAL, None)?;

	hfile.WriteFile(new_contents.as_bytes(), None)?;
	hfile.CloseHandle().map_err(|e| e.into())
}
