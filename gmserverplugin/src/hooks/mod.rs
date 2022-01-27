use std::{ffi::c_void, path::Path};

use crate::{sigs, state, util::SingleThreadSingleton};

pub mod newstate; // When the Lua state is created, and we have a pointer to it
pub mod before_init; // Before includes/init.lua has executed
pub mod after_init; // After includes/init.lua has been executed

pub type HookFn = extern "C" fn(lua: *mut c_void, lua_interface: *mut c_void);

pub(crate) static LUA_STATE: SingleThreadSingleton<(*mut c_void, *mut c_void)> = SingleThreadSingleton::new((std::ptr::null_mut(), std::ptr::null_mut()));
pub(super) fn hook() {
	unsafe { sigs::sigscan() };
}

fn fire_callbacks<P: AsRef<Path>>(path: P) {
	let lua_state = *LUA_STATE;
	assert_ne!(lua_state.0, std::ptr::null_mut());
	assert_ne!(lua_state.1, std::ptr::null_mut());
	for callback in state::read_hooks(&path).expect("Failed to read newstate hooks") {
		#[cfg(debug_assertions)]
		println!("gmserverplugin: firing callback {:?} for {}", callback as *const (), path.as_ref().display());

		callback(lua_state.0, lua_state.1);
	}
}