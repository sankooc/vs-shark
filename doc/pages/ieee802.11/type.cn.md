### **常见的 802.11 链路类型（Link Type）**  
IEEE 802.11 链路类型（在 Wireshark/tcpdump 等抓包工具中称为 `DLT_*`）定义了 Wi-Fi 数据帧在抓包文件中的封装格式。以下是主要的 802.11 相关链路类型：  

| 数值 | 名称（DLT_） | 描述 |
|------|-------------|------|
| **105** | `IEEE802_11` | 标准 802.11 帧（数据 + 管理/控制帧） |
| **127** | `IEEE802_11_RADIOTAP` | 带 Radiotap 头的 802.11 帧（含物理层元数据） |
| **163** | `IEEE802_11_PRISM` | 带 PRISM 监控头的 802.11 帧（旧版） |
| **164** | `IEEE802_11_AVS` | 带 AVS 头的 802.11 帧（Radiotap 替代方案） |
| **283** | `IEEE802_11_TAP` | macOS 特有的 802.11 抓包格式 |

---

### **主要区别**
1. **`IEEE802_11` (105)**  
   - 纯 802.11 数据帧，不含额外元数据。  
   - 仅包含 MAC 层信息（无信号强度、时间戳等）。  

2. **`IEEE802_11_RADIOTAP` (127)**  
   - 每个帧前附加 Radiotap 头。  
   - 包含物理层信息（RSSI、噪声、信道、MCS 速率等）。  
   - 现代 Wi-Fi 监控的**标准格式**。  

3. **`IEEE802_11_PRISM` (163)**  
   - 旧版监控头（已逐渐被 Radiotap 取代）。  
   - 曾用于 Kismet 等老式工具。  

4. **`IEEE802_11_AVS` (164)**  
   - AVS（Adaptive Wireless Vendor-Specific）头。  
   - 部分厂商的私有格式（类似 Radiotap）。  

5. **`IEEE802_11_TAP` (283)**  
   - macOS 系统特有的伪头格式。  

---

### **如何在 Wireshark 中查看链路类型？**  
1. 打开抓包文件（`.pcap` 或 `.pcapng`）。  
2. 点击 **统计（Statistics） > 捕获文件属性（Capture File Properties）**。  
3. 查看 **"Data link type"**（如 `DLT_IEEE802_11_RADIOTAP`）。  

---

### **典型应用场景**  
- **网络分析**：使用 `RADIOTAP` 分析信号质量、干扰。  
- **安全研究**：用 `IEEE802_11` 解析原始帧（如 WPA 握手过程）。  
- **驱动调试**：`AVS/PRISM` 用于厂商特定的诊断。  

对于现代 Wi-Fi 分析，**`IEEE802_11_RADIOTAP` (127)** 是最常用的格式，因其携带丰富的物理层元数据。