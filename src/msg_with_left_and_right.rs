#[macro_export]
macro_rules! msg_with_left_and_right {
    ($msg:expr, $key:expr, $left:expr, $right:expr $(,)?) => ({
        format!(
            concat!(
                "{}: `{}(left, right)`\n",
                "  left: `{:?}`,\n",
                " right: `{:?}`"
            ),
            $msg,
            $key,
            $left,
            $right
        )
    });
}

#[macro_export]
macro_rules! msg_with_left_and_right_and_err {
    ($msg:expr, $key:expr, $left:expr, $right:expr, $err:expr $(,)?) => ({
        format!(
            concat!(
                "{}: `{}(left, right)`\n",
                "  left: `{:?}`,\n",
                " right: `{:?}`"
                "   err: `{:?}`"
            ),
            $msg,
            $key,
            $left,
            $right
        )
    });
}
