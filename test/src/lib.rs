use std::ffi::c_void;

extern "C" fn newstate(ptr: *mut c_void) {
	assert_ne!(ptr, std::ptr::null_mut());
	println!("newstate: {:?}", ptr);
}

extern "C" fn before_init(ptr: *mut c_void) {
	assert_ne!(ptr, std::ptr::null_mut());
	println!("before_init: {:?}", ptr);
}

extern "C" fn after_init(ptr: *mut c_void) {
	assert_ne!(ptr, std::ptr::null_mut());
	println!("after_init: {:?}", ptr);
}

#[ctor::ctor]
unsafe fn dllentry() {
	println!("CreateInterface");

	println!("Hooking");
	gmserverplugin::init();

	println!("Adding newstate callbacks");
	gmserverplugin::newstate(newstate);
	gmserverplugin::newstate(newstate);
	gmserverplugin::newstate(newstate);

	println!("Adding before_init callbacks");
	gmserverplugin::before_init(before_init);
	gmserverplugin::before_init(before_init);
	gmserverplugin::before_init(before_init);

	println!("Adding after_init callbacks");
	gmserverplugin::after_init(after_init);
	gmserverplugin::after_init(after_init);
	gmserverplugin::after_init(after_init);

	println!("Done");
}