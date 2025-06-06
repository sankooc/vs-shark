The IEEE 802.11 link types (also known as "data link types" or "DLT_*" in packet capture tools like Wireshark/tcpdump) define how Wi-Fi frames are encapsulated in packet captures. Here are the most common 802.11-related link types:

### **Common 802.11 Link Types**
| Value | Name (DLT_) | Description |
|-------|------------|-------------|
| **105** | `IEEE802_11` | Standard 802.11 frame (data + management/control) |
| **127** | `IEEE802_11_RADIOTAP` | 802.11 with Radiotap header (metadata + PHY info) |
| **163** | `IEEE802_11_PRISM` | 802.11 with Prism monitoring header (legacy) |
| **164** | `IEEE802_11_AVS` | 802.11 with AVS header (alternative to Radiotap) |
| **283** | `IEEE802_11_TAP` | 802.11 with pseudo-header (macOS capture) |

---

### **Key Differences**
1. **`IEEE802_11` (105)**
   - Raw 802.11 frames without metadata.
   - Captures only MAC-layer data (no signal strength/timing info).

2. **`IEEE802_11_RADIOTAP` (127)**
   - Includes Radiotap header before each frame.
   - Contains PHY-layer metadata (RSSI, noise, channel, MCS rates, etc.).
   - Modern standard for Wi-Fi monitoring.

3. **`IEEE802_11_PRISM` (163)**
   - Legacy monitoring header (deprecated in favor of Radiotap).
   - Used in older tools like Kismet.

4. **`IEEE802_11_AVS` (164)**
   - AVS (Adaptive Wireless Vendor-Specific) header.
   - Alternative to Radiotap/Prism, used by some vendors.

5. **`IEEE802_11_TAP` (283)**
   - macOS-specific pseudo-header for Wi-Fi captures.

---

### **How to Check Link Type in Wireshark**
1. Open a PCAP file in Wireshark.
2. Go to **Statistics > Capture File Properties**.
3. Look for "Data link type" (e.g., "DLT_IEEE802_11_RADIOTAP").

---

### **Example Use Cases**
- **Network Analysis**: Use `RADIOTAP` to study signal quality/interference.
- **Security Research**: `IEEE802_11` for raw frame inspection (e.g., WPA handshakes).
- **Driver Debugging**: `AVS/PRISM` for vendor-specific diagnostics.

For modern Wi-Fi analysis, **`IEEE802_11_RADIOTAP` (127)** is the most useful due to its rich metadata.