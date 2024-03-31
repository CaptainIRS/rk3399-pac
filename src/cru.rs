#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lpll_con0: LpllCon0,
    lpll_con1: LpllCon1,
    lpll_con2: LpllCon2,
    lpll_con3: LpllCon3,
    lpll_con4: LpllCon4,
    lpll_con5: LpllCon5,
    _reserved6: [u8; 0x08],
    bpll_con0: BpllCon0,
    bpll_con1: BpllCon1,
    bpll_con2: BpllCon2,
    bpll_con3: BpllCon3,
    bpll_con4: BpllCon4,
    bpll_con5: BpllCon5,
    _reserved12: [u8; 0x08],
    dpll_con0: DpllCon0,
    dpll_con1: DpllCon1,
    dpll_con2: DpllCon2,
    dpll_con3: DpllCon3,
    dpll_con4: DpllCon4,
    dpll_con5: DpllCon5,
    _reserved18: [u8; 0x08],
    cpll_con0: CpllCon0,
    cpll_con1: CpllCon1,
    cpll_con2: CpllCon2,
    cpll_con3: CpllCon3,
    cpll_con4: CpllCon4,
    cpll_con5: CpllCon5,
    _reserved24: [u8; 0x08],
    gpll_con0: GpllCon0,
    gpll_con1: GpllCon1,
    gpll_con2: GpllCon2,
    gpll_con3: GpllCon3,
    gpll_con4: GpllCon4,
    gpll_con5: GpllCon5,
    _reserved30: [u8; 0x08],
    npll_con0: NpllCon0,
    npll_con1: NpllCon1,
    npll_con2: NpllCon2,
    npll_con3: NpllCon3,
    npll_con4: NpllCon4,
    npll_con5: NpllCon5,
    _reserved36: [u8; 0x08],
    vpll_con0: VpllCon0,
    vpll_con1: VpllCon1,
    vpll_con2: VpllCon2,
    vpll_con3: VpllCon3,
    vpll_con4: VpllCon4,
    vpll_con5: VpllCon5,
    _reserved42: [u8; 0x28],
    clksel_con0: ClkselCon0,
    clksel_con1: ClkselCon1,
    clksel_con2: ClkselCon2,
    clksel_con3: ClkselCon3,
    clksel_con4: ClkselCon4,
    clksel_con5: ClkselCon5,
    clksel_con6: ClkselCon6,
    clksel_con7: ClkselCon7,
    clksel_con8: ClkselCon8,
    clksel_con9: ClkselCon9,
    clksel_con10: ClkselCon10,
    clksel_con11: ClkselCon11,
    clksel_con12: ClkselCon12,
    clksel_con13: ClkselCon13,
    clksel_con14: ClkselCon14,
    clksel_con15: ClkselCon15,
    clksel_con16: ClkselCon16,
    clksel_con17: ClkselCon17,
    clksel_con18: ClkselCon18,
    clksel_con19: ClkselCon19,
    clksel_con20: ClkselCon20,
    clksel_con21: ClkselCon21,
    clksel_con22: ClkselCon22,
    clksel_con23: ClkselCon23,
    clksel_con24: ClkselCon24,
    clksel_con25: ClkselCon25,
    clksel_con26: ClkselCon26,
    clksel_con27: ClkselCon27,
    clksel_con28: ClkselCon28,
    clksel_con29: ClkselCon29,
    clksel_con30: ClkselCon30,
    clksel_con31: ClkselCon31,
    clksel_con32: ClkselCon32,
    clksel_con33: ClkselCon33,
    clksel_con34: ClkselCon34,
    clksel_con35: ClkselCon35,
    clksel_con36: ClkselCon36,
    _reserved79: [u8; 0x04],
    clksel_con38: ClkselCon38,
    clksel_con39: ClkselCon39,
    clksel_con40: ClkselCon40,
    clksel_con41: ClkselCon41,
    clksel_con42: ClkselCon42,
    clksel_con43: ClkselCon43,
    clksel_con44: ClkselCon44,
    clksel_con45: ClkselCon45,
    clksel_con46: ClkselCon46,
    clksel_con47: ClkselCon47,
    clksel_con48: ClkselCon48,
    clksel_con49: ClkselCon49,
    clksel_con50: ClkselCon50,
    clksel_con51: ClkselCon51,
    clksel_con52: ClkselCon52,
    clksel_con53: ClkselCon53,
    clksel_con54: ClkselCon54,
    clksel_con55: ClkselCon55,
    clksel_con56: ClkselCon56,
    clksel_con57: ClkselCon57,
    clksel_con58: ClkselCon58,
    clksel_con59: ClkselCon59,
    clksel_con60: ClkselCon60,
    clksel_con61: ClkselCon61,
    clksel_con62: ClkselCon62,
    clksel_con63: ClkselCon63,
    clksel_con64: ClkselCon64,
    clksel_con65: ClkselCon65,
    _reserved107: [u8; 0x78],
    clksel_con96: ClkselCon96,
    clksel_con97: ClkselCon97,
    clksel_con98: ClkselCon98,
    clksel_con99: ClkselCon99,
    clksel_con100: ClkselCon100,
    clksel_con101: ClkselCon101,
    clksel_con102: ClkselCon102,
    clksel_con103: ClkselCon103,
    _reserved115: [u8; 0x04],
    clksel_con105: ClkselCon105,
    clksel_con106: ClkselCon106,
    clksel_con107: ClkselCon107,
    _reserved118: [u8; 0x50],
    clkgate_con0: ClkgateCon0,
    clkgate_con1: ClkgateCon1,
    clkgate_con2: ClkgateCon2,
    clkgate_con3: ClkgateCon3,
    clkgate_con4: ClkgateCon4,
    clkgate_con5: ClkgateCon5,
    clkgate_con6: ClkgateCon6,
    clkgate_con7: ClkgateCon7,
    clkgate_con8: ClkgateCon8,
    clkgate_con9: ClkgateCon9,
    clkgate_con10: ClkgateCon10,
    clkgate_con11: ClkgateCon11,
    clkgate_con12: ClkgateCon12,
    clkgate_con13: ClkgateCon13,
    clkgate_con14: ClkgateCon14,
    clkgate_con15: ClkgateCon15,
    clkgate_con16: ClkgateCon16,
    clkgate_con17: ClkgateCon17,
    clkgate_con18: ClkgateCon18,
    clkgate_con19: ClkgateCon19,
    clkgate_con20: ClkgateCon20,
    clkgate_con21: ClkgateCon21,
    clkgate_con22: ClkgateCon22,
    clkgate_con23: ClkgateCon23,
    clkgate_con24: ClkgateCon24,
    clkgate_con25: ClkgateCon25,
    clkgate_con26: ClkgateCon26,
    clkgate_con27: ClkgateCon27,
    clkgate_con28: ClkgateCon28,
    clkgate_con29: ClkgateCon29,
    clkgate_con30: ClkgateCon30,
    clkgate_con31: ClkgateCon31,
    clkgate_con32: ClkgateCon32,
    clkgate_con33: ClkgateCon33,
    clkgate_con34: ClkgateCon34,
    _reserved153: [u8; 0x74],
    softrst_con0: SoftrstCon0,
    softrst_con1: SoftrstCon1,
    softrst_con2: SoftrstCon2,
    softrst_con3: SoftrstCon3,
    softrst_con4: SoftrstCon4,
    softrst_con5: SoftrstCon5,
    softrst_con6: SoftrstCon6,
    softrst_con7: SoftrstCon7,
    softrst_con8: SoftrstCon8,
    softrst_con9: SoftrstCon9,
    softrst_con10: SoftrstCon10,
    softrst_con11: SoftrstCon11,
    softrst_con12: SoftrstCon12,
    softrst_con13: SoftrstCon13,
    softrst_con14: SoftrstCon14,
    softrst_con15: SoftrstCon15,
    softrst_con16: SoftrstCon16,
    softrst_con17: SoftrstCon17,
    softrst_con18: SoftrstCon18,
    softrst_con19: SoftrstCon19,
    softrst_con20: SoftrstCon20,
    _reserved174: [u8; 0xac],
    glb_srst_fst_value: GlbSrstFstValue,
    glb_srst_snd_value: GlbSrstSndValue,
    glb_cnt_th: GlbCntTh,
    misc_con: MiscCon,
    glb_rst_con: GlbRstCon,
    glb_rst_st: GlbRstSt,
    _reserved180: [u8; 0x68],
    sdmmc_con0: SdmmcCon0,
    sdmmc_con1: SdmmcCon1,
    sdio0_con0: Sdio0Con0,
    sdio0_con1: Sdio0Con1,
}
impl RegisterBlock {
    #[doc = "0x00 - LPLL configuration register0"]
    #[inline(always)]
    pub const fn lpll_con0(&self) -> &LpllCon0 {
        &self.lpll_con0
    }
    #[doc = "0x04 - LPLL configuration register1"]
    #[inline(always)]
    pub const fn lpll_con1(&self) -> &LpllCon1 {
        &self.lpll_con1
    }
    #[doc = "0x08 - LPLL configuration register2"]
    #[inline(always)]
    pub const fn lpll_con2(&self) -> &LpllCon2 {
        &self.lpll_con2
    }
    #[doc = "0x0c - LPLL configuration register3"]
    #[inline(always)]
    pub const fn lpll_con3(&self) -> &LpllCon3 {
        &self.lpll_con3
    }
    #[doc = "0x10 - LPLL configuration register4"]
    #[inline(always)]
    pub const fn lpll_con4(&self) -> &LpllCon4 {
        &self.lpll_con4
    }
    #[doc = "0x14 - LPLL configuration register5"]
    #[inline(always)]
    pub const fn lpll_con5(&self) -> &LpllCon5 {
        &self.lpll_con5
    }
    #[doc = "0x20 - BPLL configuration register0"]
    #[inline(always)]
    pub const fn bpll_con0(&self) -> &BpllCon0 {
        &self.bpll_con0
    }
    #[doc = "0x24 - BPLL configuration register1"]
    #[inline(always)]
    pub const fn bpll_con1(&self) -> &BpllCon1 {
        &self.bpll_con1
    }
    #[doc = "0x28 - BPLL configuration register2"]
    #[inline(always)]
    pub const fn bpll_con2(&self) -> &BpllCon2 {
        &self.bpll_con2
    }
    #[doc = "0x2c - BPLL configuration register3"]
    #[inline(always)]
    pub const fn bpll_con3(&self) -> &BpllCon3 {
        &self.bpll_con3
    }
    #[doc = "0x30 - BPLL configuration register4"]
    #[inline(always)]
    pub const fn bpll_con4(&self) -> &BpllCon4 {
        &self.bpll_con4
    }
    #[doc = "0x34 - BPLL configuration register5"]
    #[inline(always)]
    pub const fn bpll_con5(&self) -> &BpllCon5 {
        &self.bpll_con5
    }
    #[doc = "0x40 - DPLL configuration register0"]
    #[inline(always)]
    pub const fn dpll_con0(&self) -> &DpllCon0 {
        &self.dpll_con0
    }
    #[doc = "0x44 - DPLL configuration register1"]
    #[inline(always)]
    pub const fn dpll_con1(&self) -> &DpllCon1 {
        &self.dpll_con1
    }
    #[doc = "0x48 - DPLL configuration register2"]
    #[inline(always)]
    pub const fn dpll_con2(&self) -> &DpllCon2 {
        &self.dpll_con2
    }
    #[doc = "0x4c - DPLL configuration register3"]
    #[inline(always)]
    pub const fn dpll_con3(&self) -> &DpllCon3 {
        &self.dpll_con3
    }
    #[doc = "0x50 - DPLL configuration register4"]
    #[inline(always)]
    pub const fn dpll_con4(&self) -> &DpllCon4 {
        &self.dpll_con4
    }
    #[doc = "0x54 - DPLL configuration register5"]
    #[inline(always)]
    pub const fn dpll_con5(&self) -> &DpllCon5 {
        &self.dpll_con5
    }
    #[doc = "0x60 - CPLL configuration register0"]
    #[inline(always)]
    pub const fn cpll_con0(&self) -> &CpllCon0 {
        &self.cpll_con0
    }
    #[doc = "0x64 - CPLL configuration register1"]
    #[inline(always)]
    pub const fn cpll_con1(&self) -> &CpllCon1 {
        &self.cpll_con1
    }
    #[doc = "0x68 - CPLL configuration register2"]
    #[inline(always)]
    pub const fn cpll_con2(&self) -> &CpllCon2 {
        &self.cpll_con2
    }
    #[doc = "0x6c - CPLL configuration register3"]
    #[inline(always)]
    pub const fn cpll_con3(&self) -> &CpllCon3 {
        &self.cpll_con3
    }
    #[doc = "0x70 - CPLL configuration register4"]
    #[inline(always)]
    pub const fn cpll_con4(&self) -> &CpllCon4 {
        &self.cpll_con4
    }
    #[doc = "0x74 - CPLL configuration register5"]
    #[inline(always)]
    pub const fn cpll_con5(&self) -> &CpllCon5 {
        &self.cpll_con5
    }
    #[doc = "0x80 - GPLL configuration register0"]
    #[inline(always)]
    pub const fn gpll_con0(&self) -> &GpllCon0 {
        &self.gpll_con0
    }
    #[doc = "0x84 - GPLL configuration register1"]
    #[inline(always)]
    pub const fn gpll_con1(&self) -> &GpllCon1 {
        &self.gpll_con1
    }
    #[doc = "0x88 - GPLL configuration register2"]
    #[inline(always)]
    pub const fn gpll_con2(&self) -> &GpllCon2 {
        &self.gpll_con2
    }
    #[doc = "0x8c - GPLL configuration register3"]
    #[inline(always)]
    pub const fn gpll_con3(&self) -> &GpllCon3 {
        &self.gpll_con3
    }
    #[doc = "0x90 - GPLL configuration register4"]
    #[inline(always)]
    pub const fn gpll_con4(&self) -> &GpllCon4 {
        &self.gpll_con4
    }
    #[doc = "0x94 - GPLL configuration register5"]
    #[inline(always)]
    pub const fn gpll_con5(&self) -> &GpllCon5 {
        &self.gpll_con5
    }
    #[doc = "0xa0 - NPLL configuration register0"]
    #[inline(always)]
    pub const fn npll_con0(&self) -> &NpllCon0 {
        &self.npll_con0
    }
    #[doc = "0xa4 - NPLL configuration register1"]
    #[inline(always)]
    pub const fn npll_con1(&self) -> &NpllCon1 {
        &self.npll_con1
    }
    #[doc = "0xa8 - NPLL configuration register2"]
    #[inline(always)]
    pub const fn npll_con2(&self) -> &NpllCon2 {
        &self.npll_con2
    }
    #[doc = "0xac - NPLL configuration register3"]
    #[inline(always)]
    pub const fn npll_con3(&self) -> &NpllCon3 {
        &self.npll_con3
    }
    #[doc = "0xb0 - NPLL configuration register4"]
    #[inline(always)]
    pub const fn npll_con4(&self) -> &NpllCon4 {
        &self.npll_con4
    }
    #[doc = "0xb4 - NPLL configuration register5"]
    #[inline(always)]
    pub const fn npll_con5(&self) -> &NpllCon5 {
        &self.npll_con5
    }
    #[doc = "0xc0 - VPLL configuration register0"]
    #[inline(always)]
    pub const fn vpll_con0(&self) -> &VpllCon0 {
        &self.vpll_con0
    }
    #[doc = "0xc4 - VPLL configuration register1"]
    #[inline(always)]
    pub const fn vpll_con1(&self) -> &VpllCon1 {
        &self.vpll_con1
    }
    #[doc = "0xc8 - VPLL configuration register2"]
    #[inline(always)]
    pub const fn vpll_con2(&self) -> &VpllCon2 {
        &self.vpll_con2
    }
    #[doc = "0xcc - VPLL configuration register3"]
    #[inline(always)]
    pub const fn vpll_con3(&self) -> &VpllCon3 {
        &self.vpll_con3
    }
    #[doc = "0xd0 - VPLL configuration register4"]
    #[inline(always)]
    pub const fn vpll_con4(&self) -> &VpllCon4 {
        &self.vpll_con4
    }
    #[doc = "0xd4 - VPLL configuration register5"]
    #[inline(always)]
    pub const fn vpll_con5(&self) -> &VpllCon5 {
        &self.vpll_con5
    }
    #[doc = "0x100 - Internal clock select and divide register0"]
    #[inline(always)]
    pub const fn clksel_con0(&self) -> &ClkselCon0 {
        &self.clksel_con0
    }
    #[doc = "0x104 - Internal clock select and divide register1"]
    #[inline(always)]
    pub const fn clksel_con1(&self) -> &ClkselCon1 {
        &self.clksel_con1
    }
    #[doc = "0x108 - Internal clock select and divide register2"]
    #[inline(always)]
    pub const fn clksel_con2(&self) -> &ClkselCon2 {
        &self.clksel_con2
    }
    #[doc = "0x10c - Internal clock select and divide register3"]
    #[inline(always)]
    pub const fn clksel_con3(&self) -> &ClkselCon3 {
        &self.clksel_con3
    }
    #[doc = "0x110 - Internal clock select and divide register4"]
    #[inline(always)]
    pub const fn clksel_con4(&self) -> &ClkselCon4 {
        &self.clksel_con4
    }
    #[doc = "0x114 - Internal clock select and divide register5"]
    #[inline(always)]
    pub const fn clksel_con5(&self) -> &ClkselCon5 {
        &self.clksel_con5
    }
    #[doc = "0x118 - Internal clock select and divide register6"]
    #[inline(always)]
    pub const fn clksel_con6(&self) -> &ClkselCon6 {
        &self.clksel_con6
    }
    #[doc = "0x11c - Internal clock select and divide register7"]
    #[inline(always)]
    pub const fn clksel_con7(&self) -> &ClkselCon7 {
        &self.clksel_con7
    }
    #[doc = "0x120 - Internal clock select and divide register8"]
    #[inline(always)]
    pub const fn clksel_con8(&self) -> &ClkselCon8 {
        &self.clksel_con8
    }
    #[doc = "0x124 - Internal clock select and divide register9"]
    #[inline(always)]
    pub const fn clksel_con9(&self) -> &ClkselCon9 {
        &self.clksel_con9
    }
    #[doc = "0x128 - Internal clock select and divide register10"]
    #[inline(always)]
    pub const fn clksel_con10(&self) -> &ClkselCon10 {
        &self.clksel_con10
    }
    #[doc = "0x12c - Internal clock select and divide register11"]
    #[inline(always)]
    pub const fn clksel_con11(&self) -> &ClkselCon11 {
        &self.clksel_con11
    }
    #[doc = "0x130 - Internal clock select and divide register12"]
    #[inline(always)]
    pub const fn clksel_con12(&self) -> &ClkselCon12 {
        &self.clksel_con12
    }
    #[doc = "0x134 - Internal clock select and divide register13"]
    #[inline(always)]
    pub const fn clksel_con13(&self) -> &ClkselCon13 {
        &self.clksel_con13
    }
    #[doc = "0x138 - Internal clock select and divide register14"]
    #[inline(always)]
    pub const fn clksel_con14(&self) -> &ClkselCon14 {
        &self.clksel_con14
    }
    #[doc = "0x13c - Internal clock select and divide register15"]
    #[inline(always)]
    pub const fn clksel_con15(&self) -> &ClkselCon15 {
        &self.clksel_con15
    }
    #[doc = "0x140 - Internal clock select and divide register16"]
    #[inline(always)]
    pub const fn clksel_con16(&self) -> &ClkselCon16 {
        &self.clksel_con16
    }
    #[doc = "0x144 - Internal clock select and divide register17"]
    #[inline(always)]
    pub const fn clksel_con17(&self) -> &ClkselCon17 {
        &self.clksel_con17
    }
    #[doc = "0x148 - Internal clock select and divide register18"]
    #[inline(always)]
    pub const fn clksel_con18(&self) -> &ClkselCon18 {
        &self.clksel_con18
    }
    #[doc = "0x14c - Internal clock select and divide register19"]
    #[inline(always)]
    pub const fn clksel_con19(&self) -> &ClkselCon19 {
        &self.clksel_con19
    }
    #[doc = "0x150 - Internal clock select and divide register20"]
    #[inline(always)]
    pub const fn clksel_con20(&self) -> &ClkselCon20 {
        &self.clksel_con20
    }
    #[doc = "0x154 - Internal clock select and divide register21"]
    #[inline(always)]
    pub const fn clksel_con21(&self) -> &ClkselCon21 {
        &self.clksel_con21
    }
    #[doc = "0x158 - Internal clock select and divide register22"]
    #[inline(always)]
    pub const fn clksel_con22(&self) -> &ClkselCon22 {
        &self.clksel_con22
    }
    #[doc = "0x15c - Internal clock select and divide register23"]
    #[inline(always)]
    pub const fn clksel_con23(&self) -> &ClkselCon23 {
        &self.clksel_con23
    }
    #[doc = "0x160 - Internal clock select and divide register24"]
    #[inline(always)]
    pub const fn clksel_con24(&self) -> &ClkselCon24 {
        &self.clksel_con24
    }
    #[doc = "0x164 - Internal clock select and divide register25"]
    #[inline(always)]
    pub const fn clksel_con25(&self) -> &ClkselCon25 {
        &self.clksel_con25
    }
    #[doc = "0x168 - Internal clock select and divide register26"]
    #[inline(always)]
    pub const fn clksel_con26(&self) -> &ClkselCon26 {
        &self.clksel_con26
    }
    #[doc = "0x16c - Internal clock select and divide register27"]
    #[inline(always)]
    pub const fn clksel_con27(&self) -> &ClkselCon27 {
        &self.clksel_con27
    }
    #[doc = "0x170 - Internal clock select and divide register28"]
    #[inline(always)]
    pub const fn clksel_con28(&self) -> &ClkselCon28 {
        &self.clksel_con28
    }
    #[doc = "0x174 - Internal clock select and divide register29"]
    #[inline(always)]
    pub const fn clksel_con29(&self) -> &ClkselCon29 {
        &self.clksel_con29
    }
    #[doc = "0x178 - Internal clock select and divide register30"]
    #[inline(always)]
    pub const fn clksel_con30(&self) -> &ClkselCon30 {
        &self.clksel_con30
    }
    #[doc = "0x17c - Internal clock select and divide register31"]
    #[inline(always)]
    pub const fn clksel_con31(&self) -> &ClkselCon31 {
        &self.clksel_con31
    }
    #[doc = "0x180 - Internal clock select and divide register32"]
    #[inline(always)]
    pub const fn clksel_con32(&self) -> &ClkselCon32 {
        &self.clksel_con32
    }
    #[doc = "0x184 - Internal clock select and divide register33"]
    #[inline(always)]
    pub const fn clksel_con33(&self) -> &ClkselCon33 {
        &self.clksel_con33
    }
    #[doc = "0x188 - Internal clock select and divide register34"]
    #[inline(always)]
    pub const fn clksel_con34(&self) -> &ClkselCon34 {
        &self.clksel_con34
    }
    #[doc = "0x18c - Internal clock select and divide register35"]
    #[inline(always)]
    pub const fn clksel_con35(&self) -> &ClkselCon35 {
        &self.clksel_con35
    }
    #[doc = "0x190 - Internal clock select and divide register36"]
    #[inline(always)]
    pub const fn clksel_con36(&self) -> &ClkselCon36 {
        &self.clksel_con36
    }
    #[doc = "0x198 - Internal clock select and divide register38"]
    #[inline(always)]
    pub const fn clksel_con38(&self) -> &ClkselCon38 {
        &self.clksel_con38
    }
    #[doc = "0x19c - Internal clock select and divide register39"]
    #[inline(always)]
    pub const fn clksel_con39(&self) -> &ClkselCon39 {
        &self.clksel_con39
    }
    #[doc = "0x1a0 - Internal clock select and divide register40"]
    #[inline(always)]
    pub const fn clksel_con40(&self) -> &ClkselCon40 {
        &self.clksel_con40
    }
    #[doc = "0x1a4 - Internal clock select and divide register41"]
    #[inline(always)]
    pub const fn clksel_con41(&self) -> &ClkselCon41 {
        &self.clksel_con41
    }
    #[doc = "0x1a8 - Internal clock select and divide register42"]
    #[inline(always)]
    pub const fn clksel_con42(&self) -> &ClkselCon42 {
        &self.clksel_con42
    }
    #[doc = "0x1ac - Internal clock select and divide register43"]
    #[inline(always)]
    pub const fn clksel_con43(&self) -> &ClkselCon43 {
        &self.clksel_con43
    }
    #[doc = "0x1b0 - Internal clock select and divide register44"]
    #[inline(always)]
    pub const fn clksel_con44(&self) -> &ClkselCon44 {
        &self.clksel_con44
    }
    #[doc = "0x1b4 - Internal clock select and divide register45"]
    #[inline(always)]
    pub const fn clksel_con45(&self) -> &ClkselCon45 {
        &self.clksel_con45
    }
    #[doc = "0x1b8 - Internal clock select and divide register46"]
    #[inline(always)]
    pub const fn clksel_con46(&self) -> &ClkselCon46 {
        &self.clksel_con46
    }
    #[doc = "0x1bc - Internal clock select and divide register47"]
    #[inline(always)]
    pub const fn clksel_con47(&self) -> &ClkselCon47 {
        &self.clksel_con47
    }
    #[doc = "0x1c0 - Internal clock select and divide register48"]
    #[inline(always)]
    pub const fn clksel_con48(&self) -> &ClkselCon48 {
        &self.clksel_con48
    }
    #[doc = "0x1c4 - Internal clock select and divide register49"]
    #[inline(always)]
    pub const fn clksel_con49(&self) -> &ClkselCon49 {
        &self.clksel_con49
    }
    #[doc = "0x1c8 - Internal clock select and divide register50"]
    #[inline(always)]
    pub const fn clksel_con50(&self) -> &ClkselCon50 {
        &self.clksel_con50
    }
    #[doc = "0x1cc - Internal clock select and divide register51"]
    #[inline(always)]
    pub const fn clksel_con51(&self) -> &ClkselCon51 {
        &self.clksel_con51
    }
    #[doc = "0x1d0 - Internal clock select and divide register52"]
    #[inline(always)]
    pub const fn clksel_con52(&self) -> &ClkselCon52 {
        &self.clksel_con52
    }
    #[doc = "0x1d4 - Internal clock select and divide register53"]
    #[inline(always)]
    pub const fn clksel_con53(&self) -> &ClkselCon53 {
        &self.clksel_con53
    }
    #[doc = "0x1d8 - Internal clock select and divide register54"]
    #[inline(always)]
    pub const fn clksel_con54(&self) -> &ClkselCon54 {
        &self.clksel_con54
    }
    #[doc = "0x1dc - Internal clock select and divide register55"]
    #[inline(always)]
    pub const fn clksel_con55(&self) -> &ClkselCon55 {
        &self.clksel_con55
    }
    #[doc = "0x1e0 - Internal clock select and divide register56"]
    #[inline(always)]
    pub const fn clksel_con56(&self) -> &ClkselCon56 {
        &self.clksel_con56
    }
    #[doc = "0x1e4 - Internal clock select and divide register57"]
    #[inline(always)]
    pub const fn clksel_con57(&self) -> &ClkselCon57 {
        &self.clksel_con57
    }
    #[doc = "0x1e8 - Internal clock select and divide register58"]
    #[inline(always)]
    pub const fn clksel_con58(&self) -> &ClkselCon58 {
        &self.clksel_con58
    }
    #[doc = "0x1ec - Internal clock select and divide register59"]
    #[inline(always)]
    pub const fn clksel_con59(&self) -> &ClkselCon59 {
        &self.clksel_con59
    }
    #[doc = "0x1f0 - Internal clock select and divide register60"]
    #[inline(always)]
    pub const fn clksel_con60(&self) -> &ClkselCon60 {
        &self.clksel_con60
    }
    #[doc = "0x1f4 - Internal clock select and divide register61"]
    #[inline(always)]
    pub const fn clksel_con61(&self) -> &ClkselCon61 {
        &self.clksel_con61
    }
    #[doc = "0x1f8 - Internal clock select and divide register62"]
    #[inline(always)]
    pub const fn clksel_con62(&self) -> &ClkselCon62 {
        &self.clksel_con62
    }
    #[doc = "0x1fc - Internal clock select and divide register63"]
    #[inline(always)]
    pub const fn clksel_con63(&self) -> &ClkselCon63 {
        &self.clksel_con63
    }
    #[doc = "0x200 - Internal clock select and divide register64"]
    #[inline(always)]
    pub const fn clksel_con64(&self) -> &ClkselCon64 {
        &self.clksel_con64
    }
    #[doc = "0x204 - Internal clock select and divide register65"]
    #[inline(always)]
    pub const fn clksel_con65(&self) -> &ClkselCon65 {
        &self.clksel_con65
    }
    #[doc = "0x280 - Internal clock select and divide register80"]
    #[inline(always)]
    pub const fn clksel_con96(&self) -> &ClkselCon96 {
        &self.clksel_con96
    }
    #[doc = "0x284 - Internal clock select and divide register81"]
    #[inline(always)]
    pub const fn clksel_con97(&self) -> &ClkselCon97 {
        &self.clksel_con97
    }
    #[doc = "0x288 - Internal clock select and divide register82"]
    #[inline(always)]
    pub const fn clksel_con98(&self) -> &ClkselCon98 {
        &self.clksel_con98
    }
    #[doc = "0x28c - Internal clock select and divide register83"]
    #[inline(always)]
    pub const fn clksel_con99(&self) -> &ClkselCon99 {
        &self.clksel_con99
    }
    #[doc = "0x290 - Internal clock select and divide register84"]
    #[inline(always)]
    pub const fn clksel_con100(&self) -> &ClkselCon100 {
        &self.clksel_con100
    }
    #[doc = "0x294 - Internal clock select and divide register85"]
    #[inline(always)]
    pub const fn clksel_con101(&self) -> &ClkselCon101 {
        &self.clksel_con101
    }
    #[doc = "0x298 - Internal clock select and divide register86"]
    #[inline(always)]
    pub const fn clksel_con102(&self) -> &ClkselCon102 {
        &self.clksel_con102
    }
    #[doc = "0x29c - Internal clock select and divide register87"]
    #[inline(always)]
    pub const fn clksel_con103(&self) -> &ClkselCon103 {
        &self.clksel_con103
    }
    #[doc = "0x2a4 - Internal clock select and divide register89"]
    #[inline(always)]
    pub const fn clksel_con105(&self) -> &ClkselCon105 {
        &self.clksel_con105
    }
    #[doc = "0x2a8 - Internal clock select and divide register90"]
    #[inline(always)]
    pub const fn clksel_con106(&self) -> &ClkselCon106 {
        &self.clksel_con106
    }
    #[doc = "0x2ac - Internal clock select and divide register91"]
    #[inline(always)]
    pub const fn clksel_con107(&self) -> &ClkselCon107 {
        &self.clksel_con107
    }
    #[doc = "0x300 - Internal clock gating register0"]
    #[inline(always)]
    pub const fn clkgate_con0(&self) -> &ClkgateCon0 {
        &self.clkgate_con0
    }
    #[doc = "0x304 - Internal clock gating register1"]
    #[inline(always)]
    pub const fn clkgate_con1(&self) -> &ClkgateCon1 {
        &self.clkgate_con1
    }
    #[doc = "0x308 - Internal clock gating register2"]
    #[inline(always)]
    pub const fn clkgate_con2(&self) -> &ClkgateCon2 {
        &self.clkgate_con2
    }
    #[doc = "0x30c - Internal clock gating register3"]
    #[inline(always)]
    pub const fn clkgate_con3(&self) -> &ClkgateCon3 {
        &self.clkgate_con3
    }
    #[doc = "0x310 - Internal clock gating register4"]
    #[inline(always)]
    pub const fn clkgate_con4(&self) -> &ClkgateCon4 {
        &self.clkgate_con4
    }
    #[doc = "0x314 - Internal clock gating register5"]
    #[inline(always)]
    pub const fn clkgate_con5(&self) -> &ClkgateCon5 {
        &self.clkgate_con5
    }
    #[doc = "0x318 - Internal clock gating register6"]
    #[inline(always)]
    pub const fn clkgate_con6(&self) -> &ClkgateCon6 {
        &self.clkgate_con6
    }
    #[doc = "0x31c - Internal clock gating register7"]
    #[inline(always)]
    pub const fn clkgate_con7(&self) -> &ClkgateCon7 {
        &self.clkgate_con7
    }
    #[doc = "0x320 - Internal clock gating register8"]
    #[inline(always)]
    pub const fn clkgate_con8(&self) -> &ClkgateCon8 {
        &self.clkgate_con8
    }
    #[doc = "0x324 - Internal clock gating register9"]
    #[inline(always)]
    pub const fn clkgate_con9(&self) -> &ClkgateCon9 {
        &self.clkgate_con9
    }
    #[doc = "0x328 - Internal clock gating register10"]
    #[inline(always)]
    pub const fn clkgate_con10(&self) -> &ClkgateCon10 {
        &self.clkgate_con10
    }
    #[doc = "0x32c - Internal clock gating register11"]
    #[inline(always)]
    pub const fn clkgate_con11(&self) -> &ClkgateCon11 {
        &self.clkgate_con11
    }
    #[doc = "0x330 - Internal clock gating register12"]
    #[inline(always)]
    pub const fn clkgate_con12(&self) -> &ClkgateCon12 {
        &self.clkgate_con12
    }
    #[doc = "0x334 - Internal clock gating register13"]
    #[inline(always)]
    pub const fn clkgate_con13(&self) -> &ClkgateCon13 {
        &self.clkgate_con13
    }
    #[doc = "0x338 - Internal clock gating register14"]
    #[inline(always)]
    pub const fn clkgate_con14(&self) -> &ClkgateCon14 {
        &self.clkgate_con14
    }
    #[doc = "0x33c - Internal clock gating register15"]
    #[inline(always)]
    pub const fn clkgate_con15(&self) -> &ClkgateCon15 {
        &self.clkgate_con15
    }
    #[doc = "0x340 - Internal clock gating register16"]
    #[inline(always)]
    pub const fn clkgate_con16(&self) -> &ClkgateCon16 {
        &self.clkgate_con16
    }
    #[doc = "0x344 - Internal clock gating register17"]
    #[inline(always)]
    pub const fn clkgate_con17(&self) -> &ClkgateCon17 {
        &self.clkgate_con17
    }
    #[doc = "0x348 - Internal clock gating register18"]
    #[inline(always)]
    pub const fn clkgate_con18(&self) -> &ClkgateCon18 {
        &self.clkgate_con18
    }
    #[doc = "0x34c - Internal clock gating register19"]
    #[inline(always)]
    pub const fn clkgate_con19(&self) -> &ClkgateCon19 {
        &self.clkgate_con19
    }
    #[doc = "0x350 - Internal clock gating register20"]
    #[inline(always)]
    pub const fn clkgate_con20(&self) -> &ClkgateCon20 {
        &self.clkgate_con20
    }
    #[doc = "0x354 - Internal clock gating register21"]
    #[inline(always)]
    pub const fn clkgate_con21(&self) -> &ClkgateCon21 {
        &self.clkgate_con21
    }
    #[doc = "0x358 - Internal clock gating register22"]
    #[inline(always)]
    pub const fn clkgate_con22(&self) -> &ClkgateCon22 {
        &self.clkgate_con22
    }
    #[doc = "0x35c - Internal clock gating register23"]
    #[inline(always)]
    pub const fn clkgate_con23(&self) -> &ClkgateCon23 {
        &self.clkgate_con23
    }
    #[doc = "0x360 - Internal clock gating register24"]
    #[inline(always)]
    pub const fn clkgate_con24(&self) -> &ClkgateCon24 {
        &self.clkgate_con24
    }
    #[doc = "0x364 - Internal clock gating register25"]
    #[inline(always)]
    pub const fn clkgate_con25(&self) -> &ClkgateCon25 {
        &self.clkgate_con25
    }
    #[doc = "0x368 - Internal clock gating register26"]
    #[inline(always)]
    pub const fn clkgate_con26(&self) -> &ClkgateCon26 {
        &self.clkgate_con26
    }
    #[doc = "0x36c - Internal clock gating register27"]
    #[inline(always)]
    pub const fn clkgate_con27(&self) -> &ClkgateCon27 {
        &self.clkgate_con27
    }
    #[doc = "0x370 - Internal clock gating register28"]
    #[inline(always)]
    pub const fn clkgate_con28(&self) -> &ClkgateCon28 {
        &self.clkgate_con28
    }
    #[doc = "0x374 - Internal clock gating register29"]
    #[inline(always)]
    pub const fn clkgate_con29(&self) -> &ClkgateCon29 {
        &self.clkgate_con29
    }
    #[doc = "0x378 - Internal clock gating register30"]
    #[inline(always)]
    pub const fn clkgate_con30(&self) -> &ClkgateCon30 {
        &self.clkgate_con30
    }
    #[doc = "0x37c - Internal clock gating register31"]
    #[inline(always)]
    pub const fn clkgate_con31(&self) -> &ClkgateCon31 {
        &self.clkgate_con31
    }
    #[doc = "0x380 - Internal clock gating register32"]
    #[inline(always)]
    pub const fn clkgate_con32(&self) -> &ClkgateCon32 {
        &self.clkgate_con32
    }
    #[doc = "0x384 - Internal clock gating register33"]
    #[inline(always)]
    pub const fn clkgate_con33(&self) -> &ClkgateCon33 {
        &self.clkgate_con33
    }
    #[doc = "0x388 - Internal clock gating register34"]
    #[inline(always)]
    pub const fn clkgate_con34(&self) -> &ClkgateCon34 {
        &self.clkgate_con34
    }
    #[doc = "0x400 - Internal software reset control register0"]
    #[inline(always)]
    pub const fn softrst_con0(&self) -> &SoftrstCon0 {
        &self.softrst_con0
    }
    #[doc = "0x404 - Internal software reset control register1"]
    #[inline(always)]
    pub const fn softrst_con1(&self) -> &SoftrstCon1 {
        &self.softrst_con1
    }
    #[doc = "0x408 - Internal software reset control register2"]
    #[inline(always)]
    pub const fn softrst_con2(&self) -> &SoftrstCon2 {
        &self.softrst_con2
    }
    #[doc = "0x40c - Internal software reset control register3"]
    #[inline(always)]
    pub const fn softrst_con3(&self) -> &SoftrstCon3 {
        &self.softrst_con3
    }
    #[doc = "0x410 - Internal software reset control register4"]
    #[inline(always)]
    pub const fn softrst_con4(&self) -> &SoftrstCon4 {
        &self.softrst_con4
    }
    #[doc = "0x414 - Internal software reset control register5"]
    #[inline(always)]
    pub const fn softrst_con5(&self) -> &SoftrstCon5 {
        &self.softrst_con5
    }
    #[doc = "0x418 - Internal software reset control register6"]
    #[inline(always)]
    pub const fn softrst_con6(&self) -> &SoftrstCon6 {
        &self.softrst_con6
    }
    #[doc = "0x41c - Internal software reset control register7"]
    #[inline(always)]
    pub const fn softrst_con7(&self) -> &SoftrstCon7 {
        &self.softrst_con7
    }
    #[doc = "0x420 - Internal software reset control register8"]
    #[inline(always)]
    pub const fn softrst_con8(&self) -> &SoftrstCon8 {
        &self.softrst_con8
    }
    #[doc = "0x424 - Internal software reset control register9"]
    #[inline(always)]
    pub const fn softrst_con9(&self) -> &SoftrstCon9 {
        &self.softrst_con9
    }
    #[doc = "0x428 - Internal software reset control register10"]
    #[inline(always)]
    pub const fn softrst_con10(&self) -> &SoftrstCon10 {
        &self.softrst_con10
    }
    #[doc = "0x42c - Internal software reset control register11"]
    #[inline(always)]
    pub const fn softrst_con11(&self) -> &SoftrstCon11 {
        &self.softrst_con11
    }
    #[doc = "0x430 - Internal software reset control register12"]
    #[inline(always)]
    pub const fn softrst_con12(&self) -> &SoftrstCon12 {
        &self.softrst_con12
    }
    #[doc = "0x434 - Internal software reset control register13"]
    #[inline(always)]
    pub const fn softrst_con13(&self) -> &SoftrstCon13 {
        &self.softrst_con13
    }
    #[doc = "0x438 - Internal software reset control register14"]
    #[inline(always)]
    pub const fn softrst_con14(&self) -> &SoftrstCon14 {
        &self.softrst_con14
    }
    #[doc = "0x43c - Internal software reset control register15"]
    #[inline(always)]
    pub const fn softrst_con15(&self) -> &SoftrstCon15 {
        &self.softrst_con15
    }
    #[doc = "0x440 - Internal software reset control register16"]
    #[inline(always)]
    pub const fn softrst_con16(&self) -> &SoftrstCon16 {
        &self.softrst_con16
    }
    #[doc = "0x444 - Internal software reset control register17"]
    #[inline(always)]
    pub const fn softrst_con17(&self) -> &SoftrstCon17 {
        &self.softrst_con17
    }
    #[doc = "0x448 - Internal software reset control register18"]
    #[inline(always)]
    pub const fn softrst_con18(&self) -> &SoftrstCon18 {
        &self.softrst_con18
    }
    #[doc = "0x44c - Internal software reset control register19"]
    #[inline(always)]
    pub const fn softrst_con19(&self) -> &SoftrstCon19 {
        &self.softrst_con19
    }
    #[doc = "0x450 - Internal software reset control register20"]
    #[inline(always)]
    pub const fn softrst_con20(&self) -> &SoftrstCon20 {
        &self.softrst_con20
    }
    #[doc = "0x500 - The first global software reset config value"]
    #[inline(always)]
    pub const fn glb_srst_fst_value(&self) -> &GlbSrstFstValue {
        &self.glb_srst_fst_value
    }
    #[doc = "0x504 - The second global software reset config value"]
    #[inline(always)]
    pub const fn glb_srst_snd_value(&self) -> &GlbSrstSndValue {
        &self.glb_srst_snd_value
    }
    #[doc = "0x508 - Global soft reset counter threshold"]
    #[inline(always)]
    pub const fn glb_cnt_th(&self) -> &GlbCntTh {
        &self.glb_cnt_th
    }
    #[doc = "0x50c - Output clock selection for test"]
    #[inline(always)]
    pub const fn misc_con(&self) -> &MiscCon {
        &self.misc_con
    }
    #[doc = "0x510 - Global reset trigger select"]
    #[inline(always)]
    pub const fn glb_rst_con(&self) -> &GlbRstCon {
        &self.glb_rst_con
    }
    #[doc = "0x514 - Global reset status"]
    #[inline(always)]
    pub const fn glb_rst_st(&self) -> &GlbRstSt {
        &self.glb_rst_st
    }
    #[doc = "0x580 - sdmmc control0"]
    #[inline(always)]
    pub const fn sdmmc_con0(&self) -> &SdmmcCon0 {
        &self.sdmmc_con0
    }
    #[doc = "0x584 - sdmmc control1"]
    #[inline(always)]
    pub const fn sdmmc_con1(&self) -> &SdmmcCon1 {
        &self.sdmmc_con1
    }
    #[doc = "0x588 - sdio0 control0"]
    #[inline(always)]
    pub const fn sdio0_con0(&self) -> &Sdio0Con0 {
        &self.sdio0_con0
    }
    #[doc = "0x58c - sdio0 control1"]
    #[inline(always)]
    pub const fn sdio0_con1(&self) -> &Sdio0Con1 {
        &self.sdio0_con1
    }
}
#[doc = "LPLL_CON0 (rw) register accessor: LPLL configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpll_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpll_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpll_con0`]
module"]
#[doc(alias = "LPLL_CON0")]
pub type LpllCon0 = crate::Reg<lpll_con0::LpllCon0Spec>;
#[doc = "LPLL configuration register0"]
pub mod lpll_con0;
#[doc = "LPLL_CON1 (rw) register accessor: LPLL configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpll_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpll_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpll_con1`]
module"]
#[doc(alias = "LPLL_CON1")]
pub type LpllCon1 = crate::Reg<lpll_con1::LpllCon1Spec>;
#[doc = "LPLL configuration register1"]
pub mod lpll_con1;
#[doc = "LPLL_CON2 (rw) register accessor: LPLL configuration register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpll_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpll_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpll_con2`]
module"]
#[doc(alias = "LPLL_CON2")]
pub type LpllCon2 = crate::Reg<lpll_con2::LpllCon2Spec>;
#[doc = "LPLL configuration register2"]
pub mod lpll_con2;
#[doc = "LPLL_CON3 (rw) register accessor: LPLL configuration register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpll_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpll_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpll_con3`]
module"]
#[doc(alias = "LPLL_CON3")]
pub type LpllCon3 = crate::Reg<lpll_con3::LpllCon3Spec>;
#[doc = "LPLL configuration register3"]
pub mod lpll_con3;
#[doc = "LPLL_CON4 (rw) register accessor: LPLL configuration register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpll_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpll_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpll_con4`]
module"]
#[doc(alias = "LPLL_CON4")]
pub type LpllCon4 = crate::Reg<lpll_con4::LpllCon4Spec>;
#[doc = "LPLL configuration register4"]
pub mod lpll_con4;
#[doc = "LPLL_CON5 (rw) register accessor: LPLL configuration register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpll_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpll_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpll_con5`]
module"]
#[doc(alias = "LPLL_CON5")]
pub type LpllCon5 = crate::Reg<lpll_con5::LpllCon5Spec>;
#[doc = "LPLL configuration register5"]
pub mod lpll_con5;
#[doc = "BPLL_CON0 (rw) register accessor: BPLL configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpll_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpll_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpll_con0`]
module"]
#[doc(alias = "BPLL_CON0")]
pub type BpllCon0 = crate::Reg<bpll_con0::BpllCon0Spec>;
#[doc = "BPLL configuration register0"]
pub mod bpll_con0;
#[doc = "BPLL_CON1 (rw) register accessor: BPLL configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpll_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpll_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpll_con1`]
module"]
#[doc(alias = "BPLL_CON1")]
pub type BpllCon1 = crate::Reg<bpll_con1::BpllCon1Spec>;
#[doc = "BPLL configuration register1"]
pub mod bpll_con1;
#[doc = "BPLL_CON2 (rw) register accessor: BPLL configuration register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpll_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpll_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpll_con2`]
module"]
#[doc(alias = "BPLL_CON2")]
pub type BpllCon2 = crate::Reg<bpll_con2::BpllCon2Spec>;
#[doc = "BPLL configuration register2"]
pub mod bpll_con2;
#[doc = "BPLL_CON3 (rw) register accessor: BPLL configuration register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpll_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpll_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpll_con3`]
module"]
#[doc(alias = "BPLL_CON3")]
pub type BpllCon3 = crate::Reg<bpll_con3::BpllCon3Spec>;
#[doc = "BPLL configuration register3"]
pub mod bpll_con3;
#[doc = "BPLL_CON4 (rw) register accessor: BPLL configuration register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpll_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpll_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpll_con4`]
module"]
#[doc(alias = "BPLL_CON4")]
pub type BpllCon4 = crate::Reg<bpll_con4::BpllCon4Spec>;
#[doc = "BPLL configuration register4"]
pub mod bpll_con4;
#[doc = "BPLL_CON5 (rw) register accessor: BPLL configuration register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpll_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpll_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpll_con5`]
module"]
#[doc(alias = "BPLL_CON5")]
pub type BpllCon5 = crate::Reg<bpll_con5::BpllCon5Spec>;
#[doc = "BPLL configuration register5"]
pub mod bpll_con5;
#[doc = "DPLL_CON0 (rw) register accessor: DPLL configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpll_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpll_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpll_con0`]
module"]
#[doc(alias = "DPLL_CON0")]
pub type DpllCon0 = crate::Reg<dpll_con0::DpllCon0Spec>;
#[doc = "DPLL configuration register0"]
pub mod dpll_con0;
#[doc = "DPLL_CON1 (rw) register accessor: DPLL configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpll_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpll_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpll_con1`]
module"]
#[doc(alias = "DPLL_CON1")]
pub type DpllCon1 = crate::Reg<dpll_con1::DpllCon1Spec>;
#[doc = "DPLL configuration register1"]
pub mod dpll_con1;
#[doc = "DPLL_CON2 (rw) register accessor: DPLL configuration register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpll_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpll_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpll_con2`]
module"]
#[doc(alias = "DPLL_CON2")]
pub type DpllCon2 = crate::Reg<dpll_con2::DpllCon2Spec>;
#[doc = "DPLL configuration register2"]
pub mod dpll_con2;
#[doc = "DPLL_CON3 (rw) register accessor: DPLL configuration register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpll_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpll_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpll_con3`]
module"]
#[doc(alias = "DPLL_CON3")]
pub type DpllCon3 = crate::Reg<dpll_con3::DpllCon3Spec>;
#[doc = "DPLL configuration register3"]
pub mod dpll_con3;
#[doc = "DPLL_CON4 (rw) register accessor: DPLL configuration register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpll_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpll_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpll_con4`]
module"]
#[doc(alias = "DPLL_CON4")]
pub type DpllCon4 = crate::Reg<dpll_con4::DpllCon4Spec>;
#[doc = "DPLL configuration register4"]
pub mod dpll_con4;
#[doc = "DPLL_CON5 (rw) register accessor: DPLL configuration register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpll_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpll_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpll_con5`]
module"]
#[doc(alias = "DPLL_CON5")]
pub type DpllCon5 = crate::Reg<dpll_con5::DpllCon5Spec>;
#[doc = "DPLL configuration register5"]
pub mod dpll_con5;
#[doc = "CPLL_CON0 (rw) register accessor: CPLL configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpll_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpll_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpll_con0`]
module"]
#[doc(alias = "CPLL_CON0")]
pub type CpllCon0 = crate::Reg<cpll_con0::CpllCon0Spec>;
#[doc = "CPLL configuration register0"]
pub mod cpll_con0;
#[doc = "CPLL_CON1 (rw) register accessor: CPLL configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpll_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpll_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpll_con1`]
module"]
#[doc(alias = "CPLL_CON1")]
pub type CpllCon1 = crate::Reg<cpll_con1::CpllCon1Spec>;
#[doc = "CPLL configuration register1"]
pub mod cpll_con1;
#[doc = "CPLL_CON2 (rw) register accessor: CPLL configuration register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpll_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpll_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpll_con2`]
module"]
#[doc(alias = "CPLL_CON2")]
pub type CpllCon2 = crate::Reg<cpll_con2::CpllCon2Spec>;
#[doc = "CPLL configuration register2"]
pub mod cpll_con2;
#[doc = "CPLL_CON3 (rw) register accessor: CPLL configuration register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpll_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpll_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpll_con3`]
module"]
#[doc(alias = "CPLL_CON3")]
pub type CpllCon3 = crate::Reg<cpll_con3::CpllCon3Spec>;
#[doc = "CPLL configuration register3"]
pub mod cpll_con3;
#[doc = "CPLL_CON4 (rw) register accessor: CPLL configuration register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpll_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpll_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpll_con4`]
module"]
#[doc(alias = "CPLL_CON4")]
pub type CpllCon4 = crate::Reg<cpll_con4::CpllCon4Spec>;
#[doc = "CPLL configuration register4"]
pub mod cpll_con4;
#[doc = "CPLL_CON5 (rw) register accessor: CPLL configuration register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpll_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpll_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpll_con5`]
module"]
#[doc(alias = "CPLL_CON5")]
pub type CpllCon5 = crate::Reg<cpll_con5::CpllCon5Spec>;
#[doc = "CPLL configuration register5"]
pub mod cpll_con5;
#[doc = "GPLL_CON0 (rw) register accessor: GPLL configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpll_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpll_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpll_con0`]
module"]
#[doc(alias = "GPLL_CON0")]
pub type GpllCon0 = crate::Reg<gpll_con0::GpllCon0Spec>;
#[doc = "GPLL configuration register0"]
pub mod gpll_con0;
#[doc = "GPLL_CON1 (rw) register accessor: GPLL configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpll_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpll_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpll_con1`]
module"]
#[doc(alias = "GPLL_CON1")]
pub type GpllCon1 = crate::Reg<gpll_con1::GpllCon1Spec>;
#[doc = "GPLL configuration register1"]
pub mod gpll_con1;
#[doc = "GPLL_CON2 (rw) register accessor: GPLL configuration register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpll_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpll_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpll_con2`]
module"]
#[doc(alias = "GPLL_CON2")]
pub type GpllCon2 = crate::Reg<gpll_con2::GpllCon2Spec>;
#[doc = "GPLL configuration register2"]
pub mod gpll_con2;
#[doc = "GPLL_CON3 (rw) register accessor: GPLL configuration register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpll_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpll_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpll_con3`]
module"]
#[doc(alias = "GPLL_CON3")]
pub type GpllCon3 = crate::Reg<gpll_con3::GpllCon3Spec>;
#[doc = "GPLL configuration register3"]
pub mod gpll_con3;
#[doc = "GPLL_CON4 (rw) register accessor: GPLL configuration register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpll_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpll_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpll_con4`]
module"]
#[doc(alias = "GPLL_CON4")]
pub type GpllCon4 = crate::Reg<gpll_con4::GpllCon4Spec>;
#[doc = "GPLL configuration register4"]
pub mod gpll_con4;
#[doc = "GPLL_CON5 (rw) register accessor: GPLL configuration register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpll_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpll_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpll_con5`]
module"]
#[doc(alias = "GPLL_CON5")]
pub type GpllCon5 = crate::Reg<gpll_con5::GpllCon5Spec>;
#[doc = "GPLL configuration register5"]
pub mod gpll_con5;
#[doc = "NPLL_CON0 (rw) register accessor: NPLL configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npll_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npll_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@npll_con0`]
module"]
#[doc(alias = "NPLL_CON0")]
pub type NpllCon0 = crate::Reg<npll_con0::NpllCon0Spec>;
#[doc = "NPLL configuration register0"]
pub mod npll_con0;
#[doc = "NPLL_CON1 (rw) register accessor: NPLL configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npll_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npll_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@npll_con1`]
module"]
#[doc(alias = "NPLL_CON1")]
pub type NpllCon1 = crate::Reg<npll_con1::NpllCon1Spec>;
#[doc = "NPLL configuration register1"]
pub mod npll_con1;
#[doc = "NPLL_CON2 (rw) register accessor: NPLL configuration register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npll_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npll_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@npll_con2`]
module"]
#[doc(alias = "NPLL_CON2")]
pub type NpllCon2 = crate::Reg<npll_con2::NpllCon2Spec>;
#[doc = "NPLL configuration register2"]
pub mod npll_con2;
#[doc = "NPLL_CON3 (rw) register accessor: NPLL configuration register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npll_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npll_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@npll_con3`]
module"]
#[doc(alias = "NPLL_CON3")]
pub type NpllCon3 = crate::Reg<npll_con3::NpllCon3Spec>;
#[doc = "NPLL configuration register3"]
pub mod npll_con3;
#[doc = "NPLL_CON4 (rw) register accessor: NPLL configuration register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npll_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npll_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@npll_con4`]
module"]
#[doc(alias = "NPLL_CON4")]
pub type NpllCon4 = crate::Reg<npll_con4::NpllCon4Spec>;
#[doc = "NPLL configuration register4"]
pub mod npll_con4;
#[doc = "NPLL_CON5 (rw) register accessor: NPLL configuration register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npll_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npll_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@npll_con5`]
module"]
#[doc(alias = "NPLL_CON5")]
pub type NpllCon5 = crate::Reg<npll_con5::NpllCon5Spec>;
#[doc = "NPLL configuration register5"]
pub mod npll_con5;
#[doc = "VPLL_CON0 (rw) register accessor: VPLL configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vpll_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vpll_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vpll_con0`]
module"]
#[doc(alias = "VPLL_CON0")]
pub type VpllCon0 = crate::Reg<vpll_con0::VpllCon0Spec>;
#[doc = "VPLL configuration register0"]
pub mod vpll_con0;
#[doc = "VPLL_CON1 (rw) register accessor: VPLL configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vpll_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vpll_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vpll_con1`]
module"]
#[doc(alias = "VPLL_CON1")]
pub type VpllCon1 = crate::Reg<vpll_con1::VpllCon1Spec>;
#[doc = "VPLL configuration register1"]
pub mod vpll_con1;
#[doc = "VPLL_CON2 (rw) register accessor: VPLL configuration register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vpll_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vpll_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vpll_con2`]
module"]
#[doc(alias = "VPLL_CON2")]
pub type VpllCon2 = crate::Reg<vpll_con2::VpllCon2Spec>;
#[doc = "VPLL configuration register2"]
pub mod vpll_con2;
#[doc = "VPLL_CON3 (rw) register accessor: VPLL configuration register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vpll_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vpll_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vpll_con3`]
module"]
#[doc(alias = "VPLL_CON3")]
pub type VpllCon3 = crate::Reg<vpll_con3::VpllCon3Spec>;
#[doc = "VPLL configuration register3"]
pub mod vpll_con3;
#[doc = "VPLL_CON4 (rw) register accessor: VPLL configuration register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vpll_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vpll_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vpll_con4`]
module"]
#[doc(alias = "VPLL_CON4")]
pub type VpllCon4 = crate::Reg<vpll_con4::VpllCon4Spec>;
#[doc = "VPLL configuration register4"]
pub mod vpll_con4;
#[doc = "VPLL_CON5 (rw) register accessor: VPLL configuration register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vpll_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vpll_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vpll_con5`]
module"]
#[doc(alias = "VPLL_CON5")]
pub type VpllCon5 = crate::Reg<vpll_con5::VpllCon5Spec>;
#[doc = "VPLL configuration register5"]
pub mod vpll_con5;
#[doc = "CLKSEL_CON0 (rw) register accessor: Internal clock select and divide register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con0`]
module"]
#[doc(alias = "CLKSEL_CON0")]
pub type ClkselCon0 = crate::Reg<clksel_con0::ClkselCon0Spec>;
#[doc = "Internal clock select and divide register0"]
pub mod clksel_con0;
#[doc = "CLKSEL_CON1 (rw) register accessor: Internal clock select and divide register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con1`]
module"]
#[doc(alias = "CLKSEL_CON1")]
pub type ClkselCon1 = crate::Reg<clksel_con1::ClkselCon1Spec>;
#[doc = "Internal clock select and divide register1"]
pub mod clksel_con1;
#[doc = "CLKSEL_CON2 (rw) register accessor: Internal clock select and divide register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con2`]
module"]
#[doc(alias = "CLKSEL_CON2")]
pub type ClkselCon2 = crate::Reg<clksel_con2::ClkselCon2Spec>;
#[doc = "Internal clock select and divide register2"]
pub mod clksel_con2;
#[doc = "CLKSEL_CON3 (rw) register accessor: Internal clock select and divide register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con3`]
module"]
#[doc(alias = "CLKSEL_CON3")]
pub type ClkselCon3 = crate::Reg<clksel_con3::ClkselCon3Spec>;
#[doc = "Internal clock select and divide register3"]
pub mod clksel_con3;
#[doc = "CLKSEL_CON4 (rw) register accessor: Internal clock select and divide register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con4`]
module"]
#[doc(alias = "CLKSEL_CON4")]
pub type ClkselCon4 = crate::Reg<clksel_con4::ClkselCon4Spec>;
#[doc = "Internal clock select and divide register4"]
pub mod clksel_con4;
#[doc = "CLKSEL_CON5 (rw) register accessor: Internal clock select and divide register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con5`]
module"]
#[doc(alias = "CLKSEL_CON5")]
pub type ClkselCon5 = crate::Reg<clksel_con5::ClkselCon5Spec>;
#[doc = "Internal clock select and divide register5"]
pub mod clksel_con5;
#[doc = "CLKSEL_CON6 (rw) register accessor: Internal clock select and divide register6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con6`]
module"]
#[doc(alias = "CLKSEL_CON6")]
pub type ClkselCon6 = crate::Reg<clksel_con6::ClkselCon6Spec>;
#[doc = "Internal clock select and divide register6"]
pub mod clksel_con6;
#[doc = "CLKSEL_CON7 (rw) register accessor: Internal clock select and divide register7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con7`]
module"]
#[doc(alias = "CLKSEL_CON7")]
pub type ClkselCon7 = crate::Reg<clksel_con7::ClkselCon7Spec>;
#[doc = "Internal clock select and divide register7"]
pub mod clksel_con7;
#[doc = "CLKSEL_CON8 (rw) register accessor: Internal clock select and divide register8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con8`]
module"]
#[doc(alias = "CLKSEL_CON8")]
pub type ClkselCon8 = crate::Reg<clksel_con8::ClkselCon8Spec>;
#[doc = "Internal clock select and divide register8"]
pub mod clksel_con8;
#[doc = "CLKSEL_CON9 (rw) register accessor: Internal clock select and divide register9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con9`]
module"]
#[doc(alias = "CLKSEL_CON9")]
pub type ClkselCon9 = crate::Reg<clksel_con9::ClkselCon9Spec>;
#[doc = "Internal clock select and divide register9"]
pub mod clksel_con9;
#[doc = "CLKSEL_CON10 (rw) register accessor: Internal clock select and divide register10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con10`]
module"]
#[doc(alias = "CLKSEL_CON10")]
pub type ClkselCon10 = crate::Reg<clksel_con10::ClkselCon10Spec>;
#[doc = "Internal clock select and divide register10"]
pub mod clksel_con10;
#[doc = "CLKSEL_CON11 (rw) register accessor: Internal clock select and divide register11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con11`]
module"]
#[doc(alias = "CLKSEL_CON11")]
pub type ClkselCon11 = crate::Reg<clksel_con11::ClkselCon11Spec>;
#[doc = "Internal clock select and divide register11"]
pub mod clksel_con11;
#[doc = "CLKSEL_CON12 (rw) register accessor: Internal clock select and divide register12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con12`]
module"]
#[doc(alias = "CLKSEL_CON12")]
pub type ClkselCon12 = crate::Reg<clksel_con12::ClkselCon12Spec>;
#[doc = "Internal clock select and divide register12"]
pub mod clksel_con12;
#[doc = "CLKSEL_CON13 (rw) register accessor: Internal clock select and divide register13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con13`]
module"]
#[doc(alias = "CLKSEL_CON13")]
pub type ClkselCon13 = crate::Reg<clksel_con13::ClkselCon13Spec>;
#[doc = "Internal clock select and divide register13"]
pub mod clksel_con13;
#[doc = "CLKSEL_CON14 (rw) register accessor: Internal clock select and divide register14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con14`]
module"]
#[doc(alias = "CLKSEL_CON14")]
pub type ClkselCon14 = crate::Reg<clksel_con14::ClkselCon14Spec>;
#[doc = "Internal clock select and divide register14"]
pub mod clksel_con14;
#[doc = "CLKSEL_CON15 (rw) register accessor: Internal clock select and divide register15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con15`]
module"]
#[doc(alias = "CLKSEL_CON15")]
pub type ClkselCon15 = crate::Reg<clksel_con15::ClkselCon15Spec>;
#[doc = "Internal clock select and divide register15"]
pub mod clksel_con15;
#[doc = "CLKSEL_CON16 (rw) register accessor: Internal clock select and divide register16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con16`]
module"]
#[doc(alias = "CLKSEL_CON16")]
pub type ClkselCon16 = crate::Reg<clksel_con16::ClkselCon16Spec>;
#[doc = "Internal clock select and divide register16"]
pub mod clksel_con16;
#[doc = "CLKSEL_CON17 (rw) register accessor: Internal clock select and divide register17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con17`]
module"]
#[doc(alias = "CLKSEL_CON17")]
pub type ClkselCon17 = crate::Reg<clksel_con17::ClkselCon17Spec>;
#[doc = "Internal clock select and divide register17"]
pub mod clksel_con17;
#[doc = "CLKSEL_CON18 (rw) register accessor: Internal clock select and divide register18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con18`]
module"]
#[doc(alias = "CLKSEL_CON18")]
pub type ClkselCon18 = crate::Reg<clksel_con18::ClkselCon18Spec>;
#[doc = "Internal clock select and divide register18"]
pub mod clksel_con18;
#[doc = "CLKSEL_CON19 (rw) register accessor: Internal clock select and divide register19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con19`]
module"]
#[doc(alias = "CLKSEL_CON19")]
pub type ClkselCon19 = crate::Reg<clksel_con19::ClkselCon19Spec>;
#[doc = "Internal clock select and divide register19"]
pub mod clksel_con19;
#[doc = "CLKSEL_CON20 (rw) register accessor: Internal clock select and divide register20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con20`]
module"]
#[doc(alias = "CLKSEL_CON20")]
pub type ClkselCon20 = crate::Reg<clksel_con20::ClkselCon20Spec>;
#[doc = "Internal clock select and divide register20"]
pub mod clksel_con20;
#[doc = "CLKSEL_CON21 (rw) register accessor: Internal clock select and divide register21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con21`]
module"]
#[doc(alias = "CLKSEL_CON21")]
pub type ClkselCon21 = crate::Reg<clksel_con21::ClkselCon21Spec>;
#[doc = "Internal clock select and divide register21"]
pub mod clksel_con21;
#[doc = "CLKSEL_CON22 (rw) register accessor: Internal clock select and divide register22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con22`]
module"]
#[doc(alias = "CLKSEL_CON22")]
pub type ClkselCon22 = crate::Reg<clksel_con22::ClkselCon22Spec>;
#[doc = "Internal clock select and divide register22"]
pub mod clksel_con22;
#[doc = "CLKSEL_CON23 (rw) register accessor: Internal clock select and divide register23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con23`]
module"]
#[doc(alias = "CLKSEL_CON23")]
pub type ClkselCon23 = crate::Reg<clksel_con23::ClkselCon23Spec>;
#[doc = "Internal clock select and divide register23"]
pub mod clksel_con23;
#[doc = "CLKSEL_CON24 (rw) register accessor: Internal clock select and divide register24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con24`]
module"]
#[doc(alias = "CLKSEL_CON24")]
pub type ClkselCon24 = crate::Reg<clksel_con24::ClkselCon24Spec>;
#[doc = "Internal clock select and divide register24"]
pub mod clksel_con24;
#[doc = "CLKSEL_CON25 (rw) register accessor: Internal clock select and divide register25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con25`]
module"]
#[doc(alias = "CLKSEL_CON25")]
pub type ClkselCon25 = crate::Reg<clksel_con25::ClkselCon25Spec>;
#[doc = "Internal clock select and divide register25"]
pub mod clksel_con25;
#[doc = "CLKSEL_CON26 (rw) register accessor: Internal clock select and divide register26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con26`]
module"]
#[doc(alias = "CLKSEL_CON26")]
pub type ClkselCon26 = crate::Reg<clksel_con26::ClkselCon26Spec>;
#[doc = "Internal clock select and divide register26"]
pub mod clksel_con26;
#[doc = "CLKSEL_CON27 (rw) register accessor: Internal clock select and divide register27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con27`]
module"]
#[doc(alias = "CLKSEL_CON27")]
pub type ClkselCon27 = crate::Reg<clksel_con27::ClkselCon27Spec>;
#[doc = "Internal clock select and divide register27"]
pub mod clksel_con27;
#[doc = "CLKSEL_CON28 (rw) register accessor: Internal clock select and divide register28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con28`]
module"]
#[doc(alias = "CLKSEL_CON28")]
pub type ClkselCon28 = crate::Reg<clksel_con28::ClkselCon28Spec>;
#[doc = "Internal clock select and divide register28"]
pub mod clksel_con28;
#[doc = "CLKSEL_CON29 (rw) register accessor: Internal clock select and divide register29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con29`]
module"]
#[doc(alias = "CLKSEL_CON29")]
pub type ClkselCon29 = crate::Reg<clksel_con29::ClkselCon29Spec>;
#[doc = "Internal clock select and divide register29"]
pub mod clksel_con29;
#[doc = "CLKSEL_CON30 (rw) register accessor: Internal clock select and divide register30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con30`]
module"]
#[doc(alias = "CLKSEL_CON30")]
pub type ClkselCon30 = crate::Reg<clksel_con30::ClkselCon30Spec>;
#[doc = "Internal clock select and divide register30"]
pub mod clksel_con30;
#[doc = "CLKSEL_CON31 (rw) register accessor: Internal clock select and divide register31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con31`]
module"]
#[doc(alias = "CLKSEL_CON31")]
pub type ClkselCon31 = crate::Reg<clksel_con31::ClkselCon31Spec>;
#[doc = "Internal clock select and divide register31"]
pub mod clksel_con31;
#[doc = "CLKSEL_CON32 (rw) register accessor: Internal clock select and divide register32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con32`]
module"]
#[doc(alias = "CLKSEL_CON32")]
pub type ClkselCon32 = crate::Reg<clksel_con32::ClkselCon32Spec>;
#[doc = "Internal clock select and divide register32"]
pub mod clksel_con32;
#[doc = "CLKSEL_CON33 (rw) register accessor: Internal clock select and divide register33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con33`]
module"]
#[doc(alias = "CLKSEL_CON33")]
pub type ClkselCon33 = crate::Reg<clksel_con33::ClkselCon33Spec>;
#[doc = "Internal clock select and divide register33"]
pub mod clksel_con33;
#[doc = "CLKSEL_CON34 (rw) register accessor: Internal clock select and divide register34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con34`]
module"]
#[doc(alias = "CLKSEL_CON34")]
pub type ClkselCon34 = crate::Reg<clksel_con34::ClkselCon34Spec>;
#[doc = "Internal clock select and divide register34"]
pub mod clksel_con34;
#[doc = "CLKSEL_CON35 (rw) register accessor: Internal clock select and divide register35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con35`]
module"]
#[doc(alias = "CLKSEL_CON35")]
pub type ClkselCon35 = crate::Reg<clksel_con35::ClkselCon35Spec>;
#[doc = "Internal clock select and divide register35"]
pub mod clksel_con35;
#[doc = "CLKSEL_CON36 (rw) register accessor: Internal clock select and divide register36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con36`]
module"]
#[doc(alias = "CLKSEL_CON36")]
pub type ClkselCon36 = crate::Reg<clksel_con36::ClkselCon36Spec>;
#[doc = "Internal clock select and divide register36"]
pub mod clksel_con36;
#[doc = "CLKSEL_CON38 (rw) register accessor: Internal clock select and divide register38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con38`]
module"]
#[doc(alias = "CLKSEL_CON38")]
pub type ClkselCon38 = crate::Reg<clksel_con38::ClkselCon38Spec>;
#[doc = "Internal clock select and divide register38"]
pub mod clksel_con38;
#[doc = "CLKSEL_CON39 (rw) register accessor: Internal clock select and divide register39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con39`]
module"]
#[doc(alias = "CLKSEL_CON39")]
pub type ClkselCon39 = crate::Reg<clksel_con39::ClkselCon39Spec>;
#[doc = "Internal clock select and divide register39"]
pub mod clksel_con39;
#[doc = "CLKSEL_CON40 (rw) register accessor: Internal clock select and divide register40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con40`]
module"]
#[doc(alias = "CLKSEL_CON40")]
pub type ClkselCon40 = crate::Reg<clksel_con40::ClkselCon40Spec>;
#[doc = "Internal clock select and divide register40"]
pub mod clksel_con40;
#[doc = "CLKSEL_CON41 (rw) register accessor: Internal clock select and divide register41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con41`]
module"]
#[doc(alias = "CLKSEL_CON41")]
pub type ClkselCon41 = crate::Reg<clksel_con41::ClkselCon41Spec>;
#[doc = "Internal clock select and divide register41"]
pub mod clksel_con41;
#[doc = "CLKSEL_CON42 (rw) register accessor: Internal clock select and divide register42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con42`]
module"]
#[doc(alias = "CLKSEL_CON42")]
pub type ClkselCon42 = crate::Reg<clksel_con42::ClkselCon42Spec>;
#[doc = "Internal clock select and divide register42"]
pub mod clksel_con42;
#[doc = "CLKSEL_CON43 (rw) register accessor: Internal clock select and divide register43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con43`]
module"]
#[doc(alias = "CLKSEL_CON43")]
pub type ClkselCon43 = crate::Reg<clksel_con43::ClkselCon43Spec>;
#[doc = "Internal clock select and divide register43"]
pub mod clksel_con43;
#[doc = "CLKSEL_CON44 (rw) register accessor: Internal clock select and divide register44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con44`]
module"]
#[doc(alias = "CLKSEL_CON44")]
pub type ClkselCon44 = crate::Reg<clksel_con44::ClkselCon44Spec>;
#[doc = "Internal clock select and divide register44"]
pub mod clksel_con44;
#[doc = "CLKSEL_CON45 (rw) register accessor: Internal clock select and divide register45\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con45`]
module"]
#[doc(alias = "CLKSEL_CON45")]
pub type ClkselCon45 = crate::Reg<clksel_con45::ClkselCon45Spec>;
#[doc = "Internal clock select and divide register45"]
pub mod clksel_con45;
#[doc = "CLKSEL_CON46 (rw) register accessor: Internal clock select and divide register46\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con46`]
module"]
#[doc(alias = "CLKSEL_CON46")]
pub type ClkselCon46 = crate::Reg<clksel_con46::ClkselCon46Spec>;
#[doc = "Internal clock select and divide register46"]
pub mod clksel_con46;
#[doc = "CLKSEL_CON47 (rw) register accessor: Internal clock select and divide register47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con47`]
module"]
#[doc(alias = "CLKSEL_CON47")]
pub type ClkselCon47 = crate::Reg<clksel_con47::ClkselCon47Spec>;
#[doc = "Internal clock select and divide register47"]
pub mod clksel_con47;
#[doc = "CLKSEL_CON48 (rw) register accessor: Internal clock select and divide register48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con48`]
module"]
#[doc(alias = "CLKSEL_CON48")]
pub type ClkselCon48 = crate::Reg<clksel_con48::ClkselCon48Spec>;
#[doc = "Internal clock select and divide register48"]
pub mod clksel_con48;
#[doc = "CLKSEL_CON49 (rw) register accessor: Internal clock select and divide register49\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con49`]
module"]
#[doc(alias = "CLKSEL_CON49")]
pub type ClkselCon49 = crate::Reg<clksel_con49::ClkselCon49Spec>;
#[doc = "Internal clock select and divide register49"]
pub mod clksel_con49;
#[doc = "CLKSEL_CON50 (rw) register accessor: Internal clock select and divide register50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con50::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con50`]
module"]
#[doc(alias = "CLKSEL_CON50")]
pub type ClkselCon50 = crate::Reg<clksel_con50::ClkselCon50Spec>;
#[doc = "Internal clock select and divide register50"]
pub mod clksel_con50;
#[doc = "CLKSEL_CON51 (rw) register accessor: Internal clock select and divide register51\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con51`]
module"]
#[doc(alias = "CLKSEL_CON51")]
pub type ClkselCon51 = crate::Reg<clksel_con51::ClkselCon51Spec>;
#[doc = "Internal clock select and divide register51"]
pub mod clksel_con51;
#[doc = "CLKSEL_CON52 (rw) register accessor: Internal clock select and divide register52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con52`]
module"]
#[doc(alias = "CLKSEL_CON52")]
pub type ClkselCon52 = crate::Reg<clksel_con52::ClkselCon52Spec>;
#[doc = "Internal clock select and divide register52"]
pub mod clksel_con52;
#[doc = "CLKSEL_CON53 (rw) register accessor: Internal clock select and divide register53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con53`]
module"]
#[doc(alias = "CLKSEL_CON53")]
pub type ClkselCon53 = crate::Reg<clksel_con53::ClkselCon53Spec>;
#[doc = "Internal clock select and divide register53"]
pub mod clksel_con53;
#[doc = "CLKSEL_CON54 (rw) register accessor: Internal clock select and divide register54\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con54::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con54`]
module"]
#[doc(alias = "CLKSEL_CON54")]
pub type ClkselCon54 = crate::Reg<clksel_con54::ClkselCon54Spec>;
#[doc = "Internal clock select and divide register54"]
pub mod clksel_con54;
#[doc = "CLKSEL_CON55 (rw) register accessor: Internal clock select and divide register55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con55`]
module"]
#[doc(alias = "CLKSEL_CON55")]
pub type ClkselCon55 = crate::Reg<clksel_con55::ClkselCon55Spec>;
#[doc = "Internal clock select and divide register55"]
pub mod clksel_con55;
#[doc = "CLKSEL_CON56 (rw) register accessor: Internal clock select and divide register56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con56::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con56`]
module"]
#[doc(alias = "CLKSEL_CON56")]
pub type ClkselCon56 = crate::Reg<clksel_con56::ClkselCon56Spec>;
#[doc = "Internal clock select and divide register56"]
pub mod clksel_con56;
#[doc = "CLKSEL_CON57 (rw) register accessor: Internal clock select and divide register57\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con57::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con57::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con57`]
module"]
#[doc(alias = "CLKSEL_CON57")]
pub type ClkselCon57 = crate::Reg<clksel_con57::ClkselCon57Spec>;
#[doc = "Internal clock select and divide register57"]
pub mod clksel_con57;
#[doc = "CLKSEL_CON58 (rw) register accessor: Internal clock select and divide register58\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con58::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con58::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con58`]
module"]
#[doc(alias = "CLKSEL_CON58")]
pub type ClkselCon58 = crate::Reg<clksel_con58::ClkselCon58Spec>;
#[doc = "Internal clock select and divide register58"]
pub mod clksel_con58;
#[doc = "CLKSEL_CON59 (rw) register accessor: Internal clock select and divide register59\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con59::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con59`]
module"]
#[doc(alias = "CLKSEL_CON59")]
pub type ClkselCon59 = crate::Reg<clksel_con59::ClkselCon59Spec>;
#[doc = "Internal clock select and divide register59"]
pub mod clksel_con59;
#[doc = "CLKSEL_CON60 (rw) register accessor: Internal clock select and divide register60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con60::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con60`]
module"]
#[doc(alias = "CLKSEL_CON60")]
pub type ClkselCon60 = crate::Reg<clksel_con60::ClkselCon60Spec>;
#[doc = "Internal clock select and divide register60"]
pub mod clksel_con60;
#[doc = "CLKSEL_CON61 (rw) register accessor: Internal clock select and divide register61\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con61::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con61::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con61`]
module"]
#[doc(alias = "CLKSEL_CON61")]
pub type ClkselCon61 = crate::Reg<clksel_con61::ClkselCon61Spec>;
#[doc = "Internal clock select and divide register61"]
pub mod clksel_con61;
#[doc = "CLKSEL_CON62 (rw) register accessor: Internal clock select and divide register62\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con62::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con62::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con62`]
module"]
#[doc(alias = "CLKSEL_CON62")]
pub type ClkselCon62 = crate::Reg<clksel_con62::ClkselCon62Spec>;
#[doc = "Internal clock select and divide register62"]
pub mod clksel_con62;
#[doc = "CLKSEL_CON63 (rw) register accessor: Internal clock select and divide register63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con63::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con63`]
module"]
#[doc(alias = "CLKSEL_CON63")]
pub type ClkselCon63 = crate::Reg<clksel_con63::ClkselCon63Spec>;
#[doc = "Internal clock select and divide register63"]
pub mod clksel_con63;
#[doc = "CLKSEL_CON64 (rw) register accessor: Internal clock select and divide register64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con64`]
module"]
#[doc(alias = "CLKSEL_CON64")]
pub type ClkselCon64 = crate::Reg<clksel_con64::ClkselCon64Spec>;
#[doc = "Internal clock select and divide register64"]
pub mod clksel_con64;
#[doc = "CLKSEL_CON65 (rw) register accessor: Internal clock select and divide register65\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con65::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con65::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con65`]
module"]
#[doc(alias = "CLKSEL_CON65")]
pub type ClkselCon65 = crate::Reg<clksel_con65::ClkselCon65Spec>;
#[doc = "Internal clock select and divide register65"]
pub mod clksel_con65;
#[doc = "CLKSEL_CON96 (rw) register accessor: Internal clock select and divide register80\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con96::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con96::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con96`]
module"]
#[doc(alias = "CLKSEL_CON96")]
pub type ClkselCon96 = crate::Reg<clksel_con96::ClkselCon96Spec>;
#[doc = "Internal clock select and divide register80"]
pub mod clksel_con96;
#[doc = "CLKSEL_CON97 (rw) register accessor: Internal clock select and divide register81\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con97::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con97::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con97`]
module"]
#[doc(alias = "CLKSEL_CON97")]
pub type ClkselCon97 = crate::Reg<clksel_con97::ClkselCon97Spec>;
#[doc = "Internal clock select and divide register81"]
pub mod clksel_con97;
#[doc = "CLKSEL_CON98 (rw) register accessor: Internal clock select and divide register82\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con98::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con98::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con98`]
module"]
#[doc(alias = "CLKSEL_CON98")]
pub type ClkselCon98 = crate::Reg<clksel_con98::ClkselCon98Spec>;
#[doc = "Internal clock select and divide register82"]
pub mod clksel_con98;
#[doc = "CLKSEL_CON99 (rw) register accessor: Internal clock select and divide register83\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con99::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con99::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con99`]
module"]
#[doc(alias = "CLKSEL_CON99")]
pub type ClkselCon99 = crate::Reg<clksel_con99::ClkselCon99Spec>;
#[doc = "Internal clock select and divide register83"]
pub mod clksel_con99;
#[doc = "CLKSEL_CON100 (rw) register accessor: Internal clock select and divide register84\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con100::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con100::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con100`]
module"]
#[doc(alias = "CLKSEL_CON100")]
pub type ClkselCon100 = crate::Reg<clksel_con100::ClkselCon100Spec>;
#[doc = "Internal clock select and divide register84"]
pub mod clksel_con100;
#[doc = "CLKSEL_CON101 (rw) register accessor: Internal clock select and divide register85\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con101::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con101::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con101`]
module"]
#[doc(alias = "CLKSEL_CON101")]
pub type ClkselCon101 = crate::Reg<clksel_con101::ClkselCon101Spec>;
#[doc = "Internal clock select and divide register85"]
pub mod clksel_con101;
#[doc = "CLKSEL_CON102 (rw) register accessor: Internal clock select and divide register86\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con102::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con102::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con102`]
module"]
#[doc(alias = "CLKSEL_CON102")]
pub type ClkselCon102 = crate::Reg<clksel_con102::ClkselCon102Spec>;
#[doc = "Internal clock select and divide register86"]
pub mod clksel_con102;
#[doc = "CLKSEL_CON103 (rw) register accessor: Internal clock select and divide register87\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con103::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con103::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con103`]
module"]
#[doc(alias = "CLKSEL_CON103")]
pub type ClkselCon103 = crate::Reg<clksel_con103::ClkselCon103Spec>;
#[doc = "Internal clock select and divide register87"]
pub mod clksel_con103;
#[doc = "CLKSEL_CON105 (rw) register accessor: Internal clock select and divide register89\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con105::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con105::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con105`]
module"]
#[doc(alias = "CLKSEL_CON105")]
pub type ClkselCon105 = crate::Reg<clksel_con105::ClkselCon105Spec>;
#[doc = "Internal clock select and divide register89"]
pub mod clksel_con105;
#[doc = "CLKSEL_CON106 (rw) register accessor: Internal clock select and divide register90\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con106::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con106::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con106`]
module"]
#[doc(alias = "CLKSEL_CON106")]
pub type ClkselCon106 = crate::Reg<clksel_con106::ClkselCon106Spec>;
#[doc = "Internal clock select and divide register90"]
pub mod clksel_con106;
#[doc = "CLKSEL_CON107 (rw) register accessor: Internal clock select and divide register91\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con107::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con107::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel_con107`]
module"]
#[doc(alias = "CLKSEL_CON107")]
pub type ClkselCon107 = crate::Reg<clksel_con107::ClkselCon107Spec>;
#[doc = "Internal clock select and divide register91"]
pub mod clksel_con107;
#[doc = "CLKGATE_CON0 (rw) register accessor: Internal clock gating register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con0`]
module"]
#[doc(alias = "CLKGATE_CON0")]
pub type ClkgateCon0 = crate::Reg<clkgate_con0::ClkgateCon0Spec>;
#[doc = "Internal clock gating register0"]
pub mod clkgate_con0;
#[doc = "CLKGATE_CON1 (rw) register accessor: Internal clock gating register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con1`]
module"]
#[doc(alias = "CLKGATE_CON1")]
pub type ClkgateCon1 = crate::Reg<clkgate_con1::ClkgateCon1Spec>;
#[doc = "Internal clock gating register1"]
pub mod clkgate_con1;
#[doc = "CLKGATE_CON2 (rw) register accessor: Internal clock gating register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con2`]
module"]
#[doc(alias = "CLKGATE_CON2")]
pub type ClkgateCon2 = crate::Reg<clkgate_con2::ClkgateCon2Spec>;
#[doc = "Internal clock gating register2"]
pub mod clkgate_con2;
#[doc = "CLKGATE_CON3 (rw) register accessor: Internal clock gating register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con3`]
module"]
#[doc(alias = "CLKGATE_CON3")]
pub type ClkgateCon3 = crate::Reg<clkgate_con3::ClkgateCon3Spec>;
#[doc = "Internal clock gating register3"]
pub mod clkgate_con3;
#[doc = "CLKGATE_CON4 (rw) register accessor: Internal clock gating register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con4`]
module"]
#[doc(alias = "CLKGATE_CON4")]
pub type ClkgateCon4 = crate::Reg<clkgate_con4::ClkgateCon4Spec>;
#[doc = "Internal clock gating register4"]
pub mod clkgate_con4;
#[doc = "CLKGATE_CON5 (rw) register accessor: Internal clock gating register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con5`]
module"]
#[doc(alias = "CLKGATE_CON5")]
pub type ClkgateCon5 = crate::Reg<clkgate_con5::ClkgateCon5Spec>;
#[doc = "Internal clock gating register5"]
pub mod clkgate_con5;
#[doc = "CLKGATE_CON6 (rw) register accessor: Internal clock gating register6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con6`]
module"]
#[doc(alias = "CLKGATE_CON6")]
pub type ClkgateCon6 = crate::Reg<clkgate_con6::ClkgateCon6Spec>;
#[doc = "Internal clock gating register6"]
pub mod clkgate_con6;
#[doc = "CLKGATE_CON7 (rw) register accessor: Internal clock gating register7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con7`]
module"]
#[doc(alias = "CLKGATE_CON7")]
pub type ClkgateCon7 = crate::Reg<clkgate_con7::ClkgateCon7Spec>;
#[doc = "Internal clock gating register7"]
pub mod clkgate_con7;
#[doc = "CLKGATE_CON8 (rw) register accessor: Internal clock gating register8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con8`]
module"]
#[doc(alias = "CLKGATE_CON8")]
pub type ClkgateCon8 = crate::Reg<clkgate_con8::ClkgateCon8Spec>;
#[doc = "Internal clock gating register8"]
pub mod clkgate_con8;
#[doc = "CLKGATE_CON9 (rw) register accessor: Internal clock gating register9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con9`]
module"]
#[doc(alias = "CLKGATE_CON9")]
pub type ClkgateCon9 = crate::Reg<clkgate_con9::ClkgateCon9Spec>;
#[doc = "Internal clock gating register9"]
pub mod clkgate_con9;
#[doc = "CLKGATE_CON10 (rw) register accessor: Internal clock gating register10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con10`]
module"]
#[doc(alias = "CLKGATE_CON10")]
pub type ClkgateCon10 = crate::Reg<clkgate_con10::ClkgateCon10Spec>;
#[doc = "Internal clock gating register10"]
pub mod clkgate_con10;
#[doc = "CLKGATE_CON11 (rw) register accessor: Internal clock gating register11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con11`]
module"]
#[doc(alias = "CLKGATE_CON11")]
pub type ClkgateCon11 = crate::Reg<clkgate_con11::ClkgateCon11Spec>;
#[doc = "Internal clock gating register11"]
pub mod clkgate_con11;
#[doc = "CLKGATE_CON12 (rw) register accessor: Internal clock gating register12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con12`]
module"]
#[doc(alias = "CLKGATE_CON12")]
pub type ClkgateCon12 = crate::Reg<clkgate_con12::ClkgateCon12Spec>;
#[doc = "Internal clock gating register12"]
pub mod clkgate_con12;
#[doc = "CLKGATE_CON13 (rw) register accessor: Internal clock gating register13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con13`]
module"]
#[doc(alias = "CLKGATE_CON13")]
pub type ClkgateCon13 = crate::Reg<clkgate_con13::ClkgateCon13Spec>;
#[doc = "Internal clock gating register13"]
pub mod clkgate_con13;
#[doc = "CLKGATE_CON14 (rw) register accessor: Internal clock gating register14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con14`]
module"]
#[doc(alias = "CLKGATE_CON14")]
pub type ClkgateCon14 = crate::Reg<clkgate_con14::ClkgateCon14Spec>;
#[doc = "Internal clock gating register14"]
pub mod clkgate_con14;
#[doc = "CLKGATE_CON15 (rw) register accessor: Internal clock gating register15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con15`]
module"]
#[doc(alias = "CLKGATE_CON15")]
pub type ClkgateCon15 = crate::Reg<clkgate_con15::ClkgateCon15Spec>;
#[doc = "Internal clock gating register15"]
pub mod clkgate_con15;
#[doc = "CLKGATE_CON16 (rw) register accessor: Internal clock gating register16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con16`]
module"]
#[doc(alias = "CLKGATE_CON16")]
pub type ClkgateCon16 = crate::Reg<clkgate_con16::ClkgateCon16Spec>;
#[doc = "Internal clock gating register16"]
pub mod clkgate_con16;
#[doc = "CLKGATE_CON17 (rw) register accessor: Internal clock gating register17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con17`]
module"]
#[doc(alias = "CLKGATE_CON17")]
pub type ClkgateCon17 = crate::Reg<clkgate_con17::ClkgateCon17Spec>;
#[doc = "Internal clock gating register17"]
pub mod clkgate_con17;
#[doc = "CLKGATE_CON18 (rw) register accessor: Internal clock gating register18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con18`]
module"]
#[doc(alias = "CLKGATE_CON18")]
pub type ClkgateCon18 = crate::Reg<clkgate_con18::ClkgateCon18Spec>;
#[doc = "Internal clock gating register18"]
pub mod clkgate_con18;
#[doc = "CLKGATE_CON19 (rw) register accessor: Internal clock gating register19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con19`]
module"]
#[doc(alias = "CLKGATE_CON19")]
pub type ClkgateCon19 = crate::Reg<clkgate_con19::ClkgateCon19Spec>;
#[doc = "Internal clock gating register19"]
pub mod clkgate_con19;
#[doc = "CLKGATE_CON20 (rw) register accessor: Internal clock gating register20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con20`]
module"]
#[doc(alias = "CLKGATE_CON20")]
pub type ClkgateCon20 = crate::Reg<clkgate_con20::ClkgateCon20Spec>;
#[doc = "Internal clock gating register20"]
pub mod clkgate_con20;
#[doc = "CLKGATE_CON21 (rw) register accessor: Internal clock gating register21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con21`]
module"]
#[doc(alias = "CLKGATE_CON21")]
pub type ClkgateCon21 = crate::Reg<clkgate_con21::ClkgateCon21Spec>;
#[doc = "Internal clock gating register21"]
pub mod clkgate_con21;
#[doc = "CLKGATE_CON22 (rw) register accessor: Internal clock gating register22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con22`]
module"]
#[doc(alias = "CLKGATE_CON22")]
pub type ClkgateCon22 = crate::Reg<clkgate_con22::ClkgateCon22Spec>;
#[doc = "Internal clock gating register22"]
pub mod clkgate_con22;
#[doc = "CLKGATE_CON23 (rw) register accessor: Internal clock gating register23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con23`]
module"]
#[doc(alias = "CLKGATE_CON23")]
pub type ClkgateCon23 = crate::Reg<clkgate_con23::ClkgateCon23Spec>;
#[doc = "Internal clock gating register23"]
pub mod clkgate_con23;
#[doc = "CLKGATE_CON24 (rw) register accessor: Internal clock gating register24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con24`]
module"]
#[doc(alias = "CLKGATE_CON24")]
pub type ClkgateCon24 = crate::Reg<clkgate_con24::ClkgateCon24Spec>;
#[doc = "Internal clock gating register24"]
pub mod clkgate_con24;
#[doc = "CLKGATE_CON25 (rw) register accessor: Internal clock gating register25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con25`]
module"]
#[doc(alias = "CLKGATE_CON25")]
pub type ClkgateCon25 = crate::Reg<clkgate_con25::ClkgateCon25Spec>;
#[doc = "Internal clock gating register25"]
pub mod clkgate_con25;
#[doc = "CLKGATE_CON26 (rw) register accessor: Internal clock gating register26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con26`]
module"]
#[doc(alias = "CLKGATE_CON26")]
pub type ClkgateCon26 = crate::Reg<clkgate_con26::ClkgateCon26Spec>;
#[doc = "Internal clock gating register26"]
pub mod clkgate_con26;
#[doc = "CLKGATE_CON27 (rw) register accessor: Internal clock gating register27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con27`]
module"]
#[doc(alias = "CLKGATE_CON27")]
pub type ClkgateCon27 = crate::Reg<clkgate_con27::ClkgateCon27Spec>;
#[doc = "Internal clock gating register27"]
pub mod clkgate_con27;
#[doc = "CLKGATE_CON28 (rw) register accessor: Internal clock gating register28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con28`]
module"]
#[doc(alias = "CLKGATE_CON28")]
pub type ClkgateCon28 = crate::Reg<clkgate_con28::ClkgateCon28Spec>;
#[doc = "Internal clock gating register28"]
pub mod clkgate_con28;
#[doc = "CLKGATE_CON29 (rw) register accessor: Internal clock gating register29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con29`]
module"]
#[doc(alias = "CLKGATE_CON29")]
pub type ClkgateCon29 = crate::Reg<clkgate_con29::ClkgateCon29Spec>;
#[doc = "Internal clock gating register29"]
pub mod clkgate_con29;
#[doc = "CLKGATE_CON30 (rw) register accessor: Internal clock gating register30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con30`]
module"]
#[doc(alias = "CLKGATE_CON30")]
pub type ClkgateCon30 = crate::Reg<clkgate_con30::ClkgateCon30Spec>;
#[doc = "Internal clock gating register30"]
pub mod clkgate_con30;
#[doc = "CLKGATE_CON31 (rw) register accessor: Internal clock gating register31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con31`]
module"]
#[doc(alias = "CLKGATE_CON31")]
pub type ClkgateCon31 = crate::Reg<clkgate_con31::ClkgateCon31Spec>;
#[doc = "Internal clock gating register31"]
pub mod clkgate_con31;
#[doc = "CLKGATE_CON32 (rw) register accessor: Internal clock gating register32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con32`]
module"]
#[doc(alias = "CLKGATE_CON32")]
pub type ClkgateCon32 = crate::Reg<clkgate_con32::ClkgateCon32Spec>;
#[doc = "Internal clock gating register32"]
pub mod clkgate_con32;
#[doc = "CLKGATE_CON33 (rw) register accessor: Internal clock gating register33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con33`]
module"]
#[doc(alias = "CLKGATE_CON33")]
pub type ClkgateCon33 = crate::Reg<clkgate_con33::ClkgateCon33Spec>;
#[doc = "Internal clock gating register33"]
pub mod clkgate_con33;
#[doc = "CLKGATE_CON34 (rw) register accessor: Internal clock gating register34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkgate_con34`]
module"]
#[doc(alias = "CLKGATE_CON34")]
pub type ClkgateCon34 = crate::Reg<clkgate_con34::ClkgateCon34Spec>;
#[doc = "Internal clock gating register34"]
pub mod clkgate_con34;
#[doc = "SOFTRST_CON0 (rw) register accessor: Internal software reset control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con0`]
module"]
#[doc(alias = "SOFTRST_CON0")]
pub type SoftrstCon0 = crate::Reg<softrst_con0::SoftrstCon0Spec>;
#[doc = "Internal software reset control register0"]
pub mod softrst_con0;
#[doc = "SOFTRST_CON1 (rw) register accessor: Internal software reset control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con1`]
module"]
#[doc(alias = "SOFTRST_CON1")]
pub type SoftrstCon1 = crate::Reg<softrst_con1::SoftrstCon1Spec>;
#[doc = "Internal software reset control register1"]
pub mod softrst_con1;
#[doc = "SOFTRST_CON2 (rw) register accessor: Internal software reset control register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con2`]
module"]
#[doc(alias = "SOFTRST_CON2")]
pub type SoftrstCon2 = crate::Reg<softrst_con2::SoftrstCon2Spec>;
#[doc = "Internal software reset control register2"]
pub mod softrst_con2;
#[doc = "SOFTRST_CON3 (rw) register accessor: Internal software reset control register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con3`]
module"]
#[doc(alias = "SOFTRST_CON3")]
pub type SoftrstCon3 = crate::Reg<softrst_con3::SoftrstCon3Spec>;
#[doc = "Internal software reset control register3"]
pub mod softrst_con3;
#[doc = "SOFTRST_CON4 (rw) register accessor: Internal software reset control register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con4`]
module"]
#[doc(alias = "SOFTRST_CON4")]
pub type SoftrstCon4 = crate::Reg<softrst_con4::SoftrstCon4Spec>;
#[doc = "Internal software reset control register4"]
pub mod softrst_con4;
#[doc = "SOFTRST_CON5 (rw) register accessor: Internal software reset control register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con5`]
module"]
#[doc(alias = "SOFTRST_CON5")]
pub type SoftrstCon5 = crate::Reg<softrst_con5::SoftrstCon5Spec>;
#[doc = "Internal software reset control register5"]
pub mod softrst_con5;
#[doc = "SOFTRST_CON6 (rw) register accessor: Internal software reset control register6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con6`]
module"]
#[doc(alias = "SOFTRST_CON6")]
pub type SoftrstCon6 = crate::Reg<softrst_con6::SoftrstCon6Spec>;
#[doc = "Internal software reset control register6"]
pub mod softrst_con6;
#[doc = "SOFTRST_CON7 (rw) register accessor: Internal software reset control register7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con7`]
module"]
#[doc(alias = "SOFTRST_CON7")]
pub type SoftrstCon7 = crate::Reg<softrst_con7::SoftrstCon7Spec>;
#[doc = "Internal software reset control register7"]
pub mod softrst_con7;
#[doc = "SOFTRST_CON8 (rw) register accessor: Internal software reset control register8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con8`]
module"]
#[doc(alias = "SOFTRST_CON8")]
pub type SoftrstCon8 = crate::Reg<softrst_con8::SoftrstCon8Spec>;
#[doc = "Internal software reset control register8"]
pub mod softrst_con8;
#[doc = "SOFTRST_CON9 (rw) register accessor: Internal software reset control register9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con9`]
module"]
#[doc(alias = "SOFTRST_CON9")]
pub type SoftrstCon9 = crate::Reg<softrst_con9::SoftrstCon9Spec>;
#[doc = "Internal software reset control register9"]
pub mod softrst_con9;
#[doc = "SOFTRST_CON10 (rw) register accessor: Internal software reset control register10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con10`]
module"]
#[doc(alias = "SOFTRST_CON10")]
pub type SoftrstCon10 = crate::Reg<softrst_con10::SoftrstCon10Spec>;
#[doc = "Internal software reset control register10"]
pub mod softrst_con10;
#[doc = "SOFTRST_CON11 (rw) register accessor: Internal software reset control register11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con11`]
module"]
#[doc(alias = "SOFTRST_CON11")]
pub type SoftrstCon11 = crate::Reg<softrst_con11::SoftrstCon11Spec>;
#[doc = "Internal software reset control register11"]
pub mod softrst_con11;
#[doc = "SOFTRST_CON12 (rw) register accessor: Internal software reset control register12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con12`]
module"]
#[doc(alias = "SOFTRST_CON12")]
pub type SoftrstCon12 = crate::Reg<softrst_con12::SoftrstCon12Spec>;
#[doc = "Internal software reset control register12"]
pub mod softrst_con12;
#[doc = "SOFTRST_CON13 (rw) register accessor: Internal software reset control register13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con13`]
module"]
#[doc(alias = "SOFTRST_CON13")]
pub type SoftrstCon13 = crate::Reg<softrst_con13::SoftrstCon13Spec>;
#[doc = "Internal software reset control register13"]
pub mod softrst_con13;
#[doc = "SOFTRST_CON14 (rw) register accessor: Internal software reset control register14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con14`]
module"]
#[doc(alias = "SOFTRST_CON14")]
pub type SoftrstCon14 = crate::Reg<softrst_con14::SoftrstCon14Spec>;
#[doc = "Internal software reset control register14"]
pub mod softrst_con14;
#[doc = "SOFTRST_CON15 (rw) register accessor: Internal software reset control register15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con15`]
module"]
#[doc(alias = "SOFTRST_CON15")]
pub type SoftrstCon15 = crate::Reg<softrst_con15::SoftrstCon15Spec>;
#[doc = "Internal software reset control register15"]
pub mod softrst_con15;
#[doc = "SOFTRST_CON16 (rw) register accessor: Internal software reset control register16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con16`]
module"]
#[doc(alias = "SOFTRST_CON16")]
pub type SoftrstCon16 = crate::Reg<softrst_con16::SoftrstCon16Spec>;
#[doc = "Internal software reset control register16"]
pub mod softrst_con16;
#[doc = "SOFTRST_CON17 (rw) register accessor: Internal software reset control register17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con17`]
module"]
#[doc(alias = "SOFTRST_CON17")]
pub type SoftrstCon17 = crate::Reg<softrst_con17::SoftrstCon17Spec>;
#[doc = "Internal software reset control register17"]
pub mod softrst_con17;
#[doc = "SOFTRST_CON18 (rw) register accessor: Internal software reset control register18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con18`]
module"]
#[doc(alias = "SOFTRST_CON18")]
pub type SoftrstCon18 = crate::Reg<softrst_con18::SoftrstCon18Spec>;
#[doc = "Internal software reset control register18"]
pub mod softrst_con18;
#[doc = "SOFTRST_CON19 (rw) register accessor: Internal software reset control register19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con19`]
module"]
#[doc(alias = "SOFTRST_CON19")]
pub type SoftrstCon19 = crate::Reg<softrst_con19::SoftrstCon19Spec>;
#[doc = "Internal software reset control register19"]
pub mod softrst_con19;
#[doc = "SOFTRST_CON20 (rw) register accessor: Internal software reset control register20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softrst_con20`]
module"]
#[doc(alias = "SOFTRST_CON20")]
pub type SoftrstCon20 = crate::Reg<softrst_con20::SoftrstCon20Spec>;
#[doc = "Internal software reset control register20"]
pub mod softrst_con20;
#[doc = "GLB_SRST_FST_VALUE (rw) register accessor: The first global software reset config value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_srst_fst_value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_srst_fst_value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_srst_fst_value`]
module"]
#[doc(alias = "GLB_SRST_FST_VALUE")]
pub type GlbSrstFstValue = crate::Reg<glb_srst_fst_value::GlbSrstFstValueSpec>;
#[doc = "The first global software reset config value"]
pub mod glb_srst_fst_value;
#[doc = "GLB_SRST_SND_VALUE (rw) register accessor: The second global software reset config value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_srst_snd_value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_srst_snd_value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_srst_snd_value`]
module"]
#[doc(alias = "GLB_SRST_SND_VALUE")]
pub type GlbSrstSndValue = crate::Reg<glb_srst_snd_value::GlbSrstSndValueSpec>;
#[doc = "The second global software reset config value"]
pub mod glb_srst_snd_value;
#[doc = "GLB_CNT_TH (rw) register accessor: Global soft reset counter threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_cnt_th::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_cnt_th::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_cnt_th`]
module"]
#[doc(alias = "GLB_CNT_TH")]
pub type GlbCntTh = crate::Reg<glb_cnt_th::GlbCntThSpec>;
#[doc = "Global soft reset counter threshold"]
pub mod glb_cnt_th;
#[doc = "MISC_CON (rw) register accessor: Output clock selection for test\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_con`]
module"]
#[doc(alias = "MISC_CON")]
pub type MiscCon = crate::Reg<misc_con::MiscConSpec>;
#[doc = "Output clock selection for test"]
pub mod misc_con;
#[doc = "GLB_RST_CON (rw) register accessor: Global reset trigger select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_rst_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_rst_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_rst_con`]
module"]
#[doc(alias = "GLB_RST_CON")]
pub type GlbRstCon = crate::Reg<glb_rst_con::GlbRstConSpec>;
#[doc = "Global reset trigger select"]
pub mod glb_rst_con;
#[doc = "GLB_RST_ST (rw) register accessor: Global reset status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_rst_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_rst_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_rst_st`]
module"]
#[doc(alias = "GLB_RST_ST")]
pub type GlbRstSt = crate::Reg<glb_rst_st::GlbRstStSpec>;
#[doc = "Global reset status"]
pub mod glb_rst_st;
#[doc = "SDMMC_CON0 (w) register accessor: sdmmc control0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_con0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_con0`]
module"]
#[doc(alias = "SDMMC_CON0")]
pub type SdmmcCon0 = crate::Reg<sdmmc_con0::SdmmcCon0Spec>;
#[doc = "sdmmc control0"]
pub mod sdmmc_con0;
#[doc = "SDMMC_CON1 (w) register accessor: sdmmc control1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_con1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc_con1`]
module"]
#[doc(alias = "SDMMC_CON1")]
pub type SdmmcCon1 = crate::Reg<sdmmc_con1::SdmmcCon1Spec>;
#[doc = "sdmmc control1"]
pub mod sdmmc_con1;
#[doc = "SDIO0_CON0 (w) register accessor: sdio0 control0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio0_con0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio0_con0`]
module"]
#[doc(alias = "SDIO0_CON0")]
pub type Sdio0Con0 = crate::Reg<sdio0_con0::Sdio0Con0Spec>;
#[doc = "sdio0 control0"]
pub mod sdio0_con0;
#[doc = "SDIO0_CON1 (w) register accessor: sdio0 control1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio0_con1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio0_con1`]
module"]
#[doc(alias = "SDIO0_CON1")]
pub type Sdio0Con1 = crate::Reg<sdio0_con1::Sdio0Con1Spec>;
#[doc = "sdio0 control1"]
pub mod sdio0_con1;
