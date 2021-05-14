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
			let selfc = self.clone();
			move |_| {
				selfc.lbl_path.resize_to_text().unwrap();
				true
			}
		});

		self.wnd.on().wm_command_accel_menu(co::DLGID::CANCEL.into(), {
			let selfc = self.clone();
			move || {
				selfc.wnd.hwnd().SendMessage(msg::wm::Close {});
			}
		});

		self.btn_choose.on().bn_clicked({
			let selfc = self.clone();
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

				if fileo.Show(selfc.wnd.hwnd()).unwrap() {
					selfc.txt_path.set_text(
						&fileo.GetResult().unwrap()
							.GetDisplayName(co::SIGDN::FILESYSPATH).unwrap(),
					).unwrap();

					selfc.btn_patch.hwnd().EnableWindow(true);
					selfc.wnd.hwnd().SendMessage(msg::wm::NextDlgCtl {
						hwnd_focus: w::HwndFocus::Hwnd(selfc.btn_patch.hwnd()),
					});
				}
			}
		});

		self.btn_patch.on().bn_clicked({
			let selfc = self.clone();
			move || {
				let target = selfc.txt_path.text().unwrap();

				if target.is_empty() {
					selfc.wnd.hwnd().MessageBox(
						"No installation path given.",
						"No path", co::MB::ICONERROR).unwrap();
					selfc.btn_choose.hwnd().SetFocus();
					return;
				}

				if patch::is_vscode_running().unwrap() {
					let res = selfc.wnd.hwnd().MessageBox(
						"VS Code appears to be running.\n\
						It's recommended to close it before patching.\n\n\
						Proceed anyway?",
						"VS Code running",
						co::MB::ICONEXCLAMATION | co::MB::YESNO,
					).unwrap();

					if res != co::DLGID::YES {
						return;
					}
				}

				let start = Instant::now();

				if let Err(e) = patch::patch_installation(&target) {
					selfc.wnd.hwnd().MessageBox(
						&e.to_string(), "Patching error", co::MB::ICONERROR).unwrap();
					return;
				}

				selfc.wnd.hwnd().MessageBox(
					&format!("Installation successfully patched in {}μs.",
						start.elapsed().as_micros()),
					"Done", co::MB::ICONINFORMATION).unwrap();
			}
		});
	}
}
