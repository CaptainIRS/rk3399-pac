#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    swreg_0: Swreg0,
    swreg_1: Swreg1,
    swreg_2: Swreg2,
    swreg_3: Swreg3,
    swreg_4: Swreg4,
    swreg_5: Swreg5,
    swreg_6: Swreg6,
    swreg_7: Swreg7,
    swreg_8: Swreg8,
    swreg_9: Swreg9,
    swreg_10: Swreg10,
    swreg_11: Swreg11,
    swreg_12: Swreg12,
    swreg_13: Swreg13,
    swreg_14: Swreg14,
    swreg_15: Swreg15,
    swreg_16: Swreg16,
    swreg_17: Swreg17,
    swreg_18: Swreg18,
    swreg_19: Swreg19,
    swreg_20: Swreg20,
    swreg_21: Swreg21,
    swreg_22: Swreg22,
    swreg_23: Swreg23,
    swreg_24: Swreg24,
    swreg_25: Swreg25,
    swreg_26: Swreg26,
    swreg_27: Swreg27,
    swreg_28: Swreg28,
    swreg_29: Swreg29,
    swreg_30: Swreg30,
    swreg_31: Swreg31,
    _reserved32: [u8; 0x30],
    swreg_44: Swreg44,
    swreg_45: Swreg45,
    swreg_46: Swreg46,
    swreg_47: Swreg47,
    swreg_48: Swreg48,
    swreg_49: Swreg49,
    swreg_50: Swreg50,
    swreg_51: Swreg51,
    swreg_52: Swreg52,
    swreg_53: Swreg53,
    swreg_54: Swreg54,
    swreg_55: Swreg55,
    swreg_56: Swreg56,
    swreg_57: Swreg57,
    swreg_58: Swreg58,
    swreg_59: Swreg59,
    swreg_60: Swreg60,
    swreg_61: Swreg61,
    swreg_62: Swreg62,
    swreg_63: Swreg63,
    swreg_64: Swreg64,
    swreg_65_reuse: Swreg65Reuse,
    swreg_66_reuse: Swreg66Reuse,
    swreg_67_reuse: Swreg67Reuse,
    swreg_68_reuse: Swreg68Reuse,
    swreg_69_reuse: Swreg69Reuse,
    swreg_70_reuse: Swreg70Reuse,
    swreg_71_reuse: Swreg71Reuse,
    swreg_72_reuse: Swreg72Reuse,
    swreg_73_reuse: Swreg73Reuse,
    swreg_74: Swreg74,
    swreg_75: Swreg75,
    swreg_76_reuse: Swreg76Reuse,
    swreg_77: Swreg77,
    swreg_78: Swreg78,
    swreg_79: Swreg79,
    swreg_80: Swreg80,
    swreg_81: Swreg81,
    swreg_82: Swreg82,
    swreg_83: Swreg83,
    swreg_84: Swreg84,
    swreg_85: Swreg85,
    swreg_86: Swreg86,
    swreg_87: Swreg87,
    swreg_88: Swreg88,
    swreg_89: Swreg89,
    swreg_90: Swreg90,
    swreg_91: Swreg91,
    swreg_92: Swreg92,
    swreg_93: Swreg93,
    swreg_94: Swreg94,
    swreg_95: Swreg95,
    swreg_96: Swreg96,
    swreg_97: Swreg97,
    swreg_98: Swreg98,
    swreg_99: Swreg99,
    swreg_100_reuse: Swreg100Reuse,
    swreg_101_read: Swreg101Read,
    swreg_102: Swreg102,
    swreg_103: Swreg103,
    swreg_104: Swreg104,
    swreg_105: Swreg105,
    swreg_106_reuse: Swreg106Reuse,
    swreg_107_reuse: Swreg107Reuse,
    swreg_108_reuse: Swreg108Reuse,
    swreg_109: Swreg109,
    swreg_110_read: Swreg110Read,
    _reserved99: [u8; 0x24],
    swreg_120_183: Swreg120_183,
}
impl RegisterBlock {
    #[doc = "0x00 - 1st quantization for jpeg lumin table"]
    #[inline(always)]
    pub const fn swreg_0(&self) -> &Swreg0 {
        &self.swreg_0
    }
    #[doc = "0x04 - 2st quantization for jpeg lumin table"]
    #[inline(always)]
    pub const fn swreg_1(&self) -> &Swreg1 {
        &self.swreg_1
    }
    #[doc = "0x08 - 3st quantization for jpeg lumin table"]
    #[inline(always)]
    pub const fn swreg_2(&self) -> &Swreg2 {
        &self.swreg_2
    }
    #[doc = "0x0c - 4st quantization for jpeg lumin table"]
    #[inline(always)]
    pub const fn swreg_3(&self) -> &Swreg3 {
        &self.swreg_3
    }
    #[doc = "0x10 - 5st quantization for jpeg lumin table"]
    #[inline(always)]
    pub const fn swreg_4(&self) -> &Swreg4 {
        &self.swreg_4
    }
    #[doc = "0x14 - 6st quantization for jpeg lumin table/part 1 for qp round"]
    #[inline(always)]
    pub const fn swreg_5(&self) -> &Swreg5 {
        &self.swreg_5
    }
    #[doc = "0x18 - 7st quantization for jpeg lumin table"]
    #[inline(always)]
    pub const fn swreg_6(&self) -> &Swreg6 {
        &self.swreg_6
    }
    #[doc = "0x1c - 8st quantization for jpeg lumin table"]
    #[inline(always)]
    pub const fn swreg_7(&self) -> &Swreg7 {
        &self.swreg_7
    }
    #[doc = "0x20 - 9st quantization for jpeg lumin table"]
    #[inline(always)]
    pub const fn swreg_8(&self) -> &Swreg8 {
        &self.swreg_8
    }
    #[doc = "0x24 - 10st quantization for jpeg lumin table"]
    #[inline(always)]
    pub const fn swreg_9(&self) -> &Swreg9 {
        &self.swreg_9
    }
    #[doc = "0x28 - 11st quantization for jpeg lumin table"]
    #[inline(always)]
    pub const fn swreg_10(&self) -> &Swreg10 {
        &self.swreg_10
    }
    #[doc = "0x2c - 12st quantization for jpeg lumin table"]
    #[inline(always)]
    pub const fn swreg_11(&self) -> &Swreg11 {
        &self.swreg_11
    }
    #[doc = "0x30 - 13st quantization for jpeg lumin table"]
    #[inline(always)]
    pub const fn swreg_12(&self) -> &Swreg12 {
        &self.swreg_12
    }
    #[doc = "0x34 - 14st quantization for jpeg lumin table"]
    #[inline(always)]
    pub const fn swreg_13(&self) -> &Swreg13 {
        &self.swreg_13
    }
    #[doc = "0x38 - 15st quantization for jpeg lumin table"]
    #[inline(always)]
    pub const fn swreg_14(&self) -> &Swreg14 {
        &self.swreg_14
    }
    #[doc = "0x3c - 16st quantization for jpeg lumin table"]
    #[inline(always)]
    pub const fn swreg_15(&self) -> &Swreg15 {
        &self.swreg_15
    }
    #[doc = "0x40 - 1st quantization for jpeg chroma table"]
    #[inline(always)]
    pub const fn swreg_16(&self) -> &Swreg16 {
        &self.swreg_16
    }
    #[doc = "0x44 - 2st quantization for jpeg chroma table"]
    #[inline(always)]
    pub const fn swreg_17(&self) -> &Swreg17 {
        &self.swreg_17
    }
    #[doc = "0x48 - 3st quantization for jpeg chroma table"]
    #[inline(always)]
    pub const fn swreg_18(&self) -> &Swreg18 {
        &self.swreg_18
    }
    #[doc = "0x4c - 4st quantization for jpeg chroma table"]
    #[inline(always)]
    pub const fn swreg_19(&self) -> &Swreg19 {
        &self.swreg_19
    }
    #[doc = "0x50 - 5st quantization for jpeg chroma table"]
    #[inline(always)]
    pub const fn swreg_20(&self) -> &Swreg20 {
        &self.swreg_20
    }
    #[doc = "0x54 - 6st quantization for jpeg chroma table"]
    #[inline(always)]
    pub const fn swreg_21(&self) -> &Swreg21 {
        &self.swreg_21
    }
    #[doc = "0x58 - 7st quantization for jpeg chroma table"]
    #[inline(always)]
    pub const fn swreg_22(&self) -> &Swreg22 {
        &self.swreg_22
    }
    #[doc = "0x5c - 8st quantization for jpeg chroma table/part 3 for qp round"]
    #[inline(always)]
    pub const fn swreg_23(&self) -> &Swreg23 {
        &self.swreg_23
    }
    #[doc = "0x60 - 9st quantization for jpeg chroma table"]
    #[inline(always)]
    pub const fn swreg_24(&self) -> &Swreg24 {
        &self.swreg_24
    }
    #[doc = "0x64 - 10st quantization for jpeg chroma table"]
    #[inline(always)]
    pub const fn swreg_25(&self) -> &Swreg25 {
        &self.swreg_25
    }
    #[doc = "0x68 - 11st quantization for jpeg chroma table"]
    #[inline(always)]
    pub const fn swreg_26(&self) -> &Swreg26 {
        &self.swreg_26
    }
    #[doc = "0x6c - 12st quantization for jpeg chroma"]
    #[inline(always)]
    pub const fn swreg_27(&self) -> &Swreg27 {
        &self.swreg_27
    }
    #[doc = "0x70 - 13st quantization for jpeg chroma"]
    #[inline(always)]
    pub const fn swreg_28(&self) -> &Swreg28 {
        &self.swreg_28
    }
    #[doc = "0x74 - 14st quantization for jpeg chroma"]
    #[inline(always)]
    pub const fn swreg_29(&self) -> &Swreg29 {
        &self.swreg_29
    }
    #[doc = "0x78 - 15st quantization for jpeg chroma"]
    #[inline(always)]
    pub const fn swreg_30(&self) -> &Swreg30 {
        &self.swreg_30
    }
    #[doc = "0x7c - 16st quantization for jpeg chroma"]
    #[inline(always)]
    pub const fn swreg_31(&self) -> &Swreg31 {
        &self.swreg_31
    }
    #[doc = "0xb0 - Intra slice bitmap"]
    #[inline(always)]
    pub const fn swreg_44(&self) -> &Swreg44 {
        &self.swreg_44
    }
    #[doc = "0xb4 - Intra slice bitmap"]
    #[inline(always)]
    pub const fn swreg_45(&self) -> &Swreg45 {
        &self.swreg_45
    }
    #[doc = "0xb8 - intra macro block sellect register"]
    #[inline(always)]
    pub const fn swreg_46(&self) -> &Swreg46 {
        &self.swreg_46
    }
    #[doc = "0xbc - CIR intra control register"]
    #[inline(always)]
    pub const fn swreg_47(&self) -> &Swreg47 {
        &self.swreg_47
    }
    #[doc = "0xc0 - input luma start address"]
    #[inline(always)]
    pub const fn swreg_48(&self) -> &Swreg48 {
        &self.swreg_48
    }
    #[doc = "0xc4 - input cb start address"]
    #[inline(always)]
    pub const fn swreg_49(&self) -> &Swreg49 {
        &self.swreg_49
    }
    #[doc = "0xc8 - input cr start address"]
    #[inline(always)]
    pub const fn swreg_50(&self) -> &Swreg50 {
        &self.swreg_50
    }
    #[doc = "0xcc - stream header bits left register"]
    #[inline(always)]
    pub const fn swreg_51(&self) -> &Swreg51 {
        &self.swreg_51
    }
    #[doc = "0xd0 - stream header bits left register"]
    #[inline(always)]
    pub const fn swreg_52(&self) -> &Swreg52 {
        &self.swreg_52
    }
    #[doc = "0xd4 - stream buffer register"]
    #[inline(always)]
    pub const fn swreg_53(&self) -> &Swreg53 {
        &self.swreg_53
    }
    #[doc = "0xd8 - axi control register"]
    #[inline(always)]
    pub const fn swreg_54(&self) -> &Swreg54 {
        &self.swreg_54
    }
    #[doc = "0xdc - qp related"]
    #[inline(always)]
    pub const fn swreg_55(&self) -> &Swreg55 {
        &self.swreg_55
    }
    #[doc = "0xe0 - the luma reference frame start address"]
    #[inline(always)]
    pub const fn swreg_56(&self) -> &Swreg56 {
        &self.swreg_56
    }
    #[doc = "0xe4 - the chroma reference frame start address"]
    #[inline(always)]
    pub const fn swreg_57(&self) -> &Swreg57 {
        &self.swreg_57
    }
    #[doc = "0xe8 - the result of qp sum div2"]
    #[inline(always)]
    pub const fn swreg_58(&self) -> &Swreg58 {
        &self.swreg_58
    }
    #[doc = "0xec - Register0000 Abstract"]
    #[inline(always)]
    pub const fn swreg_59(&self) -> &Swreg59 {
        &self.swreg_59
    }
    #[doc = "0xf0 - Register0001 Abstract"]
    #[inline(always)]
    pub const fn swreg_60(&self) -> &Swreg60 {
        &self.swreg_60
    }
    #[doc = "0xf4 - input luminance information"]
    #[inline(always)]
    pub const fn swreg_61(&self) -> &Swreg61 {
        &self.swreg_61
    }
    #[doc = "0xf8 - rlc_sum"]
    #[inline(always)]
    pub const fn swreg_62(&self) -> &Swreg62 {
        &self.swreg_62
    }
    #[doc = "0xfc - the reconstructed luma start address"]
    #[inline(always)]
    pub const fn swreg_63(&self) -> &Swreg63 {
        &self.swreg_63
    }
    #[doc = "0x100 - the reconstructed chroma start address"]
    #[inline(always)]
    pub const fn swreg_64(&self) -> &Swreg64 {
        &self.swreg_64
    }
    #[doc = "0x104 - checkpoint 1 and 2"]
    #[inline(always)]
    pub const fn swreg_65_reuse(&self) -> &Swreg65Reuse {
        &self.swreg_65_reuse
    }
    #[doc = "0x108 - checkpoint 3 and 4"]
    #[inline(always)]
    pub const fn swreg_66_reuse(&self) -> &Swreg66Reuse {
        &self.swreg_66_reuse
    }
    #[doc = "0x10c - checkpoint 5 and 6"]
    #[inline(always)]
    pub const fn swreg_67_reuse(&self) -> &Swreg67Reuse {
        &self.swreg_67_reuse
    }
    #[doc = "0x110 - checkpoint 7 and 8"]
    #[inline(always)]
    pub const fn swreg_68_reuse(&self) -> &Swreg68Reuse {
        &self.swreg_68_reuse
    }
    #[doc = "0x114 - checkpoint 9 and 10"]
    #[inline(always)]
    pub const fn swreg_69_reuse(&self) -> &Swreg69Reuse {
        &self.swreg_69_reuse
    }
    #[doc = "0x118 - checkpoint word error 1 and 2"]
    #[inline(always)]
    pub const fn swreg_70_reuse(&self) -> &Swreg70Reuse {
        &self.swreg_70_reuse
    }
    #[doc = "0x11c - checkpoint word error 1 and 2"]
    #[inline(always)]
    pub const fn swreg_71_reuse(&self) -> &Swreg71Reuse {
        &self.swreg_71_reuse
    }
    #[doc = "0x120 - checkpoint word error 1 and 2"]
    #[inline(always)]
    pub const fn swreg_72_reuse(&self) -> &Swreg72Reuse {
        &self.swreg_72_reuse
    }
    #[doc = "0x124 - checkpoint delta QP register"]
    #[inline(always)]
    pub const fn swreg_73_reuse(&self) -> &Swreg73Reuse {
        &self.swreg_73_reuse
    }
    #[doc = "0x128 - input image format"]
    #[inline(always)]
    pub const fn swreg_74(&self) -> &Swreg74 {
        &self.swreg_74
    }
    #[doc = "0x12c - intra/inter mode"]
    #[inline(always)]
    pub const fn swreg_75(&self) -> &Swreg75 {
        &self.swreg_75
    }
    #[doc = "0x130 - encoder control regsiter 0"]
    #[inline(always)]
    pub const fn swreg_76_reuse(&self) -> &Swreg76Reuse {
        &self.swreg_76_reuse
    }
    #[doc = "0x134 - output stream start address"]
    #[inline(always)]
    pub const fn swreg_77(&self) -> &Swreg77 {
        &self.swreg_77
    }
    #[doc = "0x138 - output control start address"]
    #[inline(always)]
    pub const fn swreg_78(&self) -> &Swreg78 {
        &self.swreg_78
    }
    #[doc = "0x13c - next picture luminance start address"]
    #[inline(always)]
    pub const fn swreg_79(&self) -> &Swreg79 {
        &self.swreg_79
    }
    #[doc = "0x140 - Base address for MV output"]
    #[inline(always)]
    pub const fn swreg_80(&self) -> &Swreg80 {
        &self.swreg_80
    }
    #[doc = "0x144 - the cabac table start address"]
    #[inline(always)]
    pub const fn swreg_81(&self) -> &Swreg81 {
        &self.swreg_81
    }
    #[doc = "0x148 - the first of ROI area register"]
    #[inline(always)]
    pub const fn swreg_82(&self) -> &Swreg82 {
        &self.swreg_82
    }
    #[doc = "0x14c - the second of ROI area register"]
    #[inline(always)]
    pub const fn swreg_83(&self) -> &Swreg83 {
        &self.swreg_83
    }
    #[doc = "0x150 - Stabilization matrix"]
    #[inline(always)]
    pub const fn swreg_84(&self) -> &Swreg84 {
        &self.swreg_84
    }
    #[doc = "0x154 - Stabilization matrix"]
    #[inline(always)]
    pub const fn swreg_85(&self) -> &Swreg85 {
        &self.swreg_85
    }
    #[doc = "0x158 - Stabilization matrix"]
    #[inline(always)]
    pub const fn swreg_86(&self) -> &Swreg86 {
        &self.swreg_86
    }
    #[doc = "0x15c - Stabilization matrix"]
    #[inline(always)]
    pub const fn swreg_87(&self) -> &Swreg87 {
        &self.swreg_87
    }
    #[doc = "0x160 - Stabilization matrix"]
    #[inline(always)]
    pub const fn swreg_88(&self) -> &Swreg88 {
        &self.swreg_88
    }
    #[doc = "0x164 - Stabilization matrix"]
    #[inline(always)]
    pub const fn swreg_89(&self) -> &Swreg89 {
        &self.swreg_89
    }
    #[doc = "0x168 - Stabilization matrix"]
    #[inline(always)]
    pub const fn swreg_90(&self) -> &Swreg90 {
        &self.swreg_90
    }
    #[doc = "0x16c - Stabilization matrix"]
    #[inline(always)]
    pub const fn swreg_91(&self) -> &Swreg91 {
        &self.swreg_91
    }
    #[doc = "0x170 - Stabilization matrix"]
    #[inline(always)]
    pub const fn swreg_92(&self) -> &Swreg92 {
        &self.swreg_92
    }
    #[doc = "0x174 - the output of Stabilization motion sum"]
    #[inline(always)]
    pub const fn swreg_93(&self) -> &Swreg93 {
        &self.swreg_93
    }
    #[doc = "0x178 - output of Stabilization"]
    #[inline(always)]
    pub const fn swreg_94(&self) -> &Swreg94 {
        &self.swreg_94
    }
    #[doc = "0x17c - RGB to YUV conversion coefficient register"]
    #[inline(always)]
    pub const fn swreg_95(&self) -> &Swreg95 {
        &self.swreg_95
    }
    #[doc = "0x180 - RGB to YUV conversion coefficient register"]
    #[inline(always)]
    pub const fn swreg_96(&self) -> &Swreg96 {
        &self.swreg_96
    }
    #[doc = "0x184 - RGB to YUV conversion coefficient register"]
    #[inline(always)]
    pub const fn swreg_97(&self) -> &Swreg97 {
        &self.swreg_97
    }
    #[doc = "0x188 - RGA MASK"]
    #[inline(always)]
    pub const fn swreg_98(&self) -> &Swreg98 {
        &self.swreg_98
    }
    #[doc = "0x18c - mv related"]
    #[inline(always)]
    pub const fn swreg_99(&self) -> &Swreg99 {
        &self.swreg_99
    }
    #[doc = "0x190 - QP register"]
    #[inline(always)]
    pub const fn swreg_100_reuse(&self) -> &Swreg100Reuse {
        &self.swreg_100_reuse
    }
    #[doc = "0x194 - hw config reg"]
    #[inline(always)]
    pub const fn swreg_101_read(&self) -> &Swreg101Read {
        &self.swreg_101_read
    }
    #[doc = "0x198 - mvc related"]
    #[inline(always)]
    pub const fn swreg_102(&self) -> &Swreg102 {
        &self.swreg_102
    }
    #[doc = "0x19c - encoder start"]
    #[inline(always)]
    pub const fn swreg_103(&self) -> &Swreg103 {
        &self.swreg_103
    }
    #[doc = "0x1a0 - mb control register"]
    #[inline(always)]
    pub const fn swreg_104(&self) -> &Swreg104 {
        &self.swreg_104
    }
    #[doc = "0x1a4 - SWAP"]
    #[inline(always)]
    pub const fn swreg_105(&self) -> &Swreg105 {
        &self.swreg_105
    }
    #[doc = "0x1a8 - encoder control register 1"]
    #[inline(always)]
    pub const fn swreg_106_reuse(&self) -> &Swreg106Reuse {
        &self.swreg_106_reuse
    }
    #[doc = "0x1ac - JPEG control regsiter"]
    #[inline(always)]
    pub const fn swreg_107_reuse(&self) -> &Swreg107Reuse {
        &self.swreg_107_reuse
    }
    #[doc = "0x1b0 - intra_slice_bmp2"]
    #[inline(always)]
    pub const fn swreg_108_reuse(&self) -> &Swreg108Reuse {
        &self.swreg_108_reuse
    }
    #[doc = "0x1b4 - encoder status"]
    #[inline(always)]
    pub const fn swreg_109(&self) -> &Swreg109 {
        &self.swreg_109
    }
    #[doc = "0x1b8 - product ID"]
    #[inline(always)]
    pub const fn swreg_110_read(&self) -> &Swreg110Read {
        &self.swreg_110_read
    }
    #[doc = "0x1e0 - DMV_4p_1p_penalty"]
    #[inline(always)]
    pub const fn swreg_120_183(&self) -> &Swreg120_183 {
        &self.swreg_120_183
    }
}
#[doc = "SWREG_0 (rw) register accessor: 1st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_0`]
module"]
#[doc(alias = "SWREG_0")]
pub type Swreg0 = crate::Reg<swreg_0::Swreg0Spec>;
#[doc = "1st quantization for jpeg lumin table"]
pub mod swreg_0;
#[doc = "SWREG_1 (rw) register accessor: 2st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_1`]
module"]
#[doc(alias = "SWREG_1")]
pub type Swreg1 = crate::Reg<swreg_1::Swreg1Spec>;
#[doc = "2st quantization for jpeg lumin table"]
pub mod swreg_1;
#[doc = "SWREG_2 (rw) register accessor: 3st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_2`]
module"]
#[doc(alias = "SWREG_2")]
pub type Swreg2 = crate::Reg<swreg_2::Swreg2Spec>;
#[doc = "3st quantization for jpeg lumin table"]
pub mod swreg_2;
#[doc = "SWREG_3 (rw) register accessor: 4st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_3`]
module"]
#[doc(alias = "SWREG_3")]
pub type Swreg3 = crate::Reg<swreg_3::Swreg3Spec>;
#[doc = "4st quantization for jpeg lumin table"]
pub mod swreg_3;
#[doc = "SWREG_4 (rw) register accessor: 5st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_4`]
module"]
#[doc(alias = "SWREG_4")]
pub type Swreg4 = crate::Reg<swreg_4::Swreg4Spec>;
#[doc = "5st quantization for jpeg lumin table"]
pub mod swreg_4;
#[doc = "SWREG_5 (rw) register accessor: 6st quantization for jpeg lumin table/part 1 for qp round\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_5`]
module"]
#[doc(alias = "SWREG_5")]
pub type Swreg5 = crate::Reg<swreg_5::Swreg5Spec>;
#[doc = "6st quantization for jpeg lumin table/part 1 for qp round"]
pub mod swreg_5;
#[doc = "SWREG_6 (rw) register accessor: 7st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_6`]
module"]
#[doc(alias = "SWREG_6")]
pub type Swreg6 = crate::Reg<swreg_6::Swreg6Spec>;
#[doc = "7st quantization for jpeg lumin table"]
pub mod swreg_6;
#[doc = "SWREG_7 (rw) register accessor: 8st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_7`]
module"]
#[doc(alias = "SWREG_7")]
pub type Swreg7 = crate::Reg<swreg_7::Swreg7Spec>;
#[doc = "8st quantization for jpeg lumin table"]
pub mod swreg_7;
#[doc = "SWREG_8 (rw) register accessor: 9st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_8`]
module"]
#[doc(alias = "SWREG_8")]
pub type Swreg8 = crate::Reg<swreg_8::Swreg8Spec>;
#[doc = "9st quantization for jpeg lumin table"]
pub mod swreg_8;
#[doc = "SWREG_9 (rw) register accessor: 10st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_9`]
module"]
#[doc(alias = "SWREG_9")]
pub type Swreg9 = crate::Reg<swreg_9::Swreg9Spec>;
#[doc = "10st quantization for jpeg lumin table"]
pub mod swreg_9;
#[doc = "SWREG_10 (rw) register accessor: 11st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_10`]
module"]
#[doc(alias = "SWREG_10")]
pub type Swreg10 = crate::Reg<swreg_10::Swreg10Spec>;
#[doc = "11st quantization for jpeg lumin table"]
pub mod swreg_10;
#[doc = "SWREG_11 (rw) register accessor: 12st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_11`]
module"]
#[doc(alias = "SWREG_11")]
pub type Swreg11 = crate::Reg<swreg_11::Swreg11Spec>;
#[doc = "12st quantization for jpeg lumin table"]
pub mod swreg_11;
#[doc = "SWREG_12 (rw) register accessor: 13st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_12`]
module"]
#[doc(alias = "SWREG_12")]
pub type Swreg12 = crate::Reg<swreg_12::Swreg12Spec>;
#[doc = "13st quantization for jpeg lumin table"]
pub mod swreg_12;
#[doc = "SWREG_13 (rw) register accessor: 14st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_13`]
module"]
#[doc(alias = "SWREG_13")]
pub type Swreg13 = crate::Reg<swreg_13::Swreg13Spec>;
#[doc = "14st quantization for jpeg lumin table"]
pub mod swreg_13;
#[doc = "SWREG_14 (rw) register accessor: 15st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_14`]
module"]
#[doc(alias = "SWREG_14")]
pub type Swreg14 = crate::Reg<swreg_14::Swreg14Spec>;
#[doc = "15st quantization for jpeg lumin table"]
pub mod swreg_14;
#[doc = "SWREG_15 (rw) register accessor: 16st quantization for jpeg lumin table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_15`]
module"]
#[doc(alias = "SWREG_15")]
pub type Swreg15 = crate::Reg<swreg_15::Swreg15Spec>;
#[doc = "16st quantization for jpeg lumin table"]
pub mod swreg_15;
#[doc = "SWREG_16 (rw) register accessor: 1st quantization for jpeg chroma table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_16`]
module"]
#[doc(alias = "SWREG_16")]
pub type Swreg16 = crate::Reg<swreg_16::Swreg16Spec>;
#[doc = "1st quantization for jpeg chroma table"]
pub mod swreg_16;
#[doc = "SWREG_17 (rw) register accessor: 2st quantization for jpeg chroma table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_17`]
module"]
#[doc(alias = "SWREG_17")]
pub type Swreg17 = crate::Reg<swreg_17::Swreg17Spec>;
#[doc = "2st quantization for jpeg chroma table"]
pub mod swreg_17;
#[doc = "SWREG_18 (rw) register accessor: 3st quantization for jpeg chroma table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_18`]
module"]
#[doc(alias = "SWREG_18")]
pub type Swreg18 = crate::Reg<swreg_18::Swreg18Spec>;
#[doc = "3st quantization for jpeg chroma table"]
pub mod swreg_18;
#[doc = "SWREG_19 (rw) register accessor: 4st quantization for jpeg chroma table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_19`]
module"]
#[doc(alias = "SWREG_19")]
pub type Swreg19 = crate::Reg<swreg_19::Swreg19Spec>;
#[doc = "4st quantization for jpeg chroma table"]
pub mod swreg_19;
#[doc = "SWREG_20 (rw) register accessor: 5st quantization for jpeg chroma table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_20`]
module"]
#[doc(alias = "SWREG_20")]
pub type Swreg20 = crate::Reg<swreg_20::Swreg20Spec>;
#[doc = "5st quantization for jpeg chroma table"]
pub mod swreg_20;
#[doc = "SWREG_21 (rw) register accessor: 6st quantization for jpeg chroma table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_21`]
module"]
#[doc(alias = "SWREG_21")]
pub type Swreg21 = crate::Reg<swreg_21::Swreg21Spec>;
#[doc = "6st quantization for jpeg chroma table"]
pub mod swreg_21;
#[doc = "SWREG_22 (rw) register accessor: 7st quantization for jpeg chroma table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_22`]
module"]
#[doc(alias = "SWREG_22")]
pub type Swreg22 = crate::Reg<swreg_22::Swreg22Spec>;
#[doc = "7st quantization for jpeg chroma table"]
pub mod swreg_22;
#[doc = "SWREG_23 (rw) register accessor: 8st quantization for jpeg chroma table/part 3 for qp round\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_23`]
module"]
#[doc(alias = "SWREG_23")]
pub type Swreg23 = crate::Reg<swreg_23::Swreg23Spec>;
#[doc = "8st quantization for jpeg chroma table/part 3 for qp round"]
pub mod swreg_23;
#[doc = "SWREG_24 (rw) register accessor: 9st quantization for jpeg chroma table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_24`]
module"]
#[doc(alias = "SWREG_24")]
pub type Swreg24 = crate::Reg<swreg_24::Swreg24Spec>;
#[doc = "9st quantization for jpeg chroma table"]
pub mod swreg_24;
#[doc = "SWREG_25 (rw) register accessor: 10st quantization for jpeg chroma table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_25`]
module"]
#[doc(alias = "SWREG_25")]
pub type Swreg25 = crate::Reg<swreg_25::Swreg25Spec>;
#[doc = "10st quantization for jpeg chroma table"]
pub mod swreg_25;
#[doc = "SWREG_26 (rw) register accessor: 11st quantization for jpeg chroma table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_26`]
module"]
#[doc(alias = "SWREG_26")]
pub type Swreg26 = crate::Reg<swreg_26::Swreg26Spec>;
#[doc = "11st quantization for jpeg chroma table"]
pub mod swreg_26;
#[doc = "SWREG_27 (rw) register accessor: 12st quantization for jpeg chroma\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_27`]
module"]
#[doc(alias = "SWREG_27")]
pub type Swreg27 = crate::Reg<swreg_27::Swreg27Spec>;
#[doc = "12st quantization for jpeg chroma"]
pub mod swreg_27;
#[doc = "SWREG_28 (rw) register accessor: 13st quantization for jpeg chroma\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_28`]
module"]
#[doc(alias = "SWREG_28")]
pub type Swreg28 = crate::Reg<swreg_28::Swreg28Spec>;
#[doc = "13st quantization for jpeg chroma"]
pub mod swreg_28;
#[doc = "SWREG_29 (rw) register accessor: 14st quantization for jpeg chroma\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_29`]
module"]
#[doc(alias = "SWREG_29")]
pub type Swreg29 = crate::Reg<swreg_29::Swreg29Spec>;
#[doc = "14st quantization for jpeg chroma"]
pub mod swreg_29;
#[doc = "SWREG_30 (rw) register accessor: 15st quantization for jpeg chroma\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_30`]
module"]
#[doc(alias = "SWREG_30")]
pub type Swreg30 = crate::Reg<swreg_30::Swreg30Spec>;
#[doc = "15st quantization for jpeg chroma"]
pub mod swreg_30;
#[doc = "SWREG_31 (rw) register accessor: 16st quantization for jpeg chroma\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_31`]
module"]
#[doc(alias = "SWREG_31")]
pub type Swreg31 = crate::Reg<swreg_31::Swreg31Spec>;
#[doc = "16st quantization for jpeg chroma"]
pub mod swreg_31;
#[doc = "SWREG_44 (rw) register accessor: Intra slice bitmap\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_44`]
module"]
#[doc(alias = "SWREG_44")]
pub type Swreg44 = crate::Reg<swreg_44::Swreg44Spec>;
#[doc = "Intra slice bitmap"]
pub mod swreg_44;
#[doc = "SWREG_45 (rw) register accessor: Intra slice bitmap\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_45`]
module"]
#[doc(alias = "SWREG_45")]
pub type Swreg45 = crate::Reg<swreg_45::Swreg45Spec>;
#[doc = "Intra slice bitmap"]
pub mod swreg_45;
#[doc = "SWREG_46 (rw) register accessor: intra macro block sellect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_46`]
module"]
#[doc(alias = "SWREG_46")]
pub type Swreg46 = crate::Reg<swreg_46::Swreg46Spec>;
#[doc = "intra macro block sellect register"]
pub mod swreg_46;
#[doc = "SWREG_47 (rw) register accessor: CIR intra control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_47`]
module"]
#[doc(alias = "SWREG_47")]
pub type Swreg47 = crate::Reg<swreg_47::Swreg47Spec>;
#[doc = "CIR intra control register"]
pub mod swreg_47;
#[doc = "SWREG_48 (rw) register accessor: input luma start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_48`]
module"]
#[doc(alias = "SWREG_48")]
pub type Swreg48 = crate::Reg<swreg_48::Swreg48Spec>;
#[doc = "input luma start address"]
pub mod swreg_48;
#[doc = "SWREG_49 (rw) register accessor: input cb start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_49`]
module"]
#[doc(alias = "SWREG_49")]
pub type Swreg49 = crate::Reg<swreg_49::Swreg49Spec>;
#[doc = "input cb start address"]
pub mod swreg_49;
#[doc = "SWREG_50 (rw) register accessor: input cr start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_50::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_50`]
module"]
#[doc(alias = "SWREG_50")]
pub type Swreg50 = crate::Reg<swreg_50::Swreg50Spec>;
#[doc = "input cr start address"]
pub mod swreg_50;
#[doc = "SWREG_51 (rw) register accessor: stream header bits left register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_51`]
module"]
#[doc(alias = "SWREG_51")]
pub type Swreg51 = crate::Reg<swreg_51::Swreg51Spec>;
#[doc = "stream header bits left register"]
pub mod swreg_51;
#[doc = "SWREG_52 (rw) register accessor: stream header bits left register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_52`]
module"]
#[doc(alias = "SWREG_52")]
pub type Swreg52 = crate::Reg<swreg_52::Swreg52Spec>;
#[doc = "stream header bits left register"]
pub mod swreg_52;
#[doc = "SWREG_53 (rw) register accessor: stream buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_53`]
module"]
#[doc(alias = "SWREG_53")]
pub type Swreg53 = crate::Reg<swreg_53::Swreg53Spec>;
#[doc = "stream buffer register"]
pub mod swreg_53;
#[doc = "SWREG_54 (rw) register accessor: axi control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_54::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_54`]
module"]
#[doc(alias = "SWREG_54")]
pub type Swreg54 = crate::Reg<swreg_54::Swreg54Spec>;
#[doc = "axi control register"]
pub mod swreg_54;
#[doc = "SWREG_55 (rw) register accessor: qp related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_55`]
module"]
#[doc(alias = "SWREG_55")]
pub type Swreg55 = crate::Reg<swreg_55::Swreg55Spec>;
#[doc = "qp related"]
pub mod swreg_55;
#[doc = "SWREG_56 (rw) register accessor: the luma reference frame start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_56::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_56`]
module"]
#[doc(alias = "SWREG_56")]
pub type Swreg56 = crate::Reg<swreg_56::Swreg56Spec>;
#[doc = "the luma reference frame start address"]
pub mod swreg_56;
#[doc = "SWREG_57 (rw) register accessor: the chroma reference frame start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_57::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_57::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_57`]
module"]
#[doc(alias = "SWREG_57")]
pub type Swreg57 = crate::Reg<swreg_57::Swreg57Spec>;
#[doc = "the chroma reference frame start address"]
pub mod swreg_57;
#[doc = "SWREG_58 (rw) register accessor: the result of qp sum div2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_58::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_58::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_58`]
module"]
#[doc(alias = "SWREG_58")]
pub type Swreg58 = crate::Reg<swreg_58::Swreg58Spec>;
#[doc = "the result of qp sum div2"]
pub mod swreg_58;
#[doc = "SWREG_59 (rw) register accessor: Register0000 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_59::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_59`]
module"]
#[doc(alias = "SWREG_59")]
pub type Swreg59 = crate::Reg<swreg_59::Swreg59Spec>;
#[doc = "Register0000 Abstract"]
pub mod swreg_59;
#[doc = "SWREG_60 (rw) register accessor: Register0001 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_60::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_60`]
module"]
#[doc(alias = "SWREG_60")]
pub type Swreg60 = crate::Reg<swreg_60::Swreg60Spec>;
#[doc = "Register0001 Abstract"]
pub mod swreg_60;
#[doc = "SWREG_61 (rw) register accessor: input luminance information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_61::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_61::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_61`]
module"]
#[doc(alias = "SWREG_61")]
pub type Swreg61 = crate::Reg<swreg_61::Swreg61Spec>;
#[doc = "input luminance information"]
pub mod swreg_61;
#[doc = "SWREG_62 (rw) register accessor: rlc_sum\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_62::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_62::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_62`]
module"]
#[doc(alias = "SWREG_62")]
pub type Swreg62 = crate::Reg<swreg_62::Swreg62Spec>;
#[doc = "rlc_sum"]
pub mod swreg_62;
#[doc = "SWREG_63 (rw) register accessor: the reconstructed luma start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_63::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_63`]
module"]
#[doc(alias = "SWREG_63")]
pub type Swreg63 = crate::Reg<swreg_63::Swreg63Spec>;
#[doc = "the reconstructed luma start address"]
pub mod swreg_63;
#[doc = "SWREG_64 (rw) register accessor: the reconstructed chroma start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_64`]
module"]
#[doc(alias = "SWREG_64")]
pub type Swreg64 = crate::Reg<swreg_64::Swreg64Spec>;
#[doc = "the reconstructed chroma start address"]
pub mod swreg_64;
#[doc = "SWREG_65_REUSE (rw) register accessor: checkpoint 1 and 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_65_reuse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_65_reuse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_65_reuse`]
module"]
#[doc(alias = "SWREG_65_REUSE")]
pub type Swreg65Reuse = crate::Reg<swreg_65_reuse::Swreg65ReuseSpec>;
#[doc = "checkpoint 1 and 2"]
pub mod swreg_65_reuse;
#[doc = "SWREG_66_REUSE (rw) register accessor: checkpoint 3 and 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_66_reuse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_66_reuse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_66_reuse`]
module"]
#[doc(alias = "SWREG_66_REUSE")]
pub type Swreg66Reuse = crate::Reg<swreg_66_reuse::Swreg66ReuseSpec>;
#[doc = "checkpoint 3 and 4"]
pub mod swreg_66_reuse;
#[doc = "SWREG_67_REUSE (rw) register accessor: checkpoint 5 and 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_67_reuse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_67_reuse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_67_reuse`]
module"]
#[doc(alias = "SWREG_67_REUSE")]
pub type Swreg67Reuse = crate::Reg<swreg_67_reuse::Swreg67ReuseSpec>;
#[doc = "checkpoint 5 and 6"]
pub mod swreg_67_reuse;
#[doc = "SWREG_68_REUSE (rw) register accessor: checkpoint 7 and 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_68_reuse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_68_reuse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_68_reuse`]
module"]
#[doc(alias = "SWREG_68_REUSE")]
pub type Swreg68Reuse = crate::Reg<swreg_68_reuse::Swreg68ReuseSpec>;
#[doc = "checkpoint 7 and 8"]
pub mod swreg_68_reuse;
#[doc = "SWREG_69_REUSE (rw) register accessor: checkpoint 9 and 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_69_reuse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_69_reuse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_69_reuse`]
module"]
#[doc(alias = "SWREG_69_REUSE")]
pub type Swreg69Reuse = crate::Reg<swreg_69_reuse::Swreg69ReuseSpec>;
#[doc = "checkpoint 9 and 10"]
pub mod swreg_69_reuse;
#[doc = "SWREG_70_REUSE (rw) register accessor: checkpoint word error 1 and 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_70_reuse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_70_reuse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_70_reuse`]
module"]
#[doc(alias = "SWREG_70_REUSE")]
pub type Swreg70Reuse = crate::Reg<swreg_70_reuse::Swreg70ReuseSpec>;
#[doc = "checkpoint word error 1 and 2"]
pub mod swreg_70_reuse;
#[doc = "SWREG_71_REUSE (rw) register accessor: checkpoint word error 1 and 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_71_reuse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_71_reuse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_71_reuse`]
module"]
#[doc(alias = "SWREG_71_REUSE")]
pub type Swreg71Reuse = crate::Reg<swreg_71_reuse::Swreg71ReuseSpec>;
#[doc = "checkpoint word error 1 and 2"]
pub mod swreg_71_reuse;
#[doc = "SWREG_72_REUSE (rw) register accessor: checkpoint word error 1 and 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_72_reuse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_72_reuse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_72_reuse`]
module"]
#[doc(alias = "SWREG_72_REUSE")]
pub type Swreg72Reuse = crate::Reg<swreg_72_reuse::Swreg72ReuseSpec>;
#[doc = "checkpoint word error 1 and 2"]
pub mod swreg_72_reuse;
#[doc = "SWREG_73_REUSE (rw) register accessor: checkpoint delta QP register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_73_reuse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_73_reuse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_73_reuse`]
module"]
#[doc(alias = "SWREG_73_REUSE")]
pub type Swreg73Reuse = crate::Reg<swreg_73_reuse::Swreg73ReuseSpec>;
#[doc = "checkpoint delta QP register"]
pub mod swreg_73_reuse;
#[doc = "SWREG_74 (rw) register accessor: input image format\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_74::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_74::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_74`]
module"]
#[doc(alias = "SWREG_74")]
pub type Swreg74 = crate::Reg<swreg_74::Swreg74Spec>;
#[doc = "input image format"]
pub mod swreg_74;
#[doc = "SWREG_75 (rw) register accessor: intra/inter mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_75::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_75::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_75`]
module"]
#[doc(alias = "SWREG_75")]
pub type Swreg75 = crate::Reg<swreg_75::Swreg75Spec>;
#[doc = "intra/inter mode"]
pub mod swreg_75;
#[doc = "SWREG_76_REUSE (rw) register accessor: encoder control regsiter 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_76_reuse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_76_reuse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_76_reuse`]
module"]
#[doc(alias = "SWREG_76_REUSE")]
pub type Swreg76Reuse = crate::Reg<swreg_76_reuse::Swreg76ReuseSpec>;
#[doc = "encoder control regsiter 0"]
pub mod swreg_76_reuse;
#[doc = "SWREG_77 (rw) register accessor: output stream start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_77::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_77::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_77`]
module"]
#[doc(alias = "SWREG_77")]
pub type Swreg77 = crate::Reg<swreg_77::Swreg77Spec>;
#[doc = "output stream start address"]
pub mod swreg_77;
#[doc = "SWREG_78 (rw) register accessor: output control start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_78::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_78::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_78`]
module"]
#[doc(alias = "SWREG_78")]
pub type Swreg78 = crate::Reg<swreg_78::Swreg78Spec>;
#[doc = "output control start address"]
pub mod swreg_78;
#[doc = "SWREG_79 (rw) register accessor: next picture luminance start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_79::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_79::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_79`]
module"]
#[doc(alias = "SWREG_79")]
pub type Swreg79 = crate::Reg<swreg_79::Swreg79Spec>;
#[doc = "next picture luminance start address"]
pub mod swreg_79;
#[doc = "SWREG_80 (rw) register accessor: Base address for MV output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_80::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_80::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_80`]
module"]
#[doc(alias = "SWREG_80")]
pub type Swreg80 = crate::Reg<swreg_80::Swreg80Spec>;
#[doc = "Base address for MV output"]
pub mod swreg_80;
#[doc = "SWREG_81 (rw) register accessor: the cabac table start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_81::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_81::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_81`]
module"]
#[doc(alias = "SWREG_81")]
pub type Swreg81 = crate::Reg<swreg_81::Swreg81Spec>;
#[doc = "the cabac table start address"]
pub mod swreg_81;
#[doc = "SWREG_82 (rw) register accessor: the first of ROI area register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_82::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_82::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_82`]
module"]
#[doc(alias = "SWREG_82")]
pub type Swreg82 = crate::Reg<swreg_82::Swreg82Spec>;
#[doc = "the first of ROI area register"]
pub mod swreg_82;
#[doc = "SWREG_83 (rw) register accessor: the second of ROI area register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_83::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_83::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_83`]
module"]
#[doc(alias = "SWREG_83")]
pub type Swreg83 = crate::Reg<swreg_83::Swreg83Spec>;
#[doc = "the second of ROI area register"]
pub mod swreg_83;
#[doc = "SWREG_84 (rw) register accessor: Stabilization matrix\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_84::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_84::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_84`]
module"]
#[doc(alias = "SWREG_84")]
pub type Swreg84 = crate::Reg<swreg_84::Swreg84Spec>;
#[doc = "Stabilization matrix"]
pub mod swreg_84;
#[doc = "SWREG_85 (rw) register accessor: Stabilization matrix\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_85::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_85::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_85`]
module"]
#[doc(alias = "SWREG_85")]
pub type Swreg85 = crate::Reg<swreg_85::Swreg85Spec>;
#[doc = "Stabilization matrix"]
pub mod swreg_85;
#[doc = "SWREG_86 (rw) register accessor: Stabilization matrix\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_86::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_86::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_86`]
module"]
#[doc(alias = "SWREG_86")]
pub type Swreg86 = crate::Reg<swreg_86::Swreg86Spec>;
#[doc = "Stabilization matrix"]
pub mod swreg_86;
#[doc = "SWREG_87 (rw) register accessor: Stabilization matrix\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_87::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_87::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_87`]
module"]
#[doc(alias = "SWREG_87")]
pub type Swreg87 = crate::Reg<swreg_87::Swreg87Spec>;
#[doc = "Stabilization matrix"]
pub mod swreg_87;
#[doc = "SWREG_88 (rw) register accessor: Stabilization matrix\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_88::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_88::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_88`]
module"]
#[doc(alias = "SWREG_88")]
pub type Swreg88 = crate::Reg<swreg_88::Swreg88Spec>;
#[doc = "Stabilization matrix"]
pub mod swreg_88;
#[doc = "SWREG_89 (rw) register accessor: Stabilization matrix\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_89::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_89::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_89`]
module"]
#[doc(alias = "SWREG_89")]
pub type Swreg89 = crate::Reg<swreg_89::Swreg89Spec>;
#[doc = "Stabilization matrix"]
pub mod swreg_89;
#[doc = "SWREG_90 (rw) register accessor: Stabilization matrix\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_90::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_90::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_90`]
module"]
#[doc(alias = "SWREG_90")]
pub type Swreg90 = crate::Reg<swreg_90::Swreg90Spec>;
#[doc = "Stabilization matrix"]
pub mod swreg_90;
#[doc = "SWREG_91 (rw) register accessor: Stabilization matrix\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_91::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_91::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_91`]
module"]
#[doc(alias = "SWREG_91")]
pub type Swreg91 = crate::Reg<swreg_91::Swreg91Spec>;
#[doc = "Stabilization matrix"]
pub mod swreg_91;
#[doc = "SWREG_92 (rw) register accessor: Stabilization matrix\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_92::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_92::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_92`]
module"]
#[doc(alias = "SWREG_92")]
pub type Swreg92 = crate::Reg<swreg_92::Swreg92Spec>;
#[doc = "Stabilization matrix"]
pub mod swreg_92;
#[doc = "SWREG_93 (rw) register accessor: the output of Stabilization motion sum\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_93::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_93::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_93`]
module"]
#[doc(alias = "SWREG_93")]
pub type Swreg93 = crate::Reg<swreg_93::Swreg93Spec>;
#[doc = "the output of Stabilization motion sum"]
pub mod swreg_93;
#[doc = "SWREG_94 (rw) register accessor: output of Stabilization\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_94::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_94::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_94`]
module"]
#[doc(alias = "SWREG_94")]
pub type Swreg94 = crate::Reg<swreg_94::Swreg94Spec>;
#[doc = "output of Stabilization"]
pub mod swreg_94;
#[doc = "SWREG_95 (rw) register accessor: RGB to YUV conversion coefficient register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_95::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_95::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_95`]
module"]
#[doc(alias = "SWREG_95")]
pub type Swreg95 = crate::Reg<swreg_95::Swreg95Spec>;
#[doc = "RGB to YUV conversion coefficient register"]
pub mod swreg_95;
#[doc = "SWREG_96 (rw) register accessor: RGB to YUV conversion coefficient register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_96::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_96::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_96`]
module"]
#[doc(alias = "SWREG_96")]
pub type Swreg96 = crate::Reg<swreg_96::Swreg96Spec>;
#[doc = "RGB to YUV conversion coefficient register"]
pub mod swreg_96;
#[doc = "SWREG_97 (rw) register accessor: RGB to YUV conversion coefficient register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_97::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_97::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_97`]
module"]
#[doc(alias = "SWREG_97")]
pub type Swreg97 = crate::Reg<swreg_97::Swreg97Spec>;
#[doc = "RGB to YUV conversion coefficient register"]
pub mod swreg_97;
#[doc = "SWREG_98 (rw) register accessor: RGA MASK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_98::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_98::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_98`]
module"]
#[doc(alias = "SWREG_98")]
pub type Swreg98 = crate::Reg<swreg_98::Swreg98Spec>;
#[doc = "RGA MASK"]
pub mod swreg_98;
#[doc = "SWREG_99 (rw) register accessor: mv related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_99::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_99::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_99`]
module"]
#[doc(alias = "SWREG_99")]
pub type Swreg99 = crate::Reg<swreg_99::Swreg99Spec>;
#[doc = "mv related"]
pub mod swreg_99;
#[doc = "SWREG_100_REUSE (rw) register accessor: QP register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_100_reuse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_100_reuse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_100_reuse`]
module"]
#[doc(alias = "SWREG_100_REUSE")]
pub type Swreg100Reuse = crate::Reg<swreg_100_reuse::Swreg100ReuseSpec>;
#[doc = "QP register"]
pub mod swreg_100_reuse;
#[doc = "SWREG_101_READ (r) register accessor: hw config reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_101_read::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_101_read`]
module"]
#[doc(alias = "SWREG_101_READ")]
pub type Swreg101Read = crate::Reg<swreg_101_read::Swreg101ReadSpec>;
#[doc = "hw config reg"]
pub mod swreg_101_read;
#[doc = "SWREG_102 (rw) register accessor: mvc related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_102::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_102::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_102`]
module"]
#[doc(alias = "SWREG_102")]
pub type Swreg102 = crate::Reg<swreg_102::Swreg102Spec>;
#[doc = "mvc related"]
pub mod swreg_102;
#[doc = "SWREG_103 (rw) register accessor: encoder start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_103::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_103::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_103`]
module"]
#[doc(alias = "SWREG_103")]
pub type Swreg103 = crate::Reg<swreg_103::Swreg103Spec>;
#[doc = "encoder start"]
pub mod swreg_103;
#[doc = "SWREG_104 (rw) register accessor: mb control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_104::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_104::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_104`]
module"]
#[doc(alias = "SWREG_104")]
pub type Swreg104 = crate::Reg<swreg_104::Swreg104Spec>;
#[doc = "mb control register"]
pub mod swreg_104;
#[doc = "SWREG_105 (rw) register accessor: SWAP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_105::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_105::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_105`]
module"]
#[doc(alias = "SWREG_105")]
pub type Swreg105 = crate::Reg<swreg_105::Swreg105Spec>;
#[doc = "SWAP"]
pub mod swreg_105;
#[doc = "SWREG_106_REUSE (rw) register accessor: encoder control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_106_reuse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_106_reuse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_106_reuse`]
module"]
#[doc(alias = "SWREG_106_REUSE")]
pub type Swreg106Reuse = crate::Reg<swreg_106_reuse::Swreg106ReuseSpec>;
#[doc = "encoder control register 1"]
pub mod swreg_106_reuse;
#[doc = "SWREG_107_REUSE (rw) register accessor: JPEG control regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_107_reuse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_107_reuse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_107_reuse`]
module"]
#[doc(alias = "SWREG_107_REUSE")]
pub type Swreg107Reuse = crate::Reg<swreg_107_reuse::Swreg107ReuseSpec>;
#[doc = "JPEG control regsiter"]
pub mod swreg_107_reuse;
#[doc = "SWREG_108_REUSE (rw) register accessor: intra_slice_bmp2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_108_reuse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_108_reuse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_108_reuse`]
module"]
#[doc(alias = "SWREG_108_REUSE")]
pub type Swreg108Reuse = crate::Reg<swreg_108_reuse::Swreg108ReuseSpec>;
#[doc = "intra_slice_bmp2"]
pub mod swreg_108_reuse;
#[doc = "SWREG_109 (rw) register accessor: encoder status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_109::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_109::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_109`]
module"]
#[doc(alias = "SWREG_109")]
pub type Swreg109 = crate::Reg<swreg_109::Swreg109Spec>;
#[doc = "encoder status"]
pub mod swreg_109;
#[doc = "SWREG_110_READ (r) register accessor: product ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_110_read::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_110_read`]
module"]
#[doc(alias = "SWREG_110_READ")]
pub type Swreg110Read = crate::Reg<swreg_110_read::Swreg110ReadSpec>;
#[doc = "product ID"]
pub mod swreg_110_read;
#[doc = "SWREG_120_183 (w) register accessor: DMV_4p_1p_penalty\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_120_183::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg_120_183`]
module"]
#[doc(alias = "SWREG_120_183")]
pub type Swreg120_183 = crate::Reg<swreg_120_183::Swreg120_183Spec>;
#[doc = "DMV_4p_1p_penalty"]
pub mod swreg_120_183;
