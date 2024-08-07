#[cfg(test)]
mod tests {
    use log::info;
    use crate::{
        common::{IPv4Address, Protocol},
        files::Field,
    };

    fn _dis(inx: usize, field: &Field) {
        //assert_eq!("hello       ", format!("{:width$}", "hello", width=12));
        info!("{:pad$}- {}", "", field.summary(), pad = inx);
        let fields = field.children();
        for f in fields.iter() {
            _dis(inx + 1, f);
        }
    }
    #[test]
    fn testbasic() -> std::io::Result<()> {
        use crate::entry::load_data;
        // use log::{error, info};
        use std::fs;
        // use env_logger::{Builder, Target};
        env_logger::builder().is_test(true).try_init().unwrap();
        // let fname = "../sandbox/demo.pcap";
        // let fname = "../sandbox/11.pcapng";
        // let fname = "../sandbox/dns.pcapng";
        let fname = "../sandbox/creden.pcapng";
        let data: Vec<u8> = fs::read(fname)?;
        let _ctx = load_data(&data).unwrap();
        let frames = _ctx.get_frames();
        for f in frames.iter() {
            // match f.summary.borrow().protocol {
            //     Protocol::DNS => (),
            //     _ => continue,
            // }
            info!(
                "inx:{} protocol: {:?} size:{}",
                f.summary.borrow().index,
                f.summary.borrow().protocol,
                f.capture_size
            );
            let ff = f.eles.borrow();
            for e in ff.iter() {
                info!("- {}", e.summary());
                let fields = e.get_fields();
                for field in fields.iter() {
                    _dis(1, field);
                }
            }
        }
        Ok(())
    }

    // use pcap::HelloMacro;
    // use pcap_derive::HelloMacro;

    // pub trait HelloMacro {
    //     fn hello_macro();
    // }
    // #[derive(HelloMacro)]
    // struct Pancakes {
    //     age: u16,
    // }
    #[test]
    fn testip() {
        // env_logger::builder().is_test(true).try_init().unwrap();
        let ip = IPv4Address {
            data: [0xff, 0xff, 0xff, 0xff],
        };
        info!("ip {}", ip);
    }
}
