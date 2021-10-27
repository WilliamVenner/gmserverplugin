#![allow(non_camel_case_types)]

use std::ffi::c_void;

use crate::{state, util::SingleThreadSingleton};

static INIT_HOOK: SingleThreadSingleton<Option<gmod::detour::GenericDetour<CLuaInterface_Init>>> = SingleThreadSingleton::new(None);
const NEWSTATE_FS: &'static str = "garrysmod/cache/gmserverplugin/newstate.mdmp";

#[no_mangle]
pub unsafe extern "C" fn newstate(callback: extern "C" fn(*mut c_void)) {
	state::add_hook(NEWSTATE_FS, callback).expect("Failed to add newstate hook");
}

#[cfg_attr(all(target_os = "windows", target_pointer_width = "64"), abi("fastcall"))]
#[cfg_attr(all(target_os = "windows", target_pointer_width = "32"), abi("thiscall"))]
#[type_alias(CLuaInterface_Init)]
extern "C" fn init_hook(this: usize, a2: usize, #[cfg(all(target_os = "windows", target_pointer_width = "32"))] a3: usize) -> usize {
	unsafe {
		let ret = INIT_HOOK.as_ref().unwrap_unchecked().call(this, a2, #[cfg(all(target_os = "windows", target_pointer_width = "32"))] a3);

		let ptr = *((this + std::mem::size_of::<usize>()) as *mut *mut c_void);
		*super::LUA_STATE.get_mut() = ptr;

		super::fire_callbacks(NEWSTATE_FS);

		ret
	}
}

pub(crate) unsafe fn hook(func: CLuaInterface_Init) {
	let hook = gmod::detour::GenericDetour::<CLuaInterface_Init>::new(func, init_hook).expect(rubat_plz!("Failed to hook CLuaInterface::Init"));
	hook.enable().expect(rubat_plz!("Failed to enable CLuaInterface::Init hook"));
	*INIT_HOOK.get_mut() = Some(hook);
}