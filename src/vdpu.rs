#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    swreg0: Swreg0,
    swreg1: Swreg1,
    swreg2: Swreg2,
    swreg3: Swreg3,
    swreg4: Swreg4,
    swreg5: Swreg5,
    swreg6: Swreg6,
    swreg7: Swreg7,
    swreg8: Swreg8,
    swreg9: Swreg9,
    swreg10: Swreg10,
    swreg11: Swreg11,
    swreg12: Swreg12,
    swreg13: Swreg13,
    swreg14: Swreg14,
    swreg15: Swreg15,
    swreg16: Swreg16,
    swreg17: Swreg17,
    swreg18: Swreg18,
    swreg19: Swreg19,
    swreg20: Swreg20,
    swreg21: Swreg21,
    swreg22: Swreg22,
    swreg23: Swreg23,
    swreg24: Swreg24,
    swreg25: Swreg25,
    swreg26: Swreg26,
    swreg27: Swreg27,
    swreg28: Swreg28,
    swreg29: Swreg29,
    swreg30: Swreg30,
    swreg31: Swreg31,
    swreg32: Swreg32,
    swreg33: Swreg33,
    swreg34: Swreg34,
    swreg35: Swreg35,
    swreg36: Swreg36,
    swreg37: Swreg37,
    swreg38: Swreg38,
    swreg39: Swreg39,
    swreg40: Swreg40,
    swreg41: Swreg41,
    _reserved42: [u8; 0x20],
    swreg50: Swreg50,
    swreg51: Swreg51,
    swreg52: Swreg52,
    swreg53: Swreg53,
    swreg54: Swreg54,
    swreg55: Swreg55,
    swreg56: Swreg56,
    swreg57: Swreg57,
    swreg58: Swreg58,
    swreg59: Swreg59,
    swreg60: Swreg60,
    swreg61: Swreg61,
    swreg62: Swreg62,
    swreg63: Swreg63,
    swreg64: Swreg64,
    swreg65: Swreg65,
    swreg66: Swreg66,
    swreg67: Swreg67,
    swreg68: Swreg68,
    swreg69: Swreg69,
    swreg70: Swreg70,
    swreg71: Swreg71,
    swreg72: Swreg72,
    swreg73: Swreg73,
    swreg74: Swreg74,
    swreg75: Swreg75,
    swreg76: Swreg76,
    swreg77: Swreg77,
    swreg78: Swreg78,
    swreg79: Swreg79,
    swreg80: Swreg80,
    swreg81: Swreg81,
    swreg82: Swreg82,
    swreg83: Swreg83,
    swreg84: Swreg84,
    swreg85: Swreg85,
    swreg86: Swreg86,
    swreg87: Swreg87,
    swreg88: Swreg88,
    swreg89: Swreg89,
    swreg90: Swreg90,
    swreg91: Swreg91,
    swreg92: Swreg92,
    swreg93: Swreg93,
    swreg94: Swreg94,
    swreg95: Swreg95,
    swreg96: Swreg96,
    swreg97: Swreg97,
    swreg98: Swreg98,
    swreg99: Swreg99,
    swreg100: Swreg100,
    swreg101: Swreg101,
    swreg102: Swreg102,
    swreg103: Swreg103,
    swreg104: Swreg104,
    swreg105: Swreg105,
    swreg106: Swreg106,
    swreg107: Swreg107,
    swreg108: Swreg108,
    swreg109: Swreg109,
    swreg110: Swreg110,
    swreg111: Swreg111,
    swreg112: Swreg112,
    swreg113: Swreg113,
    swreg114: Swreg114,
    swreg115: Swreg115,
    _reserved108: [u8; 0x10],
    swreg120: Swreg120,
    swreg121: Swreg121,
    swreg122: Swreg122,
    swreg123: Swreg123,
    swreg124: Swreg124,
    swreg125: Swreg125,
    swreg126: Swreg126,
    swreg127: Swreg127,
    swreg128: Swreg128,
    swreg129: Swreg129,
    swreg130: Swreg130,
    swreg131: Swreg131,
    swreg132: Swreg132,
    swreg133: Swreg133,
    swreg134: Swreg134,
    swreg135: Swreg135,
    swreg136: Swreg136,
    swreg137: Swreg137,
    swreg138: Swreg138,
    swreg139: Swreg139,
    swreg140: Swreg140,
    swreg141: Swreg141,
    swreg142: Swreg142,
    swreg143: Swreg143,
    swreg144: Swreg144,
    swreg145: Swreg145,
    swreg146: Swreg146,
    swreg147: Swreg147,
    swreg148: Swreg148,
    swreg149: Swreg149,
    swreg150: Swreg150,
    swreg151: Swreg151,
    swreg152: Swreg152,
    swreg153: Swreg153,
    swreg154: Swreg154,
    swreg155: Swreg155,
    swreg156: Swreg156,
    swreg157: Swreg157,
    swreg158: Swreg158,
    _reserved147: [u8; 0x14],
    swreg164_perf_latency_ctrl0: Swreg164PerfLatencyCtrl0,
    swreg165_perf_latency_ctrl1: Swreg165PerfLatencyCtrl1,
    swreg166_perf_rd_max_latency_num0: Swreg166PerfRdMaxLatencyNum0,
    swreg167_perf_rd_latency_samp_num: Swreg167PerfRdLatencySampNum,
    swreg168_perf_rd_latency_acc_sum: Swreg168PerfRdLatencyAccSum,
    swreg169_perf_rd_axi_total_byte: Swreg169PerfRdAxiTotalByte,
    swreg170_perf_wr_axi_total_byte: Swreg170PerfWrAxiTotalByte,
    swreg171_perf_working_cnt: Swreg171PerfWorkingCnt,
}
impl RegisterBlock {
    #[doc = "0x00 - axi control"]
    #[inline(always)]
    pub const fn swreg0(&self) -> &Swreg0 {
        &self.swreg0
    }
    #[doc = "0x04 - color coeff register"]
    #[inline(always)]
    pub const fn swreg1(&self) -> &Swreg1 {
        &self.swreg1
    }
    #[doc = "0x08 - color coeff register"]
    #[inline(always)]
    pub const fn swreg2(&self) -> &Swreg2 {
        &self.swreg2
    }
    #[doc = "0x0c - color coeff register"]
    #[inline(always)]
    pub const fn swreg3(&self) -> &Swreg3 {
        &self.swreg3
    }
    #[doc = "0x10 - scl ctrl register"]
    #[inline(always)]
    pub const fn swreg4(&self) -> &Swreg4 {
        &self.swreg4
    }
    #[doc = "0x14 - scl ctrl register"]
    #[inline(always)]
    pub const fn swreg5(&self) -> &Swreg5 {
        &self.swreg5
    }
    #[doc = "0x18 - scl ctrl register"]
    #[inline(always)]
    pub const fn swreg6(&self) -> &Swreg6 {
        &self.swreg6
    }
    #[doc = "0x1c - Amount of pixels beyond border"]
    #[inline(always)]
    pub const fn swreg7(&self) -> &Swreg7 {
        &self.swreg7
    }
    #[doc = "0x20 - Amount of pixels beyond border"]
    #[inline(always)]
    pub const fn swreg8(&self) -> &Swreg8 {
        &self.swreg8
    }
    #[doc = "0x24 - Rmask register"]
    #[inline(always)]
    pub const fn swreg9(&self) -> &Swreg9 {
        &self.swreg9
    }
    #[doc = "0x28 - Gmask register"]
    #[inline(always)]
    pub const fn swreg10(&self) -> &Swreg10 {
        &self.swreg10
    }
    #[doc = "0x2c - Bmask register"]
    #[inline(always)]
    pub const fn swreg11(&self) -> &Swreg11 {
        &self.swreg11
    }
    #[doc = "0x30 - PP input picture base address for Y bottom field"]
    #[inline(always)]
    pub const fn swreg12(&self) -> &Swreg12 {
        &self.swreg12
    }
    #[doc = "0x34 - PP input picture base for Ch bottom field"]
    #[inline(always)]
    pub const fn swreg13(&self) -> &Swreg13 {
        &self.swreg13
    }
    #[doc = "0x38 - coordinate used in macroblock crop"]
    #[inline(always)]
    pub const fn swreg14(&self) -> &Swreg14 {
        &self.swreg14
    }
    #[doc = "0x3c - range map register"]
    #[inline(always)]
    pub const fn swreg15(&self) -> &Swreg15 {
        &self.swreg15
    }
    #[doc = "0x40 - total num of padded for RGB"]
    #[inline(always)]
    pub const fn swreg16(&self) -> &Swreg16 {
        &self.swreg16
    }
    #[doc = "0x44 - hw support informan,read only"]
    #[inline(always)]
    pub const fn swreg17(&self) -> &Swreg17 {
        &self.swreg17
    }
    #[doc = "0x48 - base address for reading post-processing input picture uminan"]
    #[inline(always)]
    pub const fn swreg18(&self) -> &Swreg18 {
        &self.swreg18
    }
    #[doc = "0x4c - Base address for reading post-processing input picture Cb/Ch"]
    #[inline(always)]
    pub const fn swreg19(&self) -> &Swreg19 {
        &self.swreg19
    }
    #[doc = "0x50 - input cr component address"]
    #[inline(always)]
    pub const fn swreg20(&self) -> &Swreg20 {
        &self.swreg20
    }
    #[doc = "0x54 - Base address for writing post-processed picture luminance/RGB"]
    #[inline(always)]
    pub const fn swreg21(&self) -> &Swreg21 {
        &self.swreg21
    }
    #[doc = "0x58 - Base address for writing post-processed picture Ch"]
    #[inline(always)]
    pub const fn swreg22(&self) -> &Swreg22 {
        &self.swreg22
    }
    #[doc = "0x5c - Display width and PP input size extension register"]
    #[inline(always)]
    pub const fn swreg23(&self) -> &Swreg23 {
        &self.swreg23
    }
    #[doc = "0x60 - alpha blending base address"]
    #[inline(always)]
    pub const fn swreg24(&self) -> &Swreg24 {
        &self.swreg24
    }
    #[doc = "0x64 - ablend of pixels scanline"]
    #[inline(always)]
    pub const fn swreg25(&self) -> &Swreg25 {
        &self.swreg25
    }
    #[doc = "0x68 - x-coordinate of mask area 1 for Horizontal start pixel"]
    #[inline(always)]
    pub const fn swreg26(&self) -> &Swreg26 {
        &self.swreg26
    }
    #[doc = "0x6c - y-coordinate of mask area 1 for Horizontal start pixel"]
    #[inline(always)]
    pub const fn swreg27(&self) -> &Swreg27 {
        &self.swreg27
    }
    #[doc = "0x70 - x-coordinate of mask area 2 for Horizontal start pixel"]
    #[inline(always)]
    pub const fn swreg28(&self) -> &Swreg28 {
        &self.swreg28
    }
    #[doc = "0x74 - y-coordinate of mask area 2 for Horizontal start pixel"]
    #[inline(always)]
    pub const fn swreg29(&self) -> &Swreg29 {
        &self.swreg29
    }
    #[doc = "0x78 - register for deinterlace ctrl"]
    #[inline(always)]
    pub const fn swreg30(&self) -> &Swreg30 {
        &self.swreg30
    }
    #[doc = "0x7c - contrast adjust threshold"]
    #[inline(always)]
    pub const fn swreg31(&self) -> &Swreg31 {
        &self.swreg31
    }
    #[doc = "0x80 - contrast adjust offset"]
    #[inline(always)]
    pub const fn swreg32(&self) -> &Swreg32 {
        &self.swreg32
    }
    #[doc = "0x84 - Synthesis configuration register post-processor (read only)"]
    #[inline(always)]
    pub const fn swreg33(&self) -> &Swreg33 {
        &self.swreg33
    }
    #[doc = "0x88 - PP input pic size register"]
    #[inline(always)]
    pub const fn swreg34(&self) -> &Swreg34 {
        &self.swreg34
    }
    #[doc = "0x8c - PP output pic size register"]
    #[inline(always)]
    pub const fn swreg35(&self) -> &Swreg35 {
        &self.swreg35
    }
    #[doc = "0x90 - the dither mode for RGB"]
    #[inline(always)]
    pub const fn swreg36(&self) -> &Swreg36 {
        &self.swreg36
    }
    #[doc = "0x94 - PP input/output data format"]
    #[inline(always)]
    pub const fn swreg37(&self) -> &Swreg37 {
        &self.swreg37
    }
    #[doc = "0x98 - PP input/output data format"]
    #[inline(always)]
    pub const fn swreg38(&self) -> &Swreg38 {
        &self.swreg38
    }
    #[doc = "0x9c - Register0000 Abstract"]
    #[inline(always)]
    pub const fn swreg39(&self) -> &Swreg39 {
        &self.swreg39
    }
    #[doc = "0xa0 - pp int register"]
    #[inline(always)]
    pub const fn swreg40(&self) -> &Swreg40 {
        &self.swreg40
    }
    #[doc = "0xa4 - enable ctrl flag"]
    #[inline(always)]
    pub const fn swreg41(&self) -> &Swreg41 {
        &self.swreg41
    }
    #[doc = "0xc8 - video decoder ctrl register"]
    #[inline(always)]
    pub const fn swreg50(&self) -> &Swreg50 {
        &self.swreg50
    }
    #[doc = "0xcc - the stream length"]
    #[inline(always)]
    pub const fn swreg51(&self) -> &Swreg51 {
        &self.swreg51
    }
    #[doc = "0xd0 - error concealment case related"]
    #[inline(always)]
    pub const fn swreg52(&self) -> &Swreg52 {
        &self.swreg52
    }
    #[doc = "0xd4 - decoder format"]
    #[inline(always)]
    pub const fn swreg53(&self) -> &Swreg53 {
        &self.swreg53
    }
    #[doc = "0xd8 - endian for input/output data"]
    #[inline(always)]
    pub const fn swreg54(&self) -> &Swreg54 {
        &self.swreg54
    }
    #[doc = "0xdc - decoder int register"]
    #[inline(always)]
    pub const fn swreg55(&self) -> &Swreg55 {
        &self.swreg55
    }
    #[doc = "0xe0 - axi ctrl for decoder"]
    #[inline(always)]
    pub const fn swreg56(&self) -> &Swreg56 {
        &self.swreg56
    }
    #[doc = "0xe4 - enable flag for decoder"]
    #[inline(always)]
    pub const fn swreg57(&self) -> &Swreg57 {
        &self.swreg57
    }
    #[doc = "0xe8 - soft reset register"]
    #[inline(always)]
    pub const fn swreg58(&self) -> &Swreg58 {
        &self.swreg58
    }
    #[doc = "0xec - H264, MPEG4, VP6 Prediction filter tap"]
    #[inline(always)]
    pub const fn swreg59(&self) -> &Swreg59 {
        &self.swreg59
    }
    #[doc = "0xf0 - additional chrominance address"]
    #[inline(always)]
    pub const fn swreg60(&self) -> &Swreg60 {
        &self.swreg60
    }
    #[doc = "0xf4 - standard dependent tables start address"]
    #[inline(always)]
    pub const fn swreg61(&self) -> &Swreg61 {
        &self.swreg61
    }
    #[doc = "0xf8 - Direct mode motion vector write/read start address"]
    #[inline(always)]
    pub const fn swreg62(&self) -> &Swreg62 {
        &self.swreg62
    }
    #[doc = "0xfc - write decoder output picture or field start address"]
    #[inline(always)]
    pub const fn swreg63(&self) -> &Swreg63 {
        &self.swreg63
    }
    #[doc = "0x100 - rlc or vlc mode input data start addr"]
    #[inline(always)]
    pub const fn swreg64(&self) -> &Swreg64 {
        &self.swreg64
    }
    #[doc = "0x104 - refbufferd related"]
    #[inline(always)]
    pub const fn swreg65(&self) -> &Swreg65 {
        &self.swreg65
    }
    #[doc = "0x108 - ID register"]
    #[inline(always)]
    pub const fn swreg66(&self) -> &Swreg66 {
        &self.swreg66
    }
    #[doc = "0x10c - Synthesis configuration register decoder 1(read only)"]
    #[inline(always)]
    pub const fn swreg67(&self) -> &Swreg67 {
        &self.swreg67
    }
    #[doc = "0x110 - sum of partitions(read only)"]
    #[inline(always)]
    pub const fn swreg68(&self) -> &Swreg68 {
        &self.swreg68
    }
    #[doc = "0x114 - sum information (read only)"]
    #[inline(always)]
    pub const fn swreg69(&self) -> &Swreg69 {
        &self.swreg69
    }
    #[doc = "0x118 - sum of the decoded motion vector y-components(read only)"]
    #[inline(always)]
    pub const fn swreg70(&self) -> &Swreg70 {
        &self.swreg70
    }
    #[doc = "0x11c - information for read only register"]
    #[inline(always)]
    pub const fn swreg71(&self) -> &Swreg71 {
        &self.swreg71
    }
    #[doc = "0x120 - debug0"]
    #[inline(always)]
    pub const fn swreg72(&self) -> &Swreg72 {
        &self.swreg72
    }
    #[doc = "0x124 - debug registers"]
    #[inline(always)]
    pub const fn swreg73(&self) -> &Swreg73 {
        &self.swreg73
    }
    #[doc = "0x128 - MV address for h264"]
    #[inline(always)]
    pub const fn swreg74(&self) -> &Swreg74 {
        &self.swreg74
    }
    #[doc = "0x12c - H.264 Intra prediction 4x4 mode start address"]
    #[inline(always)]
    pub const fn swreg75(&self) -> &Swreg75 {
        &self.swreg75
    }
    #[doc = "0x130 - the number of referance pic"]
    #[inline(always)]
    pub const fn swreg76(&self) -> &Swreg76 {
        &self.swreg76
    }
    #[doc = "0x134 - the number of referance pic"]
    #[inline(always)]
    pub const fn swreg77(&self) -> &Swreg77 {
        &self.swreg77
    }
    #[doc = "0x138 - the number of referance pic"]
    #[inline(always)]
    pub const fn swreg78(&self) -> &Swreg78 {
        &self.swreg78
    }
    #[doc = "0x13c - the number of referance pic"]
    #[inline(always)]
    pub const fn swreg79(&self) -> &Swreg79 {
        &self.swreg79
    }
    #[doc = "0x140 - the number of referance pic"]
    #[inline(always)]
    pub const fn swreg80(&self) -> &Swreg80 {
        &self.swreg80
    }
    #[doc = "0x144 - the number of referance pic"]
    #[inline(always)]
    pub const fn swreg81(&self) -> &Swreg81 {
        &self.swreg81
    }
    #[doc = "0x148 - the number of referance pic"]
    #[inline(always)]
    pub const fn swreg82(&self) -> &Swreg82 {
        &self.swreg82
    }
    #[doc = "0x14c - the number of referance pic"]
    #[inline(always)]
    pub const fn swreg83(&self) -> &Swreg83 {
        &self.swreg83
    }
    #[doc = "0x150 - referance frame0 address for h264"]
    #[inline(always)]
    pub const fn swreg84(&self) -> &Swreg84 {
        &self.swreg84
    }
    #[doc = "0x154 - referance frame1 address for h264"]
    #[inline(always)]
    pub const fn swreg85(&self) -> &Swreg85 {
        &self.swreg85
    }
    #[doc = "0x158 - referance frame2 address for h264"]
    #[inline(always)]
    pub const fn swreg86(&self) -> &Swreg86 {
        &self.swreg86
    }
    #[doc = "0x15c - referance frame3 address for h264"]
    #[inline(always)]
    pub const fn swreg87(&self) -> &Swreg87 {
        &self.swreg87
    }
    #[doc = "0x160 - referance frame4 address for h264"]
    #[inline(always)]
    pub const fn swreg88(&self) -> &Swreg88 {
        &self.swreg88
    }
    #[doc = "0x164 - referance frame5 address for h264"]
    #[inline(always)]
    pub const fn swreg89(&self) -> &Swreg89 {
        &self.swreg89
    }
    #[doc = "0x168 - referance frame6 address for h264"]
    #[inline(always)]
    pub const fn swreg90(&self) -> &Swreg90 {
        &self.swreg90
    }
    #[doc = "0x16c - referance frame7 address for h264"]
    #[inline(always)]
    pub const fn swreg91(&self) -> &Swreg91 {
        &self.swreg91
    }
    #[doc = "0x170 - referance frame8 address for h264"]
    #[inline(always)]
    pub const fn swreg92(&self) -> &Swreg92 {
        &self.swreg92
    }
    #[doc = "0x174 - referance frame9 address for h264"]
    #[inline(always)]
    pub const fn swreg93(&self) -> &Swreg93 {
        &self.swreg93
    }
    #[doc = "0x178 - referance frame10 address for h264"]
    #[inline(always)]
    pub const fn swreg94(&self) -> &Swreg94 {
        &self.swreg94
    }
    #[doc = "0x17c - referance frame11 address for h264"]
    #[inline(always)]
    pub const fn swreg95(&self) -> &Swreg95 {
        &self.swreg95
    }
    #[doc = "0x180 - referance frame12 address for h264"]
    #[inline(always)]
    pub const fn swreg96(&self) -> &Swreg96 {
        &self.swreg96
    }
    #[doc = "0x184 - referance frame13 address for h264"]
    #[inline(always)]
    pub const fn swreg97(&self) -> &Swreg97 {
        &self.swreg97
    }
    #[doc = "0x188 - referance frame14 address for h264"]
    #[inline(always)]
    pub const fn swreg98(&self) -> &Swreg98 {
        &self.swreg98
    }
    #[doc = "0x18c - referance frame15 address for h264"]
    #[inline(always)]
    pub const fn swreg99(&self) -> &Swreg99 {
        &self.swreg99
    }
    #[doc = "0x190 - initial reference picture list related"]
    #[inline(always)]
    pub const fn swreg100(&self) -> &Swreg100 {
        &self.swreg100
    }
    #[doc = "0x194 - initial reference picture list related"]
    #[inline(always)]
    pub const fn swreg101(&self) -> &Swreg101 {
        &self.swreg101
    }
    #[doc = "0x198 - initial reference picture list related"]
    #[inline(always)]
    pub const fn swreg102(&self) -> &Swreg102 {
        &self.swreg102
    }
    #[doc = "0x19c - initial reference picture list related"]
    #[inline(always)]
    pub const fn swreg103(&self) -> &Swreg103 {
        &self.swreg103
    }
    #[doc = "0x1a0 - initial reference picture list related"]
    #[inline(always)]
    pub const fn swreg104(&self) -> &Swreg104 {
        &self.swreg104
    }
    #[doc = "0x1a4 - initial reference picture list related"]
    #[inline(always)]
    pub const fn swreg105(&self) -> &Swreg105 {
        &self.swreg105
    }
    #[doc = "0x1a8 - initial reference picture list related"]
    #[inline(always)]
    pub const fn swreg106(&self) -> &Swreg106 {
        &self.swreg106
    }
    #[doc = "0x1ac - long term flag for reference pictuure index"]
    #[inline(always)]
    pub const fn swreg107(&self) -> &Swreg107 {
        &self.swreg107
    }
    #[doc = "0x1b0 - valid flag for reference picture index"]
    #[inline(always)]
    pub const fn swreg108(&self) -> &Swreg108 {
        &self.swreg108
    }
    #[doc = "0x1b4 - the stream start word for decoder"]
    #[inline(always)]
    pub const fn swreg109(&self) -> &Swreg109 {
        &self.swreg109
    }
    #[doc = "0x1b8 - h264 pic mb size"]
    #[inline(always)]
    pub const fn swreg110(&self) -> &Swreg110 {
        &self.swreg110
    }
    #[doc = "0x1bc - h264 ctrl related"]
    #[inline(always)]
    pub const fn swreg111(&self) -> &Swreg111 {
        &self.swreg111
    }
    #[doc = "0x1c0 - current frame related"]
    #[inline(always)]
    pub const fn swreg112(&self) -> &Swreg112 {
        &self.swreg112
    }
    #[doc = "0x1c4 - reference picture related"]
    #[inline(always)]
    pub const fn swreg113(&self) -> &Swreg113 {
        &self.swreg113
    }
    #[doc = "0x1c8 - maximum reference"]
    #[inline(always)]
    pub const fn swreg114(&self) -> &Swreg114 {
        &self.swreg114
    }
    #[doc = "0x1cc - enable flag"]
    #[inline(always)]
    pub const fn swreg115(&self) -> &Swreg115 {
        &self.swreg115
    }
    #[doc = "0x1e0 - multi format reuse register0"]
    #[inline(always)]
    pub const fn swreg120(&self) -> &Swreg120 {
        &self.swreg120
    }
    #[doc = "0x1e4 - multi format reuse register1"]
    #[inline(always)]
    pub const fn swreg121(&self) -> &Swreg121 {
        &self.swreg121
    }
    #[doc = "0x1e8 - multi format reuse register2"]
    #[inline(always)]
    pub const fn swreg122(&self) -> &Swreg122 {
        &self.swreg122
    }
    #[doc = "0x1ec - multi format reuse register3"]
    #[inline(always)]
    pub const fn swreg123(&self) -> &Swreg123 {
        &self.swreg123
    }
    #[doc = "0x1f0 - multi format reuse register4"]
    #[inline(always)]
    pub const fn swreg124(&self) -> &Swreg124 {
        &self.swreg124
    }
    #[doc = "0x1f4 - multi format reuse register5"]
    #[inline(always)]
    pub const fn swreg125(&self) -> &Swreg125 {
        &self.swreg125
    }
    #[doc = "0x1f8 - multi format reuse register6"]
    #[inline(always)]
    pub const fn swreg126(&self) -> &Swreg126 {
        &self.swreg126
    }
    #[doc = "0x1fc - multi format reuse register7"]
    #[inline(always)]
    pub const fn swreg127(&self) -> &Swreg127 {
        &self.swreg127
    }
    #[doc = "0x200 - multi format reuse register8"]
    #[inline(always)]
    pub const fn swreg128(&self) -> &Swreg128 {
        &self.swreg128
    }
    #[doc = "0x204 - multi format reuse register9"]
    #[inline(always)]
    pub const fn swreg129(&self) -> &Swreg129 {
        &self.swreg129
    }
    #[doc = "0x208 - multi format reuse register10"]
    #[inline(always)]
    pub const fn swreg130(&self) -> &Swreg130 {
        &self.swreg130
    }
    #[doc = "0x20c - multi format reuse register11"]
    #[inline(always)]
    pub const fn swreg131(&self) -> &Swreg131 {
        &self.swreg131
    }
    #[doc = "0x210 - multi format reuse register12"]
    #[inline(always)]
    pub const fn swreg132(&self) -> &Swreg132 {
        &self.swreg132
    }
    #[doc = "0x214 - multi format reuse register13"]
    #[inline(always)]
    pub const fn swreg133(&self) -> &Swreg133 {
        &self.swreg133
    }
    #[doc = "0x218 - multi format reuse register14"]
    #[inline(always)]
    pub const fn swreg134(&self) -> &Swreg134 {
        &self.swreg134
    }
    #[doc = "0x21c - multi format reuse register15"]
    #[inline(always)]
    pub const fn swreg135(&self) -> &Swreg135 {
        &self.swreg135
    }
    #[doc = "0x220 - multi format reuse register16"]
    #[inline(always)]
    pub const fn swreg136(&self) -> &Swreg136 {
        &self.swreg136
    }
    #[doc = "0x224 - multi format reuse register17"]
    #[inline(always)]
    pub const fn swreg137(&self) -> &Swreg137 {
        &self.swreg137
    }
    #[doc = "0x228 - multi format reuse register18"]
    #[inline(always)]
    pub const fn swreg138(&self) -> &Swreg138 {
        &self.swreg138
    }
    #[doc = "0x22c - multi format reuse register19"]
    #[inline(always)]
    pub const fn swreg139(&self) -> &Swreg139 {
        &self.swreg139
    }
    #[doc = "0x230 - multi format reuse register20"]
    #[inline(always)]
    pub const fn swreg140(&self) -> &Swreg140 {
        &self.swreg140
    }
    #[doc = "0x234 - multi format reuse register21"]
    #[inline(always)]
    pub const fn swreg141(&self) -> &Swreg141 {
        &self.swreg141
    }
    #[doc = "0x238 - multi format reuse register22"]
    #[inline(always)]
    pub const fn swreg142(&self) -> &Swreg142 {
        &self.swreg142
    }
    #[doc = "0x23c - multi format reuse register23"]
    #[inline(always)]
    pub const fn swreg143(&self) -> &Swreg143 {
        &self.swreg143
    }
    #[doc = "0x240 - multi format reuse register24"]
    #[inline(always)]
    pub const fn swreg144(&self) -> &Swreg144 {
        &self.swreg144
    }
    #[doc = "0x244 - multi format reuse register25"]
    #[inline(always)]
    pub const fn swreg145(&self) -> &Swreg145 {
        &self.swreg145
    }
    #[doc = "0x248 - multi format reuse register26"]
    #[inline(always)]
    pub const fn swreg146(&self) -> &Swreg146 {
        &self.swreg146
    }
    #[doc = "0x24c - multi format reuse register27"]
    #[inline(always)]
    pub const fn swreg147(&self) -> &Swreg147 {
        &self.swreg147
    }
    #[doc = "0x250 - multi format reuse register28"]
    #[inline(always)]
    pub const fn swreg148(&self) -> &Swreg148 {
        &self.swreg148
    }
    #[doc = "0x254 - multi format reuse register29"]
    #[inline(always)]
    pub const fn swreg149(&self) -> &Swreg149 {
        &self.swreg149
    }
    #[doc = "0x258 - multi format reuse register30"]
    #[inline(always)]
    pub const fn swreg150(&self) -> &Swreg150 {
        &self.swreg150
    }
    #[doc = "0x25c - multi format reuse register31"]
    #[inline(always)]
    pub const fn swreg151(&self) -> &Swreg151 {
        &self.swreg151
    }
    #[doc = "0x260 - multi format reuse register32"]
    #[inline(always)]
    pub const fn swreg152(&self) -> &Swreg152 {
        &self.swreg152
    }
    #[doc = "0x264 - multi format reuse register33"]
    #[inline(always)]
    pub const fn swreg153(&self) -> &Swreg153 {
        &self.swreg153
    }
    #[doc = "0x268 - multi format reuse register34"]
    #[inline(always)]
    pub const fn swreg154(&self) -> &Swreg154 {
        &self.swreg154
    }
    #[doc = "0x26c - multi format reuse register35"]
    #[inline(always)]
    pub const fn swreg155(&self) -> &Swreg155 {
        &self.swreg155
    }
    #[doc = "0x270 - multi format reuse register36"]
    #[inline(always)]
    pub const fn swreg156(&self) -> &Swreg156 {
        &self.swreg156
    }
    #[doc = "0x274 - multi format reuse register37"]
    #[inline(always)]
    pub const fn swreg157(&self) -> &Swreg157 {
        &self.swreg157
    }
    #[doc = "0x278 - multi format reuse register38"]
    #[inline(always)]
    pub const fn swreg158(&self) -> &Swreg158 {
        &self.swreg158
    }
    #[doc = "0x290 - Axi performance latency module contrl register0"]
    #[inline(always)]
    pub const fn swreg164_perf_latency_ctrl0(&self) -> &Swreg164PerfLatencyCtrl0 {
        &self.swreg164_perf_latency_ctrl0
    }
    #[doc = "0x294 - PERF_LATENCY_CTRL1"]
    #[inline(always)]
    pub const fn swreg165_perf_latency_ctrl1(&self) -> &Swreg165PerfLatencyCtrl1 {
        &self.swreg165_perf_latency_ctrl1
    }
    #[doc = "0x298 - Read max latency number"]
    #[inline(always)]
    pub const fn swreg166_perf_rd_max_latency_num0(&self) -> &Swreg166PerfRdMaxLatencyNum0 {
        &self.swreg166_perf_rd_max_latency_num0
    }
    #[doc = "0x29c - The number of bigger than configed threshold value"]
    #[inline(always)]
    pub const fn swreg167_perf_rd_latency_samp_num(&self) -> &Swreg167PerfRdLatencySampNum {
        &self.swreg167_perf_rd_latency_samp_num
    }
    #[doc = "0x2a0 - Total sample number"]
    #[inline(always)]
    pub const fn swreg168_perf_rd_latency_acc_sum(&self) -> &Swreg168PerfRdLatencyAccSum {
        &self.swreg168_perf_rd_latency_acc_sum
    }
    #[doc = "0x2a4 - perf_rd_axi_total_byte"]
    #[inline(always)]
    pub const fn swreg169_perf_rd_axi_total_byte(&self) -> &Swreg169PerfRdAxiTotalByte {
        &self.swreg169_perf_rd_axi_total_byte
    }
    #[doc = "0x2a8 - perf_wr_axi_total_byte"]
    #[inline(always)]
    pub const fn swreg170_perf_wr_axi_total_byte(&self) -> &Swreg170PerfWrAxiTotalByte {
        &self.swreg170_perf_wr_axi_total_byte
    }
    #[doc = "0x2ac - perf_working_cnt"]
    #[inline(always)]
    pub const fn swreg171_perf_working_cnt(&self) -> &Swreg171PerfWorkingCnt {
        &self.swreg171_perf_working_cnt
    }
}
#[doc = "SWREG0 (rw) register accessor: axi control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg0`]
module"]
#[doc(alias = "SWREG0")]
pub type Swreg0 = crate::Reg<swreg0::Swreg0Spec>;
#[doc = "axi control"]
pub mod swreg0;
#[doc = "SWREG1 (rw) register accessor: color coeff register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg1`]
module"]
#[doc(alias = "SWREG1")]
pub type Swreg1 = crate::Reg<swreg1::Swreg1Spec>;
#[doc = "color coeff register"]
pub mod swreg1;
#[doc = "SWREG2 (rw) register accessor: color coeff register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg2`]
module"]
#[doc(alias = "SWREG2")]
pub type Swreg2 = crate::Reg<swreg2::Swreg2Spec>;
#[doc = "color coeff register"]
pub mod swreg2;
#[doc = "SWREG3 (rw) register accessor: color coeff register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg3`]
module"]
#[doc(alias = "SWREG3")]
pub type Swreg3 = crate::Reg<swreg3::Swreg3Spec>;
#[doc = "color coeff register"]
pub mod swreg3;
#[doc = "SWREG4 (rw) register accessor: scl ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg4`]
module"]
#[doc(alias = "SWREG4")]
pub type Swreg4 = crate::Reg<swreg4::Swreg4Spec>;
#[doc = "scl ctrl register"]
pub mod swreg4;
#[doc = "SWREG5 (rw) register accessor: scl ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg5`]
module"]
#[doc(alias = "SWREG5")]
pub type Swreg5 = crate::Reg<swreg5::Swreg5Spec>;
#[doc = "scl ctrl register"]
pub mod swreg5;
#[doc = "SWREG6 (rw) register accessor: scl ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg6`]
module"]
#[doc(alias = "SWREG6")]
pub type Swreg6 = crate::Reg<swreg6::Swreg6Spec>;
#[doc = "scl ctrl register"]
pub mod swreg6;
#[doc = "SWREG7 (rw) register accessor: Amount of pixels beyond border\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg7`]
module"]
#[doc(alias = "SWREG7")]
pub type Swreg7 = crate::Reg<swreg7::Swreg7Spec>;
#[doc = "Amount of pixels beyond border"]
pub mod swreg7;
#[doc = "SWREG8 (rw) register accessor: Amount of pixels beyond border\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg8`]
module"]
#[doc(alias = "SWREG8")]
pub type Swreg8 = crate::Reg<swreg8::Swreg8Spec>;
#[doc = "Amount of pixels beyond border"]
pub mod swreg8;
#[doc = "SWREG9 (rw) register accessor: Rmask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg9`]
module"]
#[doc(alias = "SWREG9")]
pub type Swreg9 = crate::Reg<swreg9::Swreg9Spec>;
#[doc = "Rmask register"]
pub mod swreg9;
#[doc = "SWREG10 (rw) register accessor: Gmask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg10`]
module"]
#[doc(alias = "SWREG10")]
pub type Swreg10 = crate::Reg<swreg10::Swreg10Spec>;
#[doc = "Gmask register"]
pub mod swreg10;
#[doc = "SWREG11 (rw) register accessor: Bmask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg11`]
module"]
#[doc(alias = "SWREG11")]
pub type Swreg11 = crate::Reg<swreg11::Swreg11Spec>;
#[doc = "Bmask register"]
pub mod swreg11;
#[doc = "SWREG12 (rw) register accessor: PP input picture base address for Y bottom field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg12`]
module"]
#[doc(alias = "SWREG12")]
pub type Swreg12 = crate::Reg<swreg12::Swreg12Spec>;
#[doc = "PP input picture base address for Y bottom field"]
pub mod swreg12;
#[doc = "SWREG13 (rw) register accessor: PP input picture base for Ch bottom field\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg13`]
module"]
#[doc(alias = "SWREG13")]
pub type Swreg13 = crate::Reg<swreg13::Swreg13Spec>;
#[doc = "PP input picture base for Ch bottom field"]
pub mod swreg13;
#[doc = "SWREG14 (rw) register accessor: coordinate used in macroblock crop\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg14`]
module"]
#[doc(alias = "SWREG14")]
pub type Swreg14 = crate::Reg<swreg14::Swreg14Spec>;
#[doc = "coordinate used in macroblock crop"]
pub mod swreg14;
#[doc = "SWREG15 (rw) register accessor: range map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg15`]
module"]
#[doc(alias = "SWREG15")]
pub type Swreg15 = crate::Reg<swreg15::Swreg15Spec>;
#[doc = "range map register"]
pub mod swreg15;
#[doc = "SWREG16 (rw) register accessor: total num of padded for RGB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg16`]
module"]
#[doc(alias = "SWREG16")]
pub type Swreg16 = crate::Reg<swreg16::Swreg16Spec>;
#[doc = "total num of padded for RGB"]
pub mod swreg16;
#[doc = "SWREG17 (r) register accessor: hw support informan,read only\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg17::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg17`]
module"]
#[doc(alias = "SWREG17")]
pub type Swreg17 = crate::Reg<swreg17::Swreg17Spec>;
#[doc = "hw support informan,read only"]
pub mod swreg17;
#[doc = "SWREG18 (rw) register accessor: base address for reading post-processing input picture uminan\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg18`]
module"]
#[doc(alias = "SWREG18")]
pub type Swreg18 = crate::Reg<swreg18::Swreg18Spec>;
#[doc = "base address for reading post-processing input picture uminan"]
pub mod swreg18;
#[doc = "SWREG19 (rw) register accessor: Base address for reading post-processing input picture Cb/Ch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg19`]
module"]
#[doc(alias = "SWREG19")]
pub type Swreg19 = crate::Reg<swreg19::Swreg19Spec>;
#[doc = "Base address for reading post-processing input picture Cb/Ch"]
pub mod swreg19;
#[doc = "SWREG20 (rw) register accessor: input cr component address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg20`]
module"]
#[doc(alias = "SWREG20")]
pub type Swreg20 = crate::Reg<swreg20::Swreg20Spec>;
#[doc = "input cr component address"]
pub mod swreg20;
#[doc = "SWREG21 (rw) register accessor: Base address for writing post-processed picture luminance/RGB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg21`]
module"]
#[doc(alias = "SWREG21")]
pub type Swreg21 = crate::Reg<swreg21::Swreg21Spec>;
#[doc = "Base address for writing post-processed picture luminance/RGB"]
pub mod swreg21;
#[doc = "SWREG22 (rw) register accessor: Base address for writing post-processed picture Ch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg22`]
module"]
#[doc(alias = "SWREG22")]
pub type Swreg22 = crate::Reg<swreg22::Swreg22Spec>;
#[doc = "Base address for writing post-processed picture Ch"]
pub mod swreg22;
#[doc = "SWREG23 (rw) register accessor: Display width and PP input size extension register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg23`]
module"]
#[doc(alias = "SWREG23")]
pub type Swreg23 = crate::Reg<swreg23::Swreg23Spec>;
#[doc = "Display width and PP input size extension register"]
pub mod swreg23;
#[doc = "SWREG24 (rw) register accessor: alpha blending base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg24`]
module"]
#[doc(alias = "SWREG24")]
pub type Swreg24 = crate::Reg<swreg24::Swreg24Spec>;
#[doc = "alpha blending base address"]
pub mod swreg24;
#[doc = "SWREG25 (rw) register accessor: ablend of pixels scanline\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg25`]
module"]
#[doc(alias = "SWREG25")]
pub type Swreg25 = crate::Reg<swreg25::Swreg25Spec>;
#[doc = "ablend of pixels scanline"]
pub mod swreg25;
#[doc = "SWREG26 (rw) register accessor: x-coordinate of mask area 1 for Horizontal start pixel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg26`]
module"]
#[doc(alias = "SWREG26")]
pub type Swreg26 = crate::Reg<swreg26::Swreg26Spec>;
#[doc = "x-coordinate of mask area 1 for Horizontal start pixel"]
pub mod swreg26;
#[doc = "SWREG27 (rw) register accessor: y-coordinate of mask area 1 for Horizontal start pixel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg27`]
module"]
#[doc(alias = "SWREG27")]
pub type Swreg27 = crate::Reg<swreg27::Swreg27Spec>;
#[doc = "y-coordinate of mask area 1 for Horizontal start pixel"]
pub mod swreg27;
#[doc = "SWREG28 (rw) register accessor: x-coordinate of mask area 2 for Horizontal start pixel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg28`]
module"]
#[doc(alias = "SWREG28")]
pub type Swreg28 = crate::Reg<swreg28::Swreg28Spec>;
#[doc = "x-coordinate of mask area 2 for Horizontal start pixel"]
pub mod swreg28;
#[doc = "SWREG29 (rw) register accessor: y-coordinate of mask area 2 for Horizontal start pixel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg29`]
module"]
#[doc(alias = "SWREG29")]
pub type Swreg29 = crate::Reg<swreg29::Swreg29Spec>;
#[doc = "y-coordinate of mask area 2 for Horizontal start pixel"]
pub mod swreg29;
#[doc = "SWREG30 (rw) register accessor: register for deinterlace ctrl\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg30`]
module"]
#[doc(alias = "SWREG30")]
pub type Swreg30 = crate::Reg<swreg30::Swreg30Spec>;
#[doc = "register for deinterlace ctrl"]
pub mod swreg30;
#[doc = "SWREG31 (rw) register accessor: contrast adjust threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg31`]
module"]
#[doc(alias = "SWREG31")]
pub type Swreg31 = crate::Reg<swreg31::Swreg31Spec>;
#[doc = "contrast adjust threshold"]
pub mod swreg31;
#[doc = "SWREG32 (rw) register accessor: contrast adjust offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg32`]
module"]
#[doc(alias = "SWREG32")]
pub type Swreg32 = crate::Reg<swreg32::Swreg32Spec>;
#[doc = "contrast adjust offset"]
pub mod swreg32;
#[doc = "SWREG33 (r) register accessor: Synthesis configuration register post-processor (read only)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg33::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg33`]
module"]
#[doc(alias = "SWREG33")]
pub type Swreg33 = crate::Reg<swreg33::Swreg33Spec>;
#[doc = "Synthesis configuration register post-processor (read only)"]
pub mod swreg33;
#[doc = "SWREG34 (rw) register accessor: PP input pic size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg34`]
module"]
#[doc(alias = "SWREG34")]
pub type Swreg34 = crate::Reg<swreg34::Swreg34Spec>;
#[doc = "PP input pic size register"]
pub mod swreg34;
#[doc = "SWREG35 (rw) register accessor: PP output pic size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg35`]
module"]
#[doc(alias = "SWREG35")]
pub type Swreg35 = crate::Reg<swreg35::Swreg35Spec>;
#[doc = "PP output pic size register"]
pub mod swreg35;
#[doc = "SWREG36 (rw) register accessor: the dither mode for RGB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg36`]
module"]
#[doc(alias = "SWREG36")]
pub type Swreg36 = crate::Reg<swreg36::Swreg36Spec>;
#[doc = "the dither mode for RGB"]
pub mod swreg36;
#[doc = "SWREG37 (rw) register accessor: PP input/output data format\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg37`]
module"]
#[doc(alias = "SWREG37")]
pub type Swreg37 = crate::Reg<swreg37::Swreg37Spec>;
#[doc = "PP input/output data format"]
pub mod swreg37;
#[doc = "SWREG38 (rw) register accessor: PP input/output data format\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg38`]
module"]
#[doc(alias = "SWREG38")]
pub type Swreg38 = crate::Reg<swreg38::Swreg38Spec>;
#[doc = "PP input/output data format"]
pub mod swreg38;
#[doc = "SWREG39 (rw) register accessor: Register0000 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg39`]
module"]
#[doc(alias = "SWREG39")]
pub type Swreg39 = crate::Reg<swreg39::Swreg39Spec>;
#[doc = "Register0000 Abstract"]
pub mod swreg39;
#[doc = "SWREG40 (rw) register accessor: pp int register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg40`]
module"]
#[doc(alias = "SWREG40")]
pub type Swreg40 = crate::Reg<swreg40::Swreg40Spec>;
#[doc = "pp int register"]
pub mod swreg40;
#[doc = "SWREG41 (rw) register accessor: enable ctrl flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg41`]
module"]
#[doc(alias = "SWREG41")]
pub type Swreg41 = crate::Reg<swreg41::Swreg41Spec>;
#[doc = "enable ctrl flag"]
pub mod swreg41;
#[doc = "SWREG50 (rw) register accessor: video decoder ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg50::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg50`]
module"]
#[doc(alias = "SWREG50")]
pub type Swreg50 = crate::Reg<swreg50::Swreg50Spec>;
#[doc = "video decoder ctrl register"]
pub mod swreg50;
#[doc = "SWREG51 (rw) register accessor: the stream length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg51`]
module"]
#[doc(alias = "SWREG51")]
pub type Swreg51 = crate::Reg<swreg51::Swreg51Spec>;
#[doc = "the stream length"]
pub mod swreg51;
#[doc = "SWREG52 (rw) register accessor: error concealment case related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg52`]
module"]
#[doc(alias = "SWREG52")]
pub type Swreg52 = crate::Reg<swreg52::Swreg52Spec>;
#[doc = "error concealment case related"]
pub mod swreg52;
#[doc = "SWREG53 (rw) register accessor: decoder format\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg53`]
module"]
#[doc(alias = "SWREG53")]
pub type Swreg53 = crate::Reg<swreg53::Swreg53Spec>;
#[doc = "decoder format"]
pub mod swreg53;
#[doc = "SWREG54 (rw) register accessor: endian for input/output data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg54::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg54`]
module"]
#[doc(alias = "SWREG54")]
pub type Swreg54 = crate::Reg<swreg54::Swreg54Spec>;
#[doc = "endian for input/output data"]
pub mod swreg54;
#[doc = "SWREG55 (rw) register accessor: decoder int register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg55`]
module"]
#[doc(alias = "SWREG55")]
pub type Swreg55 = crate::Reg<swreg55::Swreg55Spec>;
#[doc = "decoder int register"]
pub mod swreg55;
#[doc = "SWREG56 (rw) register accessor: axi ctrl for decoder\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg56::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg56`]
module"]
#[doc(alias = "SWREG56")]
pub type Swreg56 = crate::Reg<swreg56::Swreg56Spec>;
#[doc = "axi ctrl for decoder"]
pub mod swreg56;
#[doc = "SWREG57 (rw) register accessor: enable flag for decoder\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg57::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg57::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg57`]
module"]
#[doc(alias = "SWREG57")]
pub type Swreg57 = crate::Reg<swreg57::Swreg57Spec>;
#[doc = "enable flag for decoder"]
pub mod swreg57;
#[doc = "SWREG58 (rw) register accessor: soft reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg58::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg58::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg58`]
module"]
#[doc(alias = "SWREG58")]
pub type Swreg58 = crate::Reg<swreg58::Swreg58Spec>;
#[doc = "soft reset register"]
pub mod swreg58;
#[doc = "SWREG59 (rw) register accessor: H264, MPEG4, VP6 Prediction filter tap\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg59::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg59`]
module"]
#[doc(alias = "SWREG59")]
pub type Swreg59 = crate::Reg<swreg59::Swreg59Spec>;
#[doc = "H264, MPEG4, VP6 Prediction filter tap"]
pub mod swreg59;
#[doc = "SWREG60 (rw) register accessor: additional chrominance address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg60::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg60`]
module"]
#[doc(alias = "SWREG60")]
pub type Swreg60 = crate::Reg<swreg60::Swreg60Spec>;
#[doc = "additional chrominance address"]
pub mod swreg60;
#[doc = "SWREG61 (rw) register accessor: standard dependent tables start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg61::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg61::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg61`]
module"]
#[doc(alias = "SWREG61")]
pub type Swreg61 = crate::Reg<swreg61::Swreg61Spec>;
#[doc = "standard dependent tables start address"]
pub mod swreg61;
#[doc = "SWREG62 (rw) register accessor: Direct mode motion vector write/read start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg62::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg62::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg62`]
module"]
#[doc(alias = "SWREG62")]
pub type Swreg62 = crate::Reg<swreg62::Swreg62Spec>;
#[doc = "Direct mode motion vector write/read start address"]
pub mod swreg62;
#[doc = "SWREG63 (rw) register accessor: write decoder output picture or field start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg63::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg63`]
module"]
#[doc(alias = "SWREG63")]
pub type Swreg63 = crate::Reg<swreg63::Swreg63Spec>;
#[doc = "write decoder output picture or field start address"]
pub mod swreg63;
#[doc = "SWREG64 (rw) register accessor: rlc or vlc mode input data start addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg64`]
module"]
#[doc(alias = "SWREG64")]
pub type Swreg64 = crate::Reg<swreg64::Swreg64Spec>;
#[doc = "rlc or vlc mode input data start addr"]
pub mod swreg64;
#[doc = "SWREG65 (rw) register accessor: refbufferd related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg65::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg65::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg65`]
module"]
#[doc(alias = "SWREG65")]
pub type Swreg65 = crate::Reg<swreg65::Swreg65Spec>;
#[doc = "refbufferd related"]
pub mod swreg65;
#[doc = "SWREG66 (r) register accessor: ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg66::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg66`]
module"]
#[doc(alias = "SWREG66")]
pub type Swreg66 = crate::Reg<swreg66::Swreg66Spec>;
#[doc = "ID register"]
pub mod swreg66;
#[doc = "SWREG67 (r) register accessor: Synthesis configuration register decoder 1(read only)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg67::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg67`]
module"]
#[doc(alias = "SWREG67")]
pub type Swreg67 = crate::Reg<swreg67::Swreg67Spec>;
#[doc = "Synthesis configuration register decoder 1(read only)"]
pub mod swreg67;
#[doc = "SWREG68 (r) register accessor: sum of partitions(read only)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg68::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg68`]
module"]
#[doc(alias = "SWREG68")]
pub type Swreg68 = crate::Reg<swreg68::Swreg68Spec>;
#[doc = "sum of partitions(read only)"]
pub mod swreg68;
#[doc = "SWREG69 (r) register accessor: sum information (read only)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg69::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg69`]
module"]
#[doc(alias = "SWREG69")]
pub type Swreg69 = crate::Reg<swreg69::Swreg69Spec>;
#[doc = "sum information (read only)"]
pub mod swreg69;
#[doc = "SWREG70 (r) register accessor: sum of the decoded motion vector y-components(read only)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg70::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg70`]
module"]
#[doc(alias = "SWREG70")]
pub type Swreg70 = crate::Reg<swreg70::Swreg70Spec>;
#[doc = "sum of the decoded motion vector y-components(read only)"]
pub mod swreg70;
#[doc = "SWREG71 (r) register accessor: information for read only register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg71::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg71`]
module"]
#[doc(alias = "SWREG71")]
pub type Swreg71 = crate::Reg<swreg71::Swreg71Spec>;
#[doc = "information for read only register"]
pub mod swreg71;
#[doc = "SWREG72 (r) register accessor: debug0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg72::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg72`]
module"]
#[doc(alias = "SWREG72")]
pub type Swreg72 = crate::Reg<swreg72::Swreg72Spec>;
#[doc = "debug0"]
pub mod swreg72;
#[doc = "SWREG73 (r) register accessor: debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg73::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg73`]
module"]
#[doc(alias = "SWREG73")]
pub type Swreg73 = crate::Reg<swreg73::Swreg73Spec>;
#[doc = "debug registers"]
pub mod swreg73;
#[doc = "SWREG74 (rw) register accessor: MV address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg74::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg74::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg74`]
module"]
#[doc(alias = "SWREG74")]
pub type Swreg74 = crate::Reg<swreg74::Swreg74Spec>;
#[doc = "MV address for h264"]
pub mod swreg74;
#[doc = "SWREG75 (rw) register accessor: H.264 Intra prediction 4x4 mode start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg75::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg75::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg75`]
module"]
#[doc(alias = "SWREG75")]
pub type Swreg75 = crate::Reg<swreg75::Swreg75Spec>;
#[doc = "H.264 Intra prediction 4x4 mode start address"]
pub mod swreg75;
#[doc = "SWREG76 (rw) register accessor: the number of referance pic\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg76::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg76::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg76`]
module"]
#[doc(alias = "SWREG76")]
pub type Swreg76 = crate::Reg<swreg76::Swreg76Spec>;
#[doc = "the number of referance pic"]
pub mod swreg76;
#[doc = "SWREG77 (rw) register accessor: the number of referance pic\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg77::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg77::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg77`]
module"]
#[doc(alias = "SWREG77")]
pub type Swreg77 = crate::Reg<swreg77::Swreg77Spec>;
#[doc = "the number of referance pic"]
pub mod swreg77;
#[doc = "SWREG78 (rw) register accessor: the number of referance pic\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg78::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg78::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg78`]
module"]
#[doc(alias = "SWREG78")]
pub type Swreg78 = crate::Reg<swreg78::Swreg78Spec>;
#[doc = "the number of referance pic"]
pub mod swreg78;
#[doc = "SWREG79 (rw) register accessor: the number of referance pic\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg79::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg79::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg79`]
module"]
#[doc(alias = "SWREG79")]
pub type Swreg79 = crate::Reg<swreg79::Swreg79Spec>;
#[doc = "the number of referance pic"]
pub mod swreg79;
#[doc = "SWREG80 (rw) register accessor: the number of referance pic\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg80::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg80::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg80`]
module"]
#[doc(alias = "SWREG80")]
pub type Swreg80 = crate::Reg<swreg80::Swreg80Spec>;
#[doc = "the number of referance pic"]
pub mod swreg80;
#[doc = "SWREG81 (rw) register accessor: the number of referance pic\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg81::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg81::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg81`]
module"]
#[doc(alias = "SWREG81")]
pub type Swreg81 = crate::Reg<swreg81::Swreg81Spec>;
#[doc = "the number of referance pic"]
pub mod swreg81;
#[doc = "SWREG82 (rw) register accessor: the number of referance pic\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg82::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg82::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg82`]
module"]
#[doc(alias = "SWREG82")]
pub type Swreg82 = crate::Reg<swreg82::Swreg82Spec>;
#[doc = "the number of referance pic"]
pub mod swreg82;
#[doc = "SWREG83 (rw) register accessor: the number of referance pic\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg83::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg83::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg83`]
module"]
#[doc(alias = "SWREG83")]
pub type Swreg83 = crate::Reg<swreg83::Swreg83Spec>;
#[doc = "the number of referance pic"]
pub mod swreg83;
#[doc = "SWREG84 (rw) register accessor: referance frame0 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg84::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg84::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg84`]
module"]
#[doc(alias = "SWREG84")]
pub type Swreg84 = crate::Reg<swreg84::Swreg84Spec>;
#[doc = "referance frame0 address for h264"]
pub mod swreg84;
#[doc = "SWREG85 (rw) register accessor: referance frame1 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg85::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg85::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg85`]
module"]
#[doc(alias = "SWREG85")]
pub type Swreg85 = crate::Reg<swreg85::Swreg85Spec>;
#[doc = "referance frame1 address for h264"]
pub mod swreg85;
#[doc = "SWREG86 (rw) register accessor: referance frame2 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg86::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg86::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg86`]
module"]
#[doc(alias = "SWREG86")]
pub type Swreg86 = crate::Reg<swreg86::Swreg86Spec>;
#[doc = "referance frame2 address for h264"]
pub mod swreg86;
#[doc = "SWREG87 (rw) register accessor: referance frame3 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg87::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg87::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg87`]
module"]
#[doc(alias = "SWREG87")]
pub type Swreg87 = crate::Reg<swreg87::Swreg87Spec>;
#[doc = "referance frame3 address for h264"]
pub mod swreg87;
#[doc = "SWREG88 (rw) register accessor: referance frame4 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg88::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg88::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg88`]
module"]
#[doc(alias = "SWREG88")]
pub type Swreg88 = crate::Reg<swreg88::Swreg88Spec>;
#[doc = "referance frame4 address for h264"]
pub mod swreg88;
#[doc = "SWREG89 (rw) register accessor: referance frame5 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg89::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg89::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg89`]
module"]
#[doc(alias = "SWREG89")]
pub type Swreg89 = crate::Reg<swreg89::Swreg89Spec>;
#[doc = "referance frame5 address for h264"]
pub mod swreg89;
#[doc = "SWREG90 (rw) register accessor: referance frame6 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg90::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg90::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg90`]
module"]
#[doc(alias = "SWREG90")]
pub type Swreg90 = crate::Reg<swreg90::Swreg90Spec>;
#[doc = "referance frame6 address for h264"]
pub mod swreg90;
#[doc = "SWREG91 (rw) register accessor: referance frame7 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg91::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg91::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg91`]
module"]
#[doc(alias = "SWREG91")]
pub type Swreg91 = crate::Reg<swreg91::Swreg91Spec>;
#[doc = "referance frame7 address for h264"]
pub mod swreg91;
#[doc = "SWREG92 (rw) register accessor: referance frame8 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg92::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg92::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg92`]
module"]
#[doc(alias = "SWREG92")]
pub type Swreg92 = crate::Reg<swreg92::Swreg92Spec>;
#[doc = "referance frame8 address for h264"]
pub mod swreg92;
#[doc = "SWREG93 (rw) register accessor: referance frame9 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg93::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg93::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg93`]
module"]
#[doc(alias = "SWREG93")]
pub type Swreg93 = crate::Reg<swreg93::Swreg93Spec>;
#[doc = "referance frame9 address for h264"]
pub mod swreg93;
#[doc = "SWREG94 (rw) register accessor: referance frame10 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg94::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg94::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg94`]
module"]
#[doc(alias = "SWREG94")]
pub type Swreg94 = crate::Reg<swreg94::Swreg94Spec>;
#[doc = "referance frame10 address for h264"]
pub mod swreg94;
#[doc = "SWREG95 (rw) register accessor: referance frame11 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg95::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg95::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg95`]
module"]
#[doc(alias = "SWREG95")]
pub type Swreg95 = crate::Reg<swreg95::Swreg95Spec>;
#[doc = "referance frame11 address for h264"]
pub mod swreg95;
#[doc = "SWREG96 (rw) register accessor: referance frame12 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg96::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg96::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg96`]
module"]
#[doc(alias = "SWREG96")]
pub type Swreg96 = crate::Reg<swreg96::Swreg96Spec>;
#[doc = "referance frame12 address for h264"]
pub mod swreg96;
#[doc = "SWREG97 (rw) register accessor: referance frame13 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg97::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg97::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg97`]
module"]
#[doc(alias = "SWREG97")]
pub type Swreg97 = crate::Reg<swreg97::Swreg97Spec>;
#[doc = "referance frame13 address for h264"]
pub mod swreg97;
#[doc = "SWREG98 (rw) register accessor: referance frame14 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg98::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg98::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg98`]
module"]
#[doc(alias = "SWREG98")]
pub type Swreg98 = crate::Reg<swreg98::Swreg98Spec>;
#[doc = "referance frame14 address for h264"]
pub mod swreg98;
#[doc = "SWREG99 (rw) register accessor: referance frame15 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg99::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg99::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg99`]
module"]
#[doc(alias = "SWREG99")]
pub type Swreg99 = crate::Reg<swreg99::Swreg99Spec>;
#[doc = "referance frame15 address for h264"]
pub mod swreg99;
#[doc = "SWREG100 (rw) register accessor: initial reference picture list related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg100::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg100::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg100`]
module"]
#[doc(alias = "SWREG100")]
pub type Swreg100 = crate::Reg<swreg100::Swreg100Spec>;
#[doc = "initial reference picture list related"]
pub mod swreg100;
#[doc = "SWREG101 (rw) register accessor: initial reference picture list related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg101::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg101::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg101`]
module"]
#[doc(alias = "SWREG101")]
pub type Swreg101 = crate::Reg<swreg101::Swreg101Spec>;
#[doc = "initial reference picture list related"]
pub mod swreg101;
#[doc = "SWREG102 (rw) register accessor: initial reference picture list related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg102::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg102::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg102`]
module"]
#[doc(alias = "SWREG102")]
pub type Swreg102 = crate::Reg<swreg102::Swreg102Spec>;
#[doc = "initial reference picture list related"]
pub mod swreg102;
#[doc = "SWREG103 (rw) register accessor: initial reference picture list related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg103::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg103::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg103`]
module"]
#[doc(alias = "SWREG103")]
pub type Swreg103 = crate::Reg<swreg103::Swreg103Spec>;
#[doc = "initial reference picture list related"]
pub mod swreg103;
#[doc = "SWREG104 (rw) register accessor: initial reference picture list related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg104::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg104::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg104`]
module"]
#[doc(alias = "SWREG104")]
pub type Swreg104 = crate::Reg<swreg104::Swreg104Spec>;
#[doc = "initial reference picture list related"]
pub mod swreg104;
#[doc = "SWREG105 (rw) register accessor: initial reference picture list related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg105::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg105::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg105`]
module"]
#[doc(alias = "SWREG105")]
pub type Swreg105 = crate::Reg<swreg105::Swreg105Spec>;
#[doc = "initial reference picture list related"]
pub mod swreg105;
#[doc = "SWREG106 (rw) register accessor: initial reference picture list related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg106::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg106::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg106`]
module"]
#[doc(alias = "SWREG106")]
pub type Swreg106 = crate::Reg<swreg106::Swreg106Spec>;
#[doc = "initial reference picture list related"]
pub mod swreg106;
#[doc = "SWREG107 (rw) register accessor: long term flag for reference pictuure index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg107::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg107::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg107`]
module"]
#[doc(alias = "SWREG107")]
pub type Swreg107 = crate::Reg<swreg107::Swreg107Spec>;
#[doc = "long term flag for reference pictuure index"]
pub mod swreg107;
#[doc = "SWREG108 (rw) register accessor: valid flag for reference picture index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg108::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg108::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg108`]
module"]
#[doc(alias = "SWREG108")]
pub type Swreg108 = crate::Reg<swreg108::Swreg108Spec>;
#[doc = "valid flag for reference picture index"]
pub mod swreg108;
#[doc = "SWREG109 (rw) register accessor: the stream start word for decoder\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg109::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg109::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg109`]
module"]
#[doc(alias = "SWREG109")]
pub type Swreg109 = crate::Reg<swreg109::Swreg109Spec>;
#[doc = "the stream start word for decoder"]
pub mod swreg109;
#[doc = "SWREG110 (rw) register accessor: h264 pic mb size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg110::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg110::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg110`]
module"]
#[doc(alias = "SWREG110")]
pub type Swreg110 = crate::Reg<swreg110::Swreg110Spec>;
#[doc = "h264 pic mb size"]
pub mod swreg110;
#[doc = "SWREG111 (rw) register accessor: h264 ctrl related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg111::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg111::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg111`]
module"]
#[doc(alias = "SWREG111")]
pub type Swreg111 = crate::Reg<swreg111::Swreg111Spec>;
#[doc = "h264 ctrl related"]
pub mod swreg111;
#[doc = "SWREG112 (rw) register accessor: current frame related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg112::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg112::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg112`]
module"]
#[doc(alias = "SWREG112")]
pub type Swreg112 = crate::Reg<swreg112::Swreg112Spec>;
#[doc = "current frame related"]
pub mod swreg112;
#[doc = "SWREG113 (rw) register accessor: reference picture related\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg113::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg113::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg113`]
module"]
#[doc(alias = "SWREG113")]
pub type Swreg113 = crate::Reg<swreg113::Swreg113Spec>;
#[doc = "reference picture related"]
pub mod swreg113;
#[doc = "SWREG114 (rw) register accessor: maximum reference\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg114::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg114::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg114`]
module"]
#[doc(alias = "SWREG114")]
pub type Swreg114 = crate::Reg<swreg114::Swreg114Spec>;
#[doc = "maximum reference"]
pub mod swreg114;
#[doc = "SWREG115 (rw) register accessor: enable flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg115::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg115::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg115`]
module"]
#[doc(alias = "SWREG115")]
pub type Swreg115 = crate::Reg<swreg115::Swreg115Spec>;
#[doc = "enable flag"]
pub mod swreg115;
#[doc = "SWREG120 (rw) register accessor: multi format reuse register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg120::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg120::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg120`]
module"]
#[doc(alias = "SWREG120")]
pub type Swreg120 = crate::Reg<swreg120::Swreg120Spec>;
#[doc = "multi format reuse register0"]
pub mod swreg120;
#[doc = "SWREG121 (rw) register accessor: multi format reuse register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg121::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg121::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg121`]
module"]
#[doc(alias = "SWREG121")]
pub type Swreg121 = crate::Reg<swreg121::Swreg121Spec>;
#[doc = "multi format reuse register1"]
pub mod swreg121;
#[doc = "SWREG122 (rw) register accessor: multi format reuse register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg122::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg122::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg122`]
module"]
#[doc(alias = "SWREG122")]
pub type Swreg122 = crate::Reg<swreg122::Swreg122Spec>;
#[doc = "multi format reuse register2"]
pub mod swreg122;
#[doc = "SWREG123 (rw) register accessor: multi format reuse register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg123::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg123::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg123`]
module"]
#[doc(alias = "SWREG123")]
pub type Swreg123 = crate::Reg<swreg123::Swreg123Spec>;
#[doc = "multi format reuse register3"]
pub mod swreg123;
#[doc = "SWREG124 (rw) register accessor: multi format reuse register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg124::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg124::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg124`]
module"]
#[doc(alias = "SWREG124")]
pub type Swreg124 = crate::Reg<swreg124::Swreg124Spec>;
#[doc = "multi format reuse register4"]
pub mod swreg124;
#[doc = "SWREG125 (rw) register accessor: multi format reuse register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg125::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg125::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg125`]
module"]
#[doc(alias = "SWREG125")]
pub type Swreg125 = crate::Reg<swreg125::Swreg125Spec>;
#[doc = "multi format reuse register5"]
pub mod swreg125;
#[doc = "SWREG126 (rw) register accessor: multi format reuse register6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg126::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg126::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg126`]
module"]
#[doc(alias = "SWREG126")]
pub type Swreg126 = crate::Reg<swreg126::Swreg126Spec>;
#[doc = "multi format reuse register6"]
pub mod swreg126;
#[doc = "SWREG127 (rw) register accessor: multi format reuse register7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg127::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg127::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg127`]
module"]
#[doc(alias = "SWREG127")]
pub type Swreg127 = crate::Reg<swreg127::Swreg127Spec>;
#[doc = "multi format reuse register7"]
pub mod swreg127;
#[doc = "SWREG128 (rw) register accessor: multi format reuse register8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg128::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg128::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg128`]
module"]
#[doc(alias = "SWREG128")]
pub type Swreg128 = crate::Reg<swreg128::Swreg128Spec>;
#[doc = "multi format reuse register8"]
pub mod swreg128;
#[doc = "SWREG129 (rw) register accessor: multi format reuse register9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg129::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg129::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg129`]
module"]
#[doc(alias = "SWREG129")]
pub type Swreg129 = crate::Reg<swreg129::Swreg129Spec>;
#[doc = "multi format reuse register9"]
pub mod swreg129;
#[doc = "SWREG130 (rw) register accessor: multi format reuse register10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg130::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg130::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg130`]
module"]
#[doc(alias = "SWREG130")]
pub type Swreg130 = crate::Reg<swreg130::Swreg130Spec>;
#[doc = "multi format reuse register10"]
pub mod swreg130;
#[doc = "SWREG131 (rw) register accessor: multi format reuse register11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg131::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg131::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg131`]
module"]
#[doc(alias = "SWREG131")]
pub type Swreg131 = crate::Reg<swreg131::Swreg131Spec>;
#[doc = "multi format reuse register11"]
pub mod swreg131;
#[doc = "SWREG132 (rw) register accessor: multi format reuse register12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg132::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg132::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg132`]
module"]
#[doc(alias = "SWREG132")]
pub type Swreg132 = crate::Reg<swreg132::Swreg132Spec>;
#[doc = "multi format reuse register12"]
pub mod swreg132;
#[doc = "SWREG133 (rw) register accessor: multi format reuse register13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg133::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg133::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg133`]
module"]
#[doc(alias = "SWREG133")]
pub type Swreg133 = crate::Reg<swreg133::Swreg133Spec>;
#[doc = "multi format reuse register13"]
pub mod swreg133;
#[doc = "SWREG134 (rw) register accessor: multi format reuse register14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg134::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg134::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg134`]
module"]
#[doc(alias = "SWREG134")]
pub type Swreg134 = crate::Reg<swreg134::Swreg134Spec>;
#[doc = "multi format reuse register14"]
pub mod swreg134;
#[doc = "SWREG135 (rw) register accessor: multi format reuse register15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg135::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg135::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg135`]
module"]
#[doc(alias = "SWREG135")]
pub type Swreg135 = crate::Reg<swreg135::Swreg135Spec>;
#[doc = "multi format reuse register15"]
pub mod swreg135;
#[doc = "SWREG136 (rw) register accessor: multi format reuse register16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg136::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg136::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg136`]
module"]
#[doc(alias = "SWREG136")]
pub type Swreg136 = crate::Reg<swreg136::Swreg136Spec>;
#[doc = "multi format reuse register16"]
pub mod swreg136;
#[doc = "SWREG137 (rw) register accessor: multi format reuse register17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg137::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg137::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg137`]
module"]
#[doc(alias = "SWREG137")]
pub type Swreg137 = crate::Reg<swreg137::Swreg137Spec>;
#[doc = "multi format reuse register17"]
pub mod swreg137;
#[doc = "SWREG138 (rw) register accessor: multi format reuse register18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg138::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg138::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg138`]
module"]
#[doc(alias = "SWREG138")]
pub type Swreg138 = crate::Reg<swreg138::Swreg138Spec>;
#[doc = "multi format reuse register18"]
pub mod swreg138;
#[doc = "SWREG139 (rw) register accessor: multi format reuse register19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg139::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg139::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg139`]
module"]
#[doc(alias = "SWREG139")]
pub type Swreg139 = crate::Reg<swreg139::Swreg139Spec>;
#[doc = "multi format reuse register19"]
pub mod swreg139;
#[doc = "SWREG140 (rw) register accessor: multi format reuse register20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg140::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg140::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg140`]
module"]
#[doc(alias = "SWREG140")]
pub type Swreg140 = crate::Reg<swreg140::Swreg140Spec>;
#[doc = "multi format reuse register20"]
pub mod swreg140;
#[doc = "SWREG141 (rw) register accessor: multi format reuse register21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg141::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg141::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg141`]
module"]
#[doc(alias = "SWREG141")]
pub type Swreg141 = crate::Reg<swreg141::Swreg141Spec>;
#[doc = "multi format reuse register21"]
pub mod swreg141;
#[doc = "SWREG142 (rw) register accessor: multi format reuse register22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg142::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg142::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg142`]
module"]
#[doc(alias = "SWREG142")]
pub type Swreg142 = crate::Reg<swreg142::Swreg142Spec>;
#[doc = "multi format reuse register22"]
pub mod swreg142;
#[doc = "SWREG143 (rw) register accessor: multi format reuse register23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg143::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg143::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg143`]
module"]
#[doc(alias = "SWREG143")]
pub type Swreg143 = crate::Reg<swreg143::Swreg143Spec>;
#[doc = "multi format reuse register23"]
pub mod swreg143;
#[doc = "SWREG144 (rw) register accessor: multi format reuse register24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg144::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg144::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg144`]
module"]
#[doc(alias = "SWREG144")]
pub type Swreg144 = crate::Reg<swreg144::Swreg144Spec>;
#[doc = "multi format reuse register24"]
pub mod swreg144;
#[doc = "SWREG145 (rw) register accessor: multi format reuse register25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg145::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg145::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg145`]
module"]
#[doc(alias = "SWREG145")]
pub type Swreg145 = crate::Reg<swreg145::Swreg145Spec>;
#[doc = "multi format reuse register25"]
pub mod swreg145;
#[doc = "SWREG146 (rw) register accessor: multi format reuse register26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg146::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg146::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg146`]
module"]
#[doc(alias = "SWREG146")]
pub type Swreg146 = crate::Reg<swreg146::Swreg146Spec>;
#[doc = "multi format reuse register26"]
pub mod swreg146;
#[doc = "SWREG147 (rw) register accessor: multi format reuse register27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg147::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg147::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg147`]
module"]
#[doc(alias = "SWREG147")]
pub type Swreg147 = crate::Reg<swreg147::Swreg147Spec>;
#[doc = "multi format reuse register27"]
pub mod swreg147;
#[doc = "SWREG148 (rw) register accessor: multi format reuse register28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg148::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg148::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg148`]
module"]
#[doc(alias = "SWREG148")]
pub type Swreg148 = crate::Reg<swreg148::Swreg148Spec>;
#[doc = "multi format reuse register28"]
pub mod swreg148;
#[doc = "SWREG149 (rw) register accessor: multi format reuse register29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg149::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg149::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg149`]
module"]
#[doc(alias = "SWREG149")]
pub type Swreg149 = crate::Reg<swreg149::Swreg149Spec>;
#[doc = "multi format reuse register29"]
pub mod swreg149;
#[doc = "SWREG150 (rw) register accessor: multi format reuse register30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg150::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg150::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg150`]
module"]
#[doc(alias = "SWREG150")]
pub type Swreg150 = crate::Reg<swreg150::Swreg150Spec>;
#[doc = "multi format reuse register30"]
pub mod swreg150;
#[doc = "SWREG151 (rw) register accessor: multi format reuse register31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg151::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg151::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg151`]
module"]
#[doc(alias = "SWREG151")]
pub type Swreg151 = crate::Reg<swreg151::Swreg151Spec>;
#[doc = "multi format reuse register31"]
pub mod swreg151;
#[doc = "SWREG152 (rw) register accessor: multi format reuse register32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg152::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg152::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg152`]
module"]
#[doc(alias = "SWREG152")]
pub type Swreg152 = crate::Reg<swreg152::Swreg152Spec>;
#[doc = "multi format reuse register32"]
pub mod swreg152;
#[doc = "SWREG153 (rw) register accessor: multi format reuse register33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg153::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg153::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg153`]
module"]
#[doc(alias = "SWREG153")]
pub type Swreg153 = crate::Reg<swreg153::Swreg153Spec>;
#[doc = "multi format reuse register33"]
pub mod swreg153;
#[doc = "SWREG154 (rw) register accessor: multi format reuse register34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg154::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg154::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg154`]
module"]
#[doc(alias = "SWREG154")]
pub type Swreg154 = crate::Reg<swreg154::Swreg154Spec>;
#[doc = "multi format reuse register34"]
pub mod swreg154;
#[doc = "SWREG155 (rw) register accessor: multi format reuse register35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg155::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg155::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg155`]
module"]
#[doc(alias = "SWREG155")]
pub type Swreg155 = crate::Reg<swreg155::Swreg155Spec>;
#[doc = "multi format reuse register35"]
pub mod swreg155;
#[doc = "SWREG156 (rw) register accessor: multi format reuse register36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg156::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg156::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg156`]
module"]
#[doc(alias = "SWREG156")]
pub type Swreg156 = crate::Reg<swreg156::Swreg156Spec>;
#[doc = "multi format reuse register36"]
pub mod swreg156;
#[doc = "SWREG157 (rw) register accessor: multi format reuse register37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg157::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg157::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg157`]
module"]
#[doc(alias = "SWREG157")]
pub type Swreg157 = crate::Reg<swreg157::Swreg157Spec>;
#[doc = "multi format reuse register37"]
pub mod swreg157;
#[doc = "SWREG158 (rw) register accessor: multi format reuse register38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg158::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg158::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg158`]
module"]
#[doc(alias = "SWREG158")]
pub type Swreg158 = crate::Reg<swreg158::Swreg158Spec>;
#[doc = "multi format reuse register38"]
pub mod swreg158;
#[doc = "SWREG164_PERF_LATENCY_CTRL0 (rw) register accessor: Axi performance latency module contrl register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg164_perf_latency_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg164_perf_latency_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg164_perf_latency_ctrl0`]
module"]
#[doc(alias = "SWREG164_PERF_LATENCY_CTRL0")]
pub type Swreg164PerfLatencyCtrl0 =
    crate::Reg<swreg164_perf_latency_ctrl0::Swreg164PerfLatencyCtrl0Spec>;
#[doc = "Axi performance latency module contrl register0"]
pub mod swreg164_perf_latency_ctrl0;
#[doc = "SWREG165_PERF_LATENCY_CTRL1 (rw) register accessor: PERF_LATENCY_CTRL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg165_perf_latency_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg165_perf_latency_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg165_perf_latency_ctrl1`]
module"]
#[doc(alias = "SWREG165_PERF_LATENCY_CTRL1")]
pub type Swreg165PerfLatencyCtrl1 =
    crate::Reg<swreg165_perf_latency_ctrl1::Swreg165PerfLatencyCtrl1Spec>;
#[doc = "PERF_LATENCY_CTRL1"]
pub mod swreg165_perf_latency_ctrl1;
#[doc = "SWREG166_PERF_RD_MAX_LATENCY_NUM0 (r) register accessor: Read max latency number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg166_perf_rd_max_latency_num0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg166_perf_rd_max_latency_num0`]
module"]
#[doc(alias = "SWREG166_PERF_RD_MAX_LATENCY_NUM0")]
pub type Swreg166PerfRdMaxLatencyNum0 =
    crate::Reg<swreg166_perf_rd_max_latency_num0::Swreg166PerfRdMaxLatencyNum0Spec>;
#[doc = "Read max latency number"]
pub mod swreg166_perf_rd_max_latency_num0;
#[doc = "SWREG167_PERF_RD_LATENCY_SAMP_NUM (r) register accessor: The number of bigger than configed threshold value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg167_perf_rd_latency_samp_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg167_perf_rd_latency_samp_num`]
module"]
#[doc(alias = "SWREG167_PERF_RD_LATENCY_SAMP_NUM")]
pub type Swreg167PerfRdLatencySampNum =
    crate::Reg<swreg167_perf_rd_latency_samp_num::Swreg167PerfRdLatencySampNumSpec>;
#[doc = "The number of bigger than configed threshold value"]
pub mod swreg167_perf_rd_latency_samp_num;
#[doc = "SWREG168_PERF_RD_LATENCY_ACC_SUM (r) register accessor: Total sample number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg168_perf_rd_latency_acc_sum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg168_perf_rd_latency_acc_sum`]
module"]
#[doc(alias = "SWREG168_PERF_RD_LATENCY_ACC_SUM")]
pub type Swreg168PerfRdLatencyAccSum =
    crate::Reg<swreg168_perf_rd_latency_acc_sum::Swreg168PerfRdLatencyAccSumSpec>;
#[doc = "Total sample number"]
pub mod swreg168_perf_rd_latency_acc_sum;
#[doc = "SWREG169_PERF_RD_AXI_TOTAL_BYTE (rw) register accessor: perf_rd_axi_total_byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg169_perf_rd_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg169_perf_rd_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg169_perf_rd_axi_total_byte`]
module"]
#[doc(alias = "SWREG169_PERF_RD_AXI_TOTAL_BYTE")]
pub type Swreg169PerfRdAxiTotalByte =
    crate::Reg<swreg169_perf_rd_axi_total_byte::Swreg169PerfRdAxiTotalByteSpec>;
#[doc = "perf_rd_axi_total_byte"]
pub mod swreg169_perf_rd_axi_total_byte;
#[doc = "SWREG170_PERF_WR_AXI_TOTAL_BYTE (rw) register accessor: perf_wr_axi_total_byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg170_perf_wr_axi_total_byte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg170_perf_wr_axi_total_byte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg170_perf_wr_axi_total_byte`]
module"]
#[doc(alias = "SWREG170_PERF_WR_AXI_TOTAL_BYTE")]
pub type Swreg170PerfWrAxiTotalByte =
    crate::Reg<swreg170_perf_wr_axi_total_byte::Swreg170PerfWrAxiTotalByteSpec>;
#[doc = "perf_wr_axi_total_byte"]
pub mod swreg170_perf_wr_axi_total_byte;
#[doc = "SWREG171_PERF_WORKING_CNT (rw) register accessor: perf_working_cnt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg171_perf_working_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg171_perf_working_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreg171_perf_working_cnt`]
module"]
#[doc(alias = "SWREG171_PERF_WORKING_CNT")]
pub type Swreg171PerfWorkingCnt = crate::Reg<swreg171_perf_working_cnt::Swreg171PerfWorkingCntSpec>;
#[doc = "perf_working_cnt"]
pub mod swreg171_perf_working_cnt;
