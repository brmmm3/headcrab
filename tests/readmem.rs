//! This is a simple test to read memory from a child process.

mod test_utils;

#[cfg(target_os = "linux")]
use headcrab::{symbol::Dwarf, target::LinuxTarget, target::UnixTarget};

static BIN_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/testees/hello");

// FIXME: this should be an internal impl detail
#[cfg(target_os = "macos")]
static MAC_DSYM_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/testees/hello.dSYM/Contents/Resources/DWARF/hello"
);

// FIXME: Running this test just for linux because of privileges issue on macOS. Enable for everything after fixing.
#[cfg(target_os = "linux")]
#[test]
fn read_memory() -> Result<(), Box<dyn std::error::Error>> {
    test_utils::ensure_testees();

    #[cfg(target_os = "macos")]
    let debuginfo = Dwarf::new(MAC_DSYM_PATH)?;
    #[cfg(not(target_os = "macos"))]
    let debuginfo = Dwarf::new(BIN_PATH)?;

    let str_addr = debuginfo
        .get_var_address("STATICVAR")
        .expect("Expected static var has not been found in the target binary");

    let target = LinuxTarget::launch(BIN_PATH)?;

    // Read pointer
    let mut ptr_addr: usize = 0;
    unsafe {
        target.read().read(&mut ptr_addr, str_addr).apply()?;
    }

    // Read current value
    let mut rval = [0u8; 13];
    unsafe {
        target.read().read(&mut rval, ptr_addr).apply()?;
    }

    assert_eq!(&rval, b"Hello, world!");

    target.unpause()?;

    Ok(())
}
