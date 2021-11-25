use winsafe::gui;

mod ids;
mod wnd_main_events;
mod wnd_main_funcs;

#[derive(Clone)]
pub struct WndMain {
	wnd:             gui::WindowMain,
	lbl_path:        gui::Label,
	txt_path:        gui::Edit,
	btn_choose:      gui::Button,
	chk_patch_font:  gui::CheckBox,
	chk_patch_theme: gui::CheckBox,
	btn_patch:       gui::Button,
}
