use std::env;

pub(crate) fn print_env() {
  println!("======= \
    DYLD_LIBRARY_PATH: {:?},
	DYLD_FRAMEWORK_PATH: {:?},
	DYLD_FALLBACK_LIBRARY_PATH: {:?},
	DYLD_FALLBACK_FRAMEWORK_PATH: {:?},
	DYLD_INSERT_LIBRARIES: {:?},
	DYLD_IMAGE_SUFFIX: {:?},
	DYLD_VERSIONED_FRAMEWORK_PATH: {:?},
	DYLD_VERSIONED_LIBRARY_PATH: {:?},
	DYLD_ROOT_PAT: {:?},
    ",
    env::var("DYLD_LIBRARY_PATH").unwrap_or_default(),
	env::var("DYLD_FRAMEWORK_PATH").unwrap_or_default(),
	env::var("DYLD_FALLBACK_LIBRARY_PATH").unwrap_or_default(),
	env::var("DYLD_FALLBACK_FRAMEWORK_PATH").unwrap_or_default(),
	env::var("DYLD_INSERT_LIBRARIES").unwrap_or_default(),
	env::var("DYLD_IMAGE_SUFFIX").unwrap_or_default(),
	env::var("DYLD_VERSIONED_FRAMEWORK_PATH").unwrap_or_default(),
	env::var("DYLD_VERSIONED_LIBRARY_PATH").unwrap_or_default(),
	env::var("DYLD_ROOT_PATH").unwrap_or_default()
    );
}