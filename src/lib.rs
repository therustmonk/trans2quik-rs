extern crate winapi;

mod trans2quik {
	mod interop {
		pub use winapi::c_long;
		use winapi::minwindef::DWORD;
		use winapi::winnt::LPSTR;

		pub const TRANS2QUIK_SUCCESS					: c_long = 0;
		pub const TRANS2QUIK_FAILED						: c_long = 1;
		pub const TRANS2QUIK_QUIK_TERMINAL_NOT_FOUND	: c_long = 2;
		pub const TRANS2QUIK_DLL_VERSION_NOT_SUPPORTED	: c_long = 3;
		pub const TRANS2QUIK_ALREADY_CONNECTED_TO_QUIK	: c_long = 4;
		pub const TRANS2QUIK_WRONG_SYNTAX				: c_long = 5;
		pub const TRANS2QUIK_QUIK_NOT_CONNECTED			: c_long = 6;
		pub const TRANS2QUIK_DLL_NOT_CONNECTED			: c_long = 7;
		pub const TRANS2QUIK_QUIK_CONNECTED				: c_long = 8;
		pub const TRANS2QUIK_QUIK_DISCONNECTED			: c_long = 9;
		pub const TRANS2QUIK_DLL_CONNECTED				: c_long = 10;
		pub const TRANS2QUIK_DLL_DISCONNECTED			: c_long = 11;
		pub const TRANS2QUIK_MEMORY_ALLOCATION_ERROR	: c_long = 12;
		pub const TRANS2QUIK_WRONG_CONNECTION_HANDLE	: c_long = 13;
		pub const TRANS2QUIK_WRONG_INPUT_PARAMS			: c_long = 14;

		#[link(name="TRANS2QUIK")]
		extern "stdcall" {
			// TODO use macro to append error codes tail
			pub fn TRANS2QUIK_CONNECT(
				lpstConnectionParamsString: LPSTR, pnExtendedErrorCode: *mut c_long, lpstrErrorMessage: LPSTR, dwErrorMessageSize: DWORD
			) -> c_long;
		}
	}


	use std::os::raw::c_char;
	use std::path::PathBuf;
	use std::ffi::CString;
	use trans2quik::interop::*;	

	pub enum ConnectionError {
		TerminalNotFound,
		DllNotSupported,
		AlreadyConnected,
		Failed,
	}

	pub fn connect(path: &String) -> Result<(),ConnectionError> {
		//let path = PathBuf::from(path).as_os_str().to_bytes().unwrap();
		//let str = CString::new(b"world").unwrap();
		//let value: c_long = 0;
		let mut buf = [0 as c_char; 200];
		let mut buf2 = [0 as c_char; 200];
		//let mut path = CString::new("data data data data").unwrap().as_ptr();
		let mut cl: c_long = 0;
		let result = unsafe { TRANS2QUIK_CONNECT(buf.as_mut_ptr(), &mut cl, buf2.as_mut_ptr(), 200) };
		match result {
			TRANS2QUIK_SUCCESS =>
				Ok(()),
			TRANS2QUIK_QUIK_TERMINAL_NOT_FOUND =>
				Err(ConnectionError::TerminalNotFound),
			TRANS2QUIK_DLL_VERSION_NOT_SUPPORTED =>
				Err(ConnectionError::DllNotSupported),
			TRANS2QUIK_ALREADY_CONNECTED_TO_QUIK =>
				Err(ConnectionError::AlreadyConnected),
			TRANS2QUIK_FAILED =>
				Err(ConnectionError::Failed),
			_ =>
				Err(ConnectionError::Failed),
		}
	}
}

/*
#[test]
#[cfg(target_env = "msvc")]
fn main() {
	TRANS2QUIK_CONNECT();
}
*/
