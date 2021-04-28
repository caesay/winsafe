//! [DirectShow](https://docs.microsoft.com/en-us/windows/win32/directshow/directshow)
//! constants.

#![allow(non_upper_case_globals)]

const_type! { FILTER_STATE, u32,
	/// [`FILTER_STATE`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/ne-strmif-filter_state)
	/// enumeration (`u32`).
	=>
	Stopped, 0
	Paused, 1
	Running, 2
}

const_type! { MFVideoARMode, u32,
	/// [`MFVideoAspectRatioMode`](https://docs.microsoft.com/en-us/windows/win32/api/evr/ne-evr-mfvideoaspectratiomode)
	/// enumeration (`u32`).
	=>
	None, 0
	PreservePicture, 0x1
	PreservePixel, 0x2
	NonLinearStretch, 0x4
}

const_type! { SEEKING_FLAGS, u32,
	/// [`IMediaSeeking::SetPositions`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/nf-strmif-imediaseeking-setpositions)
	/// flags (`u32`). Originally AM_SEEKING_SeekingFlags enum.
	=>
	NoPositioning, 0x0
	AbsolutePositioning, 0x1
	RelativePositioning, 0x2
	IncrementalPositioning, 0x3
	SeekToKeyFrame, 0x4
	ReturnTime, 0x8
	Segment, 0x10
	NoFlush, 0x20
}