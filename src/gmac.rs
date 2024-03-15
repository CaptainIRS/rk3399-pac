#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gmac_mac_conf: GmacMacConf,
    gmac_mac_frm_filt: GmacMacFrmFilt,
    gmac_hash_tab_hi: GmacHashTabHi,
    gmac_hash_tab_lo: GmacHashTabLo,
    gmac_gmii_addr: GmacGmiiAddr,
    gmac_gmii_data: GmacGmiiData,
    gmac_flow_ctrl: GmacFlowCtrl,
    gmac_vlan_tag: GmacVlanTag,
    _reserved8: [u8; 0x04],
    gmac_debug: GmacDebug,
    _reserved9: [u8; 0x04],
    gmac_pmt_ctrl_sta: GmacPmtCtrlSta,
    _reserved10: [u8; 0x08],
    gmac_int_status: GmacIntStatus,
    gmac_int_mask: GmacIntMask,
    gmac_mac_addr0_hi: GmacMacAddr0Hi,
    gmac_mac_addr0_lo: GmacMacAddr0Lo,
    _reserved14: [u8; 0x78],
    gmac_an_ctrl: GmacAnCtrl,
    gmac_an_status: GmacAnStatus,
    gmac_an_adv: GmacAnAdv,
    gmac_an_link_part_ab: GmacAnLinkPartAb,
    gmac_an_exp: GmacAnExp,
    _reserved19: [u8; 0x04],
    gmac_intf_mode_sta: GmacIntfModeSta,
    _reserved20: [u8; 0x24],
    gmac_mmc_ctrl: GmacMmcCtrl,
    gmac_mmc_rx_intr: GmacMmcRxIntr,
    gmac_mmc_tx_intr: GmacMmcTxIntr,
    gmac_mmc_rx_int_msk: GmacMmcRxIntMsk,
    gmac_mmc_tx_int_msk: GmacMmcTxIntMsk,
    gmac_mmc_txoctetcnt_gb: GmacMmcTxoctetcntGb,
    gmac_mmc_txfrmcnt_gb: GmacMmcTxfrmcntGb,
    _reserved27: [u8; 0x2c],
    gmac_mmc_txundflwerr: GmacMmcTxundflwerr,
    _reserved28: [u8; 0x14],
    gmac_mmc_txcarerr: GmacMmcTxcarerr,
    gmac_mmc_txoctetcnt_g: GmacMmcTxoctetcntG,
    gmac_mmc_txfrmcnt_g: GmacMmcTxfrmcntG,
    _reserved31: [u8; 0x14],
    gmac_mmc_rxfrmcnt_gb: GmacMmcRxfrmcntGb,
    gmac_mmc_rxoctetcnt_gb: GmacMmcRxoctetcntGb,
    gmac_mmc_rxoctetcnt_g: GmacMmcRxoctetcntG,
    _reserved34: [u8; 0x04],
    gmac_mmc_rxmcfrmcnt_g: GmacMmcRxmcfrmcntG,
    gmac_mmc_rxcrcerr: GmacMmcRxcrcerr,
    _reserved36: [u8; 0x30],
    gmac_mmc_rxlenerr: GmacMmcRxlenerr,
    _reserved37: [u8; 0x08],
    gmac_mmc_rxfifoovrflw: GmacMmcRxfifoovrflw,
    _reserved38: [u8; 0x28],
    gmac_mmc_ipc_int_msk: GmacMmcIpcIntMsk,
    _reserved39: [u8; 0x04],
    gmac_mmc_ipc_intr: GmacMmcIpcIntr,
    _reserved40: [u8; 0x04],
    gmac_mmc_rxipv4gfrm: GmacMmcRxipv4gfrm,
    gmac_mmc_rxipv4hderrfrm: GmacMmcRxipv4hderrfrm,
    _reserved42: [u8; 0x0c],
    gmac_mmc_rxipv6gfrm: GmacMmcRxipv6gfrm,
    gmac_mmc_rxipv6hderrfrm: GmacMmcRxipv6hderrfrm,
    _reserved44: [u8; 0x08],
    gmac_mmc_rxudperrfrm: GmacMmcRxudperrfrm,
    _reserved45: [u8; 0x04],
    gmac_mmc_rxtcperrfrm: GmacMmcRxtcperrfrm,
    _reserved46: [u8; 0x04],
    gmac_mmc_rxicmperrfrm: GmacMmcRxicmperrfrm,
    _reserved47: [u8; 0x0c],
    gmac_mmc_rxipv4hderroct: GmacMmcRxipv4hderroct,
    _reserved48: [u8; 0x10],
    gmac_mmc_rxipv6hderroct: GmacMmcRxipv6hderroct,
    _reserved49: [u8; 0x08],
    gmac_mmc_rxudperroct: GmacMmcRxudperroct,
    _reserved50: [u8; 0x04],
    gmac_mmc_rxtcperroct: GmacMmcRxtcperroct,
    _reserved51: [u8; 0x04],
    gmac_mmc_rxicmperroct: GmacMmcRxicmperroct,
    _reserved52: [u8; 0x0d78],
    gmac_bus_mode: GmacBusMode,
    gmac_tx_poll_demand: GmacTxPollDemand,
    gmac_rx_poll_demand: GmacRxPollDemand,
    gmac_rx_desc_list_addr: GmacRxDescListAddr,
    gmac_tx_desc_list_addr: GmacTxDescListAddr,
    gmac_status: GmacStatus,
    gmac_op_mode: GmacOpMode,
    gmac_int_ena: GmacIntEna,
    gmac_overflow_cnt: GmacOverflowCnt,
    gmac_rec_int_wdt_timer: GmacRecIntWdtTimer,
    gmac_axi_bus_mode: GmacAxiBusMode,
    gmac_axi_status: GmacAxiStatus,
    _reserved64: [u8; 0x18],
    gmac_cur_host_tx_desc: GmacCurHostTxDesc,
    gmac_cur_host_rx_desc: GmacCurHostRxDesc,
    gmac_cur_host_tx_buf_addr: GmacCurHostTxBufAddr,
    gmac_cur_host_rx_buf_addr: GmacCurHostRxBufAddr,
}
impl RegisterBlock {
    #[doc = "0x00 - MAC Configuration Register"]
    #[inline(always)]
    pub const fn gmac_mac_conf(&self) -> &GmacMacConf {
        &self.gmac_mac_conf
    }
    #[doc = "0x04 - MAC Frame Filter"]
    #[inline(always)]
    pub const fn gmac_mac_frm_filt(&self) -> &GmacMacFrmFilt {
        &self.gmac_mac_frm_filt
    }
    #[doc = "0x08 - Hash Table High Register"]
    #[inline(always)]
    pub const fn gmac_hash_tab_hi(&self) -> &GmacHashTabHi {
        &self.gmac_hash_tab_hi
    }
    #[doc = "0x0c - Hash Table Low Register"]
    #[inline(always)]
    pub const fn gmac_hash_tab_lo(&self) -> &GmacHashTabLo {
        &self.gmac_hash_tab_lo
    }
    #[doc = "0x10 - GMII Address Register"]
    #[inline(always)]
    pub const fn gmac_gmii_addr(&self) -> &GmacGmiiAddr {
        &self.gmac_gmii_addr
    }
    #[doc = "0x14 - GMII Data Register"]
    #[inline(always)]
    pub const fn gmac_gmii_data(&self) -> &GmacGmiiData {
        &self.gmac_gmii_data
    }
    #[doc = "0x18 - Flow Control Register"]
    #[inline(always)]
    pub const fn gmac_flow_ctrl(&self) -> &GmacFlowCtrl {
        &self.gmac_flow_ctrl
    }
    #[doc = "0x1c - VLAN Tag Register"]
    #[inline(always)]
    pub const fn gmac_vlan_tag(&self) -> &GmacVlanTag {
        &self.gmac_vlan_tag
    }
    #[doc = "0x24 - Debug register"]
    #[inline(always)]
    pub const fn gmac_debug(&self) -> &GmacDebug {
        &self.gmac_debug
    }
    #[doc = "0x2c - PMT Control and Status Register"]
    #[inline(always)]
    pub const fn gmac_pmt_ctrl_sta(&self) -> &GmacPmtCtrlSta {
        &self.gmac_pmt_ctrl_sta
    }
    #[doc = "0x38 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn gmac_int_status(&self) -> &GmacIntStatus {
        &self.gmac_int_status
    }
    #[doc = "0x3c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn gmac_int_mask(&self) -> &GmacIntMask {
        &self.gmac_int_mask
    }
    #[doc = "0x40 - MAC Address0 High Register"]
    #[inline(always)]
    pub const fn gmac_mac_addr0_hi(&self) -> &GmacMacAddr0Hi {
        &self.gmac_mac_addr0_hi
    }
    #[doc = "0x44 - MAC Address0 Low Register"]
    #[inline(always)]
    pub const fn gmac_mac_addr0_lo(&self) -> &GmacMacAddr0Lo {
        &self.gmac_mac_addr0_lo
    }
    #[doc = "0xc0 - AN Control Register"]
    #[inline(always)]
    pub const fn gmac_an_ctrl(&self) -> &GmacAnCtrl {
        &self.gmac_an_ctrl
    }
    #[doc = "0xc4 - AN Status Register"]
    #[inline(always)]
    pub const fn gmac_an_status(&self) -> &GmacAnStatus {
        &self.gmac_an_status
    }
    #[doc = "0xc8 - Auto Negotiation Advertisement Register"]
    #[inline(always)]
    pub const fn gmac_an_adv(&self) -> &GmacAnAdv {
        &self.gmac_an_adv
    }
    #[doc = "0xcc - Auto Negotiation Link Partner Ability Register"]
    #[inline(always)]
    pub const fn gmac_an_link_part_ab(&self) -> &GmacAnLinkPartAb {
        &self.gmac_an_link_part_ab
    }
    #[doc = "0xd0 - Auto Negotiation Expansion Register"]
    #[inline(always)]
    pub const fn gmac_an_exp(&self) -> &GmacAnExp {
        &self.gmac_an_exp
    }
    #[doc = "0xd8 - RGMII Status Register"]
    #[inline(always)]
    pub const fn gmac_intf_mode_sta(&self) -> &GmacIntfModeSta {
        &self.gmac_intf_mode_sta
    }
    #[doc = "0x100 - MMC Control Register"]
    #[inline(always)]
    pub const fn gmac_mmc_ctrl(&self) -> &GmacMmcCtrl {
        &self.gmac_mmc_ctrl
    }
    #[doc = "0x104 - MMC Receive Interrupt Register"]
    #[inline(always)]
    pub const fn gmac_mmc_rx_intr(&self) -> &GmacMmcRxIntr {
        &self.gmac_mmc_rx_intr
    }
    #[doc = "0x108 - MMC Transmit Interrupt Register"]
    #[inline(always)]
    pub const fn gmac_mmc_tx_intr(&self) -> &GmacMmcTxIntr {
        &self.gmac_mmc_tx_intr
    }
    #[doc = "0x10c - MMC Receive Interrupt Mask Register"]
    #[inline(always)]
    pub const fn gmac_mmc_rx_int_msk(&self) -> &GmacMmcRxIntMsk {
        &self.gmac_mmc_rx_int_msk
    }
    #[doc = "0x110 - MMC Transmit Interrupt Mask Register"]
    #[inline(always)]
    pub const fn gmac_mmc_tx_int_msk(&self) -> &GmacMmcTxIntMsk {
        &self.gmac_mmc_tx_int_msk
    }
    #[doc = "0x114 - MMC TX OCTET Good and Bad Counter"]
    #[inline(always)]
    pub const fn gmac_mmc_txoctetcnt_gb(&self) -> &GmacMmcTxoctetcntGb {
        &self.gmac_mmc_txoctetcnt_gb
    }
    #[doc = "0x118 - MMC TX Frame Good and Bad Counter"]
    #[inline(always)]
    pub const fn gmac_mmc_txfrmcnt_gb(&self) -> &GmacMmcTxfrmcntGb {
        &self.gmac_mmc_txfrmcnt_gb
    }
    #[doc = "0x148 - MMC TX Underflow Error"]
    #[inline(always)]
    pub const fn gmac_mmc_txundflwerr(&self) -> &GmacMmcTxundflwerr {
        &self.gmac_mmc_txundflwerr
    }
    #[doc = "0x160 - MMC TX Carrier Error"]
    #[inline(always)]
    pub const fn gmac_mmc_txcarerr(&self) -> &GmacMmcTxcarerr {
        &self.gmac_mmc_txcarerr
    }
    #[doc = "0x164 - MMC TX OCTET Good Counter"]
    #[inline(always)]
    pub const fn gmac_mmc_txoctetcnt_g(&self) -> &GmacMmcTxoctetcntG {
        &self.gmac_mmc_txoctetcnt_g
    }
    #[doc = "0x168 - MMC TX Frame Good Counter"]
    #[inline(always)]
    pub const fn gmac_mmc_txfrmcnt_g(&self) -> &GmacMmcTxfrmcntG {
        &self.gmac_mmc_txfrmcnt_g
    }
    #[doc = "0x180 - MMC RX Frame Good and Bad Counter"]
    #[inline(always)]
    pub const fn gmac_mmc_rxfrmcnt_gb(&self) -> &GmacMmcRxfrmcntGb {
        &self.gmac_mmc_rxfrmcnt_gb
    }
    #[doc = "0x184 - MMC RX OCTET Good and Bad Counter"]
    #[inline(always)]
    pub const fn gmac_mmc_rxoctetcnt_gb(&self) -> &GmacMmcRxoctetcntGb {
        &self.gmac_mmc_rxoctetcnt_gb
    }
    #[doc = "0x188 - MMC RX OCTET Good Counter"]
    #[inline(always)]
    pub const fn gmac_mmc_rxoctetcnt_g(&self) -> &GmacMmcRxoctetcntG {
        &self.gmac_mmc_rxoctetcnt_g
    }
    #[doc = "0x190 - MMC RX Multicast Frame Good Counter"]
    #[inline(always)]
    pub const fn gmac_mmc_rxmcfrmcnt_g(&self) -> &GmacMmcRxmcfrmcntG {
        &self.gmac_mmc_rxmcfrmcnt_g
    }
    #[doc = "0x194 - MMC RX Carrier"]
    #[inline(always)]
    pub const fn gmac_mmc_rxcrcerr(&self) -> &GmacMmcRxcrcerr {
        &self.gmac_mmc_rxcrcerr
    }
    #[doc = "0x1c8 - MMC RX Length Error"]
    #[inline(always)]
    pub const fn gmac_mmc_rxlenerr(&self) -> &GmacMmcRxlenerr {
        &self.gmac_mmc_rxlenerr
    }
    #[doc = "0x1d4 - MMC RX FIFO Overflow"]
    #[inline(always)]
    pub const fn gmac_mmc_rxfifoovrflw(&self) -> &GmacMmcRxfifoovrflw {
        &self.gmac_mmc_rxfifoovrflw
    }
    #[doc = "0x200 - MMC Receive Checksum Offload Interrupt Mask Register"]
    #[inline(always)]
    pub const fn gmac_mmc_ipc_int_msk(&self) -> &GmacMmcIpcIntMsk {
        &self.gmac_mmc_ipc_int_msk
    }
    #[doc = "0x208 - MMC Receive Checksum Offload Interrupt Register"]
    #[inline(always)]
    pub const fn gmac_mmc_ipc_intr(&self) -> &GmacMmcIpcIntr {
        &self.gmac_mmc_ipc_intr
    }
    #[doc = "0x210 - MMC RX IPV4 Good Frame"]
    #[inline(always)]
    pub const fn gmac_mmc_rxipv4gfrm(&self) -> &GmacMmcRxipv4gfrm {
        &self.gmac_mmc_rxipv4gfrm
    }
    #[doc = "0x214 - MMC RX IPV4 Head Error Frame"]
    #[inline(always)]
    pub const fn gmac_mmc_rxipv4hderrfrm(&self) -> &GmacMmcRxipv4hderrfrm {
        &self.gmac_mmc_rxipv4hderrfrm
    }
    #[doc = "0x224 - MMC RX IPV6 Good Frame"]
    #[inline(always)]
    pub const fn gmac_mmc_rxipv6gfrm(&self) -> &GmacMmcRxipv6gfrm {
        &self.gmac_mmc_rxipv6gfrm
    }
    #[doc = "0x228 - MMC RX IPV6 Head Error Frame"]
    #[inline(always)]
    pub const fn gmac_mmc_rxipv6hderrfrm(&self) -> &GmacMmcRxipv6hderrfrm {
        &self.gmac_mmc_rxipv6hderrfrm
    }
    #[doc = "0x234 - MMC RX UDP Error Frame"]
    #[inline(always)]
    pub const fn gmac_mmc_rxudperrfrm(&self) -> &GmacMmcRxudperrfrm {
        &self.gmac_mmc_rxudperrfrm
    }
    #[doc = "0x23c - MMC RX TCP Error Frame"]
    #[inline(always)]
    pub const fn gmac_mmc_rxtcperrfrm(&self) -> &GmacMmcRxtcperrfrm {
        &self.gmac_mmc_rxtcperrfrm
    }
    #[doc = "0x244 - MMC RX ICMP Error Frame"]
    #[inline(always)]
    pub const fn gmac_mmc_rxicmperrfrm(&self) -> &GmacMmcRxicmperrfrm {
        &self.gmac_mmc_rxicmperrfrm
    }
    #[doc = "0x254 - MMC RX OCTET IPV4 Head Error"]
    #[inline(always)]
    pub const fn gmac_mmc_rxipv4hderroct(&self) -> &GmacMmcRxipv4hderroct {
        &self.gmac_mmc_rxipv4hderroct
    }
    #[doc = "0x268 - MMC RX OCTET IPV6 Head Error"]
    #[inline(always)]
    pub const fn gmac_mmc_rxipv6hderroct(&self) -> &GmacMmcRxipv6hderroct {
        &self.gmac_mmc_rxipv6hderroct
    }
    #[doc = "0x274 - MMC RX OCTET UDP Error"]
    #[inline(always)]
    pub const fn gmac_mmc_rxudperroct(&self) -> &GmacMmcRxudperroct {
        &self.gmac_mmc_rxudperroct
    }
    #[doc = "0x27c - MMC RX OCTET TCP Error"]
    #[inline(always)]
    pub const fn gmac_mmc_rxtcperroct(&self) -> &GmacMmcRxtcperroct {
        &self.gmac_mmc_rxtcperroct
    }
    #[doc = "0x284 - MMC RX OCTET ICMP Error"]
    #[inline(always)]
    pub const fn gmac_mmc_rxicmperroct(&self) -> &GmacMmcRxicmperroct {
        &self.gmac_mmc_rxicmperroct
    }
    #[doc = "0x1000 - Bus Mode Register"]
    #[inline(always)]
    pub const fn gmac_bus_mode(&self) -> &GmacBusMode {
        &self.gmac_bus_mode
    }
    #[doc = "0x1004 - Transmit Poll Demand Register"]
    #[inline(always)]
    pub const fn gmac_tx_poll_demand(&self) -> &GmacTxPollDemand {
        &self.gmac_tx_poll_demand
    }
    #[doc = "0x1008 - Receive Poll Demand Register"]
    #[inline(always)]
    pub const fn gmac_rx_poll_demand(&self) -> &GmacRxPollDemand {
        &self.gmac_rx_poll_demand
    }
    #[doc = "0x100c - Receive Descriptor List Address Register"]
    #[inline(always)]
    pub const fn gmac_rx_desc_list_addr(&self) -> &GmacRxDescListAddr {
        &self.gmac_rx_desc_list_addr
    }
    #[doc = "0x1010 - Transmit Descriptor List Address Register"]
    #[inline(always)]
    pub const fn gmac_tx_desc_list_addr(&self) -> &GmacTxDescListAddr {
        &self.gmac_tx_desc_list_addr
    }
    #[doc = "0x1014 - Status Register"]
    #[inline(always)]
    pub const fn gmac_status(&self) -> &GmacStatus {
        &self.gmac_status
    }
    #[doc = "0x1018 - Operation Mode Register"]
    #[inline(always)]
    pub const fn gmac_op_mode(&self) -> &GmacOpMode {
        &self.gmac_op_mode
    }
    #[doc = "0x101c - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn gmac_int_ena(&self) -> &GmacIntEna {
        &self.gmac_int_ena
    }
    #[doc = "0x1020 - Missed Frame and Buffer Overflow Counter Register"]
    #[inline(always)]
    pub const fn gmac_overflow_cnt(&self) -> &GmacOverflowCnt {
        &self.gmac_overflow_cnt
    }
    #[doc = "0x1024 - Receive Interrupt Watchdog Timer Register"]
    #[inline(always)]
    pub const fn gmac_rec_int_wdt_timer(&self) -> &GmacRecIntWdtTimer {
        &self.gmac_rec_int_wdt_timer
    }
    #[doc = "0x1028 - AXI Bus Mode Register"]
    #[inline(always)]
    pub const fn gmac_axi_bus_mode(&self) -> &GmacAxiBusMode {
        &self.gmac_axi_bus_mode
    }
    #[doc = "0x102c - AXI Status Register"]
    #[inline(always)]
    pub const fn gmac_axi_status(&self) -> &GmacAxiStatus {
        &self.gmac_axi_status
    }
    #[doc = "0x1048 - Current Host Transmit Descriptor Register"]
    #[inline(always)]
    pub const fn gmac_cur_host_tx_desc(&self) -> &GmacCurHostTxDesc {
        &self.gmac_cur_host_tx_desc
    }
    #[doc = "0x104c - Current Host Receive Descriptor Register"]
    #[inline(always)]
    pub const fn gmac_cur_host_rx_desc(&self) -> &GmacCurHostRxDesc {
        &self.gmac_cur_host_rx_desc
    }
    #[doc = "0x1050 - Current Host Transmit Buffer Address Register"]
    #[inline(always)]
    pub const fn gmac_cur_host_tx_buf_addr(&self) -> &GmacCurHostTxBufAddr {
        &self.gmac_cur_host_tx_buf_addr
    }
    #[doc = "0x1054 - Current Host Receive Buffer Address Register"]
    #[inline(always)]
    pub const fn gmac_cur_host_rx_buf_addr(&self) -> &GmacCurHostRxBufAddr {
        &self.gmac_cur_host_rx_buf_addr
    }
}
#[doc = "GMAC_MAC_CONF (rw) register accessor: MAC Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mac_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mac_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mac_conf`]
module"]
#[doc(alias = "GMAC_MAC_CONF")]
pub type GmacMacConf = crate::Reg<gmac_mac_conf::GmacMacConfSpec>;
#[doc = "MAC Configuration Register"]
pub mod gmac_mac_conf;
#[doc = "GMAC_MAC_FRM_FILT (rw) register accessor: MAC Frame Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mac_frm_filt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mac_frm_filt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mac_frm_filt`]
module"]
#[doc(alias = "GMAC_MAC_FRM_FILT")]
pub type GmacMacFrmFilt = crate::Reg<gmac_mac_frm_filt::GmacMacFrmFiltSpec>;
#[doc = "MAC Frame Filter"]
pub mod gmac_mac_frm_filt;
#[doc = "GMAC_HASH_TAB_HI (rw) register accessor: Hash Table High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_hash_tab_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_hash_tab_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_hash_tab_hi`]
module"]
#[doc(alias = "GMAC_HASH_TAB_HI")]
pub type GmacHashTabHi = crate::Reg<gmac_hash_tab_hi::GmacHashTabHiSpec>;
#[doc = "Hash Table High Register"]
pub mod gmac_hash_tab_hi;
#[doc = "GMAC_HASH_TAB_LO (rw) register accessor: Hash Table Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_hash_tab_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_hash_tab_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_hash_tab_lo`]
module"]
#[doc(alias = "GMAC_HASH_TAB_LO")]
pub type GmacHashTabLo = crate::Reg<gmac_hash_tab_lo::GmacHashTabLoSpec>;
#[doc = "Hash Table Low Register"]
pub mod gmac_hash_tab_lo;
#[doc = "GMAC_GMII_ADDR (rw) register accessor: GMII Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_gmii_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_gmii_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_gmii_addr`]
module"]
#[doc(alias = "GMAC_GMII_ADDR")]
pub type GmacGmiiAddr = crate::Reg<gmac_gmii_addr::GmacGmiiAddrSpec>;
#[doc = "GMII Address Register"]
pub mod gmac_gmii_addr;
#[doc = "GMAC_GMII_DATA (rw) register accessor: GMII Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_gmii_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_gmii_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_gmii_data`]
module"]
#[doc(alias = "GMAC_GMII_DATA")]
pub type GmacGmiiData = crate::Reg<gmac_gmii_data::GmacGmiiDataSpec>;
#[doc = "GMII Data Register"]
pub mod gmac_gmii_data;
#[doc = "GMAC_FLOW_CTRL (rw) register accessor: Flow Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_flow_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_flow_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_flow_ctrl`]
module"]
#[doc(alias = "GMAC_FLOW_CTRL")]
pub type GmacFlowCtrl = crate::Reg<gmac_flow_ctrl::GmacFlowCtrlSpec>;
#[doc = "Flow Control Register"]
pub mod gmac_flow_ctrl;
#[doc = "GMAC_VLAN_TAG (rw) register accessor: VLAN Tag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_vlan_tag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_vlan_tag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_vlan_tag`]
module"]
#[doc(alias = "GMAC_VLAN_TAG")]
pub type GmacVlanTag = crate::Reg<gmac_vlan_tag::GmacVlanTagSpec>;
#[doc = "VLAN Tag Register"]
pub mod gmac_vlan_tag;
#[doc = "GMAC_DEBUG (rw) register accessor: Debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_debug::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_debug::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_debug`]
module"]
#[doc(alias = "GMAC_DEBUG")]
pub type GmacDebug = crate::Reg<gmac_debug::GmacDebugSpec>;
#[doc = "Debug register"]
pub mod gmac_debug;
#[doc = "GMAC_PMT_CTRL_STA (rw) register accessor: PMT Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_pmt_ctrl_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_pmt_ctrl_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_pmt_ctrl_sta`]
module"]
#[doc(alias = "GMAC_PMT_CTRL_STA")]
pub type GmacPmtCtrlSta = crate::Reg<gmac_pmt_ctrl_sta::GmacPmtCtrlStaSpec>;
#[doc = "PMT Control and Status Register"]
pub mod gmac_pmt_ctrl_sta;
#[doc = "GMAC_INT_STATUS (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_int_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_int_status`]
module"]
#[doc(alias = "GMAC_INT_STATUS")]
pub type GmacIntStatus = crate::Reg<gmac_int_status::GmacIntStatusSpec>;
#[doc = "Interrupt Status Register"]
pub mod gmac_int_status;
#[doc = "GMAC_INT_MASK (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_int_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_int_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_int_mask`]
module"]
#[doc(alias = "GMAC_INT_MASK")]
pub type GmacIntMask = crate::Reg<gmac_int_mask::GmacIntMaskSpec>;
#[doc = "Interrupt Mask Register"]
pub mod gmac_int_mask;
#[doc = "GMAC_MAC_ADDR0_HI (rw) register accessor: MAC Address0 High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mac_addr0_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mac_addr0_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mac_addr0_hi`]
module"]
#[doc(alias = "GMAC_MAC_ADDR0_HI")]
pub type GmacMacAddr0Hi = crate::Reg<gmac_mac_addr0_hi::GmacMacAddr0HiSpec>;
#[doc = "MAC Address0 High Register"]
pub mod gmac_mac_addr0_hi;
#[doc = "GMAC_MAC_ADDR0_LO (rw) register accessor: MAC Address0 Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mac_addr0_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mac_addr0_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mac_addr0_lo`]
module"]
#[doc(alias = "GMAC_MAC_ADDR0_LO")]
pub type GmacMacAddr0Lo = crate::Reg<gmac_mac_addr0_lo::GmacMacAddr0LoSpec>;
#[doc = "MAC Address0 Low Register"]
pub mod gmac_mac_addr0_lo;
#[doc = "GMAC_AN_CTRL (rw) register accessor: AN Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_an_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_an_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_an_ctrl`]
module"]
#[doc(alias = "GMAC_AN_CTRL")]
pub type GmacAnCtrl = crate::Reg<gmac_an_ctrl::GmacAnCtrlSpec>;
#[doc = "AN Control Register"]
pub mod gmac_an_ctrl;
#[doc = "GMAC_AN_STATUS (rw) register accessor: AN Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_an_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_an_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_an_status`]
module"]
#[doc(alias = "GMAC_AN_STATUS")]
pub type GmacAnStatus = crate::Reg<gmac_an_status::GmacAnStatusSpec>;
#[doc = "AN Status Register"]
pub mod gmac_an_status;
#[doc = "GMAC_AN_ADV (rw) register accessor: Auto Negotiation Advertisement Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_an_adv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_an_adv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_an_adv`]
module"]
#[doc(alias = "GMAC_AN_ADV")]
pub type GmacAnAdv = crate::Reg<gmac_an_adv::GmacAnAdvSpec>;
#[doc = "Auto Negotiation Advertisement Register"]
pub mod gmac_an_adv;
#[doc = "GMAC_AN_LINK_PART_AB (r) register accessor: Auto Negotiation Link Partner Ability Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_an_link_part_ab::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_an_link_part_ab`]
module"]
#[doc(alias = "GMAC_AN_LINK_PART_AB")]
pub type GmacAnLinkPartAb = crate::Reg<gmac_an_link_part_ab::GmacAnLinkPartAbSpec>;
#[doc = "Auto Negotiation Link Partner Ability Register"]
pub mod gmac_an_link_part_ab;
#[doc = "GMAC_AN_EXP (r) register accessor: Auto Negotiation Expansion Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_an_exp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_an_exp`]
module"]
#[doc(alias = "GMAC_AN_EXP")]
pub type GmacAnExp = crate::Reg<gmac_an_exp::GmacAnExpSpec>;
#[doc = "Auto Negotiation Expansion Register"]
pub mod gmac_an_exp;
#[doc = "GMAC_INTF_MODE_STA (rw) register accessor: RGMII Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_intf_mode_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_intf_mode_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_intf_mode_sta`]
module"]
#[doc(alias = "GMAC_INTF_MODE_STA")]
pub type GmacIntfModeSta = crate::Reg<gmac_intf_mode_sta::GmacIntfModeStaSpec>;
#[doc = "RGMII Status Register"]
pub mod gmac_intf_mode_sta;
#[doc = "GMAC_MMC_CTRL (rw) register accessor: MMC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_ctrl`]
module"]
#[doc(alias = "GMAC_MMC_CTRL")]
pub type GmacMmcCtrl = crate::Reg<gmac_mmc_ctrl::GmacMmcCtrlSpec>;
#[doc = "MMC Control Register"]
pub mod gmac_mmc_ctrl;
#[doc = "GMAC_MMC_RX_INTR (rw) register accessor: MMC Receive Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rx_intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rx_intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rx_intr`]
module"]
#[doc(alias = "GMAC_MMC_RX_INTR")]
pub type GmacMmcRxIntr = crate::Reg<gmac_mmc_rx_intr::GmacMmcRxIntrSpec>;
#[doc = "MMC Receive Interrupt Register"]
pub mod gmac_mmc_rx_intr;
#[doc = "GMAC_MMC_TX_INTR (r) register accessor: MMC Transmit Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_tx_intr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_tx_intr`]
module"]
#[doc(alias = "GMAC_MMC_TX_INTR")]
pub type GmacMmcTxIntr = crate::Reg<gmac_mmc_tx_intr::GmacMmcTxIntrSpec>;
#[doc = "MMC Transmit Interrupt Register"]
pub mod gmac_mmc_tx_intr;
#[doc = "GMAC_MMC_RX_INT_MSK (rw) register accessor: MMC Receive Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rx_int_msk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rx_int_msk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rx_int_msk`]
module"]
#[doc(alias = "GMAC_MMC_RX_INT_MSK")]
pub type GmacMmcRxIntMsk = crate::Reg<gmac_mmc_rx_int_msk::GmacMmcRxIntMskSpec>;
#[doc = "MMC Receive Interrupt Mask Register"]
pub mod gmac_mmc_rx_int_msk;
#[doc = "GMAC_MMC_TX_INT_MSK (rw) register accessor: MMC Transmit Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_tx_int_msk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_tx_int_msk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_tx_int_msk`]
module"]
#[doc(alias = "GMAC_MMC_TX_INT_MSK")]
pub type GmacMmcTxIntMsk = crate::Reg<gmac_mmc_tx_int_msk::GmacMmcTxIntMskSpec>;
#[doc = "MMC Transmit Interrupt Mask Register"]
pub mod gmac_mmc_tx_int_msk;
#[doc = "GMAC_MMC_TXOCTETCNT_GB (rw) register accessor: MMC TX OCTET Good and Bad Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_txoctetcnt_gb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_txoctetcnt_gb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_txoctetcnt_gb`]
module"]
#[doc(alias = "GMAC_MMC_TXOCTETCNT_GB")]
pub type GmacMmcTxoctetcntGb = crate::Reg<gmac_mmc_txoctetcnt_gb::GmacMmcTxoctetcntGbSpec>;
#[doc = "MMC TX OCTET Good and Bad Counter"]
pub mod gmac_mmc_txoctetcnt_gb;
#[doc = "GMAC_MMC_TXFRMCNT_GB (rw) register accessor: MMC TX Frame Good and Bad Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_txfrmcnt_gb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_txfrmcnt_gb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_txfrmcnt_gb`]
module"]
#[doc(alias = "GMAC_MMC_TXFRMCNT_GB")]
pub type GmacMmcTxfrmcntGb = crate::Reg<gmac_mmc_txfrmcnt_gb::GmacMmcTxfrmcntGbSpec>;
#[doc = "MMC TX Frame Good and Bad Counter"]
pub mod gmac_mmc_txfrmcnt_gb;
#[doc = "GMAC_MMC_TXUNDFLWERR (rw) register accessor: MMC TX Underflow Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_txundflwerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_txundflwerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_txundflwerr`]
module"]
#[doc(alias = "GMAC_MMC_TXUNDFLWERR")]
pub type GmacMmcTxundflwerr = crate::Reg<gmac_mmc_txundflwerr::GmacMmcTxundflwerrSpec>;
#[doc = "MMC TX Underflow Error"]
pub mod gmac_mmc_txundflwerr;
#[doc = "GMAC_MMC_TXCARERR (rw) register accessor: MMC TX Carrier Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_txcarerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_txcarerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_txcarerr`]
module"]
#[doc(alias = "GMAC_MMC_TXCARERR")]
pub type GmacMmcTxcarerr = crate::Reg<gmac_mmc_txcarerr::GmacMmcTxcarerrSpec>;
#[doc = "MMC TX Carrier Error"]
pub mod gmac_mmc_txcarerr;
#[doc = "GMAC_MMC_TXOCTETCNT_G (rw) register accessor: MMC TX OCTET Good Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_txoctetcnt_g::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_txoctetcnt_g::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_txoctetcnt_g`]
module"]
#[doc(alias = "GMAC_MMC_TXOCTETCNT_G")]
pub type GmacMmcTxoctetcntG = crate::Reg<gmac_mmc_txoctetcnt_g::GmacMmcTxoctetcntGSpec>;
#[doc = "MMC TX OCTET Good Counter"]
pub mod gmac_mmc_txoctetcnt_g;
#[doc = "GMAC_MMC_TXFRMCNT_G (rw) register accessor: MMC TX Frame Good Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_txfrmcnt_g::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_txfrmcnt_g::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_txfrmcnt_g`]
module"]
#[doc(alias = "GMAC_MMC_TXFRMCNT_G")]
pub type GmacMmcTxfrmcntG = crate::Reg<gmac_mmc_txfrmcnt_g::GmacMmcTxfrmcntGSpec>;
#[doc = "MMC TX Frame Good Counter"]
pub mod gmac_mmc_txfrmcnt_g;
#[doc = "GMAC_MMC_RXFRMCNT_GB (rw) register accessor: MMC RX Frame Good and Bad Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxfrmcnt_gb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxfrmcnt_gb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxfrmcnt_gb`]
module"]
#[doc(alias = "GMAC_MMC_RXFRMCNT_GB")]
pub type GmacMmcRxfrmcntGb = crate::Reg<gmac_mmc_rxfrmcnt_gb::GmacMmcRxfrmcntGbSpec>;
#[doc = "MMC RX Frame Good and Bad Counter"]
pub mod gmac_mmc_rxfrmcnt_gb;
#[doc = "GMAC_MMC_RXOCTETCNT_GB (rw) register accessor: MMC RX OCTET Good and Bad Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxoctetcnt_gb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxoctetcnt_gb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxoctetcnt_gb`]
module"]
#[doc(alias = "GMAC_MMC_RXOCTETCNT_GB")]
pub type GmacMmcRxoctetcntGb = crate::Reg<gmac_mmc_rxoctetcnt_gb::GmacMmcRxoctetcntGbSpec>;
#[doc = "MMC RX OCTET Good and Bad Counter"]
pub mod gmac_mmc_rxoctetcnt_gb;
#[doc = "GMAC_MMC_RXOCTETCNT_G (rw) register accessor: MMC RX OCTET Good Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxoctetcnt_g::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxoctetcnt_g::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxoctetcnt_g`]
module"]
#[doc(alias = "GMAC_MMC_RXOCTETCNT_G")]
pub type GmacMmcRxoctetcntG = crate::Reg<gmac_mmc_rxoctetcnt_g::GmacMmcRxoctetcntGSpec>;
#[doc = "MMC RX OCTET Good Counter"]
pub mod gmac_mmc_rxoctetcnt_g;
#[doc = "GMAC_MMC_RXMCFRMCNT_G (rw) register accessor: MMC RX Multicast Frame Good Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxmcfrmcnt_g::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxmcfrmcnt_g::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxmcfrmcnt_g`]
module"]
#[doc(alias = "GMAC_MMC_RXMCFRMCNT_G")]
pub type GmacMmcRxmcfrmcntG = crate::Reg<gmac_mmc_rxmcfrmcnt_g::GmacMmcRxmcfrmcntGSpec>;
#[doc = "MMC RX Multicast Frame Good Counter"]
pub mod gmac_mmc_rxmcfrmcnt_g;
#[doc = "GMAC_MMC_RXCRCERR (rw) register accessor: MMC RX Carrier\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxcrcerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxcrcerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxcrcerr`]
module"]
#[doc(alias = "GMAC_MMC_RXCRCERR")]
pub type GmacMmcRxcrcerr = crate::Reg<gmac_mmc_rxcrcerr::GmacMmcRxcrcerrSpec>;
#[doc = "MMC RX Carrier"]
pub mod gmac_mmc_rxcrcerr;
#[doc = "GMAC_MMC_RXLENERR (rw) register accessor: MMC RX Length Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxlenerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxlenerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxlenerr`]
module"]
#[doc(alias = "GMAC_MMC_RXLENERR")]
pub type GmacMmcRxlenerr = crate::Reg<gmac_mmc_rxlenerr::GmacMmcRxlenerrSpec>;
#[doc = "MMC RX Length Error"]
pub mod gmac_mmc_rxlenerr;
#[doc = "GMAC_MMC_RXFIFOOVRFLW (rw) register accessor: MMC RX FIFO Overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxfifoovrflw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxfifoovrflw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxfifoovrflw`]
module"]
#[doc(alias = "GMAC_MMC_RXFIFOOVRFLW")]
pub type GmacMmcRxfifoovrflw = crate::Reg<gmac_mmc_rxfifoovrflw::GmacMmcRxfifoovrflwSpec>;
#[doc = "MMC RX FIFO Overflow"]
pub mod gmac_mmc_rxfifoovrflw;
#[doc = "GMAC_MMC_IPC_INT_MSK (rw) register accessor: MMC Receive Checksum Offload Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_ipc_int_msk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_ipc_int_msk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_ipc_int_msk`]
module"]
#[doc(alias = "GMAC_MMC_IPC_INT_MSK")]
pub type GmacMmcIpcIntMsk = crate::Reg<gmac_mmc_ipc_int_msk::GmacMmcIpcIntMskSpec>;
#[doc = "MMC Receive Checksum Offload Interrupt Mask Register"]
pub mod gmac_mmc_ipc_int_msk;
#[doc = "GMAC_MMC_IPC_INTR (r) register accessor: MMC Receive Checksum Offload Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_ipc_intr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_ipc_intr`]
module"]
#[doc(alias = "GMAC_MMC_IPC_INTR")]
pub type GmacMmcIpcIntr = crate::Reg<gmac_mmc_ipc_intr::GmacMmcIpcIntrSpec>;
#[doc = "MMC Receive Checksum Offload Interrupt Register"]
pub mod gmac_mmc_ipc_intr;
#[doc = "GMAC_MMC_RXIPV4GFRM (rw) register accessor: MMC RX IPV4 Good Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxipv4gfrm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxipv4gfrm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxipv4gfrm`]
module"]
#[doc(alias = "GMAC_MMC_RXIPV4GFRM")]
pub type GmacMmcRxipv4gfrm = crate::Reg<gmac_mmc_rxipv4gfrm::GmacMmcRxipv4gfrmSpec>;
#[doc = "MMC RX IPV4 Good Frame"]
pub mod gmac_mmc_rxipv4gfrm;
#[doc = "GMAC_MMC_RXIPV4HDERRFRM (rw) register accessor: MMC RX IPV4 Head Error Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxipv4hderrfrm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxipv4hderrfrm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxipv4hderrfrm`]
module"]
#[doc(alias = "GMAC_MMC_RXIPV4HDERRFRM")]
pub type GmacMmcRxipv4hderrfrm = crate::Reg<gmac_mmc_rxipv4hderrfrm::GmacMmcRxipv4hderrfrmSpec>;
#[doc = "MMC RX IPV4 Head Error Frame"]
pub mod gmac_mmc_rxipv4hderrfrm;
#[doc = "GMAC_MMC_RXIPV6GFRM (rw) register accessor: MMC RX IPV6 Good Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxipv6gfrm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxipv6gfrm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxipv6gfrm`]
module"]
#[doc(alias = "GMAC_MMC_RXIPV6GFRM")]
pub type GmacMmcRxipv6gfrm = crate::Reg<gmac_mmc_rxipv6gfrm::GmacMmcRxipv6gfrmSpec>;
#[doc = "MMC RX IPV6 Good Frame"]
pub mod gmac_mmc_rxipv6gfrm;
#[doc = "GMAC_MMC_RXIPV6HDERRFRM (rw) register accessor: MMC RX IPV6 Head Error Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxipv6hderrfrm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxipv6hderrfrm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxipv6hderrfrm`]
module"]
#[doc(alias = "GMAC_MMC_RXIPV6HDERRFRM")]
pub type GmacMmcRxipv6hderrfrm = crate::Reg<gmac_mmc_rxipv6hderrfrm::GmacMmcRxipv6hderrfrmSpec>;
#[doc = "MMC RX IPV6 Head Error Frame"]
pub mod gmac_mmc_rxipv6hderrfrm;
#[doc = "GMAC_MMC_RXUDPERRFRM (rw) register accessor: MMC RX UDP Error Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxudperrfrm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxudperrfrm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxudperrfrm`]
module"]
#[doc(alias = "GMAC_MMC_RXUDPERRFRM")]
pub type GmacMmcRxudperrfrm = crate::Reg<gmac_mmc_rxudperrfrm::GmacMmcRxudperrfrmSpec>;
#[doc = "MMC RX UDP Error Frame"]
pub mod gmac_mmc_rxudperrfrm;
#[doc = "GMAC_MMC_RXTCPERRFRM (rw) register accessor: MMC RX TCP Error Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxtcperrfrm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxtcperrfrm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxtcperrfrm`]
module"]
#[doc(alias = "GMAC_MMC_RXTCPERRFRM")]
pub type GmacMmcRxtcperrfrm = crate::Reg<gmac_mmc_rxtcperrfrm::GmacMmcRxtcperrfrmSpec>;
#[doc = "MMC RX TCP Error Frame"]
pub mod gmac_mmc_rxtcperrfrm;
#[doc = "GMAC_MMC_RXICMPERRFRM (rw) register accessor: MMC RX ICMP Error Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxicmperrfrm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxicmperrfrm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxicmperrfrm`]
module"]
#[doc(alias = "GMAC_MMC_RXICMPERRFRM")]
pub type GmacMmcRxicmperrfrm = crate::Reg<gmac_mmc_rxicmperrfrm::GmacMmcRxicmperrfrmSpec>;
#[doc = "MMC RX ICMP Error Frame"]
pub mod gmac_mmc_rxicmperrfrm;
#[doc = "GMAC_MMC_RXIPV4HDERROCT (rw) register accessor: MMC RX OCTET IPV4 Head Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxipv4hderroct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxipv4hderroct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxipv4hderroct`]
module"]
#[doc(alias = "GMAC_MMC_RXIPV4HDERROCT")]
pub type GmacMmcRxipv4hderroct = crate::Reg<gmac_mmc_rxipv4hderroct::GmacMmcRxipv4hderroctSpec>;
#[doc = "MMC RX OCTET IPV4 Head Error"]
pub mod gmac_mmc_rxipv4hderroct;
#[doc = "GMAC_MMC_RXIPV6HDERROCT (rw) register accessor: MMC RX OCTET IPV6 Head Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxipv6hderroct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxipv6hderroct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxipv6hderroct`]
module"]
#[doc(alias = "GMAC_MMC_RXIPV6HDERROCT")]
pub type GmacMmcRxipv6hderroct = crate::Reg<gmac_mmc_rxipv6hderroct::GmacMmcRxipv6hderroctSpec>;
#[doc = "MMC RX OCTET IPV6 Head Error"]
pub mod gmac_mmc_rxipv6hderroct;
#[doc = "GMAC_MMC_RXUDPERROCT (rw) register accessor: MMC RX OCTET UDP Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxudperroct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxudperroct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxudperroct`]
module"]
#[doc(alias = "GMAC_MMC_RXUDPERROCT")]
pub type GmacMmcRxudperroct = crate::Reg<gmac_mmc_rxudperroct::GmacMmcRxudperroctSpec>;
#[doc = "MMC RX OCTET UDP Error"]
pub mod gmac_mmc_rxudperroct;
#[doc = "GMAC_MMC_RXTCPERROCT (rw) register accessor: MMC RX OCTET TCP Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxtcperroct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxtcperroct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxtcperroct`]
module"]
#[doc(alias = "GMAC_MMC_RXTCPERROCT")]
pub type GmacMmcRxtcperroct = crate::Reg<gmac_mmc_rxtcperroct::GmacMmcRxtcperroctSpec>;
#[doc = "MMC RX OCTET TCP Error"]
pub mod gmac_mmc_rxtcperroct;
#[doc = "GMAC_MMC_RXICMPERROCT (rw) register accessor: MMC RX OCTET ICMP Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxicmperroct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxicmperroct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_mmc_rxicmperroct`]
module"]
#[doc(alias = "GMAC_MMC_RXICMPERROCT")]
pub type GmacMmcRxicmperroct = crate::Reg<gmac_mmc_rxicmperroct::GmacMmcRxicmperroctSpec>;
#[doc = "MMC RX OCTET ICMP Error"]
pub mod gmac_mmc_rxicmperroct;
#[doc = "GMAC_BUS_MODE (rw) register accessor: Bus Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_bus_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_bus_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_bus_mode`]
module"]
#[doc(alias = "GMAC_BUS_MODE")]
pub type GmacBusMode = crate::Reg<gmac_bus_mode::GmacBusModeSpec>;
#[doc = "Bus Mode Register"]
pub mod gmac_bus_mode;
#[doc = "GMAC_TX_POLL_DEMAND (r) register accessor: Transmit Poll Demand Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_tx_poll_demand::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_tx_poll_demand`]
module"]
#[doc(alias = "GMAC_TX_POLL_DEMAND")]
pub type GmacTxPollDemand = crate::Reg<gmac_tx_poll_demand::GmacTxPollDemandSpec>;
#[doc = "Transmit Poll Demand Register"]
pub mod gmac_tx_poll_demand;
#[doc = "GMAC_RX_POLL_DEMAND (r) register accessor: Receive Poll Demand Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_rx_poll_demand::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_rx_poll_demand`]
module"]
#[doc(alias = "GMAC_RX_POLL_DEMAND")]
pub type GmacRxPollDemand = crate::Reg<gmac_rx_poll_demand::GmacRxPollDemandSpec>;
#[doc = "Receive Poll Demand Register"]
pub mod gmac_rx_poll_demand;
#[doc = "GMAC_RX_DESC_LIST_ADDR (rw) register accessor: Receive Descriptor List Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_rx_desc_list_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_rx_desc_list_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_rx_desc_list_addr`]
module"]
#[doc(alias = "GMAC_RX_DESC_LIST_ADDR")]
pub type GmacRxDescListAddr = crate::Reg<gmac_rx_desc_list_addr::GmacRxDescListAddrSpec>;
#[doc = "Receive Descriptor List Address Register"]
pub mod gmac_rx_desc_list_addr;
#[doc = "GMAC_TX_DESC_LIST_ADDR (rw) register accessor: Transmit Descriptor List Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_tx_desc_list_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_tx_desc_list_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_tx_desc_list_addr`]
module"]
#[doc(alias = "GMAC_TX_DESC_LIST_ADDR")]
pub type GmacTxDescListAddr = crate::Reg<gmac_tx_desc_list_addr::GmacTxDescListAddrSpec>;
#[doc = "Transmit Descriptor List Address Register"]
pub mod gmac_tx_desc_list_addr;
#[doc = "GMAC_STATUS (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_status`]
module"]
#[doc(alias = "GMAC_STATUS")]
pub type GmacStatus = crate::Reg<gmac_status::GmacStatusSpec>;
#[doc = "Status Register"]
pub mod gmac_status;
#[doc = "GMAC_OP_MODE (rw) register accessor: Operation Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_op_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_op_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_op_mode`]
module"]
#[doc(alias = "GMAC_OP_MODE")]
pub type GmacOpMode = crate::Reg<gmac_op_mode::GmacOpModeSpec>;
#[doc = "Operation Mode Register"]
pub mod gmac_op_mode;
#[doc = "GMAC_INT_ENA (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_int_ena`]
module"]
#[doc(alias = "GMAC_INT_ENA")]
pub type GmacIntEna = crate::Reg<gmac_int_ena::GmacIntEnaSpec>;
#[doc = "Interrupt Enable Register"]
pub mod gmac_int_ena;
#[doc = "GMAC_OVERFLOW_CNT (r) register accessor: Missed Frame and Buffer Overflow Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_overflow_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_overflow_cnt`]
module"]
#[doc(alias = "GMAC_OVERFLOW_CNT")]
pub type GmacOverflowCnt = crate::Reg<gmac_overflow_cnt::GmacOverflowCntSpec>;
#[doc = "Missed Frame and Buffer Overflow Counter Register"]
pub mod gmac_overflow_cnt;
#[doc = "GMAC_REC_INT_WDT_TIMER (rw) register accessor: Receive Interrupt Watchdog Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_rec_int_wdt_timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_rec_int_wdt_timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_rec_int_wdt_timer`]
module"]
#[doc(alias = "GMAC_REC_INT_WDT_TIMER")]
pub type GmacRecIntWdtTimer = crate::Reg<gmac_rec_int_wdt_timer::GmacRecIntWdtTimerSpec>;
#[doc = "Receive Interrupt Watchdog Timer Register"]
pub mod gmac_rec_int_wdt_timer;
#[doc = "GMAC_AXI_BUS_MODE (rw) register accessor: AXI Bus Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_axi_bus_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_axi_bus_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_axi_bus_mode`]
module"]
#[doc(alias = "GMAC_AXI_BUS_MODE")]
pub type GmacAxiBusMode = crate::Reg<gmac_axi_bus_mode::GmacAxiBusModeSpec>;
#[doc = "AXI Bus Mode Register"]
pub mod gmac_axi_bus_mode;
#[doc = "GMAC_AXI_STATUS (r) register accessor: AXI Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_axi_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_axi_status`]
module"]
#[doc(alias = "GMAC_AXI_STATUS")]
pub type GmacAxiStatus = crate::Reg<gmac_axi_status::GmacAxiStatusSpec>;
#[doc = "AXI Status Register"]
pub mod gmac_axi_status;
#[doc = "GMAC_CUR_HOST_TX_DESC (r) register accessor: Current Host Transmit Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_cur_host_tx_desc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_cur_host_tx_desc`]
module"]
#[doc(alias = "GMAC_CUR_HOST_TX_DESC")]
pub type GmacCurHostTxDesc = crate::Reg<gmac_cur_host_tx_desc::GmacCurHostTxDescSpec>;
#[doc = "Current Host Transmit Descriptor Register"]
pub mod gmac_cur_host_tx_desc;
#[doc = "GMAC_CUR_HOST_RX_DESC (r) register accessor: Current Host Receive Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_cur_host_rx_desc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_cur_host_rx_desc`]
module"]
#[doc(alias = "GMAC_CUR_HOST_RX_DESC")]
pub type GmacCurHostRxDesc = crate::Reg<gmac_cur_host_rx_desc::GmacCurHostRxDescSpec>;
#[doc = "Current Host Receive Descriptor Register"]
pub mod gmac_cur_host_rx_desc;
#[doc = "GMAC_CUR_HOST_TX_BUF_ADDR (r) register accessor: Current Host Transmit Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_cur_host_tx_buf_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_cur_host_tx_buf_addr`]
module"]
#[doc(alias = "GMAC_CUR_HOST_TX_BUF_ADDR")]
pub type GmacCurHostTxBufAddr = crate::Reg<gmac_cur_host_tx_buf_addr::GmacCurHostTxBufAddrSpec>;
#[doc = "Current Host Transmit Buffer Address Register"]
pub mod gmac_cur_host_tx_buf_addr;
#[doc = "GMAC_CUR_HOST_RX_BUF_ADDR (r) register accessor: Current Host Receive Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_cur_host_rx_buf_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_cur_host_rx_buf_addr`]
module"]
#[doc(alias = "GMAC_CUR_HOST_RX_BUF_ADDR")]
pub type GmacCurHostRxBufAddr = crate::Reg<gmac_cur_host_rx_buf_addr::GmacCurHostRxBufAddrSpec>;
#[doc = "Current Host Receive Buffer Address Register"]
pub mod gmac_cur_host_rx_buf_addr;
