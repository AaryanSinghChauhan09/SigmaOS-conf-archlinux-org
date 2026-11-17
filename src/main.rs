//! SigmaOS-conf-archlinux-org — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_conf_archlinux_org_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-conf-archlinux-org: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-conf-archlinux-org: ok");
}
