#[macro_export]
macro_rules! with_range {
    ($reader:expr, $body:expr) => {{
        let start = $reader.cursor as u64;
        let result = ($body);
        let end = $reader.cursor as u64;
        (start..end, result)
    }};
}

#[macro_export]
macro_rules! read_field_format {
    ($list:expr, $reader:expr, $body:expr, $msg_template:expr) => {{
        let start = $reader.cursor;
        let content = ($body);
        let end = $reader.cursor;
        let ele = crate::common::concept::Field::label(format!($msg_template, content), start, end);
        $list.push(ele);
        content
    }};
}
#[macro_export]
macro_rules! read_field_format_fn {
    ($list:expr, $reader:expr, $body:expr, $fn_ref:expr) => {{
        let start = $reader.cursor;
        let content = ($body);
        let end = $reader.cursor;
        let msg = $fn_ref(content);
        // let ele = crate::common::FieldElement::create(msg, Some(start..end));
        let ele = crate::common::concept::Field::label(msg, start, end);
        $list.push(ele);
        content
    }};
}

#[macro_export]
macro_rules! field_back_format {
    ($list:expr, $reader:expr, $inx:expr, $msg:expr) => {{
        let start = $reader.cursor;
        let ele = crate::common::concept::Field::label($msg, start - $inx, start);
        // let ele = crate::common::FieldElement::create(crate::cache::intern($msg), Some(start-$inx..start));
        $list.push(ele);
    }};
}

#[macro_export]
macro_rules! field_rest_format {
    ($list:expr, $reader:expr, $msg:expr) => {{
        if $reader.left() > 0 {
            let start = $reader.cursor;
            let end = $reader.cursor + $reader.left();
            let ele = crate::common::concept::Field::label($msg, start, end);
            $list.push(ele);
        }
    }};
}

#[macro_export]
macro_rules! field_forward_format {
    ($list:expr, $reader:expr, $inx:expr, $msg:expr) => {{
        let start = $reader.cursor;
        let ele = crate::common::concept::Field::label($msg, start, start + $inx);
        $list.push(ele);
    }};
}

#[macro_export]
macro_rules! field_back_format_fn {
    ($list:expr, $reader:expr, $inx:expr, $body:expr) => {{
        let start = $reader.cursor;
        let content = ($body);
        let ele = crate::common::concept::Field::label(content, start - $inx, start);
        // let ele = crate::common::FieldElement::create(msg, Some(start - $inx..start));
        $list.push(ele);
    }};
}
