#![allow(non_camel_case_types)]

use std::ffi::c_void;

use crate::{state, util::SingleThreadSingleton};

static AFTER_INIT_HOOK: SingleThreadSingleton<Option<gmod::detour::GenericDetour<CLuaGamemode_LoadGamemode>>> = SingleThreadSingleton::new(None);
const AFTER_INIT_FS: &'static str = "garrysmod/cache/gmserverplugin/after_init.mdmp";

#[no_mangle]
pub unsafe extern "C" fn after_init(callback: extern "C" fn(ptr: *mut c_void)) {
	state::add_hook(AFTER_INIT_FS, callback).expect("Failed to add after_init hook");
}

#[cfg_attr(all(target_os = "windows", target_pointer_width = "32"), abi("thiscall"))]
#[cfg_attr(all(target_os = "windows", target_pointer_width = "64"), abi("fastcall"))]
#[type_alias(CLuaGamemode_LoadGamemode)]
extern "C" fn after_init_hook(this: usize, a2: *const c_void, a3: bool) {
	unsafe {
		super::fire_callbacks(AFTER_INIT_FS);

		AFTER_INIT_HOOK.as_ref().unwrap_unchecked().disable().expect("Failed to disable CLuaGamemode::LoadGamemode hook");
		AFTER_INIT_HOOK.as_ref().unwrap_unchecked().call(this, a2, a3);
	}
}

pub(crate) unsafe fn hook(func: CLuaGamemode_LoadGamemode) {
	let hook = gmod::detour::GenericDetour::<CLuaGamemode_LoadGamemode>::new(func, after_init_hook).expect(rubat_plz!("Failed to hook CLuaGamemode::LoadGamemode"));
	hook.enable().expect(rubat_plz!("Failed to enable CLuaGamemode::LoadGamemode hook"));
	*AFTER_INIT_HOOK.get_mut() = Some(hook);
}