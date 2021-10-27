use std::cell::UnsafeCell;

pub(crate) struct SingleThreadSingleton<T>(UnsafeCell<T>);
impl<T> SingleThreadSingleton<T> {
	pub(crate) const fn new(val: T) -> SingleThreadSingleton<T> {
		SingleThreadSingleton(UnsafeCell::new(val))
	}

	pub(crate) fn get_mut(&self) -> &mut T {
		unsafe { &mut *self.0.get() }
	}
}
unsafe impl<T> Sync for SingleThreadSingleton<T> {}
impl<T> std::ops::Deref for SingleThreadSingleton<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe { &*self.0.get() }
	}
}