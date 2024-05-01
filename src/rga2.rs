#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sys_ctrl: SysCtrl,
    cmd_ctrl: CmdCtrl,
    cmd_base: CmdBase,
    status1: Status1,
    int: Int,
    mmu_ctrl0: MmuCtrl0,
    mmu_cmd_base: MmuCmdBase,
    status2: Status2,
    work_cnt: WorkCnt,
    _reserved9: [u8; 0x04],
    version_info: VersionInfo,
    _reserved10: [u8; 0x14],
    perf_latency_ctrl0: PerfLatencyCtrl0,
    perf_latency_ctrl1: PerfLatencyCtrl1,
    perf_rd_max_latency_num0: PerfRdMaxLatencyNum0,
    perf_rd_latency_samp_num: PerfRdLatencySampNum,
    perf_rd_latency_acc_sum: PerfRdLatencyAccSum,
    perf_rd_axi_total_byte: PerfRdAxiTotalByte,
    perf_wr_axi_total_byte: PerfWrAxiTotalByte,
    perf_working_cnt: PerfWorkingCnt,
    _reserved18: [u8; 0xa0],
    mode_ctrl: ModeCtrl,
    src_info: SrcInfo,
    src_base0: SrcBase0,
    src_base1: SrcBase1,
    src_base2: SrcBase2,
    src_base3: SrcBase3,
    src_vir_info: SrcVirInfo,
    src_act_info: SrcActInfo,
    src_x_factor: SrcXFactor,
    src_y_factor: SrcYFactor,
    src_bg_color: SrcBgColor,
    src_fg_color: SrcFgColor,
    _reserved_30_cp_gr_a: [u8; 0x04],
    _reserved_31_cp_gr_b: [u8; 0x04],
    dst_info: DstInfo,
    dst_base0: DstBase0,
    dst_base1: DstBase1,
    dst_base2: DstBase2,
    dst_vir_info: DstVirInfo,
    dst_act_info: DstActInfo,
    alpha_ctrl0: AlphaCtrl0,
    alpha_ctrl1: AlphaCtrl1,
    fading_ctrl: FadingCtrl,
    pat_con: PatCon,
    _reserved_42_cp_gr_g: [u8; 0x04],
    _reserved_43_cp_gr_r: [u8; 0x04],
    mask_base: MaskBase,
    mmu_ctrl1: MmuCtrl1,
    mmu_src_base: MmuSrcBase,
    mmu_src1_base: MmuSrc1Base,
    mmu_dst_base: MmuDstBase,
    mmu_els_base: MmuElsBase,
}
impl RegisterBlock {
    #[doc = "0x00 - RGA system control register"]
    #[inline(always)]
    pub const fn sys_ctrl(&self) -> &SysCtrl {
        &self.sys_ctrl
    }
    #[doc = "0x04 - RGA command control register"]
    #[inline(always)]
    pub const fn cmd_ctrl(&self) -> &CmdCtrl {
        &self.cmd_ctrl
    }
    #[doc = "0x08 - RGA command codes base address register"]
    #[inline(always)]
    pub const fn cmd_base(&self) -> &CmdBase {
        &self.cmd_base
    }
    #[doc = "0x0c - RGA status register"]
    #[inline(always)]
    pub const fn status1(&self) -> &Status1 {
        &self.status1
    }
    #[doc = "0x10 - RGA interrupt register"]
    #[inline(always)]
    pub const fn int(&self) -> &Int {
        &self.int
    }
    #[doc = "0x14 - RGA MMU control 0 register"]
    #[inline(always)]
    pub const fn mmu_ctrl0(&self) -> &MmuCtrl0 {
        &self.mmu_ctrl0
    }
    #[doc = "0x18 - Register0000 Abstract"]
    #[inline(always)]
    pub const fn mmu_cmd_base(&self) -> &MmuCmdBase {
        &self.mmu_cmd_base
    }
    #[doc = "0x1c - RGA status register"]
    #[inline(always)]
    pub const fn status2(&self) -> &Status2 {
        &self.status2
    }
    #[doc = "0x20 - work counter"]
    #[inline(always)]
    pub const fn work_cnt(&self) -> &WorkCnt {
        &self.work_cnt
    }
    #[doc = "0x28 - Version number for rga"]
    #[inline(always)]
    pub const fn version_info(&self) -> &VersionInfo {
        &self.version_info
    }
    #[doc = "0x40 - Axi performance latency module contrl register0"]
    #[inline(always)]
    pub const fn perf_latency_ctrl0(&self) -> &PerfLatencyCtrl0 {
        &self.perf_latency_ctrl0
    }
    #[doc = "0x44 - PERF_LATENCY_CTRL1"]
    #[inline(always)]
    pub const fn perf_latency_ctrl1(&self) -> &PerfLatencyCtrl1 {
        &self.perf_latency_ctrl1
    }
    #[doc = "0x48 - Read max latency number"]
    #[inline(always)]
    pub const fn perf_rd_max_latency_num0(&self) -> &PerfRdMaxLatencyNum0 {
        &self.perf_rd_max_latency_num0
    }
    #[doc = "0x4c - The number of bigger than configed threshold value"]
    #[inline(always)]
    pub const fn perf_rd_latency_samp_num(&self) -> &PerfRdLatencySampNum {
        &self.perf_rd_latency_samp_num
    }
    #[doc = "0x50 - Total sample number"]
    #[inline(always)]
    pub const fn perf_rd_latency_acc_sum(&self) -> &PerfRdLatencyAccSum {
        &self.perf_rd_latency_acc_sum
    }
    #[doc = "0x54 - perf_rd_axi_total_byte"]
    #[inline(always)]
    pub const fn perf_rd_axi_total_byte(&self) -> &PerfRdAxiTotalByte {
        &self.perf_rd_axi_total_byte
    }
    #[doc = "0x58 - perf_wr_axi_total_byte"]
    #[inline(always)]
    pub const fn perf_wr_axi_total_byte(&self) -> &PerfWrAxiTotalByte {
        &self.perf_wr_axi_total_byte
    }
    #[doc = "0x5c - perf_working_cnt"]
    #[inline(always)]
    pub const fn perf_working_cnt(&self) -> &PerfWorkingCnt {
        &self.perf_working_cnt
    }
    #[doc = "0x100 - RGA mode control register"]
    #[inline(always)]
    pub const fn mode_ctrl(&self) -> &ModeCtrl {
        &self.mode_ctrl
    }
    #[doc = "0x104 - RGA source information register"]
    #[inline(always)]
    pub const fn src_info(&self) -> &SrcInfo {
        &self.src_info
    }
    #[doc = "0x108 - source image Y/RGB base address"]
    #[inline(always)]
    pub const fn src_base0(&self) -> &SrcBase0 {
        &self.src_base0
    }
    #[doc = "0x10c - RGA source image Cb/Cbr base address register"]
    #[inline(always)]
    pub const fn src_base1(&self) -> &SrcBase1 {
        &self.src_base1
    }
    #[doc = "0x110 - RGA source image Cr base address register"]
    #[inline(always)]
    pub const fn src_base2(&self) -> &SrcBase2 {
        &self.src_base2
    }
    #[doc = "0x114 - RGA source image 1 base address register"]
    #[inline(always)]
    pub const fn src_base3(&self) -> &SrcBase3 {
        &self.src_base3
    }
    #[doc = "0x118 - RGA source image virtual stride / RGA source image tile number"]
    #[inline(always)]
    pub const fn src_vir_info(&self) -> &SrcVirInfo {
        &self.src_vir_info
    }
    #[doc = "0x11c - RGA source image active width/height register"]
    #[inline(always)]
    pub const fn src_act_info(&self) -> &SrcActInfo {
        &self.src_act_info
    }
    #[doc = "0x120 - RGA source image horizontal scaling factor"]
    #[inline(always)]
    pub const fn src_x_factor(&self) -> &SrcXFactor {
        &self.src_x_factor
    }
    #[doc = "0x124 - RGA source image vertical scaling factor"]
    #[inline(always)]
    pub const fn src_y_factor(&self) -> &SrcYFactor {
        &self.src_y_factor
    }
    #[doc = "0x128 - RGA source image background color"]
    #[inline(always)]
    pub const fn src_bg_color(&self) -> &SrcBgColor {
        &self.src_bg_color
    }
    #[doc = "0x12c - RGA source image foreground color"]
    #[inline(always)]
    pub const fn src_fg_color(&self) -> &SrcFgColor {
        &self.src_fg_color
    }
    #[doc = "0x130 - RGA source image transparency color min value"]
    #[inline(always)]
    pub const fn cp_gr_a(&self) -> &CpGrA {
        unsafe { &*(self as *const Self).cast::<u8>().add(304).cast() }
    }
    #[doc = "0x130 - RGA source image transparency color min value"]
    #[inline(always)]
    pub const fn src_tr_color0(&self) -> &SrcTrColor0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(304).cast() }
    }
    #[doc = "0x134 - RGA source image transparency color max value"]
    #[inline(always)]
    pub const fn cp_gr_b(&self) -> &CpGrB {
        unsafe { &*(self as *const Self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x134 - Register0000 Abstract"]
    #[inline(always)]
    pub const fn src_tr_color1(&self) -> &SrcTrColor1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x138 - RGA destination format register"]
    #[inline(always)]
    pub const fn dst_info(&self) -> &DstInfo {
        &self.dst_info
    }
    #[doc = "0x13c - RGA destination image base address 0 register"]
    #[inline(always)]
    pub const fn dst_base0(&self) -> &DstBase0 {
        &self.dst_base0
    }
    #[doc = "0x140 - RGA destination image base address 1 register"]
    #[inline(always)]
    pub const fn dst_base1(&self) -> &DstBase1 {
        &self.dst_base1
    }
    #[doc = "0x144 - RGA destination image base address 2 register"]
    #[inline(always)]
    pub const fn dst_base2(&self) -> &DstBase2 {
        &self.dst_base2
    }
    #[doc = "0x148 - RGA destination image virtual width/height register"]
    #[inline(always)]
    pub const fn dst_vir_info(&self) -> &DstVirInfo {
        &self.dst_vir_info
    }
    #[doc = "0x14c - RGA destination image active width/height register"]
    #[inline(always)]
    pub const fn dst_act_info(&self) -> &DstActInfo {
        &self.dst_act_info
    }
    #[doc = "0x150 - Alpha control register 0"]
    #[inline(always)]
    pub const fn alpha_ctrl0(&self) -> &AlphaCtrl0 {
        &self.alpha_ctrl0
    }
    #[doc = "0x154 - Register0000 Abstract"]
    #[inline(always)]
    pub const fn alpha_ctrl1(&self) -> &AlphaCtrl1 {
        &self.alpha_ctrl1
    }
    #[doc = "0x158 - Fading control register"]
    #[inline(always)]
    pub const fn fading_ctrl(&self) -> &FadingCtrl {
        &self.fading_ctrl
    }
    #[doc = "0x15c - Pattern size/offset register"]
    #[inline(always)]
    pub const fn pat_con(&self) -> &PatCon {
        &self.pat_con
    }
    #[doc = "0x160 - RGA color gradient fill step register (color fill mode)"]
    #[inline(always)]
    pub const fn cp_gr_g(&self) -> &CpGrG {
        unsafe { &*(self as *const Self).cast::<u8>().add(352).cast() }
    }
    #[doc = "0x160 - ROP code 0 control register"]
    #[inline(always)]
    pub const fn rop_con0(&self) -> &RopCon0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(352).cast() }
    }
    #[doc = "0x164 - RGA color gradient fill step register (color fill mode)"]
    #[inline(always)]
    pub const fn cp_gr_r(&self) -> &CpGrR {
        unsafe { &*(self as *const Self).cast::<u8>().add(356).cast() }
    }
    #[doc = "0x164 - ROP code 1 control register"]
    #[inline(always)]
    pub const fn rop_con1(&self) -> &RopCon1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(356).cast() }
    }
    #[doc = "0x168 - RGA mask base address register"]
    #[inline(always)]
    pub const fn mask_base(&self) -> &MaskBase {
        &self.mask_base
    }
    #[doc = "0x16c - RGA MMU control register 1"]
    #[inline(always)]
    pub const fn mmu_ctrl1(&self) -> &MmuCtrl1 {
        &self.mmu_ctrl1
    }
    #[doc = "0x170 - RGA source MMU TLB base address"]
    #[inline(always)]
    pub const fn mmu_src_base(&self) -> &MmuSrcBase {
        &self.mmu_src_base
    }
    #[doc = "0x174 - RGA source1 MMU TLB base address"]
    #[inline(always)]
    pub const fn mmu_src1_base(&self) -> &MmuSrc1Base {
        &self.mmu_src1_base
    }
    #[doc = "0x178 - RGA destination MMU TLB base address"]
    #[inline(always)]
    pub const fn mmu_dst_base(&self) -> &MmuDstBase {
        &self.mmu_dst_base
    }
    #[doc = "0x17c - RGA ELSE MMU TLB base address"]
    #[inline(always)]
    pub const fn mmu_els_base(&self) -> &MmuElsBase {
        &self.mmu_els_base
    }
}
#[doc = "SYS_CTRL (rw) register accessor: RGA system control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_ctrl`]
module"]
#[doc(alias = "SYS_CTRL")]
pub type SysCtrl = crate::Reg<sys_ctrl::SysCtrlSpec>;
#[doc = "RGA system control register"]
pub mod sys_ctrl;
#[doc = "CMD_CTRL (rw) register accessor: RGA command control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_ctrl`]
module"]
#[doc(alias = "CMD_CTRL")]
pub type CmdCtrl = crate::Reg<cmd_ctrl::CmdCtrlSpec>;
#[doc = "RGA command control register"]
pub mod cmd_ctrl;
#[doc = "CMD_BASE (rw) register accessor: RGA command codes base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_base`]
module"]
#[doc(alias = "CMD_BASE")]
pub type CmdBase = crate::Reg<cmd_base::CmdBaseSpec>;
#[doc = "RGA command codes base address register"]
pub mod cmd_base;
#[doc = "STATUS1 (r) register accessor: RGA status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1`]
module"]
#[doc(alias = "STATUS1")]
pub type Status1 = crate::Reg<status1::Status1Spec>;
#[doc = "RGA status register"]
pub mod status1;
#[doc = "INT (rw) register accessor: RGA interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int`]
module"]
#[doc(alias = "INT")]
pub type Int = crate::Reg<int::IntSpec>;
#[doc = "RGA interrupt register"]
pub mod int;
#[doc = "MMU_CTRL0 (rw) register accessor: RGA MMU control 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_ctrl0`]
module"]
#[doc(alias = "MMU_CTRL0")]
pub type MmuCtrl0 = crate::Reg<mmu_ctrl0::MmuCtrl0Spec>;
#[doc = "RGA MMU control 0 register"]
pub mod mmu_ctrl0;
#[doc = "MMU_CMD_BASE (rw) register accessor: Register0000 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_cmd_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_cmd_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_cmd_base`]
module"]
#[doc(alias = "MMU_CMD_BASE")]
pub type MmuCmdBase = crate::Reg<mmu_cmd_base::MmuCmdBaseSpec>;
#[doc = "Register0000 Abstract"]
pub mod mmu_cmd_base;
#[doc = "STATUS2 (r) register accessor: RGA status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status2`]
module"]
#[doc(alias = "STATUS2")]
pub type Status2 = crate::Reg<status2::Status2Spec>;
#[doc = "RGA status register"]
pub mod status2;
#[doc = "WORK_CNT (r) register accessor: work counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`work_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@work_cnt`]
module"]
#[doc(alias = "WORK_CNT")]
pub type WorkCnt = crate::Reg<work_cnt::WorkCntSpec>;
#[doc = "work counter"]
pub mod work_cnt;
#[doc = "VERSION_INFO (rw) register accessor: Version number for rga\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version_info`]
module"]
#[doc(alias = "VERSION_INFO")]
pub type VersionInfo = crate::Reg<version_info::VersionInfoSpec>;
#[doc = "Version number for rga"]
pub mod version_info;
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
#[doc = "PERF_RD_AXI_TOTAL_BYTE (rw) register accessor: perf_rd_axi_total_byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perf_rd_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perf_rd_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perf_rd_axi_total_byte`]
module"]
#[doc(alias = "PERF_RD_AXI_TOTAL_BYTE")]
pub type PerfRdAxiTotalByte = crate::Reg<perf_rd_axi_total_byte::PerfRdAxiTotalByteSpec>;
#[doc = "perf_rd_axi_total_byte"]
pub mod perf_rd_axi_total_byte;
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
#[doc = "MODE_CTRL (rw) register accessor: RGA mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode_ctrl`]
module"]
#[doc(alias = "MODE_CTRL")]
pub type ModeCtrl = crate::Reg<mode_ctrl::ModeCtrlSpec>;
#[doc = "RGA mode control register"]
pub mod mode_ctrl;
#[doc = "SRC_INFO (rw) register accessor: RGA source information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_info`]
module"]
#[doc(alias = "SRC_INFO")]
pub type SrcInfo = crate::Reg<src_info::SrcInfoSpec>;
#[doc = "RGA source information register"]
pub mod src_info;
#[doc = "SRC_BASE0 (rw) register accessor: source image Y/RGB base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_base0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_base0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_base0`]
module"]
#[doc(alias = "SRC_BASE0")]
pub type SrcBase0 = crate::Reg<src_base0::SrcBase0Spec>;
#[doc = "source image Y/RGB base address"]
pub mod src_base0;
#[doc = "SRC_BASE1 (rw) register accessor: RGA source image Cb/Cbr base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_base1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_base1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_base1`]
module"]
#[doc(alias = "SRC_BASE1")]
pub type SrcBase1 = crate::Reg<src_base1::SrcBase1Spec>;
#[doc = "RGA source image Cb/Cbr base address register"]
pub mod src_base1;
#[doc = "SRC_BASE2 (rw) register accessor: RGA source image Cr base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_base2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_base2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_base2`]
module"]
#[doc(alias = "SRC_BASE2")]
pub type SrcBase2 = crate::Reg<src_base2::SrcBase2Spec>;
#[doc = "RGA source image Cr base address register"]
pub mod src_base2;
#[doc = "SRC_BASE3 (rw) register accessor: RGA source image 1 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_base3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_base3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_base3`]
module"]
#[doc(alias = "SRC_BASE3")]
pub type SrcBase3 = crate::Reg<src_base3::SrcBase3Spec>;
#[doc = "RGA source image 1 base address register"]
pub mod src_base3;
#[doc = "SRC_VIR_INFO (rw) register accessor: RGA source image virtual stride / RGA source image tile number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_vir_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_vir_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_vir_info`]
module"]
#[doc(alias = "SRC_VIR_INFO")]
pub type SrcVirInfo = crate::Reg<src_vir_info::SrcVirInfoSpec>;
#[doc = "RGA source image virtual stride / RGA source image tile number"]
pub mod src_vir_info;
#[doc = "SRC_ACT_INFO (rw) register accessor: RGA source image active width/height register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_act_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_act_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_act_info`]
module"]
#[doc(alias = "SRC_ACT_INFO")]
pub type SrcActInfo = crate::Reg<src_act_info::SrcActInfoSpec>;
#[doc = "RGA source image active width/height register"]
pub mod src_act_info;
#[doc = "SRC_X_FACTOR (rw) register accessor: RGA source image horizontal scaling factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_x_factor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_x_factor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_x_factor`]
module"]
#[doc(alias = "SRC_X_FACTOR")]
pub type SrcXFactor = crate::Reg<src_x_factor::SrcXFactorSpec>;
#[doc = "RGA source image horizontal scaling factor"]
pub mod src_x_factor;
#[doc = "SRC_Y_FACTOR (rw) register accessor: RGA source image vertical scaling factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_y_factor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_y_factor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_y_factor`]
module"]
#[doc(alias = "SRC_Y_FACTOR")]
pub type SrcYFactor = crate::Reg<src_y_factor::SrcYFactorSpec>;
#[doc = "RGA source image vertical scaling factor"]
pub mod src_y_factor;
#[doc = "SRC_BG_COLOR (rw) register accessor: RGA source image background color\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_bg_color::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_bg_color::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_bg_color`]
module"]
#[doc(alias = "SRC_BG_COLOR")]
pub type SrcBgColor = crate::Reg<src_bg_color::SrcBgColorSpec>;
#[doc = "RGA source image background color"]
pub mod src_bg_color;
#[doc = "SRC_FG_COLOR (rw) register accessor: RGA source image foreground color\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_fg_color::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_fg_color::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_fg_color`]
module"]
#[doc(alias = "SRC_FG_COLOR")]
pub type SrcFgColor = crate::Reg<src_fg_color::SrcFgColorSpec>;
#[doc = "RGA source image foreground color"]
pub mod src_fg_color;
#[doc = "SRC_TR_COLOR0 (rw) register accessor: RGA source image transparency color min value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_tr_color0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_tr_color0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_tr_color0`]
module"]
#[doc(alias = "SRC_TR_COLOR0")]
pub type SrcTrColor0 = crate::Reg<src_tr_color0::SrcTrColor0Spec>;
#[doc = "RGA source image transparency color min value"]
pub mod src_tr_color0;
#[doc = "CP_GR_A (rw) register accessor: RGA source image transparency color min value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cp_gr_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cp_gr_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cp_gr_a`]
module"]
#[doc(alias = "CP_GR_A")]
pub type CpGrA = crate::Reg<cp_gr_a::CpGrASpec>;
#[doc = "RGA source image transparency color min value"]
pub mod cp_gr_a;
#[doc = "SRC_TR_COLOR1 (rw) register accessor: Register0000 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_tr_color1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_tr_color1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_tr_color1`]
module"]
#[doc(alias = "SRC_TR_COLOR1")]
pub type SrcTrColor1 = crate::Reg<src_tr_color1::SrcTrColor1Spec>;
#[doc = "Register0000 Abstract"]
pub mod src_tr_color1;
#[doc = "CP_GR_B (rw) register accessor: RGA source image transparency color max value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cp_gr_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cp_gr_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cp_gr_b`]
module"]
#[doc(alias = "CP_GR_B")]
pub type CpGrB = crate::Reg<cp_gr_b::CpGrBSpec>;
#[doc = "RGA source image transparency color max value"]
pub mod cp_gr_b;
#[doc = "DST_INFO (rw) register accessor: RGA destination format register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_info`]
module"]
#[doc(alias = "DST_INFO")]
pub type DstInfo = crate::Reg<dst_info::DstInfoSpec>;
#[doc = "RGA destination format register"]
pub mod dst_info;
#[doc = "DST_BASE0 (rw) register accessor: RGA destination image base address 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_base0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_base0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_base0`]
module"]
#[doc(alias = "DST_BASE0")]
pub type DstBase0 = crate::Reg<dst_base0::DstBase0Spec>;
#[doc = "RGA destination image base address 0 register"]
pub mod dst_base0;
#[doc = "DST_BASE1 (rw) register accessor: RGA destination image base address 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_base1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_base1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_base1`]
module"]
#[doc(alias = "DST_BASE1")]
pub type DstBase1 = crate::Reg<dst_base1::DstBase1Spec>;
#[doc = "RGA destination image base address 1 register"]
pub mod dst_base1;
#[doc = "DST_BASE2 (rw) register accessor: RGA destination image base address 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_base2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_base2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_base2`]
module"]
#[doc(alias = "DST_BASE2")]
pub type DstBase2 = crate::Reg<dst_base2::DstBase2Spec>;
#[doc = "RGA destination image base address 2 register"]
pub mod dst_base2;
#[doc = "DST_VIR_INFO (rw) register accessor: RGA destination image virtual width/height register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_vir_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_vir_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_vir_info`]
module"]
#[doc(alias = "DST_VIR_INFO")]
pub type DstVirInfo = crate::Reg<dst_vir_info::DstVirInfoSpec>;
#[doc = "RGA destination image virtual width/height register"]
pub mod dst_vir_info;
#[doc = "DST_ACT_INFO (rw) register accessor: RGA destination image active width/height register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_act_info::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_act_info::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_act_info`]
module"]
#[doc(alias = "DST_ACT_INFO")]
pub type DstActInfo = crate::Reg<dst_act_info::DstActInfoSpec>;
#[doc = "RGA destination image active width/height register"]
pub mod dst_act_info;
#[doc = "ALPHA_CTRL0 (rw) register accessor: Alpha control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alpha_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alpha_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alpha_ctrl0`]
module"]
#[doc(alias = "ALPHA_CTRL0")]
pub type AlphaCtrl0 = crate::Reg<alpha_ctrl0::AlphaCtrl0Spec>;
#[doc = "Alpha control register 0"]
pub mod alpha_ctrl0;
#[doc = "ALPHA_CTRL1 (rw) register accessor: Register0000 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alpha_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alpha_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alpha_ctrl1`]
module"]
#[doc(alias = "ALPHA_CTRL1")]
pub type AlphaCtrl1 = crate::Reg<alpha_ctrl1::AlphaCtrl1Spec>;
#[doc = "Register0000 Abstract"]
pub mod alpha_ctrl1;
#[doc = "FADING_CTRL (rw) register accessor: Fading control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fading_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fading_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fading_ctrl`]
module"]
#[doc(alias = "FADING_CTRL")]
pub type FadingCtrl = crate::Reg<fading_ctrl::FadingCtrlSpec>;
#[doc = "Fading control register"]
pub mod fading_ctrl;
#[doc = "PAT_CON (rw) register accessor: Pattern size/offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pat_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pat_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pat_con`]
module"]
#[doc(alias = "PAT_CON")]
pub type PatCon = crate::Reg<pat_con::PatConSpec>;
#[doc = "Pattern size/offset register"]
pub mod pat_con;
#[doc = "ROP_CON0 (rw) register accessor: ROP code 0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rop_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rop_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rop_con0`]
module"]
#[doc(alias = "ROP_CON0")]
pub type RopCon0 = crate::Reg<rop_con0::RopCon0Spec>;
#[doc = "ROP code 0 control register"]
pub mod rop_con0;
#[doc = "CP_GR_G (rw) register accessor: RGA color gradient fill step register (color fill mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cp_gr_g::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cp_gr_g::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cp_gr_g`]
module"]
#[doc(alias = "CP_GR_G")]
pub type CpGrG = crate::Reg<cp_gr_g::CpGrGSpec>;
#[doc = "RGA color gradient fill step register (color fill mode)"]
pub mod cp_gr_g;
#[doc = "ROP_CON1 (rw) register accessor: ROP code 1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rop_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rop_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rop_con1`]
module"]
#[doc(alias = "ROP_CON1")]
pub type RopCon1 = crate::Reg<rop_con1::RopCon1Spec>;
#[doc = "ROP code 1 control register"]
pub mod rop_con1;
#[doc = "CP_GR_R (rw) register accessor: RGA color gradient fill step register (color fill mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cp_gr_r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cp_gr_r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cp_gr_r`]
module"]
#[doc(alias = "CP_GR_R")]
pub type CpGrR = crate::Reg<cp_gr_r::CpGrRSpec>;
#[doc = "RGA color gradient fill step register (color fill mode)"]
pub mod cp_gr_r;
#[doc = "MASK_BASE (rw) register accessor: RGA mask base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_base`]
module"]
#[doc(alias = "MASK_BASE")]
pub type MaskBase = crate::Reg<mask_base::MaskBaseSpec>;
#[doc = "RGA mask base address register"]
pub mod mask_base;
#[doc = "MMU_CTRL1 (rw) register accessor: RGA MMU control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_ctrl1`]
module"]
#[doc(alias = "MMU_CTRL1")]
pub type MmuCtrl1 = crate::Reg<mmu_ctrl1::MmuCtrl1Spec>;
#[doc = "RGA MMU control register 1"]
pub mod mmu_ctrl1;
#[doc = "MMU_SRC_BASE (rw) register accessor: RGA source MMU TLB base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_src_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_src_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_src_base`]
module"]
#[doc(alias = "MMU_SRC_BASE")]
pub type MmuSrcBase = crate::Reg<mmu_src_base::MmuSrcBaseSpec>;
#[doc = "RGA source MMU TLB base address"]
pub mod mmu_src_base;
#[doc = "MMU_SRC1_BASE (rw) register accessor: RGA source1 MMU TLB base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_src1_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_src1_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_src1_base`]
module"]
#[doc(alias = "MMU_SRC1_BASE")]
pub type MmuSrc1Base = crate::Reg<mmu_src1_base::MmuSrc1BaseSpec>;
#[doc = "RGA source1 MMU TLB base address"]
pub mod mmu_src1_base;
#[doc = "MMU_DST_BASE (rw) register accessor: RGA destination MMU TLB base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_dst_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_dst_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_dst_base`]
module"]
#[doc(alias = "MMU_DST_BASE")]
pub type MmuDstBase = crate::Reg<mmu_dst_base::MmuDstBaseSpec>;
#[doc = "RGA destination MMU TLB base address"]
pub mod mmu_dst_base;
#[doc = "MMU_ELS_BASE (rw) register accessor: RGA ELSE MMU TLB base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmu_els_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmu_els_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmu_els_base`]
module"]
#[doc(alias = "MMU_ELS_BASE")]
pub type MmuElsBase = crate::Reg<mmu_els_base::MmuElsBaseSpec>;
#[doc = "RGA ELSE MMU TLB base address"]
pub mod mmu_els_base;
