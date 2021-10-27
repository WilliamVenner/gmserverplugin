fn main() {
	println!("cargo:rerun-if-changed=src/lib.rs");
	println!("cargo:rerun-if-changed=src/plugin.h");
	println!("cargo:rerun-if-changed=src/plugin.cpp");
	println!("cargo:rerun-if-changed=src/iserverplugin.h");
	println!("cargo:rerun-if-changed=build.rs");

	cxx_build::bridge("src/lib.rs")
		.file("src/plugin.cpp")
		.include("src")
		.flag_if_supported("-std=c++14")
		.compile("gmserverplugintest");
}