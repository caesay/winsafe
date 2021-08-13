//! Toolbar control
//! [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-toolbar-control-reference-messages),
//! whose constants have [`TBM`](crate::co::TBM) prefix.

use crate::aliases::WinResult;
use crate::co;
use crate::msg::{MsgSend, WndMsg};
use crate::structs::{TBADDBITMAP, TBBUTTON};

/// [`TB_ADDBITMAP`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-addbitmap)
/// message parameters.
///
/// Return type: `WinResult<u32>`.
pub struct AddBitmap<'a> {
	pub num_images: u32,
	pub info: &'a TBADDBITMAP,
}

impl<'a> MsgSend for AddBitmap<'a> {
	type RetType = WinResult<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			-1 => Err(co::ERROR::BAD_ARGUMENTS),
			idx => Ok(idx as _),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ADDBITMAP.into(),
			wparam: self.num_images as _,
			lparam: self.info as *const _ as _,
		}
	}
}

/// [`TB_ADDBUTTONS`](https://docs.microsoft.com/en-us/windows/win32/controls/tb-addbuttons)
/// message parameters.
///
/// Return type: `WinResult<()>`.
pub struct AddButtons<'a, 'b> {
	pub buttons: &'a mut [TBBUTTON<'b>],
}

impl<'a, 'b> MsgSend for AddButtons<'a, 'b> {
	type RetType = WinResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		match v {
			0 => Err(co::ERROR::BAD_ARGUMENTS),
			_ => Ok(()),
		}
	}

	fn as_generic_wm(&self) -> WndMsg {
		WndMsg {
			msg_id: co::TBM::ADDBUTTONS.into(),
			wparam: self.buttons.len() as _,
			lparam: self.buttons.as_ptr() as _,
		}
	}
}