use std::{
	io::{Read, Write},
	path::{Path, PathBuf},
};

use crate::hooks::HookFn;

fn clear_data_dir<P: AsRef<Path>>(dir: P) -> Result<(), std::io::Error> {
	let dir = dir.as_ref();
	if dir.is_dir() {
		for entry in dir.read_dir()? {
			let entry = entry?;
			if entry.file_type()?.is_dir() {
				std::fs::remove_dir_all(entry.path())?;
			} else {
				std::fs::remove_file(entry.path())?;
			}
		}
	} else {
		std::fs::create_dir_all(dir)?;
	}
	Ok(())
}

pub(crate) fn is_hooked() -> bool {
	let pid = std::process::id();
	let path = PathBuf::from(format!("garrysmod/cache/gmserverplugin/{}.mdmp", pid));
	let dir = unsafe { path.parent().unwrap_unchecked() };
	if path.exists() {
		true
	} else {
		if let Err(error) = clear_data_dir(&dir) {
			panic!(
				"Failed to clear/create directory at {}\n{}",
				dir.display(),
				error
			);
		}
		if let Err(error) = std::fs::write(&path, "") {
			panic!("Failed to write to {}\n{}", path.display(), error);
		}
		false
	}
}

pub(crate) fn read_hooks<P: AsRef<Path>>(path: P) -> Result<Vec<HookFn>, std::io::Error> {
	let mut hooks = vec![];
	if path.as_ref().exists() {
		let mut f = std::fs::File::open(path)?;
		loop {
			let mut ptr = [0u8; std::mem::size_of::<HookFn>()];
			match f.read_exact(&mut ptr) {
				Ok(_) => {
					hooks.push(unsafe { std::mem::transmute(usize::from_le_bytes(ptr) as *const ()) })
				}
				Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => break,
				Err(error) => return Err(error),
			}
		}
	}
	Ok(hooks)
}

pub(crate) fn add_hook<P: AsRef<Path>>(path: P, hook: HookFn) -> Result<(), std::io::Error> {
	std::fs::create_dir_all("garrysmod/cache")?;

	let mut f = std::fs::OpenOptions::new()
		.append(true)
		.truncate(false)
		.create(true)
		.open(path)?;

	f.write_all(&(hook as *const () as usize).to_le_bytes())
}
