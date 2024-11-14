fn channel_flag(inx: usize) -> &'static str {
    match inx {
        0 => "700 MHz spectrum",
        1 => "800 MHz spectrum",
        2 => "900 MHz spectrum",
        4 => "Turbo",
        5 => "Complementary Code Keying (CCK)",
        6 => "Orthogonal Frequency-Division Multiplexing (OFDM)",
        7 => "2 GHz spectrum",
        8 => "5 GHz spectrum",
        9 => "Passive",
        10 => "Dynamic CCK-OFDM",
        11 => "Gaussian Frequency Shift Keying (GFSK)",
        12 => "GSM (900MHz)",
        13 => "Static Turbo",
        14 => "Half Rate Channel (10MHz Channel Width)",
        15 => "Quarter Rate Channel (5MHz Channel Width)",
        _ => "",
    }
}

pub fn get_flag_list(builder: &mut String, val: u16) {
    for inx in 0..16 {
        if val & 1 << inx > 0 {
            builder.push_str(", ");
            builder.push_str(channel_flag(inx));
        }
    }
}

pub fn bit_set(val: u16, mask: u16) -> bool {
    (val & mask) == mask
}
pub fn bit_set32(val: u32, mask: u32) -> bool {
    (val & mask) == mask
}
// fn bit_ck(val: u16) -> bool {

// }

pub fn _get_x_channel_flag(builder: &mut String, val: u32) {
    let mut _append = |cont: &str| {
        builder.push_str(", ");
        builder.push_str(cont);
    };
    if bit_set32(val, 0x00000010) {
        _append("Channel Type Turbo");
    }
    if bit_set32(val, 0x00000020) {
        _append("Channel Type Complementary Code Keying (CCK) Modulation");
    }
    if bit_set32(val, 0x00000040) {
        _append("Channel Type Orthogonal Frequency-Division Multiplexing (OFDM)");
    }
    if bit_set32(val, 0x00000080) {
        _append("Channel Type 2 GHz spectrum");
    }

    if bit_set32(val, 0x00000100) {
        _append("Channel Type 5 GHz spectrum");
    }
    if bit_set32(val, 0x00000200) {
        _append("Channel Type Passive");
    }
    if bit_set32(val, 0x00000400) {
        _append("Channel Type Dynamic CCK-OFDM Channel");
    }
    if bit_set32(val, 0x00000800) {
        _append("Channel Type Gaussian Frequency Shift Keying (GFSK) Modulation");
    }

    if bit_set32(val, 0x00001000) {
        _append("Channel Type GSM");
    }
    if bit_set32(val, 0x00002000) {
        _append("Channel Type Status Turbo");
    }
    if bit_set32(val, 0x00004000) {
        _append("Channel Type Half Rate");
    }
    if bit_set32(val, 0x00008000) {
        _append("Channel Type Quarter Rate");
    }
    if bit_set32(val, 0x00010000) {
        _append("Channel Type HT/20");
    }
    if bit_set32(val, 0x00020000) {
        _append("Channel Type HT/40+");
    }
    if bit_set32(val, 0x00040000) {
        _append("Channel Type HT/40-");
    }
}

pub fn get_he(index: u8, builder: &mut String, val: u16) {
    let mut _append = |cont: &str| {
        builder.push_str(", ");
        builder.push_str(cont);
    };

    match index {
        1 => {
            if bit_set(val, 0x0003) {
                _append("HE_TRIG");
            } else if bit_set(val, 0x0001) {
                _append("HE_EXT");
            } else if bit_set(val, 0x0002) {
                _append("HE_MU");
            } else {
                _append("HE_SU");
            }
            if bit_set(val, 0x0004) {
                _append("BSS Color known");
            }
            if bit_set(val, 0x0008) {
                _append("Beam Change known");
            }
            if bit_set(val, 0x0010) {
                _append("UL/DL known");
            }
            if bit_set(val, 0x0020) {
                _append("data MCS known");
            }
            if bit_set(val, 0x0040) {
                _append("data DCM known");
            }
            if bit_set(val, 0x0080) {
                _append("Coding known");
            }
            if bit_set(val, 0x0100) {
                _append("LDPC extra symbol segment known");
            }
            if bit_set(val, 0x0200) {
                _append("STBC known");
            }
            if bit_set(val, 0x0400) {
                _append("Spatial Reuse known (HE_TRIG format)");
            }
            if bit_set(val, 0x0800) {
                _append("Spatial Reuse 2 known (HE_TRIG format)");
            }
            if bit_set(val, 0x1000) {
                _append("Spatial Reuse 3 known (HE_TRIG format)");
            }
            if bit_set(val, 0x2000) {
                _append("Spatial Reuse 4 known (HE_TRIG format)");
            }
            if bit_set(val, 0x4000) {
                _append("data BW/RU allocation known");
            }
            if bit_set(val, 0x8000) {
                _append("Doppler known");
            }
        }
        2 => {
            if bit_set(val, 0x0001) {
                _append("pri/sec 80 MHz known");
            }
            if bit_set(val, 0x0002) {
                _append("GI known");
            }
            if bit_set(val, 0x0004) {
                _append("number of LTF symbols known");
            }
            if bit_set(val, 0x0008) {
                _append("Pre-FEC Padding Factor known");
            }
            if bit_set(val, 0x0010) {
                _append("TxBF known");
            }
            if bit_set(val, 0x0020) {
                _append("PE Disambiguity known");
            }
            if bit_set(val, 0x0040) {
                _append("TXOP known");
            }
            if bit_set(val, 0x0080) {
                _append("midamble periodicity known");
            }
            if bit_set(val, 0x3f00) {
                _append("RU allocation offset");
            }
            if bit_set(val, 0x4000) {
                _append("RU allocation offset known");
            }
            if bit_set(val, 0x8000) {
                _append("pri/sec 80 MHz");
            }
        }
        3 => {
            if bit_set(val, 0x003f) {
                _append("BSS Color");
            }
            if bit_set(val, 0x0040) {
                _append("Beam Change");
            }
            if bit_set(val, 0x0080) {
                _append("UL/DL");
            }
            if bit_set(val, 0x0f00) {
                _append("data MCS (not SIG-B MCS from HE-SIG-A for HE_MU format)");
            }
            if bit_set(val, 0x1000) {
                _append("data DCM (cf. data MCS)");
            }
            if bit_set(val, 0x2000) {
                _append("Coding (0=BCC, 1=LDPC)");
            }
            if bit_set(val, 0x4000) {
                _append("LDPC extra symbol segment");
            }
            if bit_set(val, 0x8000) {
                _append("STBC");
            }
        }
        4 => {
            //todo
            // if bit_set(val, 0x000f) {
            //     _append("Spatial Reuse");
            // }
        }
        5 => {
            let set1 = val & 0x000f;
            let ru = match set1 {
                0 => "20",
                1 => "40",
                2 => "80",
                3 => "160/80+80",
                4 => "26-tone RU",
                5 => "52-tone RU",
                6 => "106-tone RU",
                7 => "242-tone RU",
                8 => "484-tone RU",
                9 => "996-tone RU",
                10 => "2x996-tone RU",
                _ => "",
            };

            _append(format!("data Bandwidth/RU allocation: {}", ru).as_str());

            let set2 = val & 0x0030;
            let gi = match set2 {
                0 => "0.8us",
                0x0010 => "1.6us",
                0x0020 => "3.2us",
                _ => "reserved",
            };
            _append(format!("GI : {}", gi).as_str());
            let set3 = (val >> 6) & 0x03;

            let symbol_size = match set3 {
                0 => "unknown",
                1 => "1x",
                2 => "2x",
                _ => "4x",
            };
            _append(format!("LTF symbol size: {}", symbol_size).as_str());

            let set4 = (val >> 8) & 0x07;
            let symbol = match set4 {
                0 => "1x",
                1 => "2x",
                2 => "4x",
                3 => "6x",
                4 => "8x",
                _ => "",
            };
            _append(format!("LTF symbol: {}", symbol).as_str());

            if bit_set(val, 0x3000) {
                _append("Pre-FEC Padding Factor");
            }
            if bit_set(val, 0x4000) {
                _append("TxBF");
            }
            if bit_set(val, 0x8000) {
                _append("PE Disambiguity");
            }
        }
        6 => {
            if bit_set(val, 0x0001) {
                _append("NSTS");
            }
            if bit_set(val, 0x0010) {
                _append("Doppler value");
            }
            if bit_set(val, 0x7f00) {
                _append("TXOP value");
            }
            let set2 = (val >> 15) & 0x01;
            if set2 == 1 {
                _append("midamble periodicity: 20");
            } else {
                _append("midamble periodicity: 10");
            }
        }
        _ => {}
    }
}

pub fn get_vht_bandwidth(val: u8) -> &'static str {
    match val {
        0 => "bandwidth: 20MHz",
        1 => "bandwidth: 40MHz",
        4 => "bandwidth: 80MHz",
        11 => "bandwidth: 160MHz",
        2 => "bandwidth: 40MHz, sideband: 20L, sideband index: 0",
        3 => "bandwidth: 40MHz, sideband: 20U, sideband index: 1",
        5 => "bandwidth: 80MHz, sideband: 40L, sideband index: 0",
        6 => "bandwidth: 80MHz, sideband: 40U, sideband index: 1",
        7 => "bandwidth: 80MHz, sideband: 20LL, sideband index: 0",
        8 => "bandwidth: 80MHz, sideband: 20LU, sideband index: 1",
        9 => "bandwidth: 80MHz, sideband: 20UL, sideband index: 2",
        10 => "bandwidth: 80MHz, sideband: 20UU, sideband index: 3",
        12 => "bandwidth: 160MHz, sideband: 80L, sideband index: 0",
        13 => "bandwidth: 160MHz, sideband: 80U, sideband index: 1",
        14 => "bandwidth: 160MHz, sideband: 40LL, sideband index: 0",
        15 => "bandwidth: 160MHz, sideband: 40LU, sideband index: 1",
        16 => "bandwidth: 160MHz, sideband: 40UL, sideband index: 2",
        17 => "bandwidth: 160MHz, sideband: 40UU, sideband index: 3",
        18 => "bandwidth: 160MHz, sideband: 20LLL, sideband index: 0",
        19 => "bandwidth: 160MHz, sideband: 20LLU, sideband index: 1",
        20 => "bandwidth: 160MHz, sideband: 20LUL, sideband index: 2",
        21 => "bandwidth: 160MHz, sideband: 20LUU, sideband index: 3",
        22 => "bandwidth: 160MHz, sideband: 20ULL, sideband index: 4",
        23 => "bandwidth: 160MHz, sideband: 20ULU, sideband index: 5",
        24 => "bandwidth: 160MHz, sideband: 20UUL, sideband index: 6",
        25 => "bandwidth: 160MHz, sideband: 20UUU, sideband index: 7",
        _ => "",
    }
}
