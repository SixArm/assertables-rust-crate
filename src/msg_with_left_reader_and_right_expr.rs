#[macro_export]
macro_rules! msg_with_left_reader_and_right_expr {
    ($msg:expr, $key:expr, $left_reader_name:expr, $right_expr_name:expr, $left_reader_size:expr, $left_reader_data:expr, $right_expr:expr $(,)?) => ({
        format!(
            concat!(
                "{}: `{}(left_reader, right_expr)`\n",
                " left reader name: `{}`,\n",
                "  right expr name: `{}`,\n",
                " left reader size: `{:?}`,\n",
                " left reader data: `{:?}`,\n",
                "       right expr: `{:?}`"
            ),
            $msg,
            $key,
            $left_reader_name,
            $right_expr_name,
            $left_reader_size,
            $left_reader_data,
            $right_expr
        )
    });
}

#[macro_export]
macro_rules! msg_with_left_reader_and_right_expr_and_err {
    ($msg:expr, $key:expr, $left_reader_name:expr, $right_expr_name:expr, $err:expr $(,)?) => ({
        format!(
            concat!(
                "{}: `{}(left_reader, right_expr)`\n",
                "  left reader name: `{:?}`,\n",
                "   right expr name: `{:?}`,\n",
                "               err: `{:?}`"
            ),
            $msg,
            $key,
            $left_reader_name,
            $right_expr_name,
            $err
        )
    });
}
