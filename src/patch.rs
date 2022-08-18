use winsafe::{prelude::*, self as w, co};

pub fn is_vscode_running() -> w::SysResult<bool> {
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

pub fn patch_font(install_dir: &str) -> w::AnyResult<()> {
	let css_path = _build_css_path(install_dir);
	let orig_contents = _read_css_contents(&css_path)?;

	const END_OF_COMMS: &str = "-*/";
	const MAGIC_PATCH: &str = "\n*{text-shadow:transparent 0px 0px 0px, rgba(0, 0, 0, 0.5) 0px 0px 0px !important;}";

	// Find index past the comments block.
	let mut idx_start_code = match orig_contents.find(END_OF_COMMS) {
		Some(idx) => idx,
		None => return Err("CSS end of comments not found.".into()),
	};

	idx_start_code += END_OF_COMMS.len();

	// Is our magic path the first thing past the comments block?
	if MAGIC_PATCH == &orig_contents[idx_start_code..(idx_start_code + MAGIC_PATCH.len())] {
		return Err("Font already patched.\n\nNothing to do.".into());
	}

	let mut new_contents = String::with_capacity(orig_contents.len() + MAGIC_PATCH.len());
	new_contents.push_str(&orig_contents[..idx_start_code]); // comments block
	new_contents.push_str(MAGIC_PATCH);
	new_contents.push_str(&orig_contents[idx_start_code..]); // rest of file

	_write_replace_css_contents(&css_path, &new_contents)?;
	Ok(())
}

pub fn patch_icon(install_dir: &str) -> w::AnyResult<()> {
	let css_path = _build_css_path(install_dir);
	let orig_contents = _read_css_contents(&css_path)?;

	const NATURAL: &str = ".monaco-editor .suggest-widget .monaco-list .monaco-list-row.focused .codicon{color:var(--vscode-editorSuggestWidget-selectedIconForeground)}";
	const PATCHED: &str = " /*.monaco-editor .suggest-widget .monaco-list .monaco-list-row.focused .codicon{color:var(--vscode-editorSuggestWidget-selectedIconForeground)}*/ ";

	if let Some(_) = orig_contents.find(PATCHED) {
		return Err("Suggestion box icon already patched.\n\nNothing to do.".into());
	}

	let idx_part = match orig_contents.find(NATURAL) {
		Some(idx) => idx,
		None => return Err("Suggestion box icon CSS entry not found.".into()),
	};

	let mut new_contents = String::with_capacity(orig_contents.len() + PATCHED.len() - NATURAL.len());
	new_contents.push_str(&orig_contents[..idx_part]); // all code up to part
	new_contents.push_str(PATCHED);
	new_contents.push_str(&orig_contents[idx_part + NATURAL.len()..]); // rest of file

	_write_replace_css_contents(&css_path, &new_contents)?;
	Ok(())
}

fn _build_css_path(install_dir: &str) -> String {
	format!("{}\\{}",
		w::path::rtrim_backslash(install_dir),
		"resources\\app\\out\\vs\\workbench\\workbench.desktop.main.css")
}

fn _read_css_contents(css_path: &str) -> w::SysResult<String> {
	let fin = w::FileMapped::open(css_path, w::FileAccess::ExistingReadOnly)?;
	let s = w::WString::parse_str(fin.as_slice())?.to_string();
	Ok(s)
}

fn _write_replace_css_contents(css_path: &str, new_contents: &str) -> w::SysResult<()> {
	let fout = w::File::open(css_path, w::FileAccess::ExistingRW)?;
	fout.erase_and_write(new_contents.as_bytes())?;
	Ok(())
}
