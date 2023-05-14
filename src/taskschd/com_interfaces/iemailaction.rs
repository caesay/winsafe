#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{COMPTR, HRES, PCSTR, PCVOID, PSTR, PVOID};
use crate::ole::decl::HrResult;
use crate::prelude::{oleaut_IDispatch, taskschd_IAction};
use crate::vt::IActionVT;

/// [`IEmailAction`](crate::IEmailAction) virtual table.
#[repr(C)]
pub struct IEmailActionVT {
	pub IAction: IActionVT,
	pub get_Server: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Server: fn(COMPTR, PCSTR) -> HRES,
	pub get_Subject: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Subject: fn(COMPTR, PCSTR) -> HRES,
	pub get_To: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_To: fn(COMPTR, PCSTR) -> HRES,
	pub get_Cc: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Cc: fn(COMPTR, PCSTR) -> HRES,
	pub get_Bcc: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Bcc: fn(COMPTR, PCSTR) -> HRES,
	pub get_ReplyTo: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_ReplyTo: fn(COMPTR, PCSTR) -> HRES,
	pub get_From: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_From: fn(COMPTR, PCSTR) -> HRES,
	pub get_HeaderFields: fn(COMPTR, *mut COMPTR) -> HRES,
	pub put_HeaderFields: fn(COMPTR, COMPTR) -> HRES,
	pub get_Body: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Body: fn(COMPTR, PCSTR) -> HRES,
	pub get_Attachments: fn(COMPTR, PVOID) -> HRES,
	pub put_Attachments: fn(COMPTR, PCVOID) -> HRES,
}

com_interface! { IEmailAction: "10f62c64-7e16-4314-a0c2-0c3683f99d40";
	/// [`IEmailAction`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nn-taskschd-iemailaction)
	/// COM interface over [`IEmailActionVT`](crate::vt::IEmailActionVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{IAction, IEmailAction};
	///
	/// let action: IAction; // initialized somewhere
	/// # let action = unsafe { IAction::null() };
	///
	/// let email_action = action
	///     .QueryInterface::<IEmailAction>()?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
}

impl oleaut_IDispatch for IEmailAction {}
impl taskschd_IAction for IEmailAction {}
impl taskschd_IEmailAction for IEmailAction {}

/// This trait is enabled with the `taskschd` feature, and provides methods for
/// [`IEmailAction`](crate::IEmailAction).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait taskschd_IEmailAction: taskschd_IAction {
	fn_bstr_get! { get_Bcc: IEmailActionVT;
		/// [`IEmailAction::get_Bcc`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-get_bcc)
		/// method.
	}

	fn_bstr_get! { get_Body: IEmailActionVT;
		/// [`IEmailAction::get_Body`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-get_body)
		/// method.
	}

	fn_bstr_get! { get_Cc: IEmailActionVT;
		/// [`IEmailAction::get_Cc`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-get_cc)
		/// method.
	}

	fn_bstr_get! { get_From: IEmailActionVT;
		/// [`IEmailAction::get_From`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-get_from)
		/// method.
	}

	fn_bstr_get! { get_ReplyTo: IEmailActionVT;
		/// [`IEmailAction::get_ReplyTo`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-get_replyto)
		/// method.
	}

	fn_bstr_get! { get_Server: IEmailActionVT;
		/// [`IEmailAction::get_Server`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-get_server)
		/// method.
	}

	fn_bstr_get! { get_Subject: IEmailActionVT;
		/// [`IEmailAction::get_Subject`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-get_subject)
		/// method.
	}

	fn_bstr_get! { get_To: IEmailActionVT;
		/// [`IEmailAction::get_To`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-get_to)
		/// method.
	}

	fn_bstr_set! { put_Bcc: IEmailActionVT, bcc;
		/// [`IEmailAction::put_Bcc`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-put_bcc)
		/// method.
	}

	fn_bstr_set! { put_Body: IEmailActionVT, body;
		/// [`IEmailAction::put_Body`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-put_body)
		/// method.
	}

	fn_bstr_set! { put_Cc: IEmailActionVT, cc;
		/// [`IEmailAction::put_Cc`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-put_cc)
		/// method.
	}

	fn_bstr_set! { put_From: IEmailActionVT, from;
		/// [`IEmailAction::put_From`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-put_from)
		/// method.
	}

	fn_bstr_set! { put_ReplyTo: IEmailActionVT, reply_to;
		/// [`IEmailAction::put_ReplyTo`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-put_replyto)
		/// method.
	}

	fn_bstr_set! { put_Server: IEmailActionVT, server;
		/// [`IEmailAction::put_Server`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-put_server)
		/// method.
	}

	fn_bstr_set! { put_Subject: IEmailActionVT, subject;
		/// [`IEmailAction::put_Subject`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-put_subject)
		/// method.
	}

	fn_bstr_set! { put_To: IEmailActionVT, to;
		/// [`IEmailAction::put_To`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/nf-taskschd-iemailaction-put_to)
		/// method.
	}
}