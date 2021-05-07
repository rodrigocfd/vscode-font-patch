use winsafe::gui;

mod patch;
mod wnd_main_events;
mod wnd_main_funcs;

#[derive(Clone)]
pub struct WndMain {
	wnd:        gui::WindowMain,
	lbl_path:   gui::Label,
	txt_path:   gui::Edit,
	btn_choose: gui::Button,
	btn_patch:  gui::Button,
}
