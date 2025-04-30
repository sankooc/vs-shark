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
        let start = $reader.cursor as u64;
        let content = ($body);
        let end = $reader.cursor as u64;
        let msg = intern(format!($msg_template, content));
        let ele = FieldElement::create(msg, Some(start..end));
        $list.push(ele);
        content
    }};
}

#[macro_export]
macro_rules! field_back_format {
    ($list:expr, $reader:expr, $inx:expr, $msg:expr) => {{
        let start = $reader.cursor as u64;
        let ele = FieldElement::create(crate::cache::intern($msg), Some(start-$inx..start));
        $list.push(ele);
    }};
}