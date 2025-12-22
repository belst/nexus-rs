pub use crate::rtapi::data::RealTimeData;

/// Returns the shared [`RealTimeData`] pointer.
#[inline]
pub fn get_rtapi_ptr() -> *const RealTimeData {
    RealTimeData::get_ptr()
}

/// Reads the shared [`RealTimeData`].
#[inline]
pub fn read_rtapi() -> Option<RealTimeData> {
    RealTimeData::read()
}
