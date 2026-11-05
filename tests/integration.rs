use sigmaos_linux::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_linux::Linux::new("sigmaos-linux").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-linux"));
}
