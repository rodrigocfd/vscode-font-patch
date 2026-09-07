#include <stdexcept>
#include "dlg_main.h"
#include "patch.h"

void DlgMain::btn_browse() {
	const std::wstring newFolder = wd::sys_dlg::folder_open(this, nullptr);
	if (!newFolder.empty())
		txtPath.set_text(newFolder);
}

void DlgMain::chk_change() {
	btnPatch.enable(chkPatchFont.is_checked() || chkPatchIcon.is_checked());
}

void DlgMain::btn_patch() {
	const std::wstring installPath = txtPath.text();
	if (!wd::file::is_dir(installPath)) [[unlikely]] {
		wd::sys_dlg::msg_err(this, L"Bad path", L"The chosen installation path is not valid:\n" + installPath);
		return;
	}

	if (patch::is_vscode_running()) {
		bool keepPatching = wd::sys_dlg::msg_ask(this,
			L"VS Code appears to be running",
			L"It's recommended to close VS Code before patching.\n"
			L"If you run the patch now, you must reload VS Code.\n\n"
			L"Patch anyway?",
			L"&Patch");
		if (!keepPatching)
			return;
	}

	try {
		patch::Res res = patch::do_patch(installPath, chkPatchFont.is_checked(), chkPatchIcon.is_checked());
		wd::sys_dlg::msg_ok(this, L"Patching finished", res.fontMsg + L"\n" + res.iconMsg);
	} catch (const std::runtime_error &e) {
		wd::sys_dlg::msg_err(this, L"Patching failed", L"Patching failed.", e);
	}
}
