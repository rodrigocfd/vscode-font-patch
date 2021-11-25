use winsafe::{prelude::*, self as w, co, gui, msg, shell};

use crate::patch;
use crate::util;
use super::WndMain;

impl WndMain {
	pub(super) fn events(&self) {
		self.wnd.on().wm_init_dialog({
			let self2 = self.clone();
			move |_| {
				self2.chk_patch_font.set_check_state(gui::CheckState::Checked);
				self2.chk_patch_theme.set_check_state(gui::CheckState::Checked);
				Ok(true)
			}
		});

		self.wnd.on().wm_command_accel_menu(co::DLGID::CANCEL.into(), {
			let self2 = self.clone();
			move || {
				self2.wnd.hwnd().SendMessage(msg::wm::Close {});
				Ok(())
			}
		});

		self.chk_patch_font.on().bn_clicked({
			let self2 = self.clone();
			move || {
				self2.maybe_enable_btn_run();
				Ok(())
			}
		});

		self.chk_patch_theme.on().bn_clicked({
			let self2 = self.clone();
			move || {
				self2.maybe_enable_btn_run();
				Ok(())
			}
		});

		self.btn_choose.on().bn_clicked({
			let self2 = self.clone();
			move || {
				let fileo = w::CoCreateInstance::<shell::IFileOpenDialog>(
					&shell::clsid::FileOpenDialog,
					None,
					co::CLSCTX::INPROC_SERVER,
				)?;

				fileo.SetOptions(
					fileo.GetOptions()?
						| shell::co::FOS::FILEMUSTEXIST
						| shell::co::FOS::PICKFOLDERS,
					)?;

				if fileo.Show(self2.wnd.hwnd())? {
					self2.txt_path.set_text(
						&fileo.GetResult()?
							.GetDisplayName(shell::co::SIGDN::FILESYSPATH)?,
					)?;

					self2.maybe_enable_btn_run();
					if self2.btn_patch.hwnd().IsWindowEnabled() {
						self2.btn_patch.focus()?;
					}
				}

				Ok(())
			}
		});

		self.btn_patch.on().bn_clicked({
			let self2 = self.clone();
			move || {
				if patch::is_vscode_running()? {
					if util::prompt::ok_cancel(self2.wnd.hwnd(),
						"VS Code appears to be running",
						"It's recommended to close VS Code before patching.\n\n\
							Proceed anyway?") != co::DLGID::OK
					{
						return Ok(());
					}
				}

				let clock = util::Timer::start();

				if let Err(e) = patch::patch_installation(
					&self2.txt_path.text()?,
					self2.chk_patch_font.is_checked(),
					self2.chk_patch_theme.is_checked(),
				) {
					util::prompt::err(self2.wnd.hwnd(), "Patching error", &e.to_string());
				} else {
					util::prompt::info(self2.wnd.hwnd(), "Operation successful",
						&format!("Installation successfully patched in {:.2}ms.", clock.now_ms()));
				}

				Ok(())
			}
		});
	}
}
