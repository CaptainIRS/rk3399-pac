#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x2000],
    usb3_perf_con0: Usb3PerfCon0,
    usb3_perf_con1: Usb3PerfCon1,
    usb3_perf_con2: Usb3PerfCon2,
    usb3_perf_rd_max_latency_num: Usb3PerfRdMaxLatencyNum,
    usb3_perf_rd_latency_samp_num: Usb3PerfRdLatencySampNum,
    usb3_perf_rd_latency_acc_num: Usb3PerfRdLatencyAccNum,
    usb3_perf_rd_axi_total_byte: Usb3PerfRdAxiTotalByte,
    usb3_perf_wr_axi_total_byte: Usb3PerfWrAxiTotalByte,
    usb3_perf_working_cnt: Usb3PerfWorkingCnt,
    _reserved9: [u8; 0x040c],
    usb3otg0_con0: Usb3otg0Con0,
    usb3otg0_con1: Usb3otg0Con1,
    _reserved11: [u8; 0x08],
    usb3otg1_con0: Usb3otg1Con0,
    usb3otg1_con1: Usb3otg1Con1,
    _reserved13: [u8; 0x08],
    usb3otg0_status_lat0: Usb3otg0StatusLat0,
    usb3otg0_status_lat1: Usb3otg0StatusLat1,
    usb3otg0_status_cb: Usb3otg0StatusCb,
    _reserved16: [u8; 0x04],
    usb3otg1_status_lat0: Usb3otg1StatusLat0,
    usb3otg1_status_lat1: Usb3otg1StatusLat1,
    usb3otg1_status_cb: Usb3otg1StatusCb,
    _reserved19: [u8; 0x1b94],
    pcie_perf_con0: PciePerfCon0,
    pcie_perf_con1: PciePerfCon1,
    pcie_perf_con2: PciePerfCon2,
    pcie_perf_rd_max_latency_num: PciePerfRdMaxLatencyNum,
    pcie_perf_rd_latency_samp_num: PciePerfRdLatencySampNum,
    pcie_perf_rd_latency_acc_num: PciePerfRdLatencyAccNum,
    pcie_perf_rd_axi_total_byte: PciePerfRdAxiTotalByte,
    pcie_perf_wr_axi_total_byte: PciePerfWrAxiTotalByte,
    pcie_perf_working_cnt: PciePerfWorkingCnt,
    _reserved28: [u8; 0xdc],
    usb20_host0_con0: Usb20Host0Con0,
    usb20_host0_con1: Usb20Host0Con1,
    _reserved30: [u8; 0x08],
    usb20_host1_con0: Usb20Host1Con0,
    usb20_host1_con1: Usb20Host1Con1,
    _reserved32: [u8; 0x08],
    hsic_con0: HsicCon0,
    hsic_con1: HsicCon1,
    _reserved34: [u8; 0x18],
    grf_usbhost0_status: GrfUsbhost0Status,
    grf_usbhost1_status: GrfUsbhost1Status,
    grf_hsic_status: GrfHsicStatus,
    _reserved37: [u8; 0x0324],
    hsicphy_con0: HsicphyCon0,
    _reserved38: [u8; 0x0c],
    usbphy0_ctrl0: Usbphy0Ctrl0,
    usbphy0_ctrl1: Usbphy0Ctrl1,
    usbphy0_ctrl2: Usbphy0Ctrl2,
    usbphy0_ctrl3: Usbphy0Ctrl3,
    usbphy0_ctrl4: Usbphy0Ctrl4,
    usbphy0_ctrl5: Usbphy0Ctrl5,
    usbphy0_ctrl6: Usbphy0Ctrl6,
    usbphy0_ctrl7: Usbphy0Ctrl7,
    usbphy0_ctrl8: Usbphy0Ctrl8,
    usbphy0_ctrl9: Usbphy0Ctrl9,
    usbphy0_ctrl10: Usbphy0Ctrl10,
    usbphy0_ctrl11: Usbphy0Ctrl11,
    usbphy0_ctrl12: Usbphy0Ctrl12,
    usbphy0_ctrl13: Usbphy0Ctrl13,
    usbphy0_ctrl14: Usbphy0Ctrl14,
    usbphy0_ctrl15: Usbphy0Ctrl15,
    usbphy0_ctrl16: Usbphy0Ctrl16,
    usbphy0_ctrl17: Usbphy0Ctrl17,
    usbphy0_ctrl18: Usbphy0Ctrl18,
    usbphy0_ctrl19: Usbphy0Ctrl19,
    usbphy0_ctrl20: Usbphy0Ctrl20,
    usbphy0_ctrl21: Usbphy0Ctrl21,
    usbphy0_ctrl22: Usbphy0Ctrl22,
    usbphy0_ctrl23: Usbphy0Ctrl23,
    usbphy0_ctrl24: Usbphy0Ctrl24,
    usbphy0_ctrl25: Usbphy0Ctrl25,
    _reserved64: [u8; 0x18],
    usbphy1_ctrl0: Usbphy1Ctrl0,
    usbphy1_ctrl1: Usbphy1Ctrl1,
    usbphy1_ctrl2: Usbphy1Ctrl2,
    usbphy1_ctrl3: Usbphy1Ctrl3,
    usbphy1_ctrl4: Usbphy1Ctrl4,
    usbphy1_ctrl5: Usbphy1Ctrl5,
    usbphy1_ctrl6: Usbphy1Ctrl6,
    usbphy1_ctrl7: Usbphy1Ctrl7,
    usbphy1_ctrl8: Usbphy1Ctrl8,
    usbphy1_ctrl9: Usbphy1Ctrl9,
    usbphy1_ctrl10: Usbphy1Ctrl10,
    usbphy1_ctrl11: Usbphy1Ctrl11,
    usbphy1_ctrl12: Usbphy1Ctrl12,
    usbphy1_ctrl13: Usbphy1Ctrl13,
    usbphy1_ctrl14: Usbphy1Ctrl14,
    usbphy1_ctrl15: Usbphy1Ctrl15,
    usbphy1_ctrl16: Usbphy1Ctrl16,
    usbphy1_ctrl17: Usbphy1Ctrl17,
    usbphy1_ctrl18: Usbphy1Ctrl18,
    usbphy1_ctrl19: Usbphy1Ctrl19,
    usbphy1_ctrl20: Usbphy1Ctrl20,
    usbphy1_ctrl21: Usbphy1Ctrl21,
    usbphy1_ctrl22: Usbphy1Ctrl22,
    usbphy1_ctrl23: Usbphy1Ctrl23,
    usbphy1_ctrl24: Usbphy1Ctrl24,
    usbphy1_ctrl25: Usbphy1Ctrl25,
    _reserved90: [u8; 0x1a98],
    hdcp22_perf_con0: Hdcp22PerfCon0,
    hdcp22_perf_con1: Hdcp22PerfCon1,
    hdcp22_perf_con2: Hdcp22PerfCon2,
    hdcp22_perf_rd_max_latency_num: Hdcp22PerfRdMaxLatencyNum,
    hdcp22_perf_rd_latency_samp_num: Hdcp22PerfRdLatencySampNum,
    hdcp22_perf_rd_latency_acc_num: Hdcp22PerfRdLatencyAccNum,
    hdcp22_perf_rd_axi_total_byte: Hdcp22PerfRdAxiTotalByte,
    hdcp22_perf_wr_axi_total_byte: Hdcp22PerfWrAxiTotalByte,
    hdcp22_perf_working_cnt: Hdcp22PerfWorkingCnt,
    _reserved99: [u8; 0x0200],
    soc_con9: SocCon9,
    _reserved100: [u8; 0x28],
    soc_con20: SocCon20,
    soc_con21: SocCon21,
    soc_con22: SocCon22,
    soc_con23: SocCon23,
    soc_con24: SocCon24,
    soc_con25: SocCon25,
    soc_con26: SocCon26,
    _reserved107: [u8; 0x1d94],
    gpu_perf_con0: GpuPerfCon0,
    gpu_perf_con1: GpuPerfCon1,
    gpu_perf_con2: GpuPerfCon2,
    gpu_perf_rd_max_latency_num: GpuPerfRdMaxLatencyNum,
    gpu_perf_rd_latency_samp_num: GpuPerfRdLatencySampNum,
    gpu_perf_rd_latency_acc_num: GpuPerfRdLatencyAccNum,
    gpu_perf_rd_axi_total_byte: GpuPerfRdAxiTotalByte,
    gpu_perf_wr_axi_total_byte: GpuPerfWrAxiTotalByte,
    gpu_perf_working_cnt: GpuPerfWorkingCnt,
    _reserved116: [u8; 0x1fdc],
    cpu_con0: CpuCon0,
    cpu_con1: CpuCon1,
    cpu_con2: CpuCon2,
    cpu_con3: CpuCon3,
    _reserved120: [u8; 0x70],
    cpu_status0: CpuStatus0,
    cpu_status1: CpuStatus1,
    cpu_status2: CpuStatus2,
    cpu_status3: CpuStatus3,
    cpu_status4: CpuStatus4,
    cpu_status5: CpuStatus5,
    _reserved126: [u8; 0x68],
    a53_perf_con0: A53PerfCon0,
    a53_perf_con1: A53PerfCon1,
    a53_perf_con2: A53PerfCon2,
    a53_perf_con3: A53PerfCon3,
    a53_perf_rd_mon_st: A53PerfRdMonSt,
    a53_perf_rd_mon_end: A53PerfRdMonEnd,
    a53_perf_wr_mon_st: A53PerfWrMonSt,
    a53_perf_wr_mon_end: A53PerfWrMonEnd,
    a53_perf_rd_max_latency_num: A53PerfRdMaxLatencyNum,
    a53_perf_rd_latency_samp_num: A53PerfRdLatencySampNum,
    a53_perf_rd_latency_acc_num: A53PerfRdLatencyAccNum,
    a53_perf_rd_axi_total_byte: A53PerfRdAxiTotalByte,
    a53_perf_wr_axi_total_byte: A53PerfWrAxiTotalByte,
    a53_perf_working_cnt: A53PerfWorkingCnt,
    a53_perf_int_status: A53PerfIntStatus,
    _reserved141: [u8; 0xc4],
    a72_perf_con0: A72PerfCon0,
    a72_perf_con1: A72PerfCon1,
    a72_perf_con2: A72PerfCon2,
    a72_perf_con3: A72PerfCon3,
    a72_perf_rd_mon_st: A72PerfRdMonSt,
    a72_perf_rd_mon_end: A72PerfRdMonEnd,
    a72_perf_wr_mon_st: A72PerfWrMonSt,
    a72_perf_wr_mon_end: A72PerfWrMonEnd,
    a72_perf_rd_max_latency_num: A72PerfRdMaxLatencyNum,
    a72_perf_rd_latency_samp_num: A72PerfRdLatencySampNum,
    a72_perf_rd_latency_acc_num: A72PerfRdLatencyAccNum,
    a72_perf_rd_axi_total_byte: A72PerfRdAxiTotalByte,
    a72_perf_wr_axi_total_byte: A72PerfWrAxiTotalByte,
    a72_perf_working_cnt: A72PerfWorkingCnt,
    a72_perf_int_status: A72PerfIntStatus,
    _reserved156: [u8; 0x1dc4],
    gmac_perf_con0: GmacPerfCon0,
    gmac_perf_con1: GmacPerfCon1,
    gmac_perf_con2: GmacPerfCon2,
    gmac_perf_rd_max_latency_num: GmacPerfRdMaxLatencyNum,
    gmac_perf_rd_latency_samp_num: GmacPerfRdLatencySampNum,
    gmac_perf_rd_latency_acc_num: GmacPerfRdLatencyAccNum,
    gmac_perf_rd_axi_total_byte: GmacPerfRdAxiTotalByte,
    gmac_perf_wr_axi_total_byte: GmacPerfWrAxiTotalByte,
    gmac_perf_working_cnt: GmacPerfWorkingCnt,
    _reserved165: [u8; 0x01f0],
    soc_con5: SocCon5,
    soc_con6: SocCon6,
    _reserved167: [u8; 0x1de4],
    gpio2a_iomux: Gpio2aIomux,
    gpio2b_iomux: Gpio2bIomux,
    gpio2c_iomux: Gpio2cIomux,
    gpio2d_iomux: Gpio2dIomux,
    gpio3a_iomux: Gpio3aIomux,
    gpio3b_iomux: Gpio3bIomux,
    gpio3c_iomux: Gpio3cIomux,
    gpio3d_iomux: Gpio3dIomux,
    gpio4a_iomux: Gpio4aIomux,
    gpio4b_iomux: Gpio4bIomux,
    gpio4c_iomux: Gpio4cIomux,
    gpio4d_iomux: Gpio4dIomux,
    _reserved179: [u8; 0x10],
    gpio2a_p: Gpio2aP,
    gpio2b_p: Gpio2bP,
    gpio2c_p: Gpio2cP,
    gpio2d_p: Gpio2dP,
    gpio3a_p: Gpio3aP,
    gpio3b_p: Gpio3bP,
    gpio3c_p: Gpio3cP,
    gpio3d_p: Gpio3dP,
    gpio4a_p: Gpio4aP,
    gpio4b_p: Gpio4bP,
    gpio4c_p: Gpio4cP,
    gpio4d_p: Gpio4dP,
    _reserved191: [u8; 0x10],
    gpio2a_sr: Gpio2aSr,
    gpio2b_sr: Gpio2bSr,
    gpio2c_sr: Gpio2cSr,
    gpio2d_sr: Gpio2dSr,
    _reserved195: [u8; 0x0c],
    gpio3d_sr: Gpio3dSr,
    gpio4a_sr: Gpio4aSr,
    gpio4b_sr: Gpio4bSr,
    gpio4c_sr: Gpio4cSr,
    gpio4d_sr: Gpio4dSr,
    _reserved200: [u8; 0x10],
    gpio2a_smt: Gpio2aSmt,
    gpio2b_smt: Gpio2bSmt,
    gpio2c_smt: Gpio2cSmt,
    gpio2d_smt: Gpio2dSmt,
    gpio3a_smt: Gpio3aSmt,
    gpio3b_smt: Gpio3bSmt,
    gpio3c_smt: Gpio3cSmt,
    gpio3d_smt: Gpio3dSmt,
    gpio4a_smt: Gpio4aSmt,
    gpio4b_smt: Gpio4bSmt,
    gpio4c_smt: Gpio4cSmt,
    gpio4d_smt: Gpio4dSmt,
    _reserved212: [u8; 0x10],
    gpio2a_e: Gpio2aE,
    gpio2b_e: Gpio2bE,
    gpio2c_e: Gpio2cE,
    gpio2d_e: Gpio2dE,
    gpio3a_e01: Gpio3aE01,
    gpio3a_e2: Gpio3aE2,
    gpio3b_e01: Gpio3bE01,
    gpio3b_e2: Gpio3bE2,
    gpio3c_e01: Gpio3cE01,
    gpio3c_e2: Gpio3cE2,
    gpio3d_e: Gpio3dE,
    gpio4a_e: Gpio4aE,
    gpio4b_e01: Gpio4bE01,
    gpio4b_e2: Gpio4bE2,
    gpio4c_e: Gpio4cE,
    gpio4d_e: Gpio4dE,
    _reserved228: [u8; 0x48],
    gpio2c_he: Gpio2cHe,
    gpio2d_he: Gpio2dHe,
    _reserved230: [u8; 0x70],
    soc_con0: SocCon0,
    soc_con1: SocCon1,
    soc_con2: SocCon2,
    soc_con3: SocCon3,
    soc_con4: SocCon4,
    soc_con_5_pcie: SocCon5Pcie,
    _reserved236: [u8; 0x04],
    soc_con7: SocCon7,
    soc_con8: SocCon8,
    soc_con_9_pcie: SocCon9Pcie,
    _reserved239: [u8; 0x78],
    soc_status0: SocStatus0,
    soc_status1: SocStatus1,
    soc_status2: SocStatus2,
    soc_status3: SocStatus3,
    soc_status4: SocStatus4,
    soc_status5: SocStatus5,
    _reserved245: [u8; 0xc8],
    ddrc0_con0: Ddrc0Con0,
    ddrc0_con1: Ddrc0Con1,
    ddrc1_con0: Ddrc1Con0,
    ddrc1_con1: Ddrc1Con1,
    _reserved249: [u8; 0x30],
    sig_detect_con0: SigDetectCon0,
    _reserved250: [u8; 0x04],
    sig_detect_con1: SigDetectCon1,
    _reserved251: [u8; 0x04],
    sig_detect_clr: SigDetectClr,
    _reserved252: [u8; 0x0c],
    sig_detect_status: SigDetectStatus,
    _reserved253: [u8; 0x6c],
    usb20_phy0_con0: Usb20Phy0Con0,
    usb20_phy0_con1: Usb20Phy0Con1,
    usb20_phy0_con2: Usb20Phy0Con2,
    usb20_phy0_con3: Usb20Phy0Con3,
    usb20_phy1_con0: Usb20Phy1Con0,
    usb20_phy1_con1: Usb20Phy1Con1,
    usb20_phy1_con2: Usb20Phy1Con2,
    usb20_phy1_con3: Usb20Phy1Con3,
    _reserved261: [u8; 0x0110],
    usb3phy0_con0: Usb3phy0Con0,
    usb3phy0_con1: Usb3phy0Con1,
    usb3phy0_con2: Usb3phy0Con2,
    usb3phy1_con0: Usb3phy1Con0,
    usb3phy1_con1: Usb3phy1Con1,
    usb3phy1_con2: Usb3phy1Con2,
    _reserved267: [u8; 0x28],
    usb3phy_status0: Usb3phyStatus0,
    usb3phy_status1: Usb3phyStatus1,
    _reserved269: [u8; 0x38],
    dll_con0: DllCon0,
    dll_con1: DllCon1,
    dll_con2: DllCon2,
    dll_con3: DllCon3,
    dll_con4: DllCon4,
    dll_con5: DllCon5,
    _reserved275: [u8; 0x08],
    dll_status0: DllStatus0,
    dll_status1: DllStatus1,
    dll_status2: DllStatus2,
    dll_status3: DllStatus3,
    dll_status4: DllStatus4,
    _reserved280: [u8; 0x0c],
    io_vsel: IoVsel,
    saradc_testbit: SaradcTestbit,
    tsadc_testbit_l: TsadcTestbitL,
    tsadc_testbit_h: TsadcTestbitH,
    _reserved284: [u8; 0x01b0],
    chip_id_addr: ChipIdAddr,
    _reserved285: [u8; 0x7c],
    fast_boot_addr: FastBootAddr,
    _reserved286: [u8; 0x077c],
    emmccore_con0: EmmccoreCon0,
    emmccore_con1: EmmccoreCon1,
    emmccore_con2: EmmccoreCon2,
    emmccore_con3: EmmccoreCon3,
    emmccore_con4: EmmccoreCon4,
    emmccore_con5: EmmccoreCon5,
    emmccore_con6: EmmccoreCon6,
    emmccore_con7: EmmccoreCon7,
    emmccore_con8: EmmccoreCon8,
    emmccore_con9: EmmccoreCon9,
    emmccore_con10: EmmccoreCon10,
    emmccore_con11: EmmccoreCon11,
    _reserved298: [u8; 0x10],
    emmccore_status0: EmmccoreStatus0,
    emmccore_status1: EmmccoreStatus1,
    emmccore_status2: EmmccoreStatus2,
    emmccore_status3: EmmccoreStatus3,
    _reserved302: [u8; 0x0730],
    emmcphy_con0: EmmcphyCon0,
    emmcphy_con1: EmmcphyCon1,
    emmcphy_con2: EmmcphyCon2,
    emmcphy_con3: EmmcphyCon3,
    emmcphy_con4: EmmcphyCon4,
    emmcphy_con5: EmmcphyCon5,
    emmcphy_con6: EmmcphyCon6,
    _reserved309: [u8; 0x04],
    emmcphy_status: EmmcphyStatus,
}
impl RegisterBlock {
    #[doc = "0x2000 - usb3 performance monitor control register"]
    #[inline(always)]
    pub const fn usb3_perf_con0(&self) -> &Usb3PerfCon0 {
        &self.usb3_perf_con0
    }
    #[doc = "0x2004 - usb3 performance monitor control register"]
    #[inline(always)]
    pub const fn usb3_perf_con1(&self) -> &Usb3PerfCon1 {
        &self.usb3_perf_con1
    }
    #[doc = "0x2008 - usb3 performance monitor control register"]
    #[inline(always)]
    pub const fn usb3_perf_con2(&self) -> &Usb3PerfCon2 {
        &self.usb3_perf_con2
    }
    #[doc = "0x200c - usb3 performance monitor status register"]
    #[inline(always)]
    pub const fn usb3_perf_rd_max_latency_num(&self) -> &Usb3PerfRdMaxLatencyNum {
        &self.usb3_perf_rd_max_latency_num
    }
    #[doc = "0x2010 - usb3 performance monitor status register"]
    #[inline(always)]
    pub const fn usb3_perf_rd_latency_samp_num(&self) -> &Usb3PerfRdLatencySampNum {
        &self.usb3_perf_rd_latency_samp_num
    }
    #[doc = "0x2014 - usb3 performance monitor status register"]
    #[inline(always)]
    pub const fn usb3_perf_rd_latency_acc_num(&self) -> &Usb3PerfRdLatencyAccNum {
        &self.usb3_perf_rd_latency_acc_num
    }
    #[doc = "0x2018 - usb3 performance monitor status register"]
    #[inline(always)]
    pub const fn usb3_perf_rd_axi_total_byte(&self) -> &Usb3PerfRdAxiTotalByte {
        &self.usb3_perf_rd_axi_total_byte
    }
    #[doc = "0x201c - usb3 performance monitor status register"]
    #[inline(always)]
    pub const fn usb3_perf_wr_axi_total_byte(&self) -> &Usb3PerfWrAxiTotalByte {
        &self.usb3_perf_wr_axi_total_byte
    }
    #[doc = "0x2020 - usb3 performance monitor status register"]
    #[inline(always)]
    pub const fn usb3_perf_working_cnt(&self) -> &Usb3PerfWorkingCnt {
        &self.usb3_perf_working_cnt
    }
    #[doc = "0x2430 - USB3 OTG0 GRF Register0"]
    #[inline(always)]
    pub const fn usb3otg0_con0(&self) -> &Usb3otg0Con0 {
        &self.usb3otg0_con0
    }
    #[doc = "0x2434 - USB3 OTG0 GRF Register1"]
    #[inline(always)]
    pub const fn usb3otg0_con1(&self) -> &Usb3otg0Con1 {
        &self.usb3otg0_con1
    }
    #[doc = "0x2440 - USB3 OTG1 GRF Register0"]
    #[inline(always)]
    pub const fn usb3otg1_con0(&self) -> &Usb3otg1Con0 {
        &self.usb3otg1_con0
    }
    #[doc = "0x2444 - USB3 OTG1 GRF Register1"]
    #[inline(always)]
    pub const fn usb3otg1_con1(&self) -> &Usb3otg1Con1 {
        &self.usb3otg1_con1
    }
    #[doc = "0x2450 - USB3 OTG0 status register"]
    #[inline(always)]
    pub const fn usb3otg0_status_lat0(&self) -> &Usb3otg0StatusLat0 {
        &self.usb3otg0_status_lat0
    }
    #[doc = "0x2454 - USB3 OTG1 status register"]
    #[inline(always)]
    pub const fn usb3otg0_status_lat1(&self) -> &Usb3otg0StatusLat1 {
        &self.usb3otg0_status_lat1
    }
    #[doc = "0x2458 - USB3 OTG0 status register"]
    #[inline(always)]
    pub const fn usb3otg0_status_cb(&self) -> &Usb3otg0StatusCb {
        &self.usb3otg0_status_cb
    }
    #[doc = "0x2460 - USB3 OTG1 status register"]
    #[inline(always)]
    pub const fn usb3otg1_status_lat0(&self) -> &Usb3otg1StatusLat0 {
        &self.usb3otg1_status_lat0
    }
    #[doc = "0x2464 - USB3 OTG1 status register"]
    #[inline(always)]
    pub const fn usb3otg1_status_lat1(&self) -> &Usb3otg1StatusLat1 {
        &self.usb3otg1_status_lat1
    }
    #[doc = "0x2468 - USB3 OTG1 status register"]
    #[inline(always)]
    pub const fn usb3otg1_status_cb(&self) -> &Usb3otg1StatusCb {
        &self.usb3otg1_status_cb
    }
    #[doc = "0x4000 - pcie performance monitor control register"]
    #[inline(always)]
    pub const fn pcie_perf_con0(&self) -> &PciePerfCon0 {
        &self.pcie_perf_con0
    }
    #[doc = "0x4004 - pcie performance monitor control register"]
    #[inline(always)]
    pub const fn pcie_perf_con1(&self) -> &PciePerfCon1 {
        &self.pcie_perf_con1
    }
    #[doc = "0x4008 - pcie performance monitor control register"]
    #[inline(always)]
    pub const fn pcie_perf_con2(&self) -> &PciePerfCon2 {
        &self.pcie_perf_con2
    }
    #[doc = "0x400c - pcieperformance monitor status register"]
    #[inline(always)]
    pub const fn pcie_perf_rd_max_latency_num(&self) -> &PciePerfRdMaxLatencyNum {
        &self.pcie_perf_rd_max_latency_num
    }
    #[doc = "0x4010 - pcie performance monitor status register"]
    #[inline(always)]
    pub const fn pcie_perf_rd_latency_samp_num(&self) -> &PciePerfRdLatencySampNum {
        &self.pcie_perf_rd_latency_samp_num
    }
    #[doc = "0x4014 - pcie performance monitor status register"]
    #[inline(always)]
    pub const fn pcie_perf_rd_latency_acc_num(&self) -> &PciePerfRdLatencyAccNum {
        &self.pcie_perf_rd_latency_acc_num
    }
    #[doc = "0x4018 - pcie performance monitor status register"]
    #[inline(always)]
    pub const fn pcie_perf_rd_axi_total_byte(&self) -> &PciePerfRdAxiTotalByte {
        &self.pcie_perf_rd_axi_total_byte
    }
    #[doc = "0x401c - pcie performance monitor status register"]
    #[inline(always)]
    pub const fn pcie_perf_wr_axi_total_byte(&self) -> &PciePerfWrAxiTotalByte {
        &self.pcie_perf_wr_axi_total_byte
    }
    #[doc = "0x4020 - pcie performance monitor status register"]
    #[inline(always)]
    pub const fn pcie_perf_working_cnt(&self) -> &PciePerfWorkingCnt {
        &self.pcie_perf_working_cnt
    }
    #[doc = "0x4100 - USB20 Host0 GRF register0"]
    #[inline(always)]
    pub const fn usb20_host0_con0(&self) -> &Usb20Host0Con0 {
        &self.usb20_host0_con0
    }
    #[doc = "0x4104 - USB20 Host0 GRF register1"]
    #[inline(always)]
    pub const fn usb20_host0_con1(&self) -> &Usb20Host0Con1 {
        &self.usb20_host0_con1
    }
    #[doc = "0x4110 - USB20 Host1 GRF register0"]
    #[inline(always)]
    pub const fn usb20_host1_con0(&self) -> &Usb20Host1Con0 {
        &self.usb20_host1_con0
    }
    #[doc = "0x4114 - USB20 Host1 GRF register1"]
    #[inline(always)]
    pub const fn usb20_host1_con1(&self) -> &Usb20Host1Con1 {
        &self.usb20_host1_con1
    }
    #[doc = "0x4120 - HSIC controller GRF register 0"]
    #[inline(always)]
    pub const fn hsic_con0(&self) -> &HsicCon0 {
        &self.hsic_con0
    }
    #[doc = "0x4124 - HSIC controller GRF register1"]
    #[inline(always)]
    pub const fn hsic_con1(&self) -> &HsicCon1 {
        &self.hsic_con1
    }
    #[doc = "0x4140 - usb host0 controller status register"]
    #[inline(always)]
    pub const fn grf_usbhost0_status(&self) -> &GrfUsbhost0Status {
        &self.grf_usbhost0_status
    }
    #[doc = "0x4144 - usb host1 controller status register"]
    #[inline(always)]
    pub const fn grf_usbhost1_status(&self) -> &GrfUsbhost1Status {
        &self.grf_usbhost1_status
    }
    #[doc = "0x4148 - hsic controller status register"]
    #[inline(always)]
    pub const fn grf_hsic_status(&self) -> &GrfHsicStatus {
        &self.grf_hsic_status
    }
    #[doc = "0x4470 - HSICPHY GRF control register"]
    #[inline(always)]
    pub const fn hsicphy_con0(&self) -> &HsicphyCon0 {
        &self.hsicphy_con0
    }
    #[doc = "0x4480 - usbphy0_ctrl0"]
    #[inline(always)]
    pub const fn usbphy0_ctrl0(&self) -> &Usbphy0Ctrl0 {
        &self.usbphy0_ctrl0
    }
    #[doc = "0x4484 - usbphy0_ctrl1"]
    #[inline(always)]
    pub const fn usbphy0_ctrl1(&self) -> &Usbphy0Ctrl1 {
        &self.usbphy0_ctrl1
    }
    #[doc = "0x4488 - usbphy0_ctrl2"]
    #[inline(always)]
    pub const fn usbphy0_ctrl2(&self) -> &Usbphy0Ctrl2 {
        &self.usbphy0_ctrl2
    }
    #[doc = "0x448c - usbphy0_ctrl3"]
    #[inline(always)]
    pub const fn usbphy0_ctrl3(&self) -> &Usbphy0Ctrl3 {
        &self.usbphy0_ctrl3
    }
    #[doc = "0x4490 - usbphy0_ctrl4"]
    #[inline(always)]
    pub const fn usbphy0_ctrl4(&self) -> &Usbphy0Ctrl4 {
        &self.usbphy0_ctrl4
    }
    #[doc = "0x4494 - usbphy0_ctrl5"]
    #[inline(always)]
    pub const fn usbphy0_ctrl5(&self) -> &Usbphy0Ctrl5 {
        &self.usbphy0_ctrl5
    }
    #[doc = "0x4498 - usbphy0_ctrl6"]
    #[inline(always)]
    pub const fn usbphy0_ctrl6(&self) -> &Usbphy0Ctrl6 {
        &self.usbphy0_ctrl6
    }
    #[doc = "0x449c - usbphy0_ctrl7"]
    #[inline(always)]
    pub const fn usbphy0_ctrl7(&self) -> &Usbphy0Ctrl7 {
        &self.usbphy0_ctrl7
    }
    #[doc = "0x44a0 - usbphy0_ctrl8"]
    #[inline(always)]
    pub const fn usbphy0_ctrl8(&self) -> &Usbphy0Ctrl8 {
        &self.usbphy0_ctrl8
    }
    #[doc = "0x44a4 - usbphy0_ctrl9"]
    #[inline(always)]
    pub const fn usbphy0_ctrl9(&self) -> &Usbphy0Ctrl9 {
        &self.usbphy0_ctrl9
    }
    #[doc = "0x44a8 - usbphy0_ctrl10"]
    #[inline(always)]
    pub const fn usbphy0_ctrl10(&self) -> &Usbphy0Ctrl10 {
        &self.usbphy0_ctrl10
    }
    #[doc = "0x44ac - usbphy0_ctrl11"]
    #[inline(always)]
    pub const fn usbphy0_ctrl11(&self) -> &Usbphy0Ctrl11 {
        &self.usbphy0_ctrl11
    }
    #[doc = "0x44b0 - usbphy0_ctrl12"]
    #[inline(always)]
    pub const fn usbphy0_ctrl12(&self) -> &Usbphy0Ctrl12 {
        &self.usbphy0_ctrl12
    }
    #[doc = "0x44b4 - usbphy0_ctrl13"]
    #[inline(always)]
    pub const fn usbphy0_ctrl13(&self) -> &Usbphy0Ctrl13 {
        &self.usbphy0_ctrl13
    }
    #[doc = "0x44b8 - usbphy0_ctrl14"]
    #[inline(always)]
    pub const fn usbphy0_ctrl14(&self) -> &Usbphy0Ctrl14 {
        &self.usbphy0_ctrl14
    }
    #[doc = "0x44bc - usbphy0_ctrl15"]
    #[inline(always)]
    pub const fn usbphy0_ctrl15(&self) -> &Usbphy0Ctrl15 {
        &self.usbphy0_ctrl15
    }
    #[doc = "0x44c0 - usbphy0_ctrl16"]
    #[inline(always)]
    pub const fn usbphy0_ctrl16(&self) -> &Usbphy0Ctrl16 {
        &self.usbphy0_ctrl16
    }
    #[doc = "0x44c4 - usbphy0_ctrl17"]
    #[inline(always)]
    pub const fn usbphy0_ctrl17(&self) -> &Usbphy0Ctrl17 {
        &self.usbphy0_ctrl17
    }
    #[doc = "0x44c8 - usbphy0_ctrl18"]
    #[inline(always)]
    pub const fn usbphy0_ctrl18(&self) -> &Usbphy0Ctrl18 {
        &self.usbphy0_ctrl18
    }
    #[doc = "0x44cc - usbphy0_ctrl19"]
    #[inline(always)]
    pub const fn usbphy0_ctrl19(&self) -> &Usbphy0Ctrl19 {
        &self.usbphy0_ctrl19
    }
    #[doc = "0x44d0 - usbphy0_ctrl20"]
    #[inline(always)]
    pub const fn usbphy0_ctrl20(&self) -> &Usbphy0Ctrl20 {
        &self.usbphy0_ctrl20
    }
    #[doc = "0x44d4 - usbphy0_ctrl21"]
    #[inline(always)]
    pub const fn usbphy0_ctrl21(&self) -> &Usbphy0Ctrl21 {
        &self.usbphy0_ctrl21
    }
    #[doc = "0x44d8 - usbphy0_ctrl22"]
    #[inline(always)]
    pub const fn usbphy0_ctrl22(&self) -> &Usbphy0Ctrl22 {
        &self.usbphy0_ctrl22
    }
    #[doc = "0x44dc - usbphy0_ctrl23"]
    #[inline(always)]
    pub const fn usbphy0_ctrl23(&self) -> &Usbphy0Ctrl23 {
        &self.usbphy0_ctrl23
    }
    #[doc = "0x44e0 - usbphy0_ctrl24"]
    #[inline(always)]
    pub const fn usbphy0_ctrl24(&self) -> &Usbphy0Ctrl24 {
        &self.usbphy0_ctrl24
    }
    #[doc = "0x44e4 - usbphy0_ctrl25"]
    #[inline(always)]
    pub const fn usbphy0_ctrl25(&self) -> &Usbphy0Ctrl25 {
        &self.usbphy0_ctrl25
    }
    #[doc = "0x4500 - usbphy1_ctrl0"]
    #[inline(always)]
    pub const fn usbphy1_ctrl0(&self) -> &Usbphy1Ctrl0 {
        &self.usbphy1_ctrl0
    }
    #[doc = "0x4504 - usbphy1_ctrl1"]
    #[inline(always)]
    pub const fn usbphy1_ctrl1(&self) -> &Usbphy1Ctrl1 {
        &self.usbphy1_ctrl1
    }
    #[doc = "0x4508 - usbphy1_ctrl2"]
    #[inline(always)]
    pub const fn usbphy1_ctrl2(&self) -> &Usbphy1Ctrl2 {
        &self.usbphy1_ctrl2
    }
    #[doc = "0x450c - usbphy1_ctrl3"]
    #[inline(always)]
    pub const fn usbphy1_ctrl3(&self) -> &Usbphy1Ctrl3 {
        &self.usbphy1_ctrl3
    }
    #[doc = "0x4510 - usbphy1_ctrl4"]
    #[inline(always)]
    pub const fn usbphy1_ctrl4(&self) -> &Usbphy1Ctrl4 {
        &self.usbphy1_ctrl4
    }
    #[doc = "0x4514 - usbphy1_ctrl5"]
    #[inline(always)]
    pub const fn usbphy1_ctrl5(&self) -> &Usbphy1Ctrl5 {
        &self.usbphy1_ctrl5
    }
    #[doc = "0x4518 - usbphy1_ctrl6"]
    #[inline(always)]
    pub const fn usbphy1_ctrl6(&self) -> &Usbphy1Ctrl6 {
        &self.usbphy1_ctrl6
    }
    #[doc = "0x451c - usbphy1_ctrl7"]
    #[inline(always)]
    pub const fn usbphy1_ctrl7(&self) -> &Usbphy1Ctrl7 {
        &self.usbphy1_ctrl7
    }
    #[doc = "0x4520 - usbphy1_ctrl8"]
    #[inline(always)]
    pub const fn usbphy1_ctrl8(&self) -> &Usbphy1Ctrl8 {
        &self.usbphy1_ctrl8
    }
    #[doc = "0x4524 - usbphy1_ctrl9"]
    #[inline(always)]
    pub const fn usbphy1_ctrl9(&self) -> &Usbphy1Ctrl9 {
        &self.usbphy1_ctrl9
    }
    #[doc = "0x4528 - usbphy1_ctrl10"]
    #[inline(always)]
    pub const fn usbphy1_ctrl10(&self) -> &Usbphy1Ctrl10 {
        &self.usbphy1_ctrl10
    }
    #[doc = "0x452c - usbphy1_ctrl11"]
    #[inline(always)]
    pub const fn usbphy1_ctrl11(&self) -> &Usbphy1Ctrl11 {
        &self.usbphy1_ctrl11
    }
    #[doc = "0x4530 - usbphy1_ctrl12"]
    #[inline(always)]
    pub const fn usbphy1_ctrl12(&self) -> &Usbphy1Ctrl12 {
        &self.usbphy1_ctrl12
    }
    #[doc = "0x4534 - usbphy1_ctrl13"]
    #[inline(always)]
    pub const fn usbphy1_ctrl13(&self) -> &Usbphy1Ctrl13 {
        &self.usbphy1_ctrl13
    }
    #[doc = "0x4538 - usbphy1_ctrl14"]
    #[inline(always)]
    pub const fn usbphy1_ctrl14(&self) -> &Usbphy1Ctrl14 {
        &self.usbphy1_ctrl14
    }
    #[doc = "0x453c - usbphy1_ctrl15"]
    #[inline(always)]
    pub const fn usbphy1_ctrl15(&self) -> &Usbphy1Ctrl15 {
        &self.usbphy1_ctrl15
    }
    #[doc = "0x4540 - usbphy1_ctrl16"]
    #[inline(always)]
    pub const fn usbphy1_ctrl16(&self) -> &Usbphy1Ctrl16 {
        &self.usbphy1_ctrl16
    }
    #[doc = "0x4544 - usbphy1_ctrl17"]
    #[inline(always)]
    pub const fn usbphy1_ctrl17(&self) -> &Usbphy1Ctrl17 {
        &self.usbphy1_ctrl17
    }
    #[doc = "0x4548 - usbphy1_ctrl18"]
    #[inline(always)]
    pub const fn usbphy1_ctrl18(&self) -> &Usbphy1Ctrl18 {
        &self.usbphy1_ctrl18
    }
    #[doc = "0x454c - usbphy1_ctrl19"]
    #[inline(always)]
    pub const fn usbphy1_ctrl19(&self) -> &Usbphy1Ctrl19 {
        &self.usbphy1_ctrl19
    }
    #[doc = "0x4550 - usbphy1_ctrl20"]
    #[inline(always)]
    pub const fn usbphy1_ctrl20(&self) -> &Usbphy1Ctrl20 {
        &self.usbphy1_ctrl20
    }
    #[doc = "0x4554 - usbphy1_ctrl21"]
    #[inline(always)]
    pub const fn usbphy1_ctrl21(&self) -> &Usbphy1Ctrl21 {
        &self.usbphy1_ctrl21
    }
    #[doc = "0x4558 - usbphy1_ctrl22"]
    #[inline(always)]
    pub const fn usbphy1_ctrl22(&self) -> &Usbphy1Ctrl22 {
        &self.usbphy1_ctrl22
    }
    #[doc = "0x455c - usbphy1_ctrl23"]
    #[inline(always)]
    pub const fn usbphy1_ctrl23(&self) -> &Usbphy1Ctrl23 {
        &self.usbphy1_ctrl23
    }
    #[doc = "0x4560 - usbphy1_ctrl24"]
    #[inline(always)]
    pub const fn usbphy1_ctrl24(&self) -> &Usbphy1Ctrl24 {
        &self.usbphy1_ctrl24
    }
    #[doc = "0x4564 - usbphy1_ctrl25"]
    #[inline(always)]
    pub const fn usbphy1_ctrl25(&self) -> &Usbphy1Ctrl25 {
        &self.usbphy1_ctrl25
    }
    #[doc = "0x6000 - hdcp performance monitor control register"]
    #[inline(always)]
    pub const fn hdcp22_perf_con0(&self) -> &Hdcp22PerfCon0 {
        &self.hdcp22_perf_con0
    }
    #[doc = "0x6004 - hdcp performance monitor control register"]
    #[inline(always)]
    pub const fn hdcp22_perf_con1(&self) -> &Hdcp22PerfCon1 {
        &self.hdcp22_perf_con1
    }
    #[doc = "0x6008 - hdcp performance monitor control register"]
    #[inline(always)]
    pub const fn hdcp22_perf_con2(&self) -> &Hdcp22PerfCon2 {
        &self.hdcp22_perf_con2
    }
    #[doc = "0x600c - hdcp performance monitor status register"]
    #[inline(always)]
    pub const fn hdcp22_perf_rd_max_latency_num(&self) -> &Hdcp22PerfRdMaxLatencyNum {
        &self.hdcp22_perf_rd_max_latency_num
    }
    #[doc = "0x6010 - hdcp performance monitor status register"]
    #[inline(always)]
    pub const fn hdcp22_perf_rd_latency_samp_num(&self) -> &Hdcp22PerfRdLatencySampNum {
        &self.hdcp22_perf_rd_latency_samp_num
    }
    #[doc = "0x6014 - hdcp performance monitor status register"]
    #[inline(always)]
    pub const fn hdcp22_perf_rd_latency_acc_num(&self) -> &Hdcp22PerfRdLatencyAccNum {
        &self.hdcp22_perf_rd_latency_acc_num
    }
    #[doc = "0x6018 - hdcp performance monitor status register"]
    #[inline(always)]
    pub const fn hdcp22_perf_rd_axi_total_byte(&self) -> &Hdcp22PerfRdAxiTotalByte {
        &self.hdcp22_perf_rd_axi_total_byte
    }
    #[doc = "0x601c - hdcp performance monitor status register"]
    #[inline(always)]
    pub const fn hdcp22_perf_wr_axi_total_byte(&self) -> &Hdcp22PerfWrAxiTotalByte {
        &self.hdcp22_perf_wr_axi_total_byte
    }
    #[doc = "0x6020 - hdcp performance monitor status register"]
    #[inline(always)]
    pub const fn hdcp22_perf_working_cnt(&self) -> &Hdcp22PerfWorkingCnt {
        &self.hdcp22_perf_working_cnt
    }
    #[doc = "0x6224 - SoC control register 9"]
    #[inline(always)]
    pub const fn soc_con9(&self) -> &SocCon9 {
        &self.soc_con9
    }
    #[doc = "0x6250 - SoC control register 20"]
    #[inline(always)]
    pub const fn soc_con20(&self) -> &SocCon20 {
        &self.soc_con20
    }
    #[doc = "0x6254 - SoC control register 21"]
    #[inline(always)]
    pub const fn soc_con21(&self) -> &SocCon21 {
        &self.soc_con21
    }
    #[doc = "0x6258 - SoC control register 22"]
    #[inline(always)]
    pub const fn soc_con22(&self) -> &SocCon22 {
        &self.soc_con22
    }
    #[doc = "0x625c - SoC control register 23"]
    #[inline(always)]
    pub const fn soc_con23(&self) -> &SocCon23 {
        &self.soc_con23
    }
    #[doc = "0x6260 - SoC control register 24"]
    #[inline(always)]
    pub const fn soc_con24(&self) -> &SocCon24 {
        &self.soc_con24
    }
    #[doc = "0x6264 - SoC control register 25"]
    #[inline(always)]
    pub const fn soc_con25(&self) -> &SocCon25 {
        &self.soc_con25
    }
    #[doc = "0x6268 - SoC control register 26"]
    #[inline(always)]
    pub const fn soc_con26(&self) -> &SocCon26 {
        &self.soc_con26
    }
    #[doc = "0x8000 - gpu performance monitor control register"]
    #[inline(always)]
    pub const fn gpu_perf_con0(&self) -> &GpuPerfCon0 {
        &self.gpu_perf_con0
    }
    #[doc = "0x8004 - gpu performance monitor control register"]
    #[inline(always)]
    pub const fn gpu_perf_con1(&self) -> &GpuPerfCon1 {
        &self.gpu_perf_con1
    }
    #[doc = "0x8008 - gpu performance monitor control register"]
    #[inline(always)]
    pub const fn gpu_perf_con2(&self) -> &GpuPerfCon2 {
        &self.gpu_perf_con2
    }
    #[doc = "0x800c - gpu performance monitor status register"]
    #[inline(always)]
    pub const fn gpu_perf_rd_max_latency_num(&self) -> &GpuPerfRdMaxLatencyNum {
        &self.gpu_perf_rd_max_latency_num
    }
    #[doc = "0x8010 - gpu performance monitor status register"]
    #[inline(always)]
    pub const fn gpu_perf_rd_latency_samp_num(&self) -> &GpuPerfRdLatencySampNum {
        &self.gpu_perf_rd_latency_samp_num
    }
    #[doc = "0x8014 - gpu performance monitor status register"]
    #[inline(always)]
    pub const fn gpu_perf_rd_latency_acc_num(&self) -> &GpuPerfRdLatencyAccNum {
        &self.gpu_perf_rd_latency_acc_num
    }
    #[doc = "0x8018 - gpu performance monitor status register"]
    #[inline(always)]
    pub const fn gpu_perf_rd_axi_total_byte(&self) -> &GpuPerfRdAxiTotalByte {
        &self.gpu_perf_rd_axi_total_byte
    }
    #[doc = "0x801c - gpu performance monitor status register"]
    #[inline(always)]
    pub const fn gpu_perf_wr_axi_total_byte(&self) -> &GpuPerfWrAxiTotalByte {
        &self.gpu_perf_wr_axi_total_byte
    }
    #[doc = "0x8020 - gpu performance monitor status register"]
    #[inline(always)]
    pub const fn gpu_perf_working_cnt(&self) -> &GpuPerfWorkingCnt {
        &self.gpu_perf_working_cnt
    }
    #[doc = "0xa000 - cpu control register 0"]
    #[inline(always)]
    pub const fn cpu_con0(&self) -> &CpuCon0 {
        &self.cpu_con0
    }
    #[doc = "0xa004 - cpu control register 1"]
    #[inline(always)]
    pub const fn cpu_con1(&self) -> &CpuCon1 {
        &self.cpu_con1
    }
    #[doc = "0xa008 - cpu control register 2"]
    #[inline(always)]
    pub const fn cpu_con2(&self) -> &CpuCon2 {
        &self.cpu_con2
    }
    #[doc = "0xa00c - cpu control register 3"]
    #[inline(always)]
    pub const fn cpu_con3(&self) -> &CpuCon3 {
        &self.cpu_con3
    }
    #[doc = "0xa080 - cpu status register 0"]
    #[inline(always)]
    pub const fn cpu_status0(&self) -> &CpuStatus0 {
        &self.cpu_status0
    }
    #[doc = "0xa084 - cpu status register 1"]
    #[inline(always)]
    pub const fn cpu_status1(&self) -> &CpuStatus1 {
        &self.cpu_status1
    }
    #[doc = "0xa088 - cpu status register 2"]
    #[inline(always)]
    pub const fn cpu_status2(&self) -> &CpuStatus2 {
        &self.cpu_status2
    }
    #[doc = "0xa08c - cpu status register 3"]
    #[inline(always)]
    pub const fn cpu_status3(&self) -> &CpuStatus3 {
        &self.cpu_status3
    }
    #[doc = "0xa090 - cpu status register 4"]
    #[inline(always)]
    pub const fn cpu_status4(&self) -> &CpuStatus4 {
        &self.cpu_status4
    }
    #[doc = "0xa094 - cpu status register 5"]
    #[inline(always)]
    pub const fn cpu_status5(&self) -> &CpuStatus5 {
        &self.cpu_status5
    }
    #[doc = "0xa100 - a53 performance monitor control register"]
    #[inline(always)]
    pub const fn a53_perf_con0(&self) -> &A53PerfCon0 {
        &self.a53_perf_con0
    }
    #[doc = "0xa104 - a53 performance monitor control register"]
    #[inline(always)]
    pub const fn a53_perf_con1(&self) -> &A53PerfCon1 {
        &self.a53_perf_con1
    }
    #[doc = "0xa108 - a53 performance monitor control register"]
    #[inline(always)]
    pub const fn a53_perf_con2(&self) -> &A53PerfCon2 {
        &self.a53_perf_con2
    }
    #[doc = "0xa10c - a53 performance monitor control register"]
    #[inline(always)]
    pub const fn a53_perf_con3(&self) -> &A53PerfCon3 {
        &self.a53_perf_con3
    }
    #[doc = "0xa110 - performance monitor read start address"]
    #[inline(always)]
    pub const fn a53_perf_rd_mon_st(&self) -> &A53PerfRdMonSt {
        &self.a53_perf_rd_mon_st
    }
    #[doc = "0xa114 - performance monitor end address"]
    #[inline(always)]
    pub const fn a53_perf_rd_mon_end(&self) -> &A53PerfRdMonEnd {
        &self.a53_perf_rd_mon_end
    }
    #[doc = "0xa118 - performance write monitor start address"]
    #[inline(always)]
    pub const fn a53_perf_wr_mon_st(&self) -> &A53PerfWrMonSt {
        &self.a53_perf_wr_mon_st
    }
    #[doc = "0xa11c - performance monitor write end address"]
    #[inline(always)]
    pub const fn a53_perf_wr_mon_end(&self) -> &A53PerfWrMonEnd {
        &self.a53_perf_wr_mon_end
    }
    #[doc = "0xa120 - a53 performance monitor status register"]
    #[inline(always)]
    pub const fn a53_perf_rd_max_latency_num(&self) -> &A53PerfRdMaxLatencyNum {
        &self.a53_perf_rd_max_latency_num
    }
    #[doc = "0xa124 - a53 performance monitor status register"]
    #[inline(always)]
    pub const fn a53_perf_rd_latency_samp_num(&self) -> &A53PerfRdLatencySampNum {
        &self.a53_perf_rd_latency_samp_num
    }
    #[doc = "0xa128 - a53 performance monitor status register"]
    #[inline(always)]
    pub const fn a53_perf_rd_latency_acc_num(&self) -> &A53PerfRdLatencyAccNum {
        &self.a53_perf_rd_latency_acc_num
    }
    #[doc = "0xa12c - a53 performance monitor status register"]
    #[inline(always)]
    pub const fn a53_perf_rd_axi_total_byte(&self) -> &A53PerfRdAxiTotalByte {
        &self.a53_perf_rd_axi_total_byte
    }
    #[doc = "0xa130 - a53 performance monitor status register"]
    #[inline(always)]
    pub const fn a53_perf_wr_axi_total_byte(&self) -> &A53PerfWrAxiTotalByte {
        &self.a53_perf_wr_axi_total_byte
    }
    #[doc = "0xa134 - a53 performance monitor status register"]
    #[inline(always)]
    pub const fn a53_perf_working_cnt(&self) -> &A53PerfWorkingCnt {
        &self.a53_perf_working_cnt
    }
    #[doc = "0xa138 - a53 performance monitor status register"]
    #[inline(always)]
    pub const fn a53_perf_int_status(&self) -> &A53PerfIntStatus {
        &self.a53_perf_int_status
    }
    #[doc = "0xa200 - a72 performance monitor control register"]
    #[inline(always)]
    pub const fn a72_perf_con0(&self) -> &A72PerfCon0 {
        &self.a72_perf_con0
    }
    #[doc = "0xa204 - a72 performance monitor control register"]
    #[inline(always)]
    pub const fn a72_perf_con1(&self) -> &A72PerfCon1 {
        &self.a72_perf_con1
    }
    #[doc = "0xa208 - a72 performance monitor control register"]
    #[inline(always)]
    pub const fn a72_perf_con2(&self) -> &A72PerfCon2 {
        &self.a72_perf_con2
    }
    #[doc = "0xa20c - a72 performance monitor control register"]
    #[inline(always)]
    pub const fn a72_perf_con3(&self) -> &A72PerfCon3 {
        &self.a72_perf_con3
    }
    #[doc = "0xa210 - performance monitor read start address"]
    #[inline(always)]
    pub const fn a72_perf_rd_mon_st(&self) -> &A72PerfRdMonSt {
        &self.a72_perf_rd_mon_st
    }
    #[doc = "0xa214 - performance monitor end address"]
    #[inline(always)]
    pub const fn a72_perf_rd_mon_end(&self) -> &A72PerfRdMonEnd {
        &self.a72_perf_rd_mon_end
    }
    #[doc = "0xa218 - performance write monitor start address"]
    #[inline(always)]
    pub const fn a72_perf_wr_mon_st(&self) -> &A72PerfWrMonSt {
        &self.a72_perf_wr_mon_st
    }
    #[doc = "0xa21c - performance monitor write end address"]
    #[inline(always)]
    pub const fn a72_perf_wr_mon_end(&self) -> &A72PerfWrMonEnd {
        &self.a72_perf_wr_mon_end
    }
    #[doc = "0xa220 - a72 performance monitor status register"]
    #[inline(always)]
    pub const fn a72_perf_rd_max_latency_num(&self) -> &A72PerfRdMaxLatencyNum {
        &self.a72_perf_rd_max_latency_num
    }
    #[doc = "0xa224 - a72 performance monitor status register"]
    #[inline(always)]
    pub const fn a72_perf_rd_latency_samp_num(&self) -> &A72PerfRdLatencySampNum {
        &self.a72_perf_rd_latency_samp_num
    }
    #[doc = "0xa228 - a72 performance monitor status register"]
    #[inline(always)]
    pub const fn a72_perf_rd_latency_acc_num(&self) -> &A72PerfRdLatencyAccNum {
        &self.a72_perf_rd_latency_acc_num
    }
    #[doc = "0xa22c - a72 performance monitor status register"]
    #[inline(always)]
    pub const fn a72_perf_rd_axi_total_byte(&self) -> &A72PerfRdAxiTotalByte {
        &self.a72_perf_rd_axi_total_byte
    }
    #[doc = "0xa230 - a72 performance monitor status register"]
    #[inline(always)]
    pub const fn a72_perf_wr_axi_total_byte(&self) -> &A72PerfWrAxiTotalByte {
        &self.a72_perf_wr_axi_total_byte
    }
    #[doc = "0xa234 - a72 performance monitor status register"]
    #[inline(always)]
    pub const fn a72_perf_working_cnt(&self) -> &A72PerfWorkingCnt {
        &self.a72_perf_working_cnt
    }
    #[doc = "0xa238 - a72 performance monitor status register"]
    #[inline(always)]
    pub const fn a72_perf_int_status(&self) -> &A72PerfIntStatus {
        &self.a72_perf_int_status
    }
    #[doc = "0xc000 - gmac performance monitor control register"]
    #[inline(always)]
    pub const fn gmac_perf_con0(&self) -> &GmacPerfCon0 {
        &self.gmac_perf_con0
    }
    #[doc = "0xc004 - gmac performance monitor control register"]
    #[inline(always)]
    pub const fn gmac_perf_con1(&self) -> &GmacPerfCon1 {
        &self.gmac_perf_con1
    }
    #[doc = "0xc008 - gmac performance monitor control register"]
    #[inline(always)]
    pub const fn gmac_perf_con2(&self) -> &GmacPerfCon2 {
        &self.gmac_perf_con2
    }
    #[doc = "0xc00c - gmac performance monitor status register"]
    #[inline(always)]
    pub const fn gmac_perf_rd_max_latency_num(&self) -> &GmacPerfRdMaxLatencyNum {
        &self.gmac_perf_rd_max_latency_num
    }
    #[doc = "0xc010 - gmac performance monitor status register"]
    #[inline(always)]
    pub const fn gmac_perf_rd_latency_samp_num(&self) -> &GmacPerfRdLatencySampNum {
        &self.gmac_perf_rd_latency_samp_num
    }
    #[doc = "0xc014 - gmac performance monitor status register"]
    #[inline(always)]
    pub const fn gmac_perf_rd_latency_acc_num(&self) -> &GmacPerfRdLatencyAccNum {
        &self.gmac_perf_rd_latency_acc_num
    }
    #[doc = "0xc018 - gmac performance monitor status register"]
    #[inline(always)]
    pub const fn gmac_perf_rd_axi_total_byte(&self) -> &GmacPerfRdAxiTotalByte {
        &self.gmac_perf_rd_axi_total_byte
    }
    #[doc = "0xc01c - gmac performance monitor status register"]
    #[inline(always)]
    pub const fn gmac_perf_wr_axi_total_byte(&self) -> &GmacPerfWrAxiTotalByte {
        &self.gmac_perf_wr_axi_total_byte
    }
    #[doc = "0xc020 - gmac performance monitor status register"]
    #[inline(always)]
    pub const fn gmac_perf_working_cnt(&self) -> &GmacPerfWorkingCnt {
        &self.gmac_perf_working_cnt
    }
    #[doc = "0xc214 - SoC control register 5"]
    #[inline(always)]
    pub const fn soc_con5(&self) -> &SocCon5 {
        &self.soc_con5
    }
    #[doc = "0xc218 - SoC control register 6"]
    #[inline(always)]
    pub const fn soc_con6(&self) -> &SocCon6 {
        &self.soc_con6
    }
    #[doc = "0xe000 - GPIO2A iomux control"]
    #[inline(always)]
    pub const fn gpio2a_iomux(&self) -> &Gpio2aIomux {
        &self.gpio2a_iomux
    }
    #[doc = "0xe004 - GPIO2B iomux control"]
    #[inline(always)]
    pub const fn gpio2b_iomux(&self) -> &Gpio2bIomux {
        &self.gpio2b_iomux
    }
    #[doc = "0xe008 - GPIO2C iomux control"]
    #[inline(always)]
    pub const fn gpio2c_iomux(&self) -> &Gpio2cIomux {
        &self.gpio2c_iomux
    }
    #[doc = "0xe00c - GPIO2D iomux control"]
    #[inline(always)]
    pub const fn gpio2d_iomux(&self) -> &Gpio2dIomux {
        &self.gpio2d_iomux
    }
    #[doc = "0xe010 - GPIO3A iomux control"]
    #[inline(always)]
    pub const fn gpio3a_iomux(&self) -> &Gpio3aIomux {
        &self.gpio3a_iomux
    }
    #[doc = "0xe014 - GPIO3B iomux control"]
    #[inline(always)]
    pub const fn gpio3b_iomux(&self) -> &Gpio3bIomux {
        &self.gpio3b_iomux
    }
    #[doc = "0xe018 - GPIO3C iomux control"]
    #[inline(always)]
    pub const fn gpio3c_iomux(&self) -> &Gpio3cIomux {
        &self.gpio3c_iomux
    }
    #[doc = "0xe01c - GPIO3D iomux control"]
    #[inline(always)]
    pub const fn gpio3d_iomux(&self) -> &Gpio3dIomux {
        &self.gpio3d_iomux
    }
    #[doc = "0xe020 - GPIO4A iomux control"]
    #[inline(always)]
    pub const fn gpio4a_iomux(&self) -> &Gpio4aIomux {
        &self.gpio4a_iomux
    }
    #[doc = "0xe024 - GPIO4B iomux control"]
    #[inline(always)]
    pub const fn gpio4b_iomux(&self) -> &Gpio4bIomux {
        &self.gpio4b_iomux
    }
    #[doc = "0xe028 - GPIO4C iomux control"]
    #[inline(always)]
    pub const fn gpio4c_iomux(&self) -> &Gpio4cIomux {
        &self.gpio4c_iomux
    }
    #[doc = "0xe02c - GPIO4D iomux control"]
    #[inline(always)]
    pub const fn gpio4d_iomux(&self) -> &Gpio4dIomux {
        &self.gpio4d_iomux
    }
    #[doc = "0xe040 - GPIO2A PU/PD control"]
    #[inline(always)]
    pub const fn gpio2a_p(&self) -> &Gpio2aP {
        &self.gpio2a_p
    }
    #[doc = "0xe044 - GPIO2B PU/PD control"]
    #[inline(always)]
    pub const fn gpio2b_p(&self) -> &Gpio2bP {
        &self.gpio2b_p
    }
    #[doc = "0xe048 - GPIO2C PU/PD control"]
    #[inline(always)]
    pub const fn gpio2c_p(&self) -> &Gpio2cP {
        &self.gpio2c_p
    }
    #[doc = "0xe04c - GPIO2D PU/PD control"]
    #[inline(always)]
    pub const fn gpio2d_p(&self) -> &Gpio2dP {
        &self.gpio2d_p
    }
    #[doc = "0xe050 - GPIO3A PU/PD control"]
    #[inline(always)]
    pub const fn gpio3a_p(&self) -> &Gpio3aP {
        &self.gpio3a_p
    }
    #[doc = "0xe054 - GPIO3B PU/PD control"]
    #[inline(always)]
    pub const fn gpio3b_p(&self) -> &Gpio3bP {
        &self.gpio3b_p
    }
    #[doc = "0xe058 - GPIO3C PU/PD control"]
    #[inline(always)]
    pub const fn gpio3c_p(&self) -> &Gpio3cP {
        &self.gpio3c_p
    }
    #[doc = "0xe05c - GPIO3D PU/PD control"]
    #[inline(always)]
    pub const fn gpio3d_p(&self) -> &Gpio3dP {
        &self.gpio3d_p
    }
    #[doc = "0xe060 - GPIO4A PU/PD control"]
    #[inline(always)]
    pub const fn gpio4a_p(&self) -> &Gpio4aP {
        &self.gpio4a_p
    }
    #[doc = "0xe064 - GPIO4B PU/PD control"]
    #[inline(always)]
    pub const fn gpio4b_p(&self) -> &Gpio4bP {
        &self.gpio4b_p
    }
    #[doc = "0xe068 - GPIO4C PU/PD control"]
    #[inline(always)]
    pub const fn gpio4c_p(&self) -> &Gpio4cP {
        &self.gpio4c_p
    }
    #[doc = "0xe06c - GPIO4D PU/PD control"]
    #[inline(always)]
    pub const fn gpio4d_p(&self) -> &Gpio4dP {
        &self.gpio4d_p
    }
    #[doc = "0xe080 - GPIO2A slew rate control"]
    #[inline(always)]
    pub const fn gpio2a_sr(&self) -> &Gpio2aSr {
        &self.gpio2a_sr
    }
    #[doc = "0xe084 - GPIO2B slew rate control"]
    #[inline(always)]
    pub const fn gpio2b_sr(&self) -> &Gpio2bSr {
        &self.gpio2b_sr
    }
    #[doc = "0xe088 - GPIO2C slew rate control"]
    #[inline(always)]
    pub const fn gpio2c_sr(&self) -> &Gpio2cSr {
        &self.gpio2c_sr
    }
    #[doc = "0xe08c - GPIO2D slew rate control"]
    #[inline(always)]
    pub const fn gpio2d_sr(&self) -> &Gpio2dSr {
        &self.gpio2d_sr
    }
    #[doc = "0xe09c - GPIO3D slew rate control"]
    #[inline(always)]
    pub const fn gpio3d_sr(&self) -> &Gpio3dSr {
        &self.gpio3d_sr
    }
    #[doc = "0xe0a0 - GPIO4A slew rate control"]
    #[inline(always)]
    pub const fn gpio4a_sr(&self) -> &Gpio4aSr {
        &self.gpio4a_sr
    }
    #[doc = "0xe0a4 - GPIO4B slew rate control"]
    #[inline(always)]
    pub const fn gpio4b_sr(&self) -> &Gpio4bSr {
        &self.gpio4b_sr
    }
    #[doc = "0xe0a8 - GPIO4C slew rate control"]
    #[inline(always)]
    pub const fn gpio4c_sr(&self) -> &Gpio4cSr {
        &self.gpio4c_sr
    }
    #[doc = "0xe0ac - GPIO4D slew rate control"]
    #[inline(always)]
    pub const fn gpio4d_sr(&self) -> &Gpio4dSr {
        &self.gpio4d_sr
    }
    #[doc = "0xe0c0 - GPIO2A smitter control register"]
    #[inline(always)]
    pub const fn gpio2a_smt(&self) -> &Gpio2aSmt {
        &self.gpio2a_smt
    }
    #[doc = "0xe0c4 - GPIO2B smitter control register"]
    #[inline(always)]
    pub const fn gpio2b_smt(&self) -> &Gpio2bSmt {
        &self.gpio2b_smt
    }
    #[doc = "0xe0c8 - GPIO2C smitter control register"]
    #[inline(always)]
    pub const fn gpio2c_smt(&self) -> &Gpio2cSmt {
        &self.gpio2c_smt
    }
    #[doc = "0xe0cc - GPIO2D smitter control register"]
    #[inline(always)]
    pub const fn gpio2d_smt(&self) -> &Gpio2dSmt {
        &self.gpio2d_smt
    }
    #[doc = "0xe0d0 - GPIO3A smitter control register"]
    #[inline(always)]
    pub const fn gpio3a_smt(&self) -> &Gpio3aSmt {
        &self.gpio3a_smt
    }
    #[doc = "0xe0d4 - GPIO3B smitter control register"]
    #[inline(always)]
    pub const fn gpio3b_smt(&self) -> &Gpio3bSmt {
        &self.gpio3b_smt
    }
    #[doc = "0xe0d8 - GPIO3C smitter control register"]
    #[inline(always)]
    pub const fn gpio3c_smt(&self) -> &Gpio3cSmt {
        &self.gpio3c_smt
    }
    #[doc = "0xe0dc - GPIO3D smitter control register"]
    #[inline(always)]
    pub const fn gpio3d_smt(&self) -> &Gpio3dSmt {
        &self.gpio3d_smt
    }
    #[doc = "0xe0e0 - GPIO4A smitter control register"]
    #[inline(always)]
    pub const fn gpio4a_smt(&self) -> &Gpio4aSmt {
        &self.gpio4a_smt
    }
    #[doc = "0xe0e4 - GPIO4B smitter control register"]
    #[inline(always)]
    pub const fn gpio4b_smt(&self) -> &Gpio4bSmt {
        &self.gpio4b_smt
    }
    #[doc = "0xe0e8 - GPIO4C smitter control register"]
    #[inline(always)]
    pub const fn gpio4c_smt(&self) -> &Gpio4cSmt {
        &self.gpio4c_smt
    }
    #[doc = "0xe0ec - GPIO4D smitter control register"]
    #[inline(always)]
    pub const fn gpio4d_smt(&self) -> &Gpio4dSmt {
        &self.gpio4d_smt
    }
    #[doc = "0xe100 - GPIO2A drive strength control"]
    #[inline(always)]
    pub const fn gpio2a_e(&self) -> &Gpio2aE {
        &self.gpio2a_e
    }
    #[doc = "0xe104 - GPIO2B drive strength control"]
    #[inline(always)]
    pub const fn gpio2b_e(&self) -> &Gpio2bE {
        &self.gpio2b_e
    }
    #[doc = "0xe108 - GPIO2C drive strength control"]
    #[inline(always)]
    pub const fn gpio2c_e(&self) -> &Gpio2cE {
        &self.gpio2c_e
    }
    #[doc = "0xe10c - GPIO2D drive strength control"]
    #[inline(always)]
    pub const fn gpio2d_e(&self) -> &Gpio2dE {
        &self.gpio2d_e
    }
    #[doc = "0xe110 - GPIO3A drive strength control"]
    #[inline(always)]
    pub const fn gpio3a_e01(&self) -> &Gpio3aE01 {
        &self.gpio3a_e01
    }
    #[doc = "0xe114 - GPIO3B drive strength control"]
    #[inline(always)]
    pub const fn gpio3a_e2(&self) -> &Gpio3aE2 {
        &self.gpio3a_e2
    }
    #[doc = "0xe118 - GPIO3B drive strength control"]
    #[inline(always)]
    pub const fn gpio3b_e01(&self) -> &Gpio3bE01 {
        &self.gpio3b_e01
    }
    #[doc = "0xe11c - GPIO3B drive strength control"]
    #[inline(always)]
    pub const fn gpio3b_e2(&self) -> &Gpio3bE2 {
        &self.gpio3b_e2
    }
    #[doc = "0xe120 - GPIO3C drive strength control"]
    #[inline(always)]
    pub const fn gpio3c_e01(&self) -> &Gpio3cE01 {
        &self.gpio3c_e01
    }
    #[doc = "0xe124 - GPIO3C drive strength control"]
    #[inline(always)]
    pub const fn gpio3c_e2(&self) -> &Gpio3cE2 {
        &self.gpio3c_e2
    }
    #[doc = "0xe128 - GPIO3D drive strength control"]
    #[inline(always)]
    pub const fn gpio3d_e(&self) -> &Gpio3dE {
        &self.gpio3d_e
    }
    #[doc = "0xe12c - GPIO4A drive strength control"]
    #[inline(always)]
    pub const fn gpio4a_e(&self) -> &Gpio4aE {
        &self.gpio4a_e
    }
    #[doc = "0xe130 - GPIO4B drive strength control"]
    #[inline(always)]
    pub const fn gpio4b_e01(&self) -> &Gpio4bE01 {
        &self.gpio4b_e01
    }
    #[doc = "0xe134 - GPIO4B drive strength control"]
    #[inline(always)]
    pub const fn gpio4b_e2(&self) -> &Gpio4bE2 {
        &self.gpio4b_e2
    }
    #[doc = "0xe138 - GPIO4C drive strength control"]
    #[inline(always)]
    pub const fn gpio4c_e(&self) -> &Gpio4cE {
        &self.gpio4c_e
    }
    #[doc = "0xe13c - GPIO4D drive strength control"]
    #[inline(always)]
    pub const fn gpio4d_e(&self) -> &Gpio4dE {
        &self.gpio4d_e
    }
    #[doc = "0xe188 - GPIO2C HE control"]
    #[inline(always)]
    pub const fn gpio2c_he(&self) -> &Gpio2cHe {
        &self.gpio2c_he
    }
    #[doc = "0xe18c - GPIO2D HE control"]
    #[inline(always)]
    pub const fn gpio2d_he(&self) -> &Gpio2dHe {
        &self.gpio2d_he
    }
    #[doc = "0xe200 - SoC control register 0"]
    #[inline(always)]
    pub const fn soc_con0(&self) -> &SocCon0 {
        &self.soc_con0
    }
    #[doc = "0xe204 - SoC control register 2"]
    #[inline(always)]
    pub const fn soc_con1(&self) -> &SocCon1 {
        &self.soc_con1
    }
    #[doc = "0xe208 - SoC control register 1"]
    #[inline(always)]
    pub const fn soc_con2(&self) -> &SocCon2 {
        &self.soc_con2
    }
    #[doc = "0xe20c - SoC control register 3"]
    #[inline(always)]
    pub const fn soc_con3(&self) -> &SocCon3 {
        &self.soc_con3
    }
    #[doc = "0xe210 - SoC control register 4"]
    #[inline(always)]
    pub const fn soc_con4(&self) -> &SocCon4 {
        &self.soc_con4
    }
    #[doc = "0xe214 - SoC control register 5"]
    #[inline(always)]
    pub const fn soc_con_5_pcie(&self) -> &SocCon5Pcie {
        &self.soc_con_5_pcie
    }
    #[doc = "0xe21c - SoC control register 7"]
    #[inline(always)]
    pub const fn soc_con7(&self) -> &SocCon7 {
        &self.soc_con7
    }
    #[doc = "0xe220 - SoC control register 8"]
    #[inline(always)]
    pub const fn soc_con8(&self) -> &SocCon8 {
        &self.soc_con8
    }
    #[doc = "0xe224 - SoC control register 9 for PCIE"]
    #[inline(always)]
    pub const fn soc_con_9_pcie(&self) -> &SocCon9Pcie {
        &self.soc_con_9_pcie
    }
    #[doc = "0xe2a0 - SOC status register 0"]
    #[inline(always)]
    pub const fn soc_status0(&self) -> &SocStatus0 {
        &self.soc_status0
    }
    #[doc = "0xe2a4 - SOC status register 1"]
    #[inline(always)]
    pub const fn soc_status1(&self) -> &SocStatus1 {
        &self.soc_status1
    }
    #[doc = "0xe2a8 - SOC status register 2"]
    #[inline(always)]
    pub const fn soc_status2(&self) -> &SocStatus2 {
        &self.soc_status2
    }
    #[doc = "0xe2ac - SOC status register 3"]
    #[inline(always)]
    pub const fn soc_status3(&self) -> &SocStatus3 {
        &self.soc_status3
    }
    #[doc = "0xe2b0 - SOC status register 4"]
    #[inline(always)]
    pub const fn soc_status4(&self) -> &SocStatus4 {
        &self.soc_status4
    }
    #[doc = "0xe2b4 - SOC status register 5"]
    #[inline(always)]
    pub const fn soc_status5(&self) -> &SocStatus5 {
        &self.soc_status5
    }
    #[doc = "0xe380 - ddrc0 control register 0"]
    #[inline(always)]
    pub const fn ddrc0_con0(&self) -> &Ddrc0Con0 {
        &self.ddrc0_con0
    }
    #[doc = "0xe384 - ddrc0 control register 1"]
    #[inline(always)]
    pub const fn ddrc0_con1(&self) -> &Ddrc0Con1 {
        &self.ddrc0_con1
    }
    #[doc = "0xe388 - ddrc1 control register 0"]
    #[inline(always)]
    pub const fn ddrc1_con0(&self) -> &Ddrc1Con0 {
        &self.ddrc1_con0
    }
    #[doc = "0xe38c - ddrc1 control register 1"]
    #[inline(always)]
    pub const fn ddrc1_con1(&self) -> &Ddrc1Con1 {
        &self.ddrc1_con1
    }
    #[doc = "0xe3c0 - Singal detect control register0"]
    #[inline(always)]
    pub const fn sig_detect_con0(&self) -> &SigDetectCon0 {
        &self.sig_detect_con0
    }
    #[doc = "0xe3c8 - Singal detect control register1"]
    #[inline(always)]
    pub const fn sig_detect_con1(&self) -> &SigDetectCon1 {
        &self.sig_detect_con1
    }
    #[doc = "0xe3d0 - Signal detect status clear register"]
    #[inline(always)]
    pub const fn sig_detect_clr(&self) -> &SigDetectClr {
        &self.sig_detect_clr
    }
    #[doc = "0xe3e0 - Signal detect status register"]
    #[inline(always)]
    pub const fn sig_detect_status(&self) -> &SigDetectStatus {
        &self.sig_detect_status
    }
    #[doc = "0xe450 - USB20 PHY0 GRF Register 0"]
    #[inline(always)]
    pub const fn usb20_phy0_con0(&self) -> &Usb20Phy0Con0 {
        &self.usb20_phy0_con0
    }
    #[doc = "0xe454 - USB20 PHY0 GRF Register 1"]
    #[inline(always)]
    pub const fn usb20_phy0_con1(&self) -> &Usb20Phy0Con1 {
        &self.usb20_phy0_con1
    }
    #[doc = "0xe458 - USB20 PHY0 GRF Register 2"]
    #[inline(always)]
    pub const fn usb20_phy0_con2(&self) -> &Usb20Phy0Con2 {
        &self.usb20_phy0_con2
    }
    #[doc = "0xe45c - USB20 PHY0 GRF Register 3"]
    #[inline(always)]
    pub const fn usb20_phy0_con3(&self) -> &Usb20Phy0Con3 {
        &self.usb20_phy0_con3
    }
    #[doc = "0xe460 - USB20 PHY1 GRF Register 0"]
    #[inline(always)]
    pub const fn usb20_phy1_con0(&self) -> &Usb20Phy1Con0 {
        &self.usb20_phy1_con0
    }
    #[doc = "0xe464 - USB20 PHY1GRF Register 1"]
    #[inline(always)]
    pub const fn usb20_phy1_con1(&self) -> &Usb20Phy1Con1 {
        &self.usb20_phy1_con1
    }
    #[doc = "0xe468 - USB20 PHY1 GRF Register 2"]
    #[inline(always)]
    pub const fn usb20_phy1_con2(&self) -> &Usb20Phy1Con2 {
        &self.usb20_phy1_con2
    }
    #[doc = "0xe46c - USB20 PHY1 GRF Register 3"]
    #[inline(always)]
    pub const fn usb20_phy1_con3(&self) -> &Usb20Phy1Con3 {
        &self.usb20_phy1_con3
    }
    #[doc = "0xe580 - TypeC PHY/TCPD PHY/TCPC Control register0"]
    #[inline(always)]
    pub const fn usb3phy0_con0(&self) -> &Usb3phy0Con0 {
        &self.usb3phy0_con0
    }
    #[doc = "0xe584 - TypeC PHY/TCPD PHY/TCPC Control register1"]
    #[inline(always)]
    pub const fn usb3phy0_con1(&self) -> &Usb3phy0Con1 {
        &self.usb3phy0_con1
    }
    #[doc = "0xe588 - TypeC PHY/TCPD PHY/TCPC Control register2"]
    #[inline(always)]
    pub const fn usb3phy0_con2(&self) -> &Usb3phy0Con2 {
        &self.usb3phy0_con2
    }
    #[doc = "0xe58c - TypeC PHY/TCPD PHY/TCPC Control register0"]
    #[inline(always)]
    pub const fn usb3phy1_con0(&self) -> &Usb3phy1Con0 {
        &self.usb3phy1_con0
    }
    #[doc = "0xe590 - TypeC PHY/TCPD PHY/TCPC Control register1"]
    #[inline(always)]
    pub const fn usb3phy1_con1(&self) -> &Usb3phy1Con1 {
        &self.usb3phy1_con1
    }
    #[doc = "0xe594 - TypeC PHY/TCPD PHY/TCPC Control register2"]
    #[inline(always)]
    pub const fn usb3phy1_con2(&self) -> &Usb3phy1Con2 {
        &self.usb3phy1_con2
    }
    #[doc = "0xe5c0 - USB3PHY_STATUS0"]
    #[inline(always)]
    pub const fn usb3phy_status0(&self) -> &Usb3phyStatus0 {
        &self.usb3phy_status0
    }
    #[doc = "0xe5c4 - USB3PHY_STATUS1"]
    #[inline(always)]
    pub const fn usb3phy_status1(&self) -> &Usb3phyStatus1 {
        &self.usb3phy_status1
    }
    #[doc = "0xe600 - pvtm control register"]
    #[inline(always)]
    pub const fn dll_con0(&self) -> &DllCon0 {
        &self.dll_con0
    }
    #[doc = "0xe604 - pvtm control register"]
    #[inline(always)]
    pub const fn dll_con1(&self) -> &DllCon1 {
        &self.dll_con1
    }
    #[doc = "0xe608 - pvtm control register"]
    #[inline(always)]
    pub const fn dll_con2(&self) -> &DllCon2 {
        &self.dll_con2
    }
    #[doc = "0xe60c - pvtm control register"]
    #[inline(always)]
    pub const fn dll_con3(&self) -> &DllCon3 {
        &self.dll_con3
    }
    #[doc = "0xe610 - pvtm control register"]
    #[inline(always)]
    pub const fn dll_con4(&self) -> &DllCon4 {
        &self.dll_con4
    }
    #[doc = "0xe614 - pvtm control register"]
    #[inline(always)]
    pub const fn dll_con5(&self) -> &DllCon5 {
        &self.dll_con5
    }
    #[doc = "0xe620 - pvtm status register"]
    #[inline(always)]
    pub const fn dll_status0(&self) -> &DllStatus0 {
        &self.dll_status0
    }
    #[doc = "0xe624 - pvtm status register"]
    #[inline(always)]
    pub const fn dll_status1(&self) -> &DllStatus1 {
        &self.dll_status1
    }
    #[doc = "0xe628 - pvtm status register"]
    #[inline(always)]
    pub const fn dll_status2(&self) -> &DllStatus2 {
        &self.dll_status2
    }
    #[doc = "0xe62c - pvtm status register"]
    #[inline(always)]
    pub const fn dll_status3(&self) -> &DllStatus3 {
        &self.dll_status3
    }
    #[doc = "0xe630 - pvtm status register"]
    #[inline(always)]
    pub const fn dll_status4(&self) -> &DllStatus4 {
        &self.dll_status4
    }
    #[doc = "0xe640 - "]
    #[inline(always)]
    pub const fn io_vsel(&self) -> &IoVsel {
        &self.io_vsel
    }
    #[doc = "0xe644 - saradc test bit control register"]
    #[inline(always)]
    pub const fn saradc_testbit(&self) -> &SaradcTestbit {
        &self.saradc_testbit
    }
    #[doc = "0xe648 - saradc test bit control register"]
    #[inline(always)]
    pub const fn tsadc_testbit_l(&self) -> &TsadcTestbitL {
        &self.tsadc_testbit_l
    }
    #[doc = "0xe64c - tsadc test bit control register"]
    #[inline(always)]
    pub const fn tsadc_testbit_h(&self) -> &TsadcTestbitH {
        &self.tsadc_testbit_h
    }
    #[doc = "0xe800 - chip id register"]
    #[inline(always)]
    pub const fn chip_id_addr(&self) -> &ChipIdAddr {
        &self.chip_id_addr
    }
    #[doc = "0xe880 - faster boot address register"]
    #[inline(always)]
    pub const fn fast_boot_addr(&self) -> &FastBootAddr {
        &self.fast_boot_addr
    }
    #[doc = "0xf000 - emmc core control register"]
    #[inline(always)]
    pub const fn emmccore_con0(&self) -> &EmmccoreCon0 {
        &self.emmccore_con0
    }
    #[doc = "0xf004 - emmc core control register"]
    #[inline(always)]
    pub const fn emmccore_con1(&self) -> &EmmccoreCon1 {
        &self.emmccore_con1
    }
    #[doc = "0xf008 - emmc core control register"]
    #[inline(always)]
    pub const fn emmccore_con2(&self) -> &EmmccoreCon2 {
        &self.emmccore_con2
    }
    #[doc = "0xf00c - emmc core control register"]
    #[inline(always)]
    pub const fn emmccore_con3(&self) -> &EmmccoreCon3 {
        &self.emmccore_con3
    }
    #[doc = "0xf010 - emmc core control register"]
    #[inline(always)]
    pub const fn emmccore_con4(&self) -> &EmmccoreCon4 {
        &self.emmccore_con4
    }
    #[doc = "0xf014 - emmc core control register"]
    #[inline(always)]
    pub const fn emmccore_con5(&self) -> &EmmccoreCon5 {
        &self.emmccore_con5
    }
    #[doc = "0xf018 - emmc core control register"]
    #[inline(always)]
    pub const fn emmccore_con6(&self) -> &EmmccoreCon6 {
        &self.emmccore_con6
    }
    #[doc = "0xf01c - emmc core control register"]
    #[inline(always)]
    pub const fn emmccore_con7(&self) -> &EmmccoreCon7 {
        &self.emmccore_con7
    }
    #[doc = "0xf020 - emmc core control register"]
    #[inline(always)]
    pub const fn emmccore_con8(&self) -> &EmmccoreCon8 {
        &self.emmccore_con8
    }
    #[doc = "0xf024 - emmc core control register"]
    #[inline(always)]
    pub const fn emmccore_con9(&self) -> &EmmccoreCon9 {
        &self.emmccore_con9
    }
    #[doc = "0xf028 - emmc core control register"]
    #[inline(always)]
    pub const fn emmccore_con10(&self) -> &EmmccoreCon10 {
        &self.emmccore_con10
    }
    #[doc = "0xf02c - emmc core control register"]
    #[inline(always)]
    pub const fn emmccore_con11(&self) -> &EmmccoreCon11 {
        &self.emmccore_con11
    }
    #[doc = "0xf040 - emmc core status register"]
    #[inline(always)]
    pub const fn emmccore_status0(&self) -> &EmmccoreStatus0 {
        &self.emmccore_status0
    }
    #[doc = "0xf044 - emmc core status register"]
    #[inline(always)]
    pub const fn emmccore_status1(&self) -> &EmmccoreStatus1 {
        &self.emmccore_status1
    }
    #[doc = "0xf048 - emmc core status register"]
    #[inline(always)]
    pub const fn emmccore_status2(&self) -> &EmmccoreStatus2 {
        &self.emmccore_status2
    }
    #[doc = "0xf04c - emmc core status register"]
    #[inline(always)]
    pub const fn emmccore_status3(&self) -> &EmmccoreStatus3 {
        &self.emmccore_status3
    }
    #[doc = "0xf780 - emmc phy control register"]
    #[inline(always)]
    pub const fn emmcphy_con0(&self) -> &EmmcphyCon0 {
        &self.emmcphy_con0
    }
    #[doc = "0xf784 - emmc phy control register"]
    #[inline(always)]
    pub const fn emmcphy_con1(&self) -> &EmmcphyCon1 {
        &self.emmcphy_con1
    }
    #[doc = "0xf788 - emmc phy control register"]
    #[inline(always)]
    pub const fn emmcphy_con2(&self) -> &EmmcphyCon2 {
        &self.emmcphy_con2
    }
    #[doc = "0xf78c - emmc phy control register"]
    #[inline(always)]
    pub const fn emmcphy_con3(&self) -> &EmmcphyCon3 {
        &self.emmcphy_con3
    }
    #[doc = "0xf790 - emmc phy control register"]
    #[inline(always)]
    pub const fn emmcphy_con4(&self) -> &EmmcphyCon4 {
        &self.emmcphy_con4
    }
    #[doc = "0xf794 - emmc phy control register"]
    #[inline(always)]
    pub const fn emmcphy_con5(&self) -> &EmmcphyCon5 {
        &self.emmcphy_con5
    }
    #[doc = "0xf798 - emmc phy control register"]
    #[inline(always)]
    pub const fn emmcphy_con6(&self) -> &EmmcphyCon6 {
        &self.emmcphy_con6
    }
    #[doc = "0xf7a0 - emmc phy status register"]
    #[inline(always)]
    pub const fn emmcphy_status(&self) -> &EmmcphyStatus {
        &self.emmcphy_status
    }
}
#[doc = "USB3_PERF_CON0 (rw) register accessor: usb3 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_perf_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_perf_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_perf_con0`]
module"]
#[doc(alias = "USB3_PERF_CON0")]
pub type Usb3PerfCon0 = crate::Reg<usb3_perf_con0::Usb3PerfCon0Spec>;
#[doc = "usb3 performance monitor control register"]
pub mod usb3_perf_con0;
#[doc = "USB3_PERF_CON1 (rw) register accessor: usb3 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_perf_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_perf_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_perf_con1`]
module"]
#[doc(alias = "USB3_PERF_CON1")]
pub type Usb3PerfCon1 = crate::Reg<usb3_perf_con1::Usb3PerfCon1Spec>;
#[doc = "usb3 performance monitor control register"]
pub mod usb3_perf_con1;
#[doc = "USB3_PERF_CON2 (rw) register accessor: usb3 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_perf_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_perf_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_perf_con2`]
module"]
#[doc(alias = "USB3_PERF_CON2")]
pub type Usb3PerfCon2 = crate::Reg<usb3_perf_con2::Usb3PerfCon2Spec>;
#[doc = "usb3 performance monitor control register"]
pub mod usb3_perf_con2;
#[doc = "USB3_PERF_RD_MAX_LATENCY_NUM (rw) register accessor: usb3 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_perf_rd_max_latency_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_perf_rd_max_latency_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_perf_rd_max_latency_num`]
module"]
#[doc(alias = "USB3_PERF_RD_MAX_LATENCY_NUM")]
pub type Usb3PerfRdMaxLatencyNum =
    crate::Reg<usb3_perf_rd_max_latency_num::Usb3PerfRdMaxLatencyNumSpec>;
#[doc = "usb3 performance monitor status register"]
pub mod usb3_perf_rd_max_latency_num;
#[doc = "USB3_PERF_RD_LATENCY_SAMP_NUM (rw) register accessor: usb3 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_perf_rd_latency_samp_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_perf_rd_latency_samp_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_perf_rd_latency_samp_num`]
module"]
#[doc(alias = "USB3_PERF_RD_LATENCY_SAMP_NUM")]
pub type Usb3PerfRdLatencySampNum =
    crate::Reg<usb3_perf_rd_latency_samp_num::Usb3PerfRdLatencySampNumSpec>;
#[doc = "usb3 performance monitor status register"]
pub mod usb3_perf_rd_latency_samp_num;
#[doc = "USB3_PERF_RD_LATENCY_ACC_NUM (rw) register accessor: usb3 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_perf_rd_latency_acc_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_perf_rd_latency_acc_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_perf_rd_latency_acc_num`]
module"]
#[doc(alias = "USB3_PERF_RD_LATENCY_ACC_NUM")]
pub type Usb3PerfRdLatencyAccNum =
    crate::Reg<usb3_perf_rd_latency_acc_num::Usb3PerfRdLatencyAccNumSpec>;
#[doc = "usb3 performance monitor status register"]
pub mod usb3_perf_rd_latency_acc_num;
#[doc = "USB3_PERF_RD_AXI_TOTAL_BYTE (rw) register accessor: usb3 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_perf_rd_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_perf_rd_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_perf_rd_axi_total_byte`]
module"]
#[doc(alias = "USB3_PERF_RD_AXI_TOTAL_BYTE")]
pub type Usb3PerfRdAxiTotalByte =
    crate::Reg<usb3_perf_rd_axi_total_byte::Usb3PerfRdAxiTotalByteSpec>;
#[doc = "usb3 performance monitor status register"]
pub mod usb3_perf_rd_axi_total_byte;
#[doc = "USB3_PERF_WR_AXI_TOTAL_BYTE (rw) register accessor: usb3 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_perf_wr_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_perf_wr_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_perf_wr_axi_total_byte`]
module"]
#[doc(alias = "USB3_PERF_WR_AXI_TOTAL_BYTE")]
pub type Usb3PerfWrAxiTotalByte =
    crate::Reg<usb3_perf_wr_axi_total_byte::Usb3PerfWrAxiTotalByteSpec>;
#[doc = "usb3 performance monitor status register"]
pub mod usb3_perf_wr_axi_total_byte;
#[doc = "USB3_PERF_WORKING_CNT (rw) register accessor: usb3 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_perf_working_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_perf_working_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3_perf_working_cnt`]
module"]
#[doc(alias = "USB3_PERF_WORKING_CNT")]
pub type Usb3PerfWorkingCnt = crate::Reg<usb3_perf_working_cnt::Usb3PerfWorkingCntSpec>;
#[doc = "usb3 performance monitor status register"]
pub mod usb3_perf_working_cnt;
#[doc = "USB3OTG0_CON0 (rw) register accessor: USB3 OTG0 GRF Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3otg0_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3otg0_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3otg0_con0`]
module"]
#[doc(alias = "USB3OTG0_CON0")]
pub type Usb3otg0Con0 = crate::Reg<usb3otg0_con0::Usb3otg0Con0Spec>;
#[doc = "USB3 OTG0 GRF Register0"]
pub mod usb3otg0_con0;
#[doc = "USB3OTG0_CON1 (rw) register accessor: USB3 OTG0 GRF Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3otg0_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3otg0_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3otg0_con1`]
module"]
#[doc(alias = "USB3OTG0_CON1")]
pub type Usb3otg0Con1 = crate::Reg<usb3otg0_con1::Usb3otg0Con1Spec>;
#[doc = "USB3 OTG0 GRF Register1"]
pub mod usb3otg0_con1;
#[doc = "USB3OTG1_CON0 (rw) register accessor: USB3 OTG1 GRF Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3otg1_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3otg1_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3otg1_con0`]
module"]
#[doc(alias = "USB3OTG1_CON0")]
pub type Usb3otg1Con0 = crate::Reg<usb3otg1_con0::Usb3otg1Con0Spec>;
#[doc = "USB3 OTG1 GRF Register0"]
pub mod usb3otg1_con0;
#[doc = "USB3OTG1_CON1 (rw) register accessor: USB3 OTG1 GRF Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3otg1_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3otg1_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3otg1_con1`]
module"]
#[doc(alias = "USB3OTG1_CON1")]
pub type Usb3otg1Con1 = crate::Reg<usb3otg1_con1::Usb3otg1Con1Spec>;
#[doc = "USB3 OTG1 GRF Register1"]
pub mod usb3otg1_con1;
#[doc = "USB3OTG0_STATUS_LAT0 (rw) register accessor: USB3 OTG0 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3otg0_status_lat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3otg0_status_lat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3otg0_status_lat0`]
module"]
#[doc(alias = "USB3OTG0_STATUS_LAT0")]
pub type Usb3otg0StatusLat0 = crate::Reg<usb3otg0_status_lat0::Usb3otg0StatusLat0Spec>;
#[doc = "USB3 OTG0 status register"]
pub mod usb3otg0_status_lat0;
#[doc = "USB3OTG0_STATUS_LAT1 (rw) register accessor: USB3 OTG1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3otg0_status_lat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3otg0_status_lat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3otg0_status_lat1`]
module"]
#[doc(alias = "USB3OTG0_STATUS_LAT1")]
pub type Usb3otg0StatusLat1 = crate::Reg<usb3otg0_status_lat1::Usb3otg0StatusLat1Spec>;
#[doc = "USB3 OTG1 status register"]
pub mod usb3otg0_status_lat1;
#[doc = "USB3OTG0_STATUS_CB (rw) register accessor: USB3 OTG0 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3otg0_status_cb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3otg0_status_cb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3otg0_status_cb`]
module"]
#[doc(alias = "USB3OTG0_STATUS_CB")]
pub type Usb3otg0StatusCb = crate::Reg<usb3otg0_status_cb::Usb3otg0StatusCbSpec>;
#[doc = "USB3 OTG0 status register"]
pub mod usb3otg0_status_cb;
#[doc = "USB3OTG1_STATUS_LAT0 (rw) register accessor: USB3 OTG1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3otg1_status_lat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3otg1_status_lat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3otg1_status_lat0`]
module"]
#[doc(alias = "USB3OTG1_STATUS_LAT0")]
pub type Usb3otg1StatusLat0 = crate::Reg<usb3otg1_status_lat0::Usb3otg1StatusLat0Spec>;
#[doc = "USB3 OTG1 status register"]
pub mod usb3otg1_status_lat0;
#[doc = "USB3OTG1_STATUS_LAT1 (rw) register accessor: USB3 OTG1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3otg1_status_lat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3otg1_status_lat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3otg1_status_lat1`]
module"]
#[doc(alias = "USB3OTG1_STATUS_LAT1")]
pub type Usb3otg1StatusLat1 = crate::Reg<usb3otg1_status_lat1::Usb3otg1StatusLat1Spec>;
#[doc = "USB3 OTG1 status register"]
pub mod usb3otg1_status_lat1;
#[doc = "USB3OTG1_STATUS_CB (rw) register accessor: USB3 OTG1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3otg1_status_cb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3otg1_status_cb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3otg1_status_cb`]
module"]
#[doc(alias = "USB3OTG1_STATUS_CB")]
pub type Usb3otg1StatusCb = crate::Reg<usb3otg1_status_cb::Usb3otg1StatusCbSpec>;
#[doc = "USB3 OTG1 status register"]
pub mod usb3otg1_status_cb;
#[doc = "PCIE_PERF_CON0 (rw) register accessor: pcie performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_perf_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_perf_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_perf_con0`]
module"]
#[doc(alias = "PCIE_PERF_CON0")]
pub type PciePerfCon0 = crate::Reg<pcie_perf_con0::PciePerfCon0Spec>;
#[doc = "pcie performance monitor control register"]
pub mod pcie_perf_con0;
#[doc = "PCIE_PERF_CON1 (rw) register accessor: pcie performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_perf_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_perf_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_perf_con1`]
module"]
#[doc(alias = "PCIE_PERF_CON1")]
pub type PciePerfCon1 = crate::Reg<pcie_perf_con1::PciePerfCon1Spec>;
#[doc = "pcie performance monitor control register"]
pub mod pcie_perf_con1;
#[doc = "PCIE_PERF_CON2 (rw) register accessor: pcie performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_perf_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_perf_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_perf_con2`]
module"]
#[doc(alias = "PCIE_PERF_CON2")]
pub type PciePerfCon2 = crate::Reg<pcie_perf_con2::PciePerfCon2Spec>;
#[doc = "pcie performance monitor control register"]
pub mod pcie_perf_con2;
#[doc = "PCIE_PERF_RD_MAX_LATENCY_NUM (rw) register accessor: pcieperformance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_perf_rd_max_latency_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_perf_rd_max_latency_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_perf_rd_max_latency_num`]
module"]
#[doc(alias = "PCIE_PERF_RD_MAX_LATENCY_NUM")]
pub type PciePerfRdMaxLatencyNum =
    crate::Reg<pcie_perf_rd_max_latency_num::PciePerfRdMaxLatencyNumSpec>;
#[doc = "pcieperformance monitor status register"]
pub mod pcie_perf_rd_max_latency_num;
#[doc = "PCIE_PERF_RD_LATENCY_SAMP_NUM (rw) register accessor: pcie performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_perf_rd_latency_samp_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_perf_rd_latency_samp_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_perf_rd_latency_samp_num`]
module"]
#[doc(alias = "PCIE_PERF_RD_LATENCY_SAMP_NUM")]
pub type PciePerfRdLatencySampNum =
    crate::Reg<pcie_perf_rd_latency_samp_num::PciePerfRdLatencySampNumSpec>;
#[doc = "pcie performance monitor status register"]
pub mod pcie_perf_rd_latency_samp_num;
#[doc = "PCIE_PERF_RD_LATENCY_ACC_NUM (rw) register accessor: pcie performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_perf_rd_latency_acc_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_perf_rd_latency_acc_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_perf_rd_latency_acc_num`]
module"]
#[doc(alias = "PCIE_PERF_RD_LATENCY_ACC_NUM")]
pub type PciePerfRdLatencyAccNum =
    crate::Reg<pcie_perf_rd_latency_acc_num::PciePerfRdLatencyAccNumSpec>;
#[doc = "pcie performance monitor status register"]
pub mod pcie_perf_rd_latency_acc_num;
#[doc = "PCIE_PERF_RD_AXI_TOTAL_BYTE (rw) register accessor: pcie performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_perf_rd_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_perf_rd_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_perf_rd_axi_total_byte`]
module"]
#[doc(alias = "PCIE_PERF_RD_AXI_TOTAL_BYTE")]
pub type PciePerfRdAxiTotalByte =
    crate::Reg<pcie_perf_rd_axi_total_byte::PciePerfRdAxiTotalByteSpec>;
#[doc = "pcie performance monitor status register"]
pub mod pcie_perf_rd_axi_total_byte;
#[doc = "PCIE_PERF_WR_AXI_TOTAL_BYTE (rw) register accessor: pcie performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_perf_wr_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_perf_wr_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_perf_wr_axi_total_byte`]
module"]
#[doc(alias = "PCIE_PERF_WR_AXI_TOTAL_BYTE")]
pub type PciePerfWrAxiTotalByte =
    crate::Reg<pcie_perf_wr_axi_total_byte::PciePerfWrAxiTotalByteSpec>;
#[doc = "pcie performance monitor status register"]
pub mod pcie_perf_wr_axi_total_byte;
#[doc = "PCIE_PERF_WORKING_CNT (rw) register accessor: pcie performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_perf_working_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_perf_working_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_perf_working_cnt`]
module"]
#[doc(alias = "PCIE_PERF_WORKING_CNT")]
pub type PciePerfWorkingCnt = crate::Reg<pcie_perf_working_cnt::PciePerfWorkingCntSpec>;
#[doc = "pcie performance monitor status register"]
pub mod pcie_perf_working_cnt;
#[doc = "USB20_HOST0_CON0 (rw) register accessor: USB20 Host0 GRF register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb20_host0_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb20_host0_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb20_host0_con0`]
module"]
#[doc(alias = "USB20_HOST0_CON0")]
pub type Usb20Host0Con0 = crate::Reg<usb20_host0_con0::Usb20Host0Con0Spec>;
#[doc = "USB20 Host0 GRF register0"]
pub mod usb20_host0_con0;
#[doc = "USB20_HOST0_CON1 (rw) register accessor: USB20 Host0 GRF register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb20_host0_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb20_host0_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb20_host0_con1`]
module"]
#[doc(alias = "USB20_HOST0_CON1")]
pub type Usb20Host0Con1 = crate::Reg<usb20_host0_con1::Usb20Host0Con1Spec>;
#[doc = "USB20 Host0 GRF register1"]
pub mod usb20_host0_con1;
#[doc = "USB20_HOST1_CON0 (rw) register accessor: USB20 Host1 GRF register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb20_host1_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb20_host1_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb20_host1_con0`]
module"]
#[doc(alias = "USB20_HOST1_CON0")]
pub type Usb20Host1Con0 = crate::Reg<usb20_host1_con0::Usb20Host1Con0Spec>;
#[doc = "USB20 Host1 GRF register0"]
pub mod usb20_host1_con0;
#[doc = "USB20_HOST1_CON1 (rw) register accessor: USB20 Host1 GRF register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb20_host1_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb20_host1_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb20_host1_con1`]
module"]
#[doc(alias = "USB20_HOST1_CON1")]
pub type Usb20Host1Con1 = crate::Reg<usb20_host1_con1::Usb20Host1Con1Spec>;
#[doc = "USB20 Host1 GRF register1"]
pub mod usb20_host1_con1;
#[doc = "HSIC_CON0 (rw) register accessor: HSIC controller GRF register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsic_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsic_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsic_con0`]
module"]
#[doc(alias = "HSIC_CON0")]
pub type HsicCon0 = crate::Reg<hsic_con0::HsicCon0Spec>;
#[doc = "HSIC controller GRF register 0"]
pub mod hsic_con0;
#[doc = "HSIC_CON1 (rw) register accessor: HSIC controller GRF register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsic_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsic_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsic_con1`]
module"]
#[doc(alias = "HSIC_CON1")]
pub type HsicCon1 = crate::Reg<hsic_con1::HsicCon1Spec>;
#[doc = "HSIC controller GRF register1"]
pub mod hsic_con1;
#[doc = "GRF_USBHOST0_STATUS (rw) register accessor: usb host0 controller status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbhost0_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbhost0_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbhost0_status`]
module"]
#[doc(alias = "GRF_USBHOST0_STATUS")]
pub type GrfUsbhost0Status = crate::Reg<grf_usbhost0_status::GrfUsbhost0StatusSpec>;
#[doc = "usb host0 controller status register"]
pub mod grf_usbhost0_status;
#[doc = "GRF_USBHOST1_STATUS (rw) register accessor: usb host1 controller status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbhost1_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbhost1_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_usbhost1_status`]
module"]
#[doc(alias = "GRF_USBHOST1_STATUS")]
pub type GrfUsbhost1Status = crate::Reg<grf_usbhost1_status::GrfUsbhost1StatusSpec>;
#[doc = "usb host1 controller status register"]
pub mod grf_usbhost1_status;
#[doc = "GRF_HSIC_STATUS (rw) register accessor: hsic controller status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_hsic_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_hsic_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grf_hsic_status`]
module"]
#[doc(alias = "GRF_HSIC_STATUS")]
pub type GrfHsicStatus = crate::Reg<grf_hsic_status::GrfHsicStatusSpec>;
#[doc = "hsic controller status register"]
pub mod grf_hsic_status;
#[doc = "HSICPHY_CON0 (rw) register accessor: HSICPHY GRF control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsicphy_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsicphy_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsicphy_con0`]
module"]
#[doc(alias = "HSICPHY_CON0")]
pub type HsicphyCon0 = crate::Reg<hsicphy_con0::HsicphyCon0Spec>;
#[doc = "HSICPHY GRF control register"]
pub mod hsicphy_con0;
#[doc = "usbphy0_ctrl0 (rw) register accessor: usbphy0_ctrl0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl0`]
module"]
#[doc(alias = "usbphy0_ctrl0")]
pub type Usbphy0Ctrl0 = crate::Reg<usbphy0_ctrl0::Usbphy0Ctrl0Spec>;
#[doc = "usbphy0_ctrl0"]
pub mod usbphy0_ctrl0;
#[doc = "usbphy0_ctrl1 (rw) register accessor: usbphy0_ctrl1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl1`]
module"]
#[doc(alias = "usbphy0_ctrl1")]
pub type Usbphy0Ctrl1 = crate::Reg<usbphy0_ctrl1::Usbphy0Ctrl1Spec>;
#[doc = "usbphy0_ctrl1"]
pub mod usbphy0_ctrl1;
#[doc = "usbphy0_ctrl2 (rw) register accessor: usbphy0_ctrl2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl2`]
module"]
#[doc(alias = "usbphy0_ctrl2")]
pub type Usbphy0Ctrl2 = crate::Reg<usbphy0_ctrl2::Usbphy0Ctrl2Spec>;
#[doc = "usbphy0_ctrl2"]
pub mod usbphy0_ctrl2;
#[doc = "usbphy0_ctrl3 (rw) register accessor: usbphy0_ctrl3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl3`]
module"]
#[doc(alias = "usbphy0_ctrl3")]
pub type Usbphy0Ctrl3 = crate::Reg<usbphy0_ctrl3::Usbphy0Ctrl3Spec>;
#[doc = "usbphy0_ctrl3"]
pub mod usbphy0_ctrl3;
#[doc = "usbphy0_ctrl4 (rw) register accessor: usbphy0_ctrl4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl4`]
module"]
#[doc(alias = "usbphy0_ctrl4")]
pub type Usbphy0Ctrl4 = crate::Reg<usbphy0_ctrl4::Usbphy0Ctrl4Spec>;
#[doc = "usbphy0_ctrl4"]
pub mod usbphy0_ctrl4;
#[doc = "usbphy0_ctrl5 (rw) register accessor: usbphy0_ctrl5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl5`]
module"]
#[doc(alias = "usbphy0_ctrl5")]
pub type Usbphy0Ctrl5 = crate::Reg<usbphy0_ctrl5::Usbphy0Ctrl5Spec>;
#[doc = "usbphy0_ctrl5"]
pub mod usbphy0_ctrl5;
#[doc = "usbphy0_ctrl6 (rw) register accessor: usbphy0_ctrl6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl6`]
module"]
#[doc(alias = "usbphy0_ctrl6")]
pub type Usbphy0Ctrl6 = crate::Reg<usbphy0_ctrl6::Usbphy0Ctrl6Spec>;
#[doc = "usbphy0_ctrl6"]
pub mod usbphy0_ctrl6;
#[doc = "usbphy0_ctrl7 (rw) register accessor: usbphy0_ctrl7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl7`]
module"]
#[doc(alias = "usbphy0_ctrl7")]
pub type Usbphy0Ctrl7 = crate::Reg<usbphy0_ctrl7::Usbphy0Ctrl7Spec>;
#[doc = "usbphy0_ctrl7"]
pub mod usbphy0_ctrl7;
#[doc = "usbphy0_ctrl8 (rw) register accessor: usbphy0_ctrl8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl8`]
module"]
#[doc(alias = "usbphy0_ctrl8")]
pub type Usbphy0Ctrl8 = crate::Reg<usbphy0_ctrl8::Usbphy0Ctrl8Spec>;
#[doc = "usbphy0_ctrl8"]
pub mod usbphy0_ctrl8;
#[doc = "usbphy0_ctrl9 (rw) register accessor: usbphy0_ctrl9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl9`]
module"]
#[doc(alias = "usbphy0_ctrl9")]
pub type Usbphy0Ctrl9 = crate::Reg<usbphy0_ctrl9::Usbphy0Ctrl9Spec>;
#[doc = "usbphy0_ctrl9"]
pub mod usbphy0_ctrl9;
#[doc = "usbphy0_ctrl10 (rw) register accessor: usbphy0_ctrl10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl10`]
module"]
#[doc(alias = "usbphy0_ctrl10")]
pub type Usbphy0Ctrl10 = crate::Reg<usbphy0_ctrl10::Usbphy0Ctrl10Spec>;
#[doc = "usbphy0_ctrl10"]
pub mod usbphy0_ctrl10;
#[doc = "usbphy0_ctrl11 (rw) register accessor: usbphy0_ctrl11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl11`]
module"]
#[doc(alias = "usbphy0_ctrl11")]
pub type Usbphy0Ctrl11 = crate::Reg<usbphy0_ctrl11::Usbphy0Ctrl11Spec>;
#[doc = "usbphy0_ctrl11"]
pub mod usbphy0_ctrl11;
#[doc = "usbphy0_ctrl12 (rw) register accessor: usbphy0_ctrl12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl12`]
module"]
#[doc(alias = "usbphy0_ctrl12")]
pub type Usbphy0Ctrl12 = crate::Reg<usbphy0_ctrl12::Usbphy0Ctrl12Spec>;
#[doc = "usbphy0_ctrl12"]
pub mod usbphy0_ctrl12;
#[doc = "usbphy0_ctrl13 (rw) register accessor: usbphy0_ctrl13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl13`]
module"]
#[doc(alias = "usbphy0_ctrl13")]
pub type Usbphy0Ctrl13 = crate::Reg<usbphy0_ctrl13::Usbphy0Ctrl13Spec>;
#[doc = "usbphy0_ctrl13"]
pub mod usbphy0_ctrl13;
#[doc = "usbphy0_ctrl14 (rw) register accessor: usbphy0_ctrl14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl14`]
module"]
#[doc(alias = "usbphy0_ctrl14")]
pub type Usbphy0Ctrl14 = crate::Reg<usbphy0_ctrl14::Usbphy0Ctrl14Spec>;
#[doc = "usbphy0_ctrl14"]
pub mod usbphy0_ctrl14;
#[doc = "usbphy0_ctrl15 (rw) register accessor: usbphy0_ctrl15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl15`]
module"]
#[doc(alias = "usbphy0_ctrl15")]
pub type Usbphy0Ctrl15 = crate::Reg<usbphy0_ctrl15::Usbphy0Ctrl15Spec>;
#[doc = "usbphy0_ctrl15"]
pub mod usbphy0_ctrl15;
#[doc = "usbphy0_ctrl16 (rw) register accessor: usbphy0_ctrl16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl16`]
module"]
#[doc(alias = "usbphy0_ctrl16")]
pub type Usbphy0Ctrl16 = crate::Reg<usbphy0_ctrl16::Usbphy0Ctrl16Spec>;
#[doc = "usbphy0_ctrl16"]
pub mod usbphy0_ctrl16;
#[doc = "usbphy0_ctrl17 (rw) register accessor: usbphy0_ctrl17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl17`]
module"]
#[doc(alias = "usbphy0_ctrl17")]
pub type Usbphy0Ctrl17 = crate::Reg<usbphy0_ctrl17::Usbphy0Ctrl17Spec>;
#[doc = "usbphy0_ctrl17"]
pub mod usbphy0_ctrl17;
#[doc = "usbphy0_ctrl18 (rw) register accessor: usbphy0_ctrl18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl18`]
module"]
#[doc(alias = "usbphy0_ctrl18")]
pub type Usbphy0Ctrl18 = crate::Reg<usbphy0_ctrl18::Usbphy0Ctrl18Spec>;
#[doc = "usbphy0_ctrl18"]
pub mod usbphy0_ctrl18;
#[doc = "usbphy0_ctrl19 (rw) register accessor: usbphy0_ctrl19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl19`]
module"]
#[doc(alias = "usbphy0_ctrl19")]
pub type Usbphy0Ctrl19 = crate::Reg<usbphy0_ctrl19::Usbphy0Ctrl19Spec>;
#[doc = "usbphy0_ctrl19"]
pub mod usbphy0_ctrl19;
#[doc = "usbphy0_ctrl20 (rw) register accessor: usbphy0_ctrl20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl20`]
module"]
#[doc(alias = "usbphy0_ctrl20")]
pub type Usbphy0Ctrl20 = crate::Reg<usbphy0_ctrl20::Usbphy0Ctrl20Spec>;
#[doc = "usbphy0_ctrl20"]
pub mod usbphy0_ctrl20;
#[doc = "usbphy0_ctrl21 (rw) register accessor: usbphy0_ctrl21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl21`]
module"]
#[doc(alias = "usbphy0_ctrl21")]
pub type Usbphy0Ctrl21 = crate::Reg<usbphy0_ctrl21::Usbphy0Ctrl21Spec>;
#[doc = "usbphy0_ctrl21"]
pub mod usbphy0_ctrl21;
#[doc = "usbphy0_ctrl22 (rw) register accessor: usbphy0_ctrl22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl22`]
module"]
#[doc(alias = "usbphy0_ctrl22")]
pub type Usbphy0Ctrl22 = crate::Reg<usbphy0_ctrl22::Usbphy0Ctrl22Spec>;
#[doc = "usbphy0_ctrl22"]
pub mod usbphy0_ctrl22;
#[doc = "usbphy0_ctrl23 (rw) register accessor: usbphy0_ctrl23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl23`]
module"]
#[doc(alias = "usbphy0_ctrl23")]
pub type Usbphy0Ctrl23 = crate::Reg<usbphy0_ctrl23::Usbphy0Ctrl23Spec>;
#[doc = "usbphy0_ctrl23"]
pub mod usbphy0_ctrl23;
#[doc = "usbphy0_ctrl24 (rw) register accessor: usbphy0_ctrl24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl24`]
module"]
#[doc(alias = "usbphy0_ctrl24")]
pub type Usbphy0Ctrl24 = crate::Reg<usbphy0_ctrl24::Usbphy0Ctrl24Spec>;
#[doc = "usbphy0_ctrl24"]
pub mod usbphy0_ctrl24;
#[doc = "usbphy0_ctrl25 (rw) register accessor: usbphy0_ctrl25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy0_ctrl25`]
module"]
#[doc(alias = "usbphy0_ctrl25")]
pub type Usbphy0Ctrl25 = crate::Reg<usbphy0_ctrl25::Usbphy0Ctrl25Spec>;
#[doc = "usbphy0_ctrl25"]
pub mod usbphy0_ctrl25;
#[doc = "usbphy1_ctrl0 (rw) register accessor: usbphy1_ctrl0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl0`]
module"]
#[doc(alias = "usbphy1_ctrl0")]
pub type Usbphy1Ctrl0 = crate::Reg<usbphy1_ctrl0::Usbphy1Ctrl0Spec>;
#[doc = "usbphy1_ctrl0"]
pub mod usbphy1_ctrl0;
#[doc = "usbphy1_ctrl1 (rw) register accessor: usbphy1_ctrl1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl1`]
module"]
#[doc(alias = "usbphy1_ctrl1")]
pub type Usbphy1Ctrl1 = crate::Reg<usbphy1_ctrl1::Usbphy1Ctrl1Spec>;
#[doc = "usbphy1_ctrl1"]
pub mod usbphy1_ctrl1;
#[doc = "usbphy1_ctrl2 (rw) register accessor: usbphy1_ctrl2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl2`]
module"]
#[doc(alias = "usbphy1_ctrl2")]
pub type Usbphy1Ctrl2 = crate::Reg<usbphy1_ctrl2::Usbphy1Ctrl2Spec>;
#[doc = "usbphy1_ctrl2"]
pub mod usbphy1_ctrl2;
#[doc = "usbphy1_ctrl3 (rw) register accessor: usbphy1_ctrl3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl3`]
module"]
#[doc(alias = "usbphy1_ctrl3")]
pub type Usbphy1Ctrl3 = crate::Reg<usbphy1_ctrl3::Usbphy1Ctrl3Spec>;
#[doc = "usbphy1_ctrl3"]
pub mod usbphy1_ctrl3;
#[doc = "usbphy1_ctrl4 (rw) register accessor: usbphy1_ctrl4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl4`]
module"]
#[doc(alias = "usbphy1_ctrl4")]
pub type Usbphy1Ctrl4 = crate::Reg<usbphy1_ctrl4::Usbphy1Ctrl4Spec>;
#[doc = "usbphy1_ctrl4"]
pub mod usbphy1_ctrl4;
#[doc = "usbphy1_ctrl5 (rw) register accessor: usbphy1_ctrl5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl5`]
module"]
#[doc(alias = "usbphy1_ctrl5")]
pub type Usbphy1Ctrl5 = crate::Reg<usbphy1_ctrl5::Usbphy1Ctrl5Spec>;
#[doc = "usbphy1_ctrl5"]
pub mod usbphy1_ctrl5;
#[doc = "usbphy1_ctrl6 (rw) register accessor: usbphy1_ctrl6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl6`]
module"]
#[doc(alias = "usbphy1_ctrl6")]
pub type Usbphy1Ctrl6 = crate::Reg<usbphy1_ctrl6::Usbphy1Ctrl6Spec>;
#[doc = "usbphy1_ctrl6"]
pub mod usbphy1_ctrl6;
#[doc = "usbphy1_ctrl7 (rw) register accessor: usbphy1_ctrl7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl7`]
module"]
#[doc(alias = "usbphy1_ctrl7")]
pub type Usbphy1Ctrl7 = crate::Reg<usbphy1_ctrl7::Usbphy1Ctrl7Spec>;
#[doc = "usbphy1_ctrl7"]
pub mod usbphy1_ctrl7;
#[doc = "usbphy1_ctrl8 (rw) register accessor: usbphy1_ctrl8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl8`]
module"]
#[doc(alias = "usbphy1_ctrl8")]
pub type Usbphy1Ctrl8 = crate::Reg<usbphy1_ctrl8::Usbphy1Ctrl8Spec>;
#[doc = "usbphy1_ctrl8"]
pub mod usbphy1_ctrl8;
#[doc = "usbphy1_ctrl9 (rw) register accessor: usbphy1_ctrl9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl9`]
module"]
#[doc(alias = "usbphy1_ctrl9")]
pub type Usbphy1Ctrl9 = crate::Reg<usbphy1_ctrl9::Usbphy1Ctrl9Spec>;
#[doc = "usbphy1_ctrl9"]
pub mod usbphy1_ctrl9;
#[doc = "usbphy1_ctrl10 (rw) register accessor: usbphy1_ctrl10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl10`]
module"]
#[doc(alias = "usbphy1_ctrl10")]
pub type Usbphy1Ctrl10 = crate::Reg<usbphy1_ctrl10::Usbphy1Ctrl10Spec>;
#[doc = "usbphy1_ctrl10"]
pub mod usbphy1_ctrl10;
#[doc = "usbphy1_ctrl11 (rw) register accessor: usbphy1_ctrl11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl11`]
module"]
#[doc(alias = "usbphy1_ctrl11")]
pub type Usbphy1Ctrl11 = crate::Reg<usbphy1_ctrl11::Usbphy1Ctrl11Spec>;
#[doc = "usbphy1_ctrl11"]
pub mod usbphy1_ctrl11;
#[doc = "usbphy1_ctrl12 (rw) register accessor: usbphy1_ctrl12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl12`]
module"]
#[doc(alias = "usbphy1_ctrl12")]
pub type Usbphy1Ctrl12 = crate::Reg<usbphy1_ctrl12::Usbphy1Ctrl12Spec>;
#[doc = "usbphy1_ctrl12"]
pub mod usbphy1_ctrl12;
#[doc = "usbphy1_ctrl13 (rw) register accessor: usbphy1_ctrl13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl13`]
module"]
#[doc(alias = "usbphy1_ctrl13")]
pub type Usbphy1Ctrl13 = crate::Reg<usbphy1_ctrl13::Usbphy1Ctrl13Spec>;
#[doc = "usbphy1_ctrl13"]
pub mod usbphy1_ctrl13;
#[doc = "usbphy1_ctrl14 (rw) register accessor: usbphy1_ctrl14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl14`]
module"]
#[doc(alias = "usbphy1_ctrl14")]
pub type Usbphy1Ctrl14 = crate::Reg<usbphy1_ctrl14::Usbphy1Ctrl14Spec>;
#[doc = "usbphy1_ctrl14"]
pub mod usbphy1_ctrl14;
#[doc = "usbphy1_ctrl15 (rw) register accessor: usbphy1_ctrl15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl15`]
module"]
#[doc(alias = "usbphy1_ctrl15")]
pub type Usbphy1Ctrl15 = crate::Reg<usbphy1_ctrl15::Usbphy1Ctrl15Spec>;
#[doc = "usbphy1_ctrl15"]
pub mod usbphy1_ctrl15;
#[doc = "usbphy1_ctrl16 (rw) register accessor: usbphy1_ctrl16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl16`]
module"]
#[doc(alias = "usbphy1_ctrl16")]
pub type Usbphy1Ctrl16 = crate::Reg<usbphy1_ctrl16::Usbphy1Ctrl16Spec>;
#[doc = "usbphy1_ctrl16"]
pub mod usbphy1_ctrl16;
#[doc = "usbphy1_ctrl17 (rw) register accessor: usbphy1_ctrl17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl17`]
module"]
#[doc(alias = "usbphy1_ctrl17")]
pub type Usbphy1Ctrl17 = crate::Reg<usbphy1_ctrl17::Usbphy1Ctrl17Spec>;
#[doc = "usbphy1_ctrl17"]
pub mod usbphy1_ctrl17;
#[doc = "usbphy1_ctrl18 (rw) register accessor: usbphy1_ctrl18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl18`]
module"]
#[doc(alias = "usbphy1_ctrl18")]
pub type Usbphy1Ctrl18 = crate::Reg<usbphy1_ctrl18::Usbphy1Ctrl18Spec>;
#[doc = "usbphy1_ctrl18"]
pub mod usbphy1_ctrl18;
#[doc = "usbphy1_ctrl19 (rw) register accessor: usbphy1_ctrl19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl19`]
module"]
#[doc(alias = "usbphy1_ctrl19")]
pub type Usbphy1Ctrl19 = crate::Reg<usbphy1_ctrl19::Usbphy1Ctrl19Spec>;
#[doc = "usbphy1_ctrl19"]
pub mod usbphy1_ctrl19;
#[doc = "usbphy1_ctrl20 (rw) register accessor: usbphy1_ctrl20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl20`]
module"]
#[doc(alias = "usbphy1_ctrl20")]
pub type Usbphy1Ctrl20 = crate::Reg<usbphy1_ctrl20::Usbphy1Ctrl20Spec>;
#[doc = "usbphy1_ctrl20"]
pub mod usbphy1_ctrl20;
#[doc = "usbphy1_ctrl21 (rw) register accessor: usbphy1_ctrl21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl21`]
module"]
#[doc(alias = "usbphy1_ctrl21")]
pub type Usbphy1Ctrl21 = crate::Reg<usbphy1_ctrl21::Usbphy1Ctrl21Spec>;
#[doc = "usbphy1_ctrl21"]
pub mod usbphy1_ctrl21;
#[doc = "usbphy1_ctrl22 (rw) register accessor: usbphy1_ctrl22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl22`]
module"]
#[doc(alias = "usbphy1_ctrl22")]
pub type Usbphy1Ctrl22 = crate::Reg<usbphy1_ctrl22::Usbphy1Ctrl22Spec>;
#[doc = "usbphy1_ctrl22"]
pub mod usbphy1_ctrl22;
#[doc = "usbphy1_ctrl23 (rw) register accessor: usbphy1_ctrl23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl23`]
module"]
#[doc(alias = "usbphy1_ctrl23")]
pub type Usbphy1Ctrl23 = crate::Reg<usbphy1_ctrl23::Usbphy1Ctrl23Spec>;
#[doc = "usbphy1_ctrl23"]
pub mod usbphy1_ctrl23;
#[doc = "usbphy1_ctrl24 (rw) register accessor: usbphy1_ctrl24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl24`]
module"]
#[doc(alias = "usbphy1_ctrl24")]
pub type Usbphy1Ctrl24 = crate::Reg<usbphy1_ctrl24::Usbphy1Ctrl24Spec>;
#[doc = "usbphy1_ctrl24"]
pub mod usbphy1_ctrl24;
#[doc = "usbphy1_ctrl25 (rw) register accessor: usbphy1_ctrl25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy1_ctrl25`]
module"]
#[doc(alias = "usbphy1_ctrl25")]
pub type Usbphy1Ctrl25 = crate::Reg<usbphy1_ctrl25::Usbphy1Ctrl25Spec>;
#[doc = "usbphy1_ctrl25"]
pub mod usbphy1_ctrl25;
#[doc = "HDCP22_PERF_CON0 (rw) register accessor: hdcp performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22_perf_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22_perf_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp22_perf_con0`]
module"]
#[doc(alias = "HDCP22_PERF_CON0")]
pub type Hdcp22PerfCon0 = crate::Reg<hdcp22_perf_con0::Hdcp22PerfCon0Spec>;
#[doc = "hdcp performance monitor control register"]
pub mod hdcp22_perf_con0;
#[doc = "HDCP22_PERF_CON1 (rw) register accessor: hdcp performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22_perf_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22_perf_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp22_perf_con1`]
module"]
#[doc(alias = "HDCP22_PERF_CON1")]
pub type Hdcp22PerfCon1 = crate::Reg<hdcp22_perf_con1::Hdcp22PerfCon1Spec>;
#[doc = "hdcp performance monitor control register"]
pub mod hdcp22_perf_con1;
#[doc = "HDCP22_PERF_CON2 (rw) register accessor: hdcp performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22_perf_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22_perf_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp22_perf_con2`]
module"]
#[doc(alias = "HDCP22_PERF_CON2")]
pub type Hdcp22PerfCon2 = crate::Reg<hdcp22_perf_con2::Hdcp22PerfCon2Spec>;
#[doc = "hdcp performance monitor control register"]
pub mod hdcp22_perf_con2;
#[doc = "HDCP22_PERF_RD_MAX_LATENCY_NUM (rw) register accessor: hdcp performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22_perf_rd_max_latency_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22_perf_rd_max_latency_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp22_perf_rd_max_latency_num`]
module"]
#[doc(alias = "HDCP22_PERF_RD_MAX_LATENCY_NUM")]
pub type Hdcp22PerfRdMaxLatencyNum =
    crate::Reg<hdcp22_perf_rd_max_latency_num::Hdcp22PerfRdMaxLatencyNumSpec>;
#[doc = "hdcp performance monitor status register"]
pub mod hdcp22_perf_rd_max_latency_num;
#[doc = "HDCP22_PERF_RD_LATENCY_SAMP_NUM (rw) register accessor: hdcp performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22_perf_rd_latency_samp_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22_perf_rd_latency_samp_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp22_perf_rd_latency_samp_num`]
module"]
#[doc(alias = "HDCP22_PERF_RD_LATENCY_SAMP_NUM")]
pub type Hdcp22PerfRdLatencySampNum =
    crate::Reg<hdcp22_perf_rd_latency_samp_num::Hdcp22PerfRdLatencySampNumSpec>;
#[doc = "hdcp performance monitor status register"]
pub mod hdcp22_perf_rd_latency_samp_num;
#[doc = "HDCP22_PERF_RD_LATENCY_ACC_NUM (rw) register accessor: hdcp performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22_perf_rd_latency_acc_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22_perf_rd_latency_acc_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp22_perf_rd_latency_acc_num`]
module"]
#[doc(alias = "HDCP22_PERF_RD_LATENCY_ACC_NUM")]
pub type Hdcp22PerfRdLatencyAccNum =
    crate::Reg<hdcp22_perf_rd_latency_acc_num::Hdcp22PerfRdLatencyAccNumSpec>;
#[doc = "hdcp performance monitor status register"]
pub mod hdcp22_perf_rd_latency_acc_num;
#[doc = "HDCP22_PERF_RD_AXI_TOTAL_BYTE (rw) register accessor: hdcp performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22_perf_rd_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22_perf_rd_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp22_perf_rd_axi_total_byte`]
module"]
#[doc(alias = "HDCP22_PERF_RD_AXI_TOTAL_BYTE")]
pub type Hdcp22PerfRdAxiTotalByte =
    crate::Reg<hdcp22_perf_rd_axi_total_byte::Hdcp22PerfRdAxiTotalByteSpec>;
#[doc = "hdcp performance monitor status register"]
pub mod hdcp22_perf_rd_axi_total_byte;
#[doc = "HDCP22_PERF_WR_AXI_TOTAL_BYTE (rw) register accessor: hdcp performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22_perf_wr_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22_perf_wr_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp22_perf_wr_axi_total_byte`]
module"]
#[doc(alias = "HDCP22_PERF_WR_AXI_TOTAL_BYTE")]
pub type Hdcp22PerfWrAxiTotalByte =
    crate::Reg<hdcp22_perf_wr_axi_total_byte::Hdcp22PerfWrAxiTotalByteSpec>;
#[doc = "hdcp performance monitor status register"]
pub mod hdcp22_perf_wr_axi_total_byte;
#[doc = "HDCP22_PERF_WORKING_CNT (rw) register accessor: hdcp performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22_perf_working_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp22_perf_working_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcp22_perf_working_cnt`]
module"]
#[doc(alias = "HDCP22_PERF_WORKING_CNT")]
pub type Hdcp22PerfWorkingCnt = crate::Reg<hdcp22_perf_working_cnt::Hdcp22PerfWorkingCntSpec>;
#[doc = "hdcp performance monitor status register"]
pub mod hdcp22_perf_working_cnt;
#[doc = "SOC_CON9 (rw) register accessor: SoC control register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con9`]
module"]
#[doc(alias = "SOC_CON9")]
pub type SocCon9 = crate::Reg<soc_con9::SocCon9Spec>;
#[doc = "SoC control register 9"]
pub mod soc_con9;
#[doc = "SOC_CON20 (rw) register accessor: SoC control register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con20`]
module"]
#[doc(alias = "SOC_CON20")]
pub type SocCon20 = crate::Reg<soc_con20::SocCon20Spec>;
#[doc = "SoC control register 20"]
pub mod soc_con20;
#[doc = "SOC_CON21 (rw) register accessor: SoC control register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con21`]
module"]
#[doc(alias = "SOC_CON21")]
pub type SocCon21 = crate::Reg<soc_con21::SocCon21Spec>;
#[doc = "SoC control register 21"]
pub mod soc_con21;
#[doc = "SOC_CON22 (rw) register accessor: SoC control register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con22`]
module"]
#[doc(alias = "SOC_CON22")]
pub type SocCon22 = crate::Reg<soc_con22::SocCon22Spec>;
#[doc = "SoC control register 22"]
pub mod soc_con22;
#[doc = "SOC_CON23 (rw) register accessor: SoC control register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con23`]
module"]
#[doc(alias = "SOC_CON23")]
pub type SocCon23 = crate::Reg<soc_con23::SocCon23Spec>;
#[doc = "SoC control register 23"]
pub mod soc_con23;
#[doc = "SOC_CON24 (rw) register accessor: SoC control register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con24`]
module"]
#[doc(alias = "SOC_CON24")]
pub type SocCon24 = crate::Reg<soc_con24::SocCon24Spec>;
#[doc = "SoC control register 24"]
pub mod soc_con24;
#[doc = "SOC_CON25 (rw) register accessor: SoC control register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con25`]
module"]
#[doc(alias = "SOC_CON25")]
pub type SocCon25 = crate::Reg<soc_con25::SocCon25Spec>;
#[doc = "SoC control register 25"]
pub mod soc_con25;
#[doc = "SOC_CON26 (rw) register accessor: SoC control register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con26`]
module"]
#[doc(alias = "SOC_CON26")]
pub type SocCon26 = crate::Reg<soc_con26::SocCon26Spec>;
#[doc = "SoC control register 26"]
pub mod soc_con26;
#[doc = "GPU_PERF_CON0 (rw) register accessor: gpu performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpu_perf_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpu_perf_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpu_perf_con0`]
module"]
#[doc(alias = "GPU_PERF_CON0")]
pub type GpuPerfCon0 = crate::Reg<gpu_perf_con0::GpuPerfCon0Spec>;
#[doc = "gpu performance monitor control register"]
pub mod gpu_perf_con0;
#[doc = "GPU_PERF_CON1 (rw) register accessor: gpu performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpu_perf_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpu_perf_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpu_perf_con1`]
module"]
#[doc(alias = "GPU_PERF_CON1")]
pub type GpuPerfCon1 = crate::Reg<gpu_perf_con1::GpuPerfCon1Spec>;
#[doc = "gpu performance monitor control register"]
pub mod gpu_perf_con1;
#[doc = "GPU_PERF_CON2 (rw) register accessor: gpu performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpu_perf_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpu_perf_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpu_perf_con2`]
module"]
#[doc(alias = "GPU_PERF_CON2")]
pub type GpuPerfCon2 = crate::Reg<gpu_perf_con2::GpuPerfCon2Spec>;
#[doc = "gpu performance monitor control register"]
pub mod gpu_perf_con2;
#[doc = "GPU_PERF_RD_MAX_LATENCY_NUM (rw) register accessor: gpu performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpu_perf_rd_max_latency_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpu_perf_rd_max_latency_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpu_perf_rd_max_latency_num`]
module"]
#[doc(alias = "GPU_PERF_RD_MAX_LATENCY_NUM")]
pub type GpuPerfRdMaxLatencyNum =
    crate::Reg<gpu_perf_rd_max_latency_num::GpuPerfRdMaxLatencyNumSpec>;
#[doc = "gpu performance monitor status register"]
pub mod gpu_perf_rd_max_latency_num;
#[doc = "GPU_PERF_RD_LATENCY_SAMP_NUM (rw) register accessor: gpu performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpu_perf_rd_latency_samp_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpu_perf_rd_latency_samp_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpu_perf_rd_latency_samp_num`]
module"]
#[doc(alias = "GPU_PERF_RD_LATENCY_SAMP_NUM")]
pub type GpuPerfRdLatencySampNum =
    crate::Reg<gpu_perf_rd_latency_samp_num::GpuPerfRdLatencySampNumSpec>;
#[doc = "gpu performance monitor status register"]
pub mod gpu_perf_rd_latency_samp_num;
#[doc = "GPU_PERF_RD_LATENCY_ACC_NUM (rw) register accessor: gpu performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpu_perf_rd_latency_acc_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpu_perf_rd_latency_acc_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpu_perf_rd_latency_acc_num`]
module"]
#[doc(alias = "GPU_PERF_RD_LATENCY_ACC_NUM")]
pub type GpuPerfRdLatencyAccNum =
    crate::Reg<gpu_perf_rd_latency_acc_num::GpuPerfRdLatencyAccNumSpec>;
#[doc = "gpu performance monitor status register"]
pub mod gpu_perf_rd_latency_acc_num;
#[doc = "GPU_PERF_RD_AXI_TOTAL_BYTE (rw) register accessor: gpu performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpu_perf_rd_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpu_perf_rd_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpu_perf_rd_axi_total_byte`]
module"]
#[doc(alias = "GPU_PERF_RD_AXI_TOTAL_BYTE")]
pub type GpuPerfRdAxiTotalByte = crate::Reg<gpu_perf_rd_axi_total_byte::GpuPerfRdAxiTotalByteSpec>;
#[doc = "gpu performance monitor status register"]
pub mod gpu_perf_rd_axi_total_byte;
#[doc = "GPU_PERF_WR_AXI_TOTAL_BYTE (rw) register accessor: gpu performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpu_perf_wr_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpu_perf_wr_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpu_perf_wr_axi_total_byte`]
module"]
#[doc(alias = "GPU_PERF_WR_AXI_TOTAL_BYTE")]
pub type GpuPerfWrAxiTotalByte = crate::Reg<gpu_perf_wr_axi_total_byte::GpuPerfWrAxiTotalByteSpec>;
#[doc = "gpu performance monitor status register"]
pub mod gpu_perf_wr_axi_total_byte;
#[doc = "GPU_PERF_WORKING_CNT (rw) register accessor: gpu performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpu_perf_working_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpu_perf_working_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpu_perf_working_cnt`]
module"]
#[doc(alias = "GPU_PERF_WORKING_CNT")]
pub type GpuPerfWorkingCnt = crate::Reg<gpu_perf_working_cnt::GpuPerfWorkingCntSpec>;
#[doc = "gpu performance monitor status register"]
pub mod gpu_perf_working_cnt;
#[doc = "CPU_CON0 (rw) register accessor: cpu control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_con0`]
module"]
#[doc(alias = "CPU_CON0")]
pub type CpuCon0 = crate::Reg<cpu_con0::CpuCon0Spec>;
#[doc = "cpu control register 0"]
pub mod cpu_con0;
#[doc = "CPU_CON1 (rw) register accessor: cpu control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_con1`]
module"]
#[doc(alias = "CPU_CON1")]
pub type CpuCon1 = crate::Reg<cpu_con1::CpuCon1Spec>;
#[doc = "cpu control register 1"]
pub mod cpu_con1;
#[doc = "CPU_CON2 (rw) register accessor: cpu control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_con2`]
module"]
#[doc(alias = "CPU_CON2")]
pub type CpuCon2 = crate::Reg<cpu_con2::CpuCon2Spec>;
#[doc = "cpu control register 2"]
pub mod cpu_con2;
#[doc = "CPU_CON3 (rw) register accessor: cpu control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_con3`]
module"]
#[doc(alias = "CPU_CON3")]
pub type CpuCon3 = crate::Reg<cpu_con3::CpuCon3Spec>;
#[doc = "cpu control register 3"]
pub mod cpu_con3;
#[doc = "CPU_STATUS0 (rw) register accessor: cpu status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_status0`]
module"]
#[doc(alias = "CPU_STATUS0")]
pub type CpuStatus0 = crate::Reg<cpu_status0::CpuStatus0Spec>;
#[doc = "cpu status register 0"]
pub mod cpu_status0;
#[doc = "CPU_STATUS1 (rw) register accessor: cpu status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_status1`]
module"]
#[doc(alias = "CPU_STATUS1")]
pub type CpuStatus1 = crate::Reg<cpu_status1::CpuStatus1Spec>;
#[doc = "cpu status register 1"]
pub mod cpu_status1;
#[doc = "CPU_STATUS2 (rw) register accessor: cpu status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_status2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_status2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_status2`]
module"]
#[doc(alias = "CPU_STATUS2")]
pub type CpuStatus2 = crate::Reg<cpu_status2::CpuStatus2Spec>;
#[doc = "cpu status register 2"]
pub mod cpu_status2;
#[doc = "CPU_STATUS3 (rw) register accessor: cpu status register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_status3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_status3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_status3`]
module"]
#[doc(alias = "CPU_STATUS3")]
pub type CpuStatus3 = crate::Reg<cpu_status3::CpuStatus3Spec>;
#[doc = "cpu status register 3"]
pub mod cpu_status3;
#[doc = "CPU_STATUS4 (rw) register accessor: cpu status register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_status4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_status4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_status4`]
module"]
#[doc(alias = "CPU_STATUS4")]
pub type CpuStatus4 = crate::Reg<cpu_status4::CpuStatus4Spec>;
#[doc = "cpu status register 4"]
pub mod cpu_status4;
#[doc = "CPU_STATUS5 (rw) register accessor: cpu status register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_status5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_status5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_status5`]
module"]
#[doc(alias = "CPU_STATUS5")]
pub type CpuStatus5 = crate::Reg<cpu_status5::CpuStatus5Spec>;
#[doc = "cpu status register 5"]
pub mod cpu_status5;
#[doc = "A53_PERF_CON0 (rw) register accessor: a53 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a53_perf_con0`]
module"]
#[doc(alias = "A53_PERF_CON0")]
pub type A53PerfCon0 = crate::Reg<a53_perf_con0::A53PerfCon0Spec>;
#[doc = "a53 performance monitor control register"]
pub mod a53_perf_con0;
#[doc = "A53_PERF_CON1 (rw) register accessor: a53 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a53_perf_con1`]
module"]
#[doc(alias = "A53_PERF_CON1")]
pub type A53PerfCon1 = crate::Reg<a53_perf_con1::A53PerfCon1Spec>;
#[doc = "a53 performance monitor control register"]
pub mod a53_perf_con1;
#[doc = "A53_PERF_CON2 (rw) register accessor: a53 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a53_perf_con2`]
module"]
#[doc(alias = "A53_PERF_CON2")]
pub type A53PerfCon2 = crate::Reg<a53_perf_con2::A53PerfCon2Spec>;
#[doc = "a53 performance monitor control register"]
pub mod a53_perf_con2;
#[doc = "A53_PERF_CON3 (rw) register accessor: a53 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a53_perf_con3`]
module"]
#[doc(alias = "A53_PERF_CON3")]
pub type A53PerfCon3 = crate::Reg<a53_perf_con3::A53PerfCon3Spec>;
#[doc = "a53 performance monitor control register"]
pub mod a53_perf_con3;
#[doc = "A53_PERF_RD_MON_ST (rw) register accessor: performance monitor read start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_rd_mon_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_rd_mon_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a53_perf_rd_mon_st`]
module"]
#[doc(alias = "A53_PERF_RD_MON_ST")]
pub type A53PerfRdMonSt = crate::Reg<a53_perf_rd_mon_st::A53PerfRdMonStSpec>;
#[doc = "performance monitor read start address"]
pub mod a53_perf_rd_mon_st;
#[doc = "A53_PERF_RD_MON_END (rw) register accessor: performance monitor end address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_rd_mon_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_rd_mon_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a53_perf_rd_mon_end`]
module"]
#[doc(alias = "A53_PERF_RD_MON_END")]
pub type A53PerfRdMonEnd = crate::Reg<a53_perf_rd_mon_end::A53PerfRdMonEndSpec>;
#[doc = "performance monitor end address"]
pub mod a53_perf_rd_mon_end;
#[doc = "A53_PERF_WR_MON_ST (rw) register accessor: performance write monitor start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_wr_mon_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_wr_mon_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a53_perf_wr_mon_st`]
module"]
#[doc(alias = "A53_PERF_WR_MON_ST")]
pub type A53PerfWrMonSt = crate::Reg<a53_perf_wr_mon_st::A53PerfWrMonStSpec>;
#[doc = "performance write monitor start address"]
pub mod a53_perf_wr_mon_st;
#[doc = "A53_PERF_WR_MON_END (rw) register accessor: performance monitor write end address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_wr_mon_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_wr_mon_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a53_perf_wr_mon_end`]
module"]
#[doc(alias = "A53_PERF_WR_MON_END")]
pub type A53PerfWrMonEnd = crate::Reg<a53_perf_wr_mon_end::A53PerfWrMonEndSpec>;
#[doc = "performance monitor write end address"]
pub mod a53_perf_wr_mon_end;
#[doc = "A53_PERF_RD_MAX_LATENCY_NUM (rw) register accessor: a53 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_rd_max_latency_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_rd_max_latency_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a53_perf_rd_max_latency_num`]
module"]
#[doc(alias = "A53_PERF_RD_MAX_LATENCY_NUM")]
pub type A53PerfRdMaxLatencyNum =
    crate::Reg<a53_perf_rd_max_latency_num::A53PerfRdMaxLatencyNumSpec>;
#[doc = "a53 performance monitor status register"]
pub mod a53_perf_rd_max_latency_num;
#[doc = "A53_PERF_RD_LATENCY_SAMP_NUM (rw) register accessor: a53 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_rd_latency_samp_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_rd_latency_samp_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a53_perf_rd_latency_samp_num`]
module"]
#[doc(alias = "A53_PERF_RD_LATENCY_SAMP_NUM")]
pub type A53PerfRdLatencySampNum =
    crate::Reg<a53_perf_rd_latency_samp_num::A53PerfRdLatencySampNumSpec>;
#[doc = "a53 performance monitor status register"]
pub mod a53_perf_rd_latency_samp_num;
#[doc = "A53_PERF_RD_LATENCY_ACC_NUM (rw) register accessor: a53 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_rd_latency_acc_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_rd_latency_acc_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a53_perf_rd_latency_acc_num`]
module"]
#[doc(alias = "A53_PERF_RD_LATENCY_ACC_NUM")]
pub type A53PerfRdLatencyAccNum =
    crate::Reg<a53_perf_rd_latency_acc_num::A53PerfRdLatencyAccNumSpec>;
#[doc = "a53 performance monitor status register"]
pub mod a53_perf_rd_latency_acc_num;
#[doc = "A53_PERF_RD_AXI_TOTAL_BYTE (rw) register accessor: a53 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_rd_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_rd_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a53_perf_rd_axi_total_byte`]
module"]
#[doc(alias = "A53_PERF_RD_AXI_TOTAL_BYTE")]
pub type A53PerfRdAxiTotalByte = crate::Reg<a53_perf_rd_axi_total_byte::A53PerfRdAxiTotalByteSpec>;
#[doc = "a53 performance monitor status register"]
pub mod a53_perf_rd_axi_total_byte;
#[doc = "A53_PERF_WR_AXI_TOTAL_BYTE (rw) register accessor: a53 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_wr_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_wr_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a53_perf_wr_axi_total_byte`]
module"]
#[doc(alias = "A53_PERF_WR_AXI_TOTAL_BYTE")]
pub type A53PerfWrAxiTotalByte = crate::Reg<a53_perf_wr_axi_total_byte::A53PerfWrAxiTotalByteSpec>;
#[doc = "a53 performance monitor status register"]
pub mod a53_perf_wr_axi_total_byte;
#[doc = "A53_PERF_WORKING_CNT (rw) register accessor: a53 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_working_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_working_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a53_perf_working_cnt`]
module"]
#[doc(alias = "A53_PERF_WORKING_CNT")]
pub type A53PerfWorkingCnt = crate::Reg<a53_perf_working_cnt::A53PerfWorkingCntSpec>;
#[doc = "a53 performance monitor status register"]
pub mod a53_perf_working_cnt;
#[doc = "A53_PERF_INT_STATUS (rw) register accessor: a53 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_int_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_int_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a53_perf_int_status`]
module"]
#[doc(alias = "A53_PERF_INT_STATUS")]
pub type A53PerfIntStatus = crate::Reg<a53_perf_int_status::A53PerfIntStatusSpec>;
#[doc = "a53 performance monitor status register"]
pub mod a53_perf_int_status;
#[doc = "A72_PERF_CON0 (rw) register accessor: a72 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a72_perf_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a72_perf_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a72_perf_con0`]
module"]
#[doc(alias = "A72_PERF_CON0")]
pub type A72PerfCon0 = crate::Reg<a72_perf_con0::A72PerfCon0Spec>;
#[doc = "a72 performance monitor control register"]
pub mod a72_perf_con0;
#[doc = "A72_PERF_CON1 (rw) register accessor: a72 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a72_perf_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a72_perf_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a72_perf_con1`]
module"]
#[doc(alias = "A72_PERF_CON1")]
pub type A72PerfCon1 = crate::Reg<a72_perf_con1::A72PerfCon1Spec>;
#[doc = "a72 performance monitor control register"]
pub mod a72_perf_con1;
#[doc = "A72_PERF_CON2 (rw) register accessor: a72 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a72_perf_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a72_perf_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a72_perf_con2`]
module"]
#[doc(alias = "A72_PERF_CON2")]
pub type A72PerfCon2 = crate::Reg<a72_perf_con2::A72PerfCon2Spec>;
#[doc = "a72 performance monitor control register"]
pub mod a72_perf_con2;
#[doc = "A72_PERF_CON3 (rw) register accessor: a72 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a72_perf_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a72_perf_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a72_perf_con3`]
module"]
#[doc(alias = "A72_PERF_CON3")]
pub type A72PerfCon3 = crate::Reg<a72_perf_con3::A72PerfCon3Spec>;
#[doc = "a72 performance monitor control register"]
pub mod a72_perf_con3;
#[doc = "A72_PERF_RD_MON_ST (rw) register accessor: performance monitor read start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a72_perf_rd_mon_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a72_perf_rd_mon_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a72_perf_rd_mon_st`]
module"]
#[doc(alias = "A72_PERF_RD_MON_ST")]
pub type A72PerfRdMonSt = crate::Reg<a72_perf_rd_mon_st::A72PerfRdMonStSpec>;
#[doc = "performance monitor read start address"]
pub mod a72_perf_rd_mon_st;
#[doc = "A72_PERF_RD_MON_END (rw) register accessor: performance monitor end address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a72_perf_rd_mon_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a72_perf_rd_mon_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a72_perf_rd_mon_end`]
module"]
#[doc(alias = "A72_PERF_RD_MON_END")]
pub type A72PerfRdMonEnd = crate::Reg<a72_perf_rd_mon_end::A72PerfRdMonEndSpec>;
#[doc = "performance monitor end address"]
pub mod a72_perf_rd_mon_end;
#[doc = "A72_PERF_WR_MON_ST (rw) register accessor: performance write monitor start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a72_perf_wr_mon_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a72_perf_wr_mon_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a72_perf_wr_mon_st`]
module"]
#[doc(alias = "A72_PERF_WR_MON_ST")]
pub type A72PerfWrMonSt = crate::Reg<a72_perf_wr_mon_st::A72PerfWrMonStSpec>;
#[doc = "performance write monitor start address"]
pub mod a72_perf_wr_mon_st;
#[doc = "A72_PERF_WR_MON_END (rw) register accessor: performance monitor write end address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a72_perf_wr_mon_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a72_perf_wr_mon_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a72_perf_wr_mon_end`]
module"]
#[doc(alias = "A72_PERF_WR_MON_END")]
pub type A72PerfWrMonEnd = crate::Reg<a72_perf_wr_mon_end::A72PerfWrMonEndSpec>;
#[doc = "performance monitor write end address"]
pub mod a72_perf_wr_mon_end;
#[doc = "A72_PERF_RD_MAX_LATENCY_NUM (rw) register accessor: a72 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a72_perf_rd_max_latency_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a72_perf_rd_max_latency_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a72_perf_rd_max_latency_num`]
module"]
#[doc(alias = "A72_PERF_RD_MAX_LATENCY_NUM")]
pub type A72PerfRdMaxLatencyNum =
    crate::Reg<a72_perf_rd_max_latency_num::A72PerfRdMaxLatencyNumSpec>;
#[doc = "a72 performance monitor status register"]
pub mod a72_perf_rd_max_latency_num;
#[doc = "A72_PERF_RD_LATENCY_SAMP_NUM (rw) register accessor: a72 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a72_perf_rd_latency_samp_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a72_perf_rd_latency_samp_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a72_perf_rd_latency_samp_num`]
module"]
#[doc(alias = "A72_PERF_RD_LATENCY_SAMP_NUM")]
pub type A72PerfRdLatencySampNum =
    crate::Reg<a72_perf_rd_latency_samp_num::A72PerfRdLatencySampNumSpec>;
#[doc = "a72 performance monitor status register"]
pub mod a72_perf_rd_latency_samp_num;
#[doc = "A72_PERF_RD_LATENCY_ACC_NUM (rw) register accessor: a72 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a72_perf_rd_latency_acc_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a72_perf_rd_latency_acc_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a72_perf_rd_latency_acc_num`]
module"]
#[doc(alias = "A72_PERF_RD_LATENCY_ACC_NUM")]
pub type A72PerfRdLatencyAccNum =
    crate::Reg<a72_perf_rd_latency_acc_num::A72PerfRdLatencyAccNumSpec>;
#[doc = "a72 performance monitor status register"]
pub mod a72_perf_rd_latency_acc_num;
#[doc = "A72_PERF_RD_AXI_TOTAL_BYTE (rw) register accessor: a72 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a72_perf_rd_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a72_perf_rd_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a72_perf_rd_axi_total_byte`]
module"]
#[doc(alias = "A72_PERF_RD_AXI_TOTAL_BYTE")]
pub type A72PerfRdAxiTotalByte = crate::Reg<a72_perf_rd_axi_total_byte::A72PerfRdAxiTotalByteSpec>;
#[doc = "a72 performance monitor status register"]
pub mod a72_perf_rd_axi_total_byte;
#[doc = "A72_PERF_WR_AXI_TOTAL_BYTE (rw) register accessor: a72 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a72_perf_wr_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a72_perf_wr_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a72_perf_wr_axi_total_byte`]
module"]
#[doc(alias = "A72_PERF_WR_AXI_TOTAL_BYTE")]
pub type A72PerfWrAxiTotalByte = crate::Reg<a72_perf_wr_axi_total_byte::A72PerfWrAxiTotalByteSpec>;
#[doc = "a72 performance monitor status register"]
pub mod a72_perf_wr_axi_total_byte;
#[doc = "A72_PERF_WORKING_CNT (rw) register accessor: a72 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a72_perf_working_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a72_perf_working_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a72_perf_working_cnt`]
module"]
#[doc(alias = "A72_PERF_WORKING_CNT")]
pub type A72PerfWorkingCnt = crate::Reg<a72_perf_working_cnt::A72PerfWorkingCntSpec>;
#[doc = "a72 performance monitor status register"]
pub mod a72_perf_working_cnt;
#[doc = "A72_PERF_INT_STATUS (rw) register accessor: a72 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a72_perf_int_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a72_perf_int_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@a72_perf_int_status`]
module"]
#[doc(alias = "A72_PERF_INT_STATUS")]
pub type A72PerfIntStatus = crate::Reg<a72_perf_int_status::A72PerfIntStatusSpec>;
#[doc = "a72 performance monitor status register"]
pub mod a72_perf_int_status;
#[doc = "GMAC_PERF_CON0 (rw) register accessor: gmac performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_perf_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_perf_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_perf_con0`]
module"]
#[doc(alias = "GMAC_PERF_CON0")]
pub type GmacPerfCon0 = crate::Reg<gmac_perf_con0::GmacPerfCon0Spec>;
#[doc = "gmac performance monitor control register"]
pub mod gmac_perf_con0;
#[doc = "GMAC_PERF_CON1 (rw) register accessor: gmac performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_perf_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_perf_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_perf_con1`]
module"]
#[doc(alias = "GMAC_PERF_CON1")]
pub type GmacPerfCon1 = crate::Reg<gmac_perf_con1::GmacPerfCon1Spec>;
#[doc = "gmac performance monitor control register"]
pub mod gmac_perf_con1;
#[doc = "GMAC_PERF_CON2 (rw) register accessor: gmac performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_perf_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_perf_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_perf_con2`]
module"]
#[doc(alias = "GMAC_PERF_CON2")]
pub type GmacPerfCon2 = crate::Reg<gmac_perf_con2::GmacPerfCon2Spec>;
#[doc = "gmac performance monitor control register"]
pub mod gmac_perf_con2;
#[doc = "GMAC_PERF_RD_MAX_LATENCY_NUM (rw) register accessor: gmac performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_perf_rd_max_latency_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_perf_rd_max_latency_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_perf_rd_max_latency_num`]
module"]
#[doc(alias = "GMAC_PERF_RD_MAX_LATENCY_NUM")]
pub type GmacPerfRdMaxLatencyNum =
    crate::Reg<gmac_perf_rd_max_latency_num::GmacPerfRdMaxLatencyNumSpec>;
#[doc = "gmac performance monitor status register"]
pub mod gmac_perf_rd_max_latency_num;
#[doc = "GMAC_PERF_RD_LATENCY_SAMP_NUM (rw) register accessor: gmac performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_perf_rd_latency_samp_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_perf_rd_latency_samp_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_perf_rd_latency_samp_num`]
module"]
#[doc(alias = "GMAC_PERF_RD_LATENCY_SAMP_NUM")]
pub type GmacPerfRdLatencySampNum =
    crate::Reg<gmac_perf_rd_latency_samp_num::GmacPerfRdLatencySampNumSpec>;
#[doc = "gmac performance monitor status register"]
pub mod gmac_perf_rd_latency_samp_num;
#[doc = "GMAC_PERF_RD_LATENCY_ACC_NUM (rw) register accessor: gmac performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_perf_rd_latency_acc_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_perf_rd_latency_acc_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_perf_rd_latency_acc_num`]
module"]
#[doc(alias = "GMAC_PERF_RD_LATENCY_ACC_NUM")]
pub type GmacPerfRdLatencyAccNum =
    crate::Reg<gmac_perf_rd_latency_acc_num::GmacPerfRdLatencyAccNumSpec>;
#[doc = "gmac performance monitor status register"]
pub mod gmac_perf_rd_latency_acc_num;
#[doc = "GMAC_PERF_RD_AXI_TOTAL_BYTE (rw) register accessor: gmac performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_perf_rd_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_perf_rd_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_perf_rd_axi_total_byte`]
module"]
#[doc(alias = "GMAC_PERF_RD_AXI_TOTAL_BYTE")]
pub type GmacPerfRdAxiTotalByte =
    crate::Reg<gmac_perf_rd_axi_total_byte::GmacPerfRdAxiTotalByteSpec>;
#[doc = "gmac performance monitor status register"]
pub mod gmac_perf_rd_axi_total_byte;
#[doc = "GMAC_PERF_WR_AXI_TOTAL_BYTE (rw) register accessor: gmac performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_perf_wr_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_perf_wr_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_perf_wr_axi_total_byte`]
module"]
#[doc(alias = "GMAC_PERF_WR_AXI_TOTAL_BYTE")]
pub type GmacPerfWrAxiTotalByte =
    crate::Reg<gmac_perf_wr_axi_total_byte::GmacPerfWrAxiTotalByteSpec>;
#[doc = "gmac performance monitor status register"]
pub mod gmac_perf_wr_axi_total_byte;
#[doc = "GMAC_PERF_WORKING_CNT (rw) register accessor: gmac performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_perf_working_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_perf_working_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_perf_working_cnt`]
module"]
#[doc(alias = "GMAC_PERF_WORKING_CNT")]
pub type GmacPerfWorkingCnt = crate::Reg<gmac_perf_working_cnt::GmacPerfWorkingCntSpec>;
#[doc = "gmac performance monitor status register"]
pub mod gmac_perf_working_cnt;
#[doc = "SOC_CON5 (rw) register accessor: SoC control register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con5`]
module"]
#[doc(alias = "SOC_CON5")]
pub type SocCon5 = crate::Reg<soc_con5::SocCon5Spec>;
#[doc = "SoC control register 5"]
pub mod soc_con5;
#[doc = "SOC_CON6 (rw) register accessor: SoC control register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con6`]
module"]
#[doc(alias = "SOC_CON6")]
pub type SocCon6 = crate::Reg<soc_con6::SocCon6Spec>;
#[doc = "SoC control register 6"]
pub mod soc_con6;
#[doc = "GPIO2A_IOMUX (rw) register accessor: GPIO2A iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2a_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2a_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2a_iomux`]
module"]
#[doc(alias = "GPIO2A_IOMUX")]
pub type Gpio2aIomux = crate::Reg<gpio2a_iomux::Gpio2aIomuxSpec>;
#[doc = "GPIO2A iomux control"]
pub mod gpio2a_iomux;
#[doc = "GPIO2B_IOMUX (rw) register accessor: GPIO2B iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2b_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2b_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2b_iomux`]
module"]
#[doc(alias = "GPIO2B_IOMUX")]
pub type Gpio2bIomux = crate::Reg<gpio2b_iomux::Gpio2bIomuxSpec>;
#[doc = "GPIO2B iomux control"]
pub mod gpio2b_iomux;
#[doc = "GPIO2C_IOMUX (rw) register accessor: GPIO2C iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2c_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2c_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2c_iomux`]
module"]
#[doc(alias = "GPIO2C_IOMUX")]
pub type Gpio2cIomux = crate::Reg<gpio2c_iomux::Gpio2cIomuxSpec>;
#[doc = "GPIO2C iomux control"]
pub mod gpio2c_iomux;
#[doc = "GPIO2D_IOMUX (rw) register accessor: GPIO2D iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2d_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2d_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2d_iomux`]
module"]
#[doc(alias = "GPIO2D_IOMUX")]
pub type Gpio2dIomux = crate::Reg<gpio2d_iomux::Gpio2dIomuxSpec>;
#[doc = "GPIO2D iomux control"]
pub mod gpio2d_iomux;
#[doc = "GPIO3A_IOMUX (rw) register accessor: GPIO3A iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3a_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3a_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3a_iomux`]
module"]
#[doc(alias = "GPIO3A_IOMUX")]
pub type Gpio3aIomux = crate::Reg<gpio3a_iomux::Gpio3aIomuxSpec>;
#[doc = "GPIO3A iomux control"]
pub mod gpio3a_iomux;
#[doc = "GPIO3B_IOMUX (rw) register accessor: GPIO3B iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3b_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3b_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3b_iomux`]
module"]
#[doc(alias = "GPIO3B_IOMUX")]
pub type Gpio3bIomux = crate::Reg<gpio3b_iomux::Gpio3bIomuxSpec>;
#[doc = "GPIO3B iomux control"]
pub mod gpio3b_iomux;
#[doc = "GPIO3C_IOMUX (rw) register accessor: GPIO3C iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3c_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3c_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3c_iomux`]
module"]
#[doc(alias = "GPIO3C_IOMUX")]
pub type Gpio3cIomux = crate::Reg<gpio3c_iomux::Gpio3cIomuxSpec>;
#[doc = "GPIO3C iomux control"]
pub mod gpio3c_iomux;
#[doc = "GPIO3D_IOMUX (rw) register accessor: GPIO3D iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3d_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3d_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3d_iomux`]
module"]
#[doc(alias = "GPIO3D_IOMUX")]
pub type Gpio3dIomux = crate::Reg<gpio3d_iomux::Gpio3dIomuxSpec>;
#[doc = "GPIO3D iomux control"]
pub mod gpio3d_iomux;
#[doc = "GPIO4A_IOMUX (rw) register accessor: GPIO4A iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4a_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4a_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4a_iomux`]
module"]
#[doc(alias = "GPIO4A_IOMUX")]
pub type Gpio4aIomux = crate::Reg<gpio4a_iomux::Gpio4aIomuxSpec>;
#[doc = "GPIO4A iomux control"]
pub mod gpio4a_iomux;
#[doc = "GPIO4B_IOMUX (rw) register accessor: GPIO4B iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4b_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4b_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4b_iomux`]
module"]
#[doc(alias = "GPIO4B_IOMUX")]
pub type Gpio4bIomux = crate::Reg<gpio4b_iomux::Gpio4bIomuxSpec>;
#[doc = "GPIO4B iomux control"]
pub mod gpio4b_iomux;
#[doc = "GPIO4C_IOMUX (rw) register accessor: GPIO4C iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4c_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4c_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4c_iomux`]
module"]
#[doc(alias = "GPIO4C_IOMUX")]
pub type Gpio4cIomux = crate::Reg<gpio4c_iomux::Gpio4cIomuxSpec>;
#[doc = "GPIO4C iomux control"]
pub mod gpio4c_iomux;
#[doc = "GPIO4D_IOMUX (rw) register accessor: GPIO4D iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4d_iomux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4d_iomux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4d_iomux`]
module"]
#[doc(alias = "GPIO4D_IOMUX")]
pub type Gpio4dIomux = crate::Reg<gpio4d_iomux::Gpio4dIomuxSpec>;
#[doc = "GPIO4D iomux control"]
pub mod gpio4d_iomux;
#[doc = "GPIO2A_P (rw) register accessor: GPIO2A PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2a_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2a_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2a_p`]
module"]
#[doc(alias = "GPIO2A_P")]
pub type Gpio2aP = crate::Reg<gpio2a_p::Gpio2aPSpec>;
#[doc = "GPIO2A PU/PD control"]
pub mod gpio2a_p;
#[doc = "GPIO2B_P (rw) register accessor: GPIO2B PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2b_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2b_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2b_p`]
module"]
#[doc(alias = "GPIO2B_P")]
pub type Gpio2bP = crate::Reg<gpio2b_p::Gpio2bPSpec>;
#[doc = "GPIO2B PU/PD control"]
pub mod gpio2b_p;
#[doc = "GPIO2C_P (rw) register accessor: GPIO2C PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2c_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2c_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2c_p`]
module"]
#[doc(alias = "GPIO2C_P")]
pub type Gpio2cP = crate::Reg<gpio2c_p::Gpio2cPSpec>;
#[doc = "GPIO2C PU/PD control"]
pub mod gpio2c_p;
#[doc = "GPIO2D_P (rw) register accessor: GPIO2D PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2d_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2d_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2d_p`]
module"]
#[doc(alias = "GPIO2D_P")]
pub type Gpio2dP = crate::Reg<gpio2d_p::Gpio2dPSpec>;
#[doc = "GPIO2D PU/PD control"]
pub mod gpio2d_p;
#[doc = "GPIO3A_P (rw) register accessor: GPIO3A PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3a_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3a_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3a_p`]
module"]
#[doc(alias = "GPIO3A_P")]
pub type Gpio3aP = crate::Reg<gpio3a_p::Gpio3aPSpec>;
#[doc = "GPIO3A PU/PD control"]
pub mod gpio3a_p;
#[doc = "GPIO3B_P (rw) register accessor: GPIO3B PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3b_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3b_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3b_p`]
module"]
#[doc(alias = "GPIO3B_P")]
pub type Gpio3bP = crate::Reg<gpio3b_p::Gpio3bPSpec>;
#[doc = "GPIO3B PU/PD control"]
pub mod gpio3b_p;
#[doc = "GPIO3C_P (rw) register accessor: GPIO3C PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3c_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3c_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3c_p`]
module"]
#[doc(alias = "GPIO3C_P")]
pub type Gpio3cP = crate::Reg<gpio3c_p::Gpio3cPSpec>;
#[doc = "GPIO3C PU/PD control"]
pub mod gpio3c_p;
#[doc = "GPIO3D_P (rw) register accessor: GPIO3D PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3d_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3d_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3d_p`]
module"]
#[doc(alias = "GPIO3D_P")]
pub type Gpio3dP = crate::Reg<gpio3d_p::Gpio3dPSpec>;
#[doc = "GPIO3D PU/PD control"]
pub mod gpio3d_p;
#[doc = "GPIO4A_P (rw) register accessor: GPIO4A PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4a_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4a_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4a_p`]
module"]
#[doc(alias = "GPIO4A_P")]
pub type Gpio4aP = crate::Reg<gpio4a_p::Gpio4aPSpec>;
#[doc = "GPIO4A PU/PD control"]
pub mod gpio4a_p;
#[doc = "GPIO4B_P (rw) register accessor: GPIO4B PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4b_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4b_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4b_p`]
module"]
#[doc(alias = "GPIO4B_P")]
pub type Gpio4bP = crate::Reg<gpio4b_p::Gpio4bPSpec>;
#[doc = "GPIO4B PU/PD control"]
pub mod gpio4b_p;
#[doc = "GPIO4C_P (rw) register accessor: GPIO4C PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4c_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4c_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4c_p`]
module"]
#[doc(alias = "GPIO4C_P")]
pub type Gpio4cP = crate::Reg<gpio4c_p::Gpio4cPSpec>;
#[doc = "GPIO4C PU/PD control"]
pub mod gpio4c_p;
#[doc = "GPIO4D_P (rw) register accessor: GPIO4D PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4d_p::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4d_p::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4d_p`]
module"]
#[doc(alias = "GPIO4D_P")]
pub type Gpio4dP = crate::Reg<gpio4d_p::Gpio4dPSpec>;
#[doc = "GPIO4D PU/PD control"]
pub mod gpio4d_p;
#[doc = "GPIO2A_SR (rw) register accessor: GPIO2A slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2a_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2a_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2a_sr`]
module"]
#[doc(alias = "GPIO2A_SR")]
pub type Gpio2aSr = crate::Reg<gpio2a_sr::Gpio2aSrSpec>;
#[doc = "GPIO2A slew rate control"]
pub mod gpio2a_sr;
#[doc = "GPIO2B_SR (rw) register accessor: GPIO2B slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2b_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2b_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2b_sr`]
module"]
#[doc(alias = "GPIO2B_SR")]
pub type Gpio2bSr = crate::Reg<gpio2b_sr::Gpio2bSrSpec>;
#[doc = "GPIO2B slew rate control"]
pub mod gpio2b_sr;
#[doc = "GPIO2C_SR (rw) register accessor: GPIO2C slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2c_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2c_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2c_sr`]
module"]
#[doc(alias = "GPIO2C_SR")]
pub type Gpio2cSr = crate::Reg<gpio2c_sr::Gpio2cSrSpec>;
#[doc = "GPIO2C slew rate control"]
pub mod gpio2c_sr;
#[doc = "GPIO2D_SR (rw) register accessor: GPIO2D slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2d_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2d_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2d_sr`]
module"]
#[doc(alias = "GPIO2D_SR")]
pub type Gpio2dSr = crate::Reg<gpio2d_sr::Gpio2dSrSpec>;
#[doc = "GPIO2D slew rate control"]
pub mod gpio2d_sr;
#[doc = "GPIO3D_SR (rw) register accessor: GPIO3D slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3d_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3d_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3d_sr`]
module"]
#[doc(alias = "GPIO3D_SR")]
pub type Gpio3dSr = crate::Reg<gpio3d_sr::Gpio3dSrSpec>;
#[doc = "GPIO3D slew rate control"]
pub mod gpio3d_sr;
#[doc = "GPIO4A_SR (rw) register accessor: GPIO4A slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4a_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4a_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4a_sr`]
module"]
#[doc(alias = "GPIO4A_SR")]
pub type Gpio4aSr = crate::Reg<gpio4a_sr::Gpio4aSrSpec>;
#[doc = "GPIO4A slew rate control"]
pub mod gpio4a_sr;
#[doc = "GPIO4B_SR (rw) register accessor: GPIO4B slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4b_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4b_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4b_sr`]
module"]
#[doc(alias = "GPIO4B_SR")]
pub type Gpio4bSr = crate::Reg<gpio4b_sr::Gpio4bSrSpec>;
#[doc = "GPIO4B slew rate control"]
pub mod gpio4b_sr;
#[doc = "GPIO4C_SR (rw) register accessor: GPIO4C slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4c_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4c_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4c_sr`]
module"]
#[doc(alias = "GPIO4C_SR")]
pub type Gpio4cSr = crate::Reg<gpio4c_sr::Gpio4cSrSpec>;
#[doc = "GPIO4C slew rate control"]
pub mod gpio4c_sr;
#[doc = "GPIO4D_SR (rw) register accessor: GPIO4D slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4d_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4d_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4d_sr`]
module"]
#[doc(alias = "GPIO4D_SR")]
pub type Gpio4dSr = crate::Reg<gpio4d_sr::Gpio4dSrSpec>;
#[doc = "GPIO4D slew rate control"]
pub mod gpio4d_sr;
#[doc = "GPIO2A_SMT (rw) register accessor: GPIO2A smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2a_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2a_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2a_smt`]
module"]
#[doc(alias = "GPIO2A_SMT")]
pub type Gpio2aSmt = crate::Reg<gpio2a_smt::Gpio2aSmtSpec>;
#[doc = "GPIO2A smitter control register"]
pub mod gpio2a_smt;
#[doc = "GPIO2B_SMT (rw) register accessor: GPIO2B smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2b_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2b_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2b_smt`]
module"]
#[doc(alias = "GPIO2B_SMT")]
pub type Gpio2bSmt = crate::Reg<gpio2b_smt::Gpio2bSmtSpec>;
#[doc = "GPIO2B smitter control register"]
pub mod gpio2b_smt;
#[doc = "GPIO2C_SMT (rw) register accessor: GPIO2C smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2c_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2c_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2c_smt`]
module"]
#[doc(alias = "GPIO2C_SMT")]
pub type Gpio2cSmt = crate::Reg<gpio2c_smt::Gpio2cSmtSpec>;
#[doc = "GPIO2C smitter control register"]
pub mod gpio2c_smt;
#[doc = "GPIO2D_SMT (rw) register accessor: GPIO2D smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2d_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2d_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2d_smt`]
module"]
#[doc(alias = "GPIO2D_SMT")]
pub type Gpio2dSmt = crate::Reg<gpio2d_smt::Gpio2dSmtSpec>;
#[doc = "GPIO2D smitter control register"]
pub mod gpio2d_smt;
#[doc = "GPIO3A_SMT (rw) register accessor: GPIO3A smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3a_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3a_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3a_smt`]
module"]
#[doc(alias = "GPIO3A_SMT")]
pub type Gpio3aSmt = crate::Reg<gpio3a_smt::Gpio3aSmtSpec>;
#[doc = "GPIO3A smitter control register"]
pub mod gpio3a_smt;
#[doc = "GPIO3B_SMT (rw) register accessor: GPIO3B smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3b_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3b_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3b_smt`]
module"]
#[doc(alias = "GPIO3B_SMT")]
pub type Gpio3bSmt = crate::Reg<gpio3b_smt::Gpio3bSmtSpec>;
#[doc = "GPIO3B smitter control register"]
pub mod gpio3b_smt;
#[doc = "GPIO3C_SMT (rw) register accessor: GPIO3C smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3c_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3c_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3c_smt`]
module"]
#[doc(alias = "GPIO3C_SMT")]
pub type Gpio3cSmt = crate::Reg<gpio3c_smt::Gpio3cSmtSpec>;
#[doc = "GPIO3C smitter control register"]
pub mod gpio3c_smt;
#[doc = "GPIO3D_SMT (rw) register accessor: GPIO3D smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3d_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3d_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3d_smt`]
module"]
#[doc(alias = "GPIO3D_SMT")]
pub type Gpio3dSmt = crate::Reg<gpio3d_smt::Gpio3dSmtSpec>;
#[doc = "GPIO3D smitter control register"]
pub mod gpio3d_smt;
#[doc = "GPIO4A_SMT (rw) register accessor: GPIO4A smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4a_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4a_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4a_smt`]
module"]
#[doc(alias = "GPIO4A_SMT")]
pub type Gpio4aSmt = crate::Reg<gpio4a_smt::Gpio4aSmtSpec>;
#[doc = "GPIO4A smitter control register"]
pub mod gpio4a_smt;
#[doc = "GPIO4B_SMT (rw) register accessor: GPIO4B smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4b_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4b_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4b_smt`]
module"]
#[doc(alias = "GPIO4B_SMT")]
pub type Gpio4bSmt = crate::Reg<gpio4b_smt::Gpio4bSmtSpec>;
#[doc = "GPIO4B smitter control register"]
pub mod gpio4b_smt;
#[doc = "GPIO4C_SMT (rw) register accessor: GPIO4C smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4c_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4c_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4c_smt`]
module"]
#[doc(alias = "GPIO4C_SMT")]
pub type Gpio4cSmt = crate::Reg<gpio4c_smt::Gpio4cSmtSpec>;
#[doc = "GPIO4C smitter control register"]
pub mod gpio4c_smt;
#[doc = "GPIO4D_SMT (rw) register accessor: GPIO4D smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4d_smt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4d_smt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4d_smt`]
module"]
#[doc(alias = "GPIO4D_SMT")]
pub type Gpio4dSmt = crate::Reg<gpio4d_smt::Gpio4dSmtSpec>;
#[doc = "GPIO4D smitter control register"]
pub mod gpio4d_smt;
#[doc = "GPIO2A_E (rw) register accessor: GPIO2A drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2a_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2a_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2a_e`]
module"]
#[doc(alias = "GPIO2A_E")]
pub type Gpio2aE = crate::Reg<gpio2a_e::Gpio2aESpec>;
#[doc = "GPIO2A drive strength control"]
pub mod gpio2a_e;
#[doc = "GPIO2B_E (rw) register accessor: GPIO2B drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2b_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2b_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2b_e`]
module"]
#[doc(alias = "GPIO2B_E")]
pub type Gpio2bE = crate::Reg<gpio2b_e::Gpio2bESpec>;
#[doc = "GPIO2B drive strength control"]
pub mod gpio2b_e;
#[doc = "GPIO2C_E (rw) register accessor: GPIO2C drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2c_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2c_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2c_e`]
module"]
#[doc(alias = "GPIO2C_E")]
pub type Gpio2cE = crate::Reg<gpio2c_e::Gpio2cESpec>;
#[doc = "GPIO2C drive strength control"]
pub mod gpio2c_e;
#[doc = "GPIO2D_E (rw) register accessor: GPIO2D drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2d_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2d_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2d_e`]
module"]
#[doc(alias = "GPIO2D_E")]
pub type Gpio2dE = crate::Reg<gpio2d_e::Gpio2dESpec>;
#[doc = "GPIO2D drive strength control"]
pub mod gpio2d_e;
#[doc = "GPIO3A_E01 (rw) register accessor: GPIO3A drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3a_e01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3a_e01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3a_e01`]
module"]
#[doc(alias = "GPIO3A_E01")]
pub type Gpio3aE01 = crate::Reg<gpio3a_e01::Gpio3aE01Spec>;
#[doc = "GPIO3A drive strength control"]
pub mod gpio3a_e01;
#[doc = "GPIO3A_E2 (rw) register accessor: GPIO3B drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3a_e2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3a_e2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3a_e2`]
module"]
#[doc(alias = "GPIO3A_E2")]
pub type Gpio3aE2 = crate::Reg<gpio3a_e2::Gpio3aE2Spec>;
#[doc = "GPIO3B drive strength control"]
pub mod gpio3a_e2;
#[doc = "GPIO3B_E01 (rw) register accessor: GPIO3B drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3b_e01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3b_e01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3b_e01`]
module"]
#[doc(alias = "GPIO3B_E01")]
pub type Gpio3bE01 = crate::Reg<gpio3b_e01::Gpio3bE01Spec>;
#[doc = "GPIO3B drive strength control"]
pub mod gpio3b_e01;
#[doc = "GPIO3B_E2 (rw) register accessor: GPIO3B drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3b_e2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3b_e2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3b_e2`]
module"]
#[doc(alias = "GPIO3B_E2")]
pub type Gpio3bE2 = crate::Reg<gpio3b_e2::Gpio3bE2Spec>;
#[doc = "GPIO3B drive strength control"]
pub mod gpio3b_e2;
#[doc = "GPIO3C_E01 (rw) register accessor: GPIO3C drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3c_e01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3c_e01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3c_e01`]
module"]
#[doc(alias = "GPIO3C_E01")]
pub type Gpio3cE01 = crate::Reg<gpio3c_e01::Gpio3cE01Spec>;
#[doc = "GPIO3C drive strength control"]
pub mod gpio3c_e01;
#[doc = "GPIO3C_E2 (rw) register accessor: GPIO3C drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3c_e2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3c_e2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3c_e2`]
module"]
#[doc(alias = "GPIO3C_E2")]
pub type Gpio3cE2 = crate::Reg<gpio3c_e2::Gpio3cE2Spec>;
#[doc = "GPIO3C drive strength control"]
pub mod gpio3c_e2;
#[doc = "GPIO3D_E (rw) register accessor: GPIO3D drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3d_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3d_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3d_e`]
module"]
#[doc(alias = "GPIO3D_E")]
pub type Gpio3dE = crate::Reg<gpio3d_e::Gpio3dESpec>;
#[doc = "GPIO3D drive strength control"]
pub mod gpio3d_e;
#[doc = "GPIO4A_E (rw) register accessor: GPIO4A drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4a_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4a_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4a_e`]
module"]
#[doc(alias = "GPIO4A_E")]
pub type Gpio4aE = crate::Reg<gpio4a_e::Gpio4aESpec>;
#[doc = "GPIO4A drive strength control"]
pub mod gpio4a_e;
#[doc = "GPIO4B_E01 (rw) register accessor: GPIO4B drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4b_e01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4b_e01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4b_e01`]
module"]
#[doc(alias = "GPIO4B_E01")]
pub type Gpio4bE01 = crate::Reg<gpio4b_e01::Gpio4bE01Spec>;
#[doc = "GPIO4B drive strength control"]
pub mod gpio4b_e01;
#[doc = "GPIO4B_E2 (rw) register accessor: GPIO4B drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4b_e2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4b_e2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4b_e2`]
module"]
#[doc(alias = "GPIO4B_E2")]
pub type Gpio4bE2 = crate::Reg<gpio4b_e2::Gpio4bE2Spec>;
#[doc = "GPIO4B drive strength control"]
pub mod gpio4b_e2;
#[doc = "GPIO4C_E (rw) register accessor: GPIO4C drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4c_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4c_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4c_e`]
module"]
#[doc(alias = "GPIO4C_E")]
pub type Gpio4cE = crate::Reg<gpio4c_e::Gpio4cESpec>;
#[doc = "GPIO4C drive strength control"]
pub mod gpio4c_e;
#[doc = "GPIO4D_E (rw) register accessor: GPIO4D drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4d_e::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4d_e::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio4d_e`]
module"]
#[doc(alias = "GPIO4D_E")]
pub type Gpio4dE = crate::Reg<gpio4d_e::Gpio4dESpec>;
#[doc = "GPIO4D drive strength control"]
pub mod gpio4d_e;
#[doc = "GPIO2C_HE (rw) register accessor: GPIO2C HE control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2c_he::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2c_he::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2c_he`]
module"]
#[doc(alias = "GPIO2C_HE")]
pub type Gpio2cHe = crate::Reg<gpio2c_he::Gpio2cHeSpec>;
#[doc = "GPIO2C HE control"]
pub mod gpio2c_he;
#[doc = "GPIO2D_HE (rw) register accessor: GPIO2D HE control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2d_he::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2d_he::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2d_he`]
module"]
#[doc(alias = "GPIO2D_HE")]
pub type Gpio2dHe = crate::Reg<gpio2d_he::Gpio2dHeSpec>;
#[doc = "GPIO2D HE control"]
pub mod gpio2d_he;
#[doc = "SOC_CON0 (rw) register accessor: SoC control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con0`]
module"]
#[doc(alias = "SOC_CON0")]
pub type SocCon0 = crate::Reg<soc_con0::SocCon0Spec>;
#[doc = "SoC control register 0"]
pub mod soc_con0;
#[doc = "SOC_CON1 (rw) register accessor: SoC control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con1`]
module"]
#[doc(alias = "SOC_CON1")]
pub type SocCon1 = crate::Reg<soc_con1::SocCon1Spec>;
#[doc = "SoC control register 2"]
pub mod soc_con1;
#[doc = "SOC_CON2 (rw) register accessor: SoC control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con2`]
module"]
#[doc(alias = "SOC_CON2")]
pub type SocCon2 = crate::Reg<soc_con2::SocCon2Spec>;
#[doc = "SoC control register 1"]
pub mod soc_con2;
#[doc = "SOC_CON3 (rw) register accessor: SoC control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con3`]
module"]
#[doc(alias = "SOC_CON3")]
pub type SocCon3 = crate::Reg<soc_con3::SocCon3Spec>;
#[doc = "SoC control register 3"]
pub mod soc_con3;
#[doc = "SOC_CON4 (rw) register accessor: SoC control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con4`]
module"]
#[doc(alias = "SOC_CON4")]
pub type SocCon4 = crate::Reg<soc_con4::SocCon4Spec>;
#[doc = "SoC control register 4"]
pub mod soc_con4;
#[doc = "SOC_CON_5_PCIE (rw) register accessor: SoC control register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con_5_pcie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con_5_pcie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con_5_pcie`]
module"]
#[doc(alias = "SOC_CON_5_PCIE")]
pub type SocCon5Pcie = crate::Reg<soc_con_5_pcie::SocCon5PcieSpec>;
#[doc = "SoC control register 5"]
pub mod soc_con_5_pcie;
#[doc = "SOC_CON7 (rw) register accessor: SoC control register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con7`]
module"]
#[doc(alias = "SOC_CON7")]
pub type SocCon7 = crate::Reg<soc_con7::SocCon7Spec>;
#[doc = "SoC control register 7"]
pub mod soc_con7;
#[doc = "SOC_CON8 (rw) register accessor: SoC control register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con8`]
module"]
#[doc(alias = "SOC_CON8")]
pub type SocCon8 = crate::Reg<soc_con8::SocCon8Spec>;
#[doc = "SoC control register 8"]
pub mod soc_con8;
#[doc = "SOC_CON_9_PCIE (rw) register accessor: SoC control register 9 for PCIE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con_9_pcie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con_9_pcie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_con_9_pcie`]
module"]
#[doc(alias = "SOC_CON_9_PCIE")]
pub type SocCon9Pcie = crate::Reg<soc_con_9_pcie::SocCon9PcieSpec>;
#[doc = "SoC control register 9 for PCIE"]
pub mod soc_con_9_pcie;
#[doc = "SOC_STATUS0 (rw) register accessor: SOC status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_status0`]
module"]
#[doc(alias = "SOC_STATUS0")]
pub type SocStatus0 = crate::Reg<soc_status0::SocStatus0Spec>;
#[doc = "SOC status register 0"]
pub mod soc_status0;
#[doc = "SOC_STATUS1 (rw) register accessor: SOC status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_status1`]
module"]
#[doc(alias = "SOC_STATUS1")]
pub type SocStatus1 = crate::Reg<soc_status1::SocStatus1Spec>;
#[doc = "SOC status register 1"]
pub mod soc_status1;
#[doc = "SOC_STATUS2 (rw) register accessor: SOC status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_status2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_status2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_status2`]
module"]
#[doc(alias = "SOC_STATUS2")]
pub type SocStatus2 = crate::Reg<soc_status2::SocStatus2Spec>;
#[doc = "SOC status register 2"]
pub mod soc_status2;
#[doc = "SOC_STATUS3 (rw) register accessor: SOC status register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_status3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_status3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_status3`]
module"]
#[doc(alias = "SOC_STATUS3")]
pub type SocStatus3 = crate::Reg<soc_status3::SocStatus3Spec>;
#[doc = "SOC status register 3"]
pub mod soc_status3;
#[doc = "SOC_STATUS4 (rw) register accessor: SOC status register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_status4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_status4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_status4`]
module"]
#[doc(alias = "SOC_STATUS4")]
pub type SocStatus4 = crate::Reg<soc_status4::SocStatus4Spec>;
#[doc = "SOC status register 4"]
pub mod soc_status4;
#[doc = "SOC_STATUS5 (rw) register accessor: SOC status register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_status5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_status5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soc_status5`]
module"]
#[doc(alias = "SOC_STATUS5")]
pub type SocStatus5 = crate::Reg<soc_status5::SocStatus5Spec>;
#[doc = "SOC status register 5"]
pub mod soc_status5;
#[doc = "DDRC0_CON0 (rw) register accessor: ddrc0 control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrc0_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrc0_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrc0_con0`]
module"]
#[doc(alias = "DDRC0_CON0")]
pub type Ddrc0Con0 = crate::Reg<ddrc0_con0::Ddrc0Con0Spec>;
#[doc = "ddrc0 control register 0"]
pub mod ddrc0_con0;
#[doc = "DDRC0_CON1 (rw) register accessor: ddrc0 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrc0_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrc0_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrc0_con1`]
module"]
#[doc(alias = "DDRC0_CON1")]
pub type Ddrc0Con1 = crate::Reg<ddrc0_con1::Ddrc0Con1Spec>;
#[doc = "ddrc0 control register 1"]
pub mod ddrc0_con1;
#[doc = "DDRC1_CON0 (rw) register accessor: ddrc1 control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrc1_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrc1_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrc1_con0`]
module"]
#[doc(alias = "DDRC1_CON0")]
pub type Ddrc1Con0 = crate::Reg<ddrc1_con0::Ddrc1Con0Spec>;
#[doc = "ddrc1 control register 0"]
pub mod ddrc1_con0;
#[doc = "DDRC1_CON1 (rw) register accessor: ddrc1 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrc1_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrc1_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrc1_con1`]
module"]
#[doc(alias = "DDRC1_CON1")]
pub type Ddrc1Con1 = crate::Reg<ddrc1_con1::Ddrc1Con1Spec>;
#[doc = "ddrc1 control register 1"]
pub mod ddrc1_con1;
#[doc = "SIG_DETECT_CON0 (rw) register accessor: Singal detect control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sig_detect_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sig_detect_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sig_detect_con0`]
module"]
#[doc(alias = "SIG_DETECT_CON0")]
pub type SigDetectCon0 = crate::Reg<sig_detect_con0::SigDetectCon0Spec>;
#[doc = "Singal detect control register0"]
pub mod sig_detect_con0;
#[doc = "SIG_DETECT_CON1 (rw) register accessor: Singal detect control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sig_detect_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sig_detect_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sig_detect_con1`]
module"]
#[doc(alias = "SIG_DETECT_CON1")]
pub type SigDetectCon1 = crate::Reg<sig_detect_con1::SigDetectCon1Spec>;
#[doc = "Singal detect control register1"]
pub mod sig_detect_con1;
#[doc = "SIG_DETECT_CLR (rw) register accessor: Signal detect status clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sig_detect_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sig_detect_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sig_detect_clr`]
module"]
#[doc(alias = "SIG_DETECT_CLR")]
pub type SigDetectClr = crate::Reg<sig_detect_clr::SigDetectClrSpec>;
#[doc = "Signal detect status clear register"]
pub mod sig_detect_clr;
#[doc = "SIG_DETECT_STATUS (rw) register accessor: Signal detect status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sig_detect_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sig_detect_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sig_detect_status`]
module"]
#[doc(alias = "SIG_DETECT_STATUS")]
pub type SigDetectStatus = crate::Reg<sig_detect_status::SigDetectStatusSpec>;
#[doc = "Signal detect status register"]
pub mod sig_detect_status;
#[doc = "USB20_PHY0_CON0 (rw) register accessor: USB20 PHY0 GRF Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb20_phy0_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb20_phy0_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb20_phy0_con0`]
module"]
#[doc(alias = "USB20_PHY0_CON0")]
pub type Usb20Phy0Con0 = crate::Reg<usb20_phy0_con0::Usb20Phy0Con0Spec>;
#[doc = "USB20 PHY0 GRF Register 0"]
pub mod usb20_phy0_con0;
#[doc = "USB20_PHY0_CON1 (rw) register accessor: USB20 PHY0 GRF Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb20_phy0_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb20_phy0_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb20_phy0_con1`]
module"]
#[doc(alias = "USB20_PHY0_CON1")]
pub type Usb20Phy0Con1 = crate::Reg<usb20_phy0_con1::Usb20Phy0Con1Spec>;
#[doc = "USB20 PHY0 GRF Register 1"]
pub mod usb20_phy0_con1;
#[doc = "USB20_PHY0_CON2 (rw) register accessor: USB20 PHY0 GRF Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb20_phy0_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb20_phy0_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb20_phy0_con2`]
module"]
#[doc(alias = "USB20_PHY0_CON2")]
pub type Usb20Phy0Con2 = crate::Reg<usb20_phy0_con2::Usb20Phy0Con2Spec>;
#[doc = "USB20 PHY0 GRF Register 2"]
pub mod usb20_phy0_con2;
#[doc = "USB20_PHY0_CON3 (rw) register accessor: USB20 PHY0 GRF Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb20_phy0_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb20_phy0_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb20_phy0_con3`]
module"]
#[doc(alias = "USB20_PHY0_CON3")]
pub type Usb20Phy0Con3 = crate::Reg<usb20_phy0_con3::Usb20Phy0Con3Spec>;
#[doc = "USB20 PHY0 GRF Register 3"]
pub mod usb20_phy0_con3;
#[doc = "USB20_PHY1_CON0 (rw) register accessor: USB20 PHY1 GRF Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb20_phy1_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb20_phy1_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb20_phy1_con0`]
module"]
#[doc(alias = "USB20_PHY1_CON0")]
pub type Usb20Phy1Con0 = crate::Reg<usb20_phy1_con0::Usb20Phy1Con0Spec>;
#[doc = "USB20 PHY1 GRF Register 0"]
pub mod usb20_phy1_con0;
#[doc = "USB20_PHY1_CON1 (rw) register accessor: USB20 PHY1GRF Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb20_phy1_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb20_phy1_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb20_phy1_con1`]
module"]
#[doc(alias = "USB20_PHY1_CON1")]
pub type Usb20Phy1Con1 = crate::Reg<usb20_phy1_con1::Usb20Phy1Con1Spec>;
#[doc = "USB20 PHY1GRF Register 1"]
pub mod usb20_phy1_con1;
#[doc = "USB20_PHY1_CON2 (rw) register accessor: USB20 PHY1 GRF Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb20_phy1_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb20_phy1_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb20_phy1_con2`]
module"]
#[doc(alias = "USB20_PHY1_CON2")]
pub type Usb20Phy1Con2 = crate::Reg<usb20_phy1_con2::Usb20Phy1Con2Spec>;
#[doc = "USB20 PHY1 GRF Register 2"]
pub mod usb20_phy1_con2;
#[doc = "USB20_PHY1_CON3 (rw) register accessor: USB20 PHY1 GRF Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb20_phy1_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb20_phy1_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb20_phy1_con3`]
module"]
#[doc(alias = "USB20_PHY1_CON3")]
pub type Usb20Phy1Con3 = crate::Reg<usb20_phy1_con3::Usb20Phy1Con3Spec>;
#[doc = "USB20 PHY1 GRF Register 3"]
pub mod usb20_phy1_con3;
#[doc = "USB3PHY0_CON0 (rw) register accessor: TypeC PHY/TCPD PHY/TCPC Control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3phy0_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3phy0_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3phy0_con0`]
module"]
#[doc(alias = "USB3PHY0_CON0")]
pub type Usb3phy0Con0 = crate::Reg<usb3phy0_con0::Usb3phy0Con0Spec>;
#[doc = "TypeC PHY/TCPD PHY/TCPC Control register0"]
pub mod usb3phy0_con0;
#[doc = "USB3PHY0_CON1 (rw) register accessor: TypeC PHY/TCPD PHY/TCPC Control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3phy0_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3phy0_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3phy0_con1`]
module"]
#[doc(alias = "USB3PHY0_CON1")]
pub type Usb3phy0Con1 = crate::Reg<usb3phy0_con1::Usb3phy0Con1Spec>;
#[doc = "TypeC PHY/TCPD PHY/TCPC Control register1"]
pub mod usb3phy0_con1;
#[doc = "USB3PHY0_CON2 (rw) register accessor: TypeC PHY/TCPD PHY/TCPC Control register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3phy0_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3phy0_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3phy0_con2`]
module"]
#[doc(alias = "USB3PHY0_CON2")]
pub type Usb3phy0Con2 = crate::Reg<usb3phy0_con2::Usb3phy0Con2Spec>;
#[doc = "TypeC PHY/TCPD PHY/TCPC Control register2"]
pub mod usb3phy0_con2;
#[doc = "USB3PHY1_CON0 (rw) register accessor: TypeC PHY/TCPD PHY/TCPC Control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3phy1_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3phy1_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3phy1_con0`]
module"]
#[doc(alias = "USB3PHY1_CON0")]
pub type Usb3phy1Con0 = crate::Reg<usb3phy1_con0::Usb3phy1Con0Spec>;
#[doc = "TypeC PHY/TCPD PHY/TCPC Control register0"]
pub mod usb3phy1_con0;
#[doc = "USB3PHY1_CON1 (rw) register accessor: TypeC PHY/TCPD PHY/TCPC Control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3phy1_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3phy1_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3phy1_con1`]
module"]
#[doc(alias = "USB3PHY1_CON1")]
pub type Usb3phy1Con1 = crate::Reg<usb3phy1_con1::Usb3phy1Con1Spec>;
#[doc = "TypeC PHY/TCPD PHY/TCPC Control register1"]
pub mod usb3phy1_con1;
#[doc = "USB3PHY1_CON2 (rw) register accessor: TypeC PHY/TCPD PHY/TCPC Control register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3phy1_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3phy1_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3phy1_con2`]
module"]
#[doc(alias = "USB3PHY1_CON2")]
pub type Usb3phy1Con2 = crate::Reg<usb3phy1_con2::Usb3phy1Con2Spec>;
#[doc = "TypeC PHY/TCPD PHY/TCPC Control register2"]
pub mod usb3phy1_con2;
#[doc = "USB3PHY_STATUS0 (rw) register accessor: USB3PHY_STATUS0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3phy_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3phy_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3phy_status0`]
module"]
#[doc(alias = "USB3PHY_STATUS0")]
pub type Usb3phyStatus0 = crate::Reg<usb3phy_status0::Usb3phyStatus0Spec>;
#[doc = "USB3PHY_STATUS0"]
pub mod usb3phy_status0;
#[doc = "USB3PHY_STATUS1 (rw) register accessor: USB3PHY_STATUS1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3phy_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3phy_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb3phy_status1`]
module"]
#[doc(alias = "USB3PHY_STATUS1")]
pub type Usb3phyStatus1 = crate::Reg<usb3phy_status1::Usb3phyStatus1Spec>;
#[doc = "USB3PHY_STATUS1"]
pub mod usb3phy_status1;
#[doc = "DLL_CON0 (rw) register accessor: pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_con0`]
module"]
#[doc(alias = "DLL_CON0")]
pub type DllCon0 = crate::Reg<dll_con0::DllCon0Spec>;
#[doc = "pvtm control register"]
pub mod dll_con0;
#[doc = "DLL_CON1 (rw) register accessor: pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_con1`]
module"]
#[doc(alias = "DLL_CON1")]
pub type DllCon1 = crate::Reg<dll_con1::DllCon1Spec>;
#[doc = "pvtm control register"]
pub mod dll_con1;
#[doc = "DLL_CON2 (rw) register accessor: pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_con2`]
module"]
#[doc(alias = "DLL_CON2")]
pub type DllCon2 = crate::Reg<dll_con2::DllCon2Spec>;
#[doc = "pvtm control register"]
pub mod dll_con2;
#[doc = "DLL_CON3 (rw) register accessor: pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_con3`]
module"]
#[doc(alias = "DLL_CON3")]
pub type DllCon3 = crate::Reg<dll_con3::DllCon3Spec>;
#[doc = "pvtm control register"]
pub mod dll_con3;
#[doc = "DLL_CON4 (rw) register accessor: pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_con4`]
module"]
#[doc(alias = "DLL_CON4")]
pub type DllCon4 = crate::Reg<dll_con4::DllCon4Spec>;
#[doc = "pvtm control register"]
pub mod dll_con4;
#[doc = "DLL_CON5 (rw) register accessor: pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_con5`]
module"]
#[doc(alias = "DLL_CON5")]
pub type DllCon5 = crate::Reg<dll_con5::DllCon5Spec>;
#[doc = "pvtm control register"]
pub mod dll_con5;
#[doc = "DLL_STATUS0 (rw) register accessor: pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_status0`]
module"]
#[doc(alias = "DLL_STATUS0")]
pub type DllStatus0 = crate::Reg<dll_status0::DllStatus0Spec>;
#[doc = "pvtm status register"]
pub mod dll_status0;
#[doc = "DLL_STATUS1 (rw) register accessor: pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_status1`]
module"]
#[doc(alias = "DLL_STATUS1")]
pub type DllStatus1 = crate::Reg<dll_status1::DllStatus1Spec>;
#[doc = "pvtm status register"]
pub mod dll_status1;
#[doc = "DLL_STATUS2 (rw) register accessor: pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_status2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_status2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_status2`]
module"]
#[doc(alias = "DLL_STATUS2")]
pub type DllStatus2 = crate::Reg<dll_status2::DllStatus2Spec>;
#[doc = "pvtm status register"]
pub mod dll_status2;
#[doc = "DLL_STATUS3 (rw) register accessor: pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_status3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_status3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_status3`]
module"]
#[doc(alias = "DLL_STATUS3")]
pub type DllStatus3 = crate::Reg<dll_status3::DllStatus3Spec>;
#[doc = "pvtm status register"]
pub mod dll_status3;
#[doc = "DLL_STATUS4 (rw) register accessor: pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_status4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_status4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll_status4`]
module"]
#[doc(alias = "DLL_STATUS4")]
pub type DllStatus4 = crate::Reg<dll_status4::DllStatus4Spec>;
#[doc = "pvtm status register"]
pub mod dll_status4;
#[doc = "IO_VSEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io_vsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_vsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_vsel`]
module"]
#[doc(alias = "IO_VSEL")]
pub type IoVsel = crate::Reg<io_vsel::IoVselSpec>;
#[doc = ""]
pub mod io_vsel;
#[doc = "SARADC_TESTBIT (rw) register accessor: saradc test bit control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saradc_testbit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saradc_testbit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saradc_testbit`]
module"]
#[doc(alias = "SARADC_TESTBIT")]
pub type SaradcTestbit = crate::Reg<saradc_testbit::SaradcTestbitSpec>;
#[doc = "saradc test bit control register"]
pub mod saradc_testbit;
#[doc = "TSADC_TESTBIT_L (rw) register accessor: saradc test bit control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_testbit_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_testbit_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsadc_testbit_l`]
module"]
#[doc(alias = "TSADC_TESTBIT_L")]
pub type TsadcTestbitL = crate::Reg<tsadc_testbit_l::TsadcTestbitLSpec>;
#[doc = "saradc test bit control register"]
pub mod tsadc_testbit_l;
#[doc = "TSADC_TESTBIT_H (rw) register accessor: tsadc test bit control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_testbit_h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_testbit_h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsadc_testbit_h`]
module"]
#[doc(alias = "TSADC_TESTBIT_H")]
pub type TsadcTestbitH = crate::Reg<tsadc_testbit_h::TsadcTestbitHSpec>;
#[doc = "tsadc test bit control register"]
pub mod tsadc_testbit_h;
#[doc = "CHIP_ID_ADDR (rw) register accessor: chip id register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chip_id_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chip_id_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chip_id_addr`]
module"]
#[doc(alias = "CHIP_ID_ADDR")]
pub type ChipIdAddr = crate::Reg<chip_id_addr::ChipIdAddrSpec>;
#[doc = "chip id register"]
pub mod chip_id_addr;
#[doc = "FAST_BOOT_ADDR (rw) register accessor: faster boot address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fast_boot_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fast_boot_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fast_boot_addr`]
module"]
#[doc(alias = "FAST_BOOT_ADDR")]
pub type FastBootAddr = crate::Reg<fast_boot_addr::FastBootAddrSpec>;
#[doc = "faster boot address register"]
pub mod fast_boot_addr;
#[doc = "EMMCCORE_CON0 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_con0`]
module"]
#[doc(alias = "EMMCCORE_CON0")]
pub type EmmccoreCon0 = crate::Reg<emmccore_con0::EmmccoreCon0Spec>;
#[doc = "emmc core control register"]
pub mod emmccore_con0;
#[doc = "EMMCCORE_CON1 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_con1`]
module"]
#[doc(alias = "EMMCCORE_CON1")]
pub type EmmccoreCon1 = crate::Reg<emmccore_con1::EmmccoreCon1Spec>;
#[doc = "emmc core control register"]
pub mod emmccore_con1;
#[doc = "EMMCCORE_CON2 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_con2`]
module"]
#[doc(alias = "EMMCCORE_CON2")]
pub type EmmccoreCon2 = crate::Reg<emmccore_con2::EmmccoreCon2Spec>;
#[doc = "emmc core control register"]
pub mod emmccore_con2;
#[doc = "EMMCCORE_CON3 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_con3`]
module"]
#[doc(alias = "EMMCCORE_CON3")]
pub type EmmccoreCon3 = crate::Reg<emmccore_con3::EmmccoreCon3Spec>;
#[doc = "emmc core control register"]
pub mod emmccore_con3;
#[doc = "EMMCCORE_CON4 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_con4`]
module"]
#[doc(alias = "EMMCCORE_CON4")]
pub type EmmccoreCon4 = crate::Reg<emmccore_con4::EmmccoreCon4Spec>;
#[doc = "emmc core control register"]
pub mod emmccore_con4;
#[doc = "EMMCCORE_CON5 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_con5`]
module"]
#[doc(alias = "EMMCCORE_CON5")]
pub type EmmccoreCon5 = crate::Reg<emmccore_con5::EmmccoreCon5Spec>;
#[doc = "emmc core control register"]
pub mod emmccore_con5;
#[doc = "EMMCCORE_CON6 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_con6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_con6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_con6`]
module"]
#[doc(alias = "EMMCCORE_CON6")]
pub type EmmccoreCon6 = crate::Reg<emmccore_con6::EmmccoreCon6Spec>;
#[doc = "emmc core control register"]
pub mod emmccore_con6;
#[doc = "EMMCCORE_CON7 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_con7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_con7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_con7`]
module"]
#[doc(alias = "EMMCCORE_CON7")]
pub type EmmccoreCon7 = crate::Reg<emmccore_con7::EmmccoreCon7Spec>;
#[doc = "emmc core control register"]
pub mod emmccore_con7;
#[doc = "EMMCCORE_CON8 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_con8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_con8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_con8`]
module"]
#[doc(alias = "EMMCCORE_CON8")]
pub type EmmccoreCon8 = crate::Reg<emmccore_con8::EmmccoreCon8Spec>;
#[doc = "emmc core control register"]
pub mod emmccore_con8;
#[doc = "EMMCCORE_CON9 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_con9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_con9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_con9`]
module"]
#[doc(alias = "EMMCCORE_CON9")]
pub type EmmccoreCon9 = crate::Reg<emmccore_con9::EmmccoreCon9Spec>;
#[doc = "emmc core control register"]
pub mod emmccore_con9;
#[doc = "EMMCCORE_CON10 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_con10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_con10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_con10`]
module"]
#[doc(alias = "EMMCCORE_CON10")]
pub type EmmccoreCon10 = crate::Reg<emmccore_con10::EmmccoreCon10Spec>;
#[doc = "emmc core control register"]
pub mod emmccore_con10;
#[doc = "EMMCCORE_CON11 (rw) register accessor: emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_con11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_con11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_con11`]
module"]
#[doc(alias = "EMMCCORE_CON11")]
pub type EmmccoreCon11 = crate::Reg<emmccore_con11::EmmccoreCon11Spec>;
#[doc = "emmc core control register"]
pub mod emmccore_con11;
#[doc = "EMMCCORE_STATUS0 (rw) register accessor: emmc core status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_status0`]
module"]
#[doc(alias = "EMMCCORE_STATUS0")]
pub type EmmccoreStatus0 = crate::Reg<emmccore_status0::EmmccoreStatus0Spec>;
#[doc = "emmc core status register"]
pub mod emmccore_status0;
#[doc = "EMMCCORE_STATUS1 (rw) register accessor: emmc core status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_status1`]
module"]
#[doc(alias = "EMMCCORE_STATUS1")]
pub type EmmccoreStatus1 = crate::Reg<emmccore_status1::EmmccoreStatus1Spec>;
#[doc = "emmc core status register"]
pub mod emmccore_status1;
#[doc = "EMMCCORE_STATUS2 (rw) register accessor: emmc core status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_status2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_status2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_status2`]
module"]
#[doc(alias = "EMMCCORE_STATUS2")]
pub type EmmccoreStatus2 = crate::Reg<emmccore_status2::EmmccoreStatus2Spec>;
#[doc = "emmc core status register"]
pub mod emmccore_status2;
#[doc = "EMMCCORE_STATUS3 (rw) register accessor: emmc core status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_status3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_status3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmccore_status3`]
module"]
#[doc(alias = "EMMCCORE_STATUS3")]
pub type EmmccoreStatus3 = crate::Reg<emmccore_status3::EmmccoreStatus3Spec>;
#[doc = "emmc core status register"]
pub mod emmccore_status3;
#[doc = "EMMCPHY_CON0 (rw) register accessor: emmc phy control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmcphy_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmcphy_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmcphy_con0`]
module"]
#[doc(alias = "EMMCPHY_CON0")]
pub type EmmcphyCon0 = crate::Reg<emmcphy_con0::EmmcphyCon0Spec>;
#[doc = "emmc phy control register"]
pub mod emmcphy_con0;
#[doc = "EMMCPHY_CON1 (rw) register accessor: emmc phy control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmcphy_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmcphy_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmcphy_con1`]
module"]
#[doc(alias = "EMMCPHY_CON1")]
pub type EmmcphyCon1 = crate::Reg<emmcphy_con1::EmmcphyCon1Spec>;
#[doc = "emmc phy control register"]
pub mod emmcphy_con1;
#[doc = "EMMCPHY_CON2 (rw) register accessor: emmc phy control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmcphy_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmcphy_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmcphy_con2`]
module"]
#[doc(alias = "EMMCPHY_CON2")]
pub type EmmcphyCon2 = crate::Reg<emmcphy_con2::EmmcphyCon2Spec>;
#[doc = "emmc phy control register"]
pub mod emmcphy_con2;
#[doc = "EMMCPHY_CON3 (rw) register accessor: emmc phy control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmcphy_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmcphy_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmcphy_con3`]
module"]
#[doc(alias = "EMMCPHY_CON3")]
pub type EmmcphyCon3 = crate::Reg<emmcphy_con3::EmmcphyCon3Spec>;
#[doc = "emmc phy control register"]
pub mod emmcphy_con3;
#[doc = "EMMCPHY_CON4 (rw) register accessor: emmc phy control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmcphy_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmcphy_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmcphy_con4`]
module"]
#[doc(alias = "EMMCPHY_CON4")]
pub type EmmcphyCon4 = crate::Reg<emmcphy_con4::EmmcphyCon4Spec>;
#[doc = "emmc phy control register"]
pub mod emmcphy_con4;
#[doc = "EMMCPHY_CON5 (rw) register accessor: emmc phy control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmcphy_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmcphy_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmcphy_con5`]
module"]
#[doc(alias = "EMMCPHY_CON5")]
pub type EmmcphyCon5 = crate::Reg<emmcphy_con5::EmmcphyCon5Spec>;
#[doc = "emmc phy control register"]
pub mod emmcphy_con5;
#[doc = "EMMCPHY_CON6 (rw) register accessor: emmc phy control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmcphy_con6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmcphy_con6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmcphy_con6`]
module"]
#[doc(alias = "EMMCPHY_CON6")]
pub type EmmcphyCon6 = crate::Reg<emmcphy_con6::EmmcphyCon6Spec>;
#[doc = "emmc phy control register"]
pub mod emmcphy_con6;
#[doc = "EMMCPHY_STATUS (rw) register accessor: emmc phy status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmcphy_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmcphy_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmcphy_status`]
module"]
#[doc(alias = "EMMCPHY_STATUS")]
pub type EmmcphyStatus = crate::Reg<emmcphy_status::EmmcphyStatusSpec>;
#[doc = "emmc phy status register"]
pub mod emmcphy_status;
