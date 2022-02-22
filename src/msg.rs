#[macro_export]
macro_rules! msg_key_left_right {
    ($msg:expr, $key:expr, $a:expr, $b:expr $(,)?) => ({
        format!(
            concat!(
                "{}: `{}(left, right)`\n",
                "  left: `{:?}`,\n",
                " right: `{:?}`"
            ),
            $msg,
            $key,
            $a,
            $b
        )
    });
}
