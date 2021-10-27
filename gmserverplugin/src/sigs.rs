macro_rules! sigs {
	($(sigs! { for: $ident:ident, debug: $debug:literal, module: $module:literal, $($name:tt: $sig:literal,)+ })+) => {
		pub(crate) unsafe fn sigscan() {
			let (lua_shared, lua_shared_path) = open_library_srv!("lua_shared").expect("Failed to open lua_shared!");
			let (server_dll, server_dll_path) = open_library_srv!("server").expect("Failed to open server.dll!");

			$(
				let (_lib, _path) = if $module == "lua_shared" {
					(&lua_shared, lua_shared_path)
				} else if $module == "server_dll" {
					(&server_dll, server_dll_path)
				} else {
					unreachable!();
				};
				crate::hooks::$ident::hook(find_gmod_signature!((_lib, _path) -> { $($name: [@SIG = $sig],)+ }).expect(rubat_plz!("Failed to find ", $debug)));
			)+
		}
	};
}

sigs! {
	sigs! {
		for: newstate,
		debug: "CLuaInterface::Init",
		module: "lua_shared",

		win64_x86_64: "40 53 48 83 EC 20 48 8B 01 48 8B D9 48 89 91 ? ? ? ? FF 90 ? ? ? ? 33 D2",
		win32_x86_64: "55 8B EC 8B 45 08 83 EC 08 56 8B F1 57 89 86 ? ? ? ? 8B 06 FF 90 ? ? ? ? 89 86 ? ? ? ? 8D BE ? ? ? ? 0F 57 C0 33 C0",

		linux64_x86_64: "55 48 89 E5 53 48 89 FB 48 83 EC 08 48 89 B7 ? ? ? ? 48 8B 07 FF 90 ? ? ? ? 48 89 83 ? ? ? ? 31 C0 0F 1F 00 48 C7 44 03 ? ? ? ? ? 48 83 C0 08 48 3D ? ? ? ? 75 EB C7 83 ? ? ? ? ? ? ? ? 66 31 C0 66 0F 1F 44 00 ?",
		linux32_x86_64: "55 89 E5 53 83 EC 14 8B 5D 08 8B 45 0C 89 83 ? ? ? ? 8B 03 89 1C 24 FF 90 ? ? ? ? 89 83 ? ? ? ? 31 C0 8D 76 00 8D BC 27 ? ? ? ?",

		win32: "55 8B EC 8B 45 08 53 8B D9 56 57 89 83 ? ? ? ? 8B 03 FF 90 ? ? ? ? 89 83 ? ? ? ? 8D BB ? ? ? ? 33 C0 B9 ? ? ? ?",
		linux32: "55 89 E5 57 56 53 83 EC 2C 8B 5D 08 8B 45 0C 89 83 ? ? ? ? 8B 03 89 1C 24 FF 90 ? ? ? ? 89 83 ? ? ? ? 8D 43 38 83 E0 0F C1 E8 02 F7 D8 83 E0 03",
	}

	sigs! {
		for: before_init,
		debug: "MakeLuaNULLEntity",
		module: "server_dll",

		win64_x86_64: "48 83 EC 48 48 8D 4C 24 ? E8 ? ? ? ? 48 8B 0D ? ? ? ? 33 D2 48 8B 01 44 8D 42 09 FF 90 ? ? ? ? 48 8B 05 ? ? ? ? 48 8D 0D ? ? ? ? FF 90 ? ? ? ? 48 8B 0D ? ? ? ?",
		win32_x86_64: "00",

		linux64_x86_64: "00",
		linux32_x86_64: "00",

		win32: "00",
		linux32: "55 89 E5 56 53 8D 5D E4 83 EC 30 89 1C 24 E8 ? ? ? ? A1 ? ? ? ? 8B 10 C7 44 24 ? ? ? ? ? C7 44 24 ? ? ? ? ? 89 04 24 FF 92 ? ? ? ? A1 ? ? ? ?",
	}

	sigs! {
		for: after_init,
		debug: "CLuaGamemode::LoadGamemode",
		module: "server_dll",

		win64_x86_64: "40 55 56 57 41 54 41 55 41 56 41 57 48 8D 6C 24 ? 48 81 EC ? ? ? ? 48 8B 05 ? ? ? ? 48 33 C4 48 89 45 20 4C 8B F1 48 89 4C 24 ? 48 8B 0D",
		win32_x86_64: "00",

		linux64_x86_64: "00",
		linux32_x86_64: "00",

		win32: "00",
		linux32: "55 89 E5 57 56 53 81 EC ? ? ? ? 8B 3D ? ? ? ? 8B 45 10 85 FF 0F 95 85 ? ? ? ? 89 85 ? ? ? ? 0F 85 ? ? ? ?",
	}
}