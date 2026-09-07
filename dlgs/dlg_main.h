#pragma once
#include "../windlg/lib.hpp"
#include "../res/resource.h"

class DlgMain final : public wd::BaseDialog {
public:
	constexpr DlgMain() : BaseDialog{DLG_MAIN, ICO_VSGREEN} { }

private:
	bool on_init_dialog() override;
	bool on_command(WORD id, WORD code) override;

	void btn_browse();
	void chk_change();
	void btn_patch();

	wd::Edit     txtPath{this, TXT_PATH};
	wd::Button   btnBrowse{this, BTN_BROWSE};
	wd::CheckBox chkPatchFont{this, CHK_PATCH_FONT};
	wd::CheckBox chkPatchIcon{this, CHK_PATCH_ICON};
	wd::Button   btnPatch{this, BTN_PATCH};
};
