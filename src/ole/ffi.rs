use crate::kernel::ffi_types::{BOOL, HANDLE, HRES, PCSTR, PCVOID, PSTR, PVOID};

extern_sys! { "ole32";
	CLSIDFromProgID(PCSTR, PVOID) -> HRES
	CLSIDFromProgIDEx(PCSTR, PVOID) -> HRES
	CLSIDFromString(PCSTR, PVOID) -> HRES
	CoCreateInstance(PCVOID, PVOID, u32, PCVOID, *mut PVOID) -> HRES
	CoCreateInstanceEx(PCVOID, PVOID, u32, PCVOID, u32, PVOID) -> HRES
	CoInitializeEx(PVOID, u32) -> HRES
	CoLockObjectExternal(PVOID, BOOL, BOOL) -> HRES
	CoTaskMemAlloc(usize) -> PVOID
	CoTaskMemFree(PVOID)
	CoTaskMemRealloc(PVOID, usize) -> PVOID
	CoUninitialize()
	RegisterDragDrop(HANDLE, PVOID) -> HRES
	RevokeDragDrop(HANDLE) -> HRES
	StringFromCLSID(PCVOID, *mut PSTR) -> HRES
}
