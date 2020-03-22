use backtrace::Backtrace;
use std::env;
use std::panic::{self, PanicInfo};

pub fn hook_panic() {
    panic::set_hook(Box::new(|info: &PanicInfo| {
        let _ = print_report(info);
    }));
}

pub fn print_report(panic_info: &PanicInfo) -> std::io::Result<()> {
    let backtrace = Backtrace::new();
    let cwd = env::current_dir()?;
    let argv: Vec<String> = env::args().collect();

    println!("process_id: {}", std::process::id());
    println!("cwd: {}", cwd.display());
    println!("command_line: {:?}", &argv);
    println!("panic_info: {:?}", panic_info);
    println!("environment_variables:");

    for (key, value) in env::vars_os() {
        println!("{:?}: {:?}", key, value);
    }

    println!("backtrace:");
    println!("{:?}", backtrace);

    Ok(())
}

#[test]
#[should_panic]
fn check_hook_panic() {
    hook_panic();
    panic!("a test panic");
}
