#[cfg(test)]
mod tests {
    use core::files;
    use std::{borrow::Borrow, cell::RefCell, time::{SystemTime, UNIX_EPOCH}};

    use js_sys::Uint8Array;
    use log::info;

    use crate::nshark::load;
    #[derive(Default, Clone)]
    pub struct Field {
        pub start: usize,
        pub size: usize,
        summary: String,
        children: RefCell<Vec<files::Field>>,
    }
    impl Field {
        pub fn convert(embed: &files::Field) -> Self {
            let (start, size);
            files::Field { start, size, .. } = *embed;
            let summary = embed.summary.clone();
            let a:&[u8] = embed.borrow().data.as_ref();
            // let a: &[u8] = embed.borrow().deref();
            let children = embed.children.clone();
            Field {
                start,
                size,
                summary,
                children,
            }
        }
    }
    impl Field {
        pub fn summary(&self) -> String {
            self.summary.clone()
        }
        pub fn children(&self) -> Vec<Field> {
            let mut children = Vec::new();
            for c in self.children.borrow().iter() {
                children.push(Field::convert(c));
            }
            children
        }
    }
    
    // pub struct DNSRecord {
    //     name: String,
    //     _type: String,
    //     proto: String,
    //     class: String,
    //     content: String,
    //     pub ttl: u32,
    // }
    
    // impl DNSRecord {
    //     pub fn create(data: &RecordResource) -> DNSRecord {
    //         DNSRecord {
    //             name: data.name(),
    //             _type: data._type(),
    //             proto: data.proto(),
    //             class: data.class(),
    //             content: data.content(),
    //             ttl: data.ttl(),
    //         }
    //     }
    // }

    // fn _dis(inx: usize, field: &Field) {
    //     //assert_eq!("hello       ", format!("{:width$}", "hello", width=12));
    //     info!("{:pad$}- {}", "", field.summary(), pad = inx);
    //     let fields = field.children();
    //     for f in fields.iter() {
    //         _dis(inx + 1, f);
    //     }
    // } 
    #[test]
    fn testbasic() -> std::io::Result<()> {
        use core::entry::load_data;
        // use log::{error, info};
        // std::panic::catch_unwind();
        use std::fs;
        // use env_logger::{Builder, Target};
        env_logger::builder().is_test(true).try_init().unwrap();
        // let fname = "../sandbox/demo.pcap";
        // let fname = "../sandbox/demo.pcapng";
        // let fname = "../sandbox/11.pcapng";
        // let fname = "../sandbox/dns.pcapng";
        // let fname = "../sandbox/creden.pcapng";
        // let fname = "../sandbox/ftp.pcapng";
        // let fname = "../sandbox/wifi.pcap";
        let fname = "../sandbox/c1.pcap";
        let data: Vec<u8> = fs::read(fname)?;
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let _ctx = load_data(&data).unwrap();
        // let a: &[u8] = &data;
        // let uin: Uint8Array = a.into();
        // load(&uin);
        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

        println!("finish cost {}", end -start);
        Ok(())
    }

}
