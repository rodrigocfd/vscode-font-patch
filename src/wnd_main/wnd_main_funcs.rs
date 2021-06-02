use winsafe as w;
use winsafe::gui;

use crate::ids;
use super::WndMain;

impl WndMain {
	pub fn new() -> Self {
		let wnd        = gui::WindowMain::new_dlg(ids::DIALOG_MAIN, Some(ids::MAIN_ICON), None);
		let lbl_path   = gui::Label::new_dlg(&wnd, ids::LBL_PATH);
		let txt_path   = gui::Edit::new_dlg(&wnd, ids::TXT_PATH);
		let btn_choose = gui::Button::new_dlg(&wnd, ids::BTN_CHOOSE);
		let btn_patch  = gui::Button::new_dlg(&wnd, ids::BTN_PATCH);

		let self2 = Self { wnd, lbl_path, txt_path, btn_choose, btn_patch };
		self2.events();
		self2
	}

	pub fn run(&self) -> w::WinResult<()> {
		self.wnd.run_main(None)
	}
}
