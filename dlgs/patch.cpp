#include <optional>
#include "patch.h"
#include <CommCtrl.h>
#include <TlHelp32.h>

bool patch::is_vscode_running() {
	struct HandleSnap32 final {
		HANDLE h32;
		~HandleSnap32() noexcept { CloseHandle(h32); }
	};

	HandleSnap32 h{CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)};
	if (h.h32 == INVALID_HANDLE_VALUE) [[unlikely]] {
		throw wd::WinErr{GetLastError(), L"CreateToolhelp32Snapshot failed."};
	}

	PROCESSENTRY32W pe{.dwSize = sizeof(PROCESSENTRY32W)};
	if (!Process32FirstW(h.h32, &pe)) {
		DWORD err = GetLastError();
		if (err == ERROR_NO_MORE_FILES) [[likely]] {
			return false;
		} else [[unlikely]] {
			throw wd::WinErr{err, L"Process32First failed."};
		}
	}

	for (;;) {
		if (wd::StrView{pe.szExeFile}.eq_i(L"Code.exe")) // process is running
			return true;

		if (!Process32NextW(h.h32, &pe)) {
			const DWORD err = GetLastError();
			if (err == ERROR_NO_MORE_FILES) [[likely]] {
				return false;
			} else [[unlikely]] {
				throw wd::WinErr{err, L"Process32Next failed."};
			}
		}
	}
}

static std::wstring patch_font(wd::StrView cssContents) {
	wd::StrView END_OF_COMMS = L"-*/";
	wd::StrView MAGIC_PATCH = L"\n*{text-shadow:transparent 0px 0px 0px, rgba(0, 0, 0, 0.5) 0px 0px 0px !important;}";

	size_t idxStartCode = cssContents.find(END_OF_COMMS);
	if (idxStartCode == wd::StrView::npos) [[unlikely]] {
		throw wd::WinErr{E_UNEXPECTED, L"CSS end of comments not found."};
	}
	idxStartCode += END_OF_COMMS.length();

	// Is our magic path the first thing past the comments block?
	if (cssContents.find(MAGIC_PATCH, idxStartCode) == idxStartCode) {
		throw wd::WinErr{E_INVALIDARG, L"Font already patched."};
	}

	std::wstring newContents{};
	newContents.reserve(cssContents.length() + MAGIC_PATCH.length());
	newContents.append(cssContents.begin(), cssContents.begin() + idxStartCode); // comments block
	newContents.append(MAGIC_PATCH);
	newContents.append(cssContents.begin() + idxStartCode, cssContents.end()); // rest of file

	return newContents;
}

static std::wstring patch_icon(wd::StrView cssContents) {
	wd::StrView NATURAL = L".monaco-editor .suggest-widget .monaco-list .monaco-list-row.focused .codicon{color:var(--vscode-editorSuggestWidget-selectedIconForeground)}";
	wd::StrView PATCHED = L" /*.monaco-editor .suggest-widget .monaco-list .monaco-list-row.focused .codicon{color:var(--vscode-editorSuggestWidget-selectedIconForeground)}*/ ";

	if (cssContents.contains(PATCHED)) {
		throw wd::WinErr{E_INVALIDARG, L"Suggestion box icon already patched."};
	}

	const size_t idx = cssContents.find(NATURAL);
	if (idx == wd::StrView::npos) [[unlikely]] {
		throw wd::WinErr{E_INVALIDARG, L"Suggestion box icon CSS entry not found."};
	}

	std::wstring newContents{};
	newContents.reserve(cssContents.length() + PATCHED.length() - NATURAL.length());
	newContents.append(cssContents.begin(), cssContents.begin() + idx); // all code up to part
	newContents.append(PATCHED);
	newContents.append(cssContents.begin() + idx + NATURAL.length(), cssContents.end()); // rest of file

	return newContents;
}

patch::Res patch::do_patch(wd::StrView installPath, bool doFont, bool doIcon) {
	const std::wstring cssPath = installPath + L"\\resources\\app\\out\\vs\\workbench\\workbench.desktop.main.css";
	bool fontOk = true, iconOk = true;
	Res res{
		.fontMsg = L"Font unpatched.",
		.iconMsg = L"Suggestion icon unpatched.",
	};

	std::wstring cssContents = wd::str::parse(wd::file::read(cssPath));
	if (doFont) {
		try {
			cssContents = patch_font(cssContents);
			res.fontMsg = L"Font patched successfully.";
		} catch (const wd::WinErr &e) {
			fontOk = false;
			res.fontMsg = e.msg();
		}
	}
	if (doIcon) {
		try {
			cssContents = patch_icon(cssContents);
			res.iconMsg = L"Suggestion icon patched successfully.";
		} catch (const wd::WinErr &e) {
			iconOk = false;
			res.iconMsg = e.msg();
		}
	}

	if (!fontOk && !iconOk) {
		res.tdIcon = TD_ERROR_ICON;
	} else if (!fontOk || !iconOk) {
		res.tdIcon = TD_WARNING_ICON;
	} else {
		res.tdIcon = TD_INFORMATION_ICON;
	}

	wd::file::write(cssPath, wd::str::to_utf8_blob(cssContents));
	return res;
}
