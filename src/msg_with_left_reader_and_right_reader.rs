#[macro_export]
macro_rules! msg_with_left_reader_and_right_reader {
    ($msg:expr, $key:expr, $left_reader_name:expr, $right_reader_name:expr, $left_reader_size:expr, $right_reader_size:expr, $left_reader_data:expr, $right_reader_data:expr $(,)?) => ({
        format!(
            concat!(
                "{}: `{}(left_reader, right_reader)`\n",
                "  left reader name: `{}`,\n",
                " right reader name: `{}`,\n",
                "  left reader size: `{:?}`,\n",
                " right reader size: `{:?}`,\n",
                "  left reader data: `{:?}`,\n",
                " right reader data: `{:?}`"
            ),
            $msg,
            $key,
            $left_reader_name,
            $right_reader_name,
            $left_reader_size,
            $right_reader_size,
            $left_reader_data,
            $right_reader_data
        )
    });
}

#[macro_export]
macro_rules! msg_with_left_reader_and_right_reader_and_err {
    ($msg:expr, $key:expr, $left_reader_name:expr, $right_reader_name:expr, $err:expr $(,)?) => ({
        format!(
            concat!(
                "{}: `{}(left_reader, right_reader)`\n",
                "  left reader name: `{:?}`,\n",
                " right reader name: `{:?}`,\n",
                "               err: `{:?}`"
            ),
            $msg,
            $key,
            $left_reader_name,
            $right_reader_name,
            $err
        )
    });
}
