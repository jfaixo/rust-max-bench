use rust_max_bench::custom_max;

#[test]
fn simple_check() {
    let my_array = (0..1000).collect::<Vec<_>>();
    let max = custom_max(&my_array);

    assert_eq!(max, Some(999));
}