// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

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
macro_rules! add_field_label_no_range {
    ($field:expr, $msg:expr) => {{
        let start = $field.start;
        let mut ele = $crate::common::concept::Field::label($msg, start, start);
        ele.source = $field.source;
        $field.children.as_mut().unwrap().push(ele);
    }};
}

#[macro_export]
macro_rules! add_field_label {
    ($field:expr, $msg:expr) => {{
        let start = $field.start;
        let mut ele = $crate::common::concept::Field::label($msg, start, start + $field.size);
        ele.source = $field.source;
        $field.children.as_mut().unwrap().push(ele);
    }};
}

#[macro_export]
macro_rules! add_field_format {
    ($field:expr, $reader:expr, $body:expr, $msg_template:expr) => {{
        let start = $reader.cursor;
        let content = ($body);
        let end = $reader.cursor;
        let mut ele = $crate::common::concept::Field::label(format!($msg_template, content), start, end);
        ele.source = $field.source;
        $field.children.as_mut().unwrap().push(ele);
        content
    }};
}
#[macro_export]
macro_rules! add_field_format_fn {
    ($field:expr, $reader:expr, $body:expr, $fn_ref:expr) => {{
        let start = $reader.cursor;
        let content = ($body);
        let msg = $fn_ref(content);
        let end = $reader.cursor;
        let mut ele = $crate::common::concept::Field::label(msg, start, end);
        ele.source = $field.source;
        $field.children.as_mut().unwrap().push(ele);
        content
    }};
}
#[macro_export]
macro_rules! add_field_format_fn_nors {
    ($field:expr, $reader:expr, $body:expr, $fn_ref:expr) => {{
        let start = $reader.cursor;
        let content = ($body);
        let msg = $fn_ref(content);
        let end = $reader.cursor;
        let mut ele = $crate::common::concept::Field::label(msg, start, end);
        ele.source = $field.source;
        $field.children.as_mut().unwrap().push(ele);
    }};
}


#[macro_export]
macro_rules! add_field_rest_format {
    ($field:expr, $reader:expr, $msg:expr) => {{
        if $reader.left() > 0 {
            let start = $reader.cursor;
            let end = $reader.cursor + $reader.left();
            let mut ele = $crate::common::concept::Field::label($msg, start, end);
            ele.source = $field.source;
            $field.children.as_mut().unwrap().push(ele);
        }
    }};
}

#[macro_export]
macro_rules! add_sub_field {
    ($field:expr, $reader:expr, $body:expr, $fn_ref:expr) => {{
        let mut _field = Field::with_children("".into(), $reader.cursor, 0);
        _field.source = $field.source;
        let content = ($body);
        _field.size = $reader.cursor - _field.start;
        let _ = $fn_ref(content, &mut _field);
        $field.children.as_mut().unwrap().push(_field);
        content
    }};
}

#[macro_export]
macro_rules! add_sub_field_with_reader {
    ($field:expr, $reader:expr, $fn_ref:expr) => {{
        let mut _field = Field::with_children("".into(), $reader.cursor, 0);
        _field.source = $field.source;
        let rs = $fn_ref($reader, &mut _field);
        _field.size = $reader.cursor - _field.start;
        $field.children.as_mut().unwrap().push(_field);
        rs
    }};
}


#[macro_export]
macro_rules! add_field_backstep {
    ($field:expr, $reader:expr, $inx:expr, $msg:expr) => {{
        let start = $reader.cursor;
        let mut ele = $crate::common::concept::Field::label($msg, start - $inx, start);
        ele.source = $field.source;
        let inx = $field.children.as_ref().unwrap().len();
        $field.children.as_mut().unwrap().push(ele);
        inx
    }};
}
#[macro_export]
macro_rules! add_field_backstep_fn {
    ($field:expr, $reader:expr, $inx:expr, $body:expr) => {{
        let start = $reader.cursor;
        let content = ($body);
        let mut ele = $crate::common::concept::Field::label(content, start - $inx, start);
        ele.source = $field.source;
        $field.children.as_mut().unwrap().push(ele);
    }};
}
#[macro_export]
macro_rules! add_field_forward {
    ($field:expr, $reader:expr, $inx:expr, $msg:expr) => {{
        let start = $reader.cursor;
        let mut ele = $crate::common::concept::Field::label($msg, start, start + $inx);
        ele.source = $field.source;
        let inx = $field.children.as_ref().unwrap().len();
        $field.children.as_mut().unwrap().push(ele);
        inx
    }};
}

// #[macro_export]
// macro_rules! read_field_format {
//     ($list:expr, $reader:expr, $body:expr, $msg_template:expr) => {{
//         let start = $reader.cursor;
//         let content = ($body);
//         let end = $reader.cursor;
//         let ele = $crate::common::concept::Field::label(format!($msg_template, content), start, end);
//         $list.push(ele);
//         content
//     }};
// }
// #[macro_export]
// macro_rules! read_field_format_fn {
//     ($list:expr, $reader:expr, $body:expr, $fn_ref:expr) => {{
//         let start = $reader.cursor;
//         let content = ($body);
//         let end = $reader.cursor;
//         let msg = $fn_ref(content);
//         let ele = $crate::common::concept::Field::label(msg, start, end);
//         $list.push(ele);
//         content
//     }};
// }

// #[macro_export]
// macro_rules! field_back_format {
//     ($list:expr, $reader:expr, $inx:expr, $msg:expr) => {{
//         let start = $reader.cursor;
//         let ele = $crate::common::concept::Field::label($msg, start - $inx, start);
//         let inx = $list.len();
//         $list.push(ele);
//         inx
//     }};
// }

// #[macro_export]
// macro_rules! field_back_format_with_list {
//     ($list:expr, $reader:expr, $inx:expr, $msg:expr, $sub_list:expr) => {{
//         let start = $reader.cursor;
//         let mut ele = $crate::common::concept::Field::label($msg, start - $inx, start);
//         ele.children = Some($sub_list);
//         let inx = $list.len();
//         $list.push(ele);
//         inx
//     }};
// }

// #[macro_export]
// macro_rules! field_rest_format {
//     ($list:expr, $reader:expr, $msg:expr) => {{
//         if $reader.left() > 0 {
//             let start = $reader.cursor;
//             let end = $reader.cursor + $reader.left();
//             let ele = $crate::common::concept::Field::label($msg, start, end);
//             // let inx = $list.len();
//             $list.push(ele);
//         }
//     }};
// }

// #[macro_export]
// macro_rules! field_forward_format {
//     ($list:expr, $reader:expr, $inx:expr, $msg:expr) => {{
//         let start = $reader.cursor;
//         let ele = $crate::common::concept::Field::label($msg, start, start + $inx);
//         $list.push(ele);
//     }};
// }

// #[macro_export]
// macro_rules! field_back_format_fn {
//     ($list:expr, $reader:expr, $inx:expr, $body:expr) => {{
//         let start = $reader.cursor;
//         let content = ($body);
//         let ele = $crate::common::concept::Field::label(content, start - $inx, start);
//         // let ele = crate::common::FieldElement::create(msg, Some(start - $inx..start));
//         $list.push(ele);
//     }};
// }
