use winsafe::{self as w, co, msg, shell};

use crate::util;
use super::WndMain;
use super::patch;

impl WndMain {
	pub(super) fn events(&self) {
		self.wnd.on().wm_init_dialog({
			let self2 = self.clone();
			move |_: msg::wm::InitDialog| -> bool {
				self2.lbl_path.resize_to_text().unwrap();
				true
			}
		});

		self.wnd.on().wm_command_accel_menu(co::DLGID::CANCEL.into(), {
			let self2 = self.clone();
			move || {
				self2.wnd.hwnd().SendMessage(msg::wm::Close {});
			}
		});

		self.btn_choose.on().bn_clicked({
			let self2 = self.clone();
			move || {
				let fileo: shell::IFileOpenDialog = w::CoCreateInstance(
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

					self2.btn_patch.hwnd().EnableWindow(true);
					self2.wnd.hwnd().SendMessage(msg::wm::NextDlgCtl {
						hwnd_focus: w::HwndFocus::Hwnd(self2.btn_patch.hwnd()),
					});
				}
			}
		});

		self.btn_patch.on().bn_clicked({
			let self2 = self.clone();
			move || {
				let target = self2.txt_path.text().unwrap();

				if target.is_empty() {
					util::prompt::err(self2.wnd.hwnd(), "No path", "No installation path given.");
					self2.btn_choose.hwnd().SetFocus();
					return;
				}

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

				if let Err(e) = patch::patch_installation(&target) {
					util::prompt::err(self2.wnd.hwnd(), "Patching error", &e.to_string());
					return;
				}

				util::prompt::info(self2.wnd.hwnd(), "Operation successful",
					&format!("Installation successfully patched in {:.2}ms.", clock.now_ms()));
			}
		});
	}
}
