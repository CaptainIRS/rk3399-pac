#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x2000],
    grf_usb3_perf_con0: GrfUsb3PerfCon0,
    grf_usb3_perf_con1: GrfUsb3PerfCon1,
    grf_usb3_perf_con2: GrfUsb3PerfCon2,
    grf_usb3_perf_rd_max_latency_num: GrfUsb3PerfRdMaxLatencyNum,
    grf_usb3_perf_rd_latency_samp_num: GrfUsb3PerfRdLatencySampNum,
    grf_usb3_perf_rd_latency_acc_num: GrfUsb3PerfRdLatencyAccNum,
    grf_usb3_perf_rd_axi_total_byte: GrfUsb3PerfRdAxiTotalByte,
    grf_usb3_perf_wr_axi_total_byte: GrfUsb3PerfWrAxiTotalByte,
    grf_usb3_perf_working_cnt: GrfUsb3PerfWorkingCnt,
    _reserved9: [u8; 0x040c],
    grf_usb3otg0_con0: GrfUsb3otg0Con0,
    grf_usb3otg0_con1: GrfUsb3otg0Con1,
    _reserved11: [u8; 0x08],
    grf_usb3otg1_con0: GrfUsb3otg1Con0,
    grf_usb3otg1_con1: GrfUsb3otg1Con1,
    _reserved13: [u8; 0x08],
    grf_usb3otg0_status_lat0: GrfUsb3otg0StatusLat0,
    grf_usb3otg0_status_lat1: GrfUsb3otg0StatusLat1,
    grf_usb3otg0_status_cb: GrfUsb3otg0StatusCb,
    _reserved16: [u8; 0x04],
    grf_usb3otg1_status_lat0: GrfUsb3otg1StatusLat0,
    grf_usb3otg1_status_lat1: GrfUsb3otg1StatusLat1,
    grf_usb3otg1_status_cb: GrfUsb3otg1StatusCb,
    _reserved19: [u8; 0x1b94],
    grf_pcie_perf_con0: GrfPciePerfCon0,
    grf_pcie_perf_con1: GrfPciePerfCon1,
    grf_pcie_perf_con2: GrfPciePerfCon2,
    grf_pcie_perf_rd_max_latency_num: GrfPciePerfRdMaxLatencyNum,
    grf_pcie_perf_rd_latency_samp_num: GrfPciePerfRdLatencySampNum,
    grf_pcie_perf_rd_latency_acc_num: GrfPciePerfRdLatencyAccNum,
    grf_pcie_perf_rd_axi_total_byte: GrfPciePerfRdAxiTotalByte,
    grf_pcie_perf_wr_axi_total_byte: GrfPciePerfWrAxiTotalByte,
    grf_pcie_perf_working_cnt: GrfPciePerfWorkingCnt,
    _reserved28: [u8; 0xdc],
    grf_usb20_host0_con0: GrfUsb20Host0Con0,
    grf_usb20_host0_con1: GrfUsb20Host0Con1,
    _reserved30: [u8; 0x08],
    grf_usb20_host1_con0: GrfUsb20Host1Con0,
    grf_usb20_host1_con1: GrfUsb20Host1Con1,
    _reserved32: [u8; 0x08],
    grf_hsic_con0: GrfHsicCon0,
    grf_hsic_con1: GrfHsicCon1,
    _reserved34: [u8; 0x18],
    grf_grf_usbhost0_status: GrfGrfUsbhost0Status,
    grf_grf_usbhost1_status: GrfGrfUsbhost1Status,
    grf_grf_hsic_status: GrfGrfHsicStatus,
    _reserved37: [u8; 0x0324],
    grf_hsicphy_con0: GrfHsicphyCon0,
    _reserved38: [u8; 0x0c],
    grf_usbphy0_ctrl0: GrfUsbphy0Ctrl0,
    grf_usbphy0_ctrl1: GrfUsbphy0Ctrl1,
    grf_usbphy0_ctrl2: GrfUsbphy0Ctrl2,
    grf_usbphy0_ctrl3: GrfUsbphy0Ctrl3,
    grf_usbphy0_ctrl4: GrfUsbphy0Ctrl4,
    grf_usbphy0_ctrl5: GrfUsbphy0Ctrl5,
    grf_usbphy0_ctrl6: GrfUsbphy0Ctrl6,
    grf_usbphy0_ctrl7: GrfUsbphy0Ctrl7,
    grf_usbphy0_ctrl8: GrfUsbphy0Ctrl8,
    grf_usbphy0_ctrl9: GrfUsbphy0Ctrl9,
    grf_usbphy0_ctrl10: GrfUsbphy0Ctrl10,
    grf_usbphy0_ctrl11: GrfUsbphy0Ctrl11,
    grf_usbphy0_ctrl12: GrfUsbphy0Ctrl12,
    grf_usbphy0_ctrl13: GrfUsbphy0Ctrl13,
    grf_usbphy0_ctrl14: GrfUsbphy0Ctrl14,
    grf_usbphy0_ctrl15: GrfUsbphy0Ctrl15,
    grf_usbphy0_ctrl16: GrfUsbphy0Ctrl16,
    grf_usbphy0_ctrl17: GrfUsbphy0Ctrl17,
    grf_usbphy0_ctrl18: GrfUsbphy0Ctrl18,
    grf_usbphy0_ctrl19: GrfUsbphy0Ctrl19,
    grf_usbphy0_ctrl20: GrfUsbphy0Ctrl20,
    grf_usbphy0_ctrl21: GrfUsbphy0Ctrl21,
    grf_usbphy0_ctrl22: GrfUsbphy0Ctrl22,
    grf_usbphy0_ctrl23: GrfUsbphy0Ctrl23,
    grf_usbphy0_ctrl24: GrfUsbphy0Ctrl24,
    grf_usbphy0_ctrl25: GrfUsbphy0Ctrl25,
    _reserved64: [u8; 0x18],
    grf_usbphy1_ctrl0: GrfUsbphy1Ctrl0,
    grf_usbphy1_ctrl1: GrfUsbphy1Ctrl1,
    grf_usbphy1_ctrl2: GrfUsbphy1Ctrl2,
    grf_usbphy1_ctrl3: GrfUsbphy1Ctrl3,
    grf_usbphy1_ctrl4: GrfUsbphy1Ctrl4,
    grf_usbphy1_ctrl5: GrfUsbphy1Ctrl5,
    grf_usbphy1_ctrl6: GrfUsbphy1Ctrl6,
    grf_usbphy1_ctrl7: GrfUsbphy1Ctrl7,
    grf_usbphy1_ctrl8: GrfUsbphy1Ctrl8,
    grf_usbphy1_ctrl9: GrfUsbphy1Ctrl9,
    grf_usbphy1_ctrl10: GrfUsbphy1Ctrl10,
    grf_usbphy1_ctrl11: GrfUsbphy1Ctrl11,
    grf_usbphy1_ctrl12: GrfUsbphy1Ctrl12,
    grf_usbphy1_ctrl13: GrfUsbphy1Ctrl13,
    grf_usbphy1_ctrl14: GrfUsbphy1Ctrl14,
    grf_usbphy1_ctrl15: GrfUsbphy1Ctrl15,
    grf_usbphy1_ctrl16: GrfUsbphy1Ctrl16,
    grf_usbphy1_ctrl17: GrfUsbphy1Ctrl17,
    grf_usbphy1_ctrl18: GrfUsbphy1Ctrl18,
    grf_usbphy1_ctrl19: GrfUsbphy1Ctrl19,
    grf_usbphy1_ctrl20: GrfUsbphy1Ctrl20,
    grf_usbphy1_ctrl21: GrfUsbphy1Ctrl21,
    grf_usbphy1_ctrl22: GrfUsbphy1Ctrl22,
    grf_usbphy1_ctrl23: GrfUsbphy1Ctrl23,
    grf_usbphy1_ctrl24: GrfUsbphy1Ctrl24,
    grf_usbphy1_ctrl25: GrfUsbphy1Ctrl25,
    _reserved90: [u8; 0x1a98],
    grf_hdcp22_perf_con0: GrfHdcp22PerfCon0,
    grf_hdcp22_perf_con1: GrfHdcp22PerfCon1,
    grf_hdcp22_perf_con2: GrfHdcp22PerfCon2,
    grf_hdcp22_perf_rd_max_latency_num: GrfHdcp22PerfRdMaxLatencyNum,
    grf_hdcp22_perf_rd_latency_samp_num: GrfHdcp22PerfRdLatencySampNum,
    grf_hdcp22_perf_rd_latency_acc_num: GrfHdcp22PerfRdLatencyAccNum,
    grf_hdcp22_perf_rd_axi_total_byte: GrfHdcp22PerfRdAxiTotalByte,
    grf_hdcp22_perf_wr_axi_total_byte: GrfHdcp22PerfWrAxiTotalByte,
    grf_hdcp22_perf_working_cnt: GrfHdcp22PerfWorkingCnt,
    _reserved99: [u8; 0x0200],
    grf_soc_con9: GrfSocCon9,
    _reserved100: [u8; 0x28],
    grf_soc_con20: GrfSocCon20,
    grf_soc_con21: GrfSocCon21,
    grf_soc_con22: GrfSocCon22,
    grf_soc_con23: GrfSocCon23,
    grf_soc_con24: GrfSocCon24,
    grf_soc_con25: GrfSocCon25,
    grf_soc_con26: GrfSocCon26,
    _reserved107: [u8; 0x1d94],
    grf_gpu_perf_con0: GrfGpuPerfCon0,
    grf_gpu_perf_con1: GrfGpuPerfCon1,
    grf_gpu_perf_con2: GrfGpuPerfCon2,
    grf_gpu_perf_rd_max_latency_num: GrfGpuPerfRdMaxLatencyNum,
    grf_gpu_perf_rd_latency_samp_num: GrfGpuPerfRdLatencySampNum,
    grf_gpu_perf_rd_latency_acc_num: GrfGpuPerfRdLatencyAccNum,
    grf_gpu_perf_rd_axi_total_byte: GrfGpuPerfRdAxiTotalByte,
    grf_gpu_perf_wr_axi_total_byte: GrfGpuPerfWrAxiTotalByte,
    grf_gpu_perf_working_cnt: GrfGpuPerfWorkingCnt,
    _reserved116: [u8; 0x1fdc],
    grf_cpu_con0: GrfCpuCon0,
    grf_cpu_con1: GrfCpuCon1,
    grf_cpu_con2: GrfCpuCon2,
    grf_cpu_con3: GrfCpuCon3,
    _reserved120: [u8; 0x70],
    grf_cpu_status0: GrfCpuStatus0,
    grf_cpu_status1: GrfCpuStatus1,
    grf_cpu_status2: GrfCpuStatus2,
    grf_cpu_status3: GrfCpuStatus3,
    grf_cpu_status4: GrfCpuStatus4,
    grf_cpu_status5: GrfCpuStatus5,
    _reserved126: [u8; 0x68],
    grf_a53_perf_con0: GrfA53PerfCon0,
    grf_a53_perf_con1: GrfA53PerfCon1,
    grf_a53_perf_con2: GrfA53PerfCon2,
    grf_a53_perf_con3: GrfA53PerfCon3,
    grf_a53_perf_rd_mon_st: GrfA53PerfRdMonSt,
    grf_a53_perf_rd_mon_end: GrfA53PerfRdMonEnd,
    grf_a53_perf_wr_mon_st: GrfA53PerfWrMonSt,
    grf_a53_perf_wr_mon_end: GrfA53PerfWrMonEnd,
    grf_a53_perf_rd_max_latency_num: GrfA53PerfRdMaxLatencyNum,
    grf_a53_perf_rd_latency_samp_num: GrfA53PerfRdLatencySampNum,
    grf_a53_perf_rd_latency_acc_num: GrfA53PerfRdLatencyAccNum,
    grf_a53_perf_rd_axi_total_byte: GrfA53PerfRdAxiTotalByte,
    grf_a53_perf_wr_axi_total_byte: GrfA53PerfWrAxiTotalByte,
    grf_a53_perf_working_cnt: GrfA53PerfWorkingCnt,
    grf_a53_perf_int_status: GrfA53PerfIntStatus,
    _reserved141: [u8; 0xc4],
    grf_a72_perf_con0: GrfA72PerfCon0,
    grf_a72_perf_con1: GrfA72PerfCon1,
    grf_a72_perf_con2: GrfA72PerfCon2,
    grf_a72_perf_con3: GrfA72PerfCon3,
    grf_a72_perf_rd_mon_st: GrfA72PerfRdMonSt,
    grf_a72_perf_rd_mon_end: GrfA72PerfRdMonEnd,
    grf_a72_perf_wr_mon_st: GrfA72PerfWrMonSt,
    grf_a72_perf_wr_mon_end: GrfA72PerfWrMonEnd,
    grf_a72_perf_rd_max_latency_num: GrfA72PerfRdMaxLatencyNum,
    grf_a72_perf_rd_latency_samp_num: GrfA72PerfRdLatencySampNum,
    grf_a72_perf_rd_latency_acc_num: GrfA72PerfRdLatencyAccNum,
    grf_a72_perf_rd_axi_total_byte: GrfA72PerfRdAxiTotalByte,
    grf_a72_perf_wr_axi_total_byte: GrfA72PerfWrAxiTotalByte,
    grf_a72_perf_working_cnt: GrfA72PerfWorkingCnt,
    grf_a72_perf_int_status: GrfA72PerfIntStatus,
    _reserved156: [u8; 0x1dc4],
    grf_gmac_perf_con0: GrfGmacPerfCon0,
    grf_gmac_perf_con1: GrfGmacPerfCon1,
    grf_gmac_perf_con2: GrfGmacPerfCon2,
    grf_gmac_perf_rd_max_latency_num: GrfGmacPerfRdMaxLatencyNum,
    grf_gmac_perf_rd_latency_samp_num: GrfGmacPerfRdLatencySampNum,
    grf_gmac_perf_rd_latency_acc_num: GrfGmacPerfRdLatencyAccNum,
    grf_gmac_perf_rd_axi_total_byte: GrfGmacPerfRdAxiTotalByte,
    grf_gmac_perf_wr_axi_total_byte: GrfGmacPerfWrAxiTotalByte,
    grf_gmac_perf_working_cnt: GrfGmacPerfWorkingCnt,
    _reserved165: [u8; 0x01f0],
    grf_soc_con5: GrfSocCon5,
    grf_soc_con6: GrfSocCon6,
    _reserved167: [u8; 0x1de4],
    grf_gpio2a_iomux: GrfGpio2aIomux,
    grf_gpio2b_iomux: GrfGpio2bIomux,
    grf_gpio2c_iomux: GrfGpio2cIomux,
    grf_gpio2d_iomux: GrfGpio2dIomux,
    grf_gpio3a_iomux: GrfGpio3aIomux,
    grf_gpio3b_iomux: GrfGpio3bIomux,
    grf_gpio3c_iomux: GrfGpio3cIomux,
    grf_gpio3d_iomux: GrfGpio3dIomux,
    grf_gpio4a_iomux: GrfGpio4aIomux,
    grf_gpio4b_iomux: GrfGpio4bIomux,
    grf_gpio4c_iomux: GrfGpio4cIomux,
    grf_gpio4d_iomux: GrfGpio4dIomux,
    _reserved179: [u8; 0x10],
    grf_gpio2a_p: GrfGpio2aP,
    grf_gpio2b_p: GrfGpio2bP,
    grf_gpio2c_p: GrfGpio2cP,
    grf_gpio2d_p: GrfGpio2dP,
    grf_gpio3a_p: GrfGpio3aP,
    grf_gpio3b_p: GrfGpio3bP,
    grf_gpio3c_p: GrfGpio3cP,
    grf_gpio3d_p: GrfGpio3dP,
    grf_gpio4a_p: GrfGpio4aP,
    grf_gpio4b_p: GrfGpio4bP,
    grf_gpio4c_p: GrfGpio4cP,
    grf_gpio4d_p: GrfGpio4dP,
    _reserved191: [u8; 0x10],
    grf_gpio2a_sr: GrfGpio2aSr,
    grf_gpio2b_sr: GrfGpio2bSr,
    grf_gpio2c_sr: GrfGpio2cSr,
    grf_gpio2d_sr: GrfGpio2dSr,
    _reserved195: [u8; 0x0c],
    grf_gpio3d_sr: GrfGpio3dSr,
    grf_gpio4a_sr: GrfGpio4aSr,
    grf_gpio4b_sr: GrfGpio4bSr,
    grf_gpio4c_sr: GrfGpio4cSr,
    grf_gpio4d_sr: GrfGpio4dSr,
    _reserved200: [u8; 0x10],
    grf_gpio2a_smt: GrfGpio2aSmt,
    grf_gpio2b_smt: GrfGpio2bSmt,
    grf_gpio2c_smt: GrfGpio2cSmt,
    grf_gpio2d_smt: GrfGpio2dSmt,
    grf_gpio3a_smt: GrfGpio3aSmt,
    grf_gpio3b_smt: GrfGpio3bSmt,
    grf_gpio3c_smt: GrfGpio3cSmt,
    grf_gpio3d_smt: GrfGpio3dSmt,
    grf_gpio4a_smt: GrfGpio4aSmt,
    grf_gpio4b_smt: GrfGpio4bSmt,
    grf_gpio4c_smt: GrfGpio4cSmt,
    grf_gpio4d_smt: GrfGpio4dSmt,
    _reserved212: [u8; 0x10],
    grf_gpio2a_e: GrfGpio2aE,
    grf_gpio2b_e: GrfGpio2bE,
    grf_gpio2c_e: GrfGpio2cE,
    grf_gpio2d_e: GrfGpio2dE,
    grf_gpio3a_e01: GrfGpio3aE01,
    grf_gpio3a_e2: GrfGpio3aE2,
    grf_gpio3b_e01: GrfGpio3bE01,
    grf_gpio3b_e2: GrfGpio3bE2,
    grf_gpio3c_e01: GrfGpio3cE01,
    grf_gpio3c_e2: GrfGpio3cE2,
    grf_gpio3d_e: GrfGpio3dE,
    grf_gpio4a_e: GrfGpio4aE,
    grf_gpio4b_e01: GrfGpio4bE01,
    grf_gpio4b_e2: GrfGpio4bE2,
    grf_gpio4c_e: GrfGpio4cE,
    grf_gpio4d_e: GrfGpio4dE,
    _reserved228: [u8; 0x48],
    grf_gpio2c_he: GrfGpio2cHe,
    grf_gpio2d_he: GrfGpio2dHe,
    _reserved230: [u8; 0x70],
    grf_soc_con0: GrfSocCon0,
    grf_soc_con1: GrfSocCon1,
    grf_soc_con2: GrfSocCon2,
    grf_soc_con3: GrfSocCon3,
    grf_soc_con4: GrfSocCon4,
    grf_soc_con_5_pcie: GrfSocCon5Pcie,
    _reserved236: [u8; 0x04],
    grf_soc_con7: GrfSocCon7,
    grf_soc_con8: GrfSocCon8,
    grf_soc_con_9_pcie: GrfSocCon9Pcie,
    _reserved239: [u8; 0x78],
    grf_soc_status0: GrfSocStatus0,
    grf_soc_status1: GrfSocStatus1,
    grf_soc_status2: GrfSocStatus2,
    grf_soc_status3: GrfSocStatus3,
    grf_soc_status4: GrfSocStatus4,
    grf_soc_status5: GrfSocStatus5,
    _reserved245: [u8; 0xc8],
    grf_ddrc0_con0: GrfDdrc0Con0,
    grf_ddrc0_con1: GrfDdrc0Con1,
    grf_ddrc1_con0: GrfDdrc1Con0,
    grf_ddrc1_con1: GrfDdrc1Con1,
    _reserved249: [u8; 0x30],
    grf_sig_detect_con0: GrfSigDetectCon0,
    _reserved250: [u8; 0x04],
    grf_sig_detect_con1: GrfSigDetectCon1,
    _reserved251: [u8; 0x04],
    grf_sig_detect_clr: GrfSigDetectClr,
    _reserved252: [u8; 0x0c],
    grf_sig_detect_status: GrfSigDetectStatus,
    _reserved253: [u8; 0x6c],
    grf_usb20_phy0_con0: GrfUsb20Phy0Con0,
    grf_usb20_phy0_con1: GrfUsb20Phy0Con1,
    grf_usb20_phy0_con2: GrfUsb20Phy0Con2,
    grf_usb20_phy0_con3: GrfUsb20Phy0Con3,
    grf_usb20_phy1_con0: GrfUsb20Phy1Con0,
    grf_usb20_phy1_con1: GrfUsb20Phy1Con1,
    grf_usb20_phy1_con2: GrfUsb20Phy1Con2,
    grf_usb20_phy1_con3: GrfUsb20Phy1Con3,
    _reserved261: [u8; 0x0110],
    grf_usb3phy0_con0: GrfUsb3phy0Con0,
    grf_usb3phy0_con1: GrfUsb3phy0Con1,
    grf_usb3phy0_con2: GrfUsb3phy0Con2,
    grf_usb3phy1_con0: GrfUsb3phy1Con0,
    grf_usb3phy1_con1: GrfUsb3phy1Con1,
    grf_usb3phy1_con2: GrfUsb3phy1Con2,
    _reserved267: [u8; 0x28],
    grf_usb3phy_status0: GrfUsb3phyStatus0,
    grf_usb3phy_status1: GrfUsb3phyStatus1,
    _reserved269: [u8; 0x38],
    grf_dll_con0: GrfDllCon0,
    grf_dll_con1: GrfDllCon1,
    grf_dll_con2: GrfDllCon2,
    grf_dll_con3: GrfDllCon3,
    grf_dll_con4: GrfDllCon4,
    grf_dll_con5: GrfDllCon5,
    _reserved275: [u8; 0x08],
    grf_dll_status0: GrfDllStatus0,
    grf_dll_status1: GrfDllStatus1,
    grf_dll_status2: GrfDllStatus2,
    grf_dll_status3: GrfDllStatus3,
    grf_dll_status4: GrfDllStatus4,
    _reserved280: [u8; 0x0c],
    grf_io_vsel: GrfIoVsel,
    grf_saradc_testbit: GrfSaradcTestbit,
    grf_tsadc_testbit_l: GrfTsadcTestbitL,
    grf_tsadc_testbit_h: GrfTsadcTestbitH,
    _reserved284: [u8; 0x01b0],
    grf_chip_id_addr: GrfChipIdAddr,
    _reserved285: [u8; 0x7c],
    grf_fast_boot_addr: GrfFastBootAddr,
    _reserved286: [u8; 0x077c],
    grf_emmccore_con0: GrfEmmccoreCon0,
    grf_emmccore_con1: GrfEmmccoreCon1,
    grf_emmccore_con2: GrfEmmccoreCon2,
    grf_emmccore_con3: GrfEmmccoreCon3,
    grf_emmccore_con4: GrfEmmccoreCon4,
    grf_emmccore_con5: GrfEmmccoreCon5,
    grf_emmccore_con6: GrfEmmccoreCon6,
    grf_emmccore_con7: GrfEmmccoreCon7,
    grf_emmccore_con8: GrfEmmccoreCon8,
    grf_emmccore_con9: GrfEmmccoreCon9,
    grf_emmccore_con10: GrfEmmccoreCon10,
    grf_emmccore_con11: GrfEmmccoreCon11,
    _reserved298: [u8; 0x10],
    grf_emmccore_status0: GrfEmmccoreStatus0,
    grf_emmccore_status1: GrfEmmccoreStatus1,
    grf_emmccore_status2: GrfEmmccoreStatus2,
    grf_emmccore_status3: GrfEmmccoreStatus3,
    _reserved302: [u8; 0x0730],
    grf_emmcphy_con0: GrfEmmcphyCon0,
    grf_emmcphy_con1: GrfEmmcphyCon1,
    grf_emmcphy_con2: GrfEmmcphyCon2,
    grf_emmcphy_con3: GrfEmmcphyCon3,
    grf_emmcphy_con4: GrfEmmcphyCon4,
    grf_emmcphy_con5: GrfEmmcphyCon5,
    grf_emmcphy_con6: GrfEmmcphyCon6,
    _reserved309: [u8; 0x04],
    grf_emmcphy_status: GrfEmmcphyStatus,
}
impl RegisterBlock {
    #[doc = "0x2000 - usb3 performance monitor control register"]
    #[inline(always)]
    pub const fn grf_usb3_perf_con0(&self) -> &GrfUsb3PerfCon0 {
        &self.grf_usb3_perf_con0
    }
    #[doc = "0x2004 - usb3 performance monitor control register"]
    #[inline(always)]
    pub const fn grf_usb3_perf_con1(&self) -> &GrfUsb3PerfCon1 {
        &self.grf_usb3_perf_con1
    }
    #[doc = "0x2008 - usb3 performance monitor control register"]
    #[inline(always)]
    pub const fn grf_usb3_perf_con2(&self) -> &GrfUsb3PerfCon2 {
        &self.grf_usb3_perf_con2
    }
    #[doc = "0x200c - usb3 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_usb3_perf_rd_max_latency_num(&self) -> &GrfUsb3PerfRdMaxLatencyNum {
        &self.grf_usb3_perf_rd_max_latency_num
    }
    #[doc = "0x2010 - usb3 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_usb3_perf_rd_latency_samp_num(&self) -> &GrfUsb3PerfRdLatencySampNum {
        &self.grf_usb3_perf_rd_latency_samp_num
    }
    #[doc = "0x2014 - usb3 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_usb3_perf_rd_latency_acc_num(&self) -> &GrfUsb3PerfRdLatencyAccNum {
        &self.grf_usb3_perf_rd_latency_acc_num
    }
    #[doc = "0x2018 - usb3 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_usb3_perf_rd_axi_total_byte(&self) -> &GrfUsb3PerfRdAxiTotalByte {
        &self.grf_usb3_perf_rd_axi_total_byte
    }
    #[doc = "0x201c - usb3 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_usb3_perf_wr_axi_total_byte(&self) -> &GrfUsb3PerfWrAxiTotalByte {
        &self.grf_usb3_perf_wr_axi_total_byte
    }
    #[doc = "0x2020 - usb3 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_usb3_perf_working_cnt(&self) -> &GrfUsb3PerfWorkingCnt {
        &self.grf_usb3_perf_working_cnt
    }
    #[doc = "0x2430 - USB3 OTG0 GRF Register0"]
    #[inline(always)]
    pub const fn grf_usb3otg0_con0(&self) -> &GrfUsb3otg0Con0 {
        &self.grf_usb3otg0_con0
    }
    #[doc = "0x2434 - USB3 OTG0 GRF Register1"]
    #[inline(always)]
    pub const fn grf_usb3otg0_con1(&self) -> &GrfUsb3otg0Con1 {
        &self.grf_usb3otg0_con1
    }
    #[doc = "0x2440 - USB3 OTG1 GRF Register0"]
    #[inline(always)]
    pub const fn grf_usb3otg1_con0(&self) -> &GrfUsb3otg1Con0 {
        &self.grf_usb3otg1_con0
    }
    #[doc = "0x2444 - USB3 OTG1 GRF Register1"]
    #[inline(always)]
    pub const fn grf_usb3otg1_con1(&self) -> &GrfUsb3otg1Con1 {
        &self.grf_usb3otg1_con1
    }
    #[doc = "0x2450 - USB3 OTG0 status register"]
    #[inline(always)]
    pub const fn grf_usb3otg0_status_lat0(&self) -> &GrfUsb3otg0StatusLat0 {
        &self.grf_usb3otg0_status_lat0
    }
    #[doc = "0x2454 - USB3 OTG1 status register"]
    #[inline(always)]
    pub const fn grf_usb3otg0_status_lat1(&self) -> &GrfUsb3otg0StatusLat1 {
        &self.grf_usb3otg0_status_lat1
    }
    #[doc = "0x2458 - USB3 OTG0 status register"]
    #[inline(always)]
    pub const fn grf_usb3otg0_status_cb(&self) -> &GrfUsb3otg0StatusCb {
        &self.grf_usb3otg0_status_cb
    }
    #[doc = "0x2460 - USB3 OTG1 status register"]
    #[inline(always)]
    pub const fn grf_usb3otg1_status_lat0(&self) -> &GrfUsb3otg1StatusLat0 {
        &self.grf_usb3otg1_status_lat0
    }
    #[doc = "0x2464 - USB3 OTG1 status register"]
    #[inline(always)]
    pub const fn grf_usb3otg1_status_lat1(&self) -> &GrfUsb3otg1StatusLat1 {
        &self.grf_usb3otg1_status_lat1
    }
    #[doc = "0x2468 - USB3 OTG1 status register"]
    #[inline(always)]
    pub const fn grf_usb3otg1_status_cb(&self) -> &GrfUsb3otg1StatusCb {
        &self.grf_usb3otg1_status_cb
    }
    #[doc = "0x4000 - pcie performance monitor control register"]
    #[inline(always)]
    pub const fn grf_pcie_perf_con0(&self) -> &GrfPciePerfCon0 {
        &self.grf_pcie_perf_con0
    }
    #[doc = "0x4004 - pcie performance monitor control register"]
    #[inline(always)]
    pub const fn grf_pcie_perf_con1(&self) -> &GrfPciePerfCon1 {
        &self.grf_pcie_perf_con1
    }
    #[doc = "0x4008 - pcie performance monitor control register"]
    #[inline(always)]
    pub const fn grf_pcie_perf_con2(&self) -> &GrfPciePerfCon2 {
        &self.grf_pcie_perf_con2
    }
    #[doc = "0x400c - pcieperformance monitor status register"]
    #[inline(always)]
    pub const fn grf_pcie_perf_rd_max_latency_num(&self) -> &GrfPciePerfRdMaxLatencyNum {
        &self.grf_pcie_perf_rd_max_latency_num
    }
    #[doc = "0x4010 - pcie performance monitor status register"]
    #[inline(always)]
    pub const fn grf_pcie_perf_rd_latency_samp_num(&self) -> &GrfPciePerfRdLatencySampNum {
        &self.grf_pcie_perf_rd_latency_samp_num
    }
    #[doc = "0x4014 - pcie performance monitor status register"]
    #[inline(always)]
    pub const fn grf_pcie_perf_rd_latency_acc_num(&self) -> &GrfPciePerfRdLatencyAccNum {
        &self.grf_pcie_perf_rd_latency_acc_num
    }
    #[doc = "0x4018 - pcie performance monitor status register"]
    #[inline(always)]
    pub const fn grf_pcie_perf_rd_axi_total_byte(&self) -> &GrfPciePerfRdAxiTotalByte {
        &self.grf_pcie_perf_rd_axi_total_byte
    }
    #[doc = "0x401c - pcie performance monitor status register"]
    #[inline(always)]
    pub const fn grf_pcie_perf_wr_axi_total_byte(&self) -> &GrfPciePerfWrAxiTotalByte {
        &self.grf_pcie_perf_wr_axi_total_byte
    }
    #[doc = "0x4020 - pcie performance monitor status register"]
    #[inline(always)]
    pub const fn grf_pcie_perf_working_cnt(&self) -> &GrfPciePerfWorkingCnt {
        &self.grf_pcie_perf_working_cnt
    }
    #[doc = "0x4100 - USB20 Host0 GRF register0"]
    #[inline(always)]
    pub const fn grf_usb20_host0_con0(&self) -> &GrfUsb20Host0Con0 {
        &self.grf_usb20_host0_con0
    }
    #[doc = "0x4104 - USB20 Host0 GRF register1"]
    #[inline(always)]
    pub const fn grf_usb20_host0_con1(&self) -> &GrfUsb20Host0Con1 {
        &self.grf_usb20_host0_con1
    }
    #[doc = "0x4110 - USB20 Host1 GRF register0"]
    #[inline(always)]
    pub const fn grf_usb20_host1_con0(&self) -> &GrfUsb20Host1Con0 {
        &self.grf_usb20_host1_con0
    }
    #[doc = "0x4114 - USB20 Host1 GRF register1"]
    #[inline(always)]
    pub const fn grf_usb20_host1_con1(&self) -> &GrfUsb20Host1Con1 {
        &self.grf_usb20_host1_con1
    }
    #[doc = "0x4120 - HSIC controller GRF register 0"]
    #[inline(always)]
    pub const fn grf_hsic_con0(&self) -> &GrfHsicCon0 {
        &self.grf_hsic_con0
    }
    #[doc = "0x4124 - HSIC controller GRF register1"]
    #[inline(always)]
    pub const fn grf_hsic_con1(&self) -> &GrfHsicCon1 {
        &self.grf_hsic_con1
    }
    #[doc = "0x4140 - usb host0 controller status register"]
    #[inline(always)]
    pub const fn grf_grf_usbhost0_status(&self) -> &GrfGrfUsbhost0Status {
        &self.grf_grf_usbhost0_status
    }
    #[doc = "0x4144 - usb host1 controller status register"]
    #[inline(always)]
    pub const fn grf_grf_usbhost1_status(&self) -> &GrfGrfUsbhost1Status {
        &self.grf_grf_usbhost1_status
    }
    #[doc = "0x4148 - hsic controller status register"]
    #[inline(always)]
    pub const fn grf_grf_hsic_status(&self) -> &GrfGrfHsicStatus {
        &self.grf_grf_hsic_status
    }
    #[doc = "0x4470 - HSICPHY GRF control register"]
    #[inline(always)]
    pub const fn grf_hsicphy_con0(&self) -> &GrfHsicphyCon0 {
        &self.grf_hsicphy_con0
    }
    #[doc = "0x4480 - usbphy0_ctrl0"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl0(&self) -> &GrfUsbphy0Ctrl0 {
        &self.grf_usbphy0_ctrl0
    }
    #[doc = "0x4484 - usbphy0_ctrl1"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl1(&self) -> &GrfUsbphy0Ctrl1 {
        &self.grf_usbphy0_ctrl1
    }
    #[doc = "0x4488 - usbphy0_ctrl2"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl2(&self) -> &GrfUsbphy0Ctrl2 {
        &self.grf_usbphy0_ctrl2
    }
    #[doc = "0x448c - usbphy0_ctrl3"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl3(&self) -> &GrfUsbphy0Ctrl3 {
        &self.grf_usbphy0_ctrl3
    }
    #[doc = "0x4490 - usbphy0_ctrl4"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl4(&self) -> &GrfUsbphy0Ctrl4 {
        &self.grf_usbphy0_ctrl4
    }
    #[doc = "0x4494 - usbphy0_ctrl5"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl5(&self) -> &GrfUsbphy0Ctrl5 {
        &self.grf_usbphy0_ctrl5
    }
    #[doc = "0x4498 - usbphy0_ctrl6"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl6(&self) -> &GrfUsbphy0Ctrl6 {
        &self.grf_usbphy0_ctrl6
    }
    #[doc = "0x449c - usbphy0_ctrl7"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl7(&self) -> &GrfUsbphy0Ctrl7 {
        &self.grf_usbphy0_ctrl7
    }
    #[doc = "0x44a0 - usbphy0_ctrl8"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl8(&self) -> &GrfUsbphy0Ctrl8 {
        &self.grf_usbphy0_ctrl8
    }
    #[doc = "0x44a4 - usbphy0_ctrl9"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl9(&self) -> &GrfUsbphy0Ctrl9 {
        &self.grf_usbphy0_ctrl9
    }
    #[doc = "0x44a8 - usbphy0_ctrl10"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl10(&self) -> &GrfUsbphy0Ctrl10 {
        &self.grf_usbphy0_ctrl10
    }
    #[doc = "0x44ac - usbphy0_ctrl11"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl11(&self) -> &GrfUsbphy0Ctrl11 {
        &self.grf_usbphy0_ctrl11
    }
    #[doc = "0x44b0 - usbphy0_ctrl12"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl12(&self) -> &GrfUsbphy0Ctrl12 {
        &self.grf_usbphy0_ctrl12
    }
    #[doc = "0x44b4 - usbphy0_ctrl13"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl13(&self) -> &GrfUsbphy0Ctrl13 {
        &self.grf_usbphy0_ctrl13
    }
    #[doc = "0x44b8 - usbphy0_ctrl14"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl14(&self) -> &GrfUsbphy0Ctrl14 {
        &self.grf_usbphy0_ctrl14
    }
    #[doc = "0x44bc - usbphy0_ctrl15"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl15(&self) -> &GrfUsbphy0Ctrl15 {
        &self.grf_usbphy0_ctrl15
    }
    #[doc = "0x44c0 - usbphy0_ctrl16"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl16(&self) -> &GrfUsbphy0Ctrl16 {
        &self.grf_usbphy0_ctrl16
    }
    #[doc = "0x44c4 - usbphy0_ctrl17"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl17(&self) -> &GrfUsbphy0Ctrl17 {
        &self.grf_usbphy0_ctrl17
    }
    #[doc = "0x44c8 - usbphy0_ctrl18"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl18(&self) -> &GrfUsbphy0Ctrl18 {
        &self.grf_usbphy0_ctrl18
    }
    #[doc = "0x44cc - usbphy0_ctrl19"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl19(&self) -> &GrfUsbphy0Ctrl19 {
        &self.grf_usbphy0_ctrl19
    }
    #[doc = "0x44d0 - usbphy0_ctrl20"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl20(&self) -> &GrfUsbphy0Ctrl20 {
        &self.grf_usbphy0_ctrl20
    }
    #[doc = "0x44d4 - usbphy0_ctrl21"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl21(&self) -> &GrfUsbphy0Ctrl21 {
        &self.grf_usbphy0_ctrl21
    }
    #[doc = "0x44d8 - usbphy0_ctrl22"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl22(&self) -> &GrfUsbphy0Ctrl22 {
        &self.grf_usbphy0_ctrl22
    }
    #[doc = "0x44dc - usbphy0_ctrl23"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl23(&self) -> &GrfUsbphy0Ctrl23 {
        &self.grf_usbphy0_ctrl23
    }
    #[doc = "0x44e0 - usbphy0_ctrl24"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl24(&self) -> &GrfUsbphy0Ctrl24 {
        &self.grf_usbphy0_ctrl24
    }
    #[doc = "0x44e4 - usbphy0_ctrl25"]
    #[inline(always)]
    pub const fn grf_usbphy0_ctrl25(&self) -> &GrfUsbphy0Ctrl25 {
        &self.grf_usbphy0_ctrl25
    }
    #[doc = "0x4500 - usbphy1_ctrl0"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl0(&self) -> &GrfUsbphy1Ctrl0 {
        &self.grf_usbphy1_ctrl0
    }
    #[doc = "0x4504 - usbphy1_ctrl1"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl1(&self) -> &GrfUsbphy1Ctrl1 {
        &self.grf_usbphy1_ctrl1
    }
    #[doc = "0x4508 - usbphy1_ctrl2"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl2(&self) -> &GrfUsbphy1Ctrl2 {
        &self.grf_usbphy1_ctrl2
    }
    #[doc = "0x450c - usbphy1_ctrl3"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl3(&self) -> &GrfUsbphy1Ctrl3 {
        &self.grf_usbphy1_ctrl3
    }
    #[doc = "0x4510 - usbphy1_ctrl4"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl4(&self) -> &GrfUsbphy1Ctrl4 {
        &self.grf_usbphy1_ctrl4
    }
    #[doc = "0x4514 - usbphy1_ctrl5"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl5(&self) -> &GrfUsbphy1Ctrl5 {
        &self.grf_usbphy1_ctrl5
    }
    #[doc = "0x4518 - usbphy1_ctrl6"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl6(&self) -> &GrfUsbphy1Ctrl6 {
        &self.grf_usbphy1_ctrl6
    }
    #[doc = "0x451c - usbphy1_ctrl7"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl7(&self) -> &GrfUsbphy1Ctrl7 {
        &self.grf_usbphy1_ctrl7
    }
    #[doc = "0x4520 - usbphy1_ctrl8"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl8(&self) -> &GrfUsbphy1Ctrl8 {
        &self.grf_usbphy1_ctrl8
    }
    #[doc = "0x4524 - usbphy1_ctrl9"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl9(&self) -> &GrfUsbphy1Ctrl9 {
        &self.grf_usbphy1_ctrl9
    }
    #[doc = "0x4528 - usbphy1_ctrl10"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl10(&self) -> &GrfUsbphy1Ctrl10 {
        &self.grf_usbphy1_ctrl10
    }
    #[doc = "0x452c - usbphy1_ctrl11"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl11(&self) -> &GrfUsbphy1Ctrl11 {
        &self.grf_usbphy1_ctrl11
    }
    #[doc = "0x4530 - usbphy1_ctrl12"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl12(&self) -> &GrfUsbphy1Ctrl12 {
        &self.grf_usbphy1_ctrl12
    }
    #[doc = "0x4534 - usbphy1_ctrl13"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl13(&self) -> &GrfUsbphy1Ctrl13 {
        &self.grf_usbphy1_ctrl13
    }
    #[doc = "0x4538 - usbphy1_ctrl14"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl14(&self) -> &GrfUsbphy1Ctrl14 {
        &self.grf_usbphy1_ctrl14
    }
    #[doc = "0x453c - usbphy1_ctrl15"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl15(&self) -> &GrfUsbphy1Ctrl15 {
        &self.grf_usbphy1_ctrl15
    }
    #[doc = "0x4540 - usbphy1_ctrl16"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl16(&self) -> &GrfUsbphy1Ctrl16 {
        &self.grf_usbphy1_ctrl16
    }
    #[doc = "0x4544 - usbphy1_ctrl17"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl17(&self) -> &GrfUsbphy1Ctrl17 {
        &self.grf_usbphy1_ctrl17
    }
    #[doc = "0x4548 - usbphy1_ctrl18"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl18(&self) -> &GrfUsbphy1Ctrl18 {
        &self.grf_usbphy1_ctrl18
    }
    #[doc = "0x454c - usbphy1_ctrl19"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl19(&self) -> &GrfUsbphy1Ctrl19 {
        &self.grf_usbphy1_ctrl19
    }
    #[doc = "0x4550 - usbphy1_ctrl20"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl20(&self) -> &GrfUsbphy1Ctrl20 {
        &self.grf_usbphy1_ctrl20
    }
    #[doc = "0x4554 - usbphy1_ctrl21"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl21(&self) -> &GrfUsbphy1Ctrl21 {
        &self.grf_usbphy1_ctrl21
    }
    #[doc = "0x4558 - usbphy1_ctrl22"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl22(&self) -> &GrfUsbphy1Ctrl22 {
        &self.grf_usbphy1_ctrl22
    }
    #[doc = "0x455c - usbphy1_ctrl23"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl23(&self) -> &GrfUsbphy1Ctrl23 {
        &self.grf_usbphy1_ctrl23
    }
    #[doc = "0x4560 - usbphy1_ctrl24"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl24(&self) -> &GrfUsbphy1Ctrl24 {
        &self.grf_usbphy1_ctrl24
    }
    #[doc = "0x4564 - usbphy1_ctrl25"]
    #[inline(always)]
    pub const fn grf_usbphy1_ctrl25(&self) -> &GrfUsbphy1Ctrl25 {
        &self.grf_usbphy1_ctrl25
    }
    #[doc = "0x6000 - hdcp performance monitor control register"]
    #[inline(always)]
    pub const fn grf_hdcp22_perf_con0(&self) -> &GrfHdcp22PerfCon0 {
        &self.grf_hdcp22_perf_con0
    }
    #[doc = "0x6004 - hdcp performance monitor control register"]
    #[inline(always)]
    pub const fn grf_hdcp22_perf_con1(&self) -> &GrfHdcp22PerfCon1 {
        &self.grf_hdcp22_perf_con1
    }
    #[doc = "0x6008 - hdcp performance monitor control register"]
    #[inline(always)]
    pub const fn grf_hdcp22_perf_con2(&self) -> &GrfHdcp22PerfCon2 {
        &self.grf_hdcp22_perf_con2
    }
    #[doc = "0x600c - hdcp performance monitor status register"]
    #[inline(always)]
    pub const fn grf_hdcp22_perf_rd_max_latency_num(&self) -> &GrfHdcp22PerfRdMaxLatencyNum {
        &self.grf_hdcp22_perf_rd_max_latency_num
    }
    #[doc = "0x6010 - hdcp performance monitor status register"]
    #[inline(always)]
    pub const fn grf_hdcp22_perf_rd_latency_samp_num(&self) -> &GrfHdcp22PerfRdLatencySampNum {
        &self.grf_hdcp22_perf_rd_latency_samp_num
    }
    #[doc = "0x6014 - hdcp performance monitor status register"]
    #[inline(always)]
    pub const fn grf_hdcp22_perf_rd_latency_acc_num(&self) -> &GrfHdcp22PerfRdLatencyAccNum {
        &self.grf_hdcp22_perf_rd_latency_acc_num
    }
    #[doc = "0x6018 - hdcp performance monitor status register"]
    #[inline(always)]
    pub const fn grf_hdcp22_perf_rd_axi_total_byte(&self) -> &GrfHdcp22PerfRdAxiTotalByte {
        &self.grf_hdcp22_perf_rd_axi_total_byte
    }
    #[doc = "0x601c - hdcp performance monitor status register"]
    #[inline(always)]
    pub const fn grf_hdcp22_perf_wr_axi_total_byte(&self) -> &GrfHdcp22PerfWrAxiTotalByte {
        &self.grf_hdcp22_perf_wr_axi_total_byte
    }
    #[doc = "0x6020 - hdcp performance monitor status register"]
    #[inline(always)]
    pub const fn grf_hdcp22_perf_working_cnt(&self) -> &GrfHdcp22PerfWorkingCnt {
        &self.grf_hdcp22_perf_working_cnt
    }
    #[doc = "0x6224 - SoC control register 9"]
    #[inline(always)]
    pub const fn grf_soc_con9(&self) -> &GrfSocCon9 {
        &self.grf_soc_con9
    }
    #[doc = "0x6250 - SoC control register 20"]
    #[inline(always)]
    pub const fn grf_soc_con20(&self) -> &GrfSocCon20 {
        &self.grf_soc_con20
    }
    #[doc = "0x6254 - SoC control register 21"]
    #[inline(always)]
    pub const fn grf_soc_con21(&self) -> &GrfSocCon21 {
        &self.grf_soc_con21
    }
    #[doc = "0x6258 - SoC control register 22"]
    #[inline(always)]
    pub const fn grf_soc_con22(&self) -> &GrfSocCon22 {
        &self.grf_soc_con22
    }
    #[doc = "0x625c - SoC control register 23"]
    #[inline(always)]
    pub const fn grf_soc_con23(&self) -> &GrfSocCon23 {
        &self.grf_soc_con23
    }
    #[doc = "0x6260 - SoC control register 24"]
    #[inline(always)]
    pub const fn grf_soc_con24(&self) -> &GrfSocCon24 {
        &self.grf_soc_con24
    }
    #[doc = "0x6264 - SoC control register 25"]
    #[inline(always)]
    pub const fn grf_soc_con25(&self) -> &GrfSocCon25 {
        &self.grf_soc_con25
    }
    #[doc = "0x6268 - SoC control register 26"]
    #[inline(always)]
    pub const fn grf_soc_con26(&self) -> &GrfSocCon26 {
        &self.grf_soc_con26
    }
    #[doc = "0x8000 - gpu performance monitor control register"]
    #[inline(always)]
    pub const fn grf_gpu_perf_con0(&self) -> &GrfGpuPerfCon0 {
        &self.grf_gpu_perf_con0
    }
    #[doc = "0x8004 - gpu performance monitor control register"]
    #[inline(always)]
    pub const fn grf_gpu_perf_con1(&self) -> &GrfGpuPerfCon1 {
        &self.grf_gpu_perf_con1
    }
    #[doc = "0x8008 - gpu performance monitor control register"]
    #[inline(always)]
    pub const fn grf_gpu_perf_con2(&self) -> &GrfGpuPerfCon2 {
        &self.grf_gpu_perf_con2
    }
    #[doc = "0x800c - gpu performance monitor status register"]
    #[inline(always)]
    pub const fn grf_gpu_perf_rd_max_latency_num(&self) -> &GrfGpuPerfRdMaxLatencyNum {
        &self.grf_gpu_perf_rd_max_latency_num
    }
    #[doc = "0x8010 - gpu performance monitor status register"]
    #[inline(always)]
    pub const fn grf_gpu_perf_rd_latency_samp_num(&self) -> &GrfGpuPerfRdLatencySampNum {
        &self.grf_gpu_perf_rd_latency_samp_num
    }
    #[doc = "0x8014 - gpu performance monitor status register"]
    #[inline(always)]
    pub const fn grf_gpu_perf_rd_latency_acc_num(&self) -> &GrfGpuPerfRdLatencyAccNum {
        &self.grf_gpu_perf_rd_latency_acc_num
    }
    #[doc = "0x8018 - gpu performance monitor status register"]
    #[inline(always)]
    pub const fn grf_gpu_perf_rd_axi_total_byte(&self) -> &GrfGpuPerfRdAxiTotalByte {
        &self.grf_gpu_perf_rd_axi_total_byte
    }
    #[doc = "0x801c - gpu performance monitor status register"]
    #[inline(always)]
    pub const fn grf_gpu_perf_wr_axi_total_byte(&self) -> &GrfGpuPerfWrAxiTotalByte {
        &self.grf_gpu_perf_wr_axi_total_byte
    }
    #[doc = "0x8020 - gpu performance monitor status register"]
    #[inline(always)]
    pub const fn grf_gpu_perf_working_cnt(&self) -> &GrfGpuPerfWorkingCnt {
        &self.grf_gpu_perf_working_cnt
    }
    #[doc = "0xa000 - cpu control register 0"]
    #[inline(always)]
    pub const fn grf_cpu_con0(&self) -> &GrfCpuCon0 {
        &self.grf_cpu_con0
    }
    #[doc = "0xa004 - cpu control register 1"]
    #[inline(always)]
    pub const fn grf_cpu_con1(&self) -> &GrfCpuCon1 {
        &self.grf_cpu_con1
    }
    #[doc = "0xa008 - cpu control register 2"]
    #[inline(always)]
    pub const fn grf_cpu_con2(&self) -> &GrfCpuCon2 {
        &self.grf_cpu_con2
    }
    #[doc = "0xa00c - cpu control register 3"]
    #[inline(always)]
    pub const fn grf_cpu_con3(&self) -> &GrfCpuCon3 {
        &self.grf_cpu_con3
    }
    #[doc = "0xa080 - cpu status register 0"]
    #[inline(always)]
    pub const fn grf_cpu_status0(&self) -> &GrfCpuStatus0 {
        &self.grf_cpu_status0
    }
    #[doc = "0xa084 - cpu status register 1"]
    #[inline(always)]
    pub const fn grf_cpu_status1(&self) -> &GrfCpuStatus1 {
        &self.grf_cpu_status1
    }
    #[doc = "0xa088 - cpu status register 2"]
    #[inline(always)]
    pub const fn grf_cpu_status2(&self) -> &GrfCpuStatus2 {
        &self.grf_cpu_status2
    }
    #[doc = "0xa08c - cpu status register 3"]
    #[inline(always)]
    pub const fn grf_cpu_status3(&self) -> &GrfCpuStatus3 {
        &self.grf_cpu_status3
    }
    #[doc = "0xa090 - cpu status register 4"]
    #[inline(always)]
    pub const fn grf_cpu_status4(&self) -> &GrfCpuStatus4 {
        &self.grf_cpu_status4
    }
    #[doc = "0xa094 - cpu status register 5"]
    #[inline(always)]
    pub const fn grf_cpu_status5(&self) -> &GrfCpuStatus5 {
        &self.grf_cpu_status5
    }
    #[doc = "0xa100 - a53 performance monitor control register"]
    #[inline(always)]
    pub const fn grf_a53_perf_con0(&self) -> &GrfA53PerfCon0 {
        &self.grf_a53_perf_con0
    }
    #[doc = "0xa104 - a53 performance monitor control register"]
    #[inline(always)]
    pub const fn grf_a53_perf_con1(&self) -> &GrfA53PerfCon1 {
        &self.grf_a53_perf_con1
    }
    #[doc = "0xa108 - a53 performance monitor control register"]
    #[inline(always)]
    pub const fn grf_a53_perf_con2(&self) -> &GrfA53PerfCon2 {
        &self.grf_a53_perf_con2
    }
    #[doc = "0xa10c - a53 performance monitor control register"]
    #[inline(always)]
    pub const fn grf_a53_perf_con3(&self) -> &GrfA53PerfCon3 {
        &self.grf_a53_perf_con3
    }
    #[doc = "0xa110 - performance monitor read start address"]
    #[inline(always)]
    pub const fn grf_a53_perf_rd_mon_st(&self) -> &GrfA53PerfRdMonSt {
        &self.grf_a53_perf_rd_mon_st
    }
    #[doc = "0xa114 - performance monitor end address"]
    #[inline(always)]
    pub const fn grf_a53_perf_rd_mon_end(&self) -> &GrfA53PerfRdMonEnd {
        &self.grf_a53_perf_rd_mon_end
    }
    #[doc = "0xa118 - performance write monitor start address"]
    #[inline(always)]
    pub const fn grf_a53_perf_wr_mon_st(&self) -> &GrfA53PerfWrMonSt {
        &self.grf_a53_perf_wr_mon_st
    }
    #[doc = "0xa11c - performance monitor write end address"]
    #[inline(always)]
    pub const fn grf_a53_perf_wr_mon_end(&self) -> &GrfA53PerfWrMonEnd {
        &self.grf_a53_perf_wr_mon_end
    }
    #[doc = "0xa120 - a53 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_a53_perf_rd_max_latency_num(&self) -> &GrfA53PerfRdMaxLatencyNum {
        &self.grf_a53_perf_rd_max_latency_num
    }
    #[doc = "0xa124 - a53 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_a53_perf_rd_latency_samp_num(&self) -> &GrfA53PerfRdLatencySampNum {
        &self.grf_a53_perf_rd_latency_samp_num
    }
    #[doc = "0xa128 - a53 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_a53_perf_rd_latency_acc_num(&self) -> &GrfA53PerfRdLatencyAccNum {
        &self.grf_a53_perf_rd_latency_acc_num
    }
    #[doc = "0xa12c - a53 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_a53_perf_rd_axi_total_byte(&self) -> &GrfA53PerfRdAxiTotalByte {
        &self.grf_a53_perf_rd_axi_total_byte
    }
    #[doc = "0xa130 - a53 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_a53_perf_wr_axi_total_byte(&self) -> &GrfA53PerfWrAxiTotalByte {
        &self.grf_a53_perf_wr_axi_total_byte
    }
    #[doc = "0xa134 - a53 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_a53_perf_working_cnt(&self) -> &GrfA53PerfWorkingCnt {
        &self.grf_a53_perf_working_cnt
    }
    #[doc = "0xa138 - a53 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_a53_perf_int_status(&self) -> &GrfA53PerfIntStatus {
        &self.grf_a53_perf_int_status
    }
    #[doc = "0xa200 - a72 performance monitor control register"]
    #[inline(always)]
    pub const fn grf_a72_perf_con0(&self) -> &GrfA72PerfCon0 {
        &self.grf_a72_perf_con0
    }
    #[doc = "0xa204 - a72 performance monitor control register"]
    #[inline(always)]
    pub const fn grf_a72_perf_con1(&self) -> &GrfA72PerfCon1 {
        &self.grf_a72_perf_con1
    }
    #[doc = "0xa208 - a72 performance monitor control register"]
    #[inline(always)]
    pub const fn grf_a72_perf_con2(&self) -> &GrfA72PerfCon2 {
        &self.grf_a72_perf_con2
    }
    #[doc = "0xa20c - a72 performance monitor control register"]
    #[inline(always)]
    pub const fn grf_a72_perf_con3(&self) -> &GrfA72PerfCon3 {
        &self.grf_a72_perf_con3
    }
    #[doc = "0xa210 - performance monitor read start address"]
    #[inline(always)]
    pub const fn grf_a72_perf_rd_mon_st(&self) -> &GrfA72PerfRdMonSt {
        &self.grf_a72_perf_rd_mon_st
    }
    #[doc = "0xa214 - performance monitor end address"]
    #[inline(always)]
    pub const fn grf_a72_perf_rd_mon_end(&self) -> &GrfA72PerfRdMonEnd {
        &self.grf_a72_perf_rd_mon_end
    }
    #[doc = "0xa218 - performance write monitor start address"]
    #[inline(always)]
    pub const fn grf_a72_perf_wr_mon_st(&self) -> &GrfA72PerfWrMonSt {
        &self.grf_a72_perf_wr_mon_st
    }
    #[doc = "0xa21c - performance monitor write end address"]
    #[inline(always)]
    pub const fn grf_a72_perf_wr_mon_end(&self) -> &GrfA72PerfWrMonEnd {
        &self.grf_a72_perf_wr_mon_end
    }
    #[doc = "0xa220 - a72 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_a72_perf_rd_max_latency_num(&self) -> &GrfA72PerfRdMaxLatencyNum {
        &self.grf_a72_perf_rd_max_latency_num
    }
    #[doc = "0xa224 - a72 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_a72_perf_rd_latency_samp_num(&self) -> &GrfA72PerfRdLatencySampNum {
        &self.grf_a72_perf_rd_latency_samp_num
    }
    #[doc = "0xa228 - a72 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_a72_perf_rd_latency_acc_num(&self) -> &GrfA72PerfRdLatencyAccNum {
        &self.grf_a72_perf_rd_latency_acc_num
    }
    #[doc = "0xa22c - a72 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_a72_perf_rd_axi_total_byte(&self) -> &GrfA72PerfRdAxiTotalByte {
        &self.grf_a72_perf_rd_axi_total_byte
    }
    #[doc = "0xa230 - a72 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_a72_perf_wr_axi_total_byte(&self) -> &GrfA72PerfWrAxiTotalByte {
        &self.grf_a72_perf_wr_axi_total_byte
    }
    #[doc = "0xa234 - a72 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_a72_perf_working_cnt(&self) -> &GrfA72PerfWorkingCnt {
        &self.grf_a72_perf_working_cnt
    }
    #[doc = "0xa238 - a72 performance monitor status register"]
    #[inline(always)]
    pub const fn grf_a72_perf_int_status(&self) -> &GrfA72PerfIntStatus {
        &self.grf_a72_perf_int_status
    }
    #[doc = "0xc000 - gmac performance monitor control register"]
    #[inline(always)]
    pub const fn grf_gmac_perf_con0(&self) -> &GrfGmacPerfCon0 {
        &self.grf_gmac_perf_con0
    }
    #[doc = "0xc004 - gmac performance monitor control register"]
    #[inline(always)]
    pub const fn grf_gmac_perf_con1(&self) -> &GrfGmacPerfCon1 {
        &self.grf_gmac_perf_con1
    }
    #[doc = "0xc008 - gmac performance monitor control register"]
    #[inline(always)]
    pub const fn grf_gmac_perf_con2(&self) -> &GrfGmacPerfCon2 {
        &self.grf_gmac_perf_con2
    }
    #[doc = "0xc00c - gmac performance monitor status register"]
    #[inline(always)]
    pub const fn grf_gmac_perf_rd_max_latency_num(&self) -> &GrfGmacPerfRdMaxLatencyNum {
        &self.grf_gmac_perf_rd_max_latency_num
    }
    #[doc = "0xc010 - gmac performance monitor status register"]
    #[inline(always)]
    pub const fn grf_gmac_perf_rd_latency_samp_num(&self) -> &GrfGmacPerfRdLatencySampNum {
        &self.grf_gmac_perf_rd_latency_samp_num
    }
    #[doc = "0xc014 - gmac performance monitor status register"]
    #[inline(always)]
    pub const fn grf_gmac_perf_rd_latency_acc_num(&self) -> &GrfGmacPerfRdLatencyAccNum {
        &self.grf_gmac_perf_rd_latency_acc_num
    }
    #[doc = "0xc018 - gmac performance monitor status register"]
    #[inline(always)]
    pub const fn grf_gmac_perf_rd_axi_total_byte(&self) -> &GrfGmacPerfRdAxiTotalByte {
        &self.grf_gmac_perf_rd_axi_total_byte
    }
    #[doc = "0xc01c - gmac performance monitor status register"]
    #[inline(always)]
    pub const fn grf_gmac_perf_wr_axi_total_byte(&self) -> &GrfGmacPerfWrAxiTotalByte {
        &self.grf_gmac_perf_wr_axi_total_byte
    }
    #[doc = "0xc020 - gmac performance monitor status register"]
    #[inline(always)]
    pub const fn grf_gmac_perf_working_cnt(&self) -> &GrfGmacPerfWorkingCnt {
        &self.grf_gmac_perf_working_cnt
    }
    #[doc = "0xc214 - SoC control register 5"]
    #[inline(always)]
    pub const fn grf_soc_con5(&self) -> &GrfSocCon5 {
        &self.grf_soc_con5
    }
    #[doc = "0xc218 - SoC control register 6"]
    #[inline(always)]
    pub const fn grf_soc_con6(&self) -> &GrfSocCon6 {
        &self.grf_soc_con6
    }
    #[doc = "0xe000 - GPIO2A iomux control"]
    #[inline(always)]
    pub const fn grf_gpio2a_iomux(&self) -> &GrfGpio2aIomux {
        &self.grf_gpio2a_iomux
    }
    #[doc = "0xe004 - GPIO2B iomux control"]
    #[inline(always)]
    pub const fn grf_gpio2b_iomux(&self) -> &GrfGpio2bIomux {
        &self.grf_gpio2b_iomux
    }
    #[doc = "0xe008 - GPIO2C iomux control"]
    #[inline(always)]
    pub const fn grf_gpio2c_iomux(&self) -> &GrfGpio2cIomux {
        &self.grf_gpio2c_iomux
    }
    #[doc = "0xe00c - GPIO2D iomux control"]
    #[inline(always)]
    pub const fn grf_gpio2d_iomux(&self) -> &GrfGpio2dIomux {
        &self.grf_gpio2d_iomux
    }
    #[doc = "0xe010 - GPIO3A iomux control"]
    #[inline(always)]
    pub const fn grf_gpio3a_iomux(&self) -> &GrfGpio3aIomux {
        &self.grf_gpio3a_iomux
    }
    #[doc = "0xe014 - GPIO3B iomux control"]
    #[inline(always)]
    pub const fn grf_gpio3b_iomux(&self) -> &GrfGpio3bIomux {
        &self.grf_gpio3b_iomux
    }
    #[doc = "0xe018 - GPIO3C iomux control"]
    #[inline(always)]
    pub const fn grf_gpio3c_iomux(&self) -> &GrfGpio3cIomux {
        &self.grf_gpio3c_iomux
    }
    #[doc = "0xe01c - GPIO3D iomux control"]
    #[inline(always)]
    pub const fn grf_gpio3d_iomux(&self) -> &GrfGpio3dIomux {
        &self.grf_gpio3d_iomux
    }
    #[doc = "0xe020 - GPIO4A iomux control"]
    #[inline(always)]
    pub const fn grf_gpio4a_iomux(&self) -> &GrfGpio4aIomux {
        &self.grf_gpio4a_iomux
    }
    #[doc = "0xe024 - GPIO4B iomux control"]
    #[inline(always)]
    pub const fn grf_gpio4b_iomux(&self) -> &GrfGpio4bIomux {
        &self.grf_gpio4b_iomux
    }
    #[doc = "0xe028 - GPIO4C iomux control"]
    #[inline(always)]
    pub const fn grf_gpio4c_iomux(&self) -> &GrfGpio4cIomux {
        &self.grf_gpio4c_iomux
    }
    #[doc = "0xe02c - GPIO4D iomux control"]
    #[inline(always)]
    pub const fn grf_gpio4d_iomux(&self) -> &GrfGpio4dIomux {
        &self.grf_gpio4d_iomux
    }
    #[doc = "0xe040 - GPIO2A PU/PD control"]
    #[inline(always)]
    pub const fn grf_gpio2a_p(&self) -> &GrfGpio2aP {
        &self.grf_gpio2a_p
    }
    #[doc = "0xe044 - GPIO2B PU/PD control"]
    #[inline(always)]
    pub const fn grf_gpio2b_p(&self) -> &GrfGpio2bP {
        &self.grf_gpio2b_p
    }
    #[doc = "0xe048 - GPIO2C PU/PD control"]
    #[inline(always)]
    pub const fn grf_gpio2c_p(&self) -> &GrfGpio2cP {
        &self.grf_gpio2c_p
    }
    #[doc = "0xe04c - GPIO2D PU/PD control"]
    #[inline(always)]
    pub const fn grf_gpio2d_p(&self) -> &GrfGpio2dP {
        &self.grf_gpio2d_p
    }
    #[doc = "0xe050 - GPIO3A PU/PD control"]
    #[inline(always)]
    pub const fn grf_gpio3a_p(&self) -> &GrfGpio3aP {
        &self.grf_gpio3a_p
    }
    #[doc = "0xe054 - GPIO3B PU/PD control"]
    #[inline(always)]
    pub const fn grf_gpio3b_p(&self) -> &GrfGpio3bP {
        &self.grf_gpio3b_p
    }
    #[doc = "0xe058 - GPIO3C PU/PD control"]
    #[inline(always)]
    pub const fn grf_gpio3c_p(&self) -> &GrfGpio3cP {
        &self.grf_gpio3c_p
    }
    #[doc = "0xe05c - GPIO3D PU/PD control"]
    #[inline(always)]
    pub const fn grf_gpio3d_p(&self) -> &GrfGpio3dP {
        &self.grf_gpio3d_p
    }
    #[doc = "0xe060 - GPIO4A PU/PD control"]
    #[inline(always)]
    pub const fn grf_gpio4a_p(&self) -> &GrfGpio4aP {
        &self.grf_gpio4a_p
    }
    #[doc = "0xe064 - GPIO4B PU/PD control"]
    #[inline(always)]
    pub const fn grf_gpio4b_p(&self) -> &GrfGpio4bP {
        &self.grf_gpio4b_p
    }
    #[doc = "0xe068 - GPIO4C PU/PD control"]
    #[inline(always)]
    pub const fn grf_gpio4c_p(&self) -> &GrfGpio4cP {
        &self.grf_gpio4c_p
    }
    #[doc = "0xe06c - GPIO4D PU/PD control"]
    #[inline(always)]
    pub const fn grf_gpio4d_p(&self) -> &GrfGpio4dP {
        &self.grf_gpio4d_p
    }
    #[doc = "0xe080 - GPIO2A slew rate control"]
    #[inline(always)]
    pub const fn grf_gpio2a_sr(&self) -> &GrfGpio2aSr {
        &self.grf_gpio2a_sr
    }
    #[doc = "0xe084 - GPIO2B slew rate control"]
    #[inline(always)]
    pub const fn grf_gpio2b_sr(&self) -> &GrfGpio2bSr {
        &self.grf_gpio2b_sr
    }
    #[doc = "0xe088 - GPIO2C slew rate control"]
    #[inline(always)]
    pub const fn grf_gpio2c_sr(&self) -> &GrfGpio2cSr {
        &self.grf_gpio2c_sr
    }
    #[doc = "0xe08c - GPIO2D slew rate control"]
    #[inline(always)]
    pub const fn grf_gpio2d_sr(&self) -> &GrfGpio2dSr {
        &self.grf_gpio2d_sr
    }
    #[doc = "0xe09c - GPIO3D slew rate control"]
    #[inline(always)]
    pub const fn grf_gpio3d_sr(&self) -> &GrfGpio3dSr {
        &self.grf_gpio3d_sr
    }
    #[doc = "0xe0a0 - GPIO4A slew rate control"]
    #[inline(always)]
    pub const fn grf_gpio4a_sr(&self) -> &GrfGpio4aSr {
        &self.grf_gpio4a_sr
    }
    #[doc = "0xe0a4 - GPIO4B slew rate control"]
    #[inline(always)]
    pub const fn grf_gpio4b_sr(&self) -> &GrfGpio4bSr {
        &self.grf_gpio4b_sr
    }
    #[doc = "0xe0a8 - GPIO4C slew rate control"]
    #[inline(always)]
    pub const fn grf_gpio4c_sr(&self) -> &GrfGpio4cSr {
        &self.grf_gpio4c_sr
    }
    #[doc = "0xe0ac - GPIO4D slew rate control"]
    #[inline(always)]
    pub const fn grf_gpio4d_sr(&self) -> &GrfGpio4dSr {
        &self.grf_gpio4d_sr
    }
    #[doc = "0xe0c0 - GPIO2A smitter control register"]
    #[inline(always)]
    pub const fn grf_gpio2a_smt(&self) -> &GrfGpio2aSmt {
        &self.grf_gpio2a_smt
    }
    #[doc = "0xe0c4 - GPIO2B smitter control register"]
    #[inline(always)]
    pub const fn grf_gpio2b_smt(&self) -> &GrfGpio2bSmt {
        &self.grf_gpio2b_smt
    }
    #[doc = "0xe0c8 - GPIO2C smitter control register"]
    #[inline(always)]
    pub const fn grf_gpio2c_smt(&self) -> &GrfGpio2cSmt {
        &self.grf_gpio2c_smt
    }
    #[doc = "0xe0cc - GPIO2D smitter control register"]
    #[inline(always)]
    pub const fn grf_gpio2d_smt(&self) -> &GrfGpio2dSmt {
        &self.grf_gpio2d_smt
    }
    #[doc = "0xe0d0 - GPIO3A smitter control register"]
    #[inline(always)]
    pub const fn grf_gpio3a_smt(&self) -> &GrfGpio3aSmt {
        &self.grf_gpio3a_smt
    }
    #[doc = "0xe0d4 - GPIO3B smitter control register"]
    #[inline(always)]
    pub const fn grf_gpio3b_smt(&self) -> &GrfGpio3bSmt {
        &self.grf_gpio3b_smt
    }
    #[doc = "0xe0d8 - GPIO3C smitter control register"]
    #[inline(always)]
    pub const fn grf_gpio3c_smt(&self) -> &GrfGpio3cSmt {
        &self.grf_gpio3c_smt
    }
    #[doc = "0xe0dc - GPIO3D smitter control register"]
    #[inline(always)]
    pub const fn grf_gpio3d_smt(&self) -> &GrfGpio3dSmt {
        &self.grf_gpio3d_smt
    }
    #[doc = "0xe0e0 - GPIO4A smitter control register"]
    #[inline(always)]
    pub const fn grf_gpio4a_smt(&self) -> &GrfGpio4aSmt {
        &self.grf_gpio4a_smt
    }
    #[doc = "0xe0e4 - GPIO4B smitter control register"]
    #[inline(always)]
    pub const fn grf_gpio4b_smt(&self) -> &GrfGpio4bSmt {
        &self.grf_gpio4b_smt
    }
    #[doc = "0xe0e8 - GPIO4C smitter control register"]
    #[inline(always)]
    pub const fn grf_gpio4c_smt(&self) -> &GrfGpio4cSmt {
        &self.grf_gpio4c_smt
    }
    #[doc = "0xe0ec - GPIO4D smitter control register"]
    #[inline(always)]
    pub const fn grf_gpio4d_smt(&self) -> &GrfGpio4dSmt {
        &self.grf_gpio4d_smt
    }
    #[doc = "0xe100 - GPIO2A drive strength control"]
    #[inline(always)]
    pub const fn grf_gpio2a_e(&self) -> &GrfGpio2aE {
        &self.grf_gpio2a_e
    }
    #[doc = "0xe104 - GPIO2B drive strength control"]
    #[inline(always)]
    pub const fn grf_gpio2b_e(&self) -> &GrfGpio2bE {
        &self.grf_gpio2b_e
    }
    #[doc = "0xe108 - GPIO2C drive strength control"]
    #[inline(always)]
    pub const fn grf_gpio2c_e(&self) -> &GrfGpio2cE {
        &self.grf_gpio2c_e
    }
    #[doc = "0xe10c - GPIO2D drive strength control"]
    #[inline(always)]
    pub const fn grf_gpio2d_e(&self) -> &GrfGpio2dE {
        &self.grf_gpio2d_e
    }
    #[doc = "0xe110 - GPIO3A drive strength control"]
    #[inline(always)]
    pub const fn grf_gpio3a_e01(&self) -> &GrfGpio3aE01 {
        &self.grf_gpio3a_e01
    }
    #[doc = "0xe114 - GPIO3B drive strength control"]
    #[inline(always)]
    pub const fn grf_gpio3a_e2(&self) -> &GrfGpio3aE2 {
        &self.grf_gpio3a_e2
    }
    #[doc = "0xe118 - GPIO3B drive strength control"]
    #[inline(always)]
    pub const fn grf_gpio3b_e01(&self) -> &GrfGpio3bE01 {
        &self.grf_gpio3b_e01
    }
    #[doc = "0xe11c - GPIO3B drive strength control"]
    #[inline(always)]
    pub const fn grf_gpio3b_e2(&self) -> &GrfGpio3bE2 {
        &self.grf_gpio3b_e2
    }
    #[doc = "0xe120 - GPIO3C drive strength control"]
    #[inline(always)]
    pub const fn grf_gpio3c_e01(&self) -> &GrfGpio3cE01 {
        &self.grf_gpio3c_e01
    }
    #[doc = "0xe124 - GPIO3C drive strength control"]
    #[inline(always)]
    pub const fn grf_gpio3c_e2(&self) -> &GrfGpio3cE2 {
        &self.grf_gpio3c_e2
    }
    #[doc = "0xe128 - GPIO3D drive strength control"]
    #[inline(always)]
    pub const fn grf_gpio3d_e(&self) -> &GrfGpio3dE {
        &self.grf_gpio3d_e
    }
    #[doc = "0xe12c - GPIO4A drive strength control"]
    #[inline(always)]
    pub const fn grf_gpio4a_e(&self) -> &GrfGpio4aE {
        &self.grf_gpio4a_e
    }
    #[doc = "0xe130 - GPIO4B drive strength control"]
    #[inline(always)]
    pub const fn grf_gpio4b_e01(&self) -> &GrfGpio4bE01 {
        &self.grf_gpio4b_e01
    }
    #[doc = "0xe134 - GPIO4B drive strength control"]
    #[inline(always)]
    pub const fn grf_gpio4b_e2(&self) -> &GrfGpio4bE2 {
        &self.grf_gpio4b_e2
    }
    #[doc = "0xe138 - GPIO4C drive strength control"]
    #[inline(always)]
    pub const fn grf_gpio4c_e(&self) -> &GrfGpio4cE {
        &self.grf_gpio4c_e
    }
    #[doc = "0xe13c - GPIO4D drive strength control"]
    #[inline(always)]
    pub const fn grf_gpio4d_e(&self) -> &GrfGpio4dE {
        &self.grf_gpio4d_e
    }
    #[doc = "0xe188 - GPIO2C HE control"]
    #[inline(always)]
    pub const fn grf_gpio2c_he(&self) -> &GrfGpio2cHe {
        &self.grf_gpio2c_he
    }
    #[doc = "0xe18c - GPIO2D HE control"]
    #[inline(always)]
    pub const fn grf_gpio2d_he(&self) -> &GrfGpio2dHe {
        &self.grf_gpio2d_he
    }
    #[doc = "0xe200 - SoC control register 0"]
    #[inline(always)]
    pub const fn grf_soc_con0(&self) -> &GrfSocCon0 {
        &self.grf_soc_con0
    }
    #[doc = "0xe204 - SoC control register 2"]
    #[inline(always)]
    pub const fn grf_soc_con1(&self) -> &GrfSocCon1 {
        &self.grf_soc_con1
    }
    #[doc = "0xe208 - SoC control register 1"]
    #[inline(always)]
    pub const fn grf_soc_con2(&self) -> &GrfSocCon2 {
        &self.grf_soc_con2
    }
    #[doc = "0xe20c - SoC control register 3"]
    #[inline(always)]
    pub const fn grf_soc_con3(&self) -> &GrfSocCon3 {
        &self.grf_soc_con3
    }
    #[doc = "0xe210 - SoC control register 4"]
    #[inline(always)]
    pub const fn grf_soc_con4(&self) -> &GrfSocCon4 {
        &self.grf_soc_con4
    }
    #[doc = "0xe214 - SoC control register 5"]
    #[inline(always)]
    pub const fn grf_soc_con_5_pcie(&self) -> &GrfSocCon5Pcie {
        &self.grf_soc_con_5_pcie
    }
    #[doc = "0xe21c - SoC control register 7"]
    #[inline(always)]
    pub const fn grf_soc_con7(&self) -> &GrfSocCon7 {
        &self.grf_soc_con7
    }
    #[doc = "0xe220 - SoC control register 8"]
    #[inline(always)]
    pub const fn grf_soc_con8(&self) -> &GrfSocCon8 {
        &self.grf_soc_con8
    }
    #[doc = "0xe224 - SoC control register 9 for PCIE"]
    #[inline(always)]
    pub const fn grf_soc_con_9_pcie(&self) -> &GrfSocCon9Pcie {
        &self.grf_soc_con_9_pcie
    }
    #[doc = "0xe2a0 - SOC status register 0"]
    #[inline(always)]
    pub const fn grf_soc_status0(&self) -> &GrfSocStatus0 {
        &self.grf_soc_status0
    }
    #[doc = "0xe2a4 - SOC status register 1"]
    #[inline(always)]
    pub const fn grf_soc_status1(&self) -> &GrfSocStatus1 {
        &self.grf_soc_status1
    }
    #[doc = "0xe2a8 - SOC status register 2"]
    #[inline(always)]
    pub const fn grf_soc_status2(&self) -> &GrfSocStatus2 {
        &self.grf_soc_status2
    }
    #[doc = "0xe2ac - SOC status register 3"]
    #[inline(always)]
    pub const fn grf_soc_status3(&self) -> &GrfSocStatus3 {
        &self.grf_soc_status3
    }
    #[doc = "0xe2b0 - SOC status register 4"]
    #[inline(always)]
    pub const fn grf_soc_status4(&self) -> &GrfSocStatus4 {
        &self.grf_soc_status4
    }
    #[doc = "0xe2b4 - SOC status register 5"]
    #[inline(always)]
    pub const fn grf_soc_status5(&self) -> &GrfSocStatus5 {
        &self.grf_soc_status5
    }
    #[doc = "0xe380 - ddrc0 control register 0"]
    #[inline(always)]
    pub const fn grf_ddrc0_con0(&self) -> &GrfDdrc0Con0 {
        &self.grf_ddrc0_con0
    }
    #[doc = "0xe384 - ddrc0 control register 1"]
    #[inline(always)]
    pub const fn grf_ddrc0_con1(&self) -> &GrfDdrc0Con1 {
        &self.grf_ddrc0_con1
    }
    #[doc = "0xe388 - ddrc1 control register 0"]
    #[inline(always)]
    pub const fn grf_ddrc1_con0(&self) -> &GrfDdrc1Con0 {
        &self.grf_ddrc1_con0
    }
    #[doc = "0xe38c - ddrc1 control register 1"]
    #[inline(always)]
    pub const fn grf_ddrc1_con1(&self) -> &GrfDdrc1Con1 {
        &self.grf_ddrc1_con1
    }
    #[doc = "0xe3c0 - Singal detect control register0"]
    #[inline(always)]
    pub const fn grf_sig_detect_con0(&self) -> &GrfSigDetectCon0 {
        &self.grf_sig_detect_con0
    }
    #[doc = "0xe3c8 - Singal detect control register1"]
    #[inline(always)]
    pub const fn grf_sig_detect_con1(&self) -> &GrfSigDetectCon1 {
        &self.grf_sig_detect_con1
    }
    #[doc = "0xe3d0 - Signal detect status clear register"]
    #[inline(always)]
    pub const fn grf_sig_detect_clr(&self) -> &GrfSigDetectClr {
        &self.grf_sig_detect_clr
    }
    #[doc = "0xe3e0 - Signal detect status register"]
    #[inline(always)]
    pub const fn grf_sig_detect_status(&self) -> &GrfSigDetectStatus {
        &self.grf_sig_detect_status
    }
    #[doc = "0xe450 - USB20 PHY0 GRF Register 0"]
    #[inline(always)]
    pub const fn grf_usb20_phy0_con0(&self) -> &GrfUsb20Phy0Con0 {
        &self.grf_usb20_phy0_con0
    }
    #[doc = "0xe454 - USB20 PHY0 GRF Register 1"]
    #[inline(always)]
    pub const fn grf_usb20_phy0_con1(&self) -> &GrfUsb20Phy0Con1 {
        &self.grf_usb20_phy0_con1
    }
    #[doc = "0xe458 - USB20 PHY0 GRF Register 2"]
    #[inline(always)]
    pub const fn grf_usb20_phy0_con2(&self) -> &GrfUsb20Phy0Con2 {
        &self.grf_usb20_phy0_con2
    }
    #[doc = "0xe45c - USB20 PHY0 GRF Register 3"]
    #[inline(always)]
    pub const fn grf_usb20_phy0_con3(&self) -> &GrfUsb20Phy0Con3 {
        &self.grf_usb20_phy0_con3
    }
    #[doc = "0xe460 - USB20 PHY1 GRF Register 0"]
    #[inline(always)]
    pub const fn grf_usb20_phy1_con0(&self) -> &GrfUsb20Phy1Con0 {
        &self.grf_usb20_phy1_con0
    }
    #[doc = "0xe464 - USB20 PHY1GRF Register 1"]
    #[inline(always)]
    pub const fn grf_usb20_phy1_con1(&self) -> &GrfUsb20Phy1Con1 {
        &self.grf_usb20_phy1_con1
    }
    #[doc = "0xe468 - USB20 PHY1 GRF Register 2"]
    #[inline(always)]
    pub const fn grf_usb20_phy1_con2(&self) -> &GrfUsb20Phy1Con2 {
        &self.grf_usb20_phy1_con2
    }
    #[doc = "0xe46c - USB20 PHY1 GRF Register 3"]
    #[inline(always)]
    pub const fn grf_usb20_phy1_con3(&self) -> &GrfUsb20Phy1Con3 {
        &self.grf_usb20_phy1_con3
    }
    #[doc = "0xe580 - TypeC PHY/TCPD PHY/TCPC Control register0"]
    #[inline(always)]
    pub const fn grf_usb3phy0_con0(&self) -> &GrfUsb3phy0Con0 {
        &self.grf_usb3phy0_con0
    }
    #[doc = "0xe584 - TypeC PHY/TCPD PHY/TCPC Control register1"]
    #[inline(always)]
    pub const fn grf_usb3phy0_con1(&self) -> &GrfUsb3phy0Con1 {
        &self.grf_usb3phy0_con1
    }
    #[doc = "0xe588 - TypeC PHY/TCPD PHY/TCPC Control register2"]
    #[inline(always)]
    pub const fn grf_usb3phy0_con2(&self) -> &GrfUsb3phy0Con2 {
        &self.grf_usb3phy0_con2
    }
    #[doc = "0xe58c - TypeC PHY/TCPD PHY/TCPC Control register0"]
    #[inline(always)]
    pub const fn grf_usb3phy1_con0(&self) -> &GrfUsb3phy1Con0 {
        &self.grf_usb3phy1_con0
    }
    #[doc = "0xe590 - TypeC PHY/TCPD PHY/TCPC Control register1"]
    #[inline(always)]
    pub const fn grf_usb3phy1_con1(&self) -> &GrfUsb3phy1Con1 {
        &self.grf_usb3phy1_con1
    }
    #[doc = "0xe594 - TypeC PHY/TCPD PHY/TCPC Control register2"]
    #[inline(always)]
    pub const fn grf_usb3phy1_con2(&self) -> &GrfUsb3phy1Con2 {
        &self.grf_usb3phy1_con2
    }
    #[doc = "0xe5c0 - USB3PHY_STATUS0"]
    #[inline(always)]
    pub const fn grf_usb3phy_status0(&self) -> &GrfUsb3phyStatus0 {
        &self.grf_usb3phy_status0
    }
    #[doc = "0xe5c4 - USB3PHY_STATUS1"]
    #[inline(always)]
    pub const fn grf_usb3phy_status1(&self) -> &GrfUsb3phyStatus1 {
        &self.grf_usb3phy_status1
    }
    #[doc = "0xe600 - pvtm control register"]
    #[inline(always)]
    pub const fn grf_dll_con0(&self) -> &GrfDllCon0 {
        &self.grf_dll_con0
    }
    #[doc = "0xe604 - pvtm control register"]
    #[inline(always)]
    pub const fn grf_dll_con1(&self) -> &GrfDllCon1 {
        &self.grf_dll_con1
    }
    #[doc = "0xe608 - pvtm control register"]
    #[inline(always)]
    pub const fn grf_dll_con2(&self) -> &GrfDllCon2 {
        &self.grf_dll_con2
    }
    #[doc = "0xe60c - pvtm control register"]
    #[inline(always)]
    pub const fn grf_dll_con3(&self) -> &GrfDllCon3 {
        &self.grf_dll_con3
    }
    #[doc = "0xe610 - pvtm control register"]
    #[inline(always)]
    pub const fn grf_dll_con4(&self) -> &GrfDllCon4 {
        &self.grf_dll_con4
    }
    #[doc = "0xe614 - pvtm control register"]
    #[inline(always)]
    pub const fn grf_dll_con5(&self) -> &GrfDllCon5 {
        &self.grf_dll_con5
    }
    #[doc = "0xe620 - pvtm status register"]
    #[inline(always)]
    pub const fn grf_dll_status0(&self) -> &GrfDllStatus0 {
        &self.grf_dll_status0
    }
    #[doc = "0xe624 - pvtm status register"]
    #[inline(always)]
    pub const fn grf_dll_status1(&self) -> &GrfDllStatus1 {
        &self.grf_dll_status1
    }
    #[doc = "0xe628 - pvtm status register"]
    #[inline(always)]
    pub const fn grf_dll_status2(&self) -> &GrfDllStatus2 {
        &self.grf_dll_status2
    }
    #[doc = "0xe62c - pvtm status register"]
    #[inline(always)]
    pub const fn grf_dll_status3(&self) -> &GrfDllStatus3 {
        &self.grf_dll_status3
    }
    #[doc = "0xe630 - pvtm status register"]
    #[inline(always)]
    pub const fn grf_dll_status4(&self) -> &GrfDllStatus4 {
        &self.grf_dll_status4
    }
    #[doc = "0xe640 - "]
    #[inline(always)]
    pub const fn grf_io_vsel(&self) -> &GrfIoVsel {
        &self.grf_io_vsel
    }
    #[doc = "0xe644 - saradc test bit control register"]
    #[inline(always)]
    pub const fn grf_saradc_testbit(&self) -> &GrfSaradcTestbit {
        &self.grf_saradc_testbit
    }
    #[doc = "0xe648 - saradc test bit control register"]
    #[inline(always)]
    pub const fn grf_tsadc_testbit_l(&self) -> &GrfTsadcTestbitL {
        &self.grf_tsadc_testbit_l
    }
    #[doc = "0xe64c - tsadc test bit control register"]
    #[inline(always)]
    pub const fn grf_tsadc_testbit_h(&self) -> &GrfTsadcTestbitH {
        &self.grf_tsadc_testbit_h
    }
    #[doc = "0xe800 - chip id register"]
    #[inline(always)]
    pub const fn grf_chip_id_addr(&self) -> &GrfChipIdAddr {
        &self.grf_chip_id_addr
    }
    #[doc = "0xe880 - faster boot address register"]
    #[inline(always)]
    pub const fn grf_fast_boot_addr(&self) -> &GrfFastBootAddr {
        &self.grf_fast_boot_addr
    }
    #[doc = "0xf000 - emmc core control register"]
    #[inline(always)]
    pub const fn grf_emmccore_con0(&self) -> &GrfEmmccoreCon0 {
        &self.grf_emmccore_con0
    }
    #[doc = "0xf004 - emmc core control register"]
    #[inline(always)]
    pub const fn grf_emmccore_con1(&self) -> &GrfEmmccoreCon1 {
        &self.grf_emmccore_con1
    }
    #[doc = "0xf008 - emmc core control register"]
    #[inline(always)]
    pub const fn grf_emmccore_con2(&self) -> &GrfEmmccoreCon2 {
        &self.grf_emmccore_con2
    }
    #[doc = "0xf00c - emmc core control register"]
    #[inline(always)]
    pub const fn grf_emmccore_con3(&self) -> &GrfEmmccoreCon3 {
        &self.grf_emmccore_con3
    }
    #[doc = "0xf010 - emmc core control register"]
    #[inline(always)]
    pub const fn grf_emmccore_con4(&self) -> &GrfEmmccoreCon4 {
        &self.grf_emmccore_con4
    }
    #[doc = "0xf014 - emmc core control register"]
    #[inline(always)]
    pub const fn grf_emmccore_con5(&self) -> &GrfEmmccoreCon5 {
        &self.grf_emmccore_con5
    }
    #[doc = "0xf018 - emmc core control register"]
    #[inline(always)]
    pub const fn grf_emmccore_con6(&self) -> &GrfEmmccoreCon6 {
        &self.grf_emmccore_con6
    }
    #[doc = "0xf01c - emmc core control register"]
    #[inline(always)]
    pub const fn grf_emmccore_con7(&self) -> &GrfEmmccoreCon7 {
        &self.grf_emmccore_con7
    }
    #[doc = "0xf020 - emmc core control register"]
    #[inline(always)]
    pub const fn grf_emmccore_con8(&self) -> &GrfEmmccoreCon8 {
        &self.grf_emmccore_con8
    }
    #[doc = "0xf024 - emmc core control register"]
    #[inline(always)]
    pub const fn grf_emmccore_con9(&self) -> &GrfEmmccoreCon9 {
        &self.grf_emmccore_con9
    }
    #[doc = "0xf028 - emmc core control register"]
    #[inline(always)]
    pub const fn grf_emmccore_con10(&self) -> &GrfEmmccoreCon10 {
        &self.grf_emmccore_con10
    }
    #[doc = "0xf02c - emmc core control register"]
    #[inline(always)]
    pub const fn grf_emmccore_con11(&self) -> &GrfEmmccoreCon11 {
        &self.grf_emmccore_con11
    }
    #[doc = "0xf040 - emmc core status register"]
    #[inline(always)]
    pub const fn grf_emmccore_status0(&self) -> &GrfEmmccoreStatus0 {
        &self.grf_emmccore_status0
    }
    #[doc = "0xf044 - emmc core status register"]
    #[inline(always)]
    pub const fn grf_emmccore_status1(&self) -> &GrfEmmccoreStatus1 {
        &self.grf_emmccore_status1
    }
    #[doc = "0xf048 - emmc core status register"]
    #[inline(always)]
    pub const fn grf_emmccore_status2(&self) -> &GrfEmmccoreStatus2 {
        &self.grf_emmccore_status2
    }
    #[doc = "0xf04c - emmc core status register"]
    #[inline(always)]
    pub const fn grf_emmccore_status3(&self) -> &GrfEmmccoreStatus3 {
        &self.grf_emmccore_status3
    }
    #[doc = "0xf780 - emmc phy control register"]
    #[inline(always)]
    pub const fn grf_emmcphy_con0(&self) -> &GrfEmmcphyCon0 {
        &self.grf_emmcphy_con0
    }
    #[doc = "0xf784 - emmc phy control register"]
    #[inline(always)]
    pub const fn grf_emmcphy_con1(&self) -> &GrfEmmcphyCon1 {
        &self.grf_emmcphy_con1
    }
    #[doc = "0xf788 - emmc phy control register"]
    #[inline(always)]
    pub const fn grf_emmcphy_con2(&self) -> &GrfEmmcphyCon2 {
        &self.grf_emmcphy_con2
    }
    #[doc = "0xf78c - emmc phy control register"]
    #[inline(always)]
    pub const fn grf_emmcphy_con3(&self) -> &GrfEmmcphyCon3 {
        &self.grf_emmcphy_con3
    }
    #[doc = "0xf790 - emmc phy control register"]
    #[inline(always)]
    pub const fn grf_emmcphy_con4(&self) -> &GrfEmmcphyCon4 {
        &self.grf_emmcphy_con4
    }
    #[doc = "0xf794 - emmc phy control register"]
    #[inline(always)]
    pub const fn grf_emmcphy_con5(&self) -> &GrfEmmcphyCon5 {
        &self.grf_emmcphy_con5
    }
    #[doc = "0xf798 - emmc phy control register"]
    #[inline(always)]
    pub const fn grf_emmcphy_con6(&self) -> &GrfEmmcphyCon6 {
        &self.grf_emmcphy_con6
    }
    #[doc = "0xf7a0 - emmc phy status register"]
    #[inline(always)]
    pub const fn grf_emmcphy_status(&self) -> &GrfEmmcphyStatus {
        &self.grf_emmcphy_status
    }
}
#[doc = "GRF_USB3_PERF_CON0 (rw) register accessor: usb3 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3_perf_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3_perf_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3_perf_con0`]
module"]
#[doc(alias = "GRF_USB3_PERF_CON0")]
pub type GrfUsb3PerfCon0 = crate::Reg<grf_usb3_perf_con0::GrfUsb3PerfCon0Spec>;
#[doc = "usb3 performance monitor control register"]
pub mod grf_usb3_perf_con0;
#[doc = "GRF_USB3_PERF_CON1 (rw) register accessor: usb3 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3_perf_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3_perf_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3_perf_con1`]
module"]
#[doc(alias = "GRF_USB3_PERF_CON1")]
pub type GrfUsb3PerfCon1 = crate::Reg<grf_usb3_perf_con1::GrfUsb3PerfCon1Spec>;
#[doc = "usb3 performance monitor control register"]
pub mod grf_usb3_perf_con1;
#[doc = "GRF_USB3_PERF_CON2 (rw) register accessor: usb3 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3_perf_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3_perf_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3_perf_con2`]
module"]
#[doc(alias = "GRF_USB3_PERF_CON2")]
pub type GrfUsb3PerfCon2 = crate::Reg<grf_usb3_perf_con2::GrfUsb3PerfCon2Spec>;
#[doc = "usb3 performance monitor control register"]
pub mod grf_usb3_perf_con2;
#[doc = "GRF_USB3_PERF_RD_MAX_LATENCY_NUM (rw) register accessor: usb3 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3_perf_rd_max_latency_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3_perf_rd_max_latency_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3_perf_rd_max_latency_num`]
module"]
#[doc(alias = "GRF_USB3_PERF_RD_MAX_LATENCY_NUM")]
pub type GrfUsb3PerfRdMaxLatencyNum =
    crate::Reg<grf_usb3_perf_rd_max_latency_num::GrfUsb3PerfRdMaxLatencyNumSpec>;
#[doc = "usb3 performance monitor status register"]
pub mod grf_usb3_perf_rd_max_latency_num;
#[doc = "GRF_USB3_PERF_RD_LATENCY_SAMP_NUM (rw) register accessor: usb3 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3_perf_rd_latency_samp_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3_perf_rd_latency_samp_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3_perf_rd_latency_samp_num`]
module"]
#[doc(alias = "GRF_USB3_PERF_RD_LATENCY_SAMP_NUM")]
pub type GrfUsb3PerfRdLatencySampNum =
    crate::Reg<grf_usb3_perf_rd_latency_samp_num::GrfUsb3PerfRdLatencySampNumSpec>;
#[doc = "usb3 performance monitor status register"]
pub mod grf_usb3_perf_rd_latency_samp_num;
#[doc = "GRF_USB3_PERF_RD_LATENCY_ACC_NUM (rw) register accessor: usb3 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3_perf_rd_latency_acc_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3_perf_rd_latency_acc_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3_perf_rd_latency_acc_num`]
module"]
#[doc(alias = "GRF_USB3_PERF_RD_LATENCY_ACC_NUM")]
pub type GrfUsb3PerfRdLatencyAccNum =
    crate::Reg<grf_usb3_perf_rd_latency_acc_num::GrfUsb3PerfRdLatencyAccNumSpec>;
#[doc = "usb3 performance monitor status register"]
pub mod grf_usb3_perf_rd_latency_acc_num;
#[doc = "GRF_USB3_PERF_RD_AXI_TOTAL_BYTE (rw) register accessor: usb3 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3_perf_rd_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3_perf_rd_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3_perf_rd_axi_total_byte`]
module"]
#[doc(alias = "GRF_USB3_PERF_RD_AXI_TOTAL_BYTE")]
pub type GrfUsb3PerfRdAxiTotalByte =
    crate::Reg<grf_usb3_perf_rd_axi_total_byte::GrfUsb3PerfRdAxiTotalByteSpec>;
#[doc = "usb3 performance monitor status register"]
pub mod grf_usb3_perf_rd_axi_total_byte;
#[doc = "GRF_USB3_PERF_WR_AXI_TOTAL_BYTE (rw) register accessor: usb3 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3_perf_wr_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3_perf_wr_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3_perf_wr_axi_total_byte`]
module"]
#[doc(alias = "GRF_USB3_PERF_WR_AXI_TOTAL_BYTE")]
pub type GrfUsb3PerfWrAxiTotalByte =
    crate::Reg<grf_usb3_perf_wr_axi_total_byte::GrfUsb3PerfWrAxiTotalByteSpec>;
#[doc = "usb3 performance monitor status register"]
pub mod grf_usb3_perf_wr_axi_total_byte;
#[doc = "GRF_USB3_PERF_WORKING_CNT (rw) register accessor: usb3 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3_perf_working_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3_perf_working_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3_perf_working_cnt`]
module"]
#[doc(alias = "GRF_USB3_PERF_WORKING_CNT")]
pub type GrfUsb3PerfWorkingCnt = crate::Reg<grf_usb3_perf_working_cnt::GrfUsb3PerfWorkingCntSpec>;
#[doc = "usb3 performance monitor status register"]
pub mod grf_usb3_perf_working_cnt;
#[doc = "GRF_USB3OTG0_CON0 (rw) register accessor: USB3 OTG0 GRF Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3otg0_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3otg0_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3otg0_con0`]
module"]
#[doc(alias = "GRF_USB3OTG0_CON0")]
pub type GrfUsb3otg0Con0 = crate::Reg<grf_usb3otg0_con0::GrfUsb3otg0Con0Spec>;
#[doc = "USB3 OTG0 GRF Register0"]
pub mod grf_usb3otg0_con0;
#[doc = "GRF_USB3OTG0_CON1 (rw) register accessor: USB3 OTG0 GRF Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3otg0_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3otg0_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3otg0_con1`]
module"]
#[doc(alias = "GRF_USB3OTG0_CON1")]
pub type GrfUsb3otg0Con1 = crate::Reg<grf_usb3otg0_con1::GrfUsb3otg0Con1Spec>;
#[doc = "USB3 OTG0 GRF Register1"]
pub mod grf_usb3otg0_con1;
#[doc = "GRF_USB3OTG1_CON0 (rw) register accessor: USB3 OTG1 GRF Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3otg1_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3otg1_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3otg1_con0`]
module"]
#[doc(alias = "GRF_USB3OTG1_CON0")]
pub type GrfUsb3otg1Con0 = crate::Reg<grf_usb3otg1_con0::GrfUsb3otg1Con0Spec>;
#[doc = "USB3 OTG1 GRF Register0"]
pub mod grf_usb3otg1_con0;
#[doc = "GRF_USB3OTG1_CON1 (rw) register accessor: USB3 OTG1 GRF Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3otg1_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3otg1_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3otg1_con1`]
module"]
#[doc(alias = "GRF_USB3OTG1_CON1")]
pub type GrfUsb3otg1Con1 = crate::Reg<grf_usb3otg1_con1::GrfUsb3otg1Con1Spec>;
#[doc = "USB3 OTG1 GRF Register1"]
pub mod grf_usb3otg1_con1;
#[doc = "GRF_USB3OTG0_STATUS_LAT0 (rw) register accessor: USB3 OTG0 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3otg0_status_lat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3otg0_status_lat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3otg0_status_lat0`]
module"]
#[doc(alias = "GRF_USB3OTG0_STATUS_LAT0")]
pub type GrfUsb3otg0StatusLat0 = crate::Reg<grf_usb3otg0_status_lat0::GrfUsb3otg0StatusLat0Spec>;
#[doc = "USB3 OTG0 status register"]
pub mod grf_usb3otg0_status_lat0;
#[doc = "GRF_USB3OTG0_STATUS_LAT1 (rw) register accessor: USB3 OTG1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3otg0_status_lat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3otg0_status_lat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3otg0_status_lat1`]
module"]
#[doc(alias = "GRF_USB3OTG0_STATUS_LAT1")]
pub type GrfUsb3otg0StatusLat1 = crate::Reg<grf_usb3otg0_status_lat1::GrfUsb3otg0StatusLat1Spec>;
#[doc = "USB3 OTG1 status register"]
pub mod grf_usb3otg0_status_lat1;
#[doc = "GRF_USB3OTG0_STATUS_CB (rw) register accessor: USB3 OTG0 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3otg0_status_cb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3otg0_status_cb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3otg0_status_cb`]
module"]
#[doc(alias = "GRF_USB3OTG0_STATUS_CB")]
pub type GrfUsb3otg0StatusCb = crate::Reg<grf_usb3otg0_status_cb::GrfUsb3otg0StatusCbSpec>;
#[doc = "USB3 OTG0 status register"]
pub mod grf_usb3otg0_status_cb;
#[doc = "GRF_USB3OTG1_STATUS_LAT0 (rw) register accessor: USB3 OTG1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3otg1_status_lat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3otg1_status_lat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3otg1_status_lat0`]
module"]
#[doc(alias = "GRF_USB3OTG1_STATUS_LAT0")]
pub type GrfUsb3otg1StatusLat0 = crate::Reg<grf_usb3otg1_status_lat0::GrfUsb3otg1StatusLat0Spec>;
#[doc = "USB3 OTG1 status register"]
pub mod grf_usb3otg1_status_lat0;
#[doc = "GRF_USB3OTG1_STATUS_LAT1 (rw) register accessor: USB3 OTG1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3otg1_status_lat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3otg1_status_lat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3otg1_status_lat1`]
module"]
#[doc(alias = "GRF_USB3OTG1_STATUS_LAT1")]
pub type GrfUsb3otg1StatusLat1 = crate::Reg<grf_usb3otg1_status_lat1::GrfUsb3otg1StatusLat1Spec>;
#[doc = "USB3 OTG1 status register"]
pub mod grf_usb3otg1_status_lat1;
#[doc = "GRF_USB3OTG1_STATUS_CB (rw) register accessor: USB3 OTG1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3otg1_status_cb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3otg1_status_cb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3otg1_status_cb`]
module"]
#[doc(alias = "GRF_USB3OTG1_STATUS_CB")]
pub type GrfUsb3otg1StatusCb = crate::Reg<grf_usb3otg1_status_cb::GrfUsb3otg1StatusCbSpec>;
#[doc = "USB3 OTG1 status register"]
pub mod grf_usb3otg1_status_cb;
#[doc = "GRF_PCIE_PERF_CON0 (rw) register accessor: pcie performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_pcie_perf_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_pcie_perf_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_pcie_perf_con0`]
module"]
#[doc(alias = "GRF_PCIE_PERF_CON0")]
pub type GrfPciePerfCon0 = crate::Reg<grf_pcie_perf_con0::GrfPciePerfCon0Spec>;
#[doc = "pcie performance monitor control register"]
pub mod grf_pcie_perf_con0;
#[doc = "GRF_PCIE_PERF_CON1 (rw) register accessor: pcie performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_pcie_perf_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_pcie_perf_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_pcie_perf_con1`]
module"]
#[doc(alias = "GRF_PCIE_PERF_CON1")]
pub type GrfPciePerfCon1 = crate::Reg<grf_pcie_perf_con1::GrfPciePerfCon1Spec>;
#[doc = "pcie performance monitor control register"]
pub mod grf_pcie_perf_con1;
#[doc = "GRF_PCIE_PERF_CON2 (rw) register accessor: pcie performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_pcie_perf_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_pcie_perf_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_pcie_perf_con2`]
module"]
#[doc(alias = "GRF_PCIE_PERF_CON2")]
pub type GrfPciePerfCon2 = crate::Reg<grf_pcie_perf_con2::GrfPciePerfCon2Spec>;
#[doc = "pcie performance monitor control register"]
pub mod grf_pcie_perf_con2;
#[doc = "GRF_PCIE_PERF_RD_MAX_LATENCY_NUM (rw) register accessor: pcieperformance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_pcie_perf_rd_max_latency_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_pcie_perf_rd_max_latency_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_pcie_perf_rd_max_latency_num`]
module"]
#[doc(alias = "GRF_PCIE_PERF_RD_MAX_LATENCY_NUM")]
pub type GrfPciePerfRdMaxLatencyNum =
    crate::Reg<grf_pcie_perf_rd_max_latency_num::GrfPciePerfRdMaxLatencyNumSpec>;
#[doc = "pcieperformance monitor status register"]
pub mod grf_pcie_perf_rd_max_latency_num;
#[doc = "GRF_PCIE_PERF_RD_LATENCY_SAMP_NUM (rw) register accessor: pcie performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_pcie_perf_rd_latency_samp_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_pcie_perf_rd_latency_samp_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_pcie_perf_rd_latency_samp_num`]
module"]
#[doc(alias = "GRF_PCIE_PERF_RD_LATENCY_SAMP_NUM")]
pub type GrfPciePerfRdLatencySampNum =
    crate::Reg<grf_pcie_perf_rd_latency_samp_num::GrfPciePerfRdLatencySampNumSpec>;
#[doc = "pcie performance monitor status register"]
pub mod grf_pcie_perf_rd_latency_samp_num;
#[doc = "GRF_PCIE_PERF_RD_LATENCY_ACC_NUM (rw) register accessor: pcie performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_pcie_perf_rd_latency_acc_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_pcie_perf_rd_latency_acc_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_pcie_perf_rd_latency_acc_num`]
module"]
#[doc(alias = "GRF_PCIE_PERF_RD_LATENCY_ACC_NUM")]
pub type GrfPciePerfRdLatencyAccNum =
    crate::Reg<grf_pcie_perf_rd_latency_acc_num::GrfPciePerfRdLatencyAccNumSpec>;
#[doc = "pcie performance monitor status register"]
pub mod grf_pcie_perf_rd_latency_acc_num;
#[doc = "GRF_PCIE_PERF_RD_AXI_TOTAL_BYTE (rw) register accessor: pcie performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_pcie_perf_rd_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_pcie_perf_rd_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_pcie_perf_rd_axi_total_byte`]
module"]
#[doc(alias = "GRF_PCIE_PERF_RD_AXI_TOTAL_BYTE")]
pub type GrfPciePerfRdAxiTotalByte =
    crate::Reg<grf_pcie_perf_rd_axi_total_byte::GrfPciePerfRdAxiTotalByteSpec>;
#[doc = "pcie performance monitor status register"]
pub mod grf_pcie_perf_rd_axi_total_byte;
#[doc = "GRF_PCIE_PERF_WR_AXI_TOTAL_BYTE (rw) register accessor: pcie performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_pcie_perf_wr_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_pcie_perf_wr_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_pcie_perf_wr_axi_total_byte`]
module"]
#[doc(alias = "GRF_PCIE_PERF_WR_AXI_TOTAL_BYTE")]
pub type GrfPciePerfWrAxiTotalByte =
    crate::Reg<grf_pcie_perf_wr_axi_total_byte::GrfPciePerfWrAxiTotalByteSpec>;
#[doc = "pcie performance monitor status register"]
pub mod grf_pcie_perf_wr_axi_total_byte;
#[doc = "GRF_PCIE_PERF_WORKING_CNT (rw) register accessor: pcie performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_pcie_perf_working_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_pcie_perf_working_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_pcie_perf_working_cnt`]
module"]
#[doc(alias = "GRF_PCIE_PERF_WORKING_CNT")]
pub type GrfPciePerfWorkingCnt = crate::Reg<grf_pcie_perf_working_cnt::GrfPciePerfWorkingCntSpec>;
#[doc = "pcie performance monitor status register"]
pub mod grf_pcie_perf_working_cnt;
#[doc = "GRF_USB20_HOST0_CON0 (rw) register accessor: USB20 Host0 GRF register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb20_host0_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb20_host0_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb20_host0_con0`]
module"]
#[doc(alias = "GRF_USB20_HOST0_CON0")]
pub type GrfUsb20Host0Con0 = crate::Reg<grf_usb20_host0_con0::GrfUsb20Host0Con0Spec>;
#[doc = "USB20 Host0 GRF register0"]
pub mod grf_usb20_host0_con0;
#[doc = "GRF_USB20_HOST0_CON1 (rw) register accessor: USB20 Host0 GRF register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb20_host0_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb20_host0_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb20_host0_con1`]
module"]
#[doc(alias = "GRF_USB20_HOST0_CON1")]
pub type GrfUsb20Host0Con1 = crate::Reg<grf_usb20_host0_con1::GrfUsb20Host0Con1Spec>;
#[doc = "USB20 Host0 GRF register1"]
pub mod grf_usb20_host0_con1;
#[doc = "GRF_USB20_HOST1_CON0 (rw) register accessor: USB20 Host1 GRF register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb20_host1_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb20_host1_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb20_host1_con0`]
module"]
#[doc(alias = "GRF_USB20_HOST1_CON0")]
pub type GrfUsb20Host1Con0 = crate::Reg<grf_usb20_host1_con0::GrfUsb20Host1Con0Spec>;
#[doc = "USB20 Host1 GRF register0"]
pub mod grf_usb20_host1_con0;
#[doc = "GRF_USB20_HOST1_CON1 (rw) register accessor: USB20 Host1 GRF register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb20_host1_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb20_host1_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb20_host1_con1`]
module"]
#[doc(alias = "GRF_USB20_HOST1_CON1")]
pub type GrfUsb20Host1Con1 = crate::Reg<grf_usb20_host1_con1::GrfUsb20Host1Con1Spec>;
#[doc = "USB20 Host1 GRF register1"]
pub mod grf_usb20_host1_con1;
#[doc = "GRF_HSIC_CON0 (rw) register accessor: HSIC controller GRF register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_hsic_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_hsic_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_hsic_con0`]
module"]
#[doc(alias = "GRF_HSIC_CON0")]
pub type GrfHsicCon0 = crate::Reg<grf_hsic_con0::GrfHsicCon0Spec>;
#[doc = "HSIC controller GRF register 0"]
pub mod grf_hsic_con0;
#[doc = "GRF_HSIC_CON1 (rw) register accessor: HSIC controller GRF register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_hsic_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_hsic_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_hsic_con1`]
module"]
#[doc(alias = "GRF_HSIC_CON1")]
pub type GrfHsicCon1 = crate::Reg<grf_hsic_con1::GrfHsicCon1Spec>;
#[doc = "HSIC controller GRF register1"]
pub mod grf_hsic_con1;
#[doc = "GRF_GRF_USBHOST0_STATUS (rw) register accessor: usb host0 controller status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_grf_usbhost0_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_grf_usbhost0_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_grf_usbhost0_status`]
module"]
#[doc(alias = "GRF_GRF_USBHOST0_STATUS")]
pub type GrfGrfUsbhost0Status = crate::Reg<grf_grf_usbhost0_status::GrfGrfUsbhost0StatusSpec>;
#[doc = "usb host0 controller status register"]
pub mod grf_grf_usbhost0_status;
#[doc = "GRF_GRF_USBHOST1_STATUS (rw) register accessor: usb host1 controller status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_grf_usbhost1_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_grf_usbhost1_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_grf_usbhost1_status`]
module"]
#[doc(alias = "GRF_GRF_USBHOST1_STATUS")]
pub type GrfGrfUsbhost1Status = crate::Reg<grf_grf_usbhost1_status::GrfGrfUsbhost1StatusSpec>;
#[doc = "usb host1 controller status register"]
pub mod grf_grf_usbhost1_status;
#[doc = "GRF_GRF_HSIC_STATUS (rw) register accessor: hsic controller status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_grf_hsic_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_grf_hsic_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_grf_hsic_status`]
module"]
#[doc(alias = "GRF_GRF_HSIC_STATUS")]
pub type GrfGrfHsicStatus = crate::Reg<grf_grf_hsic_status::GrfGrfHsicStatusSpec>;
#[doc = "hsic controller status register"]
pub mod grf_grf_hsic_status;
#[doc = "GRF_HSICPHY_CON0 (rw) register accessor: HSICPHY GRF control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_hsicphy_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_hsicphy_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_hsicphy_con0`]
module"]
#[doc(alias = "GRF_HSICPHY_CON0")]
pub type GrfHsicphyCon0 = crate::Reg<grf_hsicphy_con0::GrfHsicphyCon0Spec>;
#[doc = "HSICPHY GRF control register"]
pub mod grf_hsicphy_con0;
#[doc = "GRF_usbphy0_ctrl0 (rw) register accessor: usbphy0_ctrl0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl0`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl0")]
pub type GrfUsbphy0Ctrl0 = crate::Reg<grf_usbphy0_ctrl0::GrfUsbphy0Ctrl0Spec>;
#[doc = "usbphy0_ctrl0"]
pub mod grf_usbphy0_ctrl0;
#[doc = "GRF_usbphy0_ctrl1 (rw) register accessor: usbphy0_ctrl1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl1`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl1")]
pub type GrfUsbphy0Ctrl1 = crate::Reg<grf_usbphy0_ctrl1::GrfUsbphy0Ctrl1Spec>;
#[doc = "usbphy0_ctrl1"]
pub mod grf_usbphy0_ctrl1;
#[doc = "GRF_usbphy0_ctrl2 (rw) register accessor: usbphy0_ctrl2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl2`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl2")]
pub type GrfUsbphy0Ctrl2 = crate::Reg<grf_usbphy0_ctrl2::GrfUsbphy0Ctrl2Spec>;
#[doc = "usbphy0_ctrl2"]
pub mod grf_usbphy0_ctrl2;
#[doc = "GRF_usbphy0_ctrl3 (rw) register accessor: usbphy0_ctrl3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl3`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl3")]
pub type GrfUsbphy0Ctrl3 = crate::Reg<grf_usbphy0_ctrl3::GrfUsbphy0Ctrl3Spec>;
#[doc = "usbphy0_ctrl3"]
pub mod grf_usbphy0_ctrl3;
#[doc = "GRF_usbphy0_ctrl4 (rw) register accessor: usbphy0_ctrl4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl4`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl4")]
pub type GrfUsbphy0Ctrl4 = crate::Reg<grf_usbphy0_ctrl4::GrfUsbphy0Ctrl4Spec>;
#[doc = "usbphy0_ctrl4"]
pub mod grf_usbphy0_ctrl4;
#[doc = "GRF_usbphy0_ctrl5 (rw) register accessor: usbphy0_ctrl5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl5`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl5")]
pub type GrfUsbphy0Ctrl5 = crate::Reg<grf_usbphy0_ctrl5::GrfUsbphy0Ctrl5Spec>;
#[doc = "usbphy0_ctrl5"]
pub mod grf_usbphy0_ctrl5;
#[doc = "GRF_usbphy0_ctrl6 (rw) register accessor: usbphy0_ctrl6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl6`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl6")]
pub type GrfUsbphy0Ctrl6 = crate::Reg<grf_usbphy0_ctrl6::GrfUsbphy0Ctrl6Spec>;
#[doc = "usbphy0_ctrl6"]
pub mod grf_usbphy0_ctrl6;
#[doc = "GRF_usbphy0_ctrl7 (rw) register accessor: usbphy0_ctrl7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl7`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl7")]
pub type GrfUsbphy0Ctrl7 = crate::Reg<grf_usbphy0_ctrl7::GrfUsbphy0Ctrl7Spec>;
#[doc = "usbphy0_ctrl7"]
pub mod grf_usbphy0_ctrl7;
#[doc = "GRF_usbphy0_ctrl8 (rw) register accessor: usbphy0_ctrl8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl8`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl8")]
pub type GrfUsbphy0Ctrl8 = crate::Reg<grf_usbphy0_ctrl8::GrfUsbphy0Ctrl8Spec>;
#[doc = "usbphy0_ctrl8"]
pub mod grf_usbphy0_ctrl8;
#[doc = "GRF_usbphy0_ctrl9 (rw) register accessor: usbphy0_ctrl9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl9`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl9")]
pub type GrfUsbphy0Ctrl9 = crate::Reg<grf_usbphy0_ctrl9::GrfUsbphy0Ctrl9Spec>;
#[doc = "usbphy0_ctrl9"]
pub mod grf_usbphy0_ctrl9;
#[doc = "GRF_usbphy0_ctrl10 (rw) register accessor: usbphy0_ctrl10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl10`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl10")]
pub type GrfUsbphy0Ctrl10 = crate::Reg<grf_usbphy0_ctrl10::GrfUsbphy0Ctrl10Spec>;
#[doc = "usbphy0_ctrl10"]
pub mod grf_usbphy0_ctrl10;
#[doc = "GRF_usbphy0_ctrl11 (rw) register accessor: usbphy0_ctrl11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl11`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl11")]
pub type GrfUsbphy0Ctrl11 = crate::Reg<grf_usbphy0_ctrl11::GrfUsbphy0Ctrl11Spec>;
#[doc = "usbphy0_ctrl11"]
pub mod grf_usbphy0_ctrl11;
#[doc = "GRF_usbphy0_ctrl12 (rw) register accessor: usbphy0_ctrl12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl12`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl12")]
pub type GrfUsbphy0Ctrl12 = crate::Reg<grf_usbphy0_ctrl12::GrfUsbphy0Ctrl12Spec>;
#[doc = "usbphy0_ctrl12"]
pub mod grf_usbphy0_ctrl12;
#[doc = "GRF_usbphy0_ctrl13 (rw) register accessor: usbphy0_ctrl13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl13`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl13")]
pub type GrfUsbphy0Ctrl13 = crate::Reg<grf_usbphy0_ctrl13::GrfUsbphy0Ctrl13Spec>;
#[doc = "usbphy0_ctrl13"]
pub mod grf_usbphy0_ctrl13;
#[doc = "GRF_usbphy0_ctrl14 (rw) register accessor: usbphy0_ctrl14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl14`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl14")]
pub type GrfUsbphy0Ctrl14 = crate::Reg<grf_usbphy0_ctrl14::GrfUsbphy0Ctrl14Spec>;
#[doc = "usbphy0_ctrl14"]
pub mod grf_usbphy0_ctrl14;
#[doc = "GRF_usbphy0_ctrl15 (rw) register accessor: usbphy0_ctrl15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl15`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl15")]
pub type GrfUsbphy0Ctrl15 = crate::Reg<grf_usbphy0_ctrl15::GrfUsbphy0Ctrl15Spec>;
#[doc = "usbphy0_ctrl15"]
pub mod grf_usbphy0_ctrl15;
#[doc = "GRF_usbphy0_ctrl16 (rw) register accessor: usbphy0_ctrl16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl16`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl16")]
pub type GrfUsbphy0Ctrl16 = crate::Reg<grf_usbphy0_ctrl16::GrfUsbphy0Ctrl16Spec>;
#[doc = "usbphy0_ctrl16"]
pub mod grf_usbphy0_ctrl16;
#[doc = "GRF_usbphy0_ctrl17 (rw) register accessor: usbphy0_ctrl17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl17`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl17")]
pub type GrfUsbphy0Ctrl17 = crate::Reg<grf_usbphy0_ctrl17::GrfUsbphy0Ctrl17Spec>;
#[doc = "usbphy0_ctrl17"]
pub mod grf_usbphy0_ctrl17;
#[doc = "GRF_usbphy0_ctrl18 (rw) register accessor: usbphy0_ctrl18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl18`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl18")]
pub type GrfUsbphy0Ctrl18 = crate::Reg<grf_usbphy0_ctrl18::GrfUsbphy0Ctrl18Spec>;
#[doc = "usbphy0_ctrl18"]
pub mod grf_usbphy0_ctrl18;
#[doc = "GRF_usbphy0_ctrl19 (rw) register accessor: usbphy0_ctrl19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl19`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl19")]
pub type GrfUsbphy0Ctrl19 = crate::Reg<grf_usbphy0_ctrl19::GrfUsbphy0Ctrl19Spec>;
#[doc = "usbphy0_ctrl19"]
pub mod grf_usbphy0_ctrl19;
#[doc = "GRF_usbphy0_ctrl20 (rw) register accessor: usbphy0_ctrl20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl20`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl20")]
pub type GrfUsbphy0Ctrl20 = crate::Reg<grf_usbphy0_ctrl20::GrfUsbphy0Ctrl20Spec>;
#[doc = "usbphy0_ctrl20"]
pub mod grf_usbphy0_ctrl20;
#[doc = "GRF_usbphy0_ctrl21 (rw) register accessor: usbphy0_ctrl21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl21`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl21")]
pub type GrfUsbphy0Ctrl21 = crate::Reg<grf_usbphy0_ctrl21::GrfUsbphy0Ctrl21Spec>;
#[doc = "usbphy0_ctrl21"]
pub mod grf_usbphy0_ctrl21;
#[doc = "GRF_usbphy0_ctrl22 (rw) register accessor: usbphy0_ctrl22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl22`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl22")]
pub type GrfUsbphy0Ctrl22 = crate::Reg<grf_usbphy0_ctrl22::GrfUsbphy0Ctrl22Spec>;
#[doc = "usbphy0_ctrl22"]
pub mod grf_usbphy0_ctrl22;
#[doc = "GRF_usbphy0_ctrl23 (rw) register accessor: usbphy0_ctrl23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl23`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl23")]
pub type GrfUsbphy0Ctrl23 = crate::Reg<grf_usbphy0_ctrl23::GrfUsbphy0Ctrl23Spec>;
#[doc = "usbphy0_ctrl23"]
pub mod grf_usbphy0_ctrl23;
#[doc = "GRF_usbphy0_ctrl24 (rw) register accessor: usbphy0_ctrl24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl24`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl24")]
pub type GrfUsbphy0Ctrl24 = crate::Reg<grf_usbphy0_ctrl24::GrfUsbphy0Ctrl24Spec>;
#[doc = "usbphy0_ctrl24"]
pub mod grf_usbphy0_ctrl24;
#[doc = "GRF_usbphy0_ctrl25 (rw) register accessor: usbphy0_ctrl25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy0_ctrl25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy0_ctrl25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy0_ctrl25`]
module"]
#[doc(alias = "GRF_usbphy0_ctrl25")]
pub type GrfUsbphy0Ctrl25 = crate::Reg<grf_usbphy0_ctrl25::GrfUsbphy0Ctrl25Spec>;
#[doc = "usbphy0_ctrl25"]
pub mod grf_usbphy0_ctrl25;
#[doc = "GRF_usbphy1_ctrl0 (rw) register accessor: usbphy1_ctrl0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl0`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl0")]
pub type GrfUsbphy1Ctrl0 = crate::Reg<grf_usbphy1_ctrl0::GrfUsbphy1Ctrl0Spec>;
#[doc = "usbphy1_ctrl0"]
pub mod grf_usbphy1_ctrl0;
#[doc = "GRF_usbphy1_ctrl1 (rw) register accessor: usbphy1_ctrl1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl1`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl1")]
pub type GrfUsbphy1Ctrl1 = crate::Reg<grf_usbphy1_ctrl1::GrfUsbphy1Ctrl1Spec>;
#[doc = "usbphy1_ctrl1"]
pub mod grf_usbphy1_ctrl1;
#[doc = "GRF_usbphy1_ctrl2 (rw) register accessor: usbphy1_ctrl2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl2`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl2")]
pub type GrfUsbphy1Ctrl2 = crate::Reg<grf_usbphy1_ctrl2::GrfUsbphy1Ctrl2Spec>;
#[doc = "usbphy1_ctrl2"]
pub mod grf_usbphy1_ctrl2;
#[doc = "GRF_usbphy1_ctrl3 (rw) register accessor: usbphy1_ctrl3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl3`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl3")]
pub type GrfUsbphy1Ctrl3 = crate::Reg<grf_usbphy1_ctrl3::GrfUsbphy1Ctrl3Spec>;
#[doc = "usbphy1_ctrl3"]
pub mod grf_usbphy1_ctrl3;
#[doc = "GRF_usbphy1_ctrl4 (rw) register accessor: usbphy1_ctrl4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl4`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl4")]
pub type GrfUsbphy1Ctrl4 = crate::Reg<grf_usbphy1_ctrl4::GrfUsbphy1Ctrl4Spec>;
#[doc = "usbphy1_ctrl4"]
pub mod grf_usbphy1_ctrl4;
#[doc = "GRF_usbphy1_ctrl5 (rw) register accessor: usbphy1_ctrl5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl5`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl5")]
pub type GrfUsbphy1Ctrl5 = crate::Reg<grf_usbphy1_ctrl5::GrfUsbphy1Ctrl5Spec>;
#[doc = "usbphy1_ctrl5"]
pub mod grf_usbphy1_ctrl5;
#[doc = "GRF_usbphy1_ctrl6 (rw) register accessor: usbphy1_ctrl6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl6`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl6")]
pub type GrfUsbphy1Ctrl6 = crate::Reg<grf_usbphy1_ctrl6::GrfUsbphy1Ctrl6Spec>;
#[doc = "usbphy1_ctrl6"]
pub mod grf_usbphy1_ctrl6;
#[doc = "GRF_usbphy1_ctrl7 (rw) register accessor: usbphy1_ctrl7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl7`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl7")]
pub type GrfUsbphy1Ctrl7 = crate::Reg<grf_usbphy1_ctrl7::GrfUsbphy1Ctrl7Spec>;
#[doc = "usbphy1_ctrl7"]
pub mod grf_usbphy1_ctrl7;
#[doc = "GRF_usbphy1_ctrl8 (rw) register accessor: usbphy1_ctrl8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl8`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl8")]
pub type GrfUsbphy1Ctrl8 = crate::Reg<grf_usbphy1_ctrl8::GrfUsbphy1Ctrl8Spec>;
#[doc = "usbphy1_ctrl8"]
pub mod grf_usbphy1_ctrl8;
#[doc = "GRF_usbphy1_ctrl9 (rw) register accessor: usbphy1_ctrl9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl9`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl9")]
pub type GrfUsbphy1Ctrl9 = crate::Reg<grf_usbphy1_ctrl9::GrfUsbphy1Ctrl9Spec>;
#[doc = "usbphy1_ctrl9"]
pub mod grf_usbphy1_ctrl9;
#[doc = "GRF_usbphy1_ctrl10 (rw) register accessor: usbphy1_ctrl10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl10`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl10")]
pub type GrfUsbphy1Ctrl10 = crate::Reg<grf_usbphy1_ctrl10::GrfUsbphy1Ctrl10Spec>;
#[doc = "usbphy1_ctrl10"]
pub mod grf_usbphy1_ctrl10;
#[doc = "GRF_usbphy1_ctrl11 (rw) register accessor: usbphy1_ctrl11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl11`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl11")]
pub type GrfUsbphy1Ctrl11 = crate::Reg<grf_usbphy1_ctrl11::GrfUsbphy1Ctrl11Spec>;
#[doc = "usbphy1_ctrl11"]
pub mod grf_usbphy1_ctrl11;
#[doc = "GRF_usbphy1_ctrl12 (rw) register accessor: usbphy1_ctrl12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl12`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl12")]
pub type GrfUsbphy1Ctrl12 = crate::Reg<grf_usbphy1_ctrl12::GrfUsbphy1Ctrl12Spec>;
#[doc = "usbphy1_ctrl12"]
pub mod grf_usbphy1_ctrl12;
#[doc = "GRF_usbphy1_ctrl13 (rw) register accessor: usbphy1_ctrl13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl13`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl13")]
pub type GrfUsbphy1Ctrl13 = crate::Reg<grf_usbphy1_ctrl13::GrfUsbphy1Ctrl13Spec>;
#[doc = "usbphy1_ctrl13"]
pub mod grf_usbphy1_ctrl13;
#[doc = "GRF_usbphy1_ctrl14 (rw) register accessor: usbphy1_ctrl14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl14`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl14")]
pub type GrfUsbphy1Ctrl14 = crate::Reg<grf_usbphy1_ctrl14::GrfUsbphy1Ctrl14Spec>;
#[doc = "usbphy1_ctrl14"]
pub mod grf_usbphy1_ctrl14;
#[doc = "GRF_usbphy1_ctrl15 (rw) register accessor: usbphy1_ctrl15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl15`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl15")]
pub type GrfUsbphy1Ctrl15 = crate::Reg<grf_usbphy1_ctrl15::GrfUsbphy1Ctrl15Spec>;
#[doc = "usbphy1_ctrl15"]
pub mod grf_usbphy1_ctrl15;
#[doc = "GRF_usbphy1_ctrl16 (rw) register accessor: usbphy1_ctrl16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl16`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl16")]
pub type GrfUsbphy1Ctrl16 = crate::Reg<grf_usbphy1_ctrl16::GrfUsbphy1Ctrl16Spec>;
#[doc = "usbphy1_ctrl16"]
pub mod grf_usbphy1_ctrl16;
#[doc = "GRF_usbphy1_ctrl17 (rw) register accessor: usbphy1_ctrl17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl17`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl17")]
pub type GrfUsbphy1Ctrl17 = crate::Reg<grf_usbphy1_ctrl17::GrfUsbphy1Ctrl17Spec>;
#[doc = "usbphy1_ctrl17"]
pub mod grf_usbphy1_ctrl17;
#[doc = "GRF_usbphy1_ctrl18 (rw) register accessor: usbphy1_ctrl18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl18`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl18")]
pub type GrfUsbphy1Ctrl18 = crate::Reg<grf_usbphy1_ctrl18::GrfUsbphy1Ctrl18Spec>;
#[doc = "usbphy1_ctrl18"]
pub mod grf_usbphy1_ctrl18;
#[doc = "GRF_usbphy1_ctrl19 (rw) register accessor: usbphy1_ctrl19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl19`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl19")]
pub type GrfUsbphy1Ctrl19 = crate::Reg<grf_usbphy1_ctrl19::GrfUsbphy1Ctrl19Spec>;
#[doc = "usbphy1_ctrl19"]
pub mod grf_usbphy1_ctrl19;
#[doc = "GRF_usbphy1_ctrl20 (rw) register accessor: usbphy1_ctrl20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl20`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl20")]
pub type GrfUsbphy1Ctrl20 = crate::Reg<grf_usbphy1_ctrl20::GrfUsbphy1Ctrl20Spec>;
#[doc = "usbphy1_ctrl20"]
pub mod grf_usbphy1_ctrl20;
#[doc = "GRF_usbphy1_ctrl21 (rw) register accessor: usbphy1_ctrl21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl21`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl21")]
pub type GrfUsbphy1Ctrl21 = crate::Reg<grf_usbphy1_ctrl21::GrfUsbphy1Ctrl21Spec>;
#[doc = "usbphy1_ctrl21"]
pub mod grf_usbphy1_ctrl21;
#[doc = "GRF_usbphy1_ctrl22 (rw) register accessor: usbphy1_ctrl22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl22`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl22")]
pub type GrfUsbphy1Ctrl22 = crate::Reg<grf_usbphy1_ctrl22::GrfUsbphy1Ctrl22Spec>;
#[doc = "usbphy1_ctrl22"]
pub mod grf_usbphy1_ctrl22;
#[doc = "GRF_usbphy1_ctrl23 (rw) register accessor: usbphy1_ctrl23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl23`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl23")]
pub type GrfUsbphy1Ctrl23 = crate::Reg<grf_usbphy1_ctrl23::GrfUsbphy1Ctrl23Spec>;
#[doc = "usbphy1_ctrl23"]
pub mod grf_usbphy1_ctrl23;
#[doc = "GRF_usbphy1_ctrl24 (rw) register accessor: usbphy1_ctrl24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl24`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl24")]
pub type GrfUsbphy1Ctrl24 = crate::Reg<grf_usbphy1_ctrl24::GrfUsbphy1Ctrl24Spec>;
#[doc = "usbphy1_ctrl24"]
pub mod grf_usbphy1_ctrl24;
#[doc = "GRF_usbphy1_ctrl25 (rw) register accessor: usbphy1_ctrl25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbphy1_ctrl25`]
module"]
#[doc(alias = "GRF_usbphy1_ctrl25")]
pub type GrfUsbphy1Ctrl25 = crate::Reg<grf_usbphy1_ctrl25::GrfUsbphy1Ctrl25Spec>;
#[doc = "usbphy1_ctrl25"]
pub mod grf_usbphy1_ctrl25;
#[doc = "GRF_HDCP22_PERF_CON0 (rw) register accessor: hdcp performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_hdcp22_perf_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_hdcp22_perf_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_hdcp22_perf_con0`]
module"]
#[doc(alias = "GRF_HDCP22_PERF_CON0")]
pub type GrfHdcp22PerfCon0 = crate::Reg<grf_hdcp22_perf_con0::GrfHdcp22PerfCon0Spec>;
#[doc = "hdcp performance monitor control register"]
pub mod grf_hdcp22_perf_con0;
#[doc = "GRF_HDCP22_PERF_CON1 (rw) register accessor: hdcp performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_hdcp22_perf_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_hdcp22_perf_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_hdcp22_perf_con1`]
module"]
#[doc(alias = "GRF_HDCP22_PERF_CON1")]
pub type GrfHdcp22PerfCon1 = crate::Reg<grf_hdcp22_perf_con1::GrfHdcp22PerfCon1Spec>;
#[doc = "hdcp performance monitor control register"]
pub mod grf_hdcp22_perf_con1;
#[doc = "GRF_HDCP22_PERF_CON2 (rw) register accessor: hdcp performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_hdcp22_perf_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_hdcp22_perf_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_hdcp22_perf_con2`]
module"]
#[doc(alias = "GRF_HDCP22_PERF_CON2")]
pub type GrfHdcp22PerfCon2 = crate::Reg<grf_hdcp22_perf_con2::GrfHdcp22PerfCon2Spec>;
#[doc = "hdcp performance monitor control register"]
pub mod grf_hdcp22_perf_con2;
#[doc = "GRF_HDCP22_PERF_RD_MAX_LATENCY_NUM (rw) register accessor: hdcp performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_hdcp22_perf_rd_max_latency_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_hdcp22_perf_rd_max_latency_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_hdcp22_perf_rd_max_latency_num`]
module"]
#[doc(alias = "GRF_HDCP22_PERF_RD_MAX_LATENCY_NUM")]
pub type GrfHdcp22PerfRdMaxLatencyNum =
    crate::Reg<grf_hdcp22_perf_rd_max_latency_num::GrfHdcp22PerfRdMaxLatencyNumSpec>;
#[doc = "hdcp performance monitor status register"]
pub mod grf_hdcp22_perf_rd_max_latency_num;
#[doc = "GRF_HDCP22_PERF_RD_LATENCY_SAMP_NUM (rw) register accessor: hdcp performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_hdcp22_perf_rd_latency_samp_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_hdcp22_perf_rd_latency_samp_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_hdcp22_perf_rd_latency_samp_num`]
module"]
#[doc(alias = "GRF_HDCP22_PERF_RD_LATENCY_SAMP_NUM")]
pub type GrfHdcp22PerfRdLatencySampNum =
    crate::Reg<grf_hdcp22_perf_rd_latency_samp_num::GrfHdcp22PerfRdLatencySampNumSpec>;
#[doc = "hdcp performance monitor status register"]
pub mod grf_hdcp22_perf_rd_latency_samp_num;
#[doc = "GRF_HDCP22_PERF_RD_LATENCY_ACC_NUM (rw) register accessor: hdcp performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_hdcp22_perf_rd_latency_acc_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_hdcp22_perf_rd_latency_acc_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_hdcp22_perf_rd_latency_acc_num`]
module"]
#[doc(alias = "GRF_HDCP22_PERF_RD_LATENCY_ACC_NUM")]
pub type GrfHdcp22PerfRdLatencyAccNum =
    crate::Reg<grf_hdcp22_perf_rd_latency_acc_num::GrfHdcp22PerfRdLatencyAccNumSpec>;
#[doc = "hdcp performance monitor status register"]
pub mod grf_hdcp22_perf_rd_latency_acc_num;
#[doc = "GRF_HDCP22_PERF_RD_AXI_TOTAL_BYTE (rw) register accessor: hdcp performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_hdcp22_perf_rd_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_hdcp22_perf_rd_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_hdcp22_perf_rd_axi_total_byte`]
module"]
#[doc(alias = "GRF_HDCP22_PERF_RD_AXI_TOTAL_BYTE")]
pub type GrfHdcp22PerfRdAxiTotalByte =
    crate::Reg<grf_hdcp22_perf_rd_axi_total_byte::GrfHdcp22PerfRdAxiTotalByteSpec>;
#[doc = "hdcp performance monitor status register"]
pub mod grf_hdcp22_perf_rd_axi_total_byte;
#[doc = "GRF_HDCP22_PERF_WR_AXI_TOTAL_BYTE (rw) register accessor: hdcp performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_hdcp22_perf_wr_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_hdcp22_perf_wr_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_hdcp22_perf_wr_axi_total_byte`]
module"]
#[doc(alias = "GRF_HDCP22_PERF_WR_AXI_TOTAL_BYTE")]
pub type GrfHdcp22PerfWrAxiTotalByte =
    crate::Reg<grf_hdcp22_perf_wr_axi_total_byte::GrfHdcp22PerfWrAxiTotalByteSpec>;
#[doc = "hdcp performance monitor status register"]
pub mod grf_hdcp22_perf_wr_axi_total_byte;
#[doc = "GRF_HDCP22_PERF_WORKING_CNT (rw) register accessor: hdcp performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_hdcp22_perf_working_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_hdcp22_perf_working_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_hdcp22_perf_working_cnt`]
module"]
#[doc(alias = "GRF_HDCP22_PERF_WORKING_CNT")]
pub type GrfHdcp22PerfWorkingCnt =
    crate::Reg<grf_hdcp22_perf_working_cnt::GrfHdcp22PerfWorkingCntSpec>;
#[doc = "hdcp performance monitor status register"]
pub mod grf_hdcp22_perf_working_cnt;
#[doc = "GRF_SOC_CON9 (rw) register accessor: SoC control register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con9`]
module"]
#[doc(alias = "GRF_SOC_CON9")]
pub type GrfSocCon9 = crate::Reg<grf_soc_con9::GrfSocCon9Spec>;
#[doc = "SoC control register 9"]
pub mod grf_soc_con9;
#[doc = "GRF_SOC_CON20 (rw) register accessor: SoC control register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con20`]
module"]
#[doc(alias = "GRF_SOC_CON20")]
pub type GrfSocCon20 = crate::Reg<grf_soc_con20::GrfSocCon20Spec>;
#[doc = "SoC control register 20"]
pub mod grf_soc_con20;
#[doc = "GRF_SOC_CON21 (rw) register accessor: SoC control register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con21`]
module"]
#[doc(alias = "GRF_SOC_CON21")]
pub type GrfSocCon21 = crate::Reg<grf_soc_con21::GrfSocCon21Spec>;
#[doc = "SoC control register 21"]
pub mod grf_soc_con21;
#[doc = "GRF_SOC_CON22 (rw) register accessor: SoC control register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con22`]
module"]
#[doc(alias = "GRF_SOC_CON22")]
pub type GrfSocCon22 = crate::Reg<grf_soc_con22::GrfSocCon22Spec>;
#[doc = "SoC control register 22"]
pub mod grf_soc_con22;
#[doc = "GRF_SOC_CON23 (rw) register accessor: SoC control register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con23`]
module"]
#[doc(alias = "GRF_SOC_CON23")]
pub type GrfSocCon23 = crate::Reg<grf_soc_con23::GrfSocCon23Spec>;
#[doc = "SoC control register 23"]
pub mod grf_soc_con23;
#[doc = "GRF_SOC_CON24 (rw) register accessor: SoC control register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con24`]
module"]
#[doc(alias = "GRF_SOC_CON24")]
pub type GrfSocCon24 = crate::Reg<grf_soc_con24::GrfSocCon24Spec>;
#[doc = "SoC control register 24"]
pub mod grf_soc_con24;
#[doc = "GRF_SOC_CON25 (rw) register accessor: SoC control register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con25`]
module"]
#[doc(alias = "GRF_SOC_CON25")]
pub type GrfSocCon25 = crate::Reg<grf_soc_con25::GrfSocCon25Spec>;
#[doc = "SoC control register 25"]
pub mod grf_soc_con25;
#[doc = "GRF_SOC_CON26 (rw) register accessor: SoC control register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con26`]
module"]
#[doc(alias = "GRF_SOC_CON26")]
pub type GrfSocCon26 = crate::Reg<grf_soc_con26::GrfSocCon26Spec>;
#[doc = "SoC control register 26"]
pub mod grf_soc_con26;
#[doc = "GRF_GPU_PERF_CON0 (rw) register accessor: gpu performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpu_perf_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpu_perf_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpu_perf_con0`]
module"]
#[doc(alias = "GRF_GPU_PERF_CON0")]
pub type GrfGpuPerfCon0 = crate::Reg<grf_gpu_perf_con0::GrfGpuPerfCon0Spec>;
#[doc = "gpu performance monitor control register"]
pub mod grf_gpu_perf_con0;
#[doc = "GRF_GPU_PERF_CON1 (rw) register accessor: gpu performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpu_perf_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpu_perf_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpu_perf_con1`]
module"]
#[doc(alias = "GRF_GPU_PERF_CON1")]
pub type GrfGpuPerfCon1 = crate::Reg<grf_gpu_perf_con1::GrfGpuPerfCon1Spec>;
#[doc = "gpu performance monitor control register"]
pub mod grf_gpu_perf_con1;
#[doc = "GRF_GPU_PERF_CON2 (rw) register accessor: gpu performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpu_perf_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpu_perf_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpu_perf_con2`]
module"]
#[doc(alias = "GRF_GPU_PERF_CON2")]
pub type GrfGpuPerfCon2 = crate::Reg<grf_gpu_perf_con2::GrfGpuPerfCon2Spec>;
#[doc = "gpu performance monitor control register"]
pub mod grf_gpu_perf_con2;
#[doc = "GRF_GPU_PERF_RD_MAX_LATENCY_NUM (rw) register accessor: gpu performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpu_perf_rd_max_latency_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpu_perf_rd_max_latency_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpu_perf_rd_max_latency_num`]
module"]
#[doc(alias = "GRF_GPU_PERF_RD_MAX_LATENCY_NUM")]
pub type GrfGpuPerfRdMaxLatencyNum =
    crate::Reg<grf_gpu_perf_rd_max_latency_num::GrfGpuPerfRdMaxLatencyNumSpec>;
#[doc = "gpu performance monitor status register"]
pub mod grf_gpu_perf_rd_max_latency_num;
#[doc = "GRF_GPU_PERF_RD_LATENCY_SAMP_NUM (rw) register accessor: gpu performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpu_perf_rd_latency_samp_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpu_perf_rd_latency_samp_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpu_perf_rd_latency_samp_num`]
module"]
#[doc(alias = "GRF_GPU_PERF_RD_LATENCY_SAMP_NUM")]
pub type GrfGpuPerfRdLatencySampNum =
    crate::Reg<grf_gpu_perf_rd_latency_samp_num::GrfGpuPerfRdLatencySampNumSpec>;
#[doc = "gpu performance monitor status register"]
pub mod grf_gpu_perf_rd_latency_samp_num;
#[doc = "GRF_GPU_PERF_RD_LATENCY_ACC_NUM (rw) register accessor: gpu performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpu_perf_rd_latency_acc_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpu_perf_rd_latency_acc_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpu_perf_rd_latency_acc_num`]
module"]
#[doc(alias = "GRF_GPU_PERF_RD_LATENCY_ACC_NUM")]
pub type GrfGpuPerfRdLatencyAccNum =
    crate::Reg<grf_gpu_perf_rd_latency_acc_num::GrfGpuPerfRdLatencyAccNumSpec>;
#[doc = "gpu performance monitor status register"]
pub mod grf_gpu_perf_rd_latency_acc_num;
#[doc = "GRF_GPU_PERF_RD_AXI_TOTAL_BYTE (rw) register accessor: gpu performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpu_perf_rd_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpu_perf_rd_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpu_perf_rd_axi_total_byte`]
module"]
#[doc(alias = "GRF_GPU_PERF_RD_AXI_TOTAL_BYTE")]
pub type GrfGpuPerfRdAxiTotalByte =
    crate::Reg<grf_gpu_perf_rd_axi_total_byte::GrfGpuPerfRdAxiTotalByteSpec>;
#[doc = "gpu performance monitor status register"]
pub mod grf_gpu_perf_rd_axi_total_byte;
#[doc = "GRF_GPU_PERF_WR_AXI_TOTAL_BYTE (rw) register accessor: gpu performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpu_perf_wr_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpu_perf_wr_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpu_perf_wr_axi_total_byte`]
module"]
#[doc(alias = "GRF_GPU_PERF_WR_AXI_TOTAL_BYTE")]
pub type GrfGpuPerfWrAxiTotalByte =
    crate::Reg<grf_gpu_perf_wr_axi_total_byte::GrfGpuPerfWrAxiTotalByteSpec>;
#[doc = "gpu performance monitor status register"]
pub mod grf_gpu_perf_wr_axi_total_byte;
#[doc = "GRF_GPU_PERF_WORKING_CNT (rw) register accessor: gpu performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpu_perf_working_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpu_perf_working_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpu_perf_working_cnt`]
module"]
#[doc(alias = "GRF_GPU_PERF_WORKING_CNT")]
pub type GrfGpuPerfWorkingCnt = crate::Reg<grf_gpu_perf_working_cnt::GrfGpuPerfWorkingCntSpec>;
#[doc = "gpu performance monitor status register"]
pub mod grf_gpu_perf_working_cnt;
#[doc = "GRF_CPU_CON0 (rw) register accessor: cpu control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_cpu_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_cpu_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_cpu_con0`]
module"]
#[doc(alias = "GRF_CPU_CON0")]
pub type GrfCpuCon0 = crate::Reg<grf_cpu_con0::GrfCpuCon0Spec>;
#[doc = "cpu control register 0"]
pub mod grf_cpu_con0;
#[doc = "GRF_CPU_CON1 (rw) register accessor: cpu control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_cpu_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_cpu_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_cpu_con1`]
module"]
#[doc(alias = "GRF_CPU_CON1")]
pub type GrfCpuCon1 = crate::Reg<grf_cpu_con1::GrfCpuCon1Spec>;
#[doc = "cpu control register 1"]
pub mod grf_cpu_con1;
#[doc = "GRF_CPU_CON2 (rw) register accessor: cpu control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_cpu_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_cpu_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_cpu_con2`]
module"]
#[doc(alias = "GRF_CPU_CON2")]
pub type GrfCpuCon2 = crate::Reg<grf_cpu_con2::GrfCpuCon2Spec>;
#[doc = "cpu control register 2"]
pub mod grf_cpu_con2;
#[doc = "GRF_CPU_CON3 (rw) register accessor: cpu control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_cpu_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_cpu_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_cpu_con3`]
module"]
#[doc(alias = "GRF_CPU_CON3")]
pub type GrfCpuCon3 = crate::Reg<grf_cpu_con3::GrfCpuCon3Spec>;
#[doc = "cpu control register 3"]
pub mod grf_cpu_con3;
#[doc = "GRF_CPU_STATUS0 (rw) register accessor: cpu status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_cpu_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_cpu_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_cpu_status0`]
module"]
#[doc(alias = "GRF_CPU_STATUS0")]
pub type GrfCpuStatus0 = crate::Reg<grf_cpu_status0::GrfCpuStatus0Spec>;
#[doc = "cpu status register 0"]
pub mod grf_cpu_status0;
#[doc = "GRF_CPU_STATUS1 (rw) register accessor: cpu status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_cpu_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_cpu_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_cpu_status1`]
module"]
#[doc(alias = "GRF_CPU_STATUS1")]
pub type GrfCpuStatus1 = crate::Reg<grf_cpu_status1::GrfCpuStatus1Spec>;
#[doc = "cpu status register 1"]
pub mod grf_cpu_status1;
#[doc = "GRF_CPU_STATUS2 (rw) register accessor: cpu status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_cpu_status2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_cpu_status2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_cpu_status2`]
module"]
#[doc(alias = "GRF_CPU_STATUS2")]
pub type GrfCpuStatus2 = crate::Reg<grf_cpu_status2::GrfCpuStatus2Spec>;
#[doc = "cpu status register 2"]
pub mod grf_cpu_status2;
#[doc = "GRF_CPU_STATUS3 (rw) register accessor: cpu status register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_cpu_status3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_cpu_status3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_cpu_status3`]
module"]
#[doc(alias = "GRF_CPU_STATUS3")]
pub type GrfCpuStatus3 = crate::Reg<grf_cpu_status3::GrfCpuStatus3Spec>;
#[doc = "cpu status register 3"]
pub mod grf_cpu_status3;
#[doc = "GRF_CPU_STATUS4 (rw) register accessor: cpu status register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_cpu_status4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_cpu_status4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_cpu_status4`]
module"]
#[doc(alias = "GRF_CPU_STATUS4")]
pub type GrfCpuStatus4 = crate::Reg<grf_cpu_status4::GrfCpuStatus4Spec>;
#[doc = "cpu status register 4"]
pub mod grf_cpu_status4;
#[doc = "GRF_CPU_STATUS5 (rw) register accessor: cpu status register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_cpu_status5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_cpu_status5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_cpu_status5`]
module"]
#[doc(alias = "GRF_CPU_STATUS5")]
pub type GrfCpuStatus5 = crate::Reg<grf_cpu_status5::GrfCpuStatus5Spec>;
#[doc = "cpu status register 5"]
pub mod grf_cpu_status5;
#[doc = "GRF_A53_PERF_CON0 (rw) register accessor: a53 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a53_perf_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a53_perf_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a53_perf_con0`]
module"]
#[doc(alias = "GRF_A53_PERF_CON0")]
pub type GrfA53PerfCon0 = crate::Reg<grf_a53_perf_con0::GrfA53PerfCon0Spec>;
#[doc = "a53 performance monitor control register"]
pub mod grf_a53_perf_con0;
#[doc = "GRF_A53_PERF_CON1 (rw) register accessor: a53 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a53_perf_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a53_perf_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a53_perf_con1`]
module"]
#[doc(alias = "GRF_A53_PERF_CON1")]
pub type GrfA53PerfCon1 = crate::Reg<grf_a53_perf_con1::GrfA53PerfCon1Spec>;
#[doc = "a53 performance monitor control register"]
pub mod grf_a53_perf_con1;
#[doc = "GRF_A53_PERF_CON2 (rw) register accessor: a53 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a53_perf_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a53_perf_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a53_perf_con2`]
module"]
#[doc(alias = "GRF_A53_PERF_CON2")]
pub type GrfA53PerfCon2 = crate::Reg<grf_a53_perf_con2::GrfA53PerfCon2Spec>;
#[doc = "a53 performance monitor control register"]
pub mod grf_a53_perf_con2;
#[doc = "GRF_A53_PERF_CON3 (rw) register accessor: a53 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a53_perf_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a53_perf_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a53_perf_con3`]
module"]
#[doc(alias = "GRF_A53_PERF_CON3")]
pub type GrfA53PerfCon3 = crate::Reg<grf_a53_perf_con3::GrfA53PerfCon3Spec>;
#[doc = "a53 performance monitor control register"]
pub mod grf_a53_perf_con3;
#[doc = "GRF_A53_PERF_RD_MON_ST (rw) register accessor: performance monitor read start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a53_perf_rd_mon_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a53_perf_rd_mon_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a53_perf_rd_mon_st`]
module"]
#[doc(alias = "GRF_A53_PERF_RD_MON_ST")]
pub type GrfA53PerfRdMonSt = crate::Reg<grf_a53_perf_rd_mon_st::GrfA53PerfRdMonStSpec>;
#[doc = "performance monitor read start address"]
pub mod grf_a53_perf_rd_mon_st;
#[doc = "GRF_A53_PERF_RD_MON_END (rw) register accessor: performance monitor end address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a53_perf_rd_mon_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a53_perf_rd_mon_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a53_perf_rd_mon_end`]
module"]
#[doc(alias = "GRF_A53_PERF_RD_MON_END")]
pub type GrfA53PerfRdMonEnd = crate::Reg<grf_a53_perf_rd_mon_end::GrfA53PerfRdMonEndSpec>;
#[doc = "performance monitor end address"]
pub mod grf_a53_perf_rd_mon_end;
#[doc = "GRF_A53_PERF_WR_MON_ST (rw) register accessor: performance write monitor start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a53_perf_wr_mon_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a53_perf_wr_mon_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a53_perf_wr_mon_st`]
module"]
#[doc(alias = "GRF_A53_PERF_WR_MON_ST")]
pub type GrfA53PerfWrMonSt = crate::Reg<grf_a53_perf_wr_mon_st::GrfA53PerfWrMonStSpec>;
#[doc = "performance write monitor start address"]
pub mod grf_a53_perf_wr_mon_st;
#[doc = "GRF_A53_PERF_WR_MON_END (rw) register accessor: performance monitor write end address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a53_perf_wr_mon_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a53_perf_wr_mon_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a53_perf_wr_mon_end`]
module"]
#[doc(alias = "GRF_A53_PERF_WR_MON_END")]
pub type GrfA53PerfWrMonEnd = crate::Reg<grf_a53_perf_wr_mon_end::GrfA53PerfWrMonEndSpec>;
#[doc = "performance monitor write end address"]
pub mod grf_a53_perf_wr_mon_end;
#[doc = "GRF_A53_PERF_RD_MAX_LATENCY_NUM (rw) register accessor: a53 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a53_perf_rd_max_latency_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a53_perf_rd_max_latency_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a53_perf_rd_max_latency_num`]
module"]
#[doc(alias = "GRF_A53_PERF_RD_MAX_LATENCY_NUM")]
pub type GrfA53PerfRdMaxLatencyNum =
    crate::Reg<grf_a53_perf_rd_max_latency_num::GrfA53PerfRdMaxLatencyNumSpec>;
#[doc = "a53 performance monitor status register"]
pub mod grf_a53_perf_rd_max_latency_num;
#[doc = "GRF_A53_PERF_RD_LATENCY_SAMP_NUM (rw) register accessor: a53 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a53_perf_rd_latency_samp_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a53_perf_rd_latency_samp_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a53_perf_rd_latency_samp_num`]
module"]
#[doc(alias = "GRF_A53_PERF_RD_LATENCY_SAMP_NUM")]
pub type GrfA53PerfRdLatencySampNum =
    crate::Reg<grf_a53_perf_rd_latency_samp_num::GrfA53PerfRdLatencySampNumSpec>;
#[doc = "a53 performance monitor status register"]
pub mod grf_a53_perf_rd_latency_samp_num;
#[doc = "GRF_A53_PERF_RD_LATENCY_ACC_NUM (rw) register accessor: a53 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a53_perf_rd_latency_acc_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a53_perf_rd_latency_acc_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a53_perf_rd_latency_acc_num`]
module"]
#[doc(alias = "GRF_A53_PERF_RD_LATENCY_ACC_NUM")]
pub type GrfA53PerfRdLatencyAccNum =
    crate::Reg<grf_a53_perf_rd_latency_acc_num::GrfA53PerfRdLatencyAccNumSpec>;
#[doc = "a53 performance monitor status register"]
pub mod grf_a53_perf_rd_latency_acc_num;
#[doc = "GRF_A53_PERF_RD_AXI_TOTAL_BYTE (rw) register accessor: a53 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a53_perf_rd_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a53_perf_rd_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a53_perf_rd_axi_total_byte`]
module"]
#[doc(alias = "GRF_A53_PERF_RD_AXI_TOTAL_BYTE")]
pub type GrfA53PerfRdAxiTotalByte =
    crate::Reg<grf_a53_perf_rd_axi_total_byte::GrfA53PerfRdAxiTotalByteSpec>;
#[doc = "a53 performance monitor status register"]
pub mod grf_a53_perf_rd_axi_total_byte;
#[doc = "GRF_A53_PERF_WR_AXI_TOTAL_BYTE (rw) register accessor: a53 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a53_perf_wr_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a53_perf_wr_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a53_perf_wr_axi_total_byte`]
module"]
#[doc(alias = "GRF_A53_PERF_WR_AXI_TOTAL_BYTE")]
pub type GrfA53PerfWrAxiTotalByte =
    crate::Reg<grf_a53_perf_wr_axi_total_byte::GrfA53PerfWrAxiTotalByteSpec>;
#[doc = "a53 performance monitor status register"]
pub mod grf_a53_perf_wr_axi_total_byte;
#[doc = "GRF_A53_PERF_WORKING_CNT (rw) register accessor: a53 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a53_perf_working_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a53_perf_working_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a53_perf_working_cnt`]
module"]
#[doc(alias = "GRF_A53_PERF_WORKING_CNT")]
pub type GrfA53PerfWorkingCnt = crate::Reg<grf_a53_perf_working_cnt::GrfA53PerfWorkingCntSpec>;
#[doc = "a53 performance monitor status register"]
pub mod grf_a53_perf_working_cnt;
#[doc = "GRF_A53_PERF_INT_STATUS (rw) register accessor: a53 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a53_perf_int_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a53_perf_int_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a53_perf_int_status`]
module"]
#[doc(alias = "GRF_A53_PERF_INT_STATUS")]
pub type GrfA53PerfIntStatus = crate::Reg<grf_a53_perf_int_status::GrfA53PerfIntStatusSpec>;
#[doc = "a53 performance monitor status register"]
pub mod grf_a53_perf_int_status;
#[doc = "GRF_A72_PERF_CON0 (rw) register accessor: a72 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a72_perf_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a72_perf_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a72_perf_con0`]
module"]
#[doc(alias = "GRF_A72_PERF_CON0")]
pub type GrfA72PerfCon0 = crate::Reg<grf_a72_perf_con0::GrfA72PerfCon0Spec>;
#[doc = "a72 performance monitor control register"]
pub mod grf_a72_perf_con0;
#[doc = "GRF_A72_PERF_CON1 (rw) register accessor: a72 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a72_perf_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a72_perf_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a72_perf_con1`]
module"]
#[doc(alias = "GRF_A72_PERF_CON1")]
pub type GrfA72PerfCon1 = crate::Reg<grf_a72_perf_con1::GrfA72PerfCon1Spec>;
#[doc = "a72 performance monitor control register"]
pub mod grf_a72_perf_con1;
#[doc = "GRF_A72_PERF_CON2 (rw) register accessor: a72 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a72_perf_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a72_perf_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a72_perf_con2`]
module"]
#[doc(alias = "GRF_A72_PERF_CON2")]
pub type GrfA72PerfCon2 = crate::Reg<grf_a72_perf_con2::GrfA72PerfCon2Spec>;
#[doc = "a72 performance monitor control register"]
pub mod grf_a72_perf_con2;
#[doc = "GRF_A72_PERF_CON3 (rw) register accessor: a72 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a72_perf_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a72_perf_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a72_perf_con3`]
module"]
#[doc(alias = "GRF_A72_PERF_CON3")]
pub type GrfA72PerfCon3 = crate::Reg<grf_a72_perf_con3::GrfA72PerfCon3Spec>;
#[doc = "a72 performance monitor control register"]
pub mod grf_a72_perf_con3;
#[doc = "GRF_A72_PERF_RD_MON_ST (rw) register accessor: performance monitor read start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a72_perf_rd_mon_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a72_perf_rd_mon_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a72_perf_rd_mon_st`]
module"]
#[doc(alias = "GRF_A72_PERF_RD_MON_ST")]
pub type GrfA72PerfRdMonSt = crate::Reg<grf_a72_perf_rd_mon_st::GrfA72PerfRdMonStSpec>;
#[doc = "performance monitor read start address"]
pub mod grf_a72_perf_rd_mon_st;
#[doc = "GRF_A72_PERF_RD_MON_END (rw) register accessor: performance monitor end address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a72_perf_rd_mon_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a72_perf_rd_mon_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a72_perf_rd_mon_end`]
module"]
#[doc(alias = "GRF_A72_PERF_RD_MON_END")]
pub type GrfA72PerfRdMonEnd = crate::Reg<grf_a72_perf_rd_mon_end::GrfA72PerfRdMonEndSpec>;
#[doc = "performance monitor end address"]
pub mod grf_a72_perf_rd_mon_end;
#[doc = "GRF_A72_PERF_WR_MON_ST (rw) register accessor: performance write monitor start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a72_perf_wr_mon_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a72_perf_wr_mon_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a72_perf_wr_mon_st`]
module"]
#[doc(alias = "GRF_A72_PERF_WR_MON_ST")]
pub type GrfA72PerfWrMonSt = crate::Reg<grf_a72_perf_wr_mon_st::GrfA72PerfWrMonStSpec>;
#[doc = "performance write monitor start address"]
pub mod grf_a72_perf_wr_mon_st;
#[doc = "GRF_A72_PERF_WR_MON_END (rw) register accessor: performance monitor write end address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a72_perf_wr_mon_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a72_perf_wr_mon_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a72_perf_wr_mon_end`]
module"]
#[doc(alias = "GRF_A72_PERF_WR_MON_END")]
pub type GrfA72PerfWrMonEnd = crate::Reg<grf_a72_perf_wr_mon_end::GrfA72PerfWrMonEndSpec>;
#[doc = "performance monitor write end address"]
pub mod grf_a72_perf_wr_mon_end;
#[doc = "GRF_A72_PERF_RD_MAX_LATENCY_NUM (rw) register accessor: a72 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a72_perf_rd_max_latency_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a72_perf_rd_max_latency_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a72_perf_rd_max_latency_num`]
module"]
#[doc(alias = "GRF_A72_PERF_RD_MAX_LATENCY_NUM")]
pub type GrfA72PerfRdMaxLatencyNum =
    crate::Reg<grf_a72_perf_rd_max_latency_num::GrfA72PerfRdMaxLatencyNumSpec>;
#[doc = "a72 performance monitor status register"]
pub mod grf_a72_perf_rd_max_latency_num;
#[doc = "GRF_A72_PERF_RD_LATENCY_SAMP_NUM (rw) register accessor: a72 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a72_perf_rd_latency_samp_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a72_perf_rd_latency_samp_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a72_perf_rd_latency_samp_num`]
module"]
#[doc(alias = "GRF_A72_PERF_RD_LATENCY_SAMP_NUM")]
pub type GrfA72PerfRdLatencySampNum =
    crate::Reg<grf_a72_perf_rd_latency_samp_num::GrfA72PerfRdLatencySampNumSpec>;
#[doc = "a72 performance monitor status register"]
pub mod grf_a72_perf_rd_latency_samp_num;
#[doc = "GRF_A72_PERF_RD_LATENCY_ACC_NUM (rw) register accessor: a72 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a72_perf_rd_latency_acc_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a72_perf_rd_latency_acc_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a72_perf_rd_latency_acc_num`]
module"]
#[doc(alias = "GRF_A72_PERF_RD_LATENCY_ACC_NUM")]
pub type GrfA72PerfRdLatencyAccNum =
    crate::Reg<grf_a72_perf_rd_latency_acc_num::GrfA72PerfRdLatencyAccNumSpec>;
#[doc = "a72 performance monitor status register"]
pub mod grf_a72_perf_rd_latency_acc_num;
#[doc = "GRF_A72_PERF_RD_AXI_TOTAL_BYTE (rw) register accessor: a72 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a72_perf_rd_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a72_perf_rd_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a72_perf_rd_axi_total_byte`]
module"]
#[doc(alias = "GRF_A72_PERF_RD_AXI_TOTAL_BYTE")]
pub type GrfA72PerfRdAxiTotalByte =
    crate::Reg<grf_a72_perf_rd_axi_total_byte::GrfA72PerfRdAxiTotalByteSpec>;
#[doc = "a72 performance monitor status register"]
pub mod grf_a72_perf_rd_axi_total_byte;
#[doc = "GRF_A72_PERF_WR_AXI_TOTAL_BYTE (rw) register accessor: a72 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a72_perf_wr_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a72_perf_wr_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a72_perf_wr_axi_total_byte`]
module"]
#[doc(alias = "GRF_A72_PERF_WR_AXI_TOTAL_BYTE")]
pub type GrfA72PerfWrAxiTotalByte =
    crate::Reg<grf_a72_perf_wr_axi_total_byte::GrfA72PerfWrAxiTotalByteSpec>;
#[doc = "a72 performance monitor status register"]
pub mod grf_a72_perf_wr_axi_total_byte;
#[doc = "GRF_A72_PERF_WORKING_CNT (rw) register accessor: a72 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a72_perf_working_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a72_perf_working_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a72_perf_working_cnt`]
module"]
#[doc(alias = "GRF_A72_PERF_WORKING_CNT")]
pub type GrfA72PerfWorkingCnt = crate::Reg<grf_a72_perf_working_cnt::GrfA72PerfWorkingCntSpec>;
#[doc = "a72 performance monitor status register"]
pub mod grf_a72_perf_working_cnt;
#[doc = "GRF_A72_PERF_INT_STATUS (rw) register accessor: a72 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a72_perf_int_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a72_perf_int_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_a72_perf_int_status`]
module"]
#[doc(alias = "GRF_A72_PERF_INT_STATUS")]
pub type GrfA72PerfIntStatus = crate::Reg<grf_a72_perf_int_status::GrfA72PerfIntStatusSpec>;
#[doc = "a72 performance monitor status register"]
pub mod grf_a72_perf_int_status;
#[doc = "GRF_GMAC_PERF_CON0 (rw) register accessor: gmac performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gmac_perf_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gmac_perf_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gmac_perf_con0`]
module"]
#[doc(alias = "GRF_GMAC_PERF_CON0")]
pub type GrfGmacPerfCon0 = crate::Reg<grf_gmac_perf_con0::GrfGmacPerfCon0Spec>;
#[doc = "gmac performance monitor control register"]
pub mod grf_gmac_perf_con0;
#[doc = "GRF_GMAC_PERF_CON1 (rw) register accessor: gmac performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gmac_perf_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gmac_perf_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gmac_perf_con1`]
module"]
#[doc(alias = "GRF_GMAC_PERF_CON1")]
pub type GrfGmacPerfCon1 = crate::Reg<grf_gmac_perf_con1::GrfGmacPerfCon1Spec>;
#[doc = "gmac performance monitor control register"]
pub mod grf_gmac_perf_con1;
#[doc = "GRF_GMAC_PERF_CON2 (rw) register accessor: gmac performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gmac_perf_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gmac_perf_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gmac_perf_con2`]
module"]
#[doc(alias = "GRF_GMAC_PERF_CON2")]
pub type GrfGmacPerfCon2 = crate::Reg<grf_gmac_perf_con2::GrfGmacPerfCon2Spec>;
#[doc = "gmac performance monitor control register"]
pub mod grf_gmac_perf_con2;
#[doc = "GRF_GMAC_PERF_RD_MAX_LATENCY_NUM (rw) register accessor: gmac performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gmac_perf_rd_max_latency_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gmac_perf_rd_max_latency_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gmac_perf_rd_max_latency_num`]
module"]
#[doc(alias = "GRF_GMAC_PERF_RD_MAX_LATENCY_NUM")]
pub type GrfGmacPerfRdMaxLatencyNum =
    crate::Reg<grf_gmac_perf_rd_max_latency_num::GrfGmacPerfRdMaxLatencyNumSpec>;
#[doc = "gmac performance monitor status register"]
pub mod grf_gmac_perf_rd_max_latency_num;
#[doc = "GRF_GMAC_PERF_RD_LATENCY_SAMP_NUM (rw) register accessor: gmac performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gmac_perf_rd_latency_samp_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gmac_perf_rd_latency_samp_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gmac_perf_rd_latency_samp_num`]
module"]
#[doc(alias = "GRF_GMAC_PERF_RD_LATENCY_SAMP_NUM")]
pub type GrfGmacPerfRdLatencySampNum =
    crate::Reg<grf_gmac_perf_rd_latency_samp_num::GrfGmacPerfRdLatencySampNumSpec>;
#[doc = "gmac performance monitor status register"]
pub mod grf_gmac_perf_rd_latency_samp_num;
#[doc = "GRF_GMAC_PERF_RD_LATENCY_ACC_NUM (rw) register accessor: gmac performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gmac_perf_rd_latency_acc_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gmac_perf_rd_latency_acc_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gmac_perf_rd_latency_acc_num`]
module"]
#[doc(alias = "GRF_GMAC_PERF_RD_LATENCY_ACC_NUM")]
pub type GrfGmacPerfRdLatencyAccNum =
    crate::Reg<grf_gmac_perf_rd_latency_acc_num::GrfGmacPerfRdLatencyAccNumSpec>;
#[doc = "gmac performance monitor status register"]
pub mod grf_gmac_perf_rd_latency_acc_num;
#[doc = "GRF_GMAC_PERF_RD_AXI_TOTAL_BYTE (rw) register accessor: gmac performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gmac_perf_rd_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gmac_perf_rd_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gmac_perf_rd_axi_total_byte`]
module"]
#[doc(alias = "GRF_GMAC_PERF_RD_AXI_TOTAL_BYTE")]
pub type GrfGmacPerfRdAxiTotalByte =
    crate::Reg<grf_gmac_perf_rd_axi_total_byte::GrfGmacPerfRdAxiTotalByteSpec>;
#[doc = "gmac performance monitor status register"]
pub mod grf_gmac_perf_rd_axi_total_byte;
#[doc = "GRF_GMAC_PERF_WR_AXI_TOTAL_BYTE (rw) register accessor: gmac performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gmac_perf_wr_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gmac_perf_wr_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gmac_perf_wr_axi_total_byte`]
module"]
#[doc(alias = "GRF_GMAC_PERF_WR_AXI_TOTAL_BYTE")]
pub type GrfGmacPerfWrAxiTotalByte =
    crate::Reg<grf_gmac_perf_wr_axi_total_byte::GrfGmacPerfWrAxiTotalByteSpec>;
#[doc = "gmac performance monitor status register"]
pub mod grf_gmac_perf_wr_axi_total_byte;
#[doc = "GRF_GMAC_PERF_WORKING_CNT (rw) register accessor: gmac performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gmac_perf_working_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gmac_perf_working_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gmac_perf_working_cnt`]
module"]
#[doc(alias = "GRF_GMAC_PERF_WORKING_CNT")]
pub type GrfGmacPerfWorkingCnt = crate::Reg<grf_gmac_perf_working_cnt::GrfGmacPerfWorkingCntSpec>;
#[doc = "gmac performance monitor status register"]
pub mod grf_gmac_perf_working_cnt;
#[doc = "GRF_SOC_CON5 (rw) register accessor: SoC control register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con5`]
module"]
#[doc(alias = "GRF_SOC_CON5")]
pub type GrfSocCon5 = crate::Reg<grf_soc_con5::GrfSocCon5Spec>;
#[doc = "SoC control register 5"]
pub mod grf_soc_con5;
#[doc = "GRF_SOC_CON6 (rw) register accessor: SoC control register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con6`]
module"]
#[doc(alias = "GRF_SOC_CON6")]
pub type GrfSocCon6 = crate::Reg<grf_soc_con6::GrfSocCon6Spec>;
#[doc = "SoC control register 6"]
pub mod grf_soc_con6;
#[doc = "GRF_GPIO2A_IOMUX (rw) register accessor: GPIO2A iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2a_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2a_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2a_iomux`]
module"]
#[doc(alias = "GRF_GPIO2A_IOMUX")]
pub type GrfGpio2aIomux = crate::Reg<grf_gpio2a_iomux::GrfGpio2aIomuxSpec>;
#[doc = "GPIO2A iomux control"]
pub mod grf_gpio2a_iomux;
#[doc = "GRF_GPIO2B_IOMUX (rw) register accessor: GPIO2B iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2b_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2b_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2b_iomux`]
module"]
#[doc(alias = "GRF_GPIO2B_IOMUX")]
pub type GrfGpio2bIomux = crate::Reg<grf_gpio2b_iomux::GrfGpio2bIomuxSpec>;
#[doc = "GPIO2B iomux control"]
pub mod grf_gpio2b_iomux;
#[doc = "GRF_GPIO2C_IOMUX (rw) register accessor: GPIO2C iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2c_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2c_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2c_iomux`]
module"]
#[doc(alias = "GRF_GPIO2C_IOMUX")]
pub type GrfGpio2cIomux = crate::Reg<grf_gpio2c_iomux::GrfGpio2cIomuxSpec>;
#[doc = "GPIO2C iomux control"]
pub mod grf_gpio2c_iomux;
#[doc = "GRF_GPIO2D_IOMUX (rw) register accessor: GPIO2D iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2d_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2d_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2d_iomux`]
module"]
#[doc(alias = "GRF_GPIO2D_IOMUX")]
pub type GrfGpio2dIomux = crate::Reg<grf_gpio2d_iomux::GrfGpio2dIomuxSpec>;
#[doc = "GPIO2D iomux control"]
pub mod grf_gpio2d_iomux;
#[doc = "GRF_GPIO3A_IOMUX (rw) register accessor: GPIO3A iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3a_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3a_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3a_iomux`]
module"]
#[doc(alias = "GRF_GPIO3A_IOMUX")]
pub type GrfGpio3aIomux = crate::Reg<grf_gpio3a_iomux::GrfGpio3aIomuxSpec>;
#[doc = "GPIO3A iomux control"]
pub mod grf_gpio3a_iomux;
#[doc = "GRF_GPIO3B_IOMUX (rw) register accessor: GPIO3B iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3b_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3b_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3b_iomux`]
module"]
#[doc(alias = "GRF_GPIO3B_IOMUX")]
pub type GrfGpio3bIomux = crate::Reg<grf_gpio3b_iomux::GrfGpio3bIomuxSpec>;
#[doc = "GPIO3B iomux control"]
pub mod grf_gpio3b_iomux;
#[doc = "GRF_GPIO3C_IOMUX (rw) register accessor: GPIO3C iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3c_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3c_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3c_iomux`]
module"]
#[doc(alias = "GRF_GPIO3C_IOMUX")]
pub type GrfGpio3cIomux = crate::Reg<grf_gpio3c_iomux::GrfGpio3cIomuxSpec>;
#[doc = "GPIO3C iomux control"]
pub mod grf_gpio3c_iomux;
#[doc = "GRF_GPIO3D_IOMUX (rw) register accessor: GPIO3D iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3d_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3d_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3d_iomux`]
module"]
#[doc(alias = "GRF_GPIO3D_IOMUX")]
pub type GrfGpio3dIomux = crate::Reg<grf_gpio3d_iomux::GrfGpio3dIomuxSpec>;
#[doc = "GPIO3D iomux control"]
pub mod grf_gpio3d_iomux;
#[doc = "GRF_GPIO4A_IOMUX (rw) register accessor: GPIO4A iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4a_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4a_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4a_iomux`]
module"]
#[doc(alias = "GRF_GPIO4A_IOMUX")]
pub type GrfGpio4aIomux = crate::Reg<grf_gpio4a_iomux::GrfGpio4aIomuxSpec>;
#[doc = "GPIO4A iomux control"]
pub mod grf_gpio4a_iomux;
#[doc = "GRF_GPIO4B_IOMUX (rw) register accessor: GPIO4B iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4b_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4b_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4b_iomux`]
module"]
#[doc(alias = "GRF_GPIO4B_IOMUX")]
pub type GrfGpio4bIomux = crate::Reg<grf_gpio4b_iomux::GrfGpio4bIomuxSpec>;
#[doc = "GPIO4B iomux control"]
pub mod grf_gpio4b_iomux;
#[doc = "GRF_GPIO4C_IOMUX (rw) register accessor: GPIO4C iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4c_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4c_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4c_iomux`]
module"]
#[doc(alias = "GRF_GPIO4C_IOMUX")]
pub type GrfGpio4cIomux = crate::Reg<grf_gpio4c_iomux::GrfGpio4cIomuxSpec>;
#[doc = "GPIO4C iomux control"]
pub mod grf_gpio4c_iomux;
#[doc = "GRF_GPIO4D_IOMUX (rw) register accessor: GPIO4D iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4d_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4d_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4d_iomux`]
module"]
#[doc(alias = "GRF_GPIO4D_IOMUX")]
pub type GrfGpio4dIomux = crate::Reg<grf_gpio4d_iomux::GrfGpio4dIomuxSpec>;
#[doc = "GPIO4D iomux control"]
pub mod grf_gpio4d_iomux;
#[doc = "GRF_GPIO2A_P (rw) register accessor: GPIO2A PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2a_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2a_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2a_p`]
module"]
#[doc(alias = "GRF_GPIO2A_P")]
pub type GrfGpio2aP = crate::Reg<grf_gpio2a_p::GrfGpio2aPSpec>;
#[doc = "GPIO2A PU/PD control"]
pub mod grf_gpio2a_p;
#[doc = "GRF_GPIO2B_P (rw) register accessor: GPIO2B PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2b_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2b_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2b_p`]
module"]
#[doc(alias = "GRF_GPIO2B_P")]
pub type GrfGpio2bP = crate::Reg<grf_gpio2b_p::GrfGpio2bPSpec>;
#[doc = "GPIO2B PU/PD control"]
pub mod grf_gpio2b_p;
#[doc = "GRF_GPIO2C_P (rw) register accessor: GPIO2C PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2c_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2c_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2c_p`]
module"]
#[doc(alias = "GRF_GPIO2C_P")]
pub type GrfGpio2cP = crate::Reg<grf_gpio2c_p::GrfGpio2cPSpec>;
#[doc = "GPIO2C PU/PD control"]
pub mod grf_gpio2c_p;
#[doc = "GRF_GPIO2D_P (rw) register accessor: GPIO2D PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2d_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2d_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2d_p`]
module"]
#[doc(alias = "GRF_GPIO2D_P")]
pub type GrfGpio2dP = crate::Reg<grf_gpio2d_p::GrfGpio2dPSpec>;
#[doc = "GPIO2D PU/PD control"]
pub mod grf_gpio2d_p;
#[doc = "GRF_GPIO3A_P (rw) register accessor: GPIO3A PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3a_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3a_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3a_p`]
module"]
#[doc(alias = "GRF_GPIO3A_P")]
pub type GrfGpio3aP = crate::Reg<grf_gpio3a_p::GrfGpio3aPSpec>;
#[doc = "GPIO3A PU/PD control"]
pub mod grf_gpio3a_p;
#[doc = "GRF_GPIO3B_P (rw) register accessor: GPIO3B PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3b_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3b_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3b_p`]
module"]
#[doc(alias = "GRF_GPIO3B_P")]
pub type GrfGpio3bP = crate::Reg<grf_gpio3b_p::GrfGpio3bPSpec>;
#[doc = "GPIO3B PU/PD control"]
pub mod grf_gpio3b_p;
#[doc = "GRF_GPIO3C_P (rw) register accessor: GPIO3C PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3c_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3c_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3c_p`]
module"]
#[doc(alias = "GRF_GPIO3C_P")]
pub type GrfGpio3cP = crate::Reg<grf_gpio3c_p::GrfGpio3cPSpec>;
#[doc = "GPIO3C PU/PD control"]
pub mod grf_gpio3c_p;
#[doc = "GRF_GPIO3D_P (rw) register accessor: GPIO3D PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3d_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3d_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3d_p`]
module"]
#[doc(alias = "GRF_GPIO3D_P")]
pub type GrfGpio3dP = crate::Reg<grf_gpio3d_p::GrfGpio3dPSpec>;
#[doc = "GPIO3D PU/PD control"]
pub mod grf_gpio3d_p;
#[doc = "GRF_GPIO4A_P (rw) register accessor: GPIO4A PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4a_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4a_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4a_p`]
module"]
#[doc(alias = "GRF_GPIO4A_P")]
pub type GrfGpio4aP = crate::Reg<grf_gpio4a_p::GrfGpio4aPSpec>;
#[doc = "GPIO4A PU/PD control"]
pub mod grf_gpio4a_p;
#[doc = "GRF_GPIO4B_P (rw) register accessor: GPIO4B PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4b_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4b_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4b_p`]
module"]
#[doc(alias = "GRF_GPIO4B_P")]
pub type GrfGpio4bP = crate::Reg<grf_gpio4b_p::GrfGpio4bPSpec>;
#[doc = "GPIO4B PU/PD control"]
pub mod grf_gpio4b_p;
#[doc = "GRF_GPIO4C_P (rw) register accessor: GPIO4C PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4c_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4c_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4c_p`]
module"]
#[doc(alias = "GRF_GPIO4C_P")]
pub type GrfGpio4cP = crate::Reg<grf_gpio4c_p::GrfGpio4cPSpec>;
#[doc = "GPIO4C PU/PD control"]
pub mod grf_gpio4c_p;
#[doc = "GRF_GPIO4D_P (rw) register accessor: GPIO4D PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4d_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4d_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4d_p`]
module"]
#[doc(alias = "GRF_GPIO4D_P")]
pub type GrfGpio4dP = crate::Reg<grf_gpio4d_p::GrfGpio4dPSpec>;
#[doc = "GPIO4D PU/PD control"]
pub mod grf_gpio4d_p;
#[doc = "GRF_GPIO2A_SR (rw) register accessor: GPIO2A slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2a_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2a_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2a_sr`]
module"]
#[doc(alias = "GRF_GPIO2A_SR")]
pub type GrfGpio2aSr = crate::Reg<grf_gpio2a_sr::GrfGpio2aSrSpec>;
#[doc = "GPIO2A slew rate control"]
pub mod grf_gpio2a_sr;
#[doc = "GRF_GPIO2B_SR (rw) register accessor: GPIO2B slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2b_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2b_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2b_sr`]
module"]
#[doc(alias = "GRF_GPIO2B_SR")]
pub type GrfGpio2bSr = crate::Reg<grf_gpio2b_sr::GrfGpio2bSrSpec>;
#[doc = "GPIO2B slew rate control"]
pub mod grf_gpio2b_sr;
#[doc = "GRF_GPIO2C_SR (rw) register accessor: GPIO2C slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2c_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2c_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2c_sr`]
module"]
#[doc(alias = "GRF_GPIO2C_SR")]
pub type GrfGpio2cSr = crate::Reg<grf_gpio2c_sr::GrfGpio2cSrSpec>;
#[doc = "GPIO2C slew rate control"]
pub mod grf_gpio2c_sr;
#[doc = "GRF_GPIO2D_SR (rw) register accessor: GPIO2D slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2d_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2d_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2d_sr`]
module"]
#[doc(alias = "GRF_GPIO2D_SR")]
pub type GrfGpio2dSr = crate::Reg<grf_gpio2d_sr::GrfGpio2dSrSpec>;
#[doc = "GPIO2D slew rate control"]
pub mod grf_gpio2d_sr;
#[doc = "GRF_GPIO3D_SR (rw) register accessor: GPIO3D slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3d_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3d_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3d_sr`]
module"]
#[doc(alias = "GRF_GPIO3D_SR")]
pub type GrfGpio3dSr = crate::Reg<grf_gpio3d_sr::GrfGpio3dSrSpec>;
#[doc = "GPIO3D slew rate control"]
pub mod grf_gpio3d_sr;
#[doc = "GRF_GPIO4A_SR (rw) register accessor: GPIO4A slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4a_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4a_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4a_sr`]
module"]
#[doc(alias = "GRF_GPIO4A_SR")]
pub type GrfGpio4aSr = crate::Reg<grf_gpio4a_sr::GrfGpio4aSrSpec>;
#[doc = "GPIO4A slew rate control"]
pub mod grf_gpio4a_sr;
#[doc = "GRF_GPIO4B_SR (rw) register accessor: GPIO4B slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4b_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4b_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4b_sr`]
module"]
#[doc(alias = "GRF_GPIO4B_SR")]
pub type GrfGpio4bSr = crate::Reg<grf_gpio4b_sr::GrfGpio4bSrSpec>;
#[doc = "GPIO4B slew rate control"]
pub mod grf_gpio4b_sr;
#[doc = "GRF_GPIO4C_SR (rw) register accessor: GPIO4C slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4c_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4c_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4c_sr`]
module"]
#[doc(alias = "GRF_GPIO4C_SR")]
pub type GrfGpio4cSr = crate::Reg<grf_gpio4c_sr::GrfGpio4cSrSpec>;
#[doc = "GPIO4C slew rate control"]
pub mod grf_gpio4c_sr;
#[doc = "GRF_GPIO4D_SR (rw) register accessor: GPIO4D slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4d_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4d_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4d_sr`]
module"]
#[doc(alias = "GRF_GPIO4D_SR")]
pub type GrfGpio4dSr = crate::Reg<grf_gpio4d_sr::GrfGpio4dSrSpec>;
#[doc = "GPIO4D slew rate control"]
pub mod grf_gpio4d_sr;
#[doc = "GRF_GPIO2A_SMT (rw) register accessor: GPIO2A smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2a_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2a_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2a_smt`]
module"]
#[doc(alias = "GRF_GPIO2A_SMT")]
pub type GrfGpio2aSmt = crate::Reg<grf_gpio2a_smt::GrfGpio2aSmtSpec>;
#[doc = "GPIO2A smitter control register"]
pub mod grf_gpio2a_smt;
#[doc = "GRF_GPIO2B_SMT (rw) register accessor: GPIO2B smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2b_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2b_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2b_smt`]
module"]
#[doc(alias = "GRF_GPIO2B_SMT")]
pub type GrfGpio2bSmt = crate::Reg<grf_gpio2b_smt::GrfGpio2bSmtSpec>;
#[doc = "GPIO2B smitter control register"]
pub mod grf_gpio2b_smt;
#[doc = "GRF_GPIO2C_SMT (rw) register accessor: GPIO2C smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2c_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2c_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2c_smt`]
module"]
#[doc(alias = "GRF_GPIO2C_SMT")]
pub type GrfGpio2cSmt = crate::Reg<grf_gpio2c_smt::GrfGpio2cSmtSpec>;
#[doc = "GPIO2C smitter control register"]
pub mod grf_gpio2c_smt;
#[doc = "GRF_GPIO2D_SMT (rw) register accessor: GPIO2D smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2d_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2d_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2d_smt`]
module"]
#[doc(alias = "GRF_GPIO2D_SMT")]
pub type GrfGpio2dSmt = crate::Reg<grf_gpio2d_smt::GrfGpio2dSmtSpec>;
#[doc = "GPIO2D smitter control register"]
pub mod grf_gpio2d_smt;
#[doc = "GRF_GPIO3A_SMT (rw) register accessor: GPIO3A smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3a_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3a_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3a_smt`]
module"]
#[doc(alias = "GRF_GPIO3A_SMT")]
pub type GrfGpio3aSmt = crate::Reg<grf_gpio3a_smt::GrfGpio3aSmtSpec>;
#[doc = "GPIO3A smitter control register"]
pub mod grf_gpio3a_smt;
#[doc = "GRF_GPIO3B_SMT (rw) register accessor: GPIO3B smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3b_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3b_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3b_smt`]
module"]
#[doc(alias = "GRF_GPIO3B_SMT")]
pub type GrfGpio3bSmt = crate::Reg<grf_gpio3b_smt::GrfGpio3bSmtSpec>;
#[doc = "GPIO3B smitter control register"]
pub mod grf_gpio3b_smt;
#[doc = "GRF_GPIO3C_SMT (rw) register accessor: GPIO3C smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3c_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3c_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3c_smt`]
module"]
#[doc(alias = "GRF_GPIO3C_SMT")]
pub type GrfGpio3cSmt = crate::Reg<grf_gpio3c_smt::GrfGpio3cSmtSpec>;
#[doc = "GPIO3C smitter control register"]
pub mod grf_gpio3c_smt;
#[doc = "GRF_GPIO3D_SMT (rw) register accessor: GPIO3D smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3d_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3d_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3d_smt`]
module"]
#[doc(alias = "GRF_GPIO3D_SMT")]
pub type GrfGpio3dSmt = crate::Reg<grf_gpio3d_smt::GrfGpio3dSmtSpec>;
#[doc = "GPIO3D smitter control register"]
pub mod grf_gpio3d_smt;
#[doc = "GRF_GPIO4A_SMT (rw) register accessor: GPIO4A smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4a_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4a_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4a_smt`]
module"]
#[doc(alias = "GRF_GPIO4A_SMT")]
pub type GrfGpio4aSmt = crate::Reg<grf_gpio4a_smt::GrfGpio4aSmtSpec>;
#[doc = "GPIO4A smitter control register"]
pub mod grf_gpio4a_smt;
#[doc = "GRF_GPIO4B_SMT (rw) register accessor: GPIO4B smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4b_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4b_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4b_smt`]
module"]
#[doc(alias = "GRF_GPIO4B_SMT")]
pub type GrfGpio4bSmt = crate::Reg<grf_gpio4b_smt::GrfGpio4bSmtSpec>;
#[doc = "GPIO4B smitter control register"]
pub mod grf_gpio4b_smt;
#[doc = "GRF_GPIO4C_SMT (rw) register accessor: GPIO4C smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4c_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4c_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4c_smt`]
module"]
#[doc(alias = "GRF_GPIO4C_SMT")]
pub type GrfGpio4cSmt = crate::Reg<grf_gpio4c_smt::GrfGpio4cSmtSpec>;
#[doc = "GPIO4C smitter control register"]
pub mod grf_gpio4c_smt;
#[doc = "GRF_GPIO4D_SMT (rw) register accessor: GPIO4D smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4d_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4d_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4d_smt`]
module"]
#[doc(alias = "GRF_GPIO4D_SMT")]
pub type GrfGpio4dSmt = crate::Reg<grf_gpio4d_smt::GrfGpio4dSmtSpec>;
#[doc = "GPIO4D smitter control register"]
pub mod grf_gpio4d_smt;
#[doc = "GRF_GPIO2A_E (rw) register accessor: GPIO2A drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2a_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2a_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2a_e`]
module"]
#[doc(alias = "GRF_GPIO2A_E")]
pub type GrfGpio2aE = crate::Reg<grf_gpio2a_e::GrfGpio2aESpec>;
#[doc = "GPIO2A drive strength control"]
pub mod grf_gpio2a_e;
#[doc = "GRF_GPIO2B_E (rw) register accessor: GPIO2B drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2b_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2b_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2b_e`]
module"]
#[doc(alias = "GRF_GPIO2B_E")]
pub type GrfGpio2bE = crate::Reg<grf_gpio2b_e::GrfGpio2bESpec>;
#[doc = "GPIO2B drive strength control"]
pub mod grf_gpio2b_e;
#[doc = "GRF_GPIO2C_E (rw) register accessor: GPIO2C drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2c_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2c_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2c_e`]
module"]
#[doc(alias = "GRF_GPIO2C_E")]
pub type GrfGpio2cE = crate::Reg<grf_gpio2c_e::GrfGpio2cESpec>;
#[doc = "GPIO2C drive strength control"]
pub mod grf_gpio2c_e;
#[doc = "GRF_GPIO2D_E (rw) register accessor: GPIO2D drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2d_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2d_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2d_e`]
module"]
#[doc(alias = "GRF_GPIO2D_E")]
pub type GrfGpio2dE = crate::Reg<grf_gpio2d_e::GrfGpio2dESpec>;
#[doc = "GPIO2D drive strength control"]
pub mod grf_gpio2d_e;
#[doc = "GRF_GPIO3A_E01 (rw) register accessor: GPIO3A drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3a_e01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3a_e01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3a_e01`]
module"]
#[doc(alias = "GRF_GPIO3A_E01")]
pub type GrfGpio3aE01 = crate::Reg<grf_gpio3a_e01::GrfGpio3aE01Spec>;
#[doc = "GPIO3A drive strength control"]
pub mod grf_gpio3a_e01;
#[doc = "GRF_GPIO3A_E2 (rw) register accessor: GPIO3B drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3a_e2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3a_e2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3a_e2`]
module"]
#[doc(alias = "GRF_GPIO3A_E2")]
pub type GrfGpio3aE2 = crate::Reg<grf_gpio3a_e2::GrfGpio3aE2Spec>;
#[doc = "GPIO3B drive strength control"]
pub mod grf_gpio3a_e2;
#[doc = "GRF_GPIO3B_E01 (rw) register accessor: GPIO3B drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3b_e01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3b_e01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3b_e01`]
module"]
#[doc(alias = "GRF_GPIO3B_E01")]
pub type GrfGpio3bE01 = crate::Reg<grf_gpio3b_e01::GrfGpio3bE01Spec>;
#[doc = "GPIO3B drive strength control"]
pub mod grf_gpio3b_e01;
#[doc = "GRF_GPIO3B_E2 (rw) register accessor: GPIO3B drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3b_e2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3b_e2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3b_e2`]
module"]
#[doc(alias = "GRF_GPIO3B_E2")]
pub type GrfGpio3bE2 = crate::Reg<grf_gpio3b_e2::GrfGpio3bE2Spec>;
#[doc = "GPIO3B drive strength control"]
pub mod grf_gpio3b_e2;
#[doc = "GRF_GPIO3C_E01 (rw) register accessor: GPIO3C drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3c_e01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3c_e01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3c_e01`]
module"]
#[doc(alias = "GRF_GPIO3C_E01")]
pub type GrfGpio3cE01 = crate::Reg<grf_gpio3c_e01::GrfGpio3cE01Spec>;
#[doc = "GPIO3C drive strength control"]
pub mod grf_gpio3c_e01;
#[doc = "GRF_GPIO3C_E2 (rw) register accessor: GPIO3C drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3c_e2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3c_e2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3c_e2`]
module"]
#[doc(alias = "GRF_GPIO3C_E2")]
pub type GrfGpio3cE2 = crate::Reg<grf_gpio3c_e2::GrfGpio3cE2Spec>;
#[doc = "GPIO3C drive strength control"]
pub mod grf_gpio3c_e2;
#[doc = "GRF_GPIO3D_E (rw) register accessor: GPIO3D drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3d_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3d_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio3d_e`]
module"]
#[doc(alias = "GRF_GPIO3D_E")]
pub type GrfGpio3dE = crate::Reg<grf_gpio3d_e::GrfGpio3dESpec>;
#[doc = "GPIO3D drive strength control"]
pub mod grf_gpio3d_e;
#[doc = "GRF_GPIO4A_E (rw) register accessor: GPIO4A drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4a_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4a_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4a_e`]
module"]
#[doc(alias = "GRF_GPIO4A_E")]
pub type GrfGpio4aE = crate::Reg<grf_gpio4a_e::GrfGpio4aESpec>;
#[doc = "GPIO4A drive strength control"]
pub mod grf_gpio4a_e;
#[doc = "GRF_GPIO4B_E01 (rw) register accessor: GPIO4B drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4b_e01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4b_e01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4b_e01`]
module"]
#[doc(alias = "GRF_GPIO4B_E01")]
pub type GrfGpio4bE01 = crate::Reg<grf_gpio4b_e01::GrfGpio4bE01Spec>;
#[doc = "GPIO4B drive strength control"]
pub mod grf_gpio4b_e01;
#[doc = "GRF_GPIO4B_E2 (rw) register accessor: GPIO4B drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4b_e2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4b_e2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4b_e2`]
module"]
#[doc(alias = "GRF_GPIO4B_E2")]
pub type GrfGpio4bE2 = crate::Reg<grf_gpio4b_e2::GrfGpio4bE2Spec>;
#[doc = "GPIO4B drive strength control"]
pub mod grf_gpio4b_e2;
#[doc = "GRF_GPIO4C_E (rw) register accessor: GPIO4C drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4c_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4c_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4c_e`]
module"]
#[doc(alias = "GRF_GPIO4C_E")]
pub type GrfGpio4cE = crate::Reg<grf_gpio4c_e::GrfGpio4cESpec>;
#[doc = "GPIO4C drive strength control"]
pub mod grf_gpio4c_e;
#[doc = "GRF_GPIO4D_E (rw) register accessor: GPIO4D drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4d_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4d_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio4d_e`]
module"]
#[doc(alias = "GRF_GPIO4D_E")]
pub type GrfGpio4dE = crate::Reg<grf_gpio4d_e::GrfGpio4dESpec>;
#[doc = "GPIO4D drive strength control"]
pub mod grf_gpio4d_e;
#[doc = "GRF_GPIO2C_HE (rw) register accessor: GPIO2C HE control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2c_he::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2c_he::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2c_he`]
module"]
#[doc(alias = "GRF_GPIO2C_HE")]
pub type GrfGpio2cHe = crate::Reg<grf_gpio2c_he::GrfGpio2cHeSpec>;
#[doc = "GPIO2C HE control"]
pub mod grf_gpio2c_he;
#[doc = "GRF_GPIO2D_HE (rw) register accessor: GPIO2D HE control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2d_he::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2d_he::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_gpio2d_he`]
module"]
#[doc(alias = "GRF_GPIO2D_HE")]
pub type GrfGpio2dHe = crate::Reg<grf_gpio2d_he::GrfGpio2dHeSpec>;
#[doc = "GPIO2D HE control"]
pub mod grf_gpio2d_he;
#[doc = "GRF_SOC_CON0 (rw) register accessor: SoC control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con0`]
module"]
#[doc(alias = "GRF_SOC_CON0")]
pub type GrfSocCon0 = crate::Reg<grf_soc_con0::GrfSocCon0Spec>;
#[doc = "SoC control register 0"]
pub mod grf_soc_con0;
#[doc = "GRF_SOC_CON1 (rw) register accessor: SoC control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con1`]
module"]
#[doc(alias = "GRF_SOC_CON1")]
pub type GrfSocCon1 = crate::Reg<grf_soc_con1::GrfSocCon1Spec>;
#[doc = "SoC control register 2"]
pub mod grf_soc_con1;
#[doc = "GRF_SOC_CON2 (rw) register accessor: SoC control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con2`]
module"]
#[doc(alias = "GRF_SOC_CON2")]
pub type GrfSocCon2 = crate::Reg<grf_soc_con2::GrfSocCon2Spec>;
#[doc = "SoC control register 1"]
pub mod grf_soc_con2;
#[doc = "GRF_SOC_CON3 (rw) register accessor: SoC control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con3`]
module"]
#[doc(alias = "GRF_SOC_CON3")]
pub type GrfSocCon3 = crate::Reg<grf_soc_con3::GrfSocCon3Spec>;
#[doc = "SoC control register 3"]
pub mod grf_soc_con3;
#[doc = "GRF_SOC_CON4 (rw) register accessor: SoC control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con4`]
module"]
#[doc(alias = "GRF_SOC_CON4")]
pub type GrfSocCon4 = crate::Reg<grf_soc_con4::GrfSocCon4Spec>;
#[doc = "SoC control register 4"]
pub mod grf_soc_con4;
#[doc = "GRF_SOC_CON_5_PCIE (rw) register accessor: SoC control register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con_5_pcie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con_5_pcie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con_5_pcie`]
module"]
#[doc(alias = "GRF_SOC_CON_5_PCIE")]
pub type GrfSocCon5Pcie = crate::Reg<grf_soc_con_5_pcie::GrfSocCon5PcieSpec>;
#[doc = "SoC control register 5"]
pub mod grf_soc_con_5_pcie;
#[doc = "GRF_SOC_CON7 (rw) register accessor: SoC control register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con7`]
module"]
#[doc(alias = "GRF_SOC_CON7")]
pub type GrfSocCon7 = crate::Reg<grf_soc_con7::GrfSocCon7Spec>;
#[doc = "SoC control register 7"]
pub mod grf_soc_con7;
#[doc = "GRF_SOC_CON8 (rw) register accessor: SoC control register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con8`]
module"]
#[doc(alias = "GRF_SOC_CON8")]
pub type GrfSocCon8 = crate::Reg<grf_soc_con8::GrfSocCon8Spec>;
#[doc = "SoC control register 8"]
pub mod grf_soc_con8;
#[doc = "GRF_SOC_CON_9_PCIE (rw) register accessor: SoC control register 9 for PCIE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con_9_pcie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con_9_pcie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_con_9_pcie`]
module"]
#[doc(alias = "GRF_SOC_CON_9_PCIE")]
pub type GrfSocCon9Pcie = crate::Reg<grf_soc_con_9_pcie::GrfSocCon9PcieSpec>;
#[doc = "SoC control register 9 for PCIE"]
pub mod grf_soc_con_9_pcie;
#[doc = "GRF_SOC_STATUS0 (rw) register accessor: SOC status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_status0`]
module"]
#[doc(alias = "GRF_SOC_STATUS0")]
pub type GrfSocStatus0 = crate::Reg<grf_soc_status0::GrfSocStatus0Spec>;
#[doc = "SOC status register 0"]
pub mod grf_soc_status0;
#[doc = "GRF_SOC_STATUS1 (rw) register accessor: SOC status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_status1`]
module"]
#[doc(alias = "GRF_SOC_STATUS1")]
pub type GrfSocStatus1 = crate::Reg<grf_soc_status1::GrfSocStatus1Spec>;
#[doc = "SOC status register 1"]
pub mod grf_soc_status1;
#[doc = "GRF_SOC_STATUS2 (rw) register accessor: SOC status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_status2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_status2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_status2`]
module"]
#[doc(alias = "GRF_SOC_STATUS2")]
pub type GrfSocStatus2 = crate::Reg<grf_soc_status2::GrfSocStatus2Spec>;
#[doc = "SOC status register 2"]
pub mod grf_soc_status2;
#[doc = "GRF_SOC_STATUS3 (rw) register accessor: SOC status register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_status3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_status3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_status3`]
module"]
#[doc(alias = "GRF_SOC_STATUS3")]
pub type GrfSocStatus3 = crate::Reg<grf_soc_status3::GrfSocStatus3Spec>;
#[doc = "SOC status register 3"]
pub mod grf_soc_status3;
#[doc = "GRF_SOC_STATUS4 (rw) register accessor: SOC status register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_status4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_status4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_status4`]
module"]
#[doc(alias = "GRF_SOC_STATUS4")]
pub type GrfSocStatus4 = crate::Reg<grf_soc_status4::GrfSocStatus4Spec>;
#[doc = "SOC status register 4"]
pub mod grf_soc_status4;
#[doc = "GRF_SOC_STATUS5 (rw) register accessor: SOC status register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_status5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_status5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_soc_status5`]
module"]
#[doc(alias = "GRF_SOC_STATUS5")]
pub type GrfSocStatus5 = crate::Reg<grf_soc_status5::GrfSocStatus5Spec>;
#[doc = "SOC status register 5"]
pub mod grf_soc_status5;
#[doc = "GRF_DDRC0_CON0 (rw) register accessor: ddrc0 control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_ddrc0_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_ddrc0_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_ddrc0_con0`]
module"]
#[doc(alias = "GRF_DDRC0_CON0")]
pub type GrfDdrc0Con0 = crate::Reg<grf_ddrc0_con0::GrfDdrc0Con0Spec>;
#[doc = "ddrc0 control register 0"]
pub mod grf_ddrc0_con0;
#[doc = "GRF_DDRC0_CON1 (rw) register accessor: ddrc0 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_ddrc0_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_ddrc0_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_ddrc0_con1`]
module"]
#[doc(alias = "GRF_DDRC0_CON1")]
pub type GrfDdrc0Con1 = crate::Reg<grf_ddrc0_con1::GrfDdrc0Con1Spec>;
#[doc = "ddrc0 control register 1"]
pub mod grf_ddrc0_con1;
#[doc = "GRF_DDRC1_CON0 (rw) register accessor: ddrc1 control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_ddrc1_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_ddrc1_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_ddrc1_con0`]
module"]
#[doc(alias = "GRF_DDRC1_CON0")]
pub type GrfDdrc1Con0 = crate::Reg<grf_ddrc1_con0::GrfDdrc1Con0Spec>;
#[doc = "ddrc1 control register 0"]
pub mod grf_ddrc1_con0;
#[doc = "GRF_DDRC1_CON1 (rw) register accessor: ddrc1 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_ddrc1_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_ddrc1_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_ddrc1_con1`]
module"]
#[doc(alias = "GRF_DDRC1_CON1")]
pub type GrfDdrc1Con1 = crate::Reg<grf_ddrc1_con1::GrfDdrc1Con1Spec>;
#[doc = "ddrc1 control register 1"]
pub mod grf_ddrc1_con1;
#[doc = "GRF_SIG_DETECT_CON0 (rw) register accessor: Singal detect control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_sig_detect_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_sig_detect_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_sig_detect_con0`]
module"]
#[doc(alias = "GRF_SIG_DETECT_CON0")]
pub type GrfSigDetectCon0 = crate::Reg<grf_sig_detect_con0::GrfSigDetectCon0Spec>;
#[doc = "Singal detect control register0"]
pub mod grf_sig_detect_con0;
#[doc = "GRF_SIG_DETECT_CON1 (rw) register accessor: Singal detect control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_sig_detect_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_sig_detect_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_sig_detect_con1`]
module"]
#[doc(alias = "GRF_SIG_DETECT_CON1")]
pub type GrfSigDetectCon1 = crate::Reg<grf_sig_detect_con1::GrfSigDetectCon1Spec>;
#[doc = "Singal detect control register1"]
pub mod grf_sig_detect_con1;
#[doc = "GRF_SIG_DETECT_CLR (rw) register accessor: Signal detect status clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_sig_detect_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_sig_detect_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_sig_detect_clr`]
module"]
#[doc(alias = "GRF_SIG_DETECT_CLR")]
pub type GrfSigDetectClr = crate::Reg<grf_sig_detect_clr::GrfSigDetectClrSpec>;
#[doc = "Signal detect status clear register"]
pub mod grf_sig_detect_clr;
#[doc = "GRF_SIG_DETECT_STATUS (rw) register accessor: Signal detect status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_sig_detect_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_sig_detect_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_sig_detect_status`]
module"]
#[doc(alias = "GRF_SIG_DETECT_STATUS")]
pub type GrfSigDetectStatus = crate::Reg<grf_sig_detect_status::GrfSigDetectStatusSpec>;
#[doc = "Signal detect status register"]
pub mod grf_sig_detect_status;
#[doc = "GRF_USB20_PHY0_CON0 (rw) register accessor: USB20 PHY0 GRF Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb20_phy0_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb20_phy0_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb20_phy0_con0`]
module"]
#[doc(alias = "GRF_USB20_PHY0_CON0")]
pub type GrfUsb20Phy0Con0 = crate::Reg<grf_usb20_phy0_con0::GrfUsb20Phy0Con0Spec>;
#[doc = "USB20 PHY0 GRF Register 0"]
pub mod grf_usb20_phy0_con0;
#[doc = "GRF_USB20_PHY0_CON1 (rw) register accessor: USB20 PHY0 GRF Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb20_phy0_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb20_phy0_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb20_phy0_con1`]
module"]
#[doc(alias = "GRF_USB20_PHY0_CON1")]
pub type GrfUsb20Phy0Con1 = crate::Reg<grf_usb20_phy0_con1::GrfUsb20Phy0Con1Spec>;
#[doc = "USB20 PHY0 GRF Register 1"]
pub mod grf_usb20_phy0_con1;
#[doc = "GRF_USB20_PHY0_CON2 (rw) register accessor: USB20 PHY0 GRF Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb20_phy0_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb20_phy0_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb20_phy0_con2`]
module"]
#[doc(alias = "GRF_USB20_PHY0_CON2")]
pub type GrfUsb20Phy0Con2 = crate::Reg<grf_usb20_phy0_con2::GrfUsb20Phy0Con2Spec>;
#[doc = "USB20 PHY0 GRF Register 2"]
pub mod grf_usb20_phy0_con2;
#[doc = "GRF_USB20_PHY0_CON3 (rw) register accessor: USB20 PHY0 GRF Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb20_phy0_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb20_phy0_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb20_phy0_con3`]
module"]
#[doc(alias = "GRF_USB20_PHY0_CON3")]
pub type GrfUsb20Phy0Con3 = crate::Reg<grf_usb20_phy0_con3::GrfUsb20Phy0Con3Spec>;
#[doc = "USB20 PHY0 GRF Register 3"]
pub mod grf_usb20_phy0_con3;
#[doc = "GRF_USB20_PHY1_CON0 (rw) register accessor: USB20 PHY1 GRF Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb20_phy1_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb20_phy1_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb20_phy1_con0`]
module"]
#[doc(alias = "GRF_USB20_PHY1_CON0")]
pub type GrfUsb20Phy1Con0 = crate::Reg<grf_usb20_phy1_con0::GrfUsb20Phy1Con0Spec>;
#[doc = "USB20 PHY1 GRF Register 0"]
pub mod grf_usb20_phy1_con0;
#[doc = "GRF_USB20_PHY1_CON1 (rw) register accessor: USB20 PHY1GRF Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb20_phy1_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb20_phy1_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb20_phy1_con1`]
module"]
#[doc(alias = "GRF_USB20_PHY1_CON1")]
pub type GrfUsb20Phy1Con1 = crate::Reg<grf_usb20_phy1_con1::GrfUsb20Phy1Con1Spec>;
#[doc = "USB20 PHY1GRF Register 1"]
pub mod grf_usb20_phy1_con1;
#[doc = "GRF_USB20_PHY1_CON2 (rw) register accessor: USB20 PHY1 GRF Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb20_phy1_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb20_phy1_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb20_phy1_con2`]
module"]
#[doc(alias = "GRF_USB20_PHY1_CON2")]
pub type GrfUsb20Phy1Con2 = crate::Reg<grf_usb20_phy1_con2::GrfUsb20Phy1Con2Spec>;
#[doc = "USB20 PHY1 GRF Register 2"]
pub mod grf_usb20_phy1_con2;
#[doc = "GRF_USB20_PHY1_CON3 (rw) register accessor: USB20 PHY1 GRF Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb20_phy1_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb20_phy1_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb20_phy1_con3`]
module"]
#[doc(alias = "GRF_USB20_PHY1_CON3")]
pub type GrfUsb20Phy1Con3 = crate::Reg<grf_usb20_phy1_con3::GrfUsb20Phy1Con3Spec>;
#[doc = "USB20 PHY1 GRF Register 3"]
pub mod grf_usb20_phy1_con3;
#[doc = "GRF_USB3PHY0_CON0 (rw) register accessor: TypeC PHY/TCPD PHY/TCPC Control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3phy0_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3phy0_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3phy0_con0`]
module"]
#[doc(alias = "GRF_USB3PHY0_CON0")]
pub type GrfUsb3phy0Con0 = crate::Reg<grf_usb3phy0_con0::GrfUsb3phy0Con0Spec>;
#[doc = "TypeC PHY/TCPD PHY/TCPC Control register0"]
pub mod grf_usb3phy0_con0;
#[doc = "GRF_USB3PHY0_CON1 (rw) register accessor: TypeC PHY/TCPD PHY/TCPC Control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3phy0_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3phy0_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3phy0_con1`]
module"]
#[doc(alias = "GRF_USB3PHY0_CON1")]
pub type GrfUsb3phy0Con1 = crate::Reg<grf_usb3phy0_con1::GrfUsb3phy0Con1Spec>;
#[doc = "TypeC PHY/TCPD PHY/TCPC Control register1"]
pub mod grf_usb3phy0_con1;
#[doc = "GRF_USB3PHY0_CON2 (rw) register accessor: TypeC PHY/TCPD PHY/TCPC Control register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3phy0_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3phy0_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3phy0_con2`]
module"]
#[doc(alias = "GRF_USB3PHY0_CON2")]
pub type GrfUsb3phy0Con2 = crate::Reg<grf_usb3phy0_con2::GrfUsb3phy0Con2Spec>;
#[doc = "TypeC PHY/TCPD PHY/TCPC Control register2"]
pub mod grf_usb3phy0_con2;
#[doc = "GRF_USB3PHY1_CON0 (rw) register accessor: TypeC PHY/TCPD PHY/TCPC Control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3phy1_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3phy1_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3phy1_con0`]
module"]
#[doc(alias = "GRF_USB3PHY1_CON0")]
pub type GrfUsb3phy1Con0 = crate::Reg<grf_usb3phy1_con0::GrfUsb3phy1Con0Spec>;
#[doc = "TypeC PHY/TCPD PHY/TCPC Control register0"]
pub mod grf_usb3phy1_con0;
#[doc = "GRF_USB3PHY1_CON1 (rw) register accessor: TypeC PHY/TCPD PHY/TCPC Control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3phy1_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3phy1_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3phy1_con1`]
module"]
#[doc(alias = "GRF_USB3PHY1_CON1")]
pub type GrfUsb3phy1Con1 = crate::Reg<grf_usb3phy1_con1::GrfUsb3phy1Con1Spec>;
#[doc = "TypeC PHY/TCPD PHY/TCPC Control register1"]
pub mod grf_usb3phy1_con1;
#[doc = "GRF_USB3PHY1_CON2 (rw) register accessor: TypeC PHY/TCPD PHY/TCPC Control register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3phy1_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3phy1_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3phy1_con2`]
module"]
#[doc(alias = "GRF_USB3PHY1_CON2")]
pub type GrfUsb3phy1Con2 = crate::Reg<grf_usb3phy1_con2::GrfUsb3phy1Con2Spec>;
#[doc = "TypeC PHY/TCPD PHY/TCPC Control register2"]
pub mod grf_usb3phy1_con2;
#[doc = "GRF_USB3PHY_STATUS0 (rw) register accessor: USB3PHY_STATUS0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3phy_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3phy_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3phy_status0`]
module"]
#[doc(alias = "GRF_USB3PHY_STATUS0")]
pub type GrfUsb3phyStatus0 = crate::Reg<grf_usb3phy_status0::GrfUsb3phyStatus0Spec>;
#[doc = "USB3PHY_STATUS0"]
pub mod grf_usb3phy_status0;
#[doc = "GRF_USB3PHY_STATUS1 (rw) register accessor: USB3PHY_STATUS1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3phy_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3phy_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usb3phy_status1`]
module"]
#[doc(alias = "GRF_USB3PHY_STATUS1")]
pub type GrfUsb3phyStatus1 = crate::Reg<grf_usb3phy_status1::GrfUsb3phyStatus1Spec>;
#[doc = "USB3PHY_STATUS1"]
pub mod grf_usb3phy_status1;
#[doc = "GRF_DLL_CON0 (rw) register accessor: pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_dll_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_dll_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_dll_con0`]
module"]
#[doc(alias = "GRF_DLL_CON0")]
pub type GrfDllCon0 = crate::Reg<grf_dll_con0::GrfDllCon0Spec>;
#[doc = "pvtm control register"]
pub mod grf_dll_con0;
#[doc = "GRF_DLL_CON1 (rw) register accessor: pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_dll_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_dll_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_dll_con1`]
module"]
#[doc(alias = "GRF_DLL_CON1")]
pub type GrfDllCon1 = crate::Reg<grf_dll_con1::GrfDllCon1Spec>;
#[doc = "pvtm control register"]
pub mod grf_dll_con1;
#[doc = "GRF_DLL_CON2 (rw) register accessor: pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_dll_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_dll_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_dll_con2`]
module"]
#[doc(alias = "GRF_DLL_CON2")]
pub type GrfDllCon2 = crate::Reg<grf_dll_con2::GrfDllCon2Spec>;
#[doc = "pvtm control register"]
pub mod grf_dll_con2;
#[doc = "GRF_DLL_CON3 (rw) register accessor: pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_dll_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_dll_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_dll_con3`]
module"]
#[doc(alias = "GRF_DLL_CON3")]
pub type GrfDllCon3 = crate::Reg<grf_dll_con3::GrfDllCon3Spec>;
#[doc = "pvtm control register"]
pub mod grf_dll_con3;
#[doc = "GRF_DLL_CON4 (rw) register accessor: pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_dll_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_dll_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_dll_con4`]
module"]
#[doc(alias = "GRF_DLL_CON4")]
pub type GrfDllCon4 = crate::Reg<grf_dll_con4::GrfDllCon4Spec>;
#[doc = "pvtm control register"]
pub mod grf_dll_con4;
#[doc = "GRF_DLL_CON5 (rw) register accessor: pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_dll_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_dll_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_dll_con5`]
module"]
#[doc(alias = "GRF_DLL_CON5")]
pub type GrfDllCon5 = crate::Reg<grf_dll_con5::GrfDllCon5Spec>;
#[doc = "pvtm control register"]
pub mod grf_dll_con5;
#[doc = "GRF_DLL_STATUS0 (rw) register accessor: pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_dll_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_dll_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_dll_status0`]
module"]
#[doc(alias = "GRF_DLL_STATUS0")]
pub type GrfDllStatus0 = crate::Reg<grf_dll_status0::GrfDllStatus0Spec>;
#[doc = "pvtm status register"]
pub mod grf_dll_status0;
#[doc = "GRF_DLL_STATUS1 (rw) register accessor: pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_dll_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_dll_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_dll_status1`]
module"]
#[doc(alias = "GRF_DLL_STATUS1")]
pub type GrfDllStatus1 = crate::Reg<grf_dll_status1::GrfDllStatus1Spec>;
#[doc = "pvtm status register"]
pub mod grf_dll_status1;
#[doc = "GRF_DLL_STATUS2 (rw) register accessor: pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_dll_status2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_dll_status2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_dll_status2`]
module"]
#[doc(alias = "GRF_DLL_STATUS2")]
pub type GrfDllStatus2 = crate::Reg<grf_dll_status2::GrfDllStatus2Spec>;
#[doc = "pvtm status register"]
pub mod grf_dll_status2;
#[doc = "GRF_DLL_STATUS3 (rw) register accessor: pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_dll_status3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_dll_status3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_dll_status3`]
module"]
#[doc(alias = "GRF_DLL_STATUS3")]
pub type GrfDllStatus3 = crate::Reg<grf_dll_status3::GrfDllStatus3Spec>;
#[doc = "pvtm status register"]
pub mod grf_dll_status3;
#[doc = "GRF_DLL_STATUS4 (rw) register accessor: pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_dll_status4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_dll_status4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_dll_status4`]
module"]
#[doc(alias = "GRF_DLL_STATUS4")]
pub type GrfDllStatus4 = crate::Reg<grf_dll_status4::GrfDllStatus4Spec>;
#[doc = "pvtm status register"]
pub mod grf_dll_status4;
#[doc = "GRF_IO_VSEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_io_vsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_io_vsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_io_vsel`]
module"]
#[doc(alias = "GRF_IO_VSEL")]
pub type GrfIoVsel = crate::Reg<grf_io_vsel::GrfIoVselSpec>;
#[doc = ""]
pub mod grf_io_vsel;
#[doc = "GRF_SARADC_TESTBIT (rw) register accessor: saradc test bit control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_saradc_testbit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_saradc_testbit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_saradc_testbit`]
module"]
#[doc(alias = "GRF_SARADC_TESTBIT")]
pub type GrfSaradcTestbit = crate::Reg<grf_saradc_testbit::GrfSaradcTestbitSpec>;
#[doc = "saradc test bit control register"]
pub mod grf_saradc_testbit;
#[doc = "GRF_TSADC_TESTBIT_L (rw) register accessor: saradc test bit control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_tsadc_testbit_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_tsadc_testbit_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_tsadc_testbit_l`]
module"]
#[doc(alias = "GRF_TSADC_TESTBIT_L")]
pub type GrfTsadcTestbitL = crate::Reg<grf_tsadc_testbit_l::GrfTsadcTestbitLSpec>;
#[doc = "saradc test bit control register"]
pub mod grf_tsadc_testbit_l;
#[doc = "GRF_TSADC_TESTBIT_H (rw) register accessor: tsadc test bit control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_tsadc_testbit_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_tsadc_testbit_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_tsadc_testbit_h`]
module"]
#[doc(alias = "GRF_TSADC_TESTBIT_H")]
pub type GrfTsadcTestbitH = crate::Reg<grf_tsadc_testbit_h::GrfTsadcTestbitHSpec>;
#[doc = "tsadc test bit control register"]
pub mod grf_tsadc_testbit_h;
#[doc = "GRF_CHIP_ID_ADDR (rw) register accessor: chip id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_chip_id_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_chip_id_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_chip_id_addr`]
module"]
#[doc(alias = "GRF_CHIP_ID_ADDR")]
pub type GrfChipIdAddr = crate::Reg<grf_chip_id_addr::GrfChipIdAddrSpec>;
#[doc = "chip id register"]
pub mod grf_chip_id_addr;
#[doc = "GRF_FAST_BOOT_ADDR (rw) register accessor: faster boot address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_fast_boot_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_fast_boot_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_fast_boot_addr`]
module"]
#[doc(alias = "GRF_FAST_BOOT_ADDR")]
pub type GrfFastBootAddr = crate::Reg<grf_fast_boot_addr::GrfFastBootAddrSpec>;
#[doc = "faster boot address register"]
pub mod grf_fast_boot_addr;
#[doc = "GRF_EMMCCORE_CON0 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmccore_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmccore_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmccore_con0`]
module"]
#[doc(alias = "GRF_EMMCCORE_CON0")]
pub type GrfEmmccoreCon0 = crate::Reg<grf_emmccore_con0::GrfEmmccoreCon0Spec>;
#[doc = "emmc core control register"]
pub mod grf_emmccore_con0;
#[doc = "GRF_EMMCCORE_CON1 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmccore_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmccore_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmccore_con1`]
module"]
#[doc(alias = "GRF_EMMCCORE_CON1")]
pub type GrfEmmccoreCon1 = crate::Reg<grf_emmccore_con1::GrfEmmccoreCon1Spec>;
#[doc = "emmc core control register"]
pub mod grf_emmccore_con1;
#[doc = "GRF_EMMCCORE_CON2 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmccore_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmccore_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmccore_con2`]
module"]
#[doc(alias = "GRF_EMMCCORE_CON2")]
pub type GrfEmmccoreCon2 = crate::Reg<grf_emmccore_con2::GrfEmmccoreCon2Spec>;
#[doc = "emmc core control register"]
pub mod grf_emmccore_con2;
#[doc = "GRF_EMMCCORE_CON3 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmccore_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmccore_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmccore_con3`]
module"]
#[doc(alias = "GRF_EMMCCORE_CON3")]
pub type GrfEmmccoreCon3 = crate::Reg<grf_emmccore_con3::GrfEmmccoreCon3Spec>;
#[doc = "emmc core control register"]
pub mod grf_emmccore_con3;
#[doc = "GRF_EMMCCORE_CON4 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmccore_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmccore_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmccore_con4`]
module"]
#[doc(alias = "GRF_EMMCCORE_CON4")]
pub type GrfEmmccoreCon4 = crate::Reg<grf_emmccore_con4::GrfEmmccoreCon4Spec>;
#[doc = "emmc core control register"]
pub mod grf_emmccore_con4;
#[doc = "GRF_EMMCCORE_CON5 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmccore_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmccore_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmccore_con5`]
module"]
#[doc(alias = "GRF_EMMCCORE_CON5")]
pub type GrfEmmccoreCon5 = crate::Reg<grf_emmccore_con5::GrfEmmccoreCon5Spec>;
#[doc = "emmc core control register"]
pub mod grf_emmccore_con5;
#[doc = "GRF_EMMCCORE_CON6 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmccore_con6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmccore_con6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmccore_con6`]
module"]
#[doc(alias = "GRF_EMMCCORE_CON6")]
pub type GrfEmmccoreCon6 = crate::Reg<grf_emmccore_con6::GrfEmmccoreCon6Spec>;
#[doc = "emmc core control register"]
pub mod grf_emmccore_con6;
#[doc = "GRF_EMMCCORE_CON7 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmccore_con7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmccore_con7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmccore_con7`]
module"]
#[doc(alias = "GRF_EMMCCORE_CON7")]
pub type GrfEmmccoreCon7 = crate::Reg<grf_emmccore_con7::GrfEmmccoreCon7Spec>;
#[doc = "emmc core control register"]
pub mod grf_emmccore_con7;
#[doc = "GRF_EMMCCORE_CON8 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmccore_con8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmccore_con8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmccore_con8`]
module"]
#[doc(alias = "GRF_EMMCCORE_CON8")]
pub type GrfEmmccoreCon8 = crate::Reg<grf_emmccore_con8::GrfEmmccoreCon8Spec>;
#[doc = "emmc core control register"]
pub mod grf_emmccore_con8;
#[doc = "GRF_EMMCCORE_CON9 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmccore_con9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmccore_con9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmccore_con9`]
module"]
#[doc(alias = "GRF_EMMCCORE_CON9")]
pub type GrfEmmccoreCon9 = crate::Reg<grf_emmccore_con9::GrfEmmccoreCon9Spec>;
#[doc = "emmc core control register"]
pub mod grf_emmccore_con9;
#[doc = "GRF_EMMCCORE_CON10 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmccore_con10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmccore_con10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmccore_con10`]
module"]
#[doc(alias = "GRF_EMMCCORE_CON10")]
pub type GrfEmmccoreCon10 = crate::Reg<grf_emmccore_con10::GrfEmmccoreCon10Spec>;
#[doc = "emmc core control register"]
pub mod grf_emmccore_con10;
#[doc = "GRF_EMMCCORE_CON11 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmccore_con11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmccore_con11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmccore_con11`]
module"]
#[doc(alias = "GRF_EMMCCORE_CON11")]
pub type GrfEmmccoreCon11 = crate::Reg<grf_emmccore_con11::GrfEmmccoreCon11Spec>;
#[doc = "emmc core control register"]
pub mod grf_emmccore_con11;
#[doc = "GRF_EMMCCORE_STATUS0 (rw) register accessor: emmc core status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmccore_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmccore_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmccore_status0`]
module"]
#[doc(alias = "GRF_EMMCCORE_STATUS0")]
pub type GrfEmmccoreStatus0 = crate::Reg<grf_emmccore_status0::GrfEmmccoreStatus0Spec>;
#[doc = "emmc core status register"]
pub mod grf_emmccore_status0;
#[doc = "GRF_EMMCCORE_STATUS1 (rw) register accessor: emmc core status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmccore_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmccore_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmccore_status1`]
module"]
#[doc(alias = "GRF_EMMCCORE_STATUS1")]
pub type GrfEmmccoreStatus1 = crate::Reg<grf_emmccore_status1::GrfEmmccoreStatus1Spec>;
#[doc = "emmc core status register"]
pub mod grf_emmccore_status1;
#[doc = "GRF_EMMCCORE_STATUS2 (rw) register accessor: emmc core status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmccore_status2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmccore_status2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmccore_status2`]
module"]
#[doc(alias = "GRF_EMMCCORE_STATUS2")]
pub type GrfEmmccoreStatus2 = crate::Reg<grf_emmccore_status2::GrfEmmccoreStatus2Spec>;
#[doc = "emmc core status register"]
pub mod grf_emmccore_status2;
#[doc = "GRF_EMMCCORE_STATUS3 (rw) register accessor: emmc core status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmccore_status3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmccore_status3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmccore_status3`]
module"]
#[doc(alias = "GRF_EMMCCORE_STATUS3")]
pub type GrfEmmccoreStatus3 = crate::Reg<grf_emmccore_status3::GrfEmmccoreStatus3Spec>;
#[doc = "emmc core status register"]
pub mod grf_emmccore_status3;
#[doc = "GRF_EMMCPHY_CON0 (rw) register accessor: emmc phy control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmcphy_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmcphy_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmcphy_con0`]
module"]
#[doc(alias = "GRF_EMMCPHY_CON0")]
pub type GrfEmmcphyCon0 = crate::Reg<grf_emmcphy_con0::GrfEmmcphyCon0Spec>;
#[doc = "emmc phy control register"]
pub mod grf_emmcphy_con0;
#[doc = "GRF_EMMCPHY_CON1 (rw) register accessor: emmc phy control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmcphy_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmcphy_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmcphy_con1`]
module"]
#[doc(alias = "GRF_EMMCPHY_CON1")]
pub type GrfEmmcphyCon1 = crate::Reg<grf_emmcphy_con1::GrfEmmcphyCon1Spec>;
#[doc = "emmc phy control register"]
pub mod grf_emmcphy_con1;
#[doc = "GRF_EMMCPHY_CON2 (rw) register accessor: emmc phy control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmcphy_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmcphy_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmcphy_con2`]
module"]
#[doc(alias = "GRF_EMMCPHY_CON2")]
pub type GrfEmmcphyCon2 = crate::Reg<grf_emmcphy_con2::GrfEmmcphyCon2Spec>;
#[doc = "emmc phy control register"]
pub mod grf_emmcphy_con2;
#[doc = "GRF_EMMCPHY_CON3 (rw) register accessor: emmc phy control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmcphy_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmcphy_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmcphy_con3`]
module"]
#[doc(alias = "GRF_EMMCPHY_CON3")]
pub type GrfEmmcphyCon3 = crate::Reg<grf_emmcphy_con3::GrfEmmcphyCon3Spec>;
#[doc = "emmc phy control register"]
pub mod grf_emmcphy_con3;
#[doc = "GRF_EMMCPHY_CON4 (rw) register accessor: emmc phy control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmcphy_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmcphy_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmcphy_con4`]
module"]
#[doc(alias = "GRF_EMMCPHY_CON4")]
pub type GrfEmmcphyCon4 = crate::Reg<grf_emmcphy_con4::GrfEmmcphyCon4Spec>;
#[doc = "emmc phy control register"]
pub mod grf_emmcphy_con4;
#[doc = "GRF_EMMCPHY_CON5 (rw) register accessor: emmc phy control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmcphy_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmcphy_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmcphy_con5`]
module"]
#[doc(alias = "GRF_EMMCPHY_CON5")]
pub type GrfEmmcphyCon5 = crate::Reg<grf_emmcphy_con5::GrfEmmcphyCon5Spec>;
#[doc = "emmc phy control register"]
pub mod grf_emmcphy_con5;
#[doc = "GRF_EMMCPHY_CON6 (rw) register accessor: emmc phy control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmcphy_con6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmcphy_con6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmcphy_con6`]
module"]
#[doc(alias = "GRF_EMMCPHY_CON6")]
pub type GrfEmmcphyCon6 = crate::Reg<grf_emmcphy_con6::GrfEmmcphyCon6Spec>;
#[doc = "emmc phy control register"]
pub mod grf_emmcphy_con6;
#[doc = "GRF_EMMCPHY_STATUS (rw) register accessor: emmc phy status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmcphy_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmcphy_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_emmcphy_status`]
module"]
#[doc(alias = "GRF_EMMCPHY_STATUS")]
pub type GrfEmmcphyStatus = crate::Reg<grf_emmcphy_status::GrfEmmcphyStatusSpec>;
#[doc = "emmc phy status register"]
pub mod grf_emmcphy_status;
