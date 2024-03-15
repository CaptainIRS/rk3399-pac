#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pi_reg_0: PiReg0,
    pi_reg_1: PiReg1,
    pi_reg_2: PiReg2,
    pi_reg_3: PiReg3,
    pi_reg_4: PiReg4,
    pi_reg_5: PiReg5,
    pi_reg_6: PiReg6,
    pi_reg_7: PiReg7,
    pi_reg_8: PiReg8,
    pi_reg_9: PiReg9,
    pi_reg_10: PiReg10,
    pi_reg_11: PiReg11,
    pi_reg_12: PiReg12,
    pi_reg_13: PiReg13,
    pi_reg_14: PiReg14,
    pi_reg_15: PiReg15,
    pi_reg_16: PiReg16,
    pi_reg_17: PiReg17,
    pi_reg_18: PiReg18,
    pi_reg_19: PiReg19,
    pi_reg_20: PiReg20,
    pi_reg_21: PiReg21,
    pi_reg_22: PiReg22,
    pi_reg_23: PiReg23,
    pi_reg_24: PiReg24,
    pi_reg_25: PiReg25,
    pi_reg_26: PiReg26,
    pi_reg_27: PiReg27,
    pi_reg_28: PiReg28,
    pi_reg_29: PiReg29,
    pi_reg_30: PiReg30,
    pi_reg_31: PiReg31,
    pi_reg_32: PiReg32,
    pi_reg_33: PiReg33,
    pi_reg_34: PiReg34,
    pi_reg_35: PiReg35,
    pi_reg_36: PiReg36,
    pi_reg_37: PiReg37,
    pi_reg_38: PiReg38,
    pi_reg_39: PiReg39,
    pi_reg_40: PiReg40,
    pi_reg_41: PiReg41,
    pi_reg_42: PiReg42,
    pi_reg_43: PiReg43,
    pi_reg_44: PiReg44,
    pi_reg_45: PiReg45,
    pi_reg_46: PiReg46,
    pi_reg_47: PiReg47,
    pi_reg_48: PiReg48,
    pi_reg_49: PiReg49,
    pi_reg_50: PiReg50,
    pi_reg_51: PiReg51,
    pi_reg_52: PiReg52,
    pi_reg_53: PiReg53,
    pi_reg_54: PiReg54,
    pi_reg_55: PiReg55,
    pi_reg_56: PiReg56,
    pi_reg_57: PiReg57,
    pi_reg_58: PiReg58,
    pi_reg_59: PiReg59,
    pi_reg_60: PiReg60,
    pi_reg_61: PiReg61,
    pi_reg_62: PiReg62,
    pi_reg_63: PiReg63,
    pi_reg_64: PiReg64,
    pi_reg_65: PiReg65,
    pi_reg_66: PiReg66,
    pi_reg_67: PiReg67,
    pi_reg_68: PiReg68,
    pi_reg_69: PiReg69,
    pi_reg_70: PiReg70,
    pi_reg_71: PiReg71,
    pi_reg_72: PiReg72,
    pi_reg_73: PiReg73,
    pi_reg_74: PiReg74,
    pi_reg_75: PiReg75,
    pi_reg_76: PiReg76,
    pi_reg_77: PiReg77,
    pi_reg_78: PiReg78,
    pi_reg_79: PiReg79,
    pi_reg_80: PiReg80,
    pi_reg_81: PiReg81,
    pi_reg_82: PiReg82,
    pi_reg_83: PiReg83,
    pi_reg_84: PiReg84,
    pi_reg_85: PiReg85,
    pi_reg_86: PiReg86,
    pi_reg_87: PiReg87,
    pi_reg_88: PiReg88,
    pi_reg_89: PiReg89,
    pi_reg_90: PiReg90,
    pi_reg_91: PiReg91,
    pi_reg_92: PiReg92,
    pi_reg_93: PiReg93,
    pi_reg_94: PiReg94,
    pi_reg_95: PiReg95,
    pi_reg_96: PiReg96,
    pi_reg_97: PiReg97,
    pi_reg_98: PiReg98,
    pi_reg_99: PiReg99,
    pi_reg_100: PiReg100,
    pi_reg_101: PiReg101,
    pi_reg_102: PiReg102,
    pi_reg_103: PiReg103,
    pi_reg_104: PiReg104,
    pi_reg_105: PiReg105,
    pi_reg_106: PiReg106,
    pi_reg_107: PiReg107,
    pi_reg_108: PiReg108,
    pi_reg_109: PiReg109,
    pi_reg_110: PiReg110,
    pi_reg_111: PiReg111,
    pi_reg_112: PiReg112,
    pi_reg_113: PiReg113,
    pi_reg_114: PiReg114,
    pi_reg_115: PiReg115,
    pi_reg_116: PiReg116,
    pi_reg_117: PiReg117,
    pi_reg_118: PiReg118,
    pi_reg_119: PiReg119,
    pi_reg_120: PiReg120,
    pi_reg_121: PiReg121,
    pi_reg_122: PiReg122,
    pi_reg_123: PiReg123,
    pi_reg_124: PiReg124,
    pi_reg_125: PiReg125,
    pi_reg_126: PiReg126,
    pi_reg_127: PiReg127,
    pi_reg_128: PiReg128,
    pi_reg_129: PiReg129,
    pi_reg_130: PiReg130,
    pi_reg_131: PiReg131,
    pi_reg_132: PiReg132,
    pi_reg_133: PiReg133,
    pi_reg_134: PiReg134,
    pi_reg_135: PiReg135,
    pi_reg_136: PiReg136,
    pi_reg_137: PiReg137,
    pi_reg_138: PiReg138,
    pi_reg_139: PiReg139,
    pi_reg_140: PiReg140,
    _reserved141: [u8; 0x38],
    pi_reg_155: PiReg155,
    pi_reg_156: PiReg156,
    pi_reg_157: PiReg157,
    pi_reg_158: PiReg158,
    pi_reg_159: PiReg159,
    pi_reg_160: PiReg160,
    pi_reg_161: PiReg161,
    pi_reg_162: PiReg162,
    pi_reg_163: PiReg163,
    pi_reg_164: PiReg164,
    pi_reg_165: PiReg165,
    pi_reg_166: PiReg166,
    pi_reg_167: PiReg167,
    pi_reg_168: PiReg168,
    pi_reg_169: PiReg169,
    _reserved156: [u8; 0x10],
    pi_reg_174: PiReg174,
    pi_reg_175: PiReg175,
    pi_reg_176: PiReg176,
    _reserved159: [u8; 0x24],
    pi_reg_186: PiReg186,
    pi_reg_187: PiReg187,
    pi_reg_188: PiReg188,
    pi_reg_189: PiReg189,
    pi_reg_190: PiReg190,
    pi_reg_191: PiReg191,
    pi_reg_192: PiReg192,
    pi_reg_193: PiReg193,
    _reserved167: [u8; 0x14],
    pi_reg_199: PiReg199,
}
impl RegisterBlock {
    #[doc = "0x00 - DDR PHY Independent Register 0"]
    #[inline(always)]
    pub const fn pi_reg_0(&self) -> &PiReg0 {
        &self.pi_reg_0
    }
    #[doc = "0x04 - DDR PHY Independent Register 1"]
    #[inline(always)]
    pub const fn pi_reg_1(&self) -> &PiReg1 {
        &self.pi_reg_1
    }
    #[doc = "0x08 - DDR PHY Independent Register 2"]
    #[inline(always)]
    pub const fn pi_reg_2(&self) -> &PiReg2 {
        &self.pi_reg_2
    }
    #[doc = "0x0c - DDR PHY Independent Register 3"]
    #[inline(always)]
    pub const fn pi_reg_3(&self) -> &PiReg3 {
        &self.pi_reg_3
    }
    #[doc = "0x10 - DDR PHY Independent Register 4"]
    #[inline(always)]
    pub const fn pi_reg_4(&self) -> &PiReg4 {
        &self.pi_reg_4
    }
    #[doc = "0x14 - DDR PHY Independent Register 5"]
    #[inline(always)]
    pub const fn pi_reg_5(&self) -> &PiReg5 {
        &self.pi_reg_5
    }
    #[doc = "0x18 - DDR PHY Independent Register 6"]
    #[inline(always)]
    pub const fn pi_reg_6(&self) -> &PiReg6 {
        &self.pi_reg_6
    }
    #[doc = "0x1c - DDR PHY Independent Register 7"]
    #[inline(always)]
    pub const fn pi_reg_7(&self) -> &PiReg7 {
        &self.pi_reg_7
    }
    #[doc = "0x20 - DDR PHY Independent Register 8"]
    #[inline(always)]
    pub const fn pi_reg_8(&self) -> &PiReg8 {
        &self.pi_reg_8
    }
    #[doc = "0x24 - DDR PHY Independent Register 9"]
    #[inline(always)]
    pub const fn pi_reg_9(&self) -> &PiReg9 {
        &self.pi_reg_9
    }
    #[doc = "0x28 - DDR PHY Independent Register 10"]
    #[inline(always)]
    pub const fn pi_reg_10(&self) -> &PiReg10 {
        &self.pi_reg_10
    }
    #[doc = "0x2c - DDR PHY Independent Register 11"]
    #[inline(always)]
    pub const fn pi_reg_11(&self) -> &PiReg11 {
        &self.pi_reg_11
    }
    #[doc = "0x30 - DDR PHY Independent Register 12"]
    #[inline(always)]
    pub const fn pi_reg_12(&self) -> &PiReg12 {
        &self.pi_reg_12
    }
    #[doc = "0x34 - DDR PHY Independent Register 13"]
    #[inline(always)]
    pub const fn pi_reg_13(&self) -> &PiReg13 {
        &self.pi_reg_13
    }
    #[doc = "0x38 - DDR PHY Independent Register 14"]
    #[inline(always)]
    pub const fn pi_reg_14(&self) -> &PiReg14 {
        &self.pi_reg_14
    }
    #[doc = "0x3c - DDR PHY Independent Register 15"]
    #[inline(always)]
    pub const fn pi_reg_15(&self) -> &PiReg15 {
        &self.pi_reg_15
    }
    #[doc = "0x40 - DDR PHY Independent Register 16"]
    #[inline(always)]
    pub const fn pi_reg_16(&self) -> &PiReg16 {
        &self.pi_reg_16
    }
    #[doc = "0x44 - DDR PHY Independent Register 17"]
    #[inline(always)]
    pub const fn pi_reg_17(&self) -> &PiReg17 {
        &self.pi_reg_17
    }
    #[doc = "0x48 - DDR PHY Independent Register 18"]
    #[inline(always)]
    pub const fn pi_reg_18(&self) -> &PiReg18 {
        &self.pi_reg_18
    }
    #[doc = "0x4c - DDR PHY Independent Register 19"]
    #[inline(always)]
    pub const fn pi_reg_19(&self) -> &PiReg19 {
        &self.pi_reg_19
    }
    #[doc = "0x50 - DDR PHY Independent Register 20"]
    #[inline(always)]
    pub const fn pi_reg_20(&self) -> &PiReg20 {
        &self.pi_reg_20
    }
    #[doc = "0x54 - DDR PHY Independent Register 21"]
    #[inline(always)]
    pub const fn pi_reg_21(&self) -> &PiReg21 {
        &self.pi_reg_21
    }
    #[doc = "0x58 - DDR PHY Independent Register 22"]
    #[inline(always)]
    pub const fn pi_reg_22(&self) -> &PiReg22 {
        &self.pi_reg_22
    }
    #[doc = "0x5c - DDR PHY Independent Register 23"]
    #[inline(always)]
    pub const fn pi_reg_23(&self) -> &PiReg23 {
        &self.pi_reg_23
    }
    #[doc = "0x60 - DDR PHY Independent Register 24"]
    #[inline(always)]
    pub const fn pi_reg_24(&self) -> &PiReg24 {
        &self.pi_reg_24
    }
    #[doc = "0x64 - DDR PHY Independent Register 25"]
    #[inline(always)]
    pub const fn pi_reg_25(&self) -> &PiReg25 {
        &self.pi_reg_25
    }
    #[doc = "0x68 - DDR PHY Independent Register 26"]
    #[inline(always)]
    pub const fn pi_reg_26(&self) -> &PiReg26 {
        &self.pi_reg_26
    }
    #[doc = "0x6c - DDR PHY Independent Register 27"]
    #[inline(always)]
    pub const fn pi_reg_27(&self) -> &PiReg27 {
        &self.pi_reg_27
    }
    #[doc = "0x70 - DDR PHY Independent Register 28"]
    #[inline(always)]
    pub const fn pi_reg_28(&self) -> &PiReg28 {
        &self.pi_reg_28
    }
    #[doc = "0x74 - DDR PHY Independent Register 29"]
    #[inline(always)]
    pub const fn pi_reg_29(&self) -> &PiReg29 {
        &self.pi_reg_29
    }
    #[doc = "0x78 - DDR PHY Independent Register 30"]
    #[inline(always)]
    pub const fn pi_reg_30(&self) -> &PiReg30 {
        &self.pi_reg_30
    }
    #[doc = "0x7c - DDR PHY Independent Register 31"]
    #[inline(always)]
    pub const fn pi_reg_31(&self) -> &PiReg31 {
        &self.pi_reg_31
    }
    #[doc = "0x80 - DDR PHY Independent Register 32"]
    #[inline(always)]
    pub const fn pi_reg_32(&self) -> &PiReg32 {
        &self.pi_reg_32
    }
    #[doc = "0x84 - DDR PHY Independent Register 33"]
    #[inline(always)]
    pub const fn pi_reg_33(&self) -> &PiReg33 {
        &self.pi_reg_33
    }
    #[doc = "0x88 - DDR PHY Independent Register 34"]
    #[inline(always)]
    pub const fn pi_reg_34(&self) -> &PiReg34 {
        &self.pi_reg_34
    }
    #[doc = "0x8c - DDR PHY Independent Register 35"]
    #[inline(always)]
    pub const fn pi_reg_35(&self) -> &PiReg35 {
        &self.pi_reg_35
    }
    #[doc = "0x90 - DDR PHY Independent Register 36"]
    #[inline(always)]
    pub const fn pi_reg_36(&self) -> &PiReg36 {
        &self.pi_reg_36
    }
    #[doc = "0x94 - DDR PHY Independent Register 37"]
    #[inline(always)]
    pub const fn pi_reg_37(&self) -> &PiReg37 {
        &self.pi_reg_37
    }
    #[doc = "0x98 - DDR PHY Independent Register 38"]
    #[inline(always)]
    pub const fn pi_reg_38(&self) -> &PiReg38 {
        &self.pi_reg_38
    }
    #[doc = "0x9c - DDR PHY Independent Register 39"]
    #[inline(always)]
    pub const fn pi_reg_39(&self) -> &PiReg39 {
        &self.pi_reg_39
    }
    #[doc = "0xa0 - DDR PHY Independent Register 40"]
    #[inline(always)]
    pub const fn pi_reg_40(&self) -> &PiReg40 {
        &self.pi_reg_40
    }
    #[doc = "0xa4 - DDR PHY Independent Register 41"]
    #[inline(always)]
    pub const fn pi_reg_41(&self) -> &PiReg41 {
        &self.pi_reg_41
    }
    #[doc = "0xa8 - DDR PHY Independent Register 42"]
    #[inline(always)]
    pub const fn pi_reg_42(&self) -> &PiReg42 {
        &self.pi_reg_42
    }
    #[doc = "0xac - DDR PHY Independent Register 43"]
    #[inline(always)]
    pub const fn pi_reg_43(&self) -> &PiReg43 {
        &self.pi_reg_43
    }
    #[doc = "0xb0 - DDR PHY Independent Register 44"]
    #[inline(always)]
    pub const fn pi_reg_44(&self) -> &PiReg44 {
        &self.pi_reg_44
    }
    #[doc = "0xb4 - DDR PHY Independent Register 45"]
    #[inline(always)]
    pub const fn pi_reg_45(&self) -> &PiReg45 {
        &self.pi_reg_45
    }
    #[doc = "0xb8 - DDR PHY Independent Register 46"]
    #[inline(always)]
    pub const fn pi_reg_46(&self) -> &PiReg46 {
        &self.pi_reg_46
    }
    #[doc = "0xbc - DDR PHY Independent Register 47"]
    #[inline(always)]
    pub const fn pi_reg_47(&self) -> &PiReg47 {
        &self.pi_reg_47
    }
    #[doc = "0xc0 - DDR PHY Independent Register 48"]
    #[inline(always)]
    pub const fn pi_reg_48(&self) -> &PiReg48 {
        &self.pi_reg_48
    }
    #[doc = "0xc4 - DDR PHY Independent Register 49"]
    #[inline(always)]
    pub const fn pi_reg_49(&self) -> &PiReg49 {
        &self.pi_reg_49
    }
    #[doc = "0xc8 - DDR PHY Independent Register 50"]
    #[inline(always)]
    pub const fn pi_reg_50(&self) -> &PiReg50 {
        &self.pi_reg_50
    }
    #[doc = "0xcc - DDR PHY Independent Register 51"]
    #[inline(always)]
    pub const fn pi_reg_51(&self) -> &PiReg51 {
        &self.pi_reg_51
    }
    #[doc = "0xd0 - DDR PHY Independent Register 52"]
    #[inline(always)]
    pub const fn pi_reg_52(&self) -> &PiReg52 {
        &self.pi_reg_52
    }
    #[doc = "0xd4 - DDR PHY Independent Register 53"]
    #[inline(always)]
    pub const fn pi_reg_53(&self) -> &PiReg53 {
        &self.pi_reg_53
    }
    #[doc = "0xd8 - DDR PHY Independent Register 54"]
    #[inline(always)]
    pub const fn pi_reg_54(&self) -> &PiReg54 {
        &self.pi_reg_54
    }
    #[doc = "0xdc - DDR PHY Independent Register 55"]
    #[inline(always)]
    pub const fn pi_reg_55(&self) -> &PiReg55 {
        &self.pi_reg_55
    }
    #[doc = "0xe0 - DDR PHY Independent Register 56"]
    #[inline(always)]
    pub const fn pi_reg_56(&self) -> &PiReg56 {
        &self.pi_reg_56
    }
    #[doc = "0xe4 - DDR PHY Independent Register 57"]
    #[inline(always)]
    pub const fn pi_reg_57(&self) -> &PiReg57 {
        &self.pi_reg_57
    }
    #[doc = "0xe8 - DDR PHY Independent Register 58"]
    #[inline(always)]
    pub const fn pi_reg_58(&self) -> &PiReg58 {
        &self.pi_reg_58
    }
    #[doc = "0xec - DDR PHY Independent Register 59"]
    #[inline(always)]
    pub const fn pi_reg_59(&self) -> &PiReg59 {
        &self.pi_reg_59
    }
    #[doc = "0xf0 - DDR PHY Independent Register 60"]
    #[inline(always)]
    pub const fn pi_reg_60(&self) -> &PiReg60 {
        &self.pi_reg_60
    }
    #[doc = "0xf4 - DDR PHY Independent Register 61"]
    #[inline(always)]
    pub const fn pi_reg_61(&self) -> &PiReg61 {
        &self.pi_reg_61
    }
    #[doc = "0xf8 - DDR PHY Independent Register 62"]
    #[inline(always)]
    pub const fn pi_reg_62(&self) -> &PiReg62 {
        &self.pi_reg_62
    }
    #[doc = "0xfc - DDR PHY Independent Register 63"]
    #[inline(always)]
    pub const fn pi_reg_63(&self) -> &PiReg63 {
        &self.pi_reg_63
    }
    #[doc = "0x100 - DDR PHY Independent Register 64"]
    #[inline(always)]
    pub const fn pi_reg_64(&self) -> &PiReg64 {
        &self.pi_reg_64
    }
    #[doc = "0x104 - DDR PHY Independent Register 65"]
    #[inline(always)]
    pub const fn pi_reg_65(&self) -> &PiReg65 {
        &self.pi_reg_65
    }
    #[doc = "0x108 - DDR PHY Independent Register 66"]
    #[inline(always)]
    pub const fn pi_reg_66(&self) -> &PiReg66 {
        &self.pi_reg_66
    }
    #[doc = "0x10c - DDR PHY Independent Register 67"]
    #[inline(always)]
    pub const fn pi_reg_67(&self) -> &PiReg67 {
        &self.pi_reg_67
    }
    #[doc = "0x110 - DDR PHY Independent Register 68"]
    #[inline(always)]
    pub const fn pi_reg_68(&self) -> &PiReg68 {
        &self.pi_reg_68
    }
    #[doc = "0x114 - DDR PHY Independent Register 69"]
    #[inline(always)]
    pub const fn pi_reg_69(&self) -> &PiReg69 {
        &self.pi_reg_69
    }
    #[doc = "0x118 - DDR PHY Independent Register 70"]
    #[inline(always)]
    pub const fn pi_reg_70(&self) -> &PiReg70 {
        &self.pi_reg_70
    }
    #[doc = "0x11c - DDR PHY Independent Register 71"]
    #[inline(always)]
    pub const fn pi_reg_71(&self) -> &PiReg71 {
        &self.pi_reg_71
    }
    #[doc = "0x120 - DDR PHY Independent Register 72"]
    #[inline(always)]
    pub const fn pi_reg_72(&self) -> &PiReg72 {
        &self.pi_reg_72
    }
    #[doc = "0x124 - DDR PHY Independent Register 73"]
    #[inline(always)]
    pub const fn pi_reg_73(&self) -> &PiReg73 {
        &self.pi_reg_73
    }
    #[doc = "0x128 - DDR PHY Independent Register 74"]
    #[inline(always)]
    pub const fn pi_reg_74(&self) -> &PiReg74 {
        &self.pi_reg_74
    }
    #[doc = "0x12c - DDR PHY Independent Register 75"]
    #[inline(always)]
    pub const fn pi_reg_75(&self) -> &PiReg75 {
        &self.pi_reg_75
    }
    #[doc = "0x130 - DDR PHY Independent Register 76"]
    #[inline(always)]
    pub const fn pi_reg_76(&self) -> &PiReg76 {
        &self.pi_reg_76
    }
    #[doc = "0x134 - DDR PHY Independent Register 77"]
    #[inline(always)]
    pub const fn pi_reg_77(&self) -> &PiReg77 {
        &self.pi_reg_77
    }
    #[doc = "0x138 - DDR PHY Independent Register 78"]
    #[inline(always)]
    pub const fn pi_reg_78(&self) -> &PiReg78 {
        &self.pi_reg_78
    }
    #[doc = "0x13c - DDR PHY Independent Register 79"]
    #[inline(always)]
    pub const fn pi_reg_79(&self) -> &PiReg79 {
        &self.pi_reg_79
    }
    #[doc = "0x140 - DDR PHY Independent Register 80"]
    #[inline(always)]
    pub const fn pi_reg_80(&self) -> &PiReg80 {
        &self.pi_reg_80
    }
    #[doc = "0x144 - DDR PHY Independent Register 81"]
    #[inline(always)]
    pub const fn pi_reg_81(&self) -> &PiReg81 {
        &self.pi_reg_81
    }
    #[doc = "0x148 - DDR PHY Independent Register 82"]
    #[inline(always)]
    pub const fn pi_reg_82(&self) -> &PiReg82 {
        &self.pi_reg_82
    }
    #[doc = "0x14c - DDR PHY Independent Register 83"]
    #[inline(always)]
    pub const fn pi_reg_83(&self) -> &PiReg83 {
        &self.pi_reg_83
    }
    #[doc = "0x150 - DDR PHY Independent Register 84"]
    #[inline(always)]
    pub const fn pi_reg_84(&self) -> &PiReg84 {
        &self.pi_reg_84
    }
    #[doc = "0x154 - DDR PHY Independent Register 85"]
    #[inline(always)]
    pub const fn pi_reg_85(&self) -> &PiReg85 {
        &self.pi_reg_85
    }
    #[doc = "0x158 - DDR PHY Independent Register 86"]
    #[inline(always)]
    pub const fn pi_reg_86(&self) -> &PiReg86 {
        &self.pi_reg_86
    }
    #[doc = "0x15c - DDR PHY Independent Register 87"]
    #[inline(always)]
    pub const fn pi_reg_87(&self) -> &PiReg87 {
        &self.pi_reg_87
    }
    #[doc = "0x160 - DDR PHY Independent Register 88"]
    #[inline(always)]
    pub const fn pi_reg_88(&self) -> &PiReg88 {
        &self.pi_reg_88
    }
    #[doc = "0x164 - DDR PHY Independent Register 89"]
    #[inline(always)]
    pub const fn pi_reg_89(&self) -> &PiReg89 {
        &self.pi_reg_89
    }
    #[doc = "0x168 - DDR PHY Independent Register 90"]
    #[inline(always)]
    pub const fn pi_reg_90(&self) -> &PiReg90 {
        &self.pi_reg_90
    }
    #[doc = "0x16c - DDR PHY Independent Register 91"]
    #[inline(always)]
    pub const fn pi_reg_91(&self) -> &PiReg91 {
        &self.pi_reg_91
    }
    #[doc = "0x170 - DDR PHY Independent Register 92"]
    #[inline(always)]
    pub const fn pi_reg_92(&self) -> &PiReg92 {
        &self.pi_reg_92
    }
    #[doc = "0x174 - DDR PHY Independent Register 93"]
    #[inline(always)]
    pub const fn pi_reg_93(&self) -> &PiReg93 {
        &self.pi_reg_93
    }
    #[doc = "0x178 - DDR PHY Independent Register 94"]
    #[inline(always)]
    pub const fn pi_reg_94(&self) -> &PiReg94 {
        &self.pi_reg_94
    }
    #[doc = "0x17c - DDR PHY Independent Register 95"]
    #[inline(always)]
    pub const fn pi_reg_95(&self) -> &PiReg95 {
        &self.pi_reg_95
    }
    #[doc = "0x180 - DDR PHY Independent Register 96"]
    #[inline(always)]
    pub const fn pi_reg_96(&self) -> &PiReg96 {
        &self.pi_reg_96
    }
    #[doc = "0x184 - DDR PHY Independent Register 97"]
    #[inline(always)]
    pub const fn pi_reg_97(&self) -> &PiReg97 {
        &self.pi_reg_97
    }
    #[doc = "0x188 - DDR PHY Independent Register 98"]
    #[inline(always)]
    pub const fn pi_reg_98(&self) -> &PiReg98 {
        &self.pi_reg_98
    }
    #[doc = "0x18c - DDR PHY Independent Register 99"]
    #[inline(always)]
    pub const fn pi_reg_99(&self) -> &PiReg99 {
        &self.pi_reg_99
    }
    #[doc = "0x190 - DDR PHY Independent Register 100"]
    #[inline(always)]
    pub const fn pi_reg_100(&self) -> &PiReg100 {
        &self.pi_reg_100
    }
    #[doc = "0x194 - DDR PHY Independent Register 101"]
    #[inline(always)]
    pub const fn pi_reg_101(&self) -> &PiReg101 {
        &self.pi_reg_101
    }
    #[doc = "0x198 - DDR PHY Independent Register 102"]
    #[inline(always)]
    pub const fn pi_reg_102(&self) -> &PiReg102 {
        &self.pi_reg_102
    }
    #[doc = "0x19c - DDR PHY Independent Register 103"]
    #[inline(always)]
    pub const fn pi_reg_103(&self) -> &PiReg103 {
        &self.pi_reg_103
    }
    #[doc = "0x1a0 - DDR PHY Independent Register 104"]
    #[inline(always)]
    pub const fn pi_reg_104(&self) -> &PiReg104 {
        &self.pi_reg_104
    }
    #[doc = "0x1a4 - DDR PHY Independent Register 105"]
    #[inline(always)]
    pub const fn pi_reg_105(&self) -> &PiReg105 {
        &self.pi_reg_105
    }
    #[doc = "0x1a8 - DDR PHY Independent Register 106"]
    #[inline(always)]
    pub const fn pi_reg_106(&self) -> &PiReg106 {
        &self.pi_reg_106
    }
    #[doc = "0x1ac - DDR PHY Independent Register 107"]
    #[inline(always)]
    pub const fn pi_reg_107(&self) -> &PiReg107 {
        &self.pi_reg_107
    }
    #[doc = "0x1b0 - DDR PHY Independent Register 108"]
    #[inline(always)]
    pub const fn pi_reg_108(&self) -> &PiReg108 {
        &self.pi_reg_108
    }
    #[doc = "0x1b4 - DDR PHY Independent Register 109"]
    #[inline(always)]
    pub const fn pi_reg_109(&self) -> &PiReg109 {
        &self.pi_reg_109
    }
    #[doc = "0x1b8 - DDR PHY Independent Register 110"]
    #[inline(always)]
    pub const fn pi_reg_110(&self) -> &PiReg110 {
        &self.pi_reg_110
    }
    #[doc = "0x1bc - DDR PHY Independent Register 111"]
    #[inline(always)]
    pub const fn pi_reg_111(&self) -> &PiReg111 {
        &self.pi_reg_111
    }
    #[doc = "0x1c0 - DDR PHY Independent Register 112"]
    #[inline(always)]
    pub const fn pi_reg_112(&self) -> &PiReg112 {
        &self.pi_reg_112
    }
    #[doc = "0x1c4 - DDR PHY Independent Register 113"]
    #[inline(always)]
    pub const fn pi_reg_113(&self) -> &PiReg113 {
        &self.pi_reg_113
    }
    #[doc = "0x1c8 - DDR PHY Independent Register 114"]
    #[inline(always)]
    pub const fn pi_reg_114(&self) -> &PiReg114 {
        &self.pi_reg_114
    }
    #[doc = "0x1cc - DDR PHY Independent Register 115"]
    #[inline(always)]
    pub const fn pi_reg_115(&self) -> &PiReg115 {
        &self.pi_reg_115
    }
    #[doc = "0x1d0 - DDR PHY Independent Register 116"]
    #[inline(always)]
    pub const fn pi_reg_116(&self) -> &PiReg116 {
        &self.pi_reg_116
    }
    #[doc = "0x1d4 - DDR PHY Independent Register 117"]
    #[inline(always)]
    pub const fn pi_reg_117(&self) -> &PiReg117 {
        &self.pi_reg_117
    }
    #[doc = "0x1d8 - DDR PHY Independent Register 118"]
    #[inline(always)]
    pub const fn pi_reg_118(&self) -> &PiReg118 {
        &self.pi_reg_118
    }
    #[doc = "0x1dc - DDR PHY Independent Register 119"]
    #[inline(always)]
    pub const fn pi_reg_119(&self) -> &PiReg119 {
        &self.pi_reg_119
    }
    #[doc = "0x1e0 - DDR PHY Independent Register 120"]
    #[inline(always)]
    pub const fn pi_reg_120(&self) -> &PiReg120 {
        &self.pi_reg_120
    }
    #[doc = "0x1e4 - DDR PHY Independent Register 121"]
    #[inline(always)]
    pub const fn pi_reg_121(&self) -> &PiReg121 {
        &self.pi_reg_121
    }
    #[doc = "0x1e8 - DDR PHY Independent Register 122"]
    #[inline(always)]
    pub const fn pi_reg_122(&self) -> &PiReg122 {
        &self.pi_reg_122
    }
    #[doc = "0x1ec - DDR PHY Independent Register 123"]
    #[inline(always)]
    pub const fn pi_reg_123(&self) -> &PiReg123 {
        &self.pi_reg_123
    }
    #[doc = "0x1f0 - DDR PHY Independent Register 124"]
    #[inline(always)]
    pub const fn pi_reg_124(&self) -> &PiReg124 {
        &self.pi_reg_124
    }
    #[doc = "0x1f4 - DDR PHY Independent Register 125"]
    #[inline(always)]
    pub const fn pi_reg_125(&self) -> &PiReg125 {
        &self.pi_reg_125
    }
    #[doc = "0x1f8 - DDR PHY Independent Register 126"]
    #[inline(always)]
    pub const fn pi_reg_126(&self) -> &PiReg126 {
        &self.pi_reg_126
    }
    #[doc = "0x1fc - DDR PHY Independent Register 127"]
    #[inline(always)]
    pub const fn pi_reg_127(&self) -> &PiReg127 {
        &self.pi_reg_127
    }
    #[doc = "0x200 - DDR PHY Independent Register 128"]
    #[inline(always)]
    pub const fn pi_reg_128(&self) -> &PiReg128 {
        &self.pi_reg_128
    }
    #[doc = "0x204 - DDR PHY Independent Register 129"]
    #[inline(always)]
    pub const fn pi_reg_129(&self) -> &PiReg129 {
        &self.pi_reg_129
    }
    #[doc = "0x208 - DDR PHY Independent Register 130"]
    #[inline(always)]
    pub const fn pi_reg_130(&self) -> &PiReg130 {
        &self.pi_reg_130
    }
    #[doc = "0x20c - DDR PHY Independent Register 131"]
    #[inline(always)]
    pub const fn pi_reg_131(&self) -> &PiReg131 {
        &self.pi_reg_131
    }
    #[doc = "0x210 - DDR PHY Independent Register 132"]
    #[inline(always)]
    pub const fn pi_reg_132(&self) -> &PiReg132 {
        &self.pi_reg_132
    }
    #[doc = "0x214 - DDR PHY Independent Register 133"]
    #[inline(always)]
    pub const fn pi_reg_133(&self) -> &PiReg133 {
        &self.pi_reg_133
    }
    #[doc = "0x218 - DDR PHY Independent Register 134"]
    #[inline(always)]
    pub const fn pi_reg_134(&self) -> &PiReg134 {
        &self.pi_reg_134
    }
    #[doc = "0x21c - DDR PHY Independent Register 135"]
    #[inline(always)]
    pub const fn pi_reg_135(&self) -> &PiReg135 {
        &self.pi_reg_135
    }
    #[doc = "0x220 - DDR PHY Independent Register 136"]
    #[inline(always)]
    pub const fn pi_reg_136(&self) -> &PiReg136 {
        &self.pi_reg_136
    }
    #[doc = "0x224 - DDR PHY Independent Register 137"]
    #[inline(always)]
    pub const fn pi_reg_137(&self) -> &PiReg137 {
        &self.pi_reg_137
    }
    #[doc = "0x228 - DDR PHY Independent Register 138"]
    #[inline(always)]
    pub const fn pi_reg_138(&self) -> &PiReg138 {
        &self.pi_reg_138
    }
    #[doc = "0x22c - DDR PHY Independent Register 139"]
    #[inline(always)]
    pub const fn pi_reg_139(&self) -> &PiReg139 {
        &self.pi_reg_139
    }
    #[doc = "0x230 - DDR PHY Independent Register 140"]
    #[inline(always)]
    pub const fn pi_reg_140(&self) -> &PiReg140 {
        &self.pi_reg_140
    }
    #[doc = "0x26c - DDR PHY Independent Register 155"]
    #[inline(always)]
    pub const fn pi_reg_155(&self) -> &PiReg155 {
        &self.pi_reg_155
    }
    #[doc = "0x270 - DDR PHY Independent Register 156"]
    #[inline(always)]
    pub const fn pi_reg_156(&self) -> &PiReg156 {
        &self.pi_reg_156
    }
    #[doc = "0x274 - DDR PHY Independent Register 157"]
    #[inline(always)]
    pub const fn pi_reg_157(&self) -> &PiReg157 {
        &self.pi_reg_157
    }
    #[doc = "0x278 - DDR PHY Independent Register 158"]
    #[inline(always)]
    pub const fn pi_reg_158(&self) -> &PiReg158 {
        &self.pi_reg_158
    }
    #[doc = "0x27c - DDR PHY Independent Register 159"]
    #[inline(always)]
    pub const fn pi_reg_159(&self) -> &PiReg159 {
        &self.pi_reg_159
    }
    #[doc = "0x280 - DDR PHY Independent Register 160"]
    #[inline(always)]
    pub const fn pi_reg_160(&self) -> &PiReg160 {
        &self.pi_reg_160
    }
    #[doc = "0x284 - DDR PHY Independent Register 161"]
    #[inline(always)]
    pub const fn pi_reg_161(&self) -> &PiReg161 {
        &self.pi_reg_161
    }
    #[doc = "0x288 - DDR PHY Independent Register 162"]
    #[inline(always)]
    pub const fn pi_reg_162(&self) -> &PiReg162 {
        &self.pi_reg_162
    }
    #[doc = "0x28c - DDR PHY Independent Register 163"]
    #[inline(always)]
    pub const fn pi_reg_163(&self) -> &PiReg163 {
        &self.pi_reg_163
    }
    #[doc = "0x290 - DDR PHY Independent Register 164"]
    #[inline(always)]
    pub const fn pi_reg_164(&self) -> &PiReg164 {
        &self.pi_reg_164
    }
    #[doc = "0x294 - DDR PHY Independent Register 165"]
    #[inline(always)]
    pub const fn pi_reg_165(&self) -> &PiReg165 {
        &self.pi_reg_165
    }
    #[doc = "0x298 - DDR PHY Independent Register 166"]
    #[inline(always)]
    pub const fn pi_reg_166(&self) -> &PiReg166 {
        &self.pi_reg_166
    }
    #[doc = "0x29c - DDR PHY Independent Register 167"]
    #[inline(always)]
    pub const fn pi_reg_167(&self) -> &PiReg167 {
        &self.pi_reg_167
    }
    #[doc = "0x2a0 - DDR PHY Independent Register 168"]
    #[inline(always)]
    pub const fn pi_reg_168(&self) -> &PiReg168 {
        &self.pi_reg_168
    }
    #[doc = "0x2a4 - DDR PHY Independent Register 169"]
    #[inline(always)]
    pub const fn pi_reg_169(&self) -> &PiReg169 {
        &self.pi_reg_169
    }
    #[doc = "0x2b8 - DDR PHY Independent Register 174"]
    #[inline(always)]
    pub const fn pi_reg_174(&self) -> &PiReg174 {
        &self.pi_reg_174
    }
    #[doc = "0x2bc - DDR PHY Independent Register 175"]
    #[inline(always)]
    pub const fn pi_reg_175(&self) -> &PiReg175 {
        &self.pi_reg_175
    }
    #[doc = "0x2c0 - DDR PHY Independent Register 176"]
    #[inline(always)]
    pub const fn pi_reg_176(&self) -> &PiReg176 {
        &self.pi_reg_176
    }
    #[doc = "0x2e8 - DDR PHY Independent Register 186"]
    #[inline(always)]
    pub const fn pi_reg_186(&self) -> &PiReg186 {
        &self.pi_reg_186
    }
    #[doc = "0x2ec - DDR PHY Independent Register 187"]
    #[inline(always)]
    pub const fn pi_reg_187(&self) -> &PiReg187 {
        &self.pi_reg_187
    }
    #[doc = "0x2f0 - DDR PHY Independent Register 188"]
    #[inline(always)]
    pub const fn pi_reg_188(&self) -> &PiReg188 {
        &self.pi_reg_188
    }
    #[doc = "0x2f4 - DDR PHY Independent Register 189"]
    #[inline(always)]
    pub const fn pi_reg_189(&self) -> &PiReg189 {
        &self.pi_reg_189
    }
    #[doc = "0x2f8 - DDR PHY Independent Register 190"]
    #[inline(always)]
    pub const fn pi_reg_190(&self) -> &PiReg190 {
        &self.pi_reg_190
    }
    #[doc = "0x2fc - DDR PHY Independent Register 191"]
    #[inline(always)]
    pub const fn pi_reg_191(&self) -> &PiReg191 {
        &self.pi_reg_191
    }
    #[doc = "0x300 - DDR PHY Independent Register 192"]
    #[inline(always)]
    pub const fn pi_reg_192(&self) -> &PiReg192 {
        &self.pi_reg_192
    }
    #[doc = "0x304 - DDR PHY Independent Register 193"]
    #[inline(always)]
    pub const fn pi_reg_193(&self) -> &PiReg193 {
        &self.pi_reg_193
    }
    #[doc = "0x31c - DDR PHY Independent Register 199"]
    #[inline(always)]
    pub const fn pi_reg_199(&self) -> &PiReg199 {
        &self.pi_reg_199
    }
}
#[doc = "PI_REG_0 (rw) register accessor: DDR PHY Independent Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_0`]
module"]
#[doc(alias = "PI_REG_0")]
pub type PiReg0 = crate::Reg<pi_reg_0::PiReg0Spec>;
#[doc = "DDR PHY Independent Register 0"]
pub mod pi_reg_0;
#[doc = "PI_REG_1 (rw) register accessor: DDR PHY Independent Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_1`]
module"]
#[doc(alias = "PI_REG_1")]
pub type PiReg1 = crate::Reg<pi_reg_1::PiReg1Spec>;
#[doc = "DDR PHY Independent Register 1"]
pub mod pi_reg_1;
#[doc = "PI_REG_2 (rw) register accessor: DDR PHY Independent Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_2`]
module"]
#[doc(alias = "PI_REG_2")]
pub type PiReg2 = crate::Reg<pi_reg_2::PiReg2Spec>;
#[doc = "DDR PHY Independent Register 2"]
pub mod pi_reg_2;
#[doc = "PI_REG_3 (rw) register accessor: DDR PHY Independent Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_3`]
module"]
#[doc(alias = "PI_REG_3")]
pub type PiReg3 = crate::Reg<pi_reg_3::PiReg3Spec>;
#[doc = "DDR PHY Independent Register 3"]
pub mod pi_reg_3;
#[doc = "PI_REG_4 (rw) register accessor: DDR PHY Independent Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_4`]
module"]
#[doc(alias = "PI_REG_4")]
pub type PiReg4 = crate::Reg<pi_reg_4::PiReg4Spec>;
#[doc = "DDR PHY Independent Register 4"]
pub mod pi_reg_4;
#[doc = "PI_REG_5 (rw) register accessor: DDR PHY Independent Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_5`]
module"]
#[doc(alias = "PI_REG_5")]
pub type PiReg5 = crate::Reg<pi_reg_5::PiReg5Spec>;
#[doc = "DDR PHY Independent Register 5"]
pub mod pi_reg_5;
#[doc = "PI_REG_6 (rw) register accessor: DDR PHY Independent Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_6`]
module"]
#[doc(alias = "PI_REG_6")]
pub type PiReg6 = crate::Reg<pi_reg_6::PiReg6Spec>;
#[doc = "DDR PHY Independent Register 6"]
pub mod pi_reg_6;
#[doc = "PI_REG_7 (rw) register accessor: DDR PHY Independent Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_7`]
module"]
#[doc(alias = "PI_REG_7")]
pub type PiReg7 = crate::Reg<pi_reg_7::PiReg7Spec>;
#[doc = "DDR PHY Independent Register 7"]
pub mod pi_reg_7;
#[doc = "PI_REG_8 (rw) register accessor: DDR PHY Independent Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_8`]
module"]
#[doc(alias = "PI_REG_8")]
pub type PiReg8 = crate::Reg<pi_reg_8::PiReg8Spec>;
#[doc = "DDR PHY Independent Register 8"]
pub mod pi_reg_8;
#[doc = "PI_REG_9 (rw) register accessor: DDR PHY Independent Register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_9`]
module"]
#[doc(alias = "PI_REG_9")]
pub type PiReg9 = crate::Reg<pi_reg_9::PiReg9Spec>;
#[doc = "DDR PHY Independent Register 9"]
pub mod pi_reg_9;
#[doc = "PI_REG_10 (rw) register accessor: DDR PHY Independent Register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_10`]
module"]
#[doc(alias = "PI_REG_10")]
pub type PiReg10 = crate::Reg<pi_reg_10::PiReg10Spec>;
#[doc = "DDR PHY Independent Register 10"]
pub mod pi_reg_10;
#[doc = "PI_REG_11 (rw) register accessor: DDR PHY Independent Register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_11`]
module"]
#[doc(alias = "PI_REG_11")]
pub type PiReg11 = crate::Reg<pi_reg_11::PiReg11Spec>;
#[doc = "DDR PHY Independent Register 11"]
pub mod pi_reg_11;
#[doc = "PI_REG_12 (rw) register accessor: DDR PHY Independent Register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_12`]
module"]
#[doc(alias = "PI_REG_12")]
pub type PiReg12 = crate::Reg<pi_reg_12::PiReg12Spec>;
#[doc = "DDR PHY Independent Register 12"]
pub mod pi_reg_12;
#[doc = "PI_REG_13 (rw) register accessor: DDR PHY Independent Register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_13`]
module"]
#[doc(alias = "PI_REG_13")]
pub type PiReg13 = crate::Reg<pi_reg_13::PiReg13Spec>;
#[doc = "DDR PHY Independent Register 13"]
pub mod pi_reg_13;
#[doc = "PI_REG_14 (rw) register accessor: DDR PHY Independent Register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_14`]
module"]
#[doc(alias = "PI_REG_14")]
pub type PiReg14 = crate::Reg<pi_reg_14::PiReg14Spec>;
#[doc = "DDR PHY Independent Register 14"]
pub mod pi_reg_14;
#[doc = "PI_REG_15 (rw) register accessor: DDR PHY Independent Register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_15`]
module"]
#[doc(alias = "PI_REG_15")]
pub type PiReg15 = crate::Reg<pi_reg_15::PiReg15Spec>;
#[doc = "DDR PHY Independent Register 15"]
pub mod pi_reg_15;
#[doc = "PI_REG_16 (rw) register accessor: DDR PHY Independent Register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_16`]
module"]
#[doc(alias = "PI_REG_16")]
pub type PiReg16 = crate::Reg<pi_reg_16::PiReg16Spec>;
#[doc = "DDR PHY Independent Register 16"]
pub mod pi_reg_16;
#[doc = "PI_REG_17 (rw) register accessor: DDR PHY Independent Register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_17`]
module"]
#[doc(alias = "PI_REG_17")]
pub type PiReg17 = crate::Reg<pi_reg_17::PiReg17Spec>;
#[doc = "DDR PHY Independent Register 17"]
pub mod pi_reg_17;
#[doc = "PI_REG_18 (rw) register accessor: DDR PHY Independent Register 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_18`]
module"]
#[doc(alias = "PI_REG_18")]
pub type PiReg18 = crate::Reg<pi_reg_18::PiReg18Spec>;
#[doc = "DDR PHY Independent Register 18"]
pub mod pi_reg_18;
#[doc = "PI_REG_19 (rw) register accessor: DDR PHY Independent Register 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_19`]
module"]
#[doc(alias = "PI_REG_19")]
pub type PiReg19 = crate::Reg<pi_reg_19::PiReg19Spec>;
#[doc = "DDR PHY Independent Register 19"]
pub mod pi_reg_19;
#[doc = "PI_REG_20 (rw) register accessor: DDR PHY Independent Register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_20`]
module"]
#[doc(alias = "PI_REG_20")]
pub type PiReg20 = crate::Reg<pi_reg_20::PiReg20Spec>;
#[doc = "DDR PHY Independent Register 20"]
pub mod pi_reg_20;
#[doc = "PI_REG_21 (rw) register accessor: DDR PHY Independent Register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_21`]
module"]
#[doc(alias = "PI_REG_21")]
pub type PiReg21 = crate::Reg<pi_reg_21::PiReg21Spec>;
#[doc = "DDR PHY Independent Register 21"]
pub mod pi_reg_21;
#[doc = "PI_REG_22 (rw) register accessor: DDR PHY Independent Register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_22`]
module"]
#[doc(alias = "PI_REG_22")]
pub type PiReg22 = crate::Reg<pi_reg_22::PiReg22Spec>;
#[doc = "DDR PHY Independent Register 22"]
pub mod pi_reg_22;
#[doc = "PI_REG_23 (rw) register accessor: DDR PHY Independent Register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_23`]
module"]
#[doc(alias = "PI_REG_23")]
pub type PiReg23 = crate::Reg<pi_reg_23::PiReg23Spec>;
#[doc = "DDR PHY Independent Register 23"]
pub mod pi_reg_23;
#[doc = "PI_REG_24 (rw) register accessor: DDR PHY Independent Register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_24`]
module"]
#[doc(alias = "PI_REG_24")]
pub type PiReg24 = crate::Reg<pi_reg_24::PiReg24Spec>;
#[doc = "DDR PHY Independent Register 24"]
pub mod pi_reg_24;
#[doc = "PI_REG_25 (rw) register accessor: DDR PHY Independent Register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_25`]
module"]
#[doc(alias = "PI_REG_25")]
pub type PiReg25 = crate::Reg<pi_reg_25::PiReg25Spec>;
#[doc = "DDR PHY Independent Register 25"]
pub mod pi_reg_25;
#[doc = "PI_REG_26 (rw) register accessor: DDR PHY Independent Register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_26`]
module"]
#[doc(alias = "PI_REG_26")]
pub type PiReg26 = crate::Reg<pi_reg_26::PiReg26Spec>;
#[doc = "DDR PHY Independent Register 26"]
pub mod pi_reg_26;
#[doc = "PI_REG_27 (rw) register accessor: DDR PHY Independent Register 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_27`]
module"]
#[doc(alias = "PI_REG_27")]
pub type PiReg27 = crate::Reg<pi_reg_27::PiReg27Spec>;
#[doc = "DDR PHY Independent Register 27"]
pub mod pi_reg_27;
#[doc = "PI_REG_28 (rw) register accessor: DDR PHY Independent Register 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_28`]
module"]
#[doc(alias = "PI_REG_28")]
pub type PiReg28 = crate::Reg<pi_reg_28::PiReg28Spec>;
#[doc = "DDR PHY Independent Register 28"]
pub mod pi_reg_28;
#[doc = "PI_REG_29 (rw) register accessor: DDR PHY Independent Register 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_29`]
module"]
#[doc(alias = "PI_REG_29")]
pub type PiReg29 = crate::Reg<pi_reg_29::PiReg29Spec>;
#[doc = "DDR PHY Independent Register 29"]
pub mod pi_reg_29;
#[doc = "PI_REG_30 (rw) register accessor: DDR PHY Independent Register 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_30`]
module"]
#[doc(alias = "PI_REG_30")]
pub type PiReg30 = crate::Reg<pi_reg_30::PiReg30Spec>;
#[doc = "DDR PHY Independent Register 30"]
pub mod pi_reg_30;
#[doc = "PI_REG_31 (rw) register accessor: DDR PHY Independent Register 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_31`]
module"]
#[doc(alias = "PI_REG_31")]
pub type PiReg31 = crate::Reg<pi_reg_31::PiReg31Spec>;
#[doc = "DDR PHY Independent Register 31"]
pub mod pi_reg_31;
#[doc = "PI_REG_32 (rw) register accessor: DDR PHY Independent Register 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_32`]
module"]
#[doc(alias = "PI_REG_32")]
pub type PiReg32 = crate::Reg<pi_reg_32::PiReg32Spec>;
#[doc = "DDR PHY Independent Register 32"]
pub mod pi_reg_32;
#[doc = "PI_REG_33 (rw) register accessor: DDR PHY Independent Register 33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_33`]
module"]
#[doc(alias = "PI_REG_33")]
pub type PiReg33 = crate::Reg<pi_reg_33::PiReg33Spec>;
#[doc = "DDR PHY Independent Register 33"]
pub mod pi_reg_33;
#[doc = "PI_REG_34 (rw) register accessor: DDR PHY Independent Register 34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_34`]
module"]
#[doc(alias = "PI_REG_34")]
pub type PiReg34 = crate::Reg<pi_reg_34::PiReg34Spec>;
#[doc = "DDR PHY Independent Register 34"]
pub mod pi_reg_34;
#[doc = "PI_REG_35 (rw) register accessor: DDR PHY Independent Register 35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_35`]
module"]
#[doc(alias = "PI_REG_35")]
pub type PiReg35 = crate::Reg<pi_reg_35::PiReg35Spec>;
#[doc = "DDR PHY Independent Register 35"]
pub mod pi_reg_35;
#[doc = "PI_REG_36 (rw) register accessor: DDR PHY Independent Register 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_36`]
module"]
#[doc(alias = "PI_REG_36")]
pub type PiReg36 = crate::Reg<pi_reg_36::PiReg36Spec>;
#[doc = "DDR PHY Independent Register 36"]
pub mod pi_reg_36;
#[doc = "PI_REG_37 (rw) register accessor: DDR PHY Independent Register 37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_37`]
module"]
#[doc(alias = "PI_REG_37")]
pub type PiReg37 = crate::Reg<pi_reg_37::PiReg37Spec>;
#[doc = "DDR PHY Independent Register 37"]
pub mod pi_reg_37;
#[doc = "PI_REG_38 (rw) register accessor: DDR PHY Independent Register 38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_38`]
module"]
#[doc(alias = "PI_REG_38")]
pub type PiReg38 = crate::Reg<pi_reg_38::PiReg38Spec>;
#[doc = "DDR PHY Independent Register 38"]
pub mod pi_reg_38;
#[doc = "PI_REG_39 (rw) register accessor: DDR PHY Independent Register 39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_39`]
module"]
#[doc(alias = "PI_REG_39")]
pub type PiReg39 = crate::Reg<pi_reg_39::PiReg39Spec>;
#[doc = "DDR PHY Independent Register 39"]
pub mod pi_reg_39;
#[doc = "PI_REG_40 (rw) register accessor: DDR PHY Independent Register 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_40`]
module"]
#[doc(alias = "PI_REG_40")]
pub type PiReg40 = crate::Reg<pi_reg_40::PiReg40Spec>;
#[doc = "DDR PHY Independent Register 40"]
pub mod pi_reg_40;
#[doc = "PI_REG_41 (rw) register accessor: DDR PHY Independent Register 41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_41`]
module"]
#[doc(alias = "PI_REG_41")]
pub type PiReg41 = crate::Reg<pi_reg_41::PiReg41Spec>;
#[doc = "DDR PHY Independent Register 41"]
pub mod pi_reg_41;
#[doc = "PI_REG_42 (rw) register accessor: DDR PHY Independent Register 42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_42`]
module"]
#[doc(alias = "PI_REG_42")]
pub type PiReg42 = crate::Reg<pi_reg_42::PiReg42Spec>;
#[doc = "DDR PHY Independent Register 42"]
pub mod pi_reg_42;
#[doc = "PI_REG_43 (rw) register accessor: DDR PHY Independent Register 43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_43`]
module"]
#[doc(alias = "PI_REG_43")]
pub type PiReg43 = crate::Reg<pi_reg_43::PiReg43Spec>;
#[doc = "DDR PHY Independent Register 43"]
pub mod pi_reg_43;
#[doc = "PI_REG_44 (rw) register accessor: DDR PHY Independent Register 44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_44`]
module"]
#[doc(alias = "PI_REG_44")]
pub type PiReg44 = crate::Reg<pi_reg_44::PiReg44Spec>;
#[doc = "DDR PHY Independent Register 44"]
pub mod pi_reg_44;
#[doc = "PI_REG_45 (rw) register accessor: DDR PHY Independent Register 45\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_45`]
module"]
#[doc(alias = "PI_REG_45")]
pub type PiReg45 = crate::Reg<pi_reg_45::PiReg45Spec>;
#[doc = "DDR PHY Independent Register 45"]
pub mod pi_reg_45;
#[doc = "PI_REG_46 (rw) register accessor: DDR PHY Independent Register 46\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_46`]
module"]
#[doc(alias = "PI_REG_46")]
pub type PiReg46 = crate::Reg<pi_reg_46::PiReg46Spec>;
#[doc = "DDR PHY Independent Register 46"]
pub mod pi_reg_46;
#[doc = "PI_REG_47 (rw) register accessor: DDR PHY Independent Register 47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_47`]
module"]
#[doc(alias = "PI_REG_47")]
pub type PiReg47 = crate::Reg<pi_reg_47::PiReg47Spec>;
#[doc = "DDR PHY Independent Register 47"]
pub mod pi_reg_47;
#[doc = "PI_REG_48 (rw) register accessor: DDR PHY Independent Register 48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_48`]
module"]
#[doc(alias = "PI_REG_48")]
pub type PiReg48 = crate::Reg<pi_reg_48::PiReg48Spec>;
#[doc = "DDR PHY Independent Register 48"]
pub mod pi_reg_48;
#[doc = "PI_REG_49 (rw) register accessor: DDR PHY Independent Register 49\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_49`]
module"]
#[doc(alias = "PI_REG_49")]
pub type PiReg49 = crate::Reg<pi_reg_49::PiReg49Spec>;
#[doc = "DDR PHY Independent Register 49"]
pub mod pi_reg_49;
#[doc = "PI_REG_50 (r) register accessor: DDR PHY Independent Register 50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_50::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_50`]
module"]
#[doc(alias = "PI_REG_50")]
pub type PiReg50 = crate::Reg<pi_reg_50::PiReg50Spec>;
#[doc = "DDR PHY Independent Register 50"]
pub mod pi_reg_50;
#[doc = "PI_REG_51 (r) register accessor: DDR PHY Independent Register 51\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_51::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_51`]
module"]
#[doc(alias = "PI_REG_51")]
pub type PiReg51 = crate::Reg<pi_reg_51::PiReg51Spec>;
#[doc = "DDR PHY Independent Register 51"]
pub mod pi_reg_51;
#[doc = "PI_REG_52 (rw) register accessor: DDR PHY Independent Register 52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_52`]
module"]
#[doc(alias = "PI_REG_52")]
pub type PiReg52 = crate::Reg<pi_reg_52::PiReg52Spec>;
#[doc = "DDR PHY Independent Register 52"]
pub mod pi_reg_52;
#[doc = "PI_REG_53 (rw) register accessor: DDR PHY Independent Register 53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_53`]
module"]
#[doc(alias = "PI_REG_53")]
pub type PiReg53 = crate::Reg<pi_reg_53::PiReg53Spec>;
#[doc = "DDR PHY Independent Register 53"]
pub mod pi_reg_53;
#[doc = "PI_REG_54 (rw) register accessor: DDR PHY Independent Register 54\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_54::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_54`]
module"]
#[doc(alias = "PI_REG_54")]
pub type PiReg54 = crate::Reg<pi_reg_54::PiReg54Spec>;
#[doc = "DDR PHY Independent Register 54"]
pub mod pi_reg_54;
#[doc = "PI_REG_55 (rw) register accessor: DDR PHY Independent Register 55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_55`]
module"]
#[doc(alias = "PI_REG_55")]
pub type PiReg55 = crate::Reg<pi_reg_55::PiReg55Spec>;
#[doc = "DDR PHY Independent Register 55"]
pub mod pi_reg_55;
#[doc = "PI_REG_56 (rw) register accessor: DDR PHY Independent Register 56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_56::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_56`]
module"]
#[doc(alias = "PI_REG_56")]
pub type PiReg56 = crate::Reg<pi_reg_56::PiReg56Spec>;
#[doc = "DDR PHY Independent Register 56"]
pub mod pi_reg_56;
#[doc = "PI_REG_57 (rw) register accessor: DDR PHY Independent Register 57\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_57::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_57::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_57`]
module"]
#[doc(alias = "PI_REG_57")]
pub type PiReg57 = crate::Reg<pi_reg_57::PiReg57Spec>;
#[doc = "DDR PHY Independent Register 57"]
pub mod pi_reg_57;
#[doc = "PI_REG_58 (rw) register accessor: DDR PHY Independent Register 58\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_58::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_58::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_58`]
module"]
#[doc(alias = "PI_REG_58")]
pub type PiReg58 = crate::Reg<pi_reg_58::PiReg58Spec>;
#[doc = "DDR PHY Independent Register 58"]
pub mod pi_reg_58;
#[doc = "PI_REG_59 (rw) register accessor: DDR PHY Independent Register 59\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_59::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_59`]
module"]
#[doc(alias = "PI_REG_59")]
pub type PiReg59 = crate::Reg<pi_reg_59::PiReg59Spec>;
#[doc = "DDR PHY Independent Register 59"]
pub mod pi_reg_59;
#[doc = "PI_REG_60 (rw) register accessor: DDR PHY Independent Register 60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_60::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_60`]
module"]
#[doc(alias = "PI_REG_60")]
pub type PiReg60 = crate::Reg<pi_reg_60::PiReg60Spec>;
#[doc = "DDR PHY Independent Register 60"]
pub mod pi_reg_60;
#[doc = "PI_REG_61 (rw) register accessor: DDR PHY Independent Register 61\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_61::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_61::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_61`]
module"]
#[doc(alias = "PI_REG_61")]
pub type PiReg61 = crate::Reg<pi_reg_61::PiReg61Spec>;
#[doc = "DDR PHY Independent Register 61"]
pub mod pi_reg_61;
#[doc = "PI_REG_62 (rw) register accessor: DDR PHY Independent Register 62\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_62::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_62::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_62`]
module"]
#[doc(alias = "PI_REG_62")]
pub type PiReg62 = crate::Reg<pi_reg_62::PiReg62Spec>;
#[doc = "DDR PHY Independent Register 62"]
pub mod pi_reg_62;
#[doc = "PI_REG_63 (rw) register accessor: DDR PHY Independent Register 63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_63::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_63`]
module"]
#[doc(alias = "PI_REG_63")]
pub type PiReg63 = crate::Reg<pi_reg_63::PiReg63Spec>;
#[doc = "DDR PHY Independent Register 63"]
pub mod pi_reg_63;
#[doc = "PI_REG_64 (rw) register accessor: DDR PHY Independent Register 64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_64`]
module"]
#[doc(alias = "PI_REG_64")]
pub type PiReg64 = crate::Reg<pi_reg_64::PiReg64Spec>;
#[doc = "DDR PHY Independent Register 64"]
pub mod pi_reg_64;
#[doc = "PI_REG_65 (rw) register accessor: DDR PHY Independent Register 65\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_65::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_65::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_65`]
module"]
#[doc(alias = "PI_REG_65")]
pub type PiReg65 = crate::Reg<pi_reg_65::PiReg65Spec>;
#[doc = "DDR PHY Independent Register 65"]
pub mod pi_reg_65;
#[doc = "PI_REG_66 (rw) register accessor: DDR PHY Independent Register 66\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_66::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_66::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_66`]
module"]
#[doc(alias = "PI_REG_66")]
pub type PiReg66 = crate::Reg<pi_reg_66::PiReg66Spec>;
#[doc = "DDR PHY Independent Register 66"]
pub mod pi_reg_66;
#[doc = "PI_REG_67 (rw) register accessor: DDR PHY Independent Register 67\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_67`]
module"]
#[doc(alias = "PI_REG_67")]
pub type PiReg67 = crate::Reg<pi_reg_67::PiReg67Spec>;
#[doc = "DDR PHY Independent Register 67"]
pub mod pi_reg_67;
#[doc = "PI_REG_68 (rw) register accessor: DDR PHY Independent Register 68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_68::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_68::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_68`]
module"]
#[doc(alias = "PI_REG_68")]
pub type PiReg68 = crate::Reg<pi_reg_68::PiReg68Spec>;
#[doc = "DDR PHY Independent Register 68"]
pub mod pi_reg_68;
#[doc = "PI_REG_69 (rw) register accessor: DDR PHY Independent Register 69\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_69::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_69::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_69`]
module"]
#[doc(alias = "PI_REG_69")]
pub type PiReg69 = crate::Reg<pi_reg_69::PiReg69Spec>;
#[doc = "DDR PHY Independent Register 69"]
pub mod pi_reg_69;
#[doc = "PI_REG_70 (rw) register accessor: DDR PHY Independent Register 70\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_70::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_70::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_70`]
module"]
#[doc(alias = "PI_REG_70")]
pub type PiReg70 = crate::Reg<pi_reg_70::PiReg70Spec>;
#[doc = "DDR PHY Independent Register 70"]
pub mod pi_reg_70;
#[doc = "PI_REG_71 (rw) register accessor: DDR PHY Independent Register 71\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_71::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_71::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_71`]
module"]
#[doc(alias = "PI_REG_71")]
pub type PiReg71 = crate::Reg<pi_reg_71::PiReg71Spec>;
#[doc = "DDR PHY Independent Register 71"]
pub mod pi_reg_71;
#[doc = "PI_REG_72 (rw) register accessor: DDR PHY Independent Register 72\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_72::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_72::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_72`]
module"]
#[doc(alias = "PI_REG_72")]
pub type PiReg72 = crate::Reg<pi_reg_72::PiReg72Spec>;
#[doc = "DDR PHY Independent Register 72"]
pub mod pi_reg_72;
#[doc = "PI_REG_73 (rw) register accessor: DDR PHY Independent Register 73\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_73::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_73::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_73`]
module"]
#[doc(alias = "PI_REG_73")]
pub type PiReg73 = crate::Reg<pi_reg_73::PiReg73Spec>;
#[doc = "DDR PHY Independent Register 73"]
pub mod pi_reg_73;
#[doc = "PI_REG_74 (rw) register accessor: DDR PHY Independent Register 74\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_74::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_74::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_74`]
module"]
#[doc(alias = "PI_REG_74")]
pub type PiReg74 = crate::Reg<pi_reg_74::PiReg74Spec>;
#[doc = "DDR PHY Independent Register 74"]
pub mod pi_reg_74;
#[doc = "PI_REG_75 (rw) register accessor: DDR PHY Independent Register 75\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_75::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_75::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_75`]
module"]
#[doc(alias = "PI_REG_75")]
pub type PiReg75 = crate::Reg<pi_reg_75::PiReg75Spec>;
#[doc = "DDR PHY Independent Register 75"]
pub mod pi_reg_75;
#[doc = "PI_REG_76 (rw) register accessor: DDR PHY Independent Register 76\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_76::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_76::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_76`]
module"]
#[doc(alias = "PI_REG_76")]
pub type PiReg76 = crate::Reg<pi_reg_76::PiReg76Spec>;
#[doc = "DDR PHY Independent Register 76"]
pub mod pi_reg_76;
#[doc = "PI_REG_77 (rw) register accessor: DDR PHY Independent Register 77\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_77::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_77::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_77`]
module"]
#[doc(alias = "PI_REG_77")]
pub type PiReg77 = crate::Reg<pi_reg_77::PiReg77Spec>;
#[doc = "DDR PHY Independent Register 77"]
pub mod pi_reg_77;
#[doc = "PI_REG_78 (rw) register accessor: DDR PHY Independent Register 78\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_78::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_78::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_78`]
module"]
#[doc(alias = "PI_REG_78")]
pub type PiReg78 = crate::Reg<pi_reg_78::PiReg78Spec>;
#[doc = "DDR PHY Independent Register 78"]
pub mod pi_reg_78;
#[doc = "PI_REG_79 (rw) register accessor: DDR PHY Independent Register 79\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_79::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_79::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_79`]
module"]
#[doc(alias = "PI_REG_79")]
pub type PiReg79 = crate::Reg<pi_reg_79::PiReg79Spec>;
#[doc = "DDR PHY Independent Register 79"]
pub mod pi_reg_79;
#[doc = "PI_REG_80 (rw) register accessor: DDR PHY Independent Register 80\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_80::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_80::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_80`]
module"]
#[doc(alias = "PI_REG_80")]
pub type PiReg80 = crate::Reg<pi_reg_80::PiReg80Spec>;
#[doc = "DDR PHY Independent Register 80"]
pub mod pi_reg_80;
#[doc = "PI_REG_81 (rw) register accessor: DDR PHY Independent Register 81\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_81::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_81::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_81`]
module"]
#[doc(alias = "PI_REG_81")]
pub type PiReg81 = crate::Reg<pi_reg_81::PiReg81Spec>;
#[doc = "DDR PHY Independent Register 81"]
pub mod pi_reg_81;
#[doc = "PI_REG_82 (rw) register accessor: DDR PHY Independent Register 82\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_82::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_82::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_82`]
module"]
#[doc(alias = "PI_REG_82")]
pub type PiReg82 = crate::Reg<pi_reg_82::PiReg82Spec>;
#[doc = "DDR PHY Independent Register 82"]
pub mod pi_reg_82;
#[doc = "PI_REG_83 (rw) register accessor: DDR PHY Independent Register 83\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_83::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_83::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_83`]
module"]
#[doc(alias = "PI_REG_83")]
pub type PiReg83 = crate::Reg<pi_reg_83::PiReg83Spec>;
#[doc = "DDR PHY Independent Register 83"]
pub mod pi_reg_83;
#[doc = "PI_REG_84 (rw) register accessor: DDR PHY Independent Register 84\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_84::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_84::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_84`]
module"]
#[doc(alias = "PI_REG_84")]
pub type PiReg84 = crate::Reg<pi_reg_84::PiReg84Spec>;
#[doc = "DDR PHY Independent Register 84"]
pub mod pi_reg_84;
#[doc = "PI_REG_85 (rw) register accessor: DDR PHY Independent Register 85\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_85::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_85::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_85`]
module"]
#[doc(alias = "PI_REG_85")]
pub type PiReg85 = crate::Reg<pi_reg_85::PiReg85Spec>;
#[doc = "DDR PHY Independent Register 85"]
pub mod pi_reg_85;
#[doc = "PI_REG_86 (rw) register accessor: DDR PHY Independent Register 86\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_86::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_86::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_86`]
module"]
#[doc(alias = "PI_REG_86")]
pub type PiReg86 = crate::Reg<pi_reg_86::PiReg86Spec>;
#[doc = "DDR PHY Independent Register 86"]
pub mod pi_reg_86;
#[doc = "PI_REG_87 (rw) register accessor: DDR PHY Independent Register 87\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_87::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_87::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_87`]
module"]
#[doc(alias = "PI_REG_87")]
pub type PiReg87 = crate::Reg<pi_reg_87::PiReg87Spec>;
#[doc = "DDR PHY Independent Register 87"]
pub mod pi_reg_87;
#[doc = "PI_REG_88 (rw) register accessor: DDR PHY Independent Register 88\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_88::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_88::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_88`]
module"]
#[doc(alias = "PI_REG_88")]
pub type PiReg88 = crate::Reg<pi_reg_88::PiReg88Spec>;
#[doc = "DDR PHY Independent Register 88"]
pub mod pi_reg_88;
#[doc = "PI_REG_89 (rw) register accessor: DDR PHY Independent Register 89\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_89::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_89::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_89`]
module"]
#[doc(alias = "PI_REG_89")]
pub type PiReg89 = crate::Reg<pi_reg_89::PiReg89Spec>;
#[doc = "DDR PHY Independent Register 89"]
pub mod pi_reg_89;
#[doc = "PI_REG_90 (rw) register accessor: DDR PHY Independent Register 90\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_90::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_90::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_90`]
module"]
#[doc(alias = "PI_REG_90")]
pub type PiReg90 = crate::Reg<pi_reg_90::PiReg90Spec>;
#[doc = "DDR PHY Independent Register 90"]
pub mod pi_reg_90;
#[doc = "PI_REG_91 (rw) register accessor: DDR PHY Independent Register 91\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_91::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_91::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_91`]
module"]
#[doc(alias = "PI_REG_91")]
pub type PiReg91 = crate::Reg<pi_reg_91::PiReg91Spec>;
#[doc = "DDR PHY Independent Register 91"]
pub mod pi_reg_91;
#[doc = "PI_REG_92 (rw) register accessor: DDR PHY Independent Register 92\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_92::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_92::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_92`]
module"]
#[doc(alias = "PI_REG_92")]
pub type PiReg92 = crate::Reg<pi_reg_92::PiReg92Spec>;
#[doc = "DDR PHY Independent Register 92"]
pub mod pi_reg_92;
#[doc = "PI_REG_93 (rw) register accessor: DDR PHY Independent Register 93\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_93::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_93::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_93`]
module"]
#[doc(alias = "PI_REG_93")]
pub type PiReg93 = crate::Reg<pi_reg_93::PiReg93Spec>;
#[doc = "DDR PHY Independent Register 93"]
pub mod pi_reg_93;
#[doc = "PI_REG_94 (rw) register accessor: DDR PHY Independent Register 94\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_94::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_94::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_94`]
module"]
#[doc(alias = "PI_REG_94")]
pub type PiReg94 = crate::Reg<pi_reg_94::PiReg94Spec>;
#[doc = "DDR PHY Independent Register 94"]
pub mod pi_reg_94;
#[doc = "PI_REG_95 (rw) register accessor: DDR PHY Independent Register 95\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_95::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_95::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_95`]
module"]
#[doc(alias = "PI_REG_95")]
pub type PiReg95 = crate::Reg<pi_reg_95::PiReg95Spec>;
#[doc = "DDR PHY Independent Register 95"]
pub mod pi_reg_95;
#[doc = "PI_REG_96 (rw) register accessor: DDR PHY Independent Register 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_96::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_96::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_96`]
module"]
#[doc(alias = "PI_REG_96")]
pub type PiReg96 = crate::Reg<pi_reg_96::PiReg96Spec>;
#[doc = "DDR PHY Independent Register 96"]
pub mod pi_reg_96;
#[doc = "PI_REG_97 (rw) register accessor: DDR PHY Independent Register 97\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_97::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_97::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_97`]
module"]
#[doc(alias = "PI_REG_97")]
pub type PiReg97 = crate::Reg<pi_reg_97::PiReg97Spec>;
#[doc = "DDR PHY Independent Register 97"]
pub mod pi_reg_97;
#[doc = "PI_REG_98 (rw) register accessor: DDR PHY Independent Register 98\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_98::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_98::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_98`]
module"]
#[doc(alias = "PI_REG_98")]
pub type PiReg98 = crate::Reg<pi_reg_98::PiReg98Spec>;
#[doc = "DDR PHY Independent Register 98"]
pub mod pi_reg_98;
#[doc = "PI_REG_99 (rw) register accessor: DDR PHY Independent Register 99\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_99::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_99::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_99`]
module"]
#[doc(alias = "PI_REG_99")]
pub type PiReg99 = crate::Reg<pi_reg_99::PiReg99Spec>;
#[doc = "DDR PHY Independent Register 99"]
pub mod pi_reg_99;
#[doc = "PI_REG_100 (rw) register accessor: DDR PHY Independent Register 100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_100::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_100::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_100`]
module"]
#[doc(alias = "PI_REG_100")]
pub type PiReg100 = crate::Reg<pi_reg_100::PiReg100Spec>;
#[doc = "DDR PHY Independent Register 100"]
pub mod pi_reg_100;
#[doc = "PI_REG_101 (rw) register accessor: DDR PHY Independent Register 101\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_101::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_101::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_101`]
module"]
#[doc(alias = "PI_REG_101")]
pub type PiReg101 = crate::Reg<pi_reg_101::PiReg101Spec>;
#[doc = "DDR PHY Independent Register 101"]
pub mod pi_reg_101;
#[doc = "PI_REG_102 (rw) register accessor: DDR PHY Independent Register 102\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_102::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_102::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_102`]
module"]
#[doc(alias = "PI_REG_102")]
pub type PiReg102 = crate::Reg<pi_reg_102::PiReg102Spec>;
#[doc = "DDR PHY Independent Register 102"]
pub mod pi_reg_102;
#[doc = "PI_REG_103 (rw) register accessor: DDR PHY Independent Register 103\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_103::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_103::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_103`]
module"]
#[doc(alias = "PI_REG_103")]
pub type PiReg103 = crate::Reg<pi_reg_103::PiReg103Spec>;
#[doc = "DDR PHY Independent Register 103"]
pub mod pi_reg_103;
#[doc = "PI_REG_104 (rw) register accessor: DDR PHY Independent Register 104\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_104::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_104::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_104`]
module"]
#[doc(alias = "PI_REG_104")]
pub type PiReg104 = crate::Reg<pi_reg_104::PiReg104Spec>;
#[doc = "DDR PHY Independent Register 104"]
pub mod pi_reg_104;
#[doc = "PI_REG_105 (rw) register accessor: DDR PHY Independent Register 105\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_105::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_105::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_105`]
module"]
#[doc(alias = "PI_REG_105")]
pub type PiReg105 = crate::Reg<pi_reg_105::PiReg105Spec>;
#[doc = "DDR PHY Independent Register 105"]
pub mod pi_reg_105;
#[doc = "PI_REG_106 (rw) register accessor: DDR PHY Independent Register 106\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_106::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_106::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_106`]
module"]
#[doc(alias = "PI_REG_106")]
pub type PiReg106 = crate::Reg<pi_reg_106::PiReg106Spec>;
#[doc = "DDR PHY Independent Register 106"]
pub mod pi_reg_106;
#[doc = "PI_REG_107 (rw) register accessor: DDR PHY Independent Register 107\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_107::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_107::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_107`]
module"]
#[doc(alias = "PI_REG_107")]
pub type PiReg107 = crate::Reg<pi_reg_107::PiReg107Spec>;
#[doc = "DDR PHY Independent Register 107"]
pub mod pi_reg_107;
#[doc = "PI_REG_108 (rw) register accessor: DDR PHY Independent Register 108\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_108::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_108::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_108`]
module"]
#[doc(alias = "PI_REG_108")]
pub type PiReg108 = crate::Reg<pi_reg_108::PiReg108Spec>;
#[doc = "DDR PHY Independent Register 108"]
pub mod pi_reg_108;
#[doc = "PI_REG_109 (rw) register accessor: DDR PHY Independent Register 109\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_109::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_109::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_109`]
module"]
#[doc(alias = "PI_REG_109")]
pub type PiReg109 = crate::Reg<pi_reg_109::PiReg109Spec>;
#[doc = "DDR PHY Independent Register 109"]
pub mod pi_reg_109;
#[doc = "PI_REG_110 (rw) register accessor: DDR PHY Independent Register 110\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_110::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_110::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_110`]
module"]
#[doc(alias = "PI_REG_110")]
pub type PiReg110 = crate::Reg<pi_reg_110::PiReg110Spec>;
#[doc = "DDR PHY Independent Register 110"]
pub mod pi_reg_110;
#[doc = "PI_REG_111 (rw) register accessor: DDR PHY Independent Register 111\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_111::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_111::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_111`]
module"]
#[doc(alias = "PI_REG_111")]
pub type PiReg111 = crate::Reg<pi_reg_111::PiReg111Spec>;
#[doc = "DDR PHY Independent Register 111"]
pub mod pi_reg_111;
#[doc = "PI_REG_112 (rw) register accessor: DDR PHY Independent Register 112\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_112::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_112::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_112`]
module"]
#[doc(alias = "PI_REG_112")]
pub type PiReg112 = crate::Reg<pi_reg_112::PiReg112Spec>;
#[doc = "DDR PHY Independent Register 112"]
pub mod pi_reg_112;
#[doc = "PI_REG_113 (rw) register accessor: DDR PHY Independent Register 113\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_113::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_113::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_113`]
module"]
#[doc(alias = "PI_REG_113")]
pub type PiReg113 = crate::Reg<pi_reg_113::PiReg113Spec>;
#[doc = "DDR PHY Independent Register 113"]
pub mod pi_reg_113;
#[doc = "PI_REG_114 (rw) register accessor: DDR PHY Independent Register 114\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_114::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_114::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_114`]
module"]
#[doc(alias = "PI_REG_114")]
pub type PiReg114 = crate::Reg<pi_reg_114::PiReg114Spec>;
#[doc = "DDR PHY Independent Register 114"]
pub mod pi_reg_114;
#[doc = "PI_REG_115 (rw) register accessor: DDR PHY Independent Register 115\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_115::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_115::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_115`]
module"]
#[doc(alias = "PI_REG_115")]
pub type PiReg115 = crate::Reg<pi_reg_115::PiReg115Spec>;
#[doc = "DDR PHY Independent Register 115"]
pub mod pi_reg_115;
#[doc = "PI_REG_116 (rw) register accessor: DDR PHY Independent Register 116\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_116::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_116::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_116`]
module"]
#[doc(alias = "PI_REG_116")]
pub type PiReg116 = crate::Reg<pi_reg_116::PiReg116Spec>;
#[doc = "DDR PHY Independent Register 116"]
pub mod pi_reg_116;
#[doc = "PI_REG_117 (rw) register accessor: DDR PHY Independent Register 117\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_117::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_117::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_117`]
module"]
#[doc(alias = "PI_REG_117")]
pub type PiReg117 = crate::Reg<pi_reg_117::PiReg117Spec>;
#[doc = "DDR PHY Independent Register 117"]
pub mod pi_reg_117;
#[doc = "PI_REG_118 (rw) register accessor: DDR PHY Independent Register 118\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_118::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_118::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_118`]
module"]
#[doc(alias = "PI_REG_118")]
pub type PiReg118 = crate::Reg<pi_reg_118::PiReg118Spec>;
#[doc = "DDR PHY Independent Register 118"]
pub mod pi_reg_118;
#[doc = "PI_REG_119 (rw) register accessor: DDR PHY Independent Register 119\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_119::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_119::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_119`]
module"]
#[doc(alias = "PI_REG_119")]
pub type PiReg119 = crate::Reg<pi_reg_119::PiReg119Spec>;
#[doc = "DDR PHY Independent Register 119"]
pub mod pi_reg_119;
#[doc = "PI_REG_120 (rw) register accessor: DDR PHY Independent Register 120\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_120::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_120::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_120`]
module"]
#[doc(alias = "PI_REG_120")]
pub type PiReg120 = crate::Reg<pi_reg_120::PiReg120Spec>;
#[doc = "DDR PHY Independent Register 120"]
pub mod pi_reg_120;
#[doc = "PI_REG_121 (rw) register accessor: DDR PHY Independent Register 121\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_121::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_121::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_121`]
module"]
#[doc(alias = "PI_REG_121")]
pub type PiReg121 = crate::Reg<pi_reg_121::PiReg121Spec>;
#[doc = "DDR PHY Independent Register 121"]
pub mod pi_reg_121;
#[doc = "PI_REG_122 (rw) register accessor: DDR PHY Independent Register 122\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_122::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_122::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_122`]
module"]
#[doc(alias = "PI_REG_122")]
pub type PiReg122 = crate::Reg<pi_reg_122::PiReg122Spec>;
#[doc = "DDR PHY Independent Register 122"]
pub mod pi_reg_122;
#[doc = "PI_REG_123 (rw) register accessor: DDR PHY Independent Register 123\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_123::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_123::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_123`]
module"]
#[doc(alias = "PI_REG_123")]
pub type PiReg123 = crate::Reg<pi_reg_123::PiReg123Spec>;
#[doc = "DDR PHY Independent Register 123"]
pub mod pi_reg_123;
#[doc = "PI_REG_124 (rw) register accessor: DDR PHY Independent Register 124\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_124::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_124::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_124`]
module"]
#[doc(alias = "PI_REG_124")]
pub type PiReg124 = crate::Reg<pi_reg_124::PiReg124Spec>;
#[doc = "DDR PHY Independent Register 124"]
pub mod pi_reg_124;
#[doc = "PI_REG_125 (rw) register accessor: DDR PHY Independent Register 125\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_125::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_125::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_125`]
module"]
#[doc(alias = "PI_REG_125")]
pub type PiReg125 = crate::Reg<pi_reg_125::PiReg125Spec>;
#[doc = "DDR PHY Independent Register 125"]
pub mod pi_reg_125;
#[doc = "PI_REG_126 (rw) register accessor: DDR PHY Independent Register 126\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_126::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_126::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_126`]
module"]
#[doc(alias = "PI_REG_126")]
pub type PiReg126 = crate::Reg<pi_reg_126::PiReg126Spec>;
#[doc = "DDR PHY Independent Register 126"]
pub mod pi_reg_126;
#[doc = "PI_REG_127 (rw) register accessor: DDR PHY Independent Register 127\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_127::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_127::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_127`]
module"]
#[doc(alias = "PI_REG_127")]
pub type PiReg127 = crate::Reg<pi_reg_127::PiReg127Spec>;
#[doc = "DDR PHY Independent Register 127"]
pub mod pi_reg_127;
#[doc = "PI_REG_128 (rw) register accessor: DDR PHY Independent Register 128\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_128::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_128::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_128`]
module"]
#[doc(alias = "PI_REG_128")]
pub type PiReg128 = crate::Reg<pi_reg_128::PiReg128Spec>;
#[doc = "DDR PHY Independent Register 128"]
pub mod pi_reg_128;
#[doc = "PI_REG_129 (rw) register accessor: DDR PHY Independent Register 129\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_129::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_129::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_129`]
module"]
#[doc(alias = "PI_REG_129")]
pub type PiReg129 = crate::Reg<pi_reg_129::PiReg129Spec>;
#[doc = "DDR PHY Independent Register 129"]
pub mod pi_reg_129;
#[doc = "PI_REG_130 (rw) register accessor: DDR PHY Independent Register 130\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_130::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_130::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_130`]
module"]
#[doc(alias = "PI_REG_130")]
pub type PiReg130 = crate::Reg<pi_reg_130::PiReg130Spec>;
#[doc = "DDR PHY Independent Register 130"]
pub mod pi_reg_130;
#[doc = "PI_REG_131 (rw) register accessor: DDR PHY Independent Register 131\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_131::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_131::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_131`]
module"]
#[doc(alias = "PI_REG_131")]
pub type PiReg131 = crate::Reg<pi_reg_131::PiReg131Spec>;
#[doc = "DDR PHY Independent Register 131"]
pub mod pi_reg_131;
#[doc = "PI_REG_132 (rw) register accessor: DDR PHY Independent Register 132\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_132::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_132::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_132`]
module"]
#[doc(alias = "PI_REG_132")]
pub type PiReg132 = crate::Reg<pi_reg_132::PiReg132Spec>;
#[doc = "DDR PHY Independent Register 132"]
pub mod pi_reg_132;
#[doc = "PI_REG_133 (rw) register accessor: DDR PHY Independent Register 133\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_133::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_133::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_133`]
module"]
#[doc(alias = "PI_REG_133")]
pub type PiReg133 = crate::Reg<pi_reg_133::PiReg133Spec>;
#[doc = "DDR PHY Independent Register 133"]
pub mod pi_reg_133;
#[doc = "PI_REG_134 (rw) register accessor: DDR PHY Independent Register 134\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_134::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_134::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_134`]
module"]
#[doc(alias = "PI_REG_134")]
pub type PiReg134 = crate::Reg<pi_reg_134::PiReg134Spec>;
#[doc = "DDR PHY Independent Register 134"]
pub mod pi_reg_134;
#[doc = "PI_REG_135 (rw) register accessor: DDR PHY Independent Register 135\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_135::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_135::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_135`]
module"]
#[doc(alias = "PI_REG_135")]
pub type PiReg135 = crate::Reg<pi_reg_135::PiReg135Spec>;
#[doc = "DDR PHY Independent Register 135"]
pub mod pi_reg_135;
#[doc = "PI_REG_136 (rw) register accessor: DDR PHY Independent Register 136\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_136::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_136::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_136`]
module"]
#[doc(alias = "PI_REG_136")]
pub type PiReg136 = crate::Reg<pi_reg_136::PiReg136Spec>;
#[doc = "DDR PHY Independent Register 136"]
pub mod pi_reg_136;
#[doc = "PI_REG_137 (rw) register accessor: DDR PHY Independent Register 137\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_137::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_137::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_137`]
module"]
#[doc(alias = "PI_REG_137")]
pub type PiReg137 = crate::Reg<pi_reg_137::PiReg137Spec>;
#[doc = "DDR PHY Independent Register 137"]
pub mod pi_reg_137;
#[doc = "PI_REG_138 (rw) register accessor: DDR PHY Independent Register 138\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_138::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_138::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_138`]
module"]
#[doc(alias = "PI_REG_138")]
pub type PiReg138 = crate::Reg<pi_reg_138::PiReg138Spec>;
#[doc = "DDR PHY Independent Register 138"]
pub mod pi_reg_138;
#[doc = "PI_REG_139 (rw) register accessor: DDR PHY Independent Register 139\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_139::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_139::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_139`]
module"]
#[doc(alias = "PI_REG_139")]
pub type PiReg139 = crate::Reg<pi_reg_139::PiReg139Spec>;
#[doc = "DDR PHY Independent Register 139"]
pub mod pi_reg_139;
#[doc = "PI_REG_140 (rw) register accessor: DDR PHY Independent Register 140\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_140::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_140::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_140`]
module"]
#[doc(alias = "PI_REG_140")]
pub type PiReg140 = crate::Reg<pi_reg_140::PiReg140Spec>;
#[doc = "DDR PHY Independent Register 140"]
pub mod pi_reg_140;
#[doc = "PI_REG_155 (rw) register accessor: DDR PHY Independent Register 155\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_155::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_155::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_155`]
module"]
#[doc(alias = "PI_REG_155")]
pub type PiReg155 = crate::Reg<pi_reg_155::PiReg155Spec>;
#[doc = "DDR PHY Independent Register 155"]
pub mod pi_reg_155;
#[doc = "PI_REG_156 (rw) register accessor: DDR PHY Independent Register 156\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_156::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_156::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_156`]
module"]
#[doc(alias = "PI_REG_156")]
pub type PiReg156 = crate::Reg<pi_reg_156::PiReg156Spec>;
#[doc = "DDR PHY Independent Register 156"]
pub mod pi_reg_156;
#[doc = "PI_REG_157 (rw) register accessor: DDR PHY Independent Register 157\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_157::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_157::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_157`]
module"]
#[doc(alias = "PI_REG_157")]
pub type PiReg157 = crate::Reg<pi_reg_157::PiReg157Spec>;
#[doc = "DDR PHY Independent Register 157"]
pub mod pi_reg_157;
#[doc = "PI_REG_158 (rw) register accessor: DDR PHY Independent Register 158\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_158::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_158::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_158`]
module"]
#[doc(alias = "PI_REG_158")]
pub type PiReg158 = crate::Reg<pi_reg_158::PiReg158Spec>;
#[doc = "DDR PHY Independent Register 158"]
pub mod pi_reg_158;
#[doc = "PI_REG_159 (rw) register accessor: DDR PHY Independent Register 159\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_159::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_159::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_159`]
module"]
#[doc(alias = "PI_REG_159")]
pub type PiReg159 = crate::Reg<pi_reg_159::PiReg159Spec>;
#[doc = "DDR PHY Independent Register 159"]
pub mod pi_reg_159;
#[doc = "PI_REG_160 (rw) register accessor: DDR PHY Independent Register 160\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_160::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_160::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_160`]
module"]
#[doc(alias = "PI_REG_160")]
pub type PiReg160 = crate::Reg<pi_reg_160::PiReg160Spec>;
#[doc = "DDR PHY Independent Register 160"]
pub mod pi_reg_160;
#[doc = "PI_REG_161 (rw) register accessor: DDR PHY Independent Register 161\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_161::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_161::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_161`]
module"]
#[doc(alias = "PI_REG_161")]
pub type PiReg161 = crate::Reg<pi_reg_161::PiReg161Spec>;
#[doc = "DDR PHY Independent Register 161"]
pub mod pi_reg_161;
#[doc = "PI_REG_162 (rw) register accessor: DDR PHY Independent Register 162\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_162::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_162::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_162`]
module"]
#[doc(alias = "PI_REG_162")]
pub type PiReg162 = crate::Reg<pi_reg_162::PiReg162Spec>;
#[doc = "DDR PHY Independent Register 162"]
pub mod pi_reg_162;
#[doc = "PI_REG_163 (rw) register accessor: DDR PHY Independent Register 163\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_163::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_163::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_163`]
module"]
#[doc(alias = "PI_REG_163")]
pub type PiReg163 = crate::Reg<pi_reg_163::PiReg163Spec>;
#[doc = "DDR PHY Independent Register 163"]
pub mod pi_reg_163;
#[doc = "PI_REG_164 (rw) register accessor: DDR PHY Independent Register 164\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_164::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_164::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_164`]
module"]
#[doc(alias = "PI_REG_164")]
pub type PiReg164 = crate::Reg<pi_reg_164::PiReg164Spec>;
#[doc = "DDR PHY Independent Register 164"]
pub mod pi_reg_164;
#[doc = "PI_REG_165 (rw) register accessor: DDR PHY Independent Register 165\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_165::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_165::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_165`]
module"]
#[doc(alias = "PI_REG_165")]
pub type PiReg165 = crate::Reg<pi_reg_165::PiReg165Spec>;
#[doc = "DDR PHY Independent Register 165"]
pub mod pi_reg_165;
#[doc = "PI_REG_166 (rw) register accessor: DDR PHY Independent Register 166\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_166::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_166::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_166`]
module"]
#[doc(alias = "PI_REG_166")]
pub type PiReg166 = crate::Reg<pi_reg_166::PiReg166Spec>;
#[doc = "DDR PHY Independent Register 166"]
pub mod pi_reg_166;
#[doc = "PI_REG_167 (rw) register accessor: DDR PHY Independent Register 167\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_167::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_167::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_167`]
module"]
#[doc(alias = "PI_REG_167")]
pub type PiReg167 = crate::Reg<pi_reg_167::PiReg167Spec>;
#[doc = "DDR PHY Independent Register 167"]
pub mod pi_reg_167;
#[doc = "PI_REG_168 (rw) register accessor: DDR PHY Independent Register 168\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_168::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_168::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_168`]
module"]
#[doc(alias = "PI_REG_168")]
pub type PiReg168 = crate::Reg<pi_reg_168::PiReg168Spec>;
#[doc = "DDR PHY Independent Register 168"]
pub mod pi_reg_168;
#[doc = "PI_REG_169 (rw) register accessor: DDR PHY Independent Register 169\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_169::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_169::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_169`]
module"]
#[doc(alias = "PI_REG_169")]
pub type PiReg169 = crate::Reg<pi_reg_169::PiReg169Spec>;
#[doc = "DDR PHY Independent Register 169"]
pub mod pi_reg_169;
#[doc = "PI_REG_174 (r) register accessor: DDR PHY Independent Register 174\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_174::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_174`]
module"]
#[doc(alias = "PI_REG_174")]
pub type PiReg174 = crate::Reg<pi_reg_174::PiReg174Spec>;
#[doc = "DDR PHY Independent Register 174"]
pub mod pi_reg_174;
#[doc = "PI_REG_175 (w) register accessor: DDR PHY Independent Register 175\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_175::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_175`]
module"]
#[doc(alias = "PI_REG_175")]
pub type PiReg175 = crate::Reg<pi_reg_175::PiReg175Spec>;
#[doc = "DDR PHY Independent Register 175"]
pub mod pi_reg_175;
#[doc = "PI_REG_176 (rw) register accessor: DDR PHY Independent Register 176\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_176::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_176::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_176`]
module"]
#[doc(alias = "PI_REG_176")]
pub type PiReg176 = crate::Reg<pi_reg_176::PiReg176Spec>;
#[doc = "DDR PHY Independent Register 176"]
pub mod pi_reg_176;
#[doc = "PI_REG_186 (rw) register accessor: DDR PHY Independent Register 186\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_186::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_186::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_186`]
module"]
#[doc(alias = "PI_REG_186")]
pub type PiReg186 = crate::Reg<pi_reg_186::PiReg186Spec>;
#[doc = "DDR PHY Independent Register 186"]
pub mod pi_reg_186;
#[doc = "PI_REG_187 (rw) register accessor: DDR PHY Independent Register 187\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_187::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_187::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_187`]
module"]
#[doc(alias = "PI_REG_187")]
pub type PiReg187 = crate::Reg<pi_reg_187::PiReg187Spec>;
#[doc = "DDR PHY Independent Register 187"]
pub mod pi_reg_187;
#[doc = "PI_REG_188 (rw) register accessor: DDR PHY Independent Register 188\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_188::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_188::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_188`]
module"]
#[doc(alias = "PI_REG_188")]
pub type PiReg188 = crate::Reg<pi_reg_188::PiReg188Spec>;
#[doc = "DDR PHY Independent Register 188"]
pub mod pi_reg_188;
#[doc = "PI_REG_189 (rw) register accessor: DDR PHY Independent Register 189\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_189::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_189::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_189`]
module"]
#[doc(alias = "PI_REG_189")]
pub type PiReg189 = crate::Reg<pi_reg_189::PiReg189Spec>;
#[doc = "DDR PHY Independent Register 189"]
pub mod pi_reg_189;
#[doc = "PI_REG_190 (rw) register accessor: DDR PHY Independent Register 190\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_190::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_190::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_190`]
module"]
#[doc(alias = "PI_REG_190")]
pub type PiReg190 = crate::Reg<pi_reg_190::PiReg190Spec>;
#[doc = "DDR PHY Independent Register 190"]
pub mod pi_reg_190;
#[doc = "PI_REG_191 (rw) register accessor: DDR PHY Independent Register 191\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_191::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_191::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_191`]
module"]
#[doc(alias = "PI_REG_191")]
pub type PiReg191 = crate::Reg<pi_reg_191::PiReg191Spec>;
#[doc = "DDR PHY Independent Register 191"]
pub mod pi_reg_191;
#[doc = "PI_REG_192 (rw) register accessor: DDR PHY Independent Register 192\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_192::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_192::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_192`]
module"]
#[doc(alias = "PI_REG_192")]
pub type PiReg192 = crate::Reg<pi_reg_192::PiReg192Spec>;
#[doc = "DDR PHY Independent Register 192"]
pub mod pi_reg_192;
#[doc = "PI_REG_193 (r) register accessor: DDR PHY Independent Register 193\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_193::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_193`]
module"]
#[doc(alias = "PI_REG_193")]
pub type PiReg193 = crate::Reg<pi_reg_193::PiReg193Spec>;
#[doc = "DDR PHY Independent Register 193"]
pub mod pi_reg_193;
#[doc = "PI_REG_199 (rw) register accessor: DDR PHY Independent Register 199\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_199::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_199::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pi_reg_199`]
module"]
#[doc(alias = "PI_REG_199")]
pub type PiReg199 = crate::Reg<pi_reg_199::PiReg199Spec>;
#[doc = "DDR PHY Independent Register 199"]
pub mod pi_reg_199;
