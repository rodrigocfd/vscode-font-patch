use winsafe as w;

pub struct Timer(i64);

impl Timer {
	pub fn start() -> Self {
		Self(w::QueryPerformanceCounter().unwrap())
	}

	pub fn now_ms(&self) -> f64 {
		let freq = w::QueryPerformanceFrequency().unwrap();
		let t1 = w::QueryPerformanceCounter().unwrap();
		((t1 - self.0) as f64 / freq as f64) * 1000.0
	}
}

pub mod prompt {
	use winsafe::{prelude::*, self as w, co};

	pub enum DefBtn { Ok, Cancel }

	pub fn err(hwnd: w::HWND, title: &str, body: &str) {
		base(hwnd, title, body, co::TDCBF::OK,
			u16::from(co::DLGID::OK) as _, co::TD_ICON::ERROR);
	}

	pub fn info(hwnd: w::HWND, title: &str, body: &str) {
		base(hwnd, title, body, co::TDCBF::OK,
			u16::from(co::DLGID::OK) as _, co::TD_ICON::INFORMATION);
	}

	pub fn ok_cancel(hwnd: w::HWND, def_btn: DefBtn, title: &str, body: &str) -> bool {
		base(hwnd, title, body, co::TDCBF::OK | co::TDCBF::CANCEL,
			match def_btn {
				DefBtn::Ok => u16::from(co::DLGID::OK) as _,
				DefBtn::Cancel => u16::from(co::DLGID::CANCEL) as _,
			},
			co::TD_ICON::WARNING) == co::DLGID::OK
	}

	fn base(hwnd: w::HWND, title: &str, body: &str,
		btns: co::TDCBF, def_btn: i32, ico: co::TD_ICON) -> co::DLGID
	{
		let mut title = w::WString::from_str(title);
		let mut body = w::WString::from_str(body);

		let mut tdc = w::TASKDIALOGCONFIG::default();
		tdc.hwndParent = hwnd;
		tdc.dwFlags = co::TDF::ALLOW_DIALOG_CANCELLATION;
		tdc.dwCommonButtons = btns;
		tdc.set_pszMainIcon(w::IconIdTdicon::Tdicon(ico));
		tdc.set_pszWindowTitle(Some(&mut title));
		tdc.set_pszContent(Some(&mut body));
		tdc.nDefaultButton = def_btn;

		let (res, _) = w::TaskDialogIndirect(&mut tdc, None).unwrap();
		res
	}
}
