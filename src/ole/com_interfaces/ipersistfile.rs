#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::vt::*;

/// [`IPersistFile`](crate::IPersistFile) virtual table.
#[repr(C)]
pub struct IPersistFileVT {
	pub IUnknownVT: IUnknownVT,
    pub GetCurFile: fn(COMPTR, PVOID) -> HRES,
    pub IsDirty: fn(COMPTR) -> HRES,
    pub Load: fn(COMPTR, PCSTR, u32) -> HRES,
    pub Save: fn(COMPTR, PCSTR, i32) -> HRES,
    pub SaveCompleted: fn(COMPTR, PCSTR) -> HRES,
}

com_interface! { IPersistFile: "0000010B-0000-0000-C000-000000000046";
    /// [`IPersistFile`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersistfile)
    /// COM interface over [`IPersistFileVT`](crate::vt::IPersistFileVT).
    ///
    /// Automatically calls
    /// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
    /// when the object goes out of scope.
}

impl ole_IPersistFile for IPersistFile {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IPersistFile`](crate::IPersistFile).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IPersistFile: ole_IUnknown {
	/// [`IPersist::GetClassID`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersist-getclassid)
	/// method.
	#[must_use]
	fn GetClassID(&self) -> HrResult<co::CLSID> {
		let mut clsid = co::CLSID::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IPersistVT>(self).GetClassID)(
					self.ptr(),
					&mut clsid as *mut _ as _,
				)
			},
		).map(|_| clsid)
	}

    /// [`IPersistFile::IsDirty`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersistfile-isdirty)
    /// method.
    #[must_use]
    fn IsDirty(&self) -> HrResult<bool> {
        match unsafe { co::HRESULT::from_raw((vt::<IPersistFileVT>(self).IsDirty)(self.ptr())) } {
            co::HRESULT::S_OK => Ok(true),
            co::HRESULT::S_FALSE => Ok(false),
            e => Err(e),
        }
    }

    /// [`IPersistFile::Load`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersistfile-load)
    /// method.
    #[must_use]
    fn Load(&self, file_name: &str, dw_mode: co::STGM) -> HrResult<()> {
        ok_to_hrresult(unsafe {
            (vt::<IPersistFileVT>(self).Load)(
                self.ptr(),
                WString::from_str(file_name).as_ptr(),
                dw_mode.raw(),
            )
        })
    }

    /// [`IPersistFile::Save`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersistfile-save)
    /// method.
    #[must_use]
    fn Save(&self, file_name: Option<&str>, remember: bool) -> HrResult<()> {
        ok_to_hrresult(unsafe {
            (vt::<IPersistFileVT>(self).Save)(
                self.ptr(),
                file_name.map_or(std::ptr::null(), |f| WString::from_str(f).as_ptr()),
                remember as i32,
            )
        })
    }

    /// [`IPersistFile::SaveCompleted`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersistfile-savecompleted)
    /// method.
    #[must_use]
    fn SaveCompleted(&self, file_name: &str) -> HrResult<()> {
        ok_to_hrresult(unsafe {
            (vt::<IPersistFileVT>(self).SaveCompleted)(self.ptr(), WString::from_str(file_name).as_ptr())
        })
    }
}
