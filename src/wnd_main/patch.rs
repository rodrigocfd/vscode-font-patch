pub fn patch_installation(install_dir: &str) {
	let css_path = build_css_path(install_dir);


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
