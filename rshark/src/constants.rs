use lazy_static::lazy_static;
use std::collections::HashMap;
lazy_static! {
	pub static ref link_type_map: HashMap<u16, &'static str> = {
		let mut m = HashMap::new();
		m.insert(0, "NULL");
		m.insert(1, "ETHERNET");
		m.insert(2, "EXP_ETHERNET");
		m.insert(3, "AX25");
		m.insert(4, "PRONET");
		m.insert(5, "CHAOS");
		m.insert(6, "IEEE802_5");
		m.insert(7, "ARCNET_BSD");
		m.insert(8, "SLIP");
		m.insert(9, "PPP");
		m.insert(10, "FDDI");
		m.insert(32, "DLT_REDBACK_SMARTEDGE");
		m.insert(50, "PPP_HDLC");
		m.insert(51, "PPP_ETHER");
		m.insert(99, "SYMANTEC_FIREWALL");
		m.insert(100, "ATM_RFC1483");
		m.insert(101, "RAW");
		m.insert(104, "C_HDLC");
		m.insert(105, "IEEE802_11");
		m.insert(106, "ATM_CLIP");
		m.insert(107, "FRELAY");
		m.insert(108, "LOOP");
		m.insert(109, "ENC");
		m.insert(112, "NETBSD_HDLC");
		m.insert(113, "LINUX_SLL");
		m.insert(114, "LTALK");
		m.insert(115, "DLT_ECONET");
		m.insert(116, "DLT_IPFILTER");
		m.insert(117, "PFLOG");
		m.insert(118, "DLT_CISCO_IOS");
		m.insert(119, "IEEE802_11_PRISM");
		m.insert(120, "DLT_AIRONET_HEADER");
		m.insert(122, "IP_OVER_FC");
		m.insert(123, "SUNATM");
		m.insert(124, "DLT_RIO");
		m.insert(125, "DLT_PCI_EXP");
		m.insert(126, "DLT_AURORA");
		m.insert(127, "IEEE802_11_RADIOTAP");
		m.insert(128, "TZSP");
		m.insert(129, "ARCNET_LINUX");
		m.insert(130, "JUNIPER_MLPPP");
		m.insert(131, "JUNIPER_MLFR");
		m.insert(132, "JUNIPER_ES");
		m.insert(133, "JUNIPER_GGSN");
		m.insert(134, "JUNIPER_MFR");
		m.insert(135, "JUNIPER_ATM2");
		m.insert(136, "JUNIPER_SERVICES");
		m.insert(137, "JUNIPER_ATM1");
		m.insert(138, "APPLE_IP_OVER_IEEE1394");
		m.insert(139, "MTP2_WITH_PHDR");
		m.insert(140, "MTP2");
		m.insert(141, "MTP3");
		m.insert(142, "SCCP");
		m.insert(143, "DOCSIS");
		m.insert(144, "LINUX_IRDA");
		m.insert(145, "IBM_SP");
		m.insert(146, "IBM_SN");
		m.insert(147, "USER0_USER15");
		m.insert(163, "IEEE802_11_AVS");
		m.insert(164, "JUNIPER_MONITOR");
		m.insert(165, "BACNET_MS_TP");
		m.insert(166, "PPP_PPPD");
		m.insert(167, "JUNIPER_PPPOE");
		m.insert(168, "JUNIPER_PPPOE_ATM");
		m.insert(169, "GPRS_LLC");
		m.insert(170, "GPF_T");
		m.insert(171, "GPF_F");
		m.insert(172, "GCOM_T1E1");
		m.insert(173, "GCOM_SERIAL");
		m.insert(174, "JUNIPER_PIC_PEER");
		m.insert(175, "ERF_ETH");
		m.insert(176, "ERF_POS");
		m.insert(177, "LINUX_LAPD");
		m.insert(178, "JUNIPER_ETHER");
		m.insert(179, "JUNIPER_PPP");
		m.insert(180, "JUNIPER_FRELAY");
		m.insert(181, "JUNIPER_CHDLC");
		m.insert(182, "MFR");
		m.insert(183, "JUNIPER_VP");
		m.insert(184, "A429");
		m.insert(185, "A653_ICM");
		m.insert(186, "USB_FREEBSD");
		m.insert(187, "BLUETOOTH_HCI_H4");
		m.insert(188, "IEEE802_16_MAC_CPS");
		m.insert(189, "USB_LINUX");
		m.insert(190, "CAN20B");
		m.insert(191, "IEEE802_15_4_LINUX");
		m.insert(192, "PPI");
		m.insert(193, "IEEE802_16_MAC_CPS_RADIO");
		m.insert(194, "JUNIPER_ISM");
		m.insert(195, "IEEE802_15_4_WITHFCS");
		m.insert(196, "SITA");
		m.insert(197, "ERF");
		m.insert(198, "RAIF1");
		m.insert(199, "IPMB_KONTRON");
		m.insert(200, "JUNIPER_ST");
		m.insert(201, "BLUETOOTH_HCI_H4_WITH_PHDR");
		m.insert(202, "AX25_KISS");
		m.insert(203, "LAPD");
		m.insert(204, "PPP_WITH_DIR");
		m.insert(205, "C_HDLC_WITH_DIR");
		m.insert(206, "FRELAY_WITH_DIR");
		m.insert(207, "LAPB_WITH_DIR");
		m.insert(209, "IPMB_LINUX");
		m.insert(210, "FLEXRAY");
		m.insert(211, "MOST");
		m.insert(212, "LIN");
		m.insert(213, "X2E_SERIAL");
		m.insert(214, "X2E_XORAYA");
		m.insert(215, "IEEE802_15_4_NONASK_PHY");
		m.insert(216, "LINUX_EVDEV");
		m.insert(217, "GSMTAP_UM");
		m.insert(218, "GSMTAP_ABIS");
		m.insert(219, "MPLS");
		m.insert(220, "USB_LINUX_MMAPPED");
		m.insert(221, "DECT");
		m.insert(222, "AOS");
		m.insert(223, "WIHART");
		m.insert(224, "FC_2");
		m.insert(225, "FC_2_WITH_FRAME_DELIMS");
		m.insert(226, "IPNET");
		m.insert(227, "CAN_SOCKETCAN");
		m.insert(228, "IPV4");
		m.insert(229, "IPV6");
		m.insert(230, "IEEE802_15_4_NOFCS");
		m.insert(231, "DBUS");
		m.insert(232, "JUNIPER_VS");
		m.insert(233, "JUNIPER_SRX_E2E");
		m.insert(234, "JUNIPER_FIBRECHANNEL");
		m.insert(235, "DVB_CI");
		m.insert(236, "MUX27010");
		m.insert(237, "STANAG_5066_D_PDU");
		m.insert(238, "JUNIPER_ATM_CEMIC");
		m.insert(239, "NFLOG");
		m.insert(240, "NETANALYZER");
		m.insert(241, "NETANALYZER_TRANSPARENT");
		m.insert(242, "IPOIB");
		m.insert(243, "MPEG_2_TS");
		m.insert(244, "NG40");
		m.insert(245, "NFC_LLCP");
		m.insert(246, "PFSYNC");
		m.insert(247, "INFINIBAND");
		m.insert(248, "SCTP");
		m.insert(249, "USBPCAP");
		m.insert(250, "RTAC_SERIAL");
		m.insert(251, "BLUETOOTH_LE_LL");
		m.insert(252, "WIRESHARK_UPPER_PDU");
		m.insert(253, "NETLINK");
		m.insert(254, "BLUETOOTH_LINUX_MONITOR");
		m.insert(255, "BLUETOOTH_BREDR_BB");
		m.insert(256, "BLUETOOTH_LE_LL_WITH_PHDR");
		m.insert(257, "PROFIBUS_DL");
		m.insert(258, "PKTAP");
		m.insert(259, "EPON");
		m.insert(260, "IPMI_HPM_2");
		m.insert(261, "ZWAVE_R1_R2");
		m.insert(262, "ZWAVE_R3");
		m.insert(263, "WATTSTOPPER_DLM");
		m.insert(264, "ISO_14443");
		m.insert(265, "RDS");
		m.insert(266, "USB_DARWIN");
		m.insert(267, "OPENFLOW");
		m.insert(268, "SDLC");
		m.insert(269, "TI_LLN_SNIFFER");
		m.insert(270, "LORATAP");
		m.insert(271, "VSOCK");
		m.insert(272, "NORDIC_BLE");
		m.insert(273, "DOCSIS31_XRA31");
		m.insert(274, "ETHERNET_MPACKET");
		m.insert(275, "DISPLAYPORT_AUX");
		m.insert(276, "LINUX_SLL2");
		m.insert(277, "SERCOS_MONITOR");
		m.insert(278, "OPENVIZSLA");
		m.insert(279, "EBHSCR");
		m.insert(280, "VPP_DISPATCH");
		m.insert(281, "DSA_TAG_BRCM");
		m.insert(282, "DSA_TAG_BRCM_PREPEND");
		m.insert(283, "IEEE802_15_4_TAP");
		m.insert(284, "DSA_TAG_DSA");
		m.insert(285, "DSA_TAG_EDSA");
		m.insert(286, "ELEE");
		m.insert(287, "Z_WAVE_SERIAL");
		m.insert(288, "USB_2_0");
		m.insert(289, "ATSC_ALP");
		m.insert(290, "ETW");
		m.insert(291, "NETANALYZER_NG");
		m.insert(292, "ZBOSS_NCP");
		m.insert(293, "USB_2_0_LOW_SPEED");
		m.insert(294, "USB_2_0_FULL_SPEED");
		m.insert(295, "USB_2_0_HIGH_SPEED");
		m.insert(296, "AUERSWALD_LOG");
		m.insert(297, "ZWAVE_TAP");
		m.insert(298, "SILABS_DEBUG_CHANNEL");
		m.insert(299, "FIRA_UCI");
		m.insert(300, "MDB");
		m.insert(301, "DECT_NR");
		m
	};
	pub static ref ip_protocol_type_map: HashMap<u16, &'static str> = {
		let mut m = HashMap::new();
		m.insert(0, "HOPOPT");
		m.insert(1, "ICMP");
		m.insert(2, "IGMP");
		m.insert(3, "GGP");
		m.insert(4, "IP-in-IP");
		m.insert(5, "ST");
		m.insert(6, "TCP");
		m.insert(7, "CBT");
		m.insert(8, "EGP");
		m.insert(9, "IGP");
		m.insert(10, "BBN-RCC-MON");
		m.insert(11, "NVP-II");
		m.insert(12, "PUP");
		m.insert(13, "ARGUS");
		m.insert(14, "EMCON");
		m.insert(15, "XNET");
		m.insert(16, "CHAOS");
		m.insert(17, "UDP");
		m.insert(18, "MUX");
		m.insert(19, "DCN-MEAS");
		m.insert(20, "HMP");
		m.insert(21, "PRM");
		m.insert(22, "XNS-IDP");
		m.insert(23, "TRUNK-1");
		m.insert(24, "TRUNK-2");
		m.insert(25, "LEAF-1");
		m.insert(26, "LEAF-2");
		m.insert(27, "RDP");
		m.insert(28, "IRTP");
		m.insert(29, "ISO-TP4");
		m.insert(30, "NETBLT");
		m.insert(31, "MFE-NSP");
		m.insert(32, "MERIT-INP");
		m.insert(33, "DCCP");
		m.insert(34, "3PC");
		m.insert(35, "IDPR");
		m.insert(36, "XTP");
		m.insert(37, "DDP");
		m.insert(38, "IDPR-CMTP");
		m.insert(39, "TP++");
		m.insert(40, "IL");
		m.insert(41, "IPv6");
		m.insert(42, "SDRP");
		m.insert(43, "IPv6-Route");
		m.insert(44, "IPv6-Frag");
		m.insert(45, "IDRP");
		m.insert(46, "RSVP");
		m.insert(47, "GRE");
		m.insert(48, "DSR");
		m.insert(49, "BNA");
		m.insert(50, "ESP");
		m.insert(51, "AH");
		m.insert(52, "I-NLSP");
		m.insert(53, "SwIPe");
		m.insert(54, "NARP");
		m.insert(55, "MOBILE");
		m.insert(56, "TLSP");
		m.insert(57, "SKIP");
		m.insert(58, "IPv6-ICMP");
		m.insert(59, "IPv6-NoNxt");
		m.insert(60, "IPv6-Opts");
		m.insert(61, "Any");
		m.insert(62, "CFTP");
		m.insert(63, "Any");
		m.insert(64, "SAT-EXPAK");
		m.insert(65, "KRYPTOLAN");
		m.insert(66, "RVD");
		m.insert(67, "IPPC");
		m.insert(68, "Any");
		m.insert(69, "SAT-MON");
		m.insert(70, "VISA");
		m.insert(71, "IPCU");
		m.insert(72, "CPNX");
		m.insert(73, "CPHB");
		m.insert(74, "WSN");
		m.insert(75, "PVP");
		m.insert(76, "BR-SAT-MON");
		m.insert(77, "SUN-ND");
		m.insert(78, "WB-MON");
		m.insert(79, "WB-EXPAK");
		m.insert(80, "ISO-IP");
		m.insert(81, "VMTP");
		m.insert(82, "SECURE-VMTP");
		m.insert(83, "VINES");
		m.insert(84, "TTP");
		m.insert(85, "NSFNET-IGP");
		m.insert(86, "DGP");
		m.insert(87, "TCF");
		m.insert(88, "EIGRP");
		m.insert(89, "OSPF");
		m.insert(90, "Sprite-RPC");
		m.insert(91, "LARP");
		m.insert(92, "MTP");
		m.insert(93, "AX.25");
		m.insert(94, "OS");
		m.insert(95, "MICP");
		m.insert(96, "SCC-SP");
		m.insert(97, "ETHERIP");
		m.insert(98, "ENCAP");
		m.insert(99, "Any");
		m.insert(100, "GMTP");
		m.insert(101, "IFMP");
		m.insert(102, "PNNI");
		m.insert(103, "PIM");
		m.insert(104, "ARIS");
		m.insert(105, "SCPS");
		m.insert(106, "QNX");
		m.insert(107, "A/N");
		m.insert(108, "IPComp");
		m.insert(109, "SNP");
		m.insert(110, "Compaq-Peer");
		m.insert(111, "IPX-in-IP");
		m.insert(112, "VRRP");
		m.insert(113, "PGM");
		m.insert(114, "Any");
		m.insert(115, "L2TP");
		m.insert(116, "DDX");
		m.insert(117, "IATP");
		m.insert(118, "STP");
		m.insert(119, "SRP");
		m.insert(120, "UTI");
		m.insert(121, "SMP");
		m.insert(122, "SM");
		m.insert(123, "PTP");
		m.insert(124, "IS-IS");
		m.insert(125, "FIRE");
		m.insert(126, "CRTP");
		m.insert(127, "CRUDP");
		m.insert(128, "SSCOPMCE");
		m.insert(129, "IPLT");
		m.insert(130, "SPS");
		m.insert(131, "PIPE");
		m.insert(132, "SCTP");
		m.insert(133, "FC");
		m.insert(134, "RSVP-E2E-IGNORE");
		m.insert(135, "Mobility");
		m.insert(136, "UDPLite");
		m.insert(137, "MPLS-in-IP");
		m.insert(138, "manet");
		m.insert(139, "HIP");
		m.insert(140, "Shim6");
		m.insert(141, "WESP");
		m.insert(142, "ROHC");
		m.insert(143, "Ethernet");
		m.insert(144, "AGGFRAG");
		m.insert(145, "NSH");
		m
	};
	pub static ref ssl_type_map: HashMap<u16, &'static str> = {
		let mut m = HashMap::new();
		m.insert(0, "Sent to us");
		m.insert(1, "Boardcast");
		m.insert(2, "Multicast not boardcast");
		m.insert(3, "Send to somebody else by somebody else");
		m.insert(4, "Send by us");
		m
	};
	pub static ref etype_map: HashMap<u16, &'static str> = {
		let mut m = HashMap::new();
		m.insert(2048, "IPv4");
		m.insert(2049, "X.75");
		m.insert(2053, "X.25 Level 3");
		m.insert(2054, "ARP");
		m.insert(2056, "Frame Relay ARP");
		m.insert(8947, "TRILL");
		m.insert(8948, "L2-IS-IS");
		m.insert(25944, "Trans Ether Bridging");
		m.insert(25945, "Raw Frame Relay");
		m.insert(32821, "RARP");
		m.insert(32923, "Appletalk");
		m.insert(33024, "802.1Q");
		m.insert(33079, "IPX/SPX");
		m.insert(33100, "SNMP");
		m.insert(34525, "IPv6");
		m.insert(34667, "TCP/IP Compression");
		m.insert(34668, "IP Autonomous Systems");
		m.insert(34669, "Secure Data");
		m.insert(34824, "IEEE Std 802.3 - Ethernet Passive Optical Network (EPON)");
		m.insert(34827, "PPP");
		m.insert(34828, "General Switch Management Protocol (GSMP)");
		m.insert(34887, "MPLS (multiprotocol label switching)");
		m.insert(34888, "MPLS with upstream-assigned label");
		m.insert(34915, "PPP over Ethernet (PPPoE) Discovery Stage");
		m.insert(34916, "PPP over Ethernet (PPPoE) Session Stage");
		m.insert(34958, "IEEE Std 802.1X - Port-based network access control");
		m.insert(34984, "IEEE Std 802.1Q - Service VLAN tag identifier (S-Tag)");
		m.insert(34999, "IEEE Std 802 - OUI Extended Ethertype");
		m.insert(35015, "IEEE Std 802.11 - Pre-Authentication (802.11i)");
		m.insert(35020, "IEEE Std 802.1AB - Link Layer Discovery Protocol (LLDP)");
		m.insert(35045, "IEEE Std 802.1AE - Media Access Control Security");
		m.insert(35061, "IEEE Std 802.1Q - Multiple VLAN Registration Protocol (MVRP)");
		m.insert(35062, "IEEE Std 802.1Q - Multiple Multicast Registration Protocol (MMRP)");
		m.insert(35131, "FGL");
		m.insert(35142, "TRILL RBridge Channel");
		m.insert(35130, "IEE 1905.1a");
		m.insert(35085, "TDLS");
		m
	};
	pub static ref tcp_option_kind_map: HashMap<u16, &'static str> = {
		let mut m = HashMap::new();
		m.insert(0, "End OF LIST");
		m.insert(1, "No Operation");
		m.insert(2, "Max segment size");
		m.insert(3, "Window scale");
		m.insert(4, "Selective Acknowledgement permitted");
		m.insert(5, "Selective ACKnowledgement SACK");
		m.insert(8, "echo of previous timestamp");
		m.insert(28, "User Timeout Option");
		m.insert(29, "TCP Authentication");
		m.insert(30, "MPTCP");
		m
	};
	pub static ref dns_class_map: HashMap<u16, &'static str> = {
		let mut m = HashMap::new();
		m.insert(1, "IN");
		m.insert(2, "CS");
		m.insert(3, "CH");
		m.insert(4, "HS");
		m
	};
	pub static ref dns_type_map: HashMap<u16, &'static str> = {
		let mut m = HashMap::new();
		m.insert(1, "A");
		m.insert(2, "NS");
		m.insert(3, "MD");
		m.insert(4, "MF");
		m.insert(5, "CNAME");
		m.insert(6, "SOA");
		m.insert(7, "MB");
		m.insert(8, "MG");
		m.insert(9, "MR");
		m.insert(10, "NULL");
		m.insert(11, "WKS");
		m.insert(12, "PTR");
		m.insert(13, "HINFO");
		m.insert(14, "MINFO");
		m.insert(15, "MX");
		m.insert(16, "TXT");
		m.insert(28, "AAAA");
		m
	};
	pub static ref arp_hardware_type_map: HashMap<u16, &'static str> = {
		let mut m = HashMap::new();
		m.insert(0, "Reserved");
		m.insert(1, "Ethernet (10Mb)");
		m.insert(2, "Experimental Ethernet (3Mb)");
		m.insert(3, "Amateur Radio AX.25");
		m.insert(4, "Proteon ProNET Token Ring");
		m.insert(5, "Chaos");
		m.insert(6, "IEEE 802 Networks");
		m.insert(7, "ARCNET");
		m.insert(8, "Hyperchannel");
		m.insert(9, "Lanstar");
		m.insert(10, "Autonet Short Address");
		m.insert(11, "LocalTalk");
		m.insert(12, "LocalNet (IBM PCNet or SYTEK LocalNET)");
		m.insert(13, "Ultra link");
		m.insert(14, "SMDS");
		m.insert(15, "Frame Relay");
		m.insert(16, "Asynchronous Transmission Mode (ATM)");
		m.insert(17, "HDLC");
		m.insert(18, "Fibre Channel");
		m.insert(19, "Asynchronous Transmission Mode (ATM)");
		m.insert(20, "Serial Line");
		m.insert(21, "Asynchronous Transmission Mode (ATM)");
		m.insert(22, "MIL-STD-188-220");
		m.insert(23, "Metricom");
		m.insert(24, "IEEE 1394.1995");
		m.insert(25, "MAPOS");
		m.insert(26, "Twinaxial");
		m.insert(27, "EUI-64");
		m.insert(28, "HIPARP");
		m.insert(29, "IP and ARP over ISO 7816-3");
		m.insert(30, "ARPSec");
		m.insert(31, "IPsec tunnel");
		m.insert(32, "InfiniBand (TM)");
		m.insert(33, "CAI");
		m.insert(34, "Wiegand Interface");
		m.insert(35, "Pure IP");
		m.insert(36, "HW_EXP1");
		m.insert(37, "HFI");
		m.insert(38, "Unified Bus (UB)");
		m.insert(256, "HW_EXP2");
		m.insert(257, "AEthernet");
		m.insert(65535, "Reserved");
		m
	};
	pub static ref arp_oper_type_map: HashMap<u16, &'static str> = {
		let mut m = HashMap::new();
		m.insert(1, "REQUEST");
		m.insert(2, "REPLY");
		m.insert(3, "request Reverse");
		m.insert(4, "reply Reverse");
		m.insert(5, "DRARP-Request");
		m.insert(6, "DRARP-Reply");
		m.insert(7, "DRARP-Error");
		m.insert(8, "InARP-Request");
		m.insert(9, "InARP-Reply");
		m.insert(10, "ARP-NAK");
		m.insert(11, "MARS-Request");
		m.insert(12, "MARS-Multi");
		m.insert(13, "MARS-MServ");
		m.insert(14, "MARS-Join");
		m.insert(15, "MARS-Leave");
		m.insert(16, "MARS-NAK");
		m.insert(17, "MARS-Unserv");
		m.insert(18, "MARS-SJoin");
		m.insert(19, "MARS-SLeave");
		m.insert(20, "MARS-Grouplist-Request");
		m.insert(21, "MARS-Grouplist-Reply");
		m.insert(22, "MARS-Redirect-Map");
		m.insert(23, "MAPOS-UNARP");
		m.insert(24, "OP_EXP1");
		m.insert(25, "OP_EXP2");
		m
	};
}pub fn link_type_mapper(code:u16) -> String {
    (*link_type_map.get(&code).unwrap_or(&"unknown")).into()
  }
pub fn ip_protocol_type_mapper(code:u16) -> String {
    (*ip_protocol_type_map.get(&code).unwrap_or(&"unknown")).into()
  }
pub fn ssl_type_mapper(code:u16) -> String {
    (*ssl_type_map.get(&code).unwrap_or(&"unknown")).into()
  }
pub fn etype_mapper(code:u16) -> String {
    (*etype_map.get(&code).unwrap_or(&"unknown")).into()
  }
pub fn tcp_option_kind_mapper(code:u16) -> String {
    (*tcp_option_kind_map.get(&code).unwrap_or(&"unknown")).into()
  }
pub fn dns_class_mapper(code:u16) -> String {
    (*dns_class_map.get(&code).unwrap_or(&"unknown")).into()
  }
pub fn dns_type_mapper(code:u16) -> String {
    (*dns_type_map.get(&code).unwrap_or(&"unknown")).into()
  }
pub fn arp_hardware_type_mapper(code:u16) -> String {
    (*arp_hardware_type_map.get(&code).unwrap_or(&"unknown")).into()
  }
pub fn arp_oper_type_mapper(code:u16) -> String {
    (*arp_oper_type_map.get(&code).unwrap_or(&"unknown")).into()
  }