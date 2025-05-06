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
        let msg = crate::cache::intern(format!($msg_template, content));
        let ele = crate::common::FieldElement::create(msg, Some(start..end));
        $list.push(ele);
        content
    }};
}
#[macro_export]
macro_rules! read_field_format_fn {
    ($list:expr, $reader:expr, $body:expr, $fn_ref:expr) => {{
        let start = $reader.cursor as u64;
        let content = ($body);
        let end = $reader.cursor as u64;
        let msg = crate::cache::intern($fn_ref(content));
        let ele = crate::common::FieldElement::create(msg, Some(start..end));
        $list.push(ele);
        content
    }};
}

#[macro_export]
macro_rules! field_back_format {
    ($list:expr, $reader:expr, $inx:expr, $msg:expr) => {{
        let start = $reader.cursor as u64;
        let ele = crate::common::FieldElement::create(crate::cache::intern($msg), Some(start-$inx..start));
        $list.push(ele);
    }};
}


#[macro_export]
macro_rules! field_back_format_fn {
    ($list:expr, $reader:expr, $inx:expr, $body:expr) => {{
        let start = $reader.cursor as u64;
        let content = ($body);
        let msg = crate::cache::intern(content);
        let ele = crate::common::FieldElement::create(msg, Some(start-$inx..start));
        $list.push(ele);
    }};
}