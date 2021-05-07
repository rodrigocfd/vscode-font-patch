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
			move |_| {
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

					self2.btn_patch.hwnd().SetFocus();
				}
			}
		});

		self.btn_patch.on().bn_clicked({
			let self2 = self.clone();
			move || {
				let target = self2.txt_path.text().unwrap();

				if target.is_empty() {
					self2.wnd.hwnd().MessageBox(
						"No installation path given.",
						"No path",
						co::MB::ICONERROR).unwrap();
					self2.btn_choose.hwnd().SetFocus();

				} else {
					match patch::patch_installation(&target) {
						Err(e) => {
							self2.wnd.hwnd()
								.MessageBox(&e.to_string(), "Patching error", co::MB::ICONERROR)
								.unwrap();
						},
						Ok(_) => {},
					}
				}
			}
		});
	}
}
