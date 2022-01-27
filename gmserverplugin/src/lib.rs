#![feature(abi_thiscall)]

#[macro_use]
extern crate gmod;

macro_rules! rubat_plz {
	($msg:literal $(, $concat:tt)*) => {
		concat!($msg, $($concat,)* " - complain here: https://github.com/Facepunch/garrysmod-requests/issues/1917")
	};
}

mod util;
mod hooks;
mod state;
mod sigs;

pub use hooks::HookFn;

#[no_mangle]
pub unsafe extern "C" fn init() {
	#[cfg(debug_assertions)]
	println!("gmserverplugin: init");

	if !state::is_hooked() {
		hooks::hook();
	}
}

pub use hooks::{
	after_init::after_init,
	before_init::before_init,
	newstate::newstate
};
