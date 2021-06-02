use std::time::Instant;

use winsafe as w;
use winsafe::co;
use winsafe::msg;
use winsafe::shell;

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
						| co::FOS::FILEMUSTEXIST | co::FOS::PICKFOLDERS,
				).unwrap();

				if fileo.Show(self2.wnd.hwnd()).unwrap() {
					self2.txt_path.set_text(
						&fileo.GetResult().unwrap()
							.GetDisplayName(co::SIGDN::FILESYSPATH).unwrap(),
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
				let target = self2.txt_path.text_str().unwrap();

				if target.is_empty() {
					self2.wnd.hwnd().TaskDialog(
						None,
						Some(&self2.wnd.hwnd().GetWindowTextStr().unwrap()),
						Some("No path"),
						Some("No installation path given."),
						co::TDCBF::OK,
						w::IdTdicon::Tdicon(co::TD_ICON::ERROR),
					).unwrap();

					self2.btn_choose.hwnd().SetFocus();
					return;
				}

				if patch::is_vscode_running().unwrap() {
					let res = self2.wnd.hwnd().TaskDialog(
						None,
						Some(&self2.wnd.hwnd().GetWindowTextStr().unwrap()),
						Some("VS Code appears to be running"),
						Some("It's recommended to close VS Code before patching.\n\
							Proceed anyway?"),
						co::TDCBF::YES | co::TDCBF::NO,
						w::IdTdicon::Tdicon(co::TD_ICON::WARNING),
					).unwrap();

					if res != co::DLGID::YES {
						return;
					}
				}

				let start = Instant::now();

				if let Err(e) = patch::patch_installation(&target) {
					self2.wnd.hwnd().TaskDialog(
						None,
						Some(&self2.wnd.hwnd().GetWindowTextStr().unwrap()),
						Some("Patching error"),
						Some(&e.to_string()),
						co::TDCBF::OK,
						w::IdTdicon::Tdicon(co::TD_ICON::ERROR),
					).unwrap();

					return;
				}

				self2.wnd.hwnd().TaskDialog(
					None,
					Some(&self2.wnd.hwnd().GetWindowTextStr().unwrap()),
					Some("Operation successful"),
					Some(&format!("Installation successfully patched in {}μs.",
						start.elapsed().as_micros())),
					co::TDCBF::OK,
					w::IdTdicon::Tdicon(co::TD_ICON::INFORMATION),
				).unwrap();
			}
		});
	}
}
