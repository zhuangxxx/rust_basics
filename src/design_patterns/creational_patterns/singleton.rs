pub static INSTANCE: std::sync::OnceLock<String> = std::sync::OnceLock::new();

#[test]
fn test_singleton() {
    println!(
        "第1次：{}",
        INSTANCE.get_or_init(|| "第1次实例化".to_owned())
    );
    println!(
        "第2次：{}",
        INSTANCE.get_or_init(|| "第2次实例化".to_owned())
    );
    println!(
        "第3次：{}",
        INSTANCE.get_or_init(|| "第3次实例化".to_owned())
    );
    println!(
        "第4次：{}",
        INSTANCE.get_or_init(|| "第4次实例化".to_owned())
    );
}
