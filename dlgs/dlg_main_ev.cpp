#include <crtdbg.h>
#include "dlg_main.h"

int APIENTRY wWinMain(_In_ HINSTANCE hInst, _In_opt_ HINSTANCE, _In_ LPWSTR, _In_ int cmdShow) {
	int ret = 0;
	{
		DlgMain dlg{};
		ret = wd::run_main_dialog(hInst, cmdShow, dlg, {
			{wd::Acc::Key::none, VK_F1, ACC_F1},
		});
	}
	if (_CrtDumpMemoryLeaks())
		MessageBoxW(nullptr, L"A memory leak was found.", L"Memory leak", MB_ICONERROR);
	return ret;
}

bool DlgMain::on_init_dialog() {
	chkPatchFont.set_check(true);
	chkPatchIcon.set_check(true);
	return true;
}

bool DlgMain::on_command(WORD id, WORD) {
	switch (id) {
		case BTN_BROWSE:     btn_browse(); return true;
		case CHK_PATCH_FONT:
		case CHK_PATCH_ICON: chk_change(); return true;
		case BTN_PATCH:      btn_patch(); return true;
		case ACC_F1:         wd::sys_dlg::msg_about(this, ICO_VSGREEN); return true;
		case IDCANCEL:       PostMessageW(hwnd(), WM_CLOSE, 0, 0); return true;
	}
	return false;
}
