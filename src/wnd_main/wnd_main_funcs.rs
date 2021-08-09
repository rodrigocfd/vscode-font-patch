use winsafe::{self as w, gui};

use crate::ids;
use super::WndMain;

impl WndMain {
	pub fn new() -> Self {
		let wnd             = gui::WindowMain::new_dlg(ids::DIALOG_MAIN, Some(ids::MAIN_ICON), None);
		let lbl_path        = gui::Label::new_dlg(&wnd, ids::LBL_PATH);
		let txt_path        = gui::Edit::new_dlg(&wnd, ids::TXT_PATH);
		let btn_choose      = gui::Button::new_dlg(&wnd, ids::BTN_CHOOSE);
		let chk_patch_font  = gui::CheckBox::new_dlg(&wnd, ids::CHK_PATCH_FONT);
		let chk_patch_theme = gui::CheckBox::new_dlg(&wnd, ids::CHK_PATCH_THEME);
		let btn_patch       = gui::Button::new_dlg(&wnd, ids::BTN_PATCH);

		let self2 = Self {
			wnd,
			lbl_path, txt_path, btn_choose,
			chk_patch_font, chk_patch_theme,
			btn_patch,
		};
		self2.events();
		self2
	}

	pub fn run(&self) -> w::WinResult<()> {
		self.wnd.run_main(None)
	}

	pub(super) fn maybe_enable_btn_run(&self) {
		self.btn_patch.hwnd().EnableWindow(
			!self.txt_path.text().unwrap().is_empty()
				&& (self.chk_patch_font.is_checked() || self.chk_patch_theme.is_checked()),
		);
	}
}
