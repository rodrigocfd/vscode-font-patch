#pragma once
#include "../windlg/lib.hpp"

namespace patch {

	struct Res final {
		LPWSTR tdIcon;
		std::wstring fontMsg;
		std::wstring iconMsg;
	};

	[[nodiscard]] bool is_vscode_running();
	[[nodiscard]] Res do_patch(wd::StrView installPath, bool doFont, bool doIcon);

}
