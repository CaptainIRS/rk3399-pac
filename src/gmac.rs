#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mac_conf: MacConf,
    mac_frm_filt: MacFrmFilt,
    hash_tab_hi: HashTabHi,
    hash_tab_lo: HashTabLo,
    gmii_addr: GmiiAddr,
    gmii_data: GmiiData,
    flow_ctrl: FlowCtrl,
    vlan_tag: VlanTag,
    _reserved8: [u8; 0x04],
    debug: Debug,
    _reserved9: [u8; 0x04],
    pmt_ctrl_sta: PmtCtrlSta,
    _reserved10: [u8; 0x08],
    int_status: IntStatus,
    int_mask: IntMask,
    mac_addr0_hi: MacAddr0Hi,
    mac_addr0_lo: MacAddr0Lo,
    _reserved14: [u8; 0x78],
    an_ctrl: AnCtrl,
    an_status: AnStatus,
    an_adv: AnAdv,
    an_link_part_ab: AnLinkPartAb,
    an_exp: AnExp,
    _reserved19: [u8; 0x04],
    intf_mode_sta: IntfModeSta,
    _reserved20: [u8; 0x24],
    mmc_ctrl: MmcCtrl,
    mmc_rx_intr: MmcRxIntr,
    mmc_tx_intr: MmcTxIntr,
    mmc_rx_int_msk: MmcRxIntMsk,
    mmc_tx_int_msk: MmcTxIntMsk,
    mmc_txoctetcnt_gb: MmcTxoctetcntGb,
    mmc_txfrmcnt_gb: MmcTxfrmcntGb,
    _reserved27: [u8; 0x2c],
    mmc_txundflwerr: MmcTxundflwerr,
    _reserved28: [u8; 0x14],
    mmc_txcarerr: MmcTxcarerr,
    mmc_txoctetcnt_g: MmcTxoctetcntG,
    mmc_txfrmcnt_g: MmcTxfrmcntG,
    _reserved31: [u8; 0x14],
    mmc_rxfrmcnt_gb: MmcRxfrmcntGb,
    mmc_rxoctetcnt_gb: MmcRxoctetcntGb,
    mmc_rxoctetcnt_g: MmcRxoctetcntG,
    _reserved34: [u8; 0x04],
    mmc_rxmcfrmcnt_g: MmcRxmcfrmcntG,
    mmc_rxcrcerr: MmcRxcrcerr,
    _reserved36: [u8; 0x30],
    mmc_rxlenerr: MmcRxlenerr,
    _reserved37: [u8; 0x08],
    mmc_rxfifoovrflw: MmcRxfifoovrflw,
    _reserved38: [u8; 0x28],
    mmc_ipc_int_msk: MmcIpcIntMsk,
    _reserved39: [u8; 0x04],
    mmc_ipc_intr: MmcIpcIntr,
    _reserved40: [u8; 0x04],
    mmc_rxipv4gfrm: MmcRxipv4gfrm,
    mmc_rxipv4hderrfrm: MmcRxipv4hderrfrm,
    _reserved42: [u8; 0x0c],
    mmc_rxipv6gfrm: MmcRxipv6gfrm,
    mmc_rxipv6hderrfrm: MmcRxipv6hderrfrm,
    _reserved44: [u8; 0x08],
    mmc_rxudperrfrm: MmcRxudperrfrm,
    _reserved45: [u8; 0x04],
    mmc_rxtcperrfrm: MmcRxtcperrfrm,
    _reserved46: [u8; 0x04],
    mmc_rxicmperrfrm: MmcRxicmperrfrm,
    _reserved47: [u8; 0x0c],
    mmc_rxipv4hderroct: MmcRxipv4hderroct,
    _reserved48: [u8; 0x10],
    mmc_rxipv6hderroct: MmcRxipv6hderroct,
    _reserved49: [u8; 0x08],
    mmc_rxudperroct: MmcRxudperroct,
    _reserved50: [u8; 0x04],
    mmc_rxtcperroct: MmcRxtcperroct,
    _reserved51: [u8; 0x04],
    mmc_rxicmperroct: MmcRxicmperroct,
    _reserved52: [u8; 0x0d78],
    bus_mode: BusMode,
    tx_poll_demand: TxPollDemand,
    rx_poll_demand: RxPollDemand,
    rx_desc_list_addr: RxDescListAddr,
    tx_desc_list_addr: TxDescListAddr,
    status: Status,
    op_mode: OpMode,
    int_ena: IntEna,
    overflow_cnt: OverflowCnt,
    rec_int_wdt_timer: RecIntWdtTimer,
    axi_bus_mode: AxiBusMode,
    axi_status: AxiStatus,
    _reserved64: [u8; 0x18],
    cur_host_tx_desc: CurHostTxDesc,
    cur_host_rx_desc: CurHostRxDesc,
    cur_host_tx_buf_addr: CurHostTxBufAddr,
    cur_host_rx_buf_addr: CurHostRxBufAddr,
}
impl RegisterBlock {
    #[doc = "0x00 - MAC Configuration Register"]
    #[inline(always)]
    pub const fn mac_conf(&self) -> &MacConf {
        &self.mac_conf
    }
    #[doc = "0x04 - MAC Frame Filter"]
    #[inline(always)]
    pub const fn mac_frm_filt(&self) -> &MacFrmFilt {
        &self.mac_frm_filt
    }
    #[doc = "0x08 - Hash Table High Register"]
    #[inline(always)]
    pub const fn hash_tab_hi(&self) -> &HashTabHi {
        &self.hash_tab_hi
    }
    #[doc = "0x0c - Hash Table Low Register"]
    #[inline(always)]
    pub const fn hash_tab_lo(&self) -> &HashTabLo {
        &self.hash_tab_lo
    }
    #[doc = "0x10 - GMII Address Register"]
    #[inline(always)]
    pub const fn gmii_addr(&self) -> &GmiiAddr {
        &self.gmii_addr
    }
    #[doc = "0x14 - GMII Data Register"]
    #[inline(always)]
    pub const fn gmii_data(&self) -> &GmiiData {
        &self.gmii_data
    }
    #[doc = "0x18 - Flow Control Register"]
    #[inline(always)]
    pub const fn flow_ctrl(&self) -> &FlowCtrl {
        &self.flow_ctrl
    }
    #[doc = "0x1c - VLAN Tag Register"]
    #[inline(always)]
    pub const fn vlan_tag(&self) -> &VlanTag {
        &self.vlan_tag
    }
    #[doc = "0x24 - Debug register"]
    #[inline(always)]
    pub const fn debug(&self) -> &Debug {
        &self.debug
    }
    #[doc = "0x2c - PMT Control and Status Register"]
    #[inline(always)]
    pub const fn pmt_ctrl_sta(&self) -> &PmtCtrlSta {
        &self.pmt_ctrl_sta
    }
    #[doc = "0x38 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn int_status(&self) -> &IntStatus {
        &self.int_status
    }
    #[doc = "0x3c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn int_mask(&self) -> &IntMask {
        &self.int_mask
    }
    #[doc = "0x40 - MAC Address0 High Register"]
    #[inline(always)]
    pub const fn mac_addr0_hi(&self) -> &MacAddr0Hi {
        &self.mac_addr0_hi
    }
    #[doc = "0x44 - MAC Address0 Low Register"]
    #[inline(always)]
    pub const fn mac_addr0_lo(&self) -> &MacAddr0Lo {
        &self.mac_addr0_lo
    }
    #[doc = "0xc0 - AN Control Register"]
    #[inline(always)]
    pub const fn an_ctrl(&self) -> &AnCtrl {
        &self.an_ctrl
    }
    #[doc = "0xc4 - AN Status Register"]
    #[inline(always)]
    pub const fn an_status(&self) -> &AnStatus {
        &self.an_status
    }
    #[doc = "0xc8 - Auto Negotiation Advertisement Register"]
    #[inline(always)]
    pub const fn an_adv(&self) -> &AnAdv {
        &self.an_adv
    }
    #[doc = "0xcc - Auto Negotiation Link Partner Ability Register"]
    #[inline(always)]
    pub const fn an_link_part_ab(&self) -> &AnLinkPartAb {
        &self.an_link_part_ab
    }
    #[doc = "0xd0 - Auto Negotiation Expansion Register"]
    #[inline(always)]
    pub const fn an_exp(&self) -> &AnExp {
        &self.an_exp
    }
    #[doc = "0xd8 - RGMII Status Register"]
    #[inline(always)]
    pub const fn intf_mode_sta(&self) -> &IntfModeSta {
        &self.intf_mode_sta
    }
    #[doc = "0x100 - MMC Control Register"]
    #[inline(always)]
    pub const fn mmc_ctrl(&self) -> &MmcCtrl {
        &self.mmc_ctrl
    }
    #[doc = "0x104 - MMC Receive Interrupt Register"]
    #[inline(always)]
    pub const fn mmc_rx_intr(&self) -> &MmcRxIntr {
        &self.mmc_rx_intr
    }
    #[doc = "0x108 - MMC Transmit Interrupt Register"]
    #[inline(always)]
    pub const fn mmc_tx_intr(&self) -> &MmcTxIntr {
        &self.mmc_tx_intr
    }
    #[doc = "0x10c - MMC Receive Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mmc_rx_int_msk(&self) -> &MmcRxIntMsk {
        &self.mmc_rx_int_msk
    }
    #[doc = "0x110 - MMC Transmit Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mmc_tx_int_msk(&self) -> &MmcTxIntMsk {
        &self.mmc_tx_int_msk
    }
    #[doc = "0x114 - MMC TX OCTET Good and Bad Counter"]
    #[inline(always)]
    pub const fn mmc_txoctetcnt_gb(&self) -> &MmcTxoctetcntGb {
        &self.mmc_txoctetcnt_gb
    }
    #[doc = "0x118 - MMC TX Frame Good and Bad Counter"]
    #[inline(always)]
    pub const fn mmc_txfrmcnt_gb(&self) -> &MmcTxfrmcntGb {
        &self.mmc_txfrmcnt_gb
    }
    #[doc = "0x148 - MMC TX Underflow Error"]
    #[inline(always)]
    pub const fn mmc_txundflwerr(&self) -> &MmcTxundflwerr {
        &self.mmc_txundflwerr
    }
    #[doc = "0x160 - MMC TX Carrier Error"]
    #[inline(always)]
    pub const fn mmc_txcarerr(&self) -> &MmcTxcarerr {
        &self.mmc_txcarerr
    }
    #[doc = "0x164 - MMC TX OCTET Good Counter"]
    #[inline(always)]
    pub const fn mmc_txoctetcnt_g(&self) -> &MmcTxoctetcntG {
        &self.mmc_txoctetcnt_g
    }
    #[doc = "0x168 - MMC TX Frame Good Counter"]
    #[inline(always)]
    pub const fn mmc_txfrmcnt_g(&self) -> &MmcTxfrmcntG {
        &self.mmc_txfrmcnt_g
    }
    #[doc = "0x180 - MMC RX Frame Good and Bad Counter"]
    #[inline(always)]
    pub const fn mmc_rxfrmcnt_gb(&self) -> &MmcRxfrmcntGb {
        &self.mmc_rxfrmcnt_gb
    }
    #[doc = "0x184 - MMC RX OCTET Good and Bad Counter"]
    #[inline(always)]
    pub const fn mmc_rxoctetcnt_gb(&self) -> &MmcRxoctetcntGb {
        &self.mmc_rxoctetcnt_gb
    }
    #[doc = "0x188 - MMC RX OCTET Good Counter"]
    #[inline(always)]
    pub const fn mmc_rxoctetcnt_g(&self) -> &MmcRxoctetcntG {
        &self.mmc_rxoctetcnt_g
    }
    #[doc = "0x190 - MMC RX Multicast Frame Good Counter"]
    #[inline(always)]
    pub const fn mmc_rxmcfrmcnt_g(&self) -> &MmcRxmcfrmcntG {
        &self.mmc_rxmcfrmcnt_g
    }
    #[doc = "0x194 - MMC RX Carrier"]
    #[inline(always)]
    pub const fn mmc_rxcrcerr(&self) -> &MmcRxcrcerr {
        &self.mmc_rxcrcerr
    }
    #[doc = "0x1c8 - MMC RX Length Error"]
    #[inline(always)]
    pub const fn mmc_rxlenerr(&self) -> &MmcRxlenerr {
        &self.mmc_rxlenerr
    }
    #[doc = "0x1d4 - MMC RX FIFO Overflow"]
    #[inline(always)]
    pub const fn mmc_rxfifoovrflw(&self) -> &MmcRxfifoovrflw {
        &self.mmc_rxfifoovrflw
    }
    #[doc = "0x200 - MMC Receive Checksum Offload Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mmc_ipc_int_msk(&self) -> &MmcIpcIntMsk {
        &self.mmc_ipc_int_msk
    }
    #[doc = "0x208 - MMC Receive Checksum Offload Interrupt Register"]
    #[inline(always)]
    pub const fn mmc_ipc_intr(&self) -> &MmcIpcIntr {
        &self.mmc_ipc_intr
    }
    #[doc = "0x210 - MMC RX IPV4 Good Frame"]
    #[inline(always)]
    pub const fn mmc_rxipv4gfrm(&self) -> &MmcRxipv4gfrm {
        &self.mmc_rxipv4gfrm
    }
    #[doc = "0x214 - MMC RX IPV4 Head Error Frame"]
    #[inline(always)]
    pub const fn mmc_rxipv4hderrfrm(&self) -> &MmcRxipv4hderrfrm {
        &self.mmc_rxipv4hderrfrm
    }
    #[doc = "0x224 - MMC RX IPV6 Good Frame"]
    #[inline(always)]
    pub const fn mmc_rxipv6gfrm(&self) -> &MmcRxipv6gfrm {
        &self.mmc_rxipv6gfrm
    }
    #[doc = "0x228 - MMC RX IPV6 Head Error Frame"]
    #[inline(always)]
    pub const fn mmc_rxipv6hderrfrm(&self) -> &MmcRxipv6hderrfrm {
        &self.mmc_rxipv6hderrfrm
    }
    #[doc = "0x234 - MMC RX UDP Error Frame"]
    #[inline(always)]
    pub const fn mmc_rxudperrfrm(&self) -> &MmcRxudperrfrm {
        &self.mmc_rxudperrfrm
    }
    #[doc = "0x23c - MMC RX TCP Error Frame"]
    #[inline(always)]
    pub const fn mmc_rxtcperrfrm(&self) -> &MmcRxtcperrfrm {
        &self.mmc_rxtcperrfrm
    }
    #[doc = "0x244 - MMC RX ICMP Error Frame"]
    #[inline(always)]
    pub const fn mmc_rxicmperrfrm(&self) -> &MmcRxicmperrfrm {
        &self.mmc_rxicmperrfrm
    }
    #[doc = "0x254 - MMC RX OCTET IPV4 Head Error"]
    #[inline(always)]
    pub const fn mmc_rxipv4hderroct(&self) -> &MmcRxipv4hderroct {
        &self.mmc_rxipv4hderroct
    }
    #[doc = "0x268 - MMC RX OCTET IPV6 Head Error"]
    #[inline(always)]
    pub const fn mmc_rxipv6hderroct(&self) -> &MmcRxipv6hderroct {
        &self.mmc_rxipv6hderroct
    }
    #[doc = "0x274 - MMC RX OCTET UDP Error"]
    #[inline(always)]
    pub const fn mmc_rxudperroct(&self) -> &MmcRxudperroct {
        &self.mmc_rxudperroct
    }
    #[doc = "0x27c - MMC RX OCTET TCP Error"]
    #[inline(always)]
    pub const fn mmc_rxtcperroct(&self) -> &MmcRxtcperroct {
        &self.mmc_rxtcperroct
    }
    #[doc = "0x284 - MMC RX OCTET ICMP Error"]
    #[inline(always)]
    pub const fn mmc_rxicmperroct(&self) -> &MmcRxicmperroct {
        &self.mmc_rxicmperroct
    }
    #[doc = "0x1000 - Bus Mode Register"]
    #[inline(always)]
    pub const fn bus_mode(&self) -> &BusMode {
        &self.bus_mode
    }
    #[doc = "0x1004 - Transmit Poll Demand Register"]
    #[inline(always)]
    pub const fn tx_poll_demand(&self) -> &TxPollDemand {
        &self.tx_poll_demand
    }
    #[doc = "0x1008 - Receive Poll Demand Register"]
    #[inline(always)]
    pub const fn rx_poll_demand(&self) -> &RxPollDemand {
        &self.rx_poll_demand
    }
    #[doc = "0x100c - Receive Descriptor List Address Register"]
    #[inline(always)]
    pub const fn rx_desc_list_addr(&self) -> &RxDescListAddr {
        &self.rx_desc_list_addr
    }
    #[doc = "0x1010 - Transmit Descriptor List Address Register"]
    #[inline(always)]
    pub const fn tx_desc_list_addr(&self) -> &TxDescListAddr {
        &self.tx_desc_list_addr
    }
    #[doc = "0x1014 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x1018 - Operation Mode Register"]
    #[inline(always)]
    pub const fn op_mode(&self) -> &OpMode {
        &self.op_mode
    }
    #[doc = "0x101c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &IntEna {
        &self.int_ena
    }
    #[doc = "0x1020 - Missed Frame and Buffer Overflow Counter Register"]
    #[inline(always)]
    pub const fn overflow_cnt(&self) -> &OverflowCnt {
        &self.overflow_cnt
    }
    #[doc = "0x1024 - Receive Interrupt Watchdog Timer Register"]
    #[inline(always)]
    pub const fn rec_int_wdt_timer(&self) -> &RecIntWdtTimer {
        &self.rec_int_wdt_timer
    }
    #[doc = "0x1028 - AXI Bus Mode Register"]
    #[inline(always)]
    pub const fn axi_bus_mode(&self) -> &AxiBusMode {
        &self.axi_bus_mode
    }
    #[doc = "0x102c - AXI Status Register"]
    #[inline(always)]
    pub const fn axi_status(&self) -> &AxiStatus {
        &self.axi_status
    }
    #[doc = "0x1048 - Current Host Transmit Descriptor Register"]
    #[inline(always)]
    pub const fn cur_host_tx_desc(&self) -> &CurHostTxDesc {
        &self.cur_host_tx_desc
    }
    #[doc = "0x104c - Current Host Receive Descriptor Register"]
    #[inline(always)]
    pub const fn cur_host_rx_desc(&self) -> &CurHostRxDesc {
        &self.cur_host_rx_desc
    }
    #[doc = "0x1050 - Current Host Transmit Buffer Address Register"]
    #[inline(always)]
    pub const fn cur_host_tx_buf_addr(&self) -> &CurHostTxBufAddr {
        &self.cur_host_tx_buf_addr
    }
    #[doc = "0x1054 - Current Host Receive Buffer Address Register"]
    #[inline(always)]
    pub const fn cur_host_rx_buf_addr(&self) -> &CurHostRxBufAddr {
        &self.cur_host_rx_buf_addr
    }
}
#[doc = "MAC_CONF (rw) register accessor: MAC Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_conf`]
module"]
#[doc(alias = "MAC_CONF")]
pub type MacConf = crate::Reg<mac_conf::MacConfSpec>;
#[doc = "MAC Configuration Register"]
pub mod mac_conf;
#[doc = "MAC_FRM_FILT (rw) register accessor: MAC Frame Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_frm_filt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_frm_filt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_frm_filt`]
module"]
#[doc(alias = "MAC_FRM_FILT")]
pub type MacFrmFilt = crate::Reg<mac_frm_filt::MacFrmFiltSpec>;
#[doc = "MAC Frame Filter"]
pub mod mac_frm_filt;
#[doc = "HASH_TAB_HI (rw) register accessor: Hash Table High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_tab_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_tab_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_tab_hi`]
module"]
#[doc(alias = "HASH_TAB_HI")]
pub type HashTabHi = crate::Reg<hash_tab_hi::HashTabHiSpec>;
#[doc = "Hash Table High Register"]
pub mod hash_tab_hi;
#[doc = "HASH_TAB_LO (rw) register accessor: Hash Table Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_tab_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_tab_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_tab_lo`]
module"]
#[doc(alias = "HASH_TAB_LO")]
pub type HashTabLo = crate::Reg<hash_tab_lo::HashTabLoSpec>;
#[doc = "Hash Table Low Register"]
pub mod hash_tab_lo;
#[doc = "GMII_ADDR (rw) register accessor: GMII Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmii_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmii_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmii_addr`]
module"]
#[doc(alias = "GMII_ADDR")]
pub type GmiiAddr = crate::Reg<gmii_addr::GmiiAddrSpec>;
#[doc = "GMII Address Register"]
pub mod gmii_addr;
#[doc = "GMII_DATA (rw) register accessor: GMII Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmii_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmii_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmii_data`]
module"]
#[doc(alias = "GMII_DATA")]
pub type GmiiData = crate::Reg<gmii_data::GmiiDataSpec>;
#[doc = "GMII Data Register"]
pub mod gmii_data;
#[doc = "FLOW_CTRL (rw) register accessor: Flow Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flow_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flow_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flow_ctrl`]
module"]
#[doc(alias = "FLOW_CTRL")]
pub type FlowCtrl = crate::Reg<flow_ctrl::FlowCtrlSpec>;
#[doc = "Flow Control Register"]
pub mod flow_ctrl;
#[doc = "VLAN_TAG (rw) register accessor: VLAN Tag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlan_tag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlan_tag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vlan_tag`]
module"]
#[doc(alias = "VLAN_TAG")]
pub type VlanTag = crate::Reg<vlan_tag::VlanTagSpec>;
#[doc = "VLAN Tag Register"]
pub mod vlan_tag;
#[doc = "DEBUG (rw) register accessor: Debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug`]
module"]
#[doc(alias = "DEBUG")]
pub type Debug = crate::Reg<debug::DebugSpec>;
#[doc = "Debug register"]
pub mod debug;
#[doc = "PMT_CTRL_STA (rw) register accessor: PMT Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmt_ctrl_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmt_ctrl_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmt_ctrl_sta`]
module"]
#[doc(alias = "PMT_CTRL_STA")]
pub type PmtCtrlSta = crate::Reg<pmt_ctrl_sta::PmtCtrlStaSpec>;
#[doc = "PMT Control and Status Register"]
pub mod pmt_ctrl_sta;
#[doc = "INT_STATUS (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status`]
module"]
#[doc(alias = "INT_STATUS")]
pub type IntStatus = crate::Reg<int_status::IntStatusSpec>;
#[doc = "Interrupt Status Register"]
pub mod int_status;
#[doc = "INT_MASK (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_mask`]
module"]
#[doc(alias = "INT_MASK")]
pub type IntMask = crate::Reg<int_mask::IntMaskSpec>;
#[doc = "Interrupt Mask Register"]
pub mod int_mask;
#[doc = "MAC_ADDR0_HI (rw) register accessor: MAC Address0 High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_addr0_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_addr0_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_addr0_hi`]
module"]
#[doc(alias = "MAC_ADDR0_HI")]
pub type MacAddr0Hi = crate::Reg<mac_addr0_hi::MacAddr0HiSpec>;
#[doc = "MAC Address0 High Register"]
pub mod mac_addr0_hi;
#[doc = "MAC_ADDR0_LO (rw) register accessor: MAC Address0 Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_addr0_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_addr0_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_addr0_lo`]
module"]
#[doc(alias = "MAC_ADDR0_LO")]
pub type MacAddr0Lo = crate::Reg<mac_addr0_lo::MacAddr0LoSpec>;
#[doc = "MAC Address0 Low Register"]
pub mod mac_addr0_lo;
#[doc = "AN_CTRL (rw) register accessor: AN Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`an_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`an_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@an_ctrl`]
module"]
#[doc(alias = "AN_CTRL")]
pub type AnCtrl = crate::Reg<an_ctrl::AnCtrlSpec>;
#[doc = "AN Control Register"]
pub mod an_ctrl;
#[doc = "AN_STATUS (rw) register accessor: AN Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`an_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`an_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@an_status`]
module"]
#[doc(alias = "AN_STATUS")]
pub type AnStatus = crate::Reg<an_status::AnStatusSpec>;
#[doc = "AN Status Register"]
pub mod an_status;
#[doc = "AN_ADV (rw) register accessor: Auto Negotiation Advertisement Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`an_adv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`an_adv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@an_adv`]
module"]
#[doc(alias = "AN_ADV")]
pub type AnAdv = crate::Reg<an_adv::AnAdvSpec>;
#[doc = "Auto Negotiation Advertisement Register"]
pub mod an_adv;
#[doc = "AN_LINK_PART_AB (r) register accessor: Auto Negotiation Link Partner Ability Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`an_link_part_ab::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@an_link_part_ab`]
module"]
#[doc(alias = "AN_LINK_PART_AB")]
pub type AnLinkPartAb = crate::Reg<an_link_part_ab::AnLinkPartAbSpec>;
#[doc = "Auto Negotiation Link Partner Ability Register"]
pub mod an_link_part_ab;
#[doc = "AN_EXP (r) register accessor: Auto Negotiation Expansion Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`an_exp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@an_exp`]
module"]
#[doc(alias = "AN_EXP")]
pub type AnExp = crate::Reg<an_exp::AnExpSpec>;
#[doc = "Auto Negotiation Expansion Register"]
pub mod an_exp;
#[doc = "INTF_MODE_STA (rw) register accessor: RGMII Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intf_mode_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf_mode_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf_mode_sta`]
module"]
#[doc(alias = "INTF_MODE_STA")]
pub type IntfModeSta = crate::Reg<intf_mode_sta::IntfModeStaSpec>;
#[doc = "RGMII Status Register"]
pub mod intf_mode_sta;
#[doc = "MMC_CTRL (rw) register accessor: MMC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_ctrl`]
module"]
#[doc(alias = "MMC_CTRL")]
pub type MmcCtrl = crate::Reg<mmc_ctrl::MmcCtrlSpec>;
#[doc = "MMC Control Register"]
pub mod mmc_ctrl;
#[doc = "MMC_RX_INTR (rw) register accessor: MMC Receive Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rx_intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rx_intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rx_intr`]
module"]
#[doc(alias = "MMC_RX_INTR")]
pub type MmcRxIntr = crate::Reg<mmc_rx_intr::MmcRxIntrSpec>;
#[doc = "MMC Receive Interrupt Register"]
pub mod mmc_rx_intr;
#[doc = "MMC_TX_INTR (r) register accessor: MMC Transmit Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_tx_intr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_tx_intr`]
module"]
#[doc(alias = "MMC_TX_INTR")]
pub type MmcTxIntr = crate::Reg<mmc_tx_intr::MmcTxIntrSpec>;
#[doc = "MMC Transmit Interrupt Register"]
pub mod mmc_tx_intr;
#[doc = "MMC_RX_INT_MSK (rw) register accessor: MMC Receive Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rx_int_msk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rx_int_msk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rx_int_msk`]
module"]
#[doc(alias = "MMC_RX_INT_MSK")]
pub type MmcRxIntMsk = crate::Reg<mmc_rx_int_msk::MmcRxIntMskSpec>;
#[doc = "MMC Receive Interrupt Mask Register"]
pub mod mmc_rx_int_msk;
#[doc = "MMC_TX_INT_MSK (rw) register accessor: MMC Transmit Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_tx_int_msk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_tx_int_msk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_tx_int_msk`]
module"]
#[doc(alias = "MMC_TX_INT_MSK")]
pub type MmcTxIntMsk = crate::Reg<mmc_tx_int_msk::MmcTxIntMskSpec>;
#[doc = "MMC Transmit Interrupt Mask Register"]
pub mod mmc_tx_int_msk;
#[doc = "MMC_TXOCTETCNT_GB (rw) register accessor: MMC TX OCTET Good and Bad Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_txoctetcnt_gb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_txoctetcnt_gb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_txoctetcnt_gb`]
module"]
#[doc(alias = "MMC_TXOCTETCNT_GB")]
pub type MmcTxoctetcntGb = crate::Reg<mmc_txoctetcnt_gb::MmcTxoctetcntGbSpec>;
#[doc = "MMC TX OCTET Good and Bad Counter"]
pub mod mmc_txoctetcnt_gb;
#[doc = "MMC_TXFRMCNT_GB (rw) register accessor: MMC TX Frame Good and Bad Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_txfrmcnt_gb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_txfrmcnt_gb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_txfrmcnt_gb`]
module"]
#[doc(alias = "MMC_TXFRMCNT_GB")]
pub type MmcTxfrmcntGb = crate::Reg<mmc_txfrmcnt_gb::MmcTxfrmcntGbSpec>;
#[doc = "MMC TX Frame Good and Bad Counter"]
pub mod mmc_txfrmcnt_gb;
#[doc = "MMC_TXUNDFLWERR (rw) register accessor: MMC TX Underflow Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_txundflwerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_txundflwerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_txundflwerr`]
module"]
#[doc(alias = "MMC_TXUNDFLWERR")]
pub type MmcTxundflwerr = crate::Reg<mmc_txundflwerr::MmcTxundflwerrSpec>;
#[doc = "MMC TX Underflow Error"]
pub mod mmc_txundflwerr;
#[doc = "MMC_TXCARERR (rw) register accessor: MMC TX Carrier Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_txcarerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_txcarerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_txcarerr`]
module"]
#[doc(alias = "MMC_TXCARERR")]
pub type MmcTxcarerr = crate::Reg<mmc_txcarerr::MmcTxcarerrSpec>;
#[doc = "MMC TX Carrier Error"]
pub mod mmc_txcarerr;
#[doc = "MMC_TXOCTETCNT_G (rw) register accessor: MMC TX OCTET Good Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_txoctetcnt_g::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_txoctetcnt_g::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_txoctetcnt_g`]
module"]
#[doc(alias = "MMC_TXOCTETCNT_G")]
pub type MmcTxoctetcntG = crate::Reg<mmc_txoctetcnt_g::MmcTxoctetcntGSpec>;
#[doc = "MMC TX OCTET Good Counter"]
pub mod mmc_txoctetcnt_g;
#[doc = "MMC_TXFRMCNT_G (rw) register accessor: MMC TX Frame Good Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_txfrmcnt_g::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_txfrmcnt_g::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_txfrmcnt_g`]
module"]
#[doc(alias = "MMC_TXFRMCNT_G")]
pub type MmcTxfrmcntG = crate::Reg<mmc_txfrmcnt_g::MmcTxfrmcntGSpec>;
#[doc = "MMC TX Frame Good Counter"]
pub mod mmc_txfrmcnt_g;
#[doc = "MMC_RXFRMCNT_GB (rw) register accessor: MMC RX Frame Good and Bad Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxfrmcnt_gb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxfrmcnt_gb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxfrmcnt_gb`]
module"]
#[doc(alias = "MMC_RXFRMCNT_GB")]
pub type MmcRxfrmcntGb = crate::Reg<mmc_rxfrmcnt_gb::MmcRxfrmcntGbSpec>;
#[doc = "MMC RX Frame Good and Bad Counter"]
pub mod mmc_rxfrmcnt_gb;
#[doc = "MMC_RXOCTETCNT_GB (rw) register accessor: MMC RX OCTET Good and Bad Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxoctetcnt_gb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxoctetcnt_gb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxoctetcnt_gb`]
module"]
#[doc(alias = "MMC_RXOCTETCNT_GB")]
pub type MmcRxoctetcntGb = crate::Reg<mmc_rxoctetcnt_gb::MmcRxoctetcntGbSpec>;
#[doc = "MMC RX OCTET Good and Bad Counter"]
pub mod mmc_rxoctetcnt_gb;
#[doc = "MMC_RXOCTETCNT_G (rw) register accessor: MMC RX OCTET Good Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxoctetcnt_g::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxoctetcnt_g::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxoctetcnt_g`]
module"]
#[doc(alias = "MMC_RXOCTETCNT_G")]
pub type MmcRxoctetcntG = crate::Reg<mmc_rxoctetcnt_g::MmcRxoctetcntGSpec>;
#[doc = "MMC RX OCTET Good Counter"]
pub mod mmc_rxoctetcnt_g;
#[doc = "MMC_RXMCFRMCNT_G (rw) register accessor: MMC RX Multicast Frame Good Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxmcfrmcnt_g::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxmcfrmcnt_g::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxmcfrmcnt_g`]
module"]
#[doc(alias = "MMC_RXMCFRMCNT_G")]
pub type MmcRxmcfrmcntG = crate::Reg<mmc_rxmcfrmcnt_g::MmcRxmcfrmcntGSpec>;
#[doc = "MMC RX Multicast Frame Good Counter"]
pub mod mmc_rxmcfrmcnt_g;
#[doc = "MMC_RXCRCERR (rw) register accessor: MMC RX Carrier\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxcrcerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxcrcerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxcrcerr`]
module"]
#[doc(alias = "MMC_RXCRCERR")]
pub type MmcRxcrcerr = crate::Reg<mmc_rxcrcerr::MmcRxcrcerrSpec>;
#[doc = "MMC RX Carrier"]
pub mod mmc_rxcrcerr;
#[doc = "MMC_RXLENERR (rw) register accessor: MMC RX Length Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxlenerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxlenerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxlenerr`]
module"]
#[doc(alias = "MMC_RXLENERR")]
pub type MmcRxlenerr = crate::Reg<mmc_rxlenerr::MmcRxlenerrSpec>;
#[doc = "MMC RX Length Error"]
pub mod mmc_rxlenerr;
#[doc = "MMC_RXFIFOOVRFLW (rw) register accessor: MMC RX FIFO Overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxfifoovrflw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxfifoovrflw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxfifoovrflw`]
module"]
#[doc(alias = "MMC_RXFIFOOVRFLW")]
pub type MmcRxfifoovrflw = crate::Reg<mmc_rxfifoovrflw::MmcRxfifoovrflwSpec>;
#[doc = "MMC RX FIFO Overflow"]
pub mod mmc_rxfifoovrflw;
#[doc = "MMC_IPC_INT_MSK (rw) register accessor: MMC Receive Checksum Offload Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_ipc_int_msk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_ipc_int_msk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_ipc_int_msk`]
module"]
#[doc(alias = "MMC_IPC_INT_MSK")]
pub type MmcIpcIntMsk = crate::Reg<mmc_ipc_int_msk::MmcIpcIntMskSpec>;
#[doc = "MMC Receive Checksum Offload Interrupt Mask Register"]
pub mod mmc_ipc_int_msk;
#[doc = "MMC_IPC_INTR (r) register accessor: MMC Receive Checksum Offload Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_ipc_intr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_ipc_intr`]
module"]
#[doc(alias = "MMC_IPC_INTR")]
pub type MmcIpcIntr = crate::Reg<mmc_ipc_intr::MmcIpcIntrSpec>;
#[doc = "MMC Receive Checksum Offload Interrupt Register"]
pub mod mmc_ipc_intr;
#[doc = "MMC_RXIPV4GFRM (rw) register accessor: MMC RX IPV4 Good Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxipv4gfrm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxipv4gfrm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxipv4gfrm`]
module"]
#[doc(alias = "MMC_RXIPV4GFRM")]
pub type MmcRxipv4gfrm = crate::Reg<mmc_rxipv4gfrm::MmcRxipv4gfrmSpec>;
#[doc = "MMC RX IPV4 Good Frame"]
pub mod mmc_rxipv4gfrm;
#[doc = "MMC_RXIPV4HDERRFRM (rw) register accessor: MMC RX IPV4 Head Error Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxipv4hderrfrm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxipv4hderrfrm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxipv4hderrfrm`]
module"]
#[doc(alias = "MMC_RXIPV4HDERRFRM")]
pub type MmcRxipv4hderrfrm = crate::Reg<mmc_rxipv4hderrfrm::MmcRxipv4hderrfrmSpec>;
#[doc = "MMC RX IPV4 Head Error Frame"]
pub mod mmc_rxipv4hderrfrm;
#[doc = "MMC_RXIPV6GFRM (rw) register accessor: MMC RX IPV6 Good Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxipv6gfrm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxipv6gfrm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxipv6gfrm`]
module"]
#[doc(alias = "MMC_RXIPV6GFRM")]
pub type MmcRxipv6gfrm = crate::Reg<mmc_rxipv6gfrm::MmcRxipv6gfrmSpec>;
#[doc = "MMC RX IPV6 Good Frame"]
pub mod mmc_rxipv6gfrm;
#[doc = "MMC_RXIPV6HDERRFRM (rw) register accessor: MMC RX IPV6 Head Error Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxipv6hderrfrm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxipv6hderrfrm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxipv6hderrfrm`]
module"]
#[doc(alias = "MMC_RXIPV6HDERRFRM")]
pub type MmcRxipv6hderrfrm = crate::Reg<mmc_rxipv6hderrfrm::MmcRxipv6hderrfrmSpec>;
#[doc = "MMC RX IPV6 Head Error Frame"]
pub mod mmc_rxipv6hderrfrm;
#[doc = "MMC_RXUDPERRFRM (rw) register accessor: MMC RX UDP Error Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxudperrfrm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxudperrfrm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxudperrfrm`]
module"]
#[doc(alias = "MMC_RXUDPERRFRM")]
pub type MmcRxudperrfrm = crate::Reg<mmc_rxudperrfrm::MmcRxudperrfrmSpec>;
#[doc = "MMC RX UDP Error Frame"]
pub mod mmc_rxudperrfrm;
#[doc = "MMC_RXTCPERRFRM (rw) register accessor: MMC RX TCP Error Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxtcperrfrm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxtcperrfrm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxtcperrfrm`]
module"]
#[doc(alias = "MMC_RXTCPERRFRM")]
pub type MmcRxtcperrfrm = crate::Reg<mmc_rxtcperrfrm::MmcRxtcperrfrmSpec>;
#[doc = "MMC RX TCP Error Frame"]
pub mod mmc_rxtcperrfrm;
#[doc = "MMC_RXICMPERRFRM (rw) register accessor: MMC RX ICMP Error Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxicmperrfrm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxicmperrfrm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxicmperrfrm`]
module"]
#[doc(alias = "MMC_RXICMPERRFRM")]
pub type MmcRxicmperrfrm = crate::Reg<mmc_rxicmperrfrm::MmcRxicmperrfrmSpec>;
#[doc = "MMC RX ICMP Error Frame"]
pub mod mmc_rxicmperrfrm;
#[doc = "MMC_RXIPV4HDERROCT (rw) register accessor: MMC RX OCTET IPV4 Head Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxipv4hderroct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxipv4hderroct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxipv4hderroct`]
module"]
#[doc(alias = "MMC_RXIPV4HDERROCT")]
pub type MmcRxipv4hderroct = crate::Reg<mmc_rxipv4hderroct::MmcRxipv4hderroctSpec>;
#[doc = "MMC RX OCTET IPV4 Head Error"]
pub mod mmc_rxipv4hderroct;
#[doc = "MMC_RXIPV6HDERROCT (rw) register accessor: MMC RX OCTET IPV6 Head Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxipv6hderroct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxipv6hderroct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxipv6hderroct`]
module"]
#[doc(alias = "MMC_RXIPV6HDERROCT")]
pub type MmcRxipv6hderroct = crate::Reg<mmc_rxipv6hderroct::MmcRxipv6hderroctSpec>;
#[doc = "MMC RX OCTET IPV6 Head Error"]
pub mod mmc_rxipv6hderroct;
#[doc = "MMC_RXUDPERROCT (rw) register accessor: MMC RX OCTET UDP Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxudperroct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxudperroct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxudperroct`]
module"]
#[doc(alias = "MMC_RXUDPERROCT")]
pub type MmcRxudperroct = crate::Reg<mmc_rxudperroct::MmcRxudperroctSpec>;
#[doc = "MMC RX OCTET UDP Error"]
pub mod mmc_rxudperroct;
#[doc = "MMC_RXTCPERROCT (rw) register accessor: MMC RX OCTET TCP Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxtcperroct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxtcperroct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxtcperroct`]
module"]
#[doc(alias = "MMC_RXTCPERROCT")]
pub type MmcRxtcperroct = crate::Reg<mmc_rxtcperroct::MmcRxtcperroctSpec>;
#[doc = "MMC RX OCTET TCP Error"]
pub mod mmc_rxtcperroct;
#[doc = "MMC_RXICMPERROCT (rw) register accessor: MMC RX OCTET ICMP Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxicmperroct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxicmperroct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmc_rxicmperroct`]
module"]
#[doc(alias = "MMC_RXICMPERROCT")]
pub type MmcRxicmperroct = crate::Reg<mmc_rxicmperroct::MmcRxicmperroctSpec>;
#[doc = "MMC RX OCTET ICMP Error"]
pub mod mmc_rxicmperroct;
#[doc = "BUS_MODE (rw) register accessor: Bus Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_mode`]
module"]
#[doc(alias = "BUS_MODE")]
pub type BusMode = crate::Reg<bus_mode::BusModeSpec>;
#[doc = "Bus Mode Register"]
pub mod bus_mode;
#[doc = "TX_POLL_DEMAND (r) register accessor: Transmit Poll Demand Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_poll_demand::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_poll_demand`]
module"]
#[doc(alias = "TX_POLL_DEMAND")]
pub type TxPollDemand = crate::Reg<tx_poll_demand::TxPollDemandSpec>;
#[doc = "Transmit Poll Demand Register"]
pub mod tx_poll_demand;
#[doc = "RX_POLL_DEMAND (r) register accessor: Receive Poll Demand Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_poll_demand::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_poll_demand`]
module"]
#[doc(alias = "RX_POLL_DEMAND")]
pub type RxPollDemand = crate::Reg<rx_poll_demand::RxPollDemandSpec>;
#[doc = "Receive Poll Demand Register"]
pub mod rx_poll_demand;
#[doc = "RX_DESC_LIST_ADDR (rw) register accessor: Receive Descriptor List Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_desc_list_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_desc_list_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_desc_list_addr`]
module"]
#[doc(alias = "RX_DESC_LIST_ADDR")]
pub type RxDescListAddr = crate::Reg<rx_desc_list_addr::RxDescListAddrSpec>;
#[doc = "Receive Descriptor List Address Register"]
pub mod rx_desc_list_addr;
#[doc = "TX_DESC_LIST_ADDR (rw) register accessor: Transmit Descriptor List Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_desc_list_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_desc_list_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_desc_list_addr`]
module"]
#[doc(alias = "TX_DESC_LIST_ADDR")]
pub type TxDescListAddr = crate::Reg<tx_desc_list_addr::TxDescListAddrSpec>;
#[doc = "Transmit Descriptor List Address Register"]
pub mod tx_desc_list_addr;
#[doc = "STATUS (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "OP_MODE (rw) register accessor: Operation Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op_mode`]
module"]
#[doc(alias = "OP_MODE")]
pub type OpMode = crate::Reg<op_mode::OpModeSpec>;
#[doc = "Operation Mode Register"]
pub mod op_mode;
#[doc = "INT_ENA (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`]
module"]
#[doc(alias = "INT_ENA")]
pub type IntEna = crate::Reg<int_ena::IntEnaSpec>;
#[doc = "Interrupt Enable Register"]
pub mod int_ena;
#[doc = "OVERFLOW_CNT (r) register accessor: Missed Frame and Buffer Overflow Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`overflow_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@overflow_cnt`]
module"]
#[doc(alias = "OVERFLOW_CNT")]
pub type OverflowCnt = crate::Reg<overflow_cnt::OverflowCntSpec>;
#[doc = "Missed Frame and Buffer Overflow Counter Register"]
pub mod overflow_cnt;
#[doc = "REC_INT_WDT_TIMER (rw) register accessor: Receive Interrupt Watchdog Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rec_int_wdt_timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rec_int_wdt_timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rec_int_wdt_timer`]
module"]
#[doc(alias = "REC_INT_WDT_TIMER")]
pub type RecIntWdtTimer = crate::Reg<rec_int_wdt_timer::RecIntWdtTimerSpec>;
#[doc = "Receive Interrupt Watchdog Timer Register"]
pub mod rec_int_wdt_timer;
#[doc = "AXI_BUS_MODE (rw) register accessor: AXI Bus Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_bus_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_bus_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_bus_mode`]
module"]
#[doc(alias = "AXI_BUS_MODE")]
pub type AxiBusMode = crate::Reg<axi_bus_mode::AxiBusModeSpec>;
#[doc = "AXI Bus Mode Register"]
pub mod axi_bus_mode;
#[doc = "AXI_STATUS (r) register accessor: AXI Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_status`]
module"]
#[doc(alias = "AXI_STATUS")]
pub type AxiStatus = crate::Reg<axi_status::AxiStatusSpec>;
#[doc = "AXI Status Register"]
pub mod axi_status;
#[doc = "CUR_HOST_TX_DESC (r) register accessor: Current Host Transmit Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cur_host_tx_desc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cur_host_tx_desc`]
module"]
#[doc(alias = "CUR_HOST_TX_DESC")]
pub type CurHostTxDesc = crate::Reg<cur_host_tx_desc::CurHostTxDescSpec>;
#[doc = "Current Host Transmit Descriptor Register"]
pub mod cur_host_tx_desc;
#[doc = "CUR_HOST_RX_DESC (r) register accessor: Current Host Receive Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cur_host_rx_desc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cur_host_rx_desc`]
module"]
#[doc(alias = "CUR_HOST_RX_DESC")]
pub type CurHostRxDesc = crate::Reg<cur_host_rx_desc::CurHostRxDescSpec>;
#[doc = "Current Host Receive Descriptor Register"]
pub mod cur_host_rx_desc;
#[doc = "CUR_HOST_TX_BUF_ADDR (r) register accessor: Current Host Transmit Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cur_host_tx_buf_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cur_host_tx_buf_addr`]
module"]
#[doc(alias = "CUR_HOST_TX_BUF_ADDR")]
pub type CurHostTxBufAddr = crate::Reg<cur_host_tx_buf_addr::CurHostTxBufAddrSpec>;
#[doc = "Current Host Transmit Buffer Address Register"]
pub mod cur_host_tx_buf_addr;
#[doc = "CUR_HOST_RX_BUF_ADDR (r) register accessor: Current Host Receive Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cur_host_rx_buf_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cur_host_rx_buf_addr`]
module"]
#[doc(alias = "CUR_HOST_RX_BUF_ADDR")]
pub type CurHostRxBufAddr = crate::Reg<cur_host_rx_buf_addr::CurHostRxBufAddrSpec>;
#[doc = "Current Host Receive Buffer Address Register"]
pub mod cur_host_rx_buf_addr;
