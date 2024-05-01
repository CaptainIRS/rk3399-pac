#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    config0: Config0,
    config1: Config1,
    status: Status,
    int: Int,
    frm_start: FrmStart,
    _reserved5: [u8; 0x04],
    config_done: ConfigDone,
    frm_cnt: FrmCnt,
    vir_img_width: VirImgWidth,
    _reserved8: [u8; 0x04],
    src_img_size: SrcImgSize,
    dst_img_size: DstImgSize,
    dst_img_width_tile0: DstImgWidthTile0,
    dst_img_width_tile1: DstImgWidthTile1,
    dst_img_width_tile2: DstImgWidthTile2,
    dst_img_width_tile3: DstImgWidthTile3,
    enh_yuv_cnfg_0: EnhYuvCnfg0,
    enh_yuv_cnfg_1: EnhYuvCnfg1,
    enh_yuv_cnfg_2: EnhYuvCnfg2,
    enh_rgb_cnfg: EnhRgbCnfg,
    enh_c_coe: EnhCCoe,
    version_info: VersionInfo,
    raw_config0: RawConfig0,
    raw_config1: RawConfig1,
    raw_vir_img_width: RawVirImgWidth,
    _reserved23: [u8; 0x04],
    raw_src_img_size: RawSrcImgSize,
    raw_dst_img_size: RawDstImgSize,
    raw_enh_yuv_cnfg_0: RawEnhYuvCnfg0,
    raw_enh_yuv_cnfg_1: RawEnhYuvCnfg1,
    raw_enh_yuv_cnfg_2: RawEnhYuvCnfg2,
    raw_enh_rgb_cnfg: RawEnhRgbCnfg,
    src_addr_yrgb: SrcAddrYrgb,
    src_addr_cbcr: SrcAddrCbcr,
    src_addr_cr: SrcAddrCr,
    src_addr_y1: SrcAddrY1,
    src_addr_cbcr1: SrcAddrCbcr1,
    src_addr_cr1: SrcAddrCr1,
    src_addr_y_itemp: SrcAddrYItemp,
    src_addr_cbcr_itemp: SrcAddrCbcrItemp,
    src_addr_cr_itemp: SrcAddrCrItemp,
    src_addr_y_ftemp: SrcAddrYFtemp,
    src_addr_cbcr_ftemp: SrcAddrCbcrFtemp,
    src_addr_cr_ftemp: SrcAddrCrFtemp,
    dst_addr_yrgb: DstAddrYrgb,
    dst_addr_cbcr: DstAddrCbcr,
    dst_addr_cr: DstAddrCr,
    dst_addr_y1: DstAddrY1,
    dst_addr_cbcr1: DstAddrCbcr1,
    dst_addr_cr1: DstAddrCr1,
    dst_addr_y_itemp: DstAddrYItemp,
    dst_addr_cbcr_itemp: DstAddrCbcrItemp,
    dst_addr_cr_itemp: DstAddrCrItemp,
    dst_addr_y_ftemp: DstAddrYFtemp,
    dst_addr_cbcr_ftemp: DstAddrCbcrFtemp,
    dst_addr_cr_ftemp: DstAddrCrFtemp,
    dil_mtn_tab0: DilMtnTab0,
    dil_mtn_tab1: DilMtnTab1,
    dil_mtn_tab2: DilMtnTab2,
    dil_mtn_tab3: DilMtnTab3,
    dil_mtn_tab4: DilMtnTab4,
    dil_mtn_tab5: DilMtnTab5,
    dil_mtn_tab6: DilMtnTab6,
    dil_mtn_tab7: DilMtnTab7,
    enh_cg_tab: EnhCgTab,
    _reserved62: [u8; 0x02fc],
    enh_dde_coe0: EnhDdeCoe0,
    _reserved63: [u8; 0xfc],
    enh_dde_coe1: EnhDdeCoe1,
    _reserved64: [u8; 0xfc],
    perf_latency_ctrl0: PerfLatencyCtrl0,
    perf_latency_ctrl1: PerfLatencyCtrl1,
    perf_rd_max_latency_num0: PerfRdMaxLatencyNum0,
    perf_rd_latency_samp_num: PerfRdLatencySampNum,
    perf_rd_latency_acc_sum: PerfRdLatencyAccSum,
    perf_wr_axi_total_byte: PerfWrAxiTotalByte,
    perf_working_cnt: PerfWorkingCnt,
    perf_rd_axi_total_byte: PerfRdAxiTotalByte,
}
impl RegisterBlock {
    #[doc = "0x00 - configuration register0"]
    #[inline(always)]
    pub const fn config0(&self) -> &Config0 {
        &self.config0
    }
    #[doc = "0x04 - configuration register1"]
    #[inline(always)]
    pub const fn config1(&self) -> &Config1 {
        &self.config1
    }
    #[doc = "0x08 - status register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x0c - interrupt register"]
    #[inline(always)]
    pub const fn int(&self) -> &Int {
        &self.int
    }
    #[doc = "0x10 - frame start"]
    #[inline(always)]
    pub const fn frm_start(&self) -> &FrmStart {
        &self.frm_start
    }
    #[doc = "0x18 - configuration done"]
    #[inline(always)]
    pub const fn config_done(&self) -> &ConfigDone {
        &self.config_done
    }
    #[doc = "0x1c - frame counter"]
    #[inline(always)]
    pub const fn frm_cnt(&self) -> &FrmCnt {
        &self.frm_cnt
    }
    #[doc = "0x20 - Image virtual width"]
    #[inline(always)]
    pub const fn vir_img_width(&self) -> &VirImgWidth {
        &self.vir_img_width
    }
    #[doc = "0x28 - Source image width/height"]
    #[inline(always)]
    pub const fn src_img_size(&self) -> &SrcImgSize {
        &self.src_img_size
    }
    #[doc = "0x2c - Destination image width/height"]
    #[inline(always)]
    pub const fn dst_img_size(&self) -> &DstImgSize {
        &self.dst_img_size
    }
    #[doc = "0x30 - Destination image tile0 width"]
    #[inline(always)]
    pub const fn dst_img_width_tile0(&self) -> &DstImgWidthTile0 {
        &self.dst_img_width_tile0
    }
    #[doc = "0x34 - Destination image tile1 width"]
    #[inline(always)]
    pub const fn dst_img_width_tile1(&self) -> &DstImgWidthTile1 {
        &self.dst_img_width_tile1
    }
    #[doc = "0x38 - Destination image tile2 width"]
    #[inline(always)]
    pub const fn dst_img_width_tile2(&self) -> &DstImgWidthTile2 {
        &self.dst_img_width_tile2
    }
    #[doc = "0x3c - Destination image tile3 width"]
    #[inline(always)]
    pub const fn dst_img_width_tile3(&self) -> &DstImgWidthTile3 {
        &self.dst_img_width_tile3
    }
    #[doc = "0x40 - brightness,contrast,saturation adjustment"]
    #[inline(always)]
    pub const fn enh_yuv_cnfg_0(&self) -> &EnhYuvCnfg0 {
        &self.enh_yuv_cnfg_0
    }
    #[doc = "0x44 - Hue configuration"]
    #[inline(always)]
    pub const fn enh_yuv_cnfg_1(&self) -> &EnhYuvCnfg1 {
        &self.enh_yuv_cnfg_1
    }
    #[doc = "0x48 - color bar configuration"]
    #[inline(always)]
    pub const fn enh_yuv_cnfg_2(&self) -> &EnhYuvCnfg2 {
        &self.enh_yuv_cnfg_2
    }
    #[doc = "0x4c - enhancement RGB configuration"]
    #[inline(always)]
    pub const fn enh_rgb_cnfg(&self) -> &EnhRgbCnfg {
        &self.enh_rgb_cnfg
    }
    #[doc = "0x50 - rgb color enhancement coefficient"]
    #[inline(always)]
    pub const fn enh_c_coe(&self) -> &EnhCCoe {
        &self.enh_c_coe
    }
    #[doc = "0x54 - Version number for iep"]
    #[inline(always)]
    pub const fn version_info(&self) -> &VersionInfo {
        &self.version_info
    }
    #[doc = "0x58 - configuration register0"]
    #[inline(always)]
    pub const fn raw_config0(&self) -> &RawConfig0 {
        &self.raw_config0
    }
    #[doc = "0x5c - configuration register1"]
    #[inline(always)]
    pub const fn raw_config1(&self) -> &RawConfig1 {
        &self.raw_config1
    }
    #[doc = "0x60 - Image virtual width"]
    #[inline(always)]
    pub const fn raw_vir_img_width(&self) -> &RawVirImgWidth {
        &self.raw_vir_img_width
    }
    #[doc = "0x68 - Source image width/height"]
    #[inline(always)]
    pub const fn raw_src_img_size(&self) -> &RawSrcImgSize {
        &self.raw_src_img_size
    }
    #[doc = "0x6c - Destination image width/height"]
    #[inline(always)]
    pub const fn raw_dst_img_size(&self) -> &RawDstImgSize {
        &self.raw_dst_img_size
    }
    #[doc = "0x70 - brightness,contrast,saturation adjustment"]
    #[inline(always)]
    pub const fn raw_enh_yuv_cnfg_0(&self) -> &RawEnhYuvCnfg0 {
        &self.raw_enh_yuv_cnfg_0
    }
    #[doc = "0x74 - Hue configuration"]
    #[inline(always)]
    pub const fn raw_enh_yuv_cnfg_1(&self) -> &RawEnhYuvCnfg1 {
        &self.raw_enh_yuv_cnfg_1
    }
    #[doc = "0x78 - color bar configuration"]
    #[inline(always)]
    pub const fn raw_enh_yuv_cnfg_2(&self) -> &RawEnhYuvCnfg2 {
        &self.raw_enh_yuv_cnfg_2
    }
    #[doc = "0x7c - enhancement RGB configuration"]
    #[inline(always)]
    pub const fn raw_enh_rgb_cnfg(&self) -> &RawEnhRgbCnfg {
        &self.raw_enh_rgb_cnfg
    }
    #[doc = "0x80 - Start address of source image(Y/RGB)"]
    #[inline(always)]
    pub const fn src_addr_yrgb(&self) -> &SrcAddrYrgb {
        &self.src_addr_yrgb
    }
    #[doc = "0x84 - Start address of source image(Cb/Cr)"]
    #[inline(always)]
    pub const fn src_addr_cbcr(&self) -> &SrcAddrCbcr {
        &self.src_addr_cbcr
    }
    #[doc = "0x88 - Start address of source image(Cr)"]
    #[inline(always)]
    pub const fn src_addr_cr(&self) -> &SrcAddrCr {
        &self.src_addr_cr
    }
    #[doc = "0x8c - Start address of source image(Y)"]
    #[inline(always)]
    pub const fn src_addr_y1(&self) -> &SrcAddrY1 {
        &self.src_addr_y1
    }
    #[doc = "0x90 - Start address of source image(Cb/Cr)"]
    #[inline(always)]
    pub const fn src_addr_cbcr1(&self) -> &SrcAddrCbcr1 {
        &self.src_addr_cbcr1
    }
    #[doc = "0x94 - Start address of source image(Cr)"]
    #[inline(always)]
    pub const fn src_addr_cr1(&self) -> &SrcAddrCr1 {
        &self.src_addr_cr1
    }
    #[doc = "0x98 - Start address of source image(Y integer part)"]
    #[inline(always)]
    pub const fn src_addr_y_itemp(&self) -> &SrcAddrYItemp {
        &self.src_addr_y_itemp
    }
    #[doc = "0x9c - Start address of source image(CBCR integer part)"]
    #[inline(always)]
    pub const fn src_addr_cbcr_itemp(&self) -> &SrcAddrCbcrItemp {
        &self.src_addr_cbcr_itemp
    }
    #[doc = "0xa0 - Start address of source image(CR integer part)"]
    #[inline(always)]
    pub const fn src_addr_cr_itemp(&self) -> &SrcAddrCrItemp {
        &self.src_addr_cr_itemp
    }
    #[doc = "0xa4 - Start address of source image(Y fraction part)"]
    #[inline(always)]
    pub const fn src_addr_y_ftemp(&self) -> &SrcAddrYFtemp {
        &self.src_addr_y_ftemp
    }
    #[doc = "0xa8 - Start address of source image(CBCR fraction part)"]
    #[inline(always)]
    pub const fn src_addr_cbcr_ftemp(&self) -> &SrcAddrCbcrFtemp {
        &self.src_addr_cbcr_ftemp
    }
    #[doc = "0xac - Start address of source image(CR fraction part)"]
    #[inline(always)]
    pub const fn src_addr_cr_ftemp(&self) -> &SrcAddrCrFtemp {
        &self.src_addr_cr_ftemp
    }
    #[doc = "0xb0 - Start address of destination image(Y/RGB)"]
    #[inline(always)]
    pub const fn dst_addr_yrgb(&self) -> &DstAddrYrgb {
        &self.dst_addr_yrgb
    }
    #[doc = "0xb4 - Start address of destination image(Cb/Cr)"]
    #[inline(always)]
    pub const fn dst_addr_cbcr(&self) -> &DstAddrCbcr {
        &self.dst_addr_cbcr
    }
    #[doc = "0xb8 - Start address of destination image(Cr)"]
    #[inline(always)]
    pub const fn dst_addr_cr(&self) -> &DstAddrCr {
        &self.dst_addr_cr
    }
    #[doc = "0xbc - Start address of destination image(Y)"]
    #[inline(always)]
    pub const fn dst_addr_y1(&self) -> &DstAddrY1 {
        &self.dst_addr_y1
    }
    #[doc = "0xc0 - Start address of destination image(Cb/Cr)"]
    #[inline(always)]
    pub const fn dst_addr_cbcr1(&self) -> &DstAddrCbcr1 {
        &self.dst_addr_cbcr1
    }
    #[doc = "0xc4 - Start address of destination image(Cr)"]
    #[inline(always)]
    pub const fn dst_addr_cr1(&self) -> &DstAddrCr1 {
        &self.dst_addr_cr1
    }
    #[doc = "0xc8 - Start address of destination image(Y integer part)"]
    #[inline(always)]
    pub const fn dst_addr_y_itemp(&self) -> &DstAddrYItemp {
        &self.dst_addr_y_itemp
    }
    #[doc = "0xcc - Start address of destination image(CBCR integer part)"]
    #[inline(always)]
    pub const fn dst_addr_cbcr_itemp(&self) -> &DstAddrCbcrItemp {
        &self.dst_addr_cbcr_itemp
    }
    #[doc = "0xd0 - Start address of destination image(CR integer part)"]
    #[inline(always)]
    pub const fn dst_addr_cr_itemp(&self) -> &DstAddrCrItemp {
        &self.dst_addr_cr_itemp
    }
    #[doc = "0xd4 - Start address of destination image(Y fraction part)"]
    #[inline(always)]
    pub const fn dst_addr_y_ftemp(&self) -> &DstAddrYFtemp {
        &self.dst_addr_y_ftemp
    }
    #[doc = "0xd8 - Start address of destination image(CBCR fraction part)"]
    #[inline(always)]
    pub const fn dst_addr_cbcr_ftemp(&self) -> &DstAddrCbcrFtemp {
        &self.dst_addr_cbcr_ftemp
    }
    #[doc = "0xdc - Start address of destination image(CR fraction part)"]
    #[inline(always)]
    pub const fn dst_addr_cr_ftemp(&self) -> &DstAddrCrFtemp {
        &self.dst_addr_cr_ftemp
    }
    #[doc = "0xe0 - Deinterlace motion table0"]
    #[inline(always)]
    pub const fn dil_mtn_tab0(&self) -> &DilMtnTab0 {
        &self.dil_mtn_tab0
    }
    #[doc = "0xe4 - Deinterlace motion table1"]
    #[inline(always)]
    pub const fn dil_mtn_tab1(&self) -> &DilMtnTab1 {
        &self.dil_mtn_tab1
    }
    #[doc = "0xe8 - Deinterlace motion table2"]
    #[inline(always)]
    pub const fn dil_mtn_tab2(&self) -> &DilMtnTab2 {
        &self.dil_mtn_tab2
    }
    #[doc = "0xec - Deinterlace motion table3"]
    #[inline(always)]
    pub const fn dil_mtn_tab3(&self) -> &DilMtnTab3 {
        &self.dil_mtn_tab3
    }
    #[doc = "0xf0 - Deinterlace motion table4"]
    #[inline(always)]
    pub const fn dil_mtn_tab4(&self) -> &DilMtnTab4 {
        &self.dil_mtn_tab4
    }
    #[doc = "0xf4 - Deinterlace motion table5"]
    #[inline(always)]
    pub const fn dil_mtn_tab5(&self) -> &DilMtnTab5 {
        &self.dil_mtn_tab5
    }
    #[doc = "0xf8 - Deinterlace motion table6"]
    #[inline(always)]
    pub const fn dil_mtn_tab6(&self) -> &DilMtnTab6 {
        &self.dil_mtn_tab6
    }
    #[doc = "0xfc - Deinterlace motion table7"]
    #[inline(always)]
    pub const fn dil_mtn_tab7(&self) -> &DilMtnTab7 {
        &self.dil_mtn_tab7
    }
    #[doc = "0x100 - contrast and gamma enhancement table"]
    #[inline(always)]
    pub const fn enh_cg_tab(&self) -> &EnhCgTab {
        &self.enh_cg_tab
    }
    #[doc = "0x400 - denoise,detail and edge enhancement coefficient"]
    #[inline(always)]
    pub const fn enh_dde_coe0(&self) -> &EnhDdeCoe0 {
        &self.enh_dde_coe0
    }
    #[doc = "0x500 - denoise,detail and edge enhancement coefficient"]
    #[inline(always)]
    pub const fn enh_dde_coe1(&self) -> &EnhDdeCoe1 {
        &self.enh_dde_coe1
    }
    #[doc = "0x600 - Axi performance latency module contrl register0"]
    #[inline(always)]
    pub const fn perf_latency_ctrl0(&self) -> &PerfLatencyCtrl0 {
        &self.perf_latency_ctrl0
    }
    #[doc = "0x604 - PERF_LATENCY_CTRL1"]
    #[inline(always)]
    pub const fn perf_latency_ctrl1(&self) -> &PerfLatencyCtrl1 {
        &self.perf_latency_ctrl1
    }
    #[doc = "0x608 - Read max latency number"]
    #[inline(always)]
    pub const fn perf_rd_max_latency_num0(&self) -> &PerfRdMaxLatencyNum0 {
        &self.perf_rd_max_latency_num0
    }
    #[doc = "0x60c - The number of bigger than configed threshold value"]
    #[inline(always)]
    pub const fn perf_rd_latency_samp_num(&self) -> &PerfRdLatencySampNum {
        &self.perf_rd_latency_samp_num
    }
    #[doc = "0x610 - Total sample number"]
    #[inline(always)]
    pub const fn perf_rd_latency_acc_sum(&self) -> &PerfRdLatencyAccSum {
        &self.perf_rd_latency_acc_sum
    }
    #[doc = "0x614 - perf_wr_axi_total_byte"]
    #[inline(always)]
    pub const fn perf_wr_axi_total_byte(&self) -> &PerfWrAxiTotalByte {
        &self.perf_wr_axi_total_byte
    }
    #[doc = "0x618 - perf_working_cnt"]
    #[inline(always)]
    pub const fn perf_working_cnt(&self) -> &PerfWorkingCnt {
        &self.perf_working_cnt
    }
    #[doc = "0x61c - perf_rd_axi_total_byte"]
    #[inline(always)]
    pub const fn perf_rd_axi_total_byte(&self) -> &PerfRdAxiTotalByte {
        &self.perf_rd_axi_total_byte
    }
}
#[doc = "CONFIG0 (rw) register accessor: configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config0`]
module"]
#[doc(alias = "CONFIG0")]
pub type Config0 = crate::Reg<config0::Config0Spec>;
#[doc = "configuration register0"]
pub mod config0;
#[doc = "CONFIG1 (rw) register accessor: configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config1`]
module"]
#[doc(alias = "CONFIG1")]
pub type Config1 = crate::Reg<config1::Config1Spec>;
#[doc = "configuration register1"]
pub mod config1;
#[doc = "STATUS (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "status register"]
pub mod status;
#[doc = "INT (rw) register accessor: interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int`]
module"]
#[doc(alias = "INT")]
pub type Int = crate::Reg<int::IntSpec>;
#[doc = "interrupt register"]
pub mod int;
#[doc = "FRM_START (rw) register accessor: frame start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frm_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frm_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frm_start`]
module"]
#[doc(alias = "FRM_START")]
pub type FrmStart = crate::Reg<frm_start::FrmStartSpec>;
#[doc = "frame start"]
pub mod frm_start;
#[doc = "CONFIG_DONE (rw) register accessor: configuration done\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_done::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_done::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_done`]
module"]
#[doc(alias = "CONFIG_DONE")]
pub type ConfigDone = crate::Reg<config_done::ConfigDoneSpec>;
#[doc = "configuration done"]
pub mod config_done;
#[doc = "FRM_CNT (rw) register accessor: frame counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frm_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frm_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frm_cnt`]
module"]
#[doc(alias = "FRM_CNT")]
pub type FrmCnt = crate::Reg<frm_cnt::FrmCntSpec>;
#[doc = "frame counter"]
pub mod frm_cnt;
#[doc = "VIR_IMG_WIDTH (rw) register accessor: Image virtual width\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vir_img_width::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vir_img_width::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vir_img_width`]
module"]
#[doc(alias = "VIR_IMG_WIDTH")]
pub type VirImgWidth = crate::Reg<vir_img_width::VirImgWidthSpec>;
#[doc = "Image virtual width"]
pub mod vir_img_width;
#[doc = "SRC_IMG_SIZE (rw) register accessor: Source image width/height\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_img_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_img_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_img_size`]
module"]
#[doc(alias = "SRC_IMG_SIZE")]
pub type SrcImgSize = crate::Reg<src_img_size::SrcImgSizeSpec>;
#[doc = "Source image width/height"]
pub mod src_img_size;
#[doc = "DST_IMG_SIZE (rw) register accessor: Destination image width/height\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_img_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_img_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_img_size`]
module"]
#[doc(alias = "DST_IMG_SIZE")]
pub type DstImgSize = crate::Reg<dst_img_size::DstImgSizeSpec>;
#[doc = "Destination image width/height"]
pub mod dst_img_size;
#[doc = "DST_IMG_WIDTH_TILE0 (rw) register accessor: Destination image tile0 width\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_img_width_tile0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_img_width_tile0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_img_width_tile0`]
module"]
#[doc(alias = "DST_IMG_WIDTH_TILE0")]
pub type DstImgWidthTile0 = crate::Reg<dst_img_width_tile0::DstImgWidthTile0Spec>;
#[doc = "Destination image tile0 width"]
pub mod dst_img_width_tile0;
#[doc = "DST_IMG_WIDTH_TILE1 (rw) register accessor: Destination image tile1 width\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_img_width_tile1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_img_width_tile1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_img_width_tile1`]
module"]
#[doc(alias = "DST_IMG_WIDTH_TILE1")]
pub type DstImgWidthTile1 = crate::Reg<dst_img_width_tile1::DstImgWidthTile1Spec>;
#[doc = "Destination image tile1 width"]
pub mod dst_img_width_tile1;
#[doc = "DST_IMG_WIDTH_TILE2 (rw) register accessor: Destination image tile2 width\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_img_width_tile2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_img_width_tile2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_img_width_tile2`]
module"]
#[doc(alias = "DST_IMG_WIDTH_TILE2")]
pub type DstImgWidthTile2 = crate::Reg<dst_img_width_tile2::DstImgWidthTile2Spec>;
#[doc = "Destination image tile2 width"]
pub mod dst_img_width_tile2;
#[doc = "DST_IMG_WIDTH_TILE3 (rw) register accessor: Destination image tile3 width\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_img_width_tile3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_img_width_tile3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_img_width_tile3`]
module"]
#[doc(alias = "DST_IMG_WIDTH_TILE3")]
pub type DstImgWidthTile3 = crate::Reg<dst_img_width_tile3::DstImgWidthTile3Spec>;
#[doc = "Destination image tile3 width"]
pub mod dst_img_width_tile3;
#[doc = "ENH_YUV_CNFG_0 (rw) register accessor: brightness,contrast,saturation adjustment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enh_yuv_cnfg_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enh_yuv_cnfg_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enh_yuv_cnfg_0`]
module"]
#[doc(alias = "ENH_YUV_CNFG_0")]
pub type EnhYuvCnfg0 = crate::Reg<enh_yuv_cnfg_0::EnhYuvCnfg0Spec>;
#[doc = "brightness,contrast,saturation adjustment"]
pub mod enh_yuv_cnfg_0;
#[doc = "ENH_YUV_CNFG_1 (rw) register accessor: Hue configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enh_yuv_cnfg_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enh_yuv_cnfg_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enh_yuv_cnfg_1`]
module"]
#[doc(alias = "ENH_YUV_CNFG_1")]
pub type EnhYuvCnfg1 = crate::Reg<enh_yuv_cnfg_1::EnhYuvCnfg1Spec>;
#[doc = "Hue configuration"]
pub mod enh_yuv_cnfg_1;
#[doc = "ENH_YUV_CNFG_2 (rw) register accessor: color bar configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enh_yuv_cnfg_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enh_yuv_cnfg_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enh_yuv_cnfg_2`]
module"]
#[doc(alias = "ENH_YUV_CNFG_2")]
pub type EnhYuvCnfg2 = crate::Reg<enh_yuv_cnfg_2::EnhYuvCnfg2Spec>;
#[doc = "color bar configuration"]
pub mod enh_yuv_cnfg_2;
#[doc = "ENH_RGB_CNFG (rw) register accessor: enhancement RGB configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enh_rgb_cnfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enh_rgb_cnfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enh_rgb_cnfg`]
module"]
#[doc(alias = "ENH_RGB_CNFG")]
pub type EnhRgbCnfg = crate::Reg<enh_rgb_cnfg::EnhRgbCnfgSpec>;
#[doc = "enhancement RGB configuration"]
pub mod enh_rgb_cnfg;
#[doc = "ENH_C_COE (rw) register accessor: rgb color enhancement coefficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enh_c_coe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enh_c_coe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enh_c_coe`]
module"]
#[doc(alias = "ENH_C_COE")]
pub type EnhCCoe = crate::Reg<enh_c_coe::EnhCCoeSpec>;
#[doc = "rgb color enhancement coefficient"]
pub mod enh_c_coe;
#[doc = "VERSION_INFO (rw) register accessor: Version number for iep\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version_info`]
module"]
#[doc(alias = "VERSION_INFO")]
pub type VersionInfo = crate::Reg<version_info::VersionInfoSpec>;
#[doc = "Version number for iep"]
pub mod version_info;
#[doc = "RAW_CONFIG0 (rw) register accessor: configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_config0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`raw_config0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_config0`]
module"]
#[doc(alias = "RAW_CONFIG0")]
pub type RawConfig0 = crate::Reg<raw_config0::RawConfig0Spec>;
#[doc = "configuration register0"]
pub mod raw_config0;
#[doc = "RAW_CONFIG1 (r) register accessor: configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_config1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_config1`]
module"]
#[doc(alias = "RAW_CONFIG1")]
pub type RawConfig1 = crate::Reg<raw_config1::RawConfig1Spec>;
#[doc = "configuration register1"]
pub mod raw_config1;
#[doc = "RAW_VIR_IMG_WIDTH (r) register accessor: Image virtual width\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_vir_img_width::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_vir_img_width`]
module"]
#[doc(alias = "RAW_VIR_IMG_WIDTH")]
pub type RawVirImgWidth = crate::Reg<raw_vir_img_width::RawVirImgWidthSpec>;
#[doc = "Image virtual width"]
pub mod raw_vir_img_width;
#[doc = "RAW_SRC_IMG_SIZE (r) register accessor: Source image width/height\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_src_img_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_src_img_size`]
module"]
#[doc(alias = "RAW_SRC_IMG_SIZE")]
pub type RawSrcImgSize = crate::Reg<raw_src_img_size::RawSrcImgSizeSpec>;
#[doc = "Source image width/height"]
pub mod raw_src_img_size;
#[doc = "RAW_DST_IMG_SIZE (r) register accessor: Destination image width/height\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_dst_img_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_dst_img_size`]
module"]
#[doc(alias = "RAW_DST_IMG_SIZE")]
pub type RawDstImgSize = crate::Reg<raw_dst_img_size::RawDstImgSizeSpec>;
#[doc = "Destination image width/height"]
pub mod raw_dst_img_size;
#[doc = "RAW_ENH_YUV_CNFG_0 (r) register accessor: brightness,contrast,saturation adjustment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_enh_yuv_cnfg_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_enh_yuv_cnfg_0`]
module"]
#[doc(alias = "RAW_ENH_YUV_CNFG_0")]
pub type RawEnhYuvCnfg0 = crate::Reg<raw_enh_yuv_cnfg_0::RawEnhYuvCnfg0Spec>;
#[doc = "brightness,contrast,saturation adjustment"]
pub mod raw_enh_yuv_cnfg_0;
#[doc = "RAW_ENH_YUV_CNFG_1 (r) register accessor: Hue configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_enh_yuv_cnfg_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_enh_yuv_cnfg_1`]
module"]
#[doc(alias = "RAW_ENH_YUV_CNFG_1")]
pub type RawEnhYuvCnfg1 = crate::Reg<raw_enh_yuv_cnfg_1::RawEnhYuvCnfg1Spec>;
#[doc = "Hue configuration"]
pub mod raw_enh_yuv_cnfg_1;
#[doc = "RAW_ENH_YUV_CNFG_2 (r) register accessor: color bar configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_enh_yuv_cnfg_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_enh_yuv_cnfg_2`]
module"]
#[doc(alias = "RAW_ENH_YUV_CNFG_2")]
pub type RawEnhYuvCnfg2 = crate::Reg<raw_enh_yuv_cnfg_2::RawEnhYuvCnfg2Spec>;
#[doc = "color bar configuration"]
pub mod raw_enh_yuv_cnfg_2;
#[doc = "RAW_ENH_RGB_CNFG (rw) register accessor: enhancement RGB configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_enh_rgb_cnfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`raw_enh_rgb_cnfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_enh_rgb_cnfg`]
module"]
#[doc(alias = "RAW_ENH_RGB_CNFG")]
pub type RawEnhRgbCnfg = crate::Reg<raw_enh_rgb_cnfg::RawEnhRgbCnfgSpec>;
#[doc = "enhancement RGB configuration"]
pub mod raw_enh_rgb_cnfg;
#[doc = "SRC_ADDR_YRGB (rw) register accessor: Start address of source image(Y/RGB)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_yrgb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_yrgb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_addr_yrgb`]
module"]
#[doc(alias = "SRC_ADDR_YRGB")]
pub type SrcAddrYrgb = crate::Reg<src_addr_yrgb::SrcAddrYrgbSpec>;
#[doc = "Start address of source image(Y/RGB)"]
pub mod src_addr_yrgb;
#[doc = "SRC_ADDR_CBCR (rw) register accessor: Start address of source image(Cb/Cr)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_cbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_cbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_addr_cbcr`]
module"]
#[doc(alias = "SRC_ADDR_CBCR")]
pub type SrcAddrCbcr = crate::Reg<src_addr_cbcr::SrcAddrCbcrSpec>;
#[doc = "Start address of source image(Cb/Cr)"]
pub mod src_addr_cbcr;
#[doc = "SRC_ADDR_CR (rw) register accessor: Start address of source image(Cr)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_addr_cr`]
module"]
#[doc(alias = "SRC_ADDR_CR")]
pub type SrcAddrCr = crate::Reg<src_addr_cr::SrcAddrCrSpec>;
#[doc = "Start address of source image(Cr)"]
pub mod src_addr_cr;
#[doc = "SRC_ADDR_Y1 (rw) register accessor: Start address of source image(Y)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_y1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_y1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_addr_y1`]
module"]
#[doc(alias = "SRC_ADDR_Y1")]
pub type SrcAddrY1 = crate::Reg<src_addr_y1::SrcAddrY1Spec>;
#[doc = "Start address of source image(Y)"]
pub mod src_addr_y1;
#[doc = "SRC_ADDR_CBCR1 (rw) register accessor: Start address of source image(Cb/Cr)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_cbcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_cbcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_addr_cbcr1`]
module"]
#[doc(alias = "SRC_ADDR_CBCR1")]
pub type SrcAddrCbcr1 = crate::Reg<src_addr_cbcr1::SrcAddrCbcr1Spec>;
#[doc = "Start address of source image(Cb/Cr)"]
pub mod src_addr_cbcr1;
#[doc = "SRC_ADDR_CR1 (rw) register accessor: Start address of source image(Cr)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_addr_cr1`]
module"]
#[doc(alias = "SRC_ADDR_CR1")]
pub type SrcAddrCr1 = crate::Reg<src_addr_cr1::SrcAddrCr1Spec>;
#[doc = "Start address of source image(Cr)"]
pub mod src_addr_cr1;
#[doc = "SRC_ADDR_Y_ITEMP (rw) register accessor: Start address of source image(Y integer part)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_y_itemp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_y_itemp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_addr_y_itemp`]
module"]
#[doc(alias = "SRC_ADDR_Y_ITEMP")]
pub type SrcAddrYItemp = crate::Reg<src_addr_y_itemp::SrcAddrYItempSpec>;
#[doc = "Start address of source image(Y integer part)"]
pub mod src_addr_y_itemp;
#[doc = "SRC_ADDR_CBCR_ITEMP (rw) register accessor: Start address of source image(CBCR integer part)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_cbcr_itemp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_cbcr_itemp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_addr_cbcr_itemp`]
module"]
#[doc(alias = "SRC_ADDR_CBCR_ITEMP")]
pub type SrcAddrCbcrItemp = crate::Reg<src_addr_cbcr_itemp::SrcAddrCbcrItempSpec>;
#[doc = "Start address of source image(CBCR integer part)"]
pub mod src_addr_cbcr_itemp;
#[doc = "SRC_ADDR_CR_ITEMP (rw) register accessor: Start address of source image(CR integer part)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_cr_itemp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_cr_itemp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_addr_cr_itemp`]
module"]
#[doc(alias = "SRC_ADDR_CR_ITEMP")]
pub type SrcAddrCrItemp = crate::Reg<src_addr_cr_itemp::SrcAddrCrItempSpec>;
#[doc = "Start address of source image(CR integer part)"]
pub mod src_addr_cr_itemp;
#[doc = "SRC_ADDR_Y_FTEMP (rw) register accessor: Start address of source image(Y fraction part)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_y_ftemp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_y_ftemp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_addr_y_ftemp`]
module"]
#[doc(alias = "SRC_ADDR_Y_FTEMP")]
pub type SrcAddrYFtemp = crate::Reg<src_addr_y_ftemp::SrcAddrYFtempSpec>;
#[doc = "Start address of source image(Y fraction part)"]
pub mod src_addr_y_ftemp;
#[doc = "SRC_ADDR_CBCR_FTEMP (rw) register accessor: Start address of source image(CBCR fraction part)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_cbcr_ftemp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_cbcr_ftemp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_addr_cbcr_ftemp`]
module"]
#[doc(alias = "SRC_ADDR_CBCR_FTEMP")]
pub type SrcAddrCbcrFtemp = crate::Reg<src_addr_cbcr_ftemp::SrcAddrCbcrFtempSpec>;
#[doc = "Start address of source image(CBCR fraction part)"]
pub mod src_addr_cbcr_ftemp;
#[doc = "SRC_ADDR_CR_FTEMP (rw) register accessor: Start address of source image(CR fraction part)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_cr_ftemp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_cr_ftemp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_addr_cr_ftemp`]
module"]
#[doc(alias = "SRC_ADDR_CR_FTEMP")]
pub type SrcAddrCrFtemp = crate::Reg<src_addr_cr_ftemp::SrcAddrCrFtempSpec>;
#[doc = "Start address of source image(CR fraction part)"]
pub mod src_addr_cr_ftemp;
#[doc = "DST_ADDR_YRGB (rw) register accessor: Start address of destination image(Y/RGB)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_yrgb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_yrgb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_addr_yrgb`]
module"]
#[doc(alias = "DST_ADDR_YRGB")]
pub type DstAddrYrgb = crate::Reg<dst_addr_yrgb::DstAddrYrgbSpec>;
#[doc = "Start address of destination image(Y/RGB)"]
pub mod dst_addr_yrgb;
#[doc = "DST_ADDR_CBCR (rw) register accessor: Start address of destination image(Cb/Cr)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_cbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_cbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_addr_cbcr`]
module"]
#[doc(alias = "DST_ADDR_CBCR")]
pub type DstAddrCbcr = crate::Reg<dst_addr_cbcr::DstAddrCbcrSpec>;
#[doc = "Start address of destination image(Cb/Cr)"]
pub mod dst_addr_cbcr;
#[doc = "DST_ADDR_CR (rw) register accessor: Start address of destination image(Cr)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_addr_cr`]
module"]
#[doc(alias = "DST_ADDR_CR")]
pub type DstAddrCr = crate::Reg<dst_addr_cr::DstAddrCrSpec>;
#[doc = "Start address of destination image(Cr)"]
pub mod dst_addr_cr;
#[doc = "DST_ADDR_Y1 (rw) register accessor: Start address of destination image(Y)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_y1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_y1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_addr_y1`]
module"]
#[doc(alias = "DST_ADDR_Y1")]
pub type DstAddrY1 = crate::Reg<dst_addr_y1::DstAddrY1Spec>;
#[doc = "Start address of destination image(Y)"]
pub mod dst_addr_y1;
#[doc = "DST_ADDR_CBCR1 (rw) register accessor: Start address of destination image(Cb/Cr)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_cbcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_cbcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_addr_cbcr1`]
module"]
#[doc(alias = "DST_ADDR_CBCR1")]
pub type DstAddrCbcr1 = crate::Reg<dst_addr_cbcr1::DstAddrCbcr1Spec>;
#[doc = "Start address of destination image(Cb/Cr)"]
pub mod dst_addr_cbcr1;
#[doc = "DST_ADDR_CR1 (rw) register accessor: Start address of destination image(Cr)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_addr_cr1`]
module"]
#[doc(alias = "DST_ADDR_CR1")]
pub type DstAddrCr1 = crate::Reg<dst_addr_cr1::DstAddrCr1Spec>;
#[doc = "Start address of destination image(Cr)"]
pub mod dst_addr_cr1;
#[doc = "DST_ADDR_Y_ITEMP (rw) register accessor: Start address of destination image(Y integer part)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_y_itemp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_y_itemp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_addr_y_itemp`]
module"]
#[doc(alias = "DST_ADDR_Y_ITEMP")]
pub type DstAddrYItemp = crate::Reg<dst_addr_y_itemp::DstAddrYItempSpec>;
#[doc = "Start address of destination image(Y integer part)"]
pub mod dst_addr_y_itemp;
#[doc = "DST_ADDR_CBCR_ITEMP (rw) register accessor: Start address of destination image(CBCR integer part)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_cbcr_itemp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_cbcr_itemp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_addr_cbcr_itemp`]
module"]
#[doc(alias = "DST_ADDR_CBCR_ITEMP")]
pub type DstAddrCbcrItemp = crate::Reg<dst_addr_cbcr_itemp::DstAddrCbcrItempSpec>;
#[doc = "Start address of destination image(CBCR integer part)"]
pub mod dst_addr_cbcr_itemp;
#[doc = "DST_ADDR_CR_ITEMP (rw) register accessor: Start address of destination image(CR integer part)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_cr_itemp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_cr_itemp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_addr_cr_itemp`]
module"]
#[doc(alias = "DST_ADDR_CR_ITEMP")]
pub type DstAddrCrItemp = crate::Reg<dst_addr_cr_itemp::DstAddrCrItempSpec>;
#[doc = "Start address of destination image(CR integer part)"]
pub mod dst_addr_cr_itemp;
#[doc = "DST_ADDR_Y_FTEMP (rw) register accessor: Start address of destination image(Y fraction part)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_y_ftemp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_y_ftemp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_addr_y_ftemp`]
module"]
#[doc(alias = "DST_ADDR_Y_FTEMP")]
pub type DstAddrYFtemp = crate::Reg<dst_addr_y_ftemp::DstAddrYFtempSpec>;
#[doc = "Start address of destination image(Y fraction part)"]
pub mod dst_addr_y_ftemp;
#[doc = "DST_ADDR_CBCR_FTEMP (rw) register accessor: Start address of destination image(CBCR fraction part)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_cbcr_ftemp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_cbcr_ftemp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_addr_cbcr_ftemp`]
module"]
#[doc(alias = "DST_ADDR_CBCR_FTEMP")]
pub type DstAddrCbcrFtemp = crate::Reg<dst_addr_cbcr_ftemp::DstAddrCbcrFtempSpec>;
#[doc = "Start address of destination image(CBCR fraction part)"]
pub mod dst_addr_cbcr_ftemp;
#[doc = "DST_ADDR_CR_FTEMP (rw) register accessor: Start address of destination image(CR fraction part)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_cr_ftemp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_cr_ftemp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_addr_cr_ftemp`]
module"]
#[doc(alias = "DST_ADDR_CR_FTEMP")]
pub type DstAddrCrFtemp = crate::Reg<dst_addr_cr_ftemp::DstAddrCrFtempSpec>;
#[doc = "Start address of destination image(CR fraction part)"]
pub mod dst_addr_cr_ftemp;
#[doc = "DIL_MTN_TAB0 (rw) register accessor: Deinterlace motion table0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dil_mtn_tab0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dil_mtn_tab0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dil_mtn_tab0`]
module"]
#[doc(alias = "DIL_MTN_TAB0")]
pub type DilMtnTab0 = crate::Reg<dil_mtn_tab0::DilMtnTab0Spec>;
#[doc = "Deinterlace motion table0"]
pub mod dil_mtn_tab0;
#[doc = "DIL_MTN_TAB1 (rw) register accessor: Deinterlace motion table1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dil_mtn_tab1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dil_mtn_tab1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dil_mtn_tab1`]
module"]
#[doc(alias = "DIL_MTN_TAB1")]
pub type DilMtnTab1 = crate::Reg<dil_mtn_tab1::DilMtnTab1Spec>;
#[doc = "Deinterlace motion table1"]
pub mod dil_mtn_tab1;
#[doc = "DIL_MTN_TAB2 (rw) register accessor: Deinterlace motion table2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dil_mtn_tab2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dil_mtn_tab2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dil_mtn_tab2`]
module"]
#[doc(alias = "DIL_MTN_TAB2")]
pub type DilMtnTab2 = crate::Reg<dil_mtn_tab2::DilMtnTab2Spec>;
#[doc = "Deinterlace motion table2"]
pub mod dil_mtn_tab2;
#[doc = "DIL_MTN_TAB3 (rw) register accessor: Deinterlace motion table3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dil_mtn_tab3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dil_mtn_tab3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dil_mtn_tab3`]
module"]
#[doc(alias = "DIL_MTN_TAB3")]
pub type DilMtnTab3 = crate::Reg<dil_mtn_tab3::DilMtnTab3Spec>;
#[doc = "Deinterlace motion table3"]
pub mod dil_mtn_tab3;
#[doc = "DIL_MTN_TAB4 (rw) register accessor: Deinterlace motion table4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dil_mtn_tab4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dil_mtn_tab4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dil_mtn_tab4`]
module"]
#[doc(alias = "DIL_MTN_TAB4")]
pub type DilMtnTab4 = crate::Reg<dil_mtn_tab4::DilMtnTab4Spec>;
#[doc = "Deinterlace motion table4"]
pub mod dil_mtn_tab4;
#[doc = "DIL_MTN_TAB5 (rw) register accessor: Deinterlace motion table5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dil_mtn_tab5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dil_mtn_tab5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dil_mtn_tab5`]
module"]
#[doc(alias = "DIL_MTN_TAB5")]
pub type DilMtnTab5 = crate::Reg<dil_mtn_tab5::DilMtnTab5Spec>;
#[doc = "Deinterlace motion table5"]
pub mod dil_mtn_tab5;
#[doc = "DIL_MTN_TAB6 (rw) register accessor: Deinterlace motion table6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dil_mtn_tab6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dil_mtn_tab6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dil_mtn_tab6`]
module"]
#[doc(alias = "DIL_MTN_TAB6")]
pub type DilMtnTab6 = crate::Reg<dil_mtn_tab6::DilMtnTab6Spec>;
#[doc = "Deinterlace motion table6"]
pub mod dil_mtn_tab6;
#[doc = "DIL_MTN_TAB7 (rw) register accessor: Deinterlace motion table7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dil_mtn_tab7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dil_mtn_tab7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dil_mtn_tab7`]
module"]
#[doc(alias = "DIL_MTN_TAB7")]
pub type DilMtnTab7 = crate::Reg<dil_mtn_tab7::DilMtnTab7Spec>;
#[doc = "Deinterlace motion table7"]
pub mod dil_mtn_tab7;
#[doc = "ENH_CG_TAB (rw) register accessor: contrast and gamma enhancement table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enh_cg_tab::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enh_cg_tab::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enh_cg_tab`]
module"]
#[doc(alias = "ENH_CG_TAB")]
pub type EnhCgTab = crate::Reg<enh_cg_tab::EnhCgTabSpec>;
#[doc = "contrast and gamma enhancement table"]
pub mod enh_cg_tab;
#[doc = "ENH_DDE_COE0 (rw) register accessor: denoise,detail and edge enhancement coefficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enh_dde_coe0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enh_dde_coe0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enh_dde_coe0`]
module"]
#[doc(alias = "ENH_DDE_COE0")]
pub type EnhDdeCoe0 = crate::Reg<enh_dde_coe0::EnhDdeCoe0Spec>;
#[doc = "denoise,detail and edge enhancement coefficient"]
pub mod enh_dde_coe0;
#[doc = "ENH_DDE_COE1 (rw) register accessor: denoise,detail and edge enhancement coefficient\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enh_dde_coe1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enh_dde_coe1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enh_dde_coe1`]
module"]
#[doc(alias = "ENH_DDE_COE1")]
pub type EnhDdeCoe1 = crate::Reg<enh_dde_coe1::EnhDdeCoe1Spec>;
#[doc = "denoise,detail and edge enhancement coefficient"]
pub mod enh_dde_coe1;
#[doc = "PERF_LATENCY_CTRL0 (rw) register accessor: Axi performance latency module contrl register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perf_latency_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perf_latency_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perf_latency_ctrl0`]
module"]
#[doc(alias = "PERF_LATENCY_CTRL0")]
pub type PerfLatencyCtrl0 = crate::Reg<perf_latency_ctrl0::PerfLatencyCtrl0Spec>;
#[doc = "Axi performance latency module contrl register0"]
pub mod perf_latency_ctrl0;
#[doc = "PERF_LATENCY_CTRL1 (rw) register accessor: PERF_LATENCY_CTRL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perf_latency_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perf_latency_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perf_latency_ctrl1`]
module"]
#[doc(alias = "PERF_LATENCY_CTRL1")]
pub type PerfLatencyCtrl1 = crate::Reg<perf_latency_ctrl1::PerfLatencyCtrl1Spec>;
#[doc = "PERF_LATENCY_CTRL1"]
pub mod perf_latency_ctrl1;
#[doc = "PERF_RD_MAX_LATENCY_NUM0 (r) register accessor: Read max latency number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perf_rd_max_latency_num0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perf_rd_max_latency_num0`]
module"]
#[doc(alias = "PERF_RD_MAX_LATENCY_NUM0")]
pub type PerfRdMaxLatencyNum0 = crate::Reg<perf_rd_max_latency_num0::PerfRdMaxLatencyNum0Spec>;
#[doc = "Read max latency number"]
pub mod perf_rd_max_latency_num0;
#[doc = "PERF_RD_LATENCY_SAMP_NUM (r) register accessor: The number of bigger than configed threshold value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perf_rd_latency_samp_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perf_rd_latency_samp_num`]
module"]
#[doc(alias = "PERF_RD_LATENCY_SAMP_NUM")]
pub type PerfRdLatencySampNum = crate::Reg<perf_rd_latency_samp_num::PerfRdLatencySampNumSpec>;
#[doc = "The number of bigger than configed threshold value"]
pub mod perf_rd_latency_samp_num;
#[doc = "PERF_RD_LATENCY_ACC_SUM (r) register accessor: Total sample number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perf_rd_latency_acc_sum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perf_rd_latency_acc_sum`]
module"]
#[doc(alias = "PERF_RD_LATENCY_ACC_SUM")]
pub type PerfRdLatencyAccSum = crate::Reg<perf_rd_latency_acc_sum::PerfRdLatencyAccSumSpec>;
#[doc = "Total sample number"]
pub mod perf_rd_latency_acc_sum;
#[doc = "PERF_WR_AXI_TOTAL_BYTE (rw) register accessor: perf_wr_axi_total_byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perf_wr_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perf_wr_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perf_wr_axi_total_byte`]
module"]
#[doc(alias = "PERF_WR_AXI_TOTAL_BYTE")]
pub type PerfWrAxiTotalByte = crate::Reg<perf_wr_axi_total_byte::PerfWrAxiTotalByteSpec>;
#[doc = "perf_wr_axi_total_byte"]
pub mod perf_wr_axi_total_byte;
#[doc = "PERF_WORKING_CNT (rw) register accessor: perf_working_cnt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perf_working_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perf_working_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perf_working_cnt`]
module"]
#[doc(alias = "PERF_WORKING_CNT")]
pub type PerfWorkingCnt = crate::Reg<perf_working_cnt::PerfWorkingCntSpec>;
#[doc = "perf_working_cnt"]
pub mod perf_working_cnt;
#[doc = "PERF_RD_AXI_TOTAL_BYTE (rw) register accessor: perf_rd_axi_total_byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perf_rd_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perf_rd_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perf_rd_axi_total_byte`]
module"]
#[doc(alias = "PERF_RD_AXI_TOTAL_BYTE")]
pub type PerfRdAxiTotalByte = crate::Reg<perf_rd_axi_total_byte::PerfRdAxiTotalByteSpec>;
#[doc = "perf_rd_axi_total_byte"]
pub mod perf_rd_axi_total_byte;
