//! Integration tests for SigmaOS-conf-archlinux-org.

use sigmaos_conf_archlinux_org_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
