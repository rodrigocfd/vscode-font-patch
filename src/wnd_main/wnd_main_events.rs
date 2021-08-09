use winsafe::{self as w, co, msg, shell};

use crate::patch;
use crate::util;
use super::WndMain;

impl WndMain {
	pub(super) fn events(&self) {
		self.wnd.on().wm_init_dialog({
			let self2 = self.clone();
			move |_: msg::wm::InitDialog| -> bool {
				self2.lbl_path.resize_to_text().unwrap();
				self2.chk_patch_font.set_check(true);
				self2.chk_patch_theme.set_check(true);
				true
			}
		});

		self.wnd.on().wm_command_accel_menu(co::DLGID::CANCEL.into(), {
			let self2 = self.clone();
			move || {
				self2.wnd.hwnd().SendMessage(msg::wm::Close {});
			}
		});

		self.chk_patch_font.on().bn_clicked({
			let self2 = self.clone();
			move || {
				self2.maybe_enable_btn_run();
			}
		});

		self.chk_patch_theme.on().bn_clicked({
			let self2 = self.clone();
			move || {
				self2.maybe_enable_btn_run();
			}
		});

		self.btn_choose.on().bn_clicked({
			let self2 = self.clone();
			move || {
				let fileo = w::CoCreateInstance::<shell::IFileOpenDialog>(
					&shell::clsid::FileOpenDialog,
					None,
					co::CLSCTX::INPROC_SERVER,
				).unwrap();

				fileo.SetOptions(
					fileo.GetOptions().unwrap()
						| shell::co::FOS::FILEMUSTEXIST
						| shell::co::FOS::PICKFOLDERS,
				).unwrap();

				if fileo.Show(self2.wnd.hwnd()).unwrap() {
					self2.txt_path.set_text(
						&fileo.GetResult().unwrap()
							.GetDisplayName(shell::co::SIGDN::FILESYSPATH).unwrap(),
					).unwrap();

					self2.maybe_enable_btn_run();
					if self2.btn_patch.hwnd().IsWindowEnabled() {
						self2.wnd.hwnd().SendMessage(msg::wm::NextDlgCtl {
							hwnd_focus: w::HwndFocus::Hwnd(self2.btn_patch.hwnd()),
						});
					}
				}
			}
		});

		self.btn_patch.on().bn_clicked({
			let self2 = self.clone();
			move || {
				if patch::is_vscode_running().unwrap() {
					if util::prompt::ok_cancel(self2.wnd.hwnd(),
						"VS Code appears to be running",
						"It's recommended to close VS Code before patching.\n\
							Proceed anyway?") != co::DLGID::OK
					{
						return;
					}
				}

				let clock = util::Timer::start();

				if let Err(e) = patch::patch_installation(
					&self2.txt_path.text().unwrap(),
					self2.chk_patch_font.is_checked(),
					self2.chk_patch_theme.is_checked(),
				) {
					util::prompt::err(self2.wnd.hwnd(), "Patching error", &e.to_string());
					return;
				}

				util::prompt::info(self2.wnd.hwnd(), "Operation successful",
					&format!("Installation successfully patched in {:.2}ms.", clock.now_ms()));
			}
		});
	}
}
