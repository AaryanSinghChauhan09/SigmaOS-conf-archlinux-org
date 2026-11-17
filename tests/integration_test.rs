// Integration test for SigmaOS-conf-archlinux-org
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_conf_archlinux_org as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
