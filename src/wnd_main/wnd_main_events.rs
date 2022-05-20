use winsafe::{prelude::*, self as w, co, msg};

use crate::patch;
use crate::util;
use super::WndMain;

impl WndMain {
	pub(super) fn _events(&self) {
		self.wnd.on().wm_command_accel_menu(co::DLGID::CANCEL.into(), {
			let self2 = self.clone();
			move || {
				self2.wnd.hwnd().SendMessage(msg::wm::Close {});
				Ok(())
			}
		});

		self.btn_choose.on().bn_clicked({
			let self2 = self.clone();
			move || {
				let fileo = w::CoCreateInstance::<w::IFileOpenDialog>(
					&co::CLSID::FileOpenDialog,
					None,
					co::CLSCTX::INPROC_SERVER,
				)?;

				fileo.SetOptions(
					fileo.GetOptions()?
						| co::FOS::FILEMUSTEXIST
						| co::FOS::PICKFOLDERS,
				)?;

				if fileo.Show(self2.wnd.hwnd())? {
					self2.txt_path.set_text(
						&fileo.GetResult()?
							.GetDisplayName(co::SIGDN::FILESYSPATH)?,
					);

					self2.btn_patch_font.hwnd().EnableWindow(true);
					self2.btn_patch_icon.hwnd().EnableWindow(true);
				}

				Ok(())
			}
		});

		self.btn_patch_font.on().bn_clicked({
			let self2 = self.clone();
			move || {
				if !self2._ok_if_running()? {
					return Ok(())
				}

				let clock = util::Timer::start();
				match patch::patch_font(&self2.txt_path.text()) {
					Err(e) => util::prompt::err(self2.wnd.hwnd(), "Patching failed", &e.to_string()),
					Ok(_) => util::prompt::info(self2.wnd.hwnd(), "Operation successful",
						&format!("Font successfully patched in {:.2}ms.", clock.now_ms())),
				}

				Ok(())
			}
		});

		self.btn_patch_icon.on().bn_clicked({
			let self2 = self.clone();
			move || {
				if !self2._ok_if_running()? {
					return Ok(())
				}

				let clock = util::Timer::start();
				match patch::patch_icon(&self2.txt_path.text()) {
					Err(e) => util::prompt::err(self2.wnd.hwnd(), "Patching failed", &e.to_string()),
					Ok(_) => util::prompt::info(self2.wnd.hwnd(), "Operation successful",
						&format!("Suggestion box icon successfully patched in {:.2}ms.", clock.now_ms())),
				}

				Ok(())
			}
		})
	}
}
