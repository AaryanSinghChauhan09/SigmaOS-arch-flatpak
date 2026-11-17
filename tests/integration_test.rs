//! Integration tests for SigmaOS-arch-flatpak.

use sigmaos_arch_flatpak_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
