#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cru_lpll_con0: CruLpllCon0,
    cru_lpll_con1: CruLpllCon1,
    cru_lpll_con2: CruLpllCon2,
    cru_lpll_con3: CruLpllCon3,
    cru_lpll_con4: CruLpllCon4,
    cru_lpll_con5: CruLpllCon5,
    _reserved6: [u8; 0x08],
    cru_bpll_con0: CruBpllCon0,
    cru_bpll_con1: CruBpllCon1,
    cru_bpll_con2: CruBpllCon2,
    cru_bpll_con3: CruBpllCon3,
    cru_bpll_con4: CruBpllCon4,
    cru_bpll_con5: CruBpllCon5,
    _reserved12: [u8; 0x08],
    cru_dpll_con0: CruDpllCon0,
    cru_dpll_con1: CruDpllCon1,
    cru_dpll_con2: CruDpllCon2,
    cru_dpll_con3: CruDpllCon3,
    cru_dpll_con4: CruDpllCon4,
    cru_dpll_con5: CruDpllCon5,
    _reserved18: [u8; 0x08],
    cru_cpll_con0: CruCpllCon0,
    cru_cpll_con1: CruCpllCon1,
    cru_cpll_con2: CruCpllCon2,
    cru_cpll_con3: CruCpllCon3,
    cru_cpll_con4: CruCpllCon4,
    cru_cpll_con5: CruCpllCon5,
    _reserved24: [u8; 0x08],
    cru_gpll_con0: CruGpllCon0,
    cru_gpll_con1: CruGpllCon1,
    cru_gpll_con2: CruGpllCon2,
    cru_gpll_con3: CruGpllCon3,
    cru_gpll_con4: CruGpllCon4,
    cru_gpll_con5: CruGpllCon5,
    _reserved30: [u8; 0x08],
    cru_npll_con0: CruNpllCon0,
    cru_npll_con1: CruNpllCon1,
    cru_npll_con2: CruNpllCon2,
    cru_npll_con3: CruNpllCon3,
    cru_npll_con4: CruNpllCon4,
    cru_npll_con5: CruNpllCon5,
    _reserved36: [u8; 0x08],
    cru_vpll_con0: CruVpllCon0,
    cru_vpll_con1: CruVpllCon1,
    cru_vpll_con2: CruVpllCon2,
    cru_vpll_con3: CruVpllCon3,
    cru_vpll_con4: CruVpllCon4,
    cru_vpll_con5: CruVpllCon5,
    _reserved42: [u8; 0x28],
    cru_clksel_con0: CruClkselCon0,
    cru_clksel_con1: CruClkselCon1,
    cru_clksel_con2: CruClkselCon2,
    cru_clksel_con3: CruClkselCon3,
    cru_clksel_con4: CruClkselCon4,
    cru_clksel_con5: CruClkselCon5,
    cru_clksel_con6: CruClkselCon6,
    cru_clksel_con7: CruClkselCon7,
    cru_clksel_con8: CruClkselCon8,
    cru_clksel_con9: CruClkselCon9,
    cru_clksel_con10: CruClkselCon10,
    cru_clksel_con11: CruClkselCon11,
    cru_clksel_con12: CruClkselCon12,
    cru_clksel_con13: CruClkselCon13,
    cru_clksel_con14: CruClkselCon14,
    cru_clksel_con15: CruClkselCon15,
    cru_clksel_con16: CruClkselCon16,
    cru_clksel_con17: CruClkselCon17,
    cru_clksel_con18: CruClkselCon18,
    cru_clksel_con19: CruClkselCon19,
    cru_clksel_con20: CruClkselCon20,
    cru_clksel_con21: CruClkselCon21,
    cru_clksel_con22: CruClkselCon22,
    cru_clksel_con23: CruClkselCon23,
    cru_clksel_con24: CruClkselCon24,
    cru_clksel_con25: CruClkselCon25,
    cru_clksel_con26: CruClkselCon26,
    cru_clksel_con27: CruClkselCon27,
    cru_clksel_con28: CruClkselCon28,
    cru_clksel_con29: CruClkselCon29,
    cru_clksel_con30: CruClkselCon30,
    cru_clksel_con31: CruClkselCon31,
    cru_clksel_con32: CruClkselCon32,
    cru_clksel_con33: CruClkselCon33,
    cru_clksel_con34: CruClkselCon34,
    cru_clksel_con35: CruClkselCon35,
    cru_clksel_con36: CruClkselCon36,
    _reserved79: [u8; 0x04],
    cru_clksel_con38: CruClkselCon38,
    cru_clksel_con39: CruClkselCon39,
    cru_clksel_con40: CruClkselCon40,
    cru_clksel_con41: CruClkselCon41,
    cru_clksel_con42: CruClkselCon42,
    cru_clksel_con43: CruClkselCon43,
    cru_clksel_con44: CruClkselCon44,
    cru_clksel_con45: CruClkselCon45,
    cru_clksel_con46: CruClkselCon46,
    cru_clksel_con47: CruClkselCon47,
    cru_clksel_con48: CruClkselCon48,
    cru_clksel_con49: CruClkselCon49,
    cru_clksel_con50: CruClkselCon50,
    cru_clksel_con51: CruClkselCon51,
    cru_clksel_con52: CruClkselCon52,
    cru_clksel_con53: CruClkselCon53,
    cru_clksel_con54: CruClkselCon54,
    cru_clksel_con55: CruClkselCon55,
    cru_clksel_con56: CruClkselCon56,
    cru_clksel_con57: CruClkselCon57,
    cru_clksel_con58: CruClkselCon58,
    cru_clksel_con59: CruClkselCon59,
    cru_clksel_con60: CruClkselCon60,
    cru_clksel_con61: CruClkselCon61,
    cru_clksel_con62: CruClkselCon62,
    cru_clksel_con63: CruClkselCon63,
    cru_clksel_con64: CruClkselCon64,
    cru_clksel_con65: CruClkselCon65,
    _reserved107: [u8; 0x78],
    cru_clksel_con96: CruClkselCon96,
    cru_clksel_con97: CruClkselCon97,
    cru_clksel_con98: CruClkselCon98,
    cru_clksel_con99: CruClkselCon99,
    cru_clksel_con100: CruClkselCon100,
    cru_clksel_con101: CruClkselCon101,
    cru_clksel_con102: CruClkselCon102,
    cru_clksel_con103: CruClkselCon103,
    _reserved115: [u8; 0x04],
    cru_clksel_con105: CruClkselCon105,
    cru_clksel_con106: CruClkselCon106,
    cru_clksel_con107: CruClkselCon107,
    _reserved118: [u8; 0x50],
    cru_clkgate_con0: CruClkgateCon0,
    cru_clkgate_con1: CruClkgateCon1,
    cru_clkgate_con2: CruClkgateCon2,
    cru_clkgate_con3: CruClkgateCon3,
    cru_clkgate_con4: CruClkgateCon4,
    cru_clkgate_con5: CruClkgateCon5,
    cru_clkgate_con6: CruClkgateCon6,
    cru_clkgate_con7: CruClkgateCon7,
    cru_clkgate_con8: CruClkgateCon8,
    cru_clkgate_con9: CruClkgateCon9,
    cru_clkgate_con10: CruClkgateCon10,
    cru_clkgate_con11: CruClkgateCon11,
    cru_clkgate_con12: CruClkgateCon12,
    cru_clkgate_con13: CruClkgateCon13,
    cru_clkgate_con14: CruClkgateCon14,
    cru_clkgate_con15: CruClkgateCon15,
    cru_clkgate_con16: CruClkgateCon16,
    cru_clkgate_con17: CruClkgateCon17,
    cru_clkgate_con18: CruClkgateCon18,
    cru_clkgate_con19: CruClkgateCon19,
    cru_clkgate_con20: CruClkgateCon20,
    cru_clkgate_con21: CruClkgateCon21,
    cru_clkgate_con22: CruClkgateCon22,
    cru_clkgate_con23: CruClkgateCon23,
    cru_clkgate_con24: CruClkgateCon24,
    cru_clkgate_con25: CruClkgateCon25,
    cru_clkgate_con26: CruClkgateCon26,
    cru_clkgate_con27: CruClkgateCon27,
    cru_clkgate_con28: CruClkgateCon28,
    cru_clkgate_con29: CruClkgateCon29,
    cru_clkgate_con30: CruClkgateCon30,
    cru_clkgate_con31: CruClkgateCon31,
    cru_clkgate_con32: CruClkgateCon32,
    cru_clkgate_con33: CruClkgateCon33,
    cru_clkgate_con34: CruClkgateCon34,
    _reserved153: [u8; 0x74],
    cru_softrst_con0: CruSoftrstCon0,
    cru_softrst_con1: CruSoftrstCon1,
    cru_softrst_con2: CruSoftrstCon2,
    cru_softrst_con3: CruSoftrstCon3,
    cru_softrst_con4: CruSoftrstCon4,
    cru_softrst_con5: CruSoftrstCon5,
    cru_softrst_con6: CruSoftrstCon6,
    cru_softrst_con7: CruSoftrstCon7,
    cru_softrst_con8: CruSoftrstCon8,
    cru_softrst_con9: CruSoftrstCon9,
    cru_softrst_con10: CruSoftrstCon10,
    cru_softrst_con11: CruSoftrstCon11,
    cru_softrst_con12: CruSoftrstCon12,
    cru_softrst_con13: CruSoftrstCon13,
    cru_softrst_con14: CruSoftrstCon14,
    cru_softrst_con15: CruSoftrstCon15,
    cru_softrst_con16: CruSoftrstCon16,
    cru_softrst_con17: CruSoftrstCon17,
    cru_softrst_con18: CruSoftrstCon18,
    cru_softrst_con19: CruSoftrstCon19,
    cru_softrst_con20: CruSoftrstCon20,
    _reserved174: [u8; 0xac],
    cru_glb_srst_fst_value: CruGlbSrstFstValue,
    cru_glb_srst_snd_value: CruGlbSrstSndValue,
    cru_glb_cnt_th: CruGlbCntTh,
    cru_misc_con: CruMiscCon,
    cru_glb_rst_con: CruGlbRstCon,
    cru_glb_rst_st: CruGlbRstSt,
    _reserved180: [u8; 0x68],
    cru_sdmmc_con0: CruSdmmcCon0,
    cru_sdmmc_con1: CruSdmmcCon1,
    cru_sdio0_con0: CruSdio0Con0,
    cru_sdio0_con1: CruSdio0Con1,
}
impl RegisterBlock {
    #[doc = "0x00 - LPLL configuration register0"]
    #[inline(always)]
    pub const fn cru_lpll_con0(&self) -> &CruLpllCon0 {
        &self.cru_lpll_con0
    }
    #[doc = "0x04 - LPLL configuration register1"]
    #[inline(always)]
    pub const fn cru_lpll_con1(&self) -> &CruLpllCon1 {
        &self.cru_lpll_con1
    }
    #[doc = "0x08 - LPLL configuration register2"]
    #[inline(always)]
    pub const fn cru_lpll_con2(&self) -> &CruLpllCon2 {
        &self.cru_lpll_con2
    }
    #[doc = "0x0c - LPLL configuration register3"]
    #[inline(always)]
    pub const fn cru_lpll_con3(&self) -> &CruLpllCon3 {
        &self.cru_lpll_con3
    }
    #[doc = "0x10 - LPLL configuration register4"]
    #[inline(always)]
    pub const fn cru_lpll_con4(&self) -> &CruLpllCon4 {
        &self.cru_lpll_con4
    }
    #[doc = "0x14 - LPLL configuration register5"]
    #[inline(always)]
    pub const fn cru_lpll_con5(&self) -> &CruLpllCon5 {
        &self.cru_lpll_con5
    }
    #[doc = "0x20 - BPLL configuration register0"]
    #[inline(always)]
    pub const fn cru_bpll_con0(&self) -> &CruBpllCon0 {
        &self.cru_bpll_con0
    }
    #[doc = "0x24 - BPLL configuration register1"]
    #[inline(always)]
    pub const fn cru_bpll_con1(&self) -> &CruBpllCon1 {
        &self.cru_bpll_con1
    }
    #[doc = "0x28 - BPLL configuration register2"]
    #[inline(always)]
    pub const fn cru_bpll_con2(&self) -> &CruBpllCon2 {
        &self.cru_bpll_con2
    }
    #[doc = "0x2c - BPLL configuration register3"]
    #[inline(always)]
    pub const fn cru_bpll_con3(&self) -> &CruBpllCon3 {
        &self.cru_bpll_con3
    }
    #[doc = "0x30 - BPLL configuration register4"]
    #[inline(always)]
    pub const fn cru_bpll_con4(&self) -> &CruBpllCon4 {
        &self.cru_bpll_con4
    }
    #[doc = "0x34 - BPLL configuration register5"]
    #[inline(always)]
    pub const fn cru_bpll_con5(&self) -> &CruBpllCon5 {
        &self.cru_bpll_con5
    }
    #[doc = "0x40 - DPLL configuration register0"]
    #[inline(always)]
    pub const fn cru_dpll_con0(&self) -> &CruDpllCon0 {
        &self.cru_dpll_con0
    }
    #[doc = "0x44 - DPLL configuration register1"]
    #[inline(always)]
    pub const fn cru_dpll_con1(&self) -> &CruDpllCon1 {
        &self.cru_dpll_con1
    }
    #[doc = "0x48 - DPLL configuration register2"]
    #[inline(always)]
    pub const fn cru_dpll_con2(&self) -> &CruDpllCon2 {
        &self.cru_dpll_con2
    }
    #[doc = "0x4c - DPLL configuration register3"]
    #[inline(always)]
    pub const fn cru_dpll_con3(&self) -> &CruDpllCon3 {
        &self.cru_dpll_con3
    }
    #[doc = "0x50 - DPLL configuration register4"]
    #[inline(always)]
    pub const fn cru_dpll_con4(&self) -> &CruDpllCon4 {
        &self.cru_dpll_con4
    }
    #[doc = "0x54 - DPLL configuration register5"]
    #[inline(always)]
    pub const fn cru_dpll_con5(&self) -> &CruDpllCon5 {
        &self.cru_dpll_con5
    }
    #[doc = "0x60 - CPLL configuration register0"]
    #[inline(always)]
    pub const fn cru_cpll_con0(&self) -> &CruCpllCon0 {
        &self.cru_cpll_con0
    }
    #[doc = "0x64 - CPLL configuration register1"]
    #[inline(always)]
    pub const fn cru_cpll_con1(&self) -> &CruCpllCon1 {
        &self.cru_cpll_con1
    }
    #[doc = "0x68 - CPLL configuration register2"]
    #[inline(always)]
    pub const fn cru_cpll_con2(&self) -> &CruCpllCon2 {
        &self.cru_cpll_con2
    }
    #[doc = "0x6c - CPLL configuration register3"]
    #[inline(always)]
    pub const fn cru_cpll_con3(&self) -> &CruCpllCon3 {
        &self.cru_cpll_con3
    }
    #[doc = "0x70 - CPLL configuration register4"]
    #[inline(always)]
    pub const fn cru_cpll_con4(&self) -> &CruCpllCon4 {
        &self.cru_cpll_con4
    }
    #[doc = "0x74 - CPLL configuration register5"]
    #[inline(always)]
    pub const fn cru_cpll_con5(&self) -> &CruCpllCon5 {
        &self.cru_cpll_con5
    }
    #[doc = "0x80 - GPLL configuration register0"]
    #[inline(always)]
    pub const fn cru_gpll_con0(&self) -> &CruGpllCon0 {
        &self.cru_gpll_con0
    }
    #[doc = "0x84 - GPLL configuration register1"]
    #[inline(always)]
    pub const fn cru_gpll_con1(&self) -> &CruGpllCon1 {
        &self.cru_gpll_con1
    }
    #[doc = "0x88 - GPLL configuration register2"]
    #[inline(always)]
    pub const fn cru_gpll_con2(&self) -> &CruGpllCon2 {
        &self.cru_gpll_con2
    }
    #[doc = "0x8c - GPLL configuration register3"]
    #[inline(always)]
    pub const fn cru_gpll_con3(&self) -> &CruGpllCon3 {
        &self.cru_gpll_con3
    }
    #[doc = "0x90 - GPLL configuration register4"]
    #[inline(always)]
    pub const fn cru_gpll_con4(&self) -> &CruGpllCon4 {
        &self.cru_gpll_con4
    }
    #[doc = "0x94 - GPLL configuration register5"]
    #[inline(always)]
    pub const fn cru_gpll_con5(&self) -> &CruGpllCon5 {
        &self.cru_gpll_con5
    }
    #[doc = "0xa0 - NPLL configuration register0"]
    #[inline(always)]
    pub const fn cru_npll_con0(&self) -> &CruNpllCon0 {
        &self.cru_npll_con0
    }
    #[doc = "0xa4 - NPLL configuration register1"]
    #[inline(always)]
    pub const fn cru_npll_con1(&self) -> &CruNpllCon1 {
        &self.cru_npll_con1
    }
    #[doc = "0xa8 - NPLL configuration register2"]
    #[inline(always)]
    pub const fn cru_npll_con2(&self) -> &CruNpllCon2 {
        &self.cru_npll_con2
    }
    #[doc = "0xac - NPLL configuration register3"]
    #[inline(always)]
    pub const fn cru_npll_con3(&self) -> &CruNpllCon3 {
        &self.cru_npll_con3
    }
    #[doc = "0xb0 - NPLL configuration register4"]
    #[inline(always)]
    pub const fn cru_npll_con4(&self) -> &CruNpllCon4 {
        &self.cru_npll_con4
    }
    #[doc = "0xb4 - NPLL configuration register5"]
    #[inline(always)]
    pub const fn cru_npll_con5(&self) -> &CruNpllCon5 {
        &self.cru_npll_con5
    }
    #[doc = "0xc0 - VPLL configuration register0"]
    #[inline(always)]
    pub const fn cru_vpll_con0(&self) -> &CruVpllCon0 {
        &self.cru_vpll_con0
    }
    #[doc = "0xc4 - VPLL configuration register1"]
    #[inline(always)]
    pub const fn cru_vpll_con1(&self) -> &CruVpllCon1 {
        &self.cru_vpll_con1
    }
    #[doc = "0xc8 - VPLL configuration register2"]
    #[inline(always)]
    pub const fn cru_vpll_con2(&self) -> &CruVpllCon2 {
        &self.cru_vpll_con2
    }
    #[doc = "0xcc - VPLL configuration register3"]
    #[inline(always)]
    pub const fn cru_vpll_con3(&self) -> &CruVpllCon3 {
        &self.cru_vpll_con3
    }
    #[doc = "0xd0 - VPLL configuration register4"]
    #[inline(always)]
    pub const fn cru_vpll_con4(&self) -> &CruVpllCon4 {
        &self.cru_vpll_con4
    }
    #[doc = "0xd4 - VPLL configuration register5"]
    #[inline(always)]
    pub const fn cru_vpll_con5(&self) -> &CruVpllCon5 {
        &self.cru_vpll_con5
    }
    #[doc = "0x100 - Internal clock select and divide register0"]
    #[inline(always)]
    pub const fn cru_clksel_con0(&self) -> &CruClkselCon0 {
        &self.cru_clksel_con0
    }
    #[doc = "0x104 - Internal clock select and divide register1"]
    #[inline(always)]
    pub const fn cru_clksel_con1(&self) -> &CruClkselCon1 {
        &self.cru_clksel_con1
    }
    #[doc = "0x108 - Internal clock select and divide register2"]
    #[inline(always)]
    pub const fn cru_clksel_con2(&self) -> &CruClkselCon2 {
        &self.cru_clksel_con2
    }
    #[doc = "0x10c - Internal clock select and divide register3"]
    #[inline(always)]
    pub const fn cru_clksel_con3(&self) -> &CruClkselCon3 {
        &self.cru_clksel_con3
    }
    #[doc = "0x110 - Internal clock select and divide register4"]
    #[inline(always)]
    pub const fn cru_clksel_con4(&self) -> &CruClkselCon4 {
        &self.cru_clksel_con4
    }
    #[doc = "0x114 - Internal clock select and divide register5"]
    #[inline(always)]
    pub const fn cru_clksel_con5(&self) -> &CruClkselCon5 {
        &self.cru_clksel_con5
    }
    #[doc = "0x118 - Internal clock select and divide register6"]
    #[inline(always)]
    pub const fn cru_clksel_con6(&self) -> &CruClkselCon6 {
        &self.cru_clksel_con6
    }
    #[doc = "0x11c - Internal clock select and divide register7"]
    #[inline(always)]
    pub const fn cru_clksel_con7(&self) -> &CruClkselCon7 {
        &self.cru_clksel_con7
    }
    #[doc = "0x120 - Internal clock select and divide register8"]
    #[inline(always)]
    pub const fn cru_clksel_con8(&self) -> &CruClkselCon8 {
        &self.cru_clksel_con8
    }
    #[doc = "0x124 - Internal clock select and divide register9"]
    #[inline(always)]
    pub const fn cru_clksel_con9(&self) -> &CruClkselCon9 {
        &self.cru_clksel_con9
    }
    #[doc = "0x128 - Internal clock select and divide register10"]
    #[inline(always)]
    pub const fn cru_clksel_con10(&self) -> &CruClkselCon10 {
        &self.cru_clksel_con10
    }
    #[doc = "0x12c - Internal clock select and divide register11"]
    #[inline(always)]
    pub const fn cru_clksel_con11(&self) -> &CruClkselCon11 {
        &self.cru_clksel_con11
    }
    #[doc = "0x130 - Internal clock select and divide register12"]
    #[inline(always)]
    pub const fn cru_clksel_con12(&self) -> &CruClkselCon12 {
        &self.cru_clksel_con12
    }
    #[doc = "0x134 - Internal clock select and divide register13"]
    #[inline(always)]
    pub const fn cru_clksel_con13(&self) -> &CruClkselCon13 {
        &self.cru_clksel_con13
    }
    #[doc = "0x138 - Internal clock select and divide register14"]
    #[inline(always)]
    pub const fn cru_clksel_con14(&self) -> &CruClkselCon14 {
        &self.cru_clksel_con14
    }
    #[doc = "0x13c - Internal clock select and divide register15"]
    #[inline(always)]
    pub const fn cru_clksel_con15(&self) -> &CruClkselCon15 {
        &self.cru_clksel_con15
    }
    #[doc = "0x140 - Internal clock select and divide register16"]
    #[inline(always)]
    pub const fn cru_clksel_con16(&self) -> &CruClkselCon16 {
        &self.cru_clksel_con16
    }
    #[doc = "0x144 - Internal clock select and divide register17"]
    #[inline(always)]
    pub const fn cru_clksel_con17(&self) -> &CruClkselCon17 {
        &self.cru_clksel_con17
    }
    #[doc = "0x148 - Internal clock select and divide register18"]
    #[inline(always)]
    pub const fn cru_clksel_con18(&self) -> &CruClkselCon18 {
        &self.cru_clksel_con18
    }
    #[doc = "0x14c - Internal clock select and divide register19"]
    #[inline(always)]
    pub const fn cru_clksel_con19(&self) -> &CruClkselCon19 {
        &self.cru_clksel_con19
    }
    #[doc = "0x150 - Internal clock select and divide register20"]
    #[inline(always)]
    pub const fn cru_clksel_con20(&self) -> &CruClkselCon20 {
        &self.cru_clksel_con20
    }
    #[doc = "0x154 - Internal clock select and divide register21"]
    #[inline(always)]
    pub const fn cru_clksel_con21(&self) -> &CruClkselCon21 {
        &self.cru_clksel_con21
    }
    #[doc = "0x158 - Internal clock select and divide register22"]
    #[inline(always)]
    pub const fn cru_clksel_con22(&self) -> &CruClkselCon22 {
        &self.cru_clksel_con22
    }
    #[doc = "0x15c - Internal clock select and divide register23"]
    #[inline(always)]
    pub const fn cru_clksel_con23(&self) -> &CruClkselCon23 {
        &self.cru_clksel_con23
    }
    #[doc = "0x160 - Internal clock select and divide register24"]
    #[inline(always)]
    pub const fn cru_clksel_con24(&self) -> &CruClkselCon24 {
        &self.cru_clksel_con24
    }
    #[doc = "0x164 - Internal clock select and divide register25"]
    #[inline(always)]
    pub const fn cru_clksel_con25(&self) -> &CruClkselCon25 {
        &self.cru_clksel_con25
    }
    #[doc = "0x168 - Internal clock select and divide register26"]
    #[inline(always)]
    pub const fn cru_clksel_con26(&self) -> &CruClkselCon26 {
        &self.cru_clksel_con26
    }
    #[doc = "0x16c - Internal clock select and divide register27"]
    #[inline(always)]
    pub const fn cru_clksel_con27(&self) -> &CruClkselCon27 {
        &self.cru_clksel_con27
    }
    #[doc = "0x170 - Internal clock select and divide register28"]
    #[inline(always)]
    pub const fn cru_clksel_con28(&self) -> &CruClkselCon28 {
        &self.cru_clksel_con28
    }
    #[doc = "0x174 - Internal clock select and divide register29"]
    #[inline(always)]
    pub const fn cru_clksel_con29(&self) -> &CruClkselCon29 {
        &self.cru_clksel_con29
    }
    #[doc = "0x178 - Internal clock select and divide register30"]
    #[inline(always)]
    pub const fn cru_clksel_con30(&self) -> &CruClkselCon30 {
        &self.cru_clksel_con30
    }
    #[doc = "0x17c - Internal clock select and divide register31"]
    #[inline(always)]
    pub const fn cru_clksel_con31(&self) -> &CruClkselCon31 {
        &self.cru_clksel_con31
    }
    #[doc = "0x180 - Internal clock select and divide register32"]
    #[inline(always)]
    pub const fn cru_clksel_con32(&self) -> &CruClkselCon32 {
        &self.cru_clksel_con32
    }
    #[doc = "0x184 - Internal clock select and divide register33"]
    #[inline(always)]
    pub const fn cru_clksel_con33(&self) -> &CruClkselCon33 {
        &self.cru_clksel_con33
    }
    #[doc = "0x188 - Internal clock select and divide register34"]
    #[inline(always)]
    pub const fn cru_clksel_con34(&self) -> &CruClkselCon34 {
        &self.cru_clksel_con34
    }
    #[doc = "0x18c - Internal clock select and divide register35"]
    #[inline(always)]
    pub const fn cru_clksel_con35(&self) -> &CruClkselCon35 {
        &self.cru_clksel_con35
    }
    #[doc = "0x190 - Internal clock select and divide register36"]
    #[inline(always)]
    pub const fn cru_clksel_con36(&self) -> &CruClkselCon36 {
        &self.cru_clksel_con36
    }
    #[doc = "0x198 - Internal clock select and divide register38"]
    #[inline(always)]
    pub const fn cru_clksel_con38(&self) -> &CruClkselCon38 {
        &self.cru_clksel_con38
    }
    #[doc = "0x19c - Internal clock select and divide register39"]
    #[inline(always)]
    pub const fn cru_clksel_con39(&self) -> &CruClkselCon39 {
        &self.cru_clksel_con39
    }
    #[doc = "0x1a0 - Internal clock select and divide register40"]
    #[inline(always)]
    pub const fn cru_clksel_con40(&self) -> &CruClkselCon40 {
        &self.cru_clksel_con40
    }
    #[doc = "0x1a4 - Internal clock select and divide register41"]
    #[inline(always)]
    pub const fn cru_clksel_con41(&self) -> &CruClkselCon41 {
        &self.cru_clksel_con41
    }
    #[doc = "0x1a8 - Internal clock select and divide register42"]
    #[inline(always)]
    pub const fn cru_clksel_con42(&self) -> &CruClkselCon42 {
        &self.cru_clksel_con42
    }
    #[doc = "0x1ac - Internal clock select and divide register43"]
    #[inline(always)]
    pub const fn cru_clksel_con43(&self) -> &CruClkselCon43 {
        &self.cru_clksel_con43
    }
    #[doc = "0x1b0 - Internal clock select and divide register44"]
    #[inline(always)]
    pub const fn cru_clksel_con44(&self) -> &CruClkselCon44 {
        &self.cru_clksel_con44
    }
    #[doc = "0x1b4 - Internal clock select and divide register45"]
    #[inline(always)]
    pub const fn cru_clksel_con45(&self) -> &CruClkselCon45 {
        &self.cru_clksel_con45
    }
    #[doc = "0x1b8 - Internal clock select and divide register46"]
    #[inline(always)]
    pub const fn cru_clksel_con46(&self) -> &CruClkselCon46 {
        &self.cru_clksel_con46
    }
    #[doc = "0x1bc - Internal clock select and divide register47"]
    #[inline(always)]
    pub const fn cru_clksel_con47(&self) -> &CruClkselCon47 {
        &self.cru_clksel_con47
    }
    #[doc = "0x1c0 - Internal clock select and divide register48"]
    #[inline(always)]
    pub const fn cru_clksel_con48(&self) -> &CruClkselCon48 {
        &self.cru_clksel_con48
    }
    #[doc = "0x1c4 - Internal clock select and divide register49"]
    #[inline(always)]
    pub const fn cru_clksel_con49(&self) -> &CruClkselCon49 {
        &self.cru_clksel_con49
    }
    #[doc = "0x1c8 - Internal clock select and divide register50"]
    #[inline(always)]
    pub const fn cru_clksel_con50(&self) -> &CruClkselCon50 {
        &self.cru_clksel_con50
    }
    #[doc = "0x1cc - Internal clock select and divide register51"]
    #[inline(always)]
    pub const fn cru_clksel_con51(&self) -> &CruClkselCon51 {
        &self.cru_clksel_con51
    }
    #[doc = "0x1d0 - Internal clock select and divide register52"]
    #[inline(always)]
    pub const fn cru_clksel_con52(&self) -> &CruClkselCon52 {
        &self.cru_clksel_con52
    }
    #[doc = "0x1d4 - Internal clock select and divide register53"]
    #[inline(always)]
    pub const fn cru_clksel_con53(&self) -> &CruClkselCon53 {
        &self.cru_clksel_con53
    }
    #[doc = "0x1d8 - Internal clock select and divide register54"]
    #[inline(always)]
    pub const fn cru_clksel_con54(&self) -> &CruClkselCon54 {
        &self.cru_clksel_con54
    }
    #[doc = "0x1dc - Internal clock select and divide register55"]
    #[inline(always)]
    pub const fn cru_clksel_con55(&self) -> &CruClkselCon55 {
        &self.cru_clksel_con55
    }
    #[doc = "0x1e0 - Internal clock select and divide register56"]
    #[inline(always)]
    pub const fn cru_clksel_con56(&self) -> &CruClkselCon56 {
        &self.cru_clksel_con56
    }
    #[doc = "0x1e4 - Internal clock select and divide register57"]
    #[inline(always)]
    pub const fn cru_clksel_con57(&self) -> &CruClkselCon57 {
        &self.cru_clksel_con57
    }
    #[doc = "0x1e8 - Internal clock select and divide register58"]
    #[inline(always)]
    pub const fn cru_clksel_con58(&self) -> &CruClkselCon58 {
        &self.cru_clksel_con58
    }
    #[doc = "0x1ec - Internal clock select and divide register59"]
    #[inline(always)]
    pub const fn cru_clksel_con59(&self) -> &CruClkselCon59 {
        &self.cru_clksel_con59
    }
    #[doc = "0x1f0 - Internal clock select and divide register60"]
    #[inline(always)]
    pub const fn cru_clksel_con60(&self) -> &CruClkselCon60 {
        &self.cru_clksel_con60
    }
    #[doc = "0x1f4 - Internal clock select and divide register61"]
    #[inline(always)]
    pub const fn cru_clksel_con61(&self) -> &CruClkselCon61 {
        &self.cru_clksel_con61
    }
    #[doc = "0x1f8 - Internal clock select and divide register62"]
    #[inline(always)]
    pub const fn cru_clksel_con62(&self) -> &CruClkselCon62 {
        &self.cru_clksel_con62
    }
    #[doc = "0x1fc - Internal clock select and divide register63"]
    #[inline(always)]
    pub const fn cru_clksel_con63(&self) -> &CruClkselCon63 {
        &self.cru_clksel_con63
    }
    #[doc = "0x200 - Internal clock select and divide register64"]
    #[inline(always)]
    pub const fn cru_clksel_con64(&self) -> &CruClkselCon64 {
        &self.cru_clksel_con64
    }
    #[doc = "0x204 - Internal clock select and divide register65"]
    #[inline(always)]
    pub const fn cru_clksel_con65(&self) -> &CruClkselCon65 {
        &self.cru_clksel_con65
    }
    #[doc = "0x280 - Internal clock select and divide register80"]
    #[inline(always)]
    pub const fn cru_clksel_con96(&self) -> &CruClkselCon96 {
        &self.cru_clksel_con96
    }
    #[doc = "0x284 - Internal clock select and divide register81"]
    #[inline(always)]
    pub const fn cru_clksel_con97(&self) -> &CruClkselCon97 {
        &self.cru_clksel_con97
    }
    #[doc = "0x288 - Internal clock select and divide register82"]
    #[inline(always)]
    pub const fn cru_clksel_con98(&self) -> &CruClkselCon98 {
        &self.cru_clksel_con98
    }
    #[doc = "0x28c - Internal clock select and divide register83"]
    #[inline(always)]
    pub const fn cru_clksel_con99(&self) -> &CruClkselCon99 {
        &self.cru_clksel_con99
    }
    #[doc = "0x290 - Internal clock select and divide register84"]
    #[inline(always)]
    pub const fn cru_clksel_con100(&self) -> &CruClkselCon100 {
        &self.cru_clksel_con100
    }
    #[doc = "0x294 - Internal clock select and divide register85"]
    #[inline(always)]
    pub const fn cru_clksel_con101(&self) -> &CruClkselCon101 {
        &self.cru_clksel_con101
    }
    #[doc = "0x298 - Internal clock select and divide register86"]
    #[inline(always)]
    pub const fn cru_clksel_con102(&self) -> &CruClkselCon102 {
        &self.cru_clksel_con102
    }
    #[doc = "0x29c - Internal clock select and divide register87"]
    #[inline(always)]
    pub const fn cru_clksel_con103(&self) -> &CruClkselCon103 {
        &self.cru_clksel_con103
    }
    #[doc = "0x2a4 - Internal clock select and divide register89"]
    #[inline(always)]
    pub const fn cru_clksel_con105(&self) -> &CruClkselCon105 {
        &self.cru_clksel_con105
    }
    #[doc = "0x2a8 - Internal clock select and divide register90"]
    #[inline(always)]
    pub const fn cru_clksel_con106(&self) -> &CruClkselCon106 {
        &self.cru_clksel_con106
    }
    #[doc = "0x2ac - Internal clock select and divide register91"]
    #[inline(always)]
    pub const fn cru_clksel_con107(&self) -> &CruClkselCon107 {
        &self.cru_clksel_con107
    }
    #[doc = "0x300 - Internal clock gating register0"]
    #[inline(always)]
    pub const fn cru_clkgate_con0(&self) -> &CruClkgateCon0 {
        &self.cru_clkgate_con0
    }
    #[doc = "0x304 - Internal clock gating register1"]
    #[inline(always)]
    pub const fn cru_clkgate_con1(&self) -> &CruClkgateCon1 {
        &self.cru_clkgate_con1
    }
    #[doc = "0x308 - Internal clock gating register2"]
    #[inline(always)]
    pub const fn cru_clkgate_con2(&self) -> &CruClkgateCon2 {
        &self.cru_clkgate_con2
    }
    #[doc = "0x30c - Internal clock gating register3"]
    #[inline(always)]
    pub const fn cru_clkgate_con3(&self) -> &CruClkgateCon3 {
        &self.cru_clkgate_con3
    }
    #[doc = "0x310 - Internal clock gating register4"]
    #[inline(always)]
    pub const fn cru_clkgate_con4(&self) -> &CruClkgateCon4 {
        &self.cru_clkgate_con4
    }
    #[doc = "0x314 - Internal clock gating register5"]
    #[inline(always)]
    pub const fn cru_clkgate_con5(&self) -> &CruClkgateCon5 {
        &self.cru_clkgate_con5
    }
    #[doc = "0x318 - Internal clock gating register6"]
    #[inline(always)]
    pub const fn cru_clkgate_con6(&self) -> &CruClkgateCon6 {
        &self.cru_clkgate_con6
    }
    #[doc = "0x31c - Internal clock gating register7"]
    #[inline(always)]
    pub const fn cru_clkgate_con7(&self) -> &CruClkgateCon7 {
        &self.cru_clkgate_con7
    }
    #[doc = "0x320 - Internal clock gating register8"]
    #[inline(always)]
    pub const fn cru_clkgate_con8(&self) -> &CruClkgateCon8 {
        &self.cru_clkgate_con8
    }
    #[doc = "0x324 - Internal clock gating register9"]
    #[inline(always)]
    pub const fn cru_clkgate_con9(&self) -> &CruClkgateCon9 {
        &self.cru_clkgate_con9
    }
    #[doc = "0x328 - Internal clock gating register10"]
    #[inline(always)]
    pub const fn cru_clkgate_con10(&self) -> &CruClkgateCon10 {
        &self.cru_clkgate_con10
    }
    #[doc = "0x32c - Internal clock gating register11"]
    #[inline(always)]
    pub const fn cru_clkgate_con11(&self) -> &CruClkgateCon11 {
        &self.cru_clkgate_con11
    }
    #[doc = "0x330 - Internal clock gating register12"]
    #[inline(always)]
    pub const fn cru_clkgate_con12(&self) -> &CruClkgateCon12 {
        &self.cru_clkgate_con12
    }
    #[doc = "0x334 - Internal clock gating register13"]
    #[inline(always)]
    pub const fn cru_clkgate_con13(&self) -> &CruClkgateCon13 {
        &self.cru_clkgate_con13
    }
    #[doc = "0x338 - Internal clock gating register14"]
    #[inline(always)]
    pub const fn cru_clkgate_con14(&self) -> &CruClkgateCon14 {
        &self.cru_clkgate_con14
    }
    #[doc = "0x33c - Internal clock gating register15"]
    #[inline(always)]
    pub const fn cru_clkgate_con15(&self) -> &CruClkgateCon15 {
        &self.cru_clkgate_con15
    }
    #[doc = "0x340 - Internal clock gating register16"]
    #[inline(always)]
    pub const fn cru_clkgate_con16(&self) -> &CruClkgateCon16 {
        &self.cru_clkgate_con16
    }
    #[doc = "0x344 - Internal clock gating register17"]
    #[inline(always)]
    pub const fn cru_clkgate_con17(&self) -> &CruClkgateCon17 {
        &self.cru_clkgate_con17
    }
    #[doc = "0x348 - Internal clock gating register18"]
    #[inline(always)]
    pub const fn cru_clkgate_con18(&self) -> &CruClkgateCon18 {
        &self.cru_clkgate_con18
    }
    #[doc = "0x34c - Internal clock gating register19"]
    #[inline(always)]
    pub const fn cru_clkgate_con19(&self) -> &CruClkgateCon19 {
        &self.cru_clkgate_con19
    }
    #[doc = "0x350 - Internal clock gating register20"]
    #[inline(always)]
    pub const fn cru_clkgate_con20(&self) -> &CruClkgateCon20 {
        &self.cru_clkgate_con20
    }
    #[doc = "0x354 - Internal clock gating register21"]
    #[inline(always)]
    pub const fn cru_clkgate_con21(&self) -> &CruClkgateCon21 {
        &self.cru_clkgate_con21
    }
    #[doc = "0x358 - Internal clock gating register22"]
    #[inline(always)]
    pub const fn cru_clkgate_con22(&self) -> &CruClkgateCon22 {
        &self.cru_clkgate_con22
    }
    #[doc = "0x35c - Internal clock gating register23"]
    #[inline(always)]
    pub const fn cru_clkgate_con23(&self) -> &CruClkgateCon23 {
        &self.cru_clkgate_con23
    }
    #[doc = "0x360 - Internal clock gating register24"]
    #[inline(always)]
    pub const fn cru_clkgate_con24(&self) -> &CruClkgateCon24 {
        &self.cru_clkgate_con24
    }
    #[doc = "0x364 - Internal clock gating register25"]
    #[inline(always)]
    pub const fn cru_clkgate_con25(&self) -> &CruClkgateCon25 {
        &self.cru_clkgate_con25
    }
    #[doc = "0x368 - Internal clock gating register26"]
    #[inline(always)]
    pub const fn cru_clkgate_con26(&self) -> &CruClkgateCon26 {
        &self.cru_clkgate_con26
    }
    #[doc = "0x36c - Internal clock gating register27"]
    #[inline(always)]
    pub const fn cru_clkgate_con27(&self) -> &CruClkgateCon27 {
        &self.cru_clkgate_con27
    }
    #[doc = "0x370 - Internal clock gating register28"]
    #[inline(always)]
    pub const fn cru_clkgate_con28(&self) -> &CruClkgateCon28 {
        &self.cru_clkgate_con28
    }
    #[doc = "0x374 - Internal clock gating register29"]
    #[inline(always)]
    pub const fn cru_clkgate_con29(&self) -> &CruClkgateCon29 {
        &self.cru_clkgate_con29
    }
    #[doc = "0x378 - Internal clock gating register30"]
    #[inline(always)]
    pub const fn cru_clkgate_con30(&self) -> &CruClkgateCon30 {
        &self.cru_clkgate_con30
    }
    #[doc = "0x37c - Internal clock gating register31"]
    #[inline(always)]
    pub const fn cru_clkgate_con31(&self) -> &CruClkgateCon31 {
        &self.cru_clkgate_con31
    }
    #[doc = "0x380 - Internal clock gating register32"]
    #[inline(always)]
    pub const fn cru_clkgate_con32(&self) -> &CruClkgateCon32 {
        &self.cru_clkgate_con32
    }
    #[doc = "0x384 - Internal clock gating register33"]
    #[inline(always)]
    pub const fn cru_clkgate_con33(&self) -> &CruClkgateCon33 {
        &self.cru_clkgate_con33
    }
    #[doc = "0x388 - Internal clock gating register34"]
    #[inline(always)]
    pub const fn cru_clkgate_con34(&self) -> &CruClkgateCon34 {
        &self.cru_clkgate_con34
    }
    #[doc = "0x400 - Internal software reset control register0"]
    #[inline(always)]
    pub const fn cru_softrst_con0(&self) -> &CruSoftrstCon0 {
        &self.cru_softrst_con0
    }
    #[doc = "0x404 - Internal software reset control register1"]
    #[inline(always)]
    pub const fn cru_softrst_con1(&self) -> &CruSoftrstCon1 {
        &self.cru_softrst_con1
    }
    #[doc = "0x408 - Internal software reset control register2"]
    #[inline(always)]
    pub const fn cru_softrst_con2(&self) -> &CruSoftrstCon2 {
        &self.cru_softrst_con2
    }
    #[doc = "0x40c - Internal software reset control register3"]
    #[inline(always)]
    pub const fn cru_softrst_con3(&self) -> &CruSoftrstCon3 {
        &self.cru_softrst_con3
    }
    #[doc = "0x410 - Internal software reset control register4"]
    #[inline(always)]
    pub const fn cru_softrst_con4(&self) -> &CruSoftrstCon4 {
        &self.cru_softrst_con4
    }
    #[doc = "0x414 - Internal software reset control register5"]
    #[inline(always)]
    pub const fn cru_softrst_con5(&self) -> &CruSoftrstCon5 {
        &self.cru_softrst_con5
    }
    #[doc = "0x418 - Internal software reset control register6"]
    #[inline(always)]
    pub const fn cru_softrst_con6(&self) -> &CruSoftrstCon6 {
        &self.cru_softrst_con6
    }
    #[doc = "0x41c - Internal software reset control register7"]
    #[inline(always)]
    pub const fn cru_softrst_con7(&self) -> &CruSoftrstCon7 {
        &self.cru_softrst_con7
    }
    #[doc = "0x420 - Internal software reset control register8"]
    #[inline(always)]
    pub const fn cru_softrst_con8(&self) -> &CruSoftrstCon8 {
        &self.cru_softrst_con8
    }
    #[doc = "0x424 - Internal software reset control register9"]
    #[inline(always)]
    pub const fn cru_softrst_con9(&self) -> &CruSoftrstCon9 {
        &self.cru_softrst_con9
    }
    #[doc = "0x428 - Internal software reset control register10"]
    #[inline(always)]
    pub const fn cru_softrst_con10(&self) -> &CruSoftrstCon10 {
        &self.cru_softrst_con10
    }
    #[doc = "0x42c - Internal software reset control register11"]
    #[inline(always)]
    pub const fn cru_softrst_con11(&self) -> &CruSoftrstCon11 {
        &self.cru_softrst_con11
    }
    #[doc = "0x430 - Internal software reset control register12"]
    #[inline(always)]
    pub const fn cru_softrst_con12(&self) -> &CruSoftrstCon12 {
        &self.cru_softrst_con12
    }
    #[doc = "0x434 - Internal software reset control register13"]
    #[inline(always)]
    pub const fn cru_softrst_con13(&self) -> &CruSoftrstCon13 {
        &self.cru_softrst_con13
    }
    #[doc = "0x438 - Internal software reset control register14"]
    #[inline(always)]
    pub const fn cru_softrst_con14(&self) -> &CruSoftrstCon14 {
        &self.cru_softrst_con14
    }
    #[doc = "0x43c - Internal software reset control register15"]
    #[inline(always)]
    pub const fn cru_softrst_con15(&self) -> &CruSoftrstCon15 {
        &self.cru_softrst_con15
    }
    #[doc = "0x440 - Internal software reset control register16"]
    #[inline(always)]
    pub const fn cru_softrst_con16(&self) -> &CruSoftrstCon16 {
        &self.cru_softrst_con16
    }
    #[doc = "0x444 - Internal software reset control register17"]
    #[inline(always)]
    pub const fn cru_softrst_con17(&self) -> &CruSoftrstCon17 {
        &self.cru_softrst_con17
    }
    #[doc = "0x448 - Internal software reset control register18"]
    #[inline(always)]
    pub const fn cru_softrst_con18(&self) -> &CruSoftrstCon18 {
        &self.cru_softrst_con18
    }
    #[doc = "0x44c - Internal software reset control register19"]
    #[inline(always)]
    pub const fn cru_softrst_con19(&self) -> &CruSoftrstCon19 {
        &self.cru_softrst_con19
    }
    #[doc = "0x450 - Internal software reset control register20"]
    #[inline(always)]
    pub const fn cru_softrst_con20(&self) -> &CruSoftrstCon20 {
        &self.cru_softrst_con20
    }
    #[doc = "0x500 - The first global software reset config value"]
    #[inline(always)]
    pub const fn cru_glb_srst_fst_value(&self) -> &CruGlbSrstFstValue {
        &self.cru_glb_srst_fst_value
    }
    #[doc = "0x504 - The second global software reset config value"]
    #[inline(always)]
    pub const fn cru_glb_srst_snd_value(&self) -> &CruGlbSrstSndValue {
        &self.cru_glb_srst_snd_value
    }
    #[doc = "0x508 - Global soft reset counter threshold"]
    #[inline(always)]
    pub const fn cru_glb_cnt_th(&self) -> &CruGlbCntTh {
        &self.cru_glb_cnt_th
    }
    #[doc = "0x50c - Output clock selection for test"]
    #[inline(always)]
    pub const fn cru_misc_con(&self) -> &CruMiscCon {
        &self.cru_misc_con
    }
    #[doc = "0x510 - Global reset trigger select"]
    #[inline(always)]
    pub const fn cru_glb_rst_con(&self) -> &CruGlbRstCon {
        &self.cru_glb_rst_con
    }
    #[doc = "0x514 - Global reset status"]
    #[inline(always)]
    pub const fn cru_glb_rst_st(&self) -> &CruGlbRstSt {
        &self.cru_glb_rst_st
    }
    #[doc = "0x580 - sdmmc control0"]
    #[inline(always)]
    pub const fn cru_sdmmc_con0(&self) -> &CruSdmmcCon0 {
        &self.cru_sdmmc_con0
    }
    #[doc = "0x584 - sdmmc control1"]
    #[inline(always)]
    pub const fn cru_sdmmc_con1(&self) -> &CruSdmmcCon1 {
        &self.cru_sdmmc_con1
    }
    #[doc = "0x588 - sdio0 control0"]
    #[inline(always)]
    pub const fn cru_sdio0_con0(&self) -> &CruSdio0Con0 {
        &self.cru_sdio0_con0
    }
    #[doc = "0x58c - sdio0 control1"]
    #[inline(always)]
    pub const fn cru_sdio0_con1(&self) -> &CruSdio0Con1 {
        &self.cru_sdio0_con1
    }
}
#[doc = "CRU_LPLL_CON0 (rw) register accessor: LPLL configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_lpll_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_lpll_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_lpll_con0`]
module"]
#[doc(alias = "CRU_LPLL_CON0")]
pub type CruLpllCon0 = crate::Reg<cru_lpll_con0::CruLpllCon0Spec>;
#[doc = "LPLL configuration register0"]
pub mod cru_lpll_con0;
#[doc = "CRU_LPLL_CON1 (rw) register accessor: LPLL configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_lpll_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_lpll_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_lpll_con1`]
module"]
#[doc(alias = "CRU_LPLL_CON1")]
pub type CruLpllCon1 = crate::Reg<cru_lpll_con1::CruLpllCon1Spec>;
#[doc = "LPLL configuration register1"]
pub mod cru_lpll_con1;
#[doc = "CRU_LPLL_CON2 (rw) register accessor: LPLL configuration register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_lpll_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_lpll_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_lpll_con2`]
module"]
#[doc(alias = "CRU_LPLL_CON2")]
pub type CruLpllCon2 = crate::Reg<cru_lpll_con2::CruLpllCon2Spec>;
#[doc = "LPLL configuration register2"]
pub mod cru_lpll_con2;
#[doc = "CRU_LPLL_CON3 (rw) register accessor: LPLL configuration register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_lpll_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_lpll_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_lpll_con3`]
module"]
#[doc(alias = "CRU_LPLL_CON3")]
pub type CruLpllCon3 = crate::Reg<cru_lpll_con3::CruLpllCon3Spec>;
#[doc = "LPLL configuration register3"]
pub mod cru_lpll_con3;
#[doc = "CRU_LPLL_CON4 (rw) register accessor: LPLL configuration register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_lpll_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_lpll_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_lpll_con4`]
module"]
#[doc(alias = "CRU_LPLL_CON4")]
pub type CruLpllCon4 = crate::Reg<cru_lpll_con4::CruLpllCon4Spec>;
#[doc = "LPLL configuration register4"]
pub mod cru_lpll_con4;
#[doc = "CRU_LPLL_CON5 (rw) register accessor: LPLL configuration register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_lpll_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_lpll_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_lpll_con5`]
module"]
#[doc(alias = "CRU_LPLL_CON5")]
pub type CruLpllCon5 = crate::Reg<cru_lpll_con5::CruLpllCon5Spec>;
#[doc = "LPLL configuration register5"]
pub mod cru_lpll_con5;
#[doc = "CRU_BPLL_CON0 (rw) register accessor: BPLL configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_bpll_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_bpll_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_bpll_con0`]
module"]
#[doc(alias = "CRU_BPLL_CON0")]
pub type CruBpllCon0 = crate::Reg<cru_bpll_con0::CruBpllCon0Spec>;
#[doc = "BPLL configuration register0"]
pub mod cru_bpll_con0;
#[doc = "CRU_BPLL_CON1 (rw) register accessor: BPLL configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_bpll_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_bpll_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_bpll_con1`]
module"]
#[doc(alias = "CRU_BPLL_CON1")]
pub type CruBpllCon1 = crate::Reg<cru_bpll_con1::CruBpllCon1Spec>;
#[doc = "BPLL configuration register1"]
pub mod cru_bpll_con1;
#[doc = "CRU_BPLL_CON2 (rw) register accessor: BPLL configuration register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_bpll_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_bpll_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_bpll_con2`]
module"]
#[doc(alias = "CRU_BPLL_CON2")]
pub type CruBpllCon2 = crate::Reg<cru_bpll_con2::CruBpllCon2Spec>;
#[doc = "BPLL configuration register2"]
pub mod cru_bpll_con2;
#[doc = "CRU_BPLL_CON3 (rw) register accessor: BPLL configuration register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_bpll_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_bpll_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_bpll_con3`]
module"]
#[doc(alias = "CRU_BPLL_CON3")]
pub type CruBpllCon3 = crate::Reg<cru_bpll_con3::CruBpllCon3Spec>;
#[doc = "BPLL configuration register3"]
pub mod cru_bpll_con3;
#[doc = "CRU_BPLL_CON4 (rw) register accessor: BPLL configuration register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_bpll_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_bpll_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_bpll_con4`]
module"]
#[doc(alias = "CRU_BPLL_CON4")]
pub type CruBpllCon4 = crate::Reg<cru_bpll_con4::CruBpllCon4Spec>;
#[doc = "BPLL configuration register4"]
pub mod cru_bpll_con4;
#[doc = "CRU_BPLL_CON5 (rw) register accessor: BPLL configuration register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_bpll_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_bpll_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_bpll_con5`]
module"]
#[doc(alias = "CRU_BPLL_CON5")]
pub type CruBpllCon5 = crate::Reg<cru_bpll_con5::CruBpllCon5Spec>;
#[doc = "BPLL configuration register5"]
pub mod cru_bpll_con5;
#[doc = "CRU_DPLL_CON0 (rw) register accessor: DPLL configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_dpll_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_dpll_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_dpll_con0`]
module"]
#[doc(alias = "CRU_DPLL_CON0")]
pub type CruDpllCon0 = crate::Reg<cru_dpll_con0::CruDpllCon0Spec>;
#[doc = "DPLL configuration register0"]
pub mod cru_dpll_con0;
#[doc = "CRU_DPLL_CON1 (rw) register accessor: DPLL configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_dpll_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_dpll_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_dpll_con1`]
module"]
#[doc(alias = "CRU_DPLL_CON1")]
pub type CruDpllCon1 = crate::Reg<cru_dpll_con1::CruDpllCon1Spec>;
#[doc = "DPLL configuration register1"]
pub mod cru_dpll_con1;
#[doc = "CRU_DPLL_CON2 (rw) register accessor: DPLL configuration register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_dpll_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_dpll_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_dpll_con2`]
module"]
#[doc(alias = "CRU_DPLL_CON2")]
pub type CruDpllCon2 = crate::Reg<cru_dpll_con2::CruDpllCon2Spec>;
#[doc = "DPLL configuration register2"]
pub mod cru_dpll_con2;
#[doc = "CRU_DPLL_CON3 (rw) register accessor: DPLL configuration register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_dpll_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_dpll_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_dpll_con3`]
module"]
#[doc(alias = "CRU_DPLL_CON3")]
pub type CruDpllCon3 = crate::Reg<cru_dpll_con3::CruDpllCon3Spec>;
#[doc = "DPLL configuration register3"]
pub mod cru_dpll_con3;
#[doc = "CRU_DPLL_CON4 (rw) register accessor: DPLL configuration register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_dpll_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_dpll_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_dpll_con4`]
module"]
#[doc(alias = "CRU_DPLL_CON4")]
pub type CruDpllCon4 = crate::Reg<cru_dpll_con4::CruDpllCon4Spec>;
#[doc = "DPLL configuration register4"]
pub mod cru_dpll_con4;
#[doc = "CRU_DPLL_CON5 (rw) register accessor: DPLL configuration register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_dpll_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_dpll_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_dpll_con5`]
module"]
#[doc(alias = "CRU_DPLL_CON5")]
pub type CruDpllCon5 = crate::Reg<cru_dpll_con5::CruDpllCon5Spec>;
#[doc = "DPLL configuration register5"]
pub mod cru_dpll_con5;
#[doc = "CRU_CPLL_CON0 (rw) register accessor: CPLL configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_cpll_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_cpll_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_cpll_con0`]
module"]
#[doc(alias = "CRU_CPLL_CON0")]
pub type CruCpllCon0 = crate::Reg<cru_cpll_con0::CruCpllCon0Spec>;
#[doc = "CPLL configuration register0"]
pub mod cru_cpll_con0;
#[doc = "CRU_CPLL_CON1 (rw) register accessor: CPLL configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_cpll_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_cpll_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_cpll_con1`]
module"]
#[doc(alias = "CRU_CPLL_CON1")]
pub type CruCpllCon1 = crate::Reg<cru_cpll_con1::CruCpllCon1Spec>;
#[doc = "CPLL configuration register1"]
pub mod cru_cpll_con1;
#[doc = "CRU_CPLL_CON2 (rw) register accessor: CPLL configuration register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_cpll_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_cpll_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_cpll_con2`]
module"]
#[doc(alias = "CRU_CPLL_CON2")]
pub type CruCpllCon2 = crate::Reg<cru_cpll_con2::CruCpllCon2Spec>;
#[doc = "CPLL configuration register2"]
pub mod cru_cpll_con2;
#[doc = "CRU_CPLL_CON3 (rw) register accessor: CPLL configuration register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_cpll_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_cpll_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_cpll_con3`]
module"]
#[doc(alias = "CRU_CPLL_CON3")]
pub type CruCpllCon3 = crate::Reg<cru_cpll_con3::CruCpllCon3Spec>;
#[doc = "CPLL configuration register3"]
pub mod cru_cpll_con3;
#[doc = "CRU_CPLL_CON4 (rw) register accessor: CPLL configuration register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_cpll_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_cpll_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_cpll_con4`]
module"]
#[doc(alias = "CRU_CPLL_CON4")]
pub type CruCpllCon4 = crate::Reg<cru_cpll_con4::CruCpllCon4Spec>;
#[doc = "CPLL configuration register4"]
pub mod cru_cpll_con4;
#[doc = "CRU_CPLL_CON5 (rw) register accessor: CPLL configuration register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_cpll_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_cpll_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_cpll_con5`]
module"]
#[doc(alias = "CRU_CPLL_CON5")]
pub type CruCpllCon5 = crate::Reg<cru_cpll_con5::CruCpllCon5Spec>;
#[doc = "CPLL configuration register5"]
pub mod cru_cpll_con5;
#[doc = "CRU_GPLL_CON0 (rw) register accessor: GPLL configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_gpll_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_gpll_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_gpll_con0`]
module"]
#[doc(alias = "CRU_GPLL_CON0")]
pub type CruGpllCon0 = crate::Reg<cru_gpll_con0::CruGpllCon0Spec>;
#[doc = "GPLL configuration register0"]
pub mod cru_gpll_con0;
#[doc = "CRU_GPLL_CON1 (rw) register accessor: GPLL configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_gpll_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_gpll_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_gpll_con1`]
module"]
#[doc(alias = "CRU_GPLL_CON1")]
pub type CruGpllCon1 = crate::Reg<cru_gpll_con1::CruGpllCon1Spec>;
#[doc = "GPLL configuration register1"]
pub mod cru_gpll_con1;
#[doc = "CRU_GPLL_CON2 (rw) register accessor: GPLL configuration register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_gpll_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_gpll_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_gpll_con2`]
module"]
#[doc(alias = "CRU_GPLL_CON2")]
pub type CruGpllCon2 = crate::Reg<cru_gpll_con2::CruGpllCon2Spec>;
#[doc = "GPLL configuration register2"]
pub mod cru_gpll_con2;
#[doc = "CRU_GPLL_CON3 (rw) register accessor: GPLL configuration register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_gpll_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_gpll_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_gpll_con3`]
module"]
#[doc(alias = "CRU_GPLL_CON3")]
pub type CruGpllCon3 = crate::Reg<cru_gpll_con3::CruGpllCon3Spec>;
#[doc = "GPLL configuration register3"]
pub mod cru_gpll_con3;
#[doc = "CRU_GPLL_CON4 (rw) register accessor: GPLL configuration register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_gpll_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_gpll_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_gpll_con4`]
module"]
#[doc(alias = "CRU_GPLL_CON4")]
pub type CruGpllCon4 = crate::Reg<cru_gpll_con4::CruGpllCon4Spec>;
#[doc = "GPLL configuration register4"]
pub mod cru_gpll_con4;
#[doc = "CRU_GPLL_CON5 (rw) register accessor: GPLL configuration register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_gpll_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_gpll_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_gpll_con5`]
module"]
#[doc(alias = "CRU_GPLL_CON5")]
pub type CruGpllCon5 = crate::Reg<cru_gpll_con5::CruGpllCon5Spec>;
#[doc = "GPLL configuration register5"]
pub mod cru_gpll_con5;
#[doc = "CRU_NPLL_CON0 (rw) register accessor: NPLL configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_npll_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_npll_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_npll_con0`]
module"]
#[doc(alias = "CRU_NPLL_CON0")]
pub type CruNpllCon0 = crate::Reg<cru_npll_con0::CruNpllCon0Spec>;
#[doc = "NPLL configuration register0"]
pub mod cru_npll_con0;
#[doc = "CRU_NPLL_CON1 (rw) register accessor: NPLL configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_npll_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_npll_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_npll_con1`]
module"]
#[doc(alias = "CRU_NPLL_CON1")]
pub type CruNpllCon1 = crate::Reg<cru_npll_con1::CruNpllCon1Spec>;
#[doc = "NPLL configuration register1"]
pub mod cru_npll_con1;
#[doc = "CRU_NPLL_CON2 (rw) register accessor: NPLL configuration register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_npll_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_npll_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_npll_con2`]
module"]
#[doc(alias = "CRU_NPLL_CON2")]
pub type CruNpllCon2 = crate::Reg<cru_npll_con2::CruNpllCon2Spec>;
#[doc = "NPLL configuration register2"]
pub mod cru_npll_con2;
#[doc = "CRU_NPLL_CON3 (rw) register accessor: NPLL configuration register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_npll_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_npll_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_npll_con3`]
module"]
#[doc(alias = "CRU_NPLL_CON3")]
pub type CruNpllCon3 = crate::Reg<cru_npll_con3::CruNpllCon3Spec>;
#[doc = "NPLL configuration register3"]
pub mod cru_npll_con3;
#[doc = "CRU_NPLL_CON4 (rw) register accessor: NPLL configuration register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_npll_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_npll_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_npll_con4`]
module"]
#[doc(alias = "CRU_NPLL_CON4")]
pub type CruNpllCon4 = crate::Reg<cru_npll_con4::CruNpllCon4Spec>;
#[doc = "NPLL configuration register4"]
pub mod cru_npll_con4;
#[doc = "CRU_NPLL_CON5 (rw) register accessor: NPLL configuration register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_npll_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_npll_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_npll_con5`]
module"]
#[doc(alias = "CRU_NPLL_CON5")]
pub type CruNpllCon5 = crate::Reg<cru_npll_con5::CruNpllCon5Spec>;
#[doc = "NPLL configuration register5"]
pub mod cru_npll_con5;
#[doc = "CRU_VPLL_CON0 (rw) register accessor: VPLL configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_vpll_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_vpll_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_vpll_con0`]
module"]
#[doc(alias = "CRU_VPLL_CON0")]
pub type CruVpllCon0 = crate::Reg<cru_vpll_con0::CruVpllCon0Spec>;
#[doc = "VPLL configuration register0"]
pub mod cru_vpll_con0;
#[doc = "CRU_VPLL_CON1 (rw) register accessor: VPLL configuration register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_vpll_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_vpll_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_vpll_con1`]
module"]
#[doc(alias = "CRU_VPLL_CON1")]
pub type CruVpllCon1 = crate::Reg<cru_vpll_con1::CruVpllCon1Spec>;
#[doc = "VPLL configuration register1"]
pub mod cru_vpll_con1;
#[doc = "CRU_VPLL_CON2 (rw) register accessor: VPLL configuration register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_vpll_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_vpll_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_vpll_con2`]
module"]
#[doc(alias = "CRU_VPLL_CON2")]
pub type CruVpllCon2 = crate::Reg<cru_vpll_con2::CruVpllCon2Spec>;
#[doc = "VPLL configuration register2"]
pub mod cru_vpll_con2;
#[doc = "CRU_VPLL_CON3 (rw) register accessor: VPLL configuration register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_vpll_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_vpll_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_vpll_con3`]
module"]
#[doc(alias = "CRU_VPLL_CON3")]
pub type CruVpllCon3 = crate::Reg<cru_vpll_con3::CruVpllCon3Spec>;
#[doc = "VPLL configuration register3"]
pub mod cru_vpll_con3;
#[doc = "CRU_VPLL_CON4 (rw) register accessor: VPLL configuration register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_vpll_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_vpll_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_vpll_con4`]
module"]
#[doc(alias = "CRU_VPLL_CON4")]
pub type CruVpllCon4 = crate::Reg<cru_vpll_con4::CruVpllCon4Spec>;
#[doc = "VPLL configuration register4"]
pub mod cru_vpll_con4;
#[doc = "CRU_VPLL_CON5 (rw) register accessor: VPLL configuration register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_vpll_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_vpll_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_vpll_con5`]
module"]
#[doc(alias = "CRU_VPLL_CON5")]
pub type CruVpllCon5 = crate::Reg<cru_vpll_con5::CruVpllCon5Spec>;
#[doc = "VPLL configuration register5"]
pub mod cru_vpll_con5;
#[doc = "CRU_CLKSEL_CON0 (rw) register accessor: Internal clock select and divide register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con0`]
module"]
#[doc(alias = "CRU_CLKSEL_CON0")]
pub type CruClkselCon0 = crate::Reg<cru_clksel_con0::CruClkselCon0Spec>;
#[doc = "Internal clock select and divide register0"]
pub mod cru_clksel_con0;
#[doc = "CRU_CLKSEL_CON1 (rw) register accessor: Internal clock select and divide register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con1`]
module"]
#[doc(alias = "CRU_CLKSEL_CON1")]
pub type CruClkselCon1 = crate::Reg<cru_clksel_con1::CruClkselCon1Spec>;
#[doc = "Internal clock select and divide register1"]
pub mod cru_clksel_con1;
#[doc = "CRU_CLKSEL_CON2 (rw) register accessor: Internal clock select and divide register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con2`]
module"]
#[doc(alias = "CRU_CLKSEL_CON2")]
pub type CruClkselCon2 = crate::Reg<cru_clksel_con2::CruClkselCon2Spec>;
#[doc = "Internal clock select and divide register2"]
pub mod cru_clksel_con2;
#[doc = "CRU_CLKSEL_CON3 (rw) register accessor: Internal clock select and divide register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con3`]
module"]
#[doc(alias = "CRU_CLKSEL_CON3")]
pub type CruClkselCon3 = crate::Reg<cru_clksel_con3::CruClkselCon3Spec>;
#[doc = "Internal clock select and divide register3"]
pub mod cru_clksel_con3;
#[doc = "CRU_CLKSEL_CON4 (rw) register accessor: Internal clock select and divide register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con4`]
module"]
#[doc(alias = "CRU_CLKSEL_CON4")]
pub type CruClkselCon4 = crate::Reg<cru_clksel_con4::CruClkselCon4Spec>;
#[doc = "Internal clock select and divide register4"]
pub mod cru_clksel_con4;
#[doc = "CRU_CLKSEL_CON5 (rw) register accessor: Internal clock select and divide register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con5`]
module"]
#[doc(alias = "CRU_CLKSEL_CON5")]
pub type CruClkselCon5 = crate::Reg<cru_clksel_con5::CruClkselCon5Spec>;
#[doc = "Internal clock select and divide register5"]
pub mod cru_clksel_con5;
#[doc = "CRU_CLKSEL_CON6 (rw) register accessor: Internal clock select and divide register6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con6`]
module"]
#[doc(alias = "CRU_CLKSEL_CON6")]
pub type CruClkselCon6 = crate::Reg<cru_clksel_con6::CruClkselCon6Spec>;
#[doc = "Internal clock select and divide register6"]
pub mod cru_clksel_con6;
#[doc = "CRU_CLKSEL_CON7 (rw) register accessor: Internal clock select and divide register7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con7`]
module"]
#[doc(alias = "CRU_CLKSEL_CON7")]
pub type CruClkselCon7 = crate::Reg<cru_clksel_con7::CruClkselCon7Spec>;
#[doc = "Internal clock select and divide register7"]
pub mod cru_clksel_con7;
#[doc = "CRU_CLKSEL_CON8 (rw) register accessor: Internal clock select and divide register8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con8`]
module"]
#[doc(alias = "CRU_CLKSEL_CON8")]
pub type CruClkselCon8 = crate::Reg<cru_clksel_con8::CruClkselCon8Spec>;
#[doc = "Internal clock select and divide register8"]
pub mod cru_clksel_con8;
#[doc = "CRU_CLKSEL_CON9 (rw) register accessor: Internal clock select and divide register9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con9`]
module"]
#[doc(alias = "CRU_CLKSEL_CON9")]
pub type CruClkselCon9 = crate::Reg<cru_clksel_con9::CruClkselCon9Spec>;
#[doc = "Internal clock select and divide register9"]
pub mod cru_clksel_con9;
#[doc = "CRU_CLKSEL_CON10 (rw) register accessor: Internal clock select and divide register10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con10`]
module"]
#[doc(alias = "CRU_CLKSEL_CON10")]
pub type CruClkselCon10 = crate::Reg<cru_clksel_con10::CruClkselCon10Spec>;
#[doc = "Internal clock select and divide register10"]
pub mod cru_clksel_con10;
#[doc = "CRU_CLKSEL_CON11 (rw) register accessor: Internal clock select and divide register11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con11`]
module"]
#[doc(alias = "CRU_CLKSEL_CON11")]
pub type CruClkselCon11 = crate::Reg<cru_clksel_con11::CruClkselCon11Spec>;
#[doc = "Internal clock select and divide register11"]
pub mod cru_clksel_con11;
#[doc = "CRU_CLKSEL_CON12 (rw) register accessor: Internal clock select and divide register12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con12`]
module"]
#[doc(alias = "CRU_CLKSEL_CON12")]
pub type CruClkselCon12 = crate::Reg<cru_clksel_con12::CruClkselCon12Spec>;
#[doc = "Internal clock select and divide register12"]
pub mod cru_clksel_con12;
#[doc = "CRU_CLKSEL_CON13 (rw) register accessor: Internal clock select and divide register13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con13`]
module"]
#[doc(alias = "CRU_CLKSEL_CON13")]
pub type CruClkselCon13 = crate::Reg<cru_clksel_con13::CruClkselCon13Spec>;
#[doc = "Internal clock select and divide register13"]
pub mod cru_clksel_con13;
#[doc = "CRU_CLKSEL_CON14 (rw) register accessor: Internal clock select and divide register14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con14`]
module"]
#[doc(alias = "CRU_CLKSEL_CON14")]
pub type CruClkselCon14 = crate::Reg<cru_clksel_con14::CruClkselCon14Spec>;
#[doc = "Internal clock select and divide register14"]
pub mod cru_clksel_con14;
#[doc = "CRU_CLKSEL_CON15 (rw) register accessor: Internal clock select and divide register15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con15`]
module"]
#[doc(alias = "CRU_CLKSEL_CON15")]
pub type CruClkselCon15 = crate::Reg<cru_clksel_con15::CruClkselCon15Spec>;
#[doc = "Internal clock select and divide register15"]
pub mod cru_clksel_con15;
#[doc = "CRU_CLKSEL_CON16 (rw) register accessor: Internal clock select and divide register16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con16`]
module"]
#[doc(alias = "CRU_CLKSEL_CON16")]
pub type CruClkselCon16 = crate::Reg<cru_clksel_con16::CruClkselCon16Spec>;
#[doc = "Internal clock select and divide register16"]
pub mod cru_clksel_con16;
#[doc = "CRU_CLKSEL_CON17 (rw) register accessor: Internal clock select and divide register17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con17`]
module"]
#[doc(alias = "CRU_CLKSEL_CON17")]
pub type CruClkselCon17 = crate::Reg<cru_clksel_con17::CruClkselCon17Spec>;
#[doc = "Internal clock select and divide register17"]
pub mod cru_clksel_con17;
#[doc = "CRU_CLKSEL_CON18 (rw) register accessor: Internal clock select and divide register18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con18`]
module"]
#[doc(alias = "CRU_CLKSEL_CON18")]
pub type CruClkselCon18 = crate::Reg<cru_clksel_con18::CruClkselCon18Spec>;
#[doc = "Internal clock select and divide register18"]
pub mod cru_clksel_con18;
#[doc = "CRU_CLKSEL_CON19 (rw) register accessor: Internal clock select and divide register19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con19`]
module"]
#[doc(alias = "CRU_CLKSEL_CON19")]
pub type CruClkselCon19 = crate::Reg<cru_clksel_con19::CruClkselCon19Spec>;
#[doc = "Internal clock select and divide register19"]
pub mod cru_clksel_con19;
#[doc = "CRU_CLKSEL_CON20 (rw) register accessor: Internal clock select and divide register20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con20`]
module"]
#[doc(alias = "CRU_CLKSEL_CON20")]
pub type CruClkselCon20 = crate::Reg<cru_clksel_con20::CruClkselCon20Spec>;
#[doc = "Internal clock select and divide register20"]
pub mod cru_clksel_con20;
#[doc = "CRU_CLKSEL_CON21 (rw) register accessor: Internal clock select and divide register21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con21`]
module"]
#[doc(alias = "CRU_CLKSEL_CON21")]
pub type CruClkselCon21 = crate::Reg<cru_clksel_con21::CruClkselCon21Spec>;
#[doc = "Internal clock select and divide register21"]
pub mod cru_clksel_con21;
#[doc = "CRU_CLKSEL_CON22 (rw) register accessor: Internal clock select and divide register22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con22`]
module"]
#[doc(alias = "CRU_CLKSEL_CON22")]
pub type CruClkselCon22 = crate::Reg<cru_clksel_con22::CruClkselCon22Spec>;
#[doc = "Internal clock select and divide register22"]
pub mod cru_clksel_con22;
#[doc = "CRU_CLKSEL_CON23 (rw) register accessor: Internal clock select and divide register23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con23`]
module"]
#[doc(alias = "CRU_CLKSEL_CON23")]
pub type CruClkselCon23 = crate::Reg<cru_clksel_con23::CruClkselCon23Spec>;
#[doc = "Internal clock select and divide register23"]
pub mod cru_clksel_con23;
#[doc = "CRU_CLKSEL_CON24 (rw) register accessor: Internal clock select and divide register24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con24`]
module"]
#[doc(alias = "CRU_CLKSEL_CON24")]
pub type CruClkselCon24 = crate::Reg<cru_clksel_con24::CruClkselCon24Spec>;
#[doc = "Internal clock select and divide register24"]
pub mod cru_clksel_con24;
#[doc = "CRU_CLKSEL_CON25 (rw) register accessor: Internal clock select and divide register25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con25`]
module"]
#[doc(alias = "CRU_CLKSEL_CON25")]
pub type CruClkselCon25 = crate::Reg<cru_clksel_con25::CruClkselCon25Spec>;
#[doc = "Internal clock select and divide register25"]
pub mod cru_clksel_con25;
#[doc = "CRU_CLKSEL_CON26 (rw) register accessor: Internal clock select and divide register26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con26`]
module"]
#[doc(alias = "CRU_CLKSEL_CON26")]
pub type CruClkselCon26 = crate::Reg<cru_clksel_con26::CruClkselCon26Spec>;
#[doc = "Internal clock select and divide register26"]
pub mod cru_clksel_con26;
#[doc = "CRU_CLKSEL_CON27 (rw) register accessor: Internal clock select and divide register27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con27`]
module"]
#[doc(alias = "CRU_CLKSEL_CON27")]
pub type CruClkselCon27 = crate::Reg<cru_clksel_con27::CruClkselCon27Spec>;
#[doc = "Internal clock select and divide register27"]
pub mod cru_clksel_con27;
#[doc = "CRU_CLKSEL_CON28 (rw) register accessor: Internal clock select and divide register28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con28`]
module"]
#[doc(alias = "CRU_CLKSEL_CON28")]
pub type CruClkselCon28 = crate::Reg<cru_clksel_con28::CruClkselCon28Spec>;
#[doc = "Internal clock select and divide register28"]
pub mod cru_clksel_con28;
#[doc = "CRU_CLKSEL_CON29 (rw) register accessor: Internal clock select and divide register29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con29`]
module"]
#[doc(alias = "CRU_CLKSEL_CON29")]
pub type CruClkselCon29 = crate::Reg<cru_clksel_con29::CruClkselCon29Spec>;
#[doc = "Internal clock select and divide register29"]
pub mod cru_clksel_con29;
#[doc = "CRU_CLKSEL_CON30 (rw) register accessor: Internal clock select and divide register30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con30`]
module"]
#[doc(alias = "CRU_CLKSEL_CON30")]
pub type CruClkselCon30 = crate::Reg<cru_clksel_con30::CruClkselCon30Spec>;
#[doc = "Internal clock select and divide register30"]
pub mod cru_clksel_con30;
#[doc = "CRU_CLKSEL_CON31 (rw) register accessor: Internal clock select and divide register31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con31`]
module"]
#[doc(alias = "CRU_CLKSEL_CON31")]
pub type CruClkselCon31 = crate::Reg<cru_clksel_con31::CruClkselCon31Spec>;
#[doc = "Internal clock select and divide register31"]
pub mod cru_clksel_con31;
#[doc = "CRU_CLKSEL_CON32 (rw) register accessor: Internal clock select and divide register32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con32`]
module"]
#[doc(alias = "CRU_CLKSEL_CON32")]
pub type CruClkselCon32 = crate::Reg<cru_clksel_con32::CruClkselCon32Spec>;
#[doc = "Internal clock select and divide register32"]
pub mod cru_clksel_con32;
#[doc = "CRU_CLKSEL_CON33 (rw) register accessor: Internal clock select and divide register33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con33`]
module"]
#[doc(alias = "CRU_CLKSEL_CON33")]
pub type CruClkselCon33 = crate::Reg<cru_clksel_con33::CruClkselCon33Spec>;
#[doc = "Internal clock select and divide register33"]
pub mod cru_clksel_con33;
#[doc = "CRU_CLKSEL_CON34 (rw) register accessor: Internal clock select and divide register34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con34`]
module"]
#[doc(alias = "CRU_CLKSEL_CON34")]
pub type CruClkselCon34 = crate::Reg<cru_clksel_con34::CruClkselCon34Spec>;
#[doc = "Internal clock select and divide register34"]
pub mod cru_clksel_con34;
#[doc = "CRU_CLKSEL_CON35 (rw) register accessor: Internal clock select and divide register35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con35`]
module"]
#[doc(alias = "CRU_CLKSEL_CON35")]
pub type CruClkselCon35 = crate::Reg<cru_clksel_con35::CruClkselCon35Spec>;
#[doc = "Internal clock select and divide register35"]
pub mod cru_clksel_con35;
#[doc = "CRU_CLKSEL_CON36 (rw) register accessor: Internal clock select and divide register36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con36`]
module"]
#[doc(alias = "CRU_CLKSEL_CON36")]
pub type CruClkselCon36 = crate::Reg<cru_clksel_con36::CruClkselCon36Spec>;
#[doc = "Internal clock select and divide register36"]
pub mod cru_clksel_con36;
#[doc = "CRU_CLKSEL_CON38 (rw) register accessor: Internal clock select and divide register38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con38`]
module"]
#[doc(alias = "CRU_CLKSEL_CON38")]
pub type CruClkselCon38 = crate::Reg<cru_clksel_con38::CruClkselCon38Spec>;
#[doc = "Internal clock select and divide register38"]
pub mod cru_clksel_con38;
#[doc = "CRU_CLKSEL_CON39 (rw) register accessor: Internal clock select and divide register39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con39`]
module"]
#[doc(alias = "CRU_CLKSEL_CON39")]
pub type CruClkselCon39 = crate::Reg<cru_clksel_con39::CruClkselCon39Spec>;
#[doc = "Internal clock select and divide register39"]
pub mod cru_clksel_con39;
#[doc = "CRU_CLKSEL_CON40 (rw) register accessor: Internal clock select and divide register40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con40`]
module"]
#[doc(alias = "CRU_CLKSEL_CON40")]
pub type CruClkselCon40 = crate::Reg<cru_clksel_con40::CruClkselCon40Spec>;
#[doc = "Internal clock select and divide register40"]
pub mod cru_clksel_con40;
#[doc = "CRU_CLKSEL_CON41 (rw) register accessor: Internal clock select and divide register41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con41`]
module"]
#[doc(alias = "CRU_CLKSEL_CON41")]
pub type CruClkselCon41 = crate::Reg<cru_clksel_con41::CruClkselCon41Spec>;
#[doc = "Internal clock select and divide register41"]
pub mod cru_clksel_con41;
#[doc = "CRU_CLKSEL_CON42 (rw) register accessor: Internal clock select and divide register42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con42`]
module"]
#[doc(alias = "CRU_CLKSEL_CON42")]
pub type CruClkselCon42 = crate::Reg<cru_clksel_con42::CruClkselCon42Spec>;
#[doc = "Internal clock select and divide register42"]
pub mod cru_clksel_con42;
#[doc = "CRU_CLKSEL_CON43 (rw) register accessor: Internal clock select and divide register43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con43`]
module"]
#[doc(alias = "CRU_CLKSEL_CON43")]
pub type CruClkselCon43 = crate::Reg<cru_clksel_con43::CruClkselCon43Spec>;
#[doc = "Internal clock select and divide register43"]
pub mod cru_clksel_con43;
#[doc = "CRU_CLKSEL_CON44 (rw) register accessor: Internal clock select and divide register44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con44`]
module"]
#[doc(alias = "CRU_CLKSEL_CON44")]
pub type CruClkselCon44 = crate::Reg<cru_clksel_con44::CruClkselCon44Spec>;
#[doc = "Internal clock select and divide register44"]
pub mod cru_clksel_con44;
#[doc = "CRU_CLKSEL_CON45 (rw) register accessor: Internal clock select and divide register45\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con45`]
module"]
#[doc(alias = "CRU_CLKSEL_CON45")]
pub type CruClkselCon45 = crate::Reg<cru_clksel_con45::CruClkselCon45Spec>;
#[doc = "Internal clock select and divide register45"]
pub mod cru_clksel_con45;
#[doc = "CRU_CLKSEL_CON46 (rw) register accessor: Internal clock select and divide register46\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con46`]
module"]
#[doc(alias = "CRU_CLKSEL_CON46")]
pub type CruClkselCon46 = crate::Reg<cru_clksel_con46::CruClkselCon46Spec>;
#[doc = "Internal clock select and divide register46"]
pub mod cru_clksel_con46;
#[doc = "CRU_CLKSEL_CON47 (rw) register accessor: Internal clock select and divide register47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con47`]
module"]
#[doc(alias = "CRU_CLKSEL_CON47")]
pub type CruClkselCon47 = crate::Reg<cru_clksel_con47::CruClkselCon47Spec>;
#[doc = "Internal clock select and divide register47"]
pub mod cru_clksel_con47;
#[doc = "CRU_CLKSEL_CON48 (rw) register accessor: Internal clock select and divide register48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con48`]
module"]
#[doc(alias = "CRU_CLKSEL_CON48")]
pub type CruClkselCon48 = crate::Reg<cru_clksel_con48::CruClkselCon48Spec>;
#[doc = "Internal clock select and divide register48"]
pub mod cru_clksel_con48;
#[doc = "CRU_CLKSEL_CON49 (rw) register accessor: Internal clock select and divide register49\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con49`]
module"]
#[doc(alias = "CRU_CLKSEL_CON49")]
pub type CruClkselCon49 = crate::Reg<cru_clksel_con49::CruClkselCon49Spec>;
#[doc = "Internal clock select and divide register49"]
pub mod cru_clksel_con49;
#[doc = "CRU_CLKSEL_CON50 (rw) register accessor: Internal clock select and divide register50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con50::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con50`]
module"]
#[doc(alias = "CRU_CLKSEL_CON50")]
pub type CruClkselCon50 = crate::Reg<cru_clksel_con50::CruClkselCon50Spec>;
#[doc = "Internal clock select and divide register50"]
pub mod cru_clksel_con50;
#[doc = "CRU_CLKSEL_CON51 (rw) register accessor: Internal clock select and divide register51\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con51`]
module"]
#[doc(alias = "CRU_CLKSEL_CON51")]
pub type CruClkselCon51 = crate::Reg<cru_clksel_con51::CruClkselCon51Spec>;
#[doc = "Internal clock select and divide register51"]
pub mod cru_clksel_con51;
#[doc = "CRU_CLKSEL_CON52 (rw) register accessor: Internal clock select and divide register52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con52`]
module"]
#[doc(alias = "CRU_CLKSEL_CON52")]
pub type CruClkselCon52 = crate::Reg<cru_clksel_con52::CruClkselCon52Spec>;
#[doc = "Internal clock select and divide register52"]
pub mod cru_clksel_con52;
#[doc = "CRU_CLKSEL_CON53 (rw) register accessor: Internal clock select and divide register53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con53`]
module"]
#[doc(alias = "CRU_CLKSEL_CON53")]
pub type CruClkselCon53 = crate::Reg<cru_clksel_con53::CruClkselCon53Spec>;
#[doc = "Internal clock select and divide register53"]
pub mod cru_clksel_con53;
#[doc = "CRU_CLKSEL_CON54 (rw) register accessor: Internal clock select and divide register54\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con54::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con54`]
module"]
#[doc(alias = "CRU_CLKSEL_CON54")]
pub type CruClkselCon54 = crate::Reg<cru_clksel_con54::CruClkselCon54Spec>;
#[doc = "Internal clock select and divide register54"]
pub mod cru_clksel_con54;
#[doc = "CRU_CLKSEL_CON55 (rw) register accessor: Internal clock select and divide register55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con55`]
module"]
#[doc(alias = "CRU_CLKSEL_CON55")]
pub type CruClkselCon55 = crate::Reg<cru_clksel_con55::CruClkselCon55Spec>;
#[doc = "Internal clock select and divide register55"]
pub mod cru_clksel_con55;
#[doc = "CRU_CLKSEL_CON56 (rw) register accessor: Internal clock select and divide register56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con56::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con56`]
module"]
#[doc(alias = "CRU_CLKSEL_CON56")]
pub type CruClkselCon56 = crate::Reg<cru_clksel_con56::CruClkselCon56Spec>;
#[doc = "Internal clock select and divide register56"]
pub mod cru_clksel_con56;
#[doc = "CRU_CLKSEL_CON57 (rw) register accessor: Internal clock select and divide register57\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con57::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con57::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con57`]
module"]
#[doc(alias = "CRU_CLKSEL_CON57")]
pub type CruClkselCon57 = crate::Reg<cru_clksel_con57::CruClkselCon57Spec>;
#[doc = "Internal clock select and divide register57"]
pub mod cru_clksel_con57;
#[doc = "CRU_CLKSEL_CON58 (rw) register accessor: Internal clock select and divide register58\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con58::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con58::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con58`]
module"]
#[doc(alias = "CRU_CLKSEL_CON58")]
pub type CruClkselCon58 = crate::Reg<cru_clksel_con58::CruClkselCon58Spec>;
#[doc = "Internal clock select and divide register58"]
pub mod cru_clksel_con58;
#[doc = "CRU_CLKSEL_CON59 (rw) register accessor: Internal clock select and divide register59\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con59::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con59`]
module"]
#[doc(alias = "CRU_CLKSEL_CON59")]
pub type CruClkselCon59 = crate::Reg<cru_clksel_con59::CruClkselCon59Spec>;
#[doc = "Internal clock select and divide register59"]
pub mod cru_clksel_con59;
#[doc = "CRU_CLKSEL_CON60 (rw) register accessor: Internal clock select and divide register60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con60::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con60`]
module"]
#[doc(alias = "CRU_CLKSEL_CON60")]
pub type CruClkselCon60 = crate::Reg<cru_clksel_con60::CruClkselCon60Spec>;
#[doc = "Internal clock select and divide register60"]
pub mod cru_clksel_con60;
#[doc = "CRU_CLKSEL_CON61 (rw) register accessor: Internal clock select and divide register61\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con61::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con61::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con61`]
module"]
#[doc(alias = "CRU_CLKSEL_CON61")]
pub type CruClkselCon61 = crate::Reg<cru_clksel_con61::CruClkselCon61Spec>;
#[doc = "Internal clock select and divide register61"]
pub mod cru_clksel_con61;
#[doc = "CRU_CLKSEL_CON62 (rw) register accessor: Internal clock select and divide register62\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con62::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con62::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con62`]
module"]
#[doc(alias = "CRU_CLKSEL_CON62")]
pub type CruClkselCon62 = crate::Reg<cru_clksel_con62::CruClkselCon62Spec>;
#[doc = "Internal clock select and divide register62"]
pub mod cru_clksel_con62;
#[doc = "CRU_CLKSEL_CON63 (rw) register accessor: Internal clock select and divide register63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con63::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con63`]
module"]
#[doc(alias = "CRU_CLKSEL_CON63")]
pub type CruClkselCon63 = crate::Reg<cru_clksel_con63::CruClkselCon63Spec>;
#[doc = "Internal clock select and divide register63"]
pub mod cru_clksel_con63;
#[doc = "CRU_CLKSEL_CON64 (rw) register accessor: Internal clock select and divide register64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con64`]
module"]
#[doc(alias = "CRU_CLKSEL_CON64")]
pub type CruClkselCon64 = crate::Reg<cru_clksel_con64::CruClkselCon64Spec>;
#[doc = "Internal clock select and divide register64"]
pub mod cru_clksel_con64;
#[doc = "CRU_CLKSEL_CON65 (rw) register accessor: Internal clock select and divide register65\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con65::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con65::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con65`]
module"]
#[doc(alias = "CRU_CLKSEL_CON65")]
pub type CruClkselCon65 = crate::Reg<cru_clksel_con65::CruClkselCon65Spec>;
#[doc = "Internal clock select and divide register65"]
pub mod cru_clksel_con65;
#[doc = "CRU_CLKSEL_CON96 (rw) register accessor: Internal clock select and divide register80\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con96::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con96::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con96`]
module"]
#[doc(alias = "CRU_CLKSEL_CON96")]
pub type CruClkselCon96 = crate::Reg<cru_clksel_con96::CruClkselCon96Spec>;
#[doc = "Internal clock select and divide register80"]
pub mod cru_clksel_con96;
#[doc = "CRU_CLKSEL_CON97 (rw) register accessor: Internal clock select and divide register81\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con97::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con97::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con97`]
module"]
#[doc(alias = "CRU_CLKSEL_CON97")]
pub type CruClkselCon97 = crate::Reg<cru_clksel_con97::CruClkselCon97Spec>;
#[doc = "Internal clock select and divide register81"]
pub mod cru_clksel_con97;
#[doc = "CRU_CLKSEL_CON98 (rw) register accessor: Internal clock select and divide register82\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con98::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con98::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con98`]
module"]
#[doc(alias = "CRU_CLKSEL_CON98")]
pub type CruClkselCon98 = crate::Reg<cru_clksel_con98::CruClkselCon98Spec>;
#[doc = "Internal clock select and divide register82"]
pub mod cru_clksel_con98;
#[doc = "CRU_CLKSEL_CON99 (rw) register accessor: Internal clock select and divide register83\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con99::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con99::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con99`]
module"]
#[doc(alias = "CRU_CLKSEL_CON99")]
pub type CruClkselCon99 = crate::Reg<cru_clksel_con99::CruClkselCon99Spec>;
#[doc = "Internal clock select and divide register83"]
pub mod cru_clksel_con99;
#[doc = "CRU_CLKSEL_CON100 (rw) register accessor: Internal clock select and divide register84\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con100::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con100::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con100`]
module"]
#[doc(alias = "CRU_CLKSEL_CON100")]
pub type CruClkselCon100 = crate::Reg<cru_clksel_con100::CruClkselCon100Spec>;
#[doc = "Internal clock select and divide register84"]
pub mod cru_clksel_con100;
#[doc = "CRU_CLKSEL_CON101 (rw) register accessor: Internal clock select and divide register85\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con101::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con101::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con101`]
module"]
#[doc(alias = "CRU_CLKSEL_CON101")]
pub type CruClkselCon101 = crate::Reg<cru_clksel_con101::CruClkselCon101Spec>;
#[doc = "Internal clock select and divide register85"]
pub mod cru_clksel_con101;
#[doc = "CRU_CLKSEL_CON102 (rw) register accessor: Internal clock select and divide register86\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con102::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con102::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con102`]
module"]
#[doc(alias = "CRU_CLKSEL_CON102")]
pub type CruClkselCon102 = crate::Reg<cru_clksel_con102::CruClkselCon102Spec>;
#[doc = "Internal clock select and divide register86"]
pub mod cru_clksel_con102;
#[doc = "CRU_CLKSEL_CON103 (rw) register accessor: Internal clock select and divide register87\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con103::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con103::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con103`]
module"]
#[doc(alias = "CRU_CLKSEL_CON103")]
pub type CruClkselCon103 = crate::Reg<cru_clksel_con103::CruClkselCon103Spec>;
#[doc = "Internal clock select and divide register87"]
pub mod cru_clksel_con103;
#[doc = "CRU_CLKSEL_CON105 (rw) register accessor: Internal clock select and divide register89\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con105::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con105::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con105`]
module"]
#[doc(alias = "CRU_CLKSEL_CON105")]
pub type CruClkselCon105 = crate::Reg<cru_clksel_con105::CruClkselCon105Spec>;
#[doc = "Internal clock select and divide register89"]
pub mod cru_clksel_con105;
#[doc = "CRU_CLKSEL_CON106 (rw) register accessor: Internal clock select and divide register90\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con106::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con106::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con106`]
module"]
#[doc(alias = "CRU_CLKSEL_CON106")]
pub type CruClkselCon106 = crate::Reg<cru_clksel_con106::CruClkselCon106Spec>;
#[doc = "Internal clock select and divide register90"]
pub mod cru_clksel_con106;
#[doc = "CRU_CLKSEL_CON107 (rw) register accessor: Internal clock select and divide register91\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con107::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con107::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clksel_con107`]
module"]
#[doc(alias = "CRU_CLKSEL_CON107")]
pub type CruClkselCon107 = crate::Reg<cru_clksel_con107::CruClkselCon107Spec>;
#[doc = "Internal clock select and divide register91"]
pub mod cru_clksel_con107;
#[doc = "CRU_CLKGATE_CON0 (rw) register accessor: Internal clock gating register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con0`]
module"]
#[doc(alias = "CRU_CLKGATE_CON0")]
pub type CruClkgateCon0 = crate::Reg<cru_clkgate_con0::CruClkgateCon0Spec>;
#[doc = "Internal clock gating register0"]
pub mod cru_clkgate_con0;
#[doc = "CRU_CLKGATE_CON1 (rw) register accessor: Internal clock gating register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con1`]
module"]
#[doc(alias = "CRU_CLKGATE_CON1")]
pub type CruClkgateCon1 = crate::Reg<cru_clkgate_con1::CruClkgateCon1Spec>;
#[doc = "Internal clock gating register1"]
pub mod cru_clkgate_con1;
#[doc = "CRU_CLKGATE_CON2 (rw) register accessor: Internal clock gating register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con2`]
module"]
#[doc(alias = "CRU_CLKGATE_CON2")]
pub type CruClkgateCon2 = crate::Reg<cru_clkgate_con2::CruClkgateCon2Spec>;
#[doc = "Internal clock gating register2"]
pub mod cru_clkgate_con2;
#[doc = "CRU_CLKGATE_CON3 (rw) register accessor: Internal clock gating register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con3`]
module"]
#[doc(alias = "CRU_CLKGATE_CON3")]
pub type CruClkgateCon3 = crate::Reg<cru_clkgate_con3::CruClkgateCon3Spec>;
#[doc = "Internal clock gating register3"]
pub mod cru_clkgate_con3;
#[doc = "CRU_CLKGATE_CON4 (rw) register accessor: Internal clock gating register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con4`]
module"]
#[doc(alias = "CRU_CLKGATE_CON4")]
pub type CruClkgateCon4 = crate::Reg<cru_clkgate_con4::CruClkgateCon4Spec>;
#[doc = "Internal clock gating register4"]
pub mod cru_clkgate_con4;
#[doc = "CRU_CLKGATE_CON5 (rw) register accessor: Internal clock gating register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con5`]
module"]
#[doc(alias = "CRU_CLKGATE_CON5")]
pub type CruClkgateCon5 = crate::Reg<cru_clkgate_con5::CruClkgateCon5Spec>;
#[doc = "Internal clock gating register5"]
pub mod cru_clkgate_con5;
#[doc = "CRU_CLKGATE_CON6 (rw) register accessor: Internal clock gating register6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con6`]
module"]
#[doc(alias = "CRU_CLKGATE_CON6")]
pub type CruClkgateCon6 = crate::Reg<cru_clkgate_con6::CruClkgateCon6Spec>;
#[doc = "Internal clock gating register6"]
pub mod cru_clkgate_con6;
#[doc = "CRU_CLKGATE_CON7 (rw) register accessor: Internal clock gating register7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con7`]
module"]
#[doc(alias = "CRU_CLKGATE_CON7")]
pub type CruClkgateCon7 = crate::Reg<cru_clkgate_con7::CruClkgateCon7Spec>;
#[doc = "Internal clock gating register7"]
pub mod cru_clkgate_con7;
#[doc = "CRU_CLKGATE_CON8 (rw) register accessor: Internal clock gating register8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con8`]
module"]
#[doc(alias = "CRU_CLKGATE_CON8")]
pub type CruClkgateCon8 = crate::Reg<cru_clkgate_con8::CruClkgateCon8Spec>;
#[doc = "Internal clock gating register8"]
pub mod cru_clkgate_con8;
#[doc = "CRU_CLKGATE_CON9 (rw) register accessor: Internal clock gating register9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con9`]
module"]
#[doc(alias = "CRU_CLKGATE_CON9")]
pub type CruClkgateCon9 = crate::Reg<cru_clkgate_con9::CruClkgateCon9Spec>;
#[doc = "Internal clock gating register9"]
pub mod cru_clkgate_con9;
#[doc = "CRU_CLKGATE_CON10 (rw) register accessor: Internal clock gating register10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con10`]
module"]
#[doc(alias = "CRU_CLKGATE_CON10")]
pub type CruClkgateCon10 = crate::Reg<cru_clkgate_con10::CruClkgateCon10Spec>;
#[doc = "Internal clock gating register10"]
pub mod cru_clkgate_con10;
#[doc = "CRU_CLKGATE_CON11 (rw) register accessor: Internal clock gating register11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con11`]
module"]
#[doc(alias = "CRU_CLKGATE_CON11")]
pub type CruClkgateCon11 = crate::Reg<cru_clkgate_con11::CruClkgateCon11Spec>;
#[doc = "Internal clock gating register11"]
pub mod cru_clkgate_con11;
#[doc = "CRU_CLKGATE_CON12 (rw) register accessor: Internal clock gating register12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con12`]
module"]
#[doc(alias = "CRU_CLKGATE_CON12")]
pub type CruClkgateCon12 = crate::Reg<cru_clkgate_con12::CruClkgateCon12Spec>;
#[doc = "Internal clock gating register12"]
pub mod cru_clkgate_con12;
#[doc = "CRU_CLKGATE_CON13 (rw) register accessor: Internal clock gating register13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con13`]
module"]
#[doc(alias = "CRU_CLKGATE_CON13")]
pub type CruClkgateCon13 = crate::Reg<cru_clkgate_con13::CruClkgateCon13Spec>;
#[doc = "Internal clock gating register13"]
pub mod cru_clkgate_con13;
#[doc = "CRU_CLKGATE_CON14 (rw) register accessor: Internal clock gating register14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con14`]
module"]
#[doc(alias = "CRU_CLKGATE_CON14")]
pub type CruClkgateCon14 = crate::Reg<cru_clkgate_con14::CruClkgateCon14Spec>;
#[doc = "Internal clock gating register14"]
pub mod cru_clkgate_con14;
#[doc = "CRU_CLKGATE_CON15 (rw) register accessor: Internal clock gating register15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con15`]
module"]
#[doc(alias = "CRU_CLKGATE_CON15")]
pub type CruClkgateCon15 = crate::Reg<cru_clkgate_con15::CruClkgateCon15Spec>;
#[doc = "Internal clock gating register15"]
pub mod cru_clkgate_con15;
#[doc = "CRU_CLKGATE_CON16 (rw) register accessor: Internal clock gating register16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con16`]
module"]
#[doc(alias = "CRU_CLKGATE_CON16")]
pub type CruClkgateCon16 = crate::Reg<cru_clkgate_con16::CruClkgateCon16Spec>;
#[doc = "Internal clock gating register16"]
pub mod cru_clkgate_con16;
#[doc = "CRU_CLKGATE_CON17 (rw) register accessor: Internal clock gating register17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con17`]
module"]
#[doc(alias = "CRU_CLKGATE_CON17")]
pub type CruClkgateCon17 = crate::Reg<cru_clkgate_con17::CruClkgateCon17Spec>;
#[doc = "Internal clock gating register17"]
pub mod cru_clkgate_con17;
#[doc = "CRU_CLKGATE_CON18 (rw) register accessor: Internal clock gating register18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con18`]
module"]
#[doc(alias = "CRU_CLKGATE_CON18")]
pub type CruClkgateCon18 = crate::Reg<cru_clkgate_con18::CruClkgateCon18Spec>;
#[doc = "Internal clock gating register18"]
pub mod cru_clkgate_con18;
#[doc = "CRU_CLKGATE_CON19 (rw) register accessor: Internal clock gating register19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con19`]
module"]
#[doc(alias = "CRU_CLKGATE_CON19")]
pub type CruClkgateCon19 = crate::Reg<cru_clkgate_con19::CruClkgateCon19Spec>;
#[doc = "Internal clock gating register19"]
pub mod cru_clkgate_con19;
#[doc = "CRU_CLKGATE_CON20 (rw) register accessor: Internal clock gating register20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con20`]
module"]
#[doc(alias = "CRU_CLKGATE_CON20")]
pub type CruClkgateCon20 = crate::Reg<cru_clkgate_con20::CruClkgateCon20Spec>;
#[doc = "Internal clock gating register20"]
pub mod cru_clkgate_con20;
#[doc = "CRU_CLKGATE_CON21 (rw) register accessor: Internal clock gating register21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con21`]
module"]
#[doc(alias = "CRU_CLKGATE_CON21")]
pub type CruClkgateCon21 = crate::Reg<cru_clkgate_con21::CruClkgateCon21Spec>;
#[doc = "Internal clock gating register21"]
pub mod cru_clkgate_con21;
#[doc = "CRU_CLKGATE_CON22 (rw) register accessor: Internal clock gating register22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con22`]
module"]
#[doc(alias = "CRU_CLKGATE_CON22")]
pub type CruClkgateCon22 = crate::Reg<cru_clkgate_con22::CruClkgateCon22Spec>;
#[doc = "Internal clock gating register22"]
pub mod cru_clkgate_con22;
#[doc = "CRU_CLKGATE_CON23 (rw) register accessor: Internal clock gating register23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con23`]
module"]
#[doc(alias = "CRU_CLKGATE_CON23")]
pub type CruClkgateCon23 = crate::Reg<cru_clkgate_con23::CruClkgateCon23Spec>;
#[doc = "Internal clock gating register23"]
pub mod cru_clkgate_con23;
#[doc = "CRU_CLKGATE_CON24 (rw) register accessor: Internal clock gating register24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con24`]
module"]
#[doc(alias = "CRU_CLKGATE_CON24")]
pub type CruClkgateCon24 = crate::Reg<cru_clkgate_con24::CruClkgateCon24Spec>;
#[doc = "Internal clock gating register24"]
pub mod cru_clkgate_con24;
#[doc = "CRU_CLKGATE_CON25 (rw) register accessor: Internal clock gating register25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con25`]
module"]
#[doc(alias = "CRU_CLKGATE_CON25")]
pub type CruClkgateCon25 = crate::Reg<cru_clkgate_con25::CruClkgateCon25Spec>;
#[doc = "Internal clock gating register25"]
pub mod cru_clkgate_con25;
#[doc = "CRU_CLKGATE_CON26 (rw) register accessor: Internal clock gating register26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con26`]
module"]
#[doc(alias = "CRU_CLKGATE_CON26")]
pub type CruClkgateCon26 = crate::Reg<cru_clkgate_con26::CruClkgateCon26Spec>;
#[doc = "Internal clock gating register26"]
pub mod cru_clkgate_con26;
#[doc = "CRU_CLKGATE_CON27 (rw) register accessor: Internal clock gating register27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con27`]
module"]
#[doc(alias = "CRU_CLKGATE_CON27")]
pub type CruClkgateCon27 = crate::Reg<cru_clkgate_con27::CruClkgateCon27Spec>;
#[doc = "Internal clock gating register27"]
pub mod cru_clkgate_con27;
#[doc = "CRU_CLKGATE_CON28 (rw) register accessor: Internal clock gating register28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con28`]
module"]
#[doc(alias = "CRU_CLKGATE_CON28")]
pub type CruClkgateCon28 = crate::Reg<cru_clkgate_con28::CruClkgateCon28Spec>;
#[doc = "Internal clock gating register28"]
pub mod cru_clkgate_con28;
#[doc = "CRU_CLKGATE_CON29 (rw) register accessor: Internal clock gating register29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con29`]
module"]
#[doc(alias = "CRU_CLKGATE_CON29")]
pub type CruClkgateCon29 = crate::Reg<cru_clkgate_con29::CruClkgateCon29Spec>;
#[doc = "Internal clock gating register29"]
pub mod cru_clkgate_con29;
#[doc = "CRU_CLKGATE_CON30 (rw) register accessor: Internal clock gating register30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con30`]
module"]
#[doc(alias = "CRU_CLKGATE_CON30")]
pub type CruClkgateCon30 = crate::Reg<cru_clkgate_con30::CruClkgateCon30Spec>;
#[doc = "Internal clock gating register30"]
pub mod cru_clkgate_con30;
#[doc = "CRU_CLKGATE_CON31 (rw) register accessor: Internal clock gating register31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con31`]
module"]
#[doc(alias = "CRU_CLKGATE_CON31")]
pub type CruClkgateCon31 = crate::Reg<cru_clkgate_con31::CruClkgateCon31Spec>;
#[doc = "Internal clock gating register31"]
pub mod cru_clkgate_con31;
#[doc = "CRU_CLKGATE_CON32 (rw) register accessor: Internal clock gating register32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con32`]
module"]
#[doc(alias = "CRU_CLKGATE_CON32")]
pub type CruClkgateCon32 = crate::Reg<cru_clkgate_con32::CruClkgateCon32Spec>;
#[doc = "Internal clock gating register32"]
pub mod cru_clkgate_con32;
#[doc = "CRU_CLKGATE_CON33 (rw) register accessor: Internal clock gating register33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con33`]
module"]
#[doc(alias = "CRU_CLKGATE_CON33")]
pub type CruClkgateCon33 = crate::Reg<cru_clkgate_con33::CruClkgateCon33Spec>;
#[doc = "Internal clock gating register33"]
pub mod cru_clkgate_con33;
#[doc = "CRU_CLKGATE_CON34 (rw) register accessor: Internal clock gating register34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_clkgate_con34`]
module"]
#[doc(alias = "CRU_CLKGATE_CON34")]
pub type CruClkgateCon34 = crate::Reg<cru_clkgate_con34::CruClkgateCon34Spec>;
#[doc = "Internal clock gating register34"]
pub mod cru_clkgate_con34;
#[doc = "CRU_SOFTRST_CON0 (rw) register accessor: Internal software reset control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con0`]
module"]
#[doc(alias = "CRU_SOFTRST_CON0")]
pub type CruSoftrstCon0 = crate::Reg<cru_softrst_con0::CruSoftrstCon0Spec>;
#[doc = "Internal software reset control register0"]
pub mod cru_softrst_con0;
#[doc = "CRU_SOFTRST_CON1 (rw) register accessor: Internal software reset control register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con1`]
module"]
#[doc(alias = "CRU_SOFTRST_CON1")]
pub type CruSoftrstCon1 = crate::Reg<cru_softrst_con1::CruSoftrstCon1Spec>;
#[doc = "Internal software reset control register1"]
pub mod cru_softrst_con1;
#[doc = "CRU_SOFTRST_CON2 (rw) register accessor: Internal software reset control register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con2`]
module"]
#[doc(alias = "CRU_SOFTRST_CON2")]
pub type CruSoftrstCon2 = crate::Reg<cru_softrst_con2::CruSoftrstCon2Spec>;
#[doc = "Internal software reset control register2"]
pub mod cru_softrst_con2;
#[doc = "CRU_SOFTRST_CON3 (rw) register accessor: Internal software reset control register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con3`]
module"]
#[doc(alias = "CRU_SOFTRST_CON3")]
pub type CruSoftrstCon3 = crate::Reg<cru_softrst_con3::CruSoftrstCon3Spec>;
#[doc = "Internal software reset control register3"]
pub mod cru_softrst_con3;
#[doc = "CRU_SOFTRST_CON4 (rw) register accessor: Internal software reset control register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con4`]
module"]
#[doc(alias = "CRU_SOFTRST_CON4")]
pub type CruSoftrstCon4 = crate::Reg<cru_softrst_con4::CruSoftrstCon4Spec>;
#[doc = "Internal software reset control register4"]
pub mod cru_softrst_con4;
#[doc = "CRU_SOFTRST_CON5 (rw) register accessor: Internal software reset control register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con5`]
module"]
#[doc(alias = "CRU_SOFTRST_CON5")]
pub type CruSoftrstCon5 = crate::Reg<cru_softrst_con5::CruSoftrstCon5Spec>;
#[doc = "Internal software reset control register5"]
pub mod cru_softrst_con5;
#[doc = "CRU_SOFTRST_CON6 (rw) register accessor: Internal software reset control register6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con6`]
module"]
#[doc(alias = "CRU_SOFTRST_CON6")]
pub type CruSoftrstCon6 = crate::Reg<cru_softrst_con6::CruSoftrstCon6Spec>;
#[doc = "Internal software reset control register6"]
pub mod cru_softrst_con6;
#[doc = "CRU_SOFTRST_CON7 (rw) register accessor: Internal software reset control register7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con7`]
module"]
#[doc(alias = "CRU_SOFTRST_CON7")]
pub type CruSoftrstCon7 = crate::Reg<cru_softrst_con7::CruSoftrstCon7Spec>;
#[doc = "Internal software reset control register7"]
pub mod cru_softrst_con7;
#[doc = "CRU_SOFTRST_CON8 (rw) register accessor: Internal software reset control register8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con8`]
module"]
#[doc(alias = "CRU_SOFTRST_CON8")]
pub type CruSoftrstCon8 = crate::Reg<cru_softrst_con8::CruSoftrstCon8Spec>;
#[doc = "Internal software reset control register8"]
pub mod cru_softrst_con8;
#[doc = "CRU_SOFTRST_CON9 (rw) register accessor: Internal software reset control register9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con9`]
module"]
#[doc(alias = "CRU_SOFTRST_CON9")]
pub type CruSoftrstCon9 = crate::Reg<cru_softrst_con9::CruSoftrstCon9Spec>;
#[doc = "Internal software reset control register9"]
pub mod cru_softrst_con9;
#[doc = "CRU_SOFTRST_CON10 (rw) register accessor: Internal software reset control register10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con10`]
module"]
#[doc(alias = "CRU_SOFTRST_CON10")]
pub type CruSoftrstCon10 = crate::Reg<cru_softrst_con10::CruSoftrstCon10Spec>;
#[doc = "Internal software reset control register10"]
pub mod cru_softrst_con10;
#[doc = "CRU_SOFTRST_CON11 (rw) register accessor: Internal software reset control register11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con11`]
module"]
#[doc(alias = "CRU_SOFTRST_CON11")]
pub type CruSoftrstCon11 = crate::Reg<cru_softrst_con11::CruSoftrstCon11Spec>;
#[doc = "Internal software reset control register11"]
pub mod cru_softrst_con11;
#[doc = "CRU_SOFTRST_CON12 (rw) register accessor: Internal software reset control register12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con12`]
module"]
#[doc(alias = "CRU_SOFTRST_CON12")]
pub type CruSoftrstCon12 = crate::Reg<cru_softrst_con12::CruSoftrstCon12Spec>;
#[doc = "Internal software reset control register12"]
pub mod cru_softrst_con12;
#[doc = "CRU_SOFTRST_CON13 (rw) register accessor: Internal software reset control register13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con13`]
module"]
#[doc(alias = "CRU_SOFTRST_CON13")]
pub type CruSoftrstCon13 = crate::Reg<cru_softrst_con13::CruSoftrstCon13Spec>;
#[doc = "Internal software reset control register13"]
pub mod cru_softrst_con13;
#[doc = "CRU_SOFTRST_CON14 (rw) register accessor: Internal software reset control register14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con14`]
module"]
#[doc(alias = "CRU_SOFTRST_CON14")]
pub type CruSoftrstCon14 = crate::Reg<cru_softrst_con14::CruSoftrstCon14Spec>;
#[doc = "Internal software reset control register14"]
pub mod cru_softrst_con14;
#[doc = "CRU_SOFTRST_CON15 (rw) register accessor: Internal software reset control register15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con15`]
module"]
#[doc(alias = "CRU_SOFTRST_CON15")]
pub type CruSoftrstCon15 = crate::Reg<cru_softrst_con15::CruSoftrstCon15Spec>;
#[doc = "Internal software reset control register15"]
pub mod cru_softrst_con15;
#[doc = "CRU_SOFTRST_CON16 (rw) register accessor: Internal software reset control register16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con16`]
module"]
#[doc(alias = "CRU_SOFTRST_CON16")]
pub type CruSoftrstCon16 = crate::Reg<cru_softrst_con16::CruSoftrstCon16Spec>;
#[doc = "Internal software reset control register16"]
pub mod cru_softrst_con16;
#[doc = "CRU_SOFTRST_CON17 (rw) register accessor: Internal software reset control register17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con17`]
module"]
#[doc(alias = "CRU_SOFTRST_CON17")]
pub type CruSoftrstCon17 = crate::Reg<cru_softrst_con17::CruSoftrstCon17Spec>;
#[doc = "Internal software reset control register17"]
pub mod cru_softrst_con17;
#[doc = "CRU_SOFTRST_CON18 (rw) register accessor: Internal software reset control register18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con18`]
module"]
#[doc(alias = "CRU_SOFTRST_CON18")]
pub type CruSoftrstCon18 = crate::Reg<cru_softrst_con18::CruSoftrstCon18Spec>;
#[doc = "Internal software reset control register18"]
pub mod cru_softrst_con18;
#[doc = "CRU_SOFTRST_CON19 (rw) register accessor: Internal software reset control register19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con19`]
module"]
#[doc(alias = "CRU_SOFTRST_CON19")]
pub type CruSoftrstCon19 = crate::Reg<cru_softrst_con19::CruSoftrstCon19Spec>;
#[doc = "Internal software reset control register19"]
pub mod cru_softrst_con19;
#[doc = "CRU_SOFTRST_CON20 (rw) register accessor: Internal software reset control register20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_softrst_con20`]
module"]
#[doc(alias = "CRU_SOFTRST_CON20")]
pub type CruSoftrstCon20 = crate::Reg<cru_softrst_con20::CruSoftrstCon20Spec>;
#[doc = "Internal software reset control register20"]
pub mod cru_softrst_con20;
#[doc = "CRU_GLB_SRST_FST_VALUE (rw) register accessor: The first global software reset config value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_glb_srst_fst_value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_glb_srst_fst_value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_glb_srst_fst_value`]
module"]
#[doc(alias = "CRU_GLB_SRST_FST_VALUE")]
pub type CruGlbSrstFstValue = crate::Reg<cru_glb_srst_fst_value::CruGlbSrstFstValueSpec>;
#[doc = "The first global software reset config value"]
pub mod cru_glb_srst_fst_value;
#[doc = "CRU_GLB_SRST_SND_VALUE (rw) register accessor: The second global software reset config value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_glb_srst_snd_value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_glb_srst_snd_value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_glb_srst_snd_value`]
module"]
#[doc(alias = "CRU_GLB_SRST_SND_VALUE")]
pub type CruGlbSrstSndValue = crate::Reg<cru_glb_srst_snd_value::CruGlbSrstSndValueSpec>;
#[doc = "The second global software reset config value"]
pub mod cru_glb_srst_snd_value;
#[doc = "CRU_GLB_CNT_TH (rw) register accessor: Global soft reset counter threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_glb_cnt_th::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_glb_cnt_th::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_glb_cnt_th`]
module"]
#[doc(alias = "CRU_GLB_CNT_TH")]
pub type CruGlbCntTh = crate::Reg<cru_glb_cnt_th::CruGlbCntThSpec>;
#[doc = "Global soft reset counter threshold"]
pub mod cru_glb_cnt_th;
#[doc = "CRU_MISC_CON (rw) register accessor: Output clock selection for test\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_misc_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_misc_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_misc_con`]
module"]
#[doc(alias = "CRU_MISC_CON")]
pub type CruMiscCon = crate::Reg<cru_misc_con::CruMiscConSpec>;
#[doc = "Output clock selection for test"]
pub mod cru_misc_con;
#[doc = "CRU_GLB_RST_CON (rw) register accessor: Global reset trigger select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_glb_rst_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_glb_rst_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_glb_rst_con`]
module"]
#[doc(alias = "CRU_GLB_RST_CON")]
pub type CruGlbRstCon = crate::Reg<cru_glb_rst_con::CruGlbRstConSpec>;
#[doc = "Global reset trigger select"]
pub mod cru_glb_rst_con;
#[doc = "CRU_GLB_RST_ST (rw) register accessor: Global reset status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_glb_rst_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_glb_rst_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_glb_rst_st`]
module"]
#[doc(alias = "CRU_GLB_RST_ST")]
pub type CruGlbRstSt = crate::Reg<cru_glb_rst_st::CruGlbRstStSpec>;
#[doc = "Global reset status"]
pub mod cru_glb_rst_st;
#[doc = "CRU_SDMMC_CON0 (w) register accessor: sdmmc control0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_sdmmc_con0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_sdmmc_con0`]
module"]
#[doc(alias = "CRU_SDMMC_CON0")]
pub type CruSdmmcCon0 = crate::Reg<cru_sdmmc_con0::CruSdmmcCon0Spec>;
#[doc = "sdmmc control0"]
pub mod cru_sdmmc_con0;
#[doc = "CRU_SDMMC_CON1 (w) register accessor: sdmmc control1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_sdmmc_con1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_sdmmc_con1`]
module"]
#[doc(alias = "CRU_SDMMC_CON1")]
pub type CruSdmmcCon1 = crate::Reg<cru_sdmmc_con1::CruSdmmcCon1Spec>;
#[doc = "sdmmc control1"]
pub mod cru_sdmmc_con1;
#[doc = "CRU_SDIO0_CON0 (w) register accessor: sdio0 control0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_sdio0_con0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_sdio0_con0`]
module"]
#[doc(alias = "CRU_SDIO0_CON0")]
pub type CruSdio0Con0 = crate::Reg<cru_sdio0_con0::CruSdio0Con0Spec>;
#[doc = "sdio0 control0"]
pub mod cru_sdio0_con0;
#[doc = "CRU_SDIO0_CON1 (w) register accessor: sdio0 control1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_sdio0_con1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cru_sdio0_con1`]
module"]
#[doc(alias = "CRU_SDIO0_CON1")]
pub type CruSdio0Con1 = crate::Reg<cru_sdio0_con1::CruSdio0Con1Spec>;
#[doc = "sdio0 control1"]
pub mod cru_sdio0_con1;
