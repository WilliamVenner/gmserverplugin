#![allow(non_camel_case_types)]

use crate::{state, util::SingleThreadSingleton, HookFn};

static BEFORE_INIT_HOOK: SingleThreadSingleton<Option<gmod::detour::GenericDetour<MakeLuaNULLEntity>>> = SingleThreadSingleton::new(None);
const BEFORE_INIT_FS: &'static str = "garrysmod/cache/gmserverplugin/before_init.mdmp";

#[no_mangle]
pub unsafe extern "C" fn before_init(callback: HookFn) {
	state::add_hook(BEFORE_INIT_FS, callback).expect("Failed to add before_init hook");
}

#[cfg_attr(all(target_os = "windows", target_pointer_width = "32"), abi("thiscall"))]
#[cfg_attr(all(target_os = "windows", target_pointer_width = "64"), abi("fastcall"))]
#[type_alias(MakeLuaNULLEntity)]
extern "C" fn before_init_hook() {
	unsafe {
		BEFORE_INIT_HOOK.as_ref().unwrap_unchecked().call();

		super::fire_callbacks(BEFORE_INIT_FS);
	}
}

pub(crate) unsafe fn hook(func: MakeLuaNULLEntity) {
	let hook = gmod::detour::GenericDetour::<MakeLuaNULLEntity>::new(func, before_init_hook).expect(rubat_plz!("Failed to hook MakeLuaNULLEntity"));
	hook.enable().expect(rubat_plz!("Failed to enable MakeLuaNULLEntity hook"));
	*BEFORE_INIT_HOOK.get_mut() = Some(hook);
}