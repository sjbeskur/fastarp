#[test]
fn compute_chunk_size_small_network() {
    assert_eq!(super::arpscan::compute_chunk_size(128), 1);
}

#[test]
fn compute_chunk_size_class_c() {
    let chunk = super::arpscan::compute_chunk_size(256);
    assert!(chunk > 0);
}
