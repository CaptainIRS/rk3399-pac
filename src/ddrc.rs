#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    denali_ctl_00: DenaliCtl00,
    denali_ctl_01: DenaliCtl01,
    denali_ctl_02: DenaliCtl02,
    denali_ctl_03: DenaliCtl03,
    denali_ctl_04: DenaliCtl04,
    denali_ctl_05: DenaliCtl05,
    denali_ctl_06: DenaliCtl06,
    denali_ctl_07: DenaliCtl07,
    denali_ctl_08: DenaliCtl08,
    denali_ctl_09: DenaliCtl09,
    denali_ctl_10: DenaliCtl10,
    denali_ctl_11: DenaliCtl11,
    denali_ctl_12: DenaliCtl12,
    denali_ctl_13: DenaliCtl13,
    denali_ctl_14: DenaliCtl14,
    denali_ctl_15: DenaliCtl15,
    denali_ctl_16: DenaliCtl16,
    denali_ctl_17: DenaliCtl17,
    denali_ctl_18: DenaliCtl18,
    denali_ctl_19: DenaliCtl19,
    denali_ctl_20: DenaliCtl20,
    denali_ctl_21: DenaliCtl21,
    denali_ctl_22: DenaliCtl22,
    denali_ctl_23: DenaliCtl23,
    denali_ctl_24: DenaliCtl24,
    denali_ctl_25: DenaliCtl25,
    denali_ctl_26: DenaliCtl26,
    denali_ctl_27: DenaliCtl27,
    denali_ctl_28: DenaliCtl28,
    denali_ctl_29: DenaliCtl29,
    denali_ctl_30: DenaliCtl30,
    denali_ctl_31: DenaliCtl31,
    denali_ctl_32: DenaliCtl32,
    denali_ctl_33: DenaliCtl33,
    denali_ctl_34: DenaliCtl34,
    denali_ctl_35: DenaliCtl35,
    denali_ctl_36: DenaliCtl36,
    denali_ctl_37: DenaliCtl37,
    denali_ctl_38: DenaliCtl38,
    denali_ctl_39: DenaliCtl39,
    denali_ctl_40: DenaliCtl40,
    denali_ctl_41: DenaliCtl41,
    denali_ctl_42: DenaliCtl42,
    denali_ctl_43: DenaliCtl43,
    denali_ctl_44: DenaliCtl44,
    denali_ctl_45: DenaliCtl45,
    denali_ctl_46: DenaliCtl46,
    denali_ctl_47: DenaliCtl47,
    denali_ctl_48: DenaliCtl48,
    denali_ctl_49: DenaliCtl49,
    denali_ctl_50: DenaliCtl50,
    denali_ctl_51: DenaliCtl51,
    denali_ctl_52: DenaliCtl52,
    denali_ctl_53: DenaliCtl53,
    denali_ctl_54: DenaliCtl54,
    denali_ctl_55: DenaliCtl55,
    denali_ctl_56: DenaliCtl56,
    denali_ctl_57: DenaliCtl57,
    denali_ctl_58: DenaliCtl58,
    denali_ctl_59: DenaliCtl59,
    denali_ctl_60: DenaliCtl60,
    denali_ctl_61: DenaliCtl61,
    denali_ctl_62: DenaliCtl62,
    denali_ctl_63: DenaliCtl63,
    denali_ctl_64: DenaliCtl64,
    denali_ctl_65: DenaliCtl65,
    denali_ctl_66: DenaliCtl66,
    denali_ctl_67: DenaliCtl67,
    denali_ctl_68: DenaliCtl68,
    denali_ctl_69: DenaliCtl69,
    denali_ctl_70: DenaliCtl70,
    denali_ctl_71: DenaliCtl71,
    denali_ctl_72: DenaliCtl72,
    denali_ctl_73: DenaliCtl73,
    _reserved74: [u8; 0x04],
    denali_ctl_75: DenaliCtl75,
    denali_ctl_76: DenaliCtl76,
    denali_ctl_77: DenaliCtl77,
    denali_ctl_78: DenaliCtl78,
    denali_ctl_79: DenaliCtl79,
    denali_ctl_80: DenaliCtl80,
    denali_ctl_81: DenaliCtl81,
    denali_ctl_82: DenaliCtl82,
    denali_ctl_83: DenaliCtl83,
    denali_ctl_84: DenaliCtl84,
    denali_ctl_85: DenaliCtl85,
    denali_ctl_86: DenaliCtl86,
    denali_ctl_87: DenaliCtl87,
    denali_ctl_88: DenaliCtl88,
    denali_ctl_89: DenaliCtl89,
    denali_ctl_90: DenaliCtl90,
    denali_ctl_91: DenaliCtl91,
    denali_ctl_92: DenaliCtl92,
    denali_ctl_93: DenaliCtl93,
    denali_ctl_94: DenaliCtl94,
    denali_ctl_95: DenaliCtl95,
    denali_ctl_96: DenaliCtl96,
    denali_ctl_97: DenaliCtl97,
    denali_ctl_98: DenaliCtl98,
    denali_ctl_99: DenaliCtl99,
    denali_ctl_100: DenaliCtl100,
    denali_ctl_101: DenaliCtl101,
    denali_ctl_102: DenaliCtl102,
    denali_ctl_103: DenaliCtl103,
    denali_ctl_104: DenaliCtl104,
    denali_ctl_105: DenaliCtl105,
    denali_ctl_106: DenaliCtl106,
    denali_ctl_107: DenaliCtl107,
    denali_ctl_108: DenaliCtl108,
    denali_ctl_109: DenaliCtl109,
    denali_ctl_110: DenaliCtl110,
    denali_ctl_111: DenaliCtl111,
    denali_ctl_112: DenaliCtl112,
    denali_ctl_113: DenaliCtl113,
    denali_ctl_114: DenaliCtl114,
    denali_ctl_115: DenaliCtl115,
    denali_ctl_116: DenaliCtl116,
    denali_ctl_117: DenaliCtl117,
    denali_ctl_118: DenaliCtl118,
    denali_ctl_119: DenaliCtl119,
    denali_ctl_120: DenaliCtl120,
    denali_ctl_121: DenaliCtl121,
    denali_ctl_122: DenaliCtl122,
    denali_ctl_123: DenaliCtl123,
    denali_ctl_124: DenaliCtl124,
    denali_ctl_125: DenaliCtl125,
    denali_ctl_126: DenaliCtl126,
    denali_ctl_127: DenaliCtl127,
    denali_ctl_128: DenaliCtl128,
    denali_ctl_129: DenaliCtl129,
    denali_ctl_130: DenaliCtl130,
    denali_ctl_131: DenaliCtl131,
    denali_ctl_132: DenaliCtl132,
    denali_ctl_133: DenaliCtl133,
    denali_ctl_134: DenaliCtl134,
    denali_ctl_135: DenaliCtl135,
    denali_ctl_136: DenaliCtl136,
    denali_ctl_137: DenaliCtl137,
    denali_ctl_138: DenaliCtl138,
    denali_ctl_139: DenaliCtl139,
    denali_ctl_140: DenaliCtl140,
    denali_ctl_141: DenaliCtl141,
    denali_ctl_142: DenaliCtl142,
    denali_ctl_143: DenaliCtl143,
    denali_ctl_144: DenaliCtl144,
    denali_ctl_145: DenaliCtl145,
    denali_ctl_146: DenaliCtl146,
    denali_ctl_147: DenaliCtl147,
    denali_ctl_148: DenaliCtl148,
    denali_ctl_149: DenaliCtl149,
    denali_ctl_150: DenaliCtl150,
    denali_ctl_151: DenaliCtl151,
    denali_ctl_152: DenaliCtl152,
    denali_ctl_153: DenaliCtl153,
    denali_ctl_154: DenaliCtl154,
    denali_ctl_155: DenaliCtl155,
    denali_ctl_156: DenaliCtl156,
    denali_ctl_157: DenaliCtl157,
    denali_ctl_158: DenaliCtl158,
    denali_ctl_159: DenaliCtl159,
    denali_ctl_160: DenaliCtl160,
    denali_ctl_161: DenaliCtl161,
    denali_ctl_162: DenaliCtl162,
    denali_ctl_163: DenaliCtl163,
    denali_ctl_164: DenaliCtl164,
    denali_ctl_165: DenaliCtl165,
    denali_ctl_166: DenaliCtl166,
    denali_ctl_167: DenaliCtl167,
    denali_ctl_168: DenaliCtl168,
    denali_ctl_169: DenaliCtl169,
    denali_ctl_170: DenaliCtl170,
    denali_ctl_171: DenaliCtl171,
    denali_ctl_172: DenaliCtl172,
    denali_ctl_173: DenaliCtl173,
    denali_ctl_174: DenaliCtl174,
    denali_ctl_175: DenaliCtl175,
    denali_ctl_176: DenaliCtl176,
    denali_ctl_177: DenaliCtl177,
    denali_ctl_178: DenaliCtl178,
    denali_ctl_179: DenaliCtl179,
    denali_ctl_180: DenaliCtl180,
    denali_ctl_181: DenaliCtl181,
    denali_ctl_182: DenaliCtl182,
    denali_ctl_183: DenaliCtl183,
    denali_ctl_184: DenaliCtl184,
    denali_ctl_185: DenaliCtl185,
    denali_ctl_186: DenaliCtl186,
    denali_ctl_187: DenaliCtl187,
    denali_ctl_188: DenaliCtl188,
    denali_ctl_189: DenaliCtl189,
    denali_ctl_190: DenaliCtl190,
    denali_ctl_191: DenaliCtl191,
    denali_ctl_192: DenaliCtl192,
    denali_ctl_193: DenaliCtl193,
    denali_ctl_194: DenaliCtl194,
    denali_ctl_195: DenaliCtl195,
    denali_ctl_196: DenaliCtl196,
    denali_ctl_197: DenaliCtl197,
    denali_ctl_198: DenaliCtl198,
    denali_ctl_199: DenaliCtl199,
    denali_ctl_200: DenaliCtl200,
    denali_ctl_201: DenaliCtl201,
    denali_ctl_202: DenaliCtl202,
    denali_ctl_203: DenaliCtl203,
    denali_ctl_204: DenaliCtl204,
    denali_ctl_205: DenaliCtl205,
    denali_ctl_206: DenaliCtl206,
    denali_ctl_207: DenaliCtl207,
    denali_ctl_208: DenaliCtl208,
    denali_ctl_209: DenaliCtl209,
    denali_ctl_210: DenaliCtl210,
    denali_ctl_211: DenaliCtl211,
    denali_ctl_212: DenaliCtl212,
    denali_ctl_213: DenaliCtl213,
    denali_ctl_214: DenaliCtl214,
    denali_ctl_215: DenaliCtl215,
    denali_ctl_216: DenaliCtl216,
    denali_ctl_217: DenaliCtl217,
    denali_ctl_218: DenaliCtl218,
    denali_ctl_219: DenaliCtl219,
    denali_ctl_220: DenaliCtl220,
    denali_ctl_221: DenaliCtl221,
    denali_ctl_222: DenaliCtl222,
    denali_ctl_223: DenaliCtl223,
    denali_ctl_224: DenaliCtl224,
    denali_ctl_225: DenaliCtl225,
    denali_ctl_226: DenaliCtl226,
    denali_ctl_227: DenaliCtl227,
    denali_ctl_228: DenaliCtl228,
    denali_ctl_229: DenaliCtl229,
    denali_ctl_230: DenaliCtl230,
    denali_ctl_231: DenaliCtl231,
    denali_ctl_232: DenaliCtl232,
    denali_ctl_233: DenaliCtl233,
    denali_ctl_234: DenaliCtl234,
    denali_ctl_235: DenaliCtl235,
    denali_ctl_236: DenaliCtl236,
    denali_ctl_237: DenaliCtl237,
    denali_ctl_238: DenaliCtl238,
    denali_ctl_239: DenaliCtl239,
    denali_ctl_240: DenaliCtl240,
    denali_ctl_241: DenaliCtl241,
    denali_ctl_242: DenaliCtl242,
    denali_ctl_243: DenaliCtl243,
    denali_ctl_244: DenaliCtl244,
    denali_ctl_245: DenaliCtl245,
    denali_ctl_246: DenaliCtl246,
    denali_ctl_247: DenaliCtl247,
    denali_ctl_248: DenaliCtl248,
    denali_ctl_249: DenaliCtl249,
    denali_ctl_250: DenaliCtl250,
    denali_ctl_251: DenaliCtl251,
    denali_ctl_252: DenaliCtl252,
    denali_ctl_253: DenaliCtl253,
    denali_ctl_254: DenaliCtl254,
    denali_ctl_255: DenaliCtl255,
    denali_ctl_256: DenaliCtl256,
    denali_ctl_257: DenaliCtl257,
    denali_ctl_258: DenaliCtl258,
    denali_ctl_259: DenaliCtl259,
    denali_ctl_260: DenaliCtl260,
    denali_ctl_261: DenaliCtl261,
    denali_ctl_262: DenaliCtl262,
    denali_ctl_263: DenaliCtl263,
    denali_ctl_264: DenaliCtl264,
    denali_ctl_265: DenaliCtl265,
    denali_ctl_266: DenaliCtl266,
    denali_ctl_267: DenaliCtl267,
    denali_ctl_268: DenaliCtl268,
    denali_ctl_269: DenaliCtl269,
    denali_ctl_270: DenaliCtl270,
    denali_ctl_271: DenaliCtl271,
    denali_ctl_272: DenaliCtl272,
    denali_ctl_273: DenaliCtl273,
    denali_ctl_274: DenaliCtl274,
    denali_ctl_275: DenaliCtl275,
    denali_ctl_276: DenaliCtl276,
    denali_ctl_277: DenaliCtl277,
    denali_ctl_278: DenaliCtl278,
    denali_ctl_279: DenaliCtl279,
    denali_ctl_280: DenaliCtl280,
    denali_ctl_281: DenaliCtl281,
    denali_ctl_282: DenaliCtl282,
    denali_ctl_283: DenaliCtl283,
    denali_ctl_284: DenaliCtl284,
    denali_ctl_285: DenaliCtl285,
    denali_ctl_286: DenaliCtl286,
    denali_ctl_287: DenaliCtl287,
    denali_ctl_288: DenaliCtl288,
    denali_ctl_289: DenaliCtl289,
    denali_ctl_290: DenaliCtl290,
    denali_ctl_291: DenaliCtl291,
    denali_ctl_292: DenaliCtl292,
    denali_ctl_293: DenaliCtl293,
    denali_ctl_294: DenaliCtl294,
    denali_ctl_295: DenaliCtl295,
    denali_ctl_296: DenaliCtl296,
    denali_ctl_297: DenaliCtl297,
    denali_ctl_298: DenaliCtl298,
    denali_ctl_299: DenaliCtl299,
    denali_ctl_300: DenaliCtl300,
    denali_ctl_301: DenaliCtl301,
    denali_ctl_302: DenaliCtl302,
    denali_ctl_303: DenaliCtl303,
    denali_ctl_304: DenaliCtl304,
    denali_ctl_305: DenaliCtl305,
    denali_ctl_306: DenaliCtl306,
    denali_ctl_307: DenaliCtl307,
    denali_ctl_308: DenaliCtl308,
    denali_ctl_309: DenaliCtl309,
    denali_ctl_310: DenaliCtl310,
    denali_ctl_311: DenaliCtl311,
    denali_ctl_312: DenaliCtl312,
    denali_ctl_313: DenaliCtl313,
    denali_ctl_314: DenaliCtl314,
    denali_ctl_315: DenaliCtl315,
    denali_ctl_316: DenaliCtl316,
    denali_ctl_317: DenaliCtl317,
    denali_ctl_318: DenaliCtl318,
    denali_ctl_319: DenaliCtl319,
    denali_ctl_320: DenaliCtl320,
    denali_ctl_321: DenaliCtl321,
    denali_ctl_322: DenaliCtl322,
    denali_ctl_323: DenaliCtl323,
    denali_ctl_324: DenaliCtl324,
    _reserved324: [u8; 0x02ec],
    ddr_pi_reg_0: DdrPiReg0,
    ddr_pi_reg_1: DdrPiReg1,
    ddr_pi_reg_2: DdrPiReg2,
    ddr_pi_reg_3: DdrPiReg3,
    ddr_pi_reg_4: DdrPiReg4,
    ddr_pi_reg_5: DdrPiReg5,
    ddr_pi_reg_6: DdrPiReg6,
    ddr_pi_reg_7: DdrPiReg7,
    ddr_pi_reg_8: DdrPiReg8,
    ddr_pi_reg_9: DdrPiReg9,
    ddr_pi_reg_10: DdrPiReg10,
    ddr_pi_reg_11: DdrPiReg11,
    ddr_pi_reg_12: DdrPiReg12,
    ddr_pi_reg_13: DdrPiReg13,
    ddr_pi_reg_14: DdrPiReg14,
    ddr_pi_reg_15: DdrPiReg15,
    ddr_pi_reg_16: DdrPiReg16,
    ddr_pi_reg_17: DdrPiReg17,
    ddr_pi_reg_18: DdrPiReg18,
    ddr_pi_reg_19: DdrPiReg19,
    ddr_pi_reg_20: DdrPiReg20,
    ddr_pi_reg_21: DdrPiReg21,
    ddr_pi_reg_22: DdrPiReg22,
    ddr_pi_reg_23: DdrPiReg23,
    ddr_pi_reg_24: DdrPiReg24,
    ddr_pi_reg_25: DdrPiReg25,
    ddr_pi_reg_26: DdrPiReg26,
    ddr_pi_reg_27: DdrPiReg27,
    ddr_pi_reg_28: DdrPiReg28,
    ddr_pi_reg_29: DdrPiReg29,
    ddr_pi_reg_30: DdrPiReg30,
    ddr_pi_reg_31: DdrPiReg31,
    ddr_pi_reg_32: DdrPiReg32,
    ddr_pi_reg_33: DdrPiReg33,
    ddr_pi_reg_34: DdrPiReg34,
    ddr_pi_reg_35: DdrPiReg35,
    ddr_pi_reg_36: DdrPiReg36,
    ddr_pi_reg_37: DdrPiReg37,
    ddr_pi_reg_38: DdrPiReg38,
    ddr_pi_reg_39: DdrPiReg39,
    ddr_pi_reg_40: DdrPiReg40,
    ddr_pi_reg_41: DdrPiReg41,
    ddr_pi_reg_42: DdrPiReg42,
    ddr_pi_reg_43: DdrPiReg43,
    ddr_pi_reg_44: DdrPiReg44,
    ddr_pi_reg_45: DdrPiReg45,
    ddr_pi_reg_46: DdrPiReg46,
    ddr_pi_reg_47: DdrPiReg47,
    ddr_pi_reg_48: DdrPiReg48,
    ddr_pi_reg_49: DdrPiReg49,
    ddr_pi_reg_50: DdrPiReg50,
    ddr_pi_reg_51: DdrPiReg51,
    ddr_pi_reg_52: DdrPiReg52,
    ddr_pi_reg_53: DdrPiReg53,
    ddr_pi_reg_54: DdrPiReg54,
    ddr_pi_reg_55: DdrPiReg55,
    ddr_pi_reg_56: DdrPiReg56,
    ddr_pi_reg_57: DdrPiReg57,
    ddr_pi_reg_58: DdrPiReg58,
    ddr_pi_reg_59: DdrPiReg59,
    ddr_pi_reg_60: DdrPiReg60,
    ddr_pi_reg_61: DdrPiReg61,
    ddr_pi_reg_62: DdrPiReg62,
    ddr_pi_reg_63: DdrPiReg63,
    ddr_pi_reg_64: DdrPiReg64,
    ddr_pi_reg_65: DdrPiReg65,
    ddr_pi_reg_66: DdrPiReg66,
    ddr_pi_reg_67: DdrPiReg67,
    ddr_pi_reg_68: DdrPiReg68,
    ddr_pi_reg_69: DdrPiReg69,
    ddr_pi_reg_70: DdrPiReg70,
    ddr_pi_reg_71: DdrPiReg71,
    ddr_pi_reg_72: DdrPiReg72,
    ddr_pi_reg_73: DdrPiReg73,
    ddr_pi_reg_74: DdrPiReg74,
    ddr_pi_reg_75: DdrPiReg75,
    ddr_pi_reg_76: DdrPiReg76,
    ddr_pi_reg_77: DdrPiReg77,
    ddr_pi_reg_78: DdrPiReg78,
    ddr_pi_reg_79: DdrPiReg79,
    ddr_pi_reg_80: DdrPiReg80,
    ddr_pi_reg_81: DdrPiReg81,
    ddr_pi_reg_82: DdrPiReg82,
    ddr_pi_reg_83: DdrPiReg83,
    ddr_pi_reg_84: DdrPiReg84,
    ddr_pi_reg_85: DdrPiReg85,
    ddr_pi_reg_86: DdrPiReg86,
    ddr_pi_reg_87: DdrPiReg87,
    ddr_pi_reg_88: DdrPiReg88,
    ddr_pi_reg_89: DdrPiReg89,
    ddr_pi_reg_90: DdrPiReg90,
    ddr_pi_reg_91: DdrPiReg91,
    ddr_pi_reg_92: DdrPiReg92,
    ddr_pi_reg_93: DdrPiReg93,
    ddr_pi_reg_94: DdrPiReg94,
    ddr_pi_reg_95: DdrPiReg95,
    ddr_pi_reg_96: DdrPiReg96,
    ddr_pi_reg_97: DdrPiReg97,
    ddr_pi_reg_98: DdrPiReg98,
    ddr_pi_reg_99: DdrPiReg99,
    ddr_pi_reg_100: DdrPiReg100,
    ddr_pi_reg_101: DdrPiReg101,
    ddr_pi_reg_102: DdrPiReg102,
    ddr_pi_reg_103: DdrPiReg103,
    ddr_pi_reg_104: DdrPiReg104,
    ddr_pi_reg_105: DdrPiReg105,
    ddr_pi_reg_106: DdrPiReg106,
    ddr_pi_reg_107: DdrPiReg107,
    ddr_pi_reg_108: DdrPiReg108,
    ddr_pi_reg_109: DdrPiReg109,
    ddr_pi_reg_110: DdrPiReg110,
    ddr_pi_reg_111: DdrPiReg111,
    ddr_pi_reg_112: DdrPiReg112,
    ddr_pi_reg_113: DdrPiReg113,
    ddr_pi_reg_114: DdrPiReg114,
    ddr_pi_reg_115: DdrPiReg115,
    ddr_pi_reg_116: DdrPiReg116,
    ddr_pi_reg_117: DdrPiReg117,
    ddr_pi_reg_118: DdrPiReg118,
    ddr_pi_reg_119: DdrPiReg119,
    ddr_pi_reg_120: DdrPiReg120,
    ddr_pi_reg_121: DdrPiReg121,
    ddr_pi_reg_122: DdrPiReg122,
    ddr_pi_reg_123: DdrPiReg123,
    ddr_pi_reg_124: DdrPiReg124,
    ddr_pi_reg_125: DdrPiReg125,
    ddr_pi_reg_126: DdrPiReg126,
    ddr_pi_reg_127: DdrPiReg127,
    ddr_pi_reg_128: DdrPiReg128,
    ddr_pi_reg_129: DdrPiReg129,
    ddr_pi_reg_130: DdrPiReg130,
    ddr_pi_reg_131: DdrPiReg131,
    ddr_pi_reg_132: DdrPiReg132,
    ddr_pi_reg_133: DdrPiReg133,
    ddr_pi_reg_134: DdrPiReg134,
    ddr_pi_reg_135: DdrPiReg135,
    ddr_pi_reg_136: DdrPiReg136,
    ddr_pi_reg_137: DdrPiReg137,
    ddr_pi_reg_138: DdrPiReg138,
    ddr_pi_reg_139: DdrPiReg139,
    ddr_pi_reg_140: DdrPiReg140,
    _reserved465: [u8; 0x38],
    ddr_pi_reg_155: DdrPiReg155,
    ddr_pi_reg_156: DdrPiReg156,
    ddr_pi_reg_157: DdrPiReg157,
    ddr_pi_reg_158: DdrPiReg158,
    ddr_pi_reg_159: DdrPiReg159,
    ddr_pi_reg_160: DdrPiReg160,
    ddr_pi_reg_161: DdrPiReg161,
    ddr_pi_reg_162: DdrPiReg162,
    ddr_pi_reg_163: DdrPiReg163,
    ddr_pi_reg_164: DdrPiReg164,
    ddr_pi_reg_165: DdrPiReg165,
    ddr_pi_reg_166: DdrPiReg166,
    ddr_pi_reg_167: DdrPiReg167,
    ddr_pi_reg_168: DdrPiReg168,
    ddr_pi_reg_169: DdrPiReg169,
    _reserved480: [u8; 0x10],
    ddr_pi_reg_174: DdrPiReg174,
    ddr_pi_reg_175: DdrPiReg175,
    ddr_pi_reg_176: DdrPiReg176,
    _reserved483: [u8; 0x24],
    ddr_pi_reg_186: DdrPiReg186,
    ddr_pi_reg_187: DdrPiReg187,
    ddr_pi_reg_188: DdrPiReg188,
    ddr_pi_reg_189: DdrPiReg189,
    ddr_pi_reg_190: DdrPiReg190,
    ddr_pi_reg_191: DdrPiReg191,
    ddr_pi_reg_192: DdrPiReg192,
    ddr_pi_reg_193: DdrPiReg193,
    _reserved491: [u8; 0x14],
    ddr_pi_reg_199: DdrPiReg199,
    _reserved492: [u8; 0x14e0],
    denali_phy_00: DenaliPhy00,
    denali_phy_01: DenaliPhy01,
    denali_phy_02: DenaliPhy02,
    denali_phy_03: DenaliPhy03,
    denali_phy_04: DenaliPhy04,
    denali_phy_05: DenaliPhy05,
    denali_phy_06: DenaliPhy06,
    denali_phy_07: DenaliPhy07,
    denali_phy_08: DenaliPhy08,
    denali_phy_09: DenaliPhy09,
    denali_phy_10: DenaliPhy10,
    denali_phy_11: DenaliPhy11,
    denali_phy_12: DenaliPhy12,
    denali_phy_13: DenaliPhy13,
    denali_phy_14: DenaliPhy14,
    denali_phy_15: DenaliPhy15,
    denali_phy_16: DenaliPhy16,
    denali_phy_17: DenaliPhy17,
    denali_phy_18: DenaliPhy18,
    denali_phy_19: DenaliPhy19,
    denali_phy_20: DenaliPhy20,
    denali_phy_21: DenaliPhy21,
    denali_phy_22: DenaliPhy22,
    denali_phy_23: DenaliPhy23,
    denali_phy_24: DenaliPhy24,
    denali_phy_25: DenaliPhy25,
    denali_phy_26: DenaliPhy26,
    denali_phy_27: DenaliPhy27,
    denali_phy_28: DenaliPhy28,
    denali_phy_29: DenaliPhy29,
    denali_phy_30: DenaliPhy30,
    denali_phy_31: DenaliPhy31,
    denali_phy_32: DenaliPhy32,
    denali_phy_33: DenaliPhy33,
    denali_phy_34: DenaliPhy34,
    denali_phy_35: DenaliPhy35,
    denali_phy_36: DenaliPhy36,
    denali_phy_37: DenaliPhy37,
    denali_phy_38: DenaliPhy38,
    denali_phy_39: DenaliPhy39,
    denali_phy_40: DenaliPhy40,
    denali_phy_41: DenaliPhy41,
    denali_phy_42: DenaliPhy42,
    denali_phy_43: DenaliPhy43,
    denali_phy_44: DenaliPhy44,
    denali_phy_45: DenaliPhy45,
    denali_phy_46: DenaliPhy46,
    denali_phy_47: DenaliPhy47,
    denali_phy_48: DenaliPhy48,
    denali_phy_49: DenaliPhy49,
    denali_phy_50: DenaliPhy50,
    denali_phy_51: DenaliPhy51,
    denali_phy_52: DenaliPhy52,
    denali_phy_53: DenaliPhy53,
    denali_phy_54: DenaliPhy54,
    denali_phy_55: DenaliPhy55,
    denali_phy_56: DenaliPhy56,
    denali_phy_57: DenaliPhy57,
    denali_phy_58: DenaliPhy58,
    denali_phy_59: DenaliPhy59,
    denali_phy_60: DenaliPhy60,
    denali_phy_61: DenaliPhy61,
    denali_phy_62: DenaliPhy62,
    denali_phy_63: DenaliPhy63,
    denali_phy_64: DenaliPhy64,
    denali_phy_65: DenaliPhy65,
    denali_phy_66: DenaliPhy66,
    denali_phy_67: DenaliPhy67,
    denali_phy_68: DenaliPhy68,
    denali_phy_69: DenaliPhy69,
    denali_phy_70: DenaliPhy70,
    denali_phy_71: DenaliPhy71,
    denali_phy_72: DenaliPhy72,
    denali_phy_73: DenaliPhy73,
    denali_phy_74: DenaliPhy74,
    denali_phy_75: DenaliPhy75,
    denali_phy_76: DenaliPhy76,
    denali_phy_77: DenaliPhy77,
    denali_phy_78: DenaliPhy78,
    denali_phy_79: DenaliPhy79,
    denali_phy_80: DenaliPhy80,
    denali_phy_81: DenaliPhy81,
    _reserved574: [u8; 0x04],
    denali_phy_83: DenaliPhy83,
    denali_phy_84: DenaliPhy84,
    denali_phy_85: DenaliPhy85,
    denali_phy_86: DenaliPhy86,
    denali_phy_87: DenaliPhy87,
    denali_phy_88: DenaliPhy88,
    denali_phy_89: DenaliPhy89,
    denali_phy_90: DenaliPhy90,
    _reserved582: [u8; 0x94],
    denali_phy_128: DenaliPhy128,
    denali_phy_129: DenaliPhy129,
    denali_phy_130: DenaliPhy130,
    denali_phy_131: DenaliPhy131,
    denali_phy_132: DenaliPhy132,
    denali_phy_133: DenaliPhy133,
    denali_phy_134: DenaliPhy134,
    denali_phy_135: DenaliPhy135,
    denali_phy_136: DenaliPhy136,
    denali_phy_137: DenaliPhy137,
    denali_phy_138: DenaliPhy138,
    denali_phy_139: DenaliPhy139,
    denali_phy_140: DenaliPhy140,
    denali_phy_141: DenaliPhy141,
    denali_phy_142: DenaliPhy142,
    denali_phy_143: DenaliPhy143,
    denali_phy_144: DenaliPhy144,
    denali_phy_145: DenaliPhy145,
    denali_phy_146: DenaliPhy146,
    denali_phy_147: DenaliPhy147,
    denali_phy_148: DenaliPhy148,
    denali_phy_149: DenaliPhy149,
    denali_phy_150: DenaliPhy150,
    denali_phy_151: DenaliPhy151,
    denali_phy_152: DenaliPhy152,
    denali_phy_153: DenaliPhy153,
    denali_phy_154: DenaliPhy154,
    denali_phy_155: DenaliPhy155,
    denali_phy_156: DenaliPhy156,
    denali_phy_157: DenaliPhy157,
    denali_phy_158: DenaliPhy158,
    denali_phy_159: DenaliPhy159,
    denali_phy_160: DenaliPhy160,
    denali_phy_161: DenaliPhy161,
    denali_phy_162: DenaliPhy162,
    denali_phy_163: DenaliPhy163,
    denali_phy_164: DenaliPhy164,
    denali_phy_165: DenaliPhy165,
    denali_phy_166: DenaliPhy166,
    denali_phy_167: DenaliPhy167,
    denali_phy_168: DenaliPhy168,
    denali_phy_169: DenaliPhy169,
    denali_phy_170: DenaliPhy170,
    denali_phy_171: DenaliPhy171,
    denali_phy_172: DenaliPhy172,
    denali_phy_173: DenaliPhy173,
    denali_phy_174: DenaliPhy174,
    denali_phy_175: DenaliPhy175,
    denali_phy_176: DenaliPhy176,
    denali_phy_177: DenaliPhy177,
    denali_phy_178: DenaliPhy178,
    denali_phy_179: DenaliPhy179,
    denali_phy_180: DenaliPhy180,
    denali_phy_181: DenaliPhy181,
    denali_phy_182: DenaliPhy182,
    denali_phy_183: DenaliPhy183,
    denali_phy_184: DenaliPhy184,
    denali_phy_185: DenaliPhy185,
    denali_phy_186: DenaliPhy186,
    denali_phy_187: DenaliPhy187,
    denali_phy_188: DenaliPhy188,
    denali_phy_189: DenaliPhy189,
    denali_phy_190: DenaliPhy190,
    denali_phy_191: DenaliPhy191,
    denali_phy_192: DenaliPhy192,
    denali_phy_193: DenaliPhy193,
    denali_phy_194: DenaliPhy194,
    denali_phy_195: DenaliPhy195,
    denali_phy_196: DenaliPhy196,
    denali_phy_197: DenaliPhy197,
    denali_phy_198: DenaliPhy198,
    denali_phy_199: DenaliPhy199,
    denali_phy_200: DenaliPhy200,
    denali_phy_201: DenaliPhy201,
    denali_phy_202: DenaliPhy202,
    denali_phy_203: DenaliPhy203,
    denali_phy_204: DenaliPhy204,
    denali_phy_205: DenaliPhy205,
    denali_phy_206: DenaliPhy206,
    denali_phy_207: DenaliPhy207,
    denali_phy_208: DenaliPhy208,
    denali_phy_209: DenaliPhy209,
    _reserved664: [u8; 0x04],
    denali_phy_211: DenaliPhy211,
    denali_phy_212: DenaliPhy212,
    denali_phy_213: DenaliPhy213,
    denali_phy_214: DenaliPhy214,
    denali_phy_215: DenaliPhy215,
    denali_phy_216: DenaliPhy216,
    denali_phy_217: DenaliPhy217,
    denali_phy_218: DenaliPhy218,
    _reserved672: [u8; 0x94],
    denali_phy_256: DenaliPhy256,
    denali_phy_257: DenaliPhy257,
    denali_phy_258: DenaliPhy258,
    denali_phy_259: DenaliPhy259,
    denali_phy_260: DenaliPhy260,
    denali_phy_261: DenaliPhy261,
    denali_phy_262: DenaliPhy262,
    denali_phy_263: DenaliPhy263,
    denali_phy_264: DenaliPhy264,
    denali_phy_265: DenaliPhy265,
    denali_phy_266: DenaliPhy266,
    denali_phy_267: DenaliPhy267,
    denali_phy_268: DenaliPhy268,
    denali_phy_269: DenaliPhy269,
    denali_phy_270: DenaliPhy270,
    denali_phy_271: DenaliPhy271,
    denali_phy_272: DenaliPhy272,
    denali_phy_273: DenaliPhy273,
    denali_phy_274: DenaliPhy274,
    denali_phy_275: DenaliPhy275,
    denali_phy_276: DenaliPhy276,
    denali_phy_277: DenaliPhy277,
    denali_phy_278: DenaliPhy278,
    denali_phy_279: DenaliPhy279,
    denali_phy_280: DenaliPhy280,
    denali_phy_281: DenaliPhy281,
    denali_phy_282: DenaliPhy282,
    denali_phy_283: DenaliPhy283,
    denali_phy_284: DenaliPhy284,
    denali_phy_285: DenaliPhy285,
    denali_phy_286: DenaliPhy286,
    denali_phy_287: DenaliPhy287,
    denali_phy_288: DenaliPhy288,
    denali_phy_289: DenaliPhy289,
    denali_phy_290: DenaliPhy290,
    denali_phy_291: DenaliPhy291,
    denali_phy_292: DenaliPhy292,
    denali_phy_293: DenaliPhy293,
    denali_phy_294: DenaliPhy294,
    denali_phy_295: DenaliPhy295,
    denali_phy_296: DenaliPhy296,
    denali_phy_297: DenaliPhy297,
    denali_phy_298: DenaliPhy298,
    denali_phy_299: DenaliPhy299,
    denali_phy_300: DenaliPhy300,
    denali_phy_301: DenaliPhy301,
    denali_phy_302: DenaliPhy302,
    denali_phy_303: DenaliPhy303,
    denali_phy_304: DenaliPhy304,
    denali_phy_305: DenaliPhy305,
    denali_phy_306: DenaliPhy306,
    denali_phy_307: DenaliPhy307,
    denali_phy_308: DenaliPhy308,
    denali_phy_309: DenaliPhy309,
    denali_phy_310: DenaliPhy310,
    denali_phy_311: DenaliPhy311,
    denali_phy_312: DenaliPhy312,
    denali_phy_313: DenaliPhy313,
    denali_phy_314: DenaliPhy314,
    denali_phy_315: DenaliPhy315,
    denali_phy_316: DenaliPhy316,
    denali_phy_317: DenaliPhy317,
    denali_phy_318: DenaliPhy318,
    denali_phy_319: DenaliPhy319,
    denali_phy_320: DenaliPhy320,
    denali_phy_321: DenaliPhy321,
    denali_phy_322: DenaliPhy322,
    denali_phy_323: DenaliPhy323,
    denali_phy_324: DenaliPhy324,
    denali_phy_325: DenaliPhy325,
    denali_phy_326: DenaliPhy326,
    denali_phy_327: DenaliPhy327,
    denali_phy_328: DenaliPhy328,
    denali_phy_329: DenaliPhy329,
    denali_phy_330: DenaliPhy330,
    denali_phy_331: DenaliPhy331,
    denali_phy_332: DenaliPhy332,
    denali_phy_333: DenaliPhy333,
    denali_phy_334: DenaliPhy334,
    denali_phy_335: DenaliPhy335,
    denali_phy_336: DenaliPhy336,
    denali_phy_337: DenaliPhy337,
    _reserved754: [u8; 0x04],
    denali_phy_339: DenaliPhy339,
    denali_phy_340: DenaliPhy340,
    denali_phy_341: DenaliPhy341,
    denali_phy_342: DenaliPhy342,
    denali_phy_343: DenaliPhy343,
    denali_phy_344: DenaliPhy344,
    denali_phy_345: DenaliPhy345,
    denali_phy_346: DenaliPhy346,
    _reserved762: [u8; 0x94],
    denali_phy_384: DenaliPhy384,
    denali_phy_385: DenaliPhy385,
    denali_phy_386: DenaliPhy386,
    denali_phy_387: DenaliPhy387,
    denali_phy_388: DenaliPhy388,
    denali_phy_389: DenaliPhy389,
    denali_phy_390: DenaliPhy390,
    denali_phy_391: DenaliPhy391,
    denali_phy_392: DenaliPhy392,
    denali_phy_393: DenaliPhy393,
    denali_phy_394: DenaliPhy394,
    denali_phy_395: DenaliPhy395,
    denali_phy_396: DenaliPhy396,
    denali_phy_397: DenaliPhy397,
    denali_phy_398: DenaliPhy398,
    denali_phy_399: DenaliPhy399,
    denali_phy_400: DenaliPhy400,
    denali_phy_401: DenaliPhy401,
    denali_phy_402: DenaliPhy402,
    denali_phy_403: DenaliPhy403,
    denali_phy_404: DenaliPhy404,
    denali_phy_405: DenaliPhy405,
    denali_phy_406: DenaliPhy406,
    denali_phy_407: DenaliPhy407,
    denali_phy_408: DenaliPhy408,
    denali_phy_409: DenaliPhy409,
    denali_phy_410: DenaliPhy410,
    denali_phy_411: DenaliPhy411,
    denali_phy_412: DenaliPhy412,
    denali_phy_413: DenaliPhy413,
    denali_phy_414: DenaliPhy414,
    denali_phy_415: DenaliPhy415,
    denali_phy_416: DenaliPhy416,
    denali_phy_417: DenaliPhy417,
    denali_phy_418: DenaliPhy418,
    denali_phy_419: DenaliPhy419,
    denali_phy_420: DenaliPhy420,
    denali_phy_421: DenaliPhy421,
    denali_phy_422: DenaliPhy422,
    denali_phy_423: DenaliPhy423,
    denali_phy_424: DenaliPhy424,
    denali_phy_425: DenaliPhy425,
    denali_phy_426: DenaliPhy426,
    denali_phy_427: DenaliPhy427,
    denali_phy_428: DenaliPhy428,
    denali_phy_429: DenaliPhy429,
    denali_phy_430: DenaliPhy430,
    denali_phy_431: DenaliPhy431,
    denali_phy_432: DenaliPhy432,
    denali_phy_433: DenaliPhy433,
    denali_phy_434: DenaliPhy434,
    denali_phy_435: DenaliPhy435,
    denali_phy_436: DenaliPhy436,
    denali_phy_437: DenaliPhy437,
    denali_phy_438: DenaliPhy438,
    denali_phy_439: DenaliPhy439,
    denali_phy_440: DenaliPhy440,
    denali_phy_441: DenaliPhy441,
    denali_phy_442: DenaliPhy442,
    denali_phy_443: DenaliPhy443,
    denali_phy_444: DenaliPhy444,
    denali_phy_445: DenaliPhy445,
    denali_phy_446: DenaliPhy446,
    denali_phy_447: DenaliPhy447,
    denali_phy_448: DenaliPhy448,
    denali_phy_449: DenaliPhy449,
    denali_phy_450: DenaliPhy450,
    denali_phy_451: DenaliPhy451,
    denali_phy_452: DenaliPhy452,
    denali_phy_453: DenaliPhy453,
    denali_phy_454: DenaliPhy454,
    denali_phy_455: DenaliPhy455,
    denali_phy_456: DenaliPhy456,
    denali_phy_457: DenaliPhy457,
    denali_phy_458: DenaliPhy458,
    denali_phy_459: DenaliPhy459,
    denali_phy_460: DenaliPhy460,
    denali_phy_461: DenaliPhy461,
    denali_phy_462: DenaliPhy462,
    denali_phy_463: DenaliPhy463,
    denali_phy_464: DenaliPhy464,
    denali_phy_465: DenaliPhy465,
    _reserved844: [u8; 0x04],
    denali_phy_467: DenaliPhy467,
    denali_phy_468: DenaliPhy468,
    denali_phy_469: DenaliPhy469,
    denali_phy_470: DenaliPhy470,
    denali_phy_471: DenaliPhy471,
    denali_phy_472: DenaliPhy472,
    denali_phy_473: DenaliPhy473,
    denali_phy_474: DenaliPhy474,
    _reserved852: [u8; 0x94],
    denali_phy_512: DenaliPhy512,
    denali_phy_513: DenaliPhy513,
    denali_phy_514: DenaliPhy514,
    denali_phy_515: DenaliPhy515,
    denali_phy_516: DenaliPhy516,
    denali_phy_517: DenaliPhy517,
    denali_phy_518: DenaliPhy518,
    denali_phy_519: DenaliPhy519,
    denali_phy_520: DenaliPhy520,
    denali_phy_521: DenaliPhy521,
    denali_phy_522: DenaliPhy522,
    denali_phy_523: DenaliPhy523,
    denali_phy_524: DenaliPhy524,
    denali_phy_525: DenaliPhy525,
    denali_phy_526: DenaliPhy526,
    denali_phy_527: DenaliPhy527,
    denali_phy_528: DenaliPhy528,
    denali_phy_529: DenaliPhy529,
    denali_phy_530: DenaliPhy530,
    denali_phy_531: DenaliPhy531,
    denali_phy_532: DenaliPhy532,
    denali_phy_533: DenaliPhy533,
    denali_phy_534: DenaliPhy534,
    denali_phy_535: DenaliPhy535,
    denali_phy_536: DenaliPhy536,
    denali_phy_537: DenaliPhy537,
    denali_phy_538: DenaliPhy538,
    denali_phy_539: DenaliPhy539,
    denali_phy_540: DenaliPhy540,
    denali_phy_541: DenaliPhy541,
    denali_phy_542: DenaliPhy542,
    denali_phy_543: DenaliPhy543,
    denali_phy_544: DenaliPhy544,
    denali_phy_545: DenaliPhy545,
    denali_phy_546: DenaliPhy546,
    denali_phy_547: DenaliPhy547,
    denali_phy_548: DenaliPhy548,
    denali_phy_549: DenaliPhy549,
    _reserved890: [u8; 0x0168],
    denali_phy_640: DenaliPhy640,
    denali_phy_641: DenaliPhy641,
    denali_phy_642: DenaliPhy642,
    denali_phy_643: DenaliPhy643,
    denali_phy_644: DenaliPhy644,
    denali_phy_645: DenaliPhy645,
    denali_phy_646: DenaliPhy646,
    denali_phy_647: DenaliPhy647,
    denali_phy_648: DenaliPhy648,
    denali_phy_649: DenaliPhy649,
    denali_phy_650: DenaliPhy650,
    denali_phy_651: DenaliPhy651,
    denali_phy_652: DenaliPhy652,
    denali_phy_653: DenaliPhy653,
    denali_phy_654: DenaliPhy654,
    denali_phy_655: DenaliPhy655,
    denali_phy_656: DenaliPhy656,
    denali_phy_657: DenaliPhy657,
    denali_phy_658: DenaliPhy658,
    denali_phy_659: DenaliPhy659,
    denali_phy_660: DenaliPhy660,
    denali_phy_661: DenaliPhy661,
    denali_phy_662: DenaliPhy662,
    denali_phy_663: DenaliPhy663,
    denali_phy_664: DenaliPhy664,
    denali_phy_665: DenaliPhy665,
    denali_phy_666: DenaliPhy666,
    denali_phy_667: DenaliPhy667,
    denali_phy_668: DenaliPhy668,
    denali_phy_669: DenaliPhy669,
    denali_phy_670: DenaliPhy670,
    denali_phy_671: DenaliPhy671,
    denali_phy_672: DenaliPhy672,
    denali_phy_673: DenaliPhy673,
    denali_phy_674: DenaliPhy674,
    denali_phy_675: DenaliPhy675,
    denali_phy_676: DenaliPhy676,
    denali_phy_677: DenaliPhy677,
    _reserved928: [u8; 0x0168],
    denali_phy_768: DenaliPhy768,
    denali_phy_769: DenaliPhy769,
    denali_phy_770: DenaliPhy770,
    denali_phy_771: DenaliPhy771,
    denali_phy_772: DenaliPhy772,
    denali_phy_773: DenaliPhy773,
    denali_phy_774: DenaliPhy774,
    denali_phy_775: DenaliPhy775,
    denali_phy_776: DenaliPhy776,
    denali_phy_777: DenaliPhy777,
    denali_phy_778: DenaliPhy778,
    denali_phy_779: DenaliPhy779,
    denali_phy_780: DenaliPhy780,
    denali_phy_781: DenaliPhy781,
    denali_phy_782: DenaliPhy782,
    denali_phy_783: DenaliPhy783,
    denali_phy_784: DenaliPhy784,
    denali_phy_785: DenaliPhy785,
    denali_phy_786: DenaliPhy786,
    denali_phy_787: DenaliPhy787,
    denali_phy_788: DenaliPhy788,
    denali_phy_789: DenaliPhy789,
    denali_phy_790: DenaliPhy790,
    denali_phy_791: DenaliPhy791,
    denali_phy_792: DenaliPhy792,
    denali_phy_793: DenaliPhy793,
    denali_phy_794: DenaliPhy794,
    denali_phy_795: DenaliPhy795,
    denali_phy_796: DenaliPhy796,
    denali_phy_797: DenaliPhy797,
    denali_phy_798: DenaliPhy798,
    denali_phy_799: DenaliPhy799,
    denali_phy_800: DenaliPhy800,
    denali_phy_801: DenaliPhy801,
    denali_phy_802: DenaliPhy802,
    denali_phy_803: DenaliPhy803,
    denali_phy_804: DenaliPhy804,
    denali_phy_805: DenaliPhy805,
    _reserved966: [u8; 0x0168],
    denali_phy_896: DenaliPhy896,
    denali_phy_897: DenaliPhy897,
    denali_phy_898: DenaliPhy898,
    denali_phy_899: DenaliPhy899,
    denali_phy_900: DenaliPhy900,
    denali_phy_901: DenaliPhy901,
    denali_phy_902: DenaliPhy902,
    denali_phy_903: DenaliPhy903,
    denali_phy_904: DenaliPhy904,
    denali_phy_905: DenaliPhy905,
    denali_phy_906: DenaliPhy906,
    denali_phy_907: DenaliPhy907,
    denali_phy_908: DenaliPhy908,
    _reserved979: [u8; 0x04],
    denali_phy_910: DenaliPhy910,
    denali_phy_911: DenaliPhy911,
    denali_phy_912: DenaliPhy912,
    denali_phy_913: DenaliPhy913,
    denali_phy_914: DenaliPhy914,
    denali_phy_915: DenaliPhy915,
    denali_phy_916: DenaliPhy916,
    denali_phy_917: DenaliPhy917,
    denali_phy_918: DenaliPhy918,
    denali_phy_919: DenaliPhy919,
    denali_phy_920: DenaliPhy920,
    denali_phy_921: DenaliPhy921,
    denali_phy_922: DenaliPhy922,
    denali_phy_923: DenaliPhy923,
    denali_phy_924: DenaliPhy924,
    denali_phy_925: DenaliPhy925,
    denali_phy_926: DenaliPhy926,
    denali_phy_927: DenaliPhy927,
    denali_phy_928: DenaliPhy928,
    denali_phy_929: DenaliPhy929,
    denali_phy_930: DenaliPhy930,
    denali_phy_931: DenaliPhy931,
    denali_phy_932: DenaliPhy932,
    denali_phy_933: DenaliPhy933,
    denali_phy_934: DenaliPhy934,
    denali_phy_935: DenaliPhy935,
    denali_phy_936: DenaliPhy936,
    denali_phy_937: DenaliPhy937,
    denali_phy_938: DenaliPhy938,
    denali_phy_939: DenaliPhy939,
    denali_phy_940: DenaliPhy940,
    denali_phy_941: DenaliPhy941,
    denali_phy_942: DenaliPhy942,
    denali_phy_943: DenaliPhy943,
    denali_phy_944: DenaliPhy944,
    denali_phy_945: DenaliPhy945,
    denali_phy_946: DenaliPhy946,
    denali_phy_947: DenaliPhy947,
    denali_phy_948: DenaliPhy948,
    denali_phy_949: DenaliPhy949,
    denali_phy_950: DenaliPhy950,
    denali_phy_951: DenaliPhy951,
    denali_phy_952: DenaliPhy952,
    denali_phy_953: DenaliPhy953,
    denali_phy_954: DenaliPhy954,
    denali_phy_955: DenaliPhy955,
    denali_phy_956: DenaliPhy956,
    denali_phy_957: DenaliPhy957,
    denali_phy_958: DenaliPhy958,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn denali_ctl_00(&self) -> &DenaliCtl00 {
        &self.denali_ctl_00
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn denali_ctl_01(&self) -> &DenaliCtl01 {
        &self.denali_ctl_01
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn denali_ctl_02(&self) -> &DenaliCtl02 {
        &self.denali_ctl_02
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn denali_ctl_03(&self) -> &DenaliCtl03 {
        &self.denali_ctl_03
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn denali_ctl_04(&self) -> &DenaliCtl04 {
        &self.denali_ctl_04
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn denali_ctl_05(&self) -> &DenaliCtl05 {
        &self.denali_ctl_05
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn denali_ctl_06(&self) -> &DenaliCtl06 {
        &self.denali_ctl_06
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn denali_ctl_07(&self) -> &DenaliCtl07 {
        &self.denali_ctl_07
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn denali_ctl_08(&self) -> &DenaliCtl08 {
        &self.denali_ctl_08
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn denali_ctl_09(&self) -> &DenaliCtl09 {
        &self.denali_ctl_09
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn denali_ctl_10(&self) -> &DenaliCtl10 {
        &self.denali_ctl_10
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn denali_ctl_11(&self) -> &DenaliCtl11 {
        &self.denali_ctl_11
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn denali_ctl_12(&self) -> &DenaliCtl12 {
        &self.denali_ctl_12
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn denali_ctl_13(&self) -> &DenaliCtl13 {
        &self.denali_ctl_13
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn denali_ctl_14(&self) -> &DenaliCtl14 {
        &self.denali_ctl_14
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn denali_ctl_15(&self) -> &DenaliCtl15 {
        &self.denali_ctl_15
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn denali_ctl_16(&self) -> &DenaliCtl16 {
        &self.denali_ctl_16
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn denali_ctl_17(&self) -> &DenaliCtl17 {
        &self.denali_ctl_17
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn denali_ctl_18(&self) -> &DenaliCtl18 {
        &self.denali_ctl_18
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn denali_ctl_19(&self) -> &DenaliCtl19 {
        &self.denali_ctl_19
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn denali_ctl_20(&self) -> &DenaliCtl20 {
        &self.denali_ctl_20
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn denali_ctl_21(&self) -> &DenaliCtl21 {
        &self.denali_ctl_21
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn denali_ctl_22(&self) -> &DenaliCtl22 {
        &self.denali_ctl_22
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn denali_ctl_23(&self) -> &DenaliCtl23 {
        &self.denali_ctl_23
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn denali_ctl_24(&self) -> &DenaliCtl24 {
        &self.denali_ctl_24
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn denali_ctl_25(&self) -> &DenaliCtl25 {
        &self.denali_ctl_25
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn denali_ctl_26(&self) -> &DenaliCtl26 {
        &self.denali_ctl_26
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn denali_ctl_27(&self) -> &DenaliCtl27 {
        &self.denali_ctl_27
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn denali_ctl_28(&self) -> &DenaliCtl28 {
        &self.denali_ctl_28
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn denali_ctl_29(&self) -> &DenaliCtl29 {
        &self.denali_ctl_29
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn denali_ctl_30(&self) -> &DenaliCtl30 {
        &self.denali_ctl_30
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn denali_ctl_31(&self) -> &DenaliCtl31 {
        &self.denali_ctl_31
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn denali_ctl_32(&self) -> &DenaliCtl32 {
        &self.denali_ctl_32
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn denali_ctl_33(&self) -> &DenaliCtl33 {
        &self.denali_ctl_33
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn denali_ctl_34(&self) -> &DenaliCtl34 {
        &self.denali_ctl_34
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn denali_ctl_35(&self) -> &DenaliCtl35 {
        &self.denali_ctl_35
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn denali_ctl_36(&self) -> &DenaliCtl36 {
        &self.denali_ctl_36
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn denali_ctl_37(&self) -> &DenaliCtl37 {
        &self.denali_ctl_37
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn denali_ctl_38(&self) -> &DenaliCtl38 {
        &self.denali_ctl_38
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn denali_ctl_39(&self) -> &DenaliCtl39 {
        &self.denali_ctl_39
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn denali_ctl_40(&self) -> &DenaliCtl40 {
        &self.denali_ctl_40
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn denali_ctl_41(&self) -> &DenaliCtl41 {
        &self.denali_ctl_41
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn denali_ctl_42(&self) -> &DenaliCtl42 {
        &self.denali_ctl_42
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn denali_ctl_43(&self) -> &DenaliCtl43 {
        &self.denali_ctl_43
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn denali_ctl_44(&self) -> &DenaliCtl44 {
        &self.denali_ctl_44
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn denali_ctl_45(&self) -> &DenaliCtl45 {
        &self.denali_ctl_45
    }
    #[doc = "0xb8 - "]
    #[inline(always)]
    pub const fn denali_ctl_46(&self) -> &DenaliCtl46 {
        &self.denali_ctl_46
    }
    #[doc = "0xbc - "]
    #[inline(always)]
    pub const fn denali_ctl_47(&self) -> &DenaliCtl47 {
        &self.denali_ctl_47
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn denali_ctl_48(&self) -> &DenaliCtl48 {
        &self.denali_ctl_48
    }
    #[doc = "0xc4 - "]
    #[inline(always)]
    pub const fn denali_ctl_49(&self) -> &DenaliCtl49 {
        &self.denali_ctl_49
    }
    #[doc = "0xc8 - "]
    #[inline(always)]
    pub const fn denali_ctl_50(&self) -> &DenaliCtl50 {
        &self.denali_ctl_50
    }
    #[doc = "0xcc - "]
    #[inline(always)]
    pub const fn denali_ctl_51(&self) -> &DenaliCtl51 {
        &self.denali_ctl_51
    }
    #[doc = "0xd0 - "]
    #[inline(always)]
    pub const fn denali_ctl_52(&self) -> &DenaliCtl52 {
        &self.denali_ctl_52
    }
    #[doc = "0xd4 - "]
    #[inline(always)]
    pub const fn denali_ctl_53(&self) -> &DenaliCtl53 {
        &self.denali_ctl_53
    }
    #[doc = "0xd8 - "]
    #[inline(always)]
    pub const fn denali_ctl_54(&self) -> &DenaliCtl54 {
        &self.denali_ctl_54
    }
    #[doc = "0xdc - "]
    #[inline(always)]
    pub const fn denali_ctl_55(&self) -> &DenaliCtl55 {
        &self.denali_ctl_55
    }
    #[doc = "0xe0 - "]
    #[inline(always)]
    pub const fn denali_ctl_56(&self) -> &DenaliCtl56 {
        &self.denali_ctl_56
    }
    #[doc = "0xe4 - "]
    #[inline(always)]
    pub const fn denali_ctl_57(&self) -> &DenaliCtl57 {
        &self.denali_ctl_57
    }
    #[doc = "0xe8 - "]
    #[inline(always)]
    pub const fn denali_ctl_58(&self) -> &DenaliCtl58 {
        &self.denali_ctl_58
    }
    #[doc = "0xec - "]
    #[inline(always)]
    pub const fn denali_ctl_59(&self) -> &DenaliCtl59 {
        &self.denali_ctl_59
    }
    #[doc = "0xf0 - "]
    #[inline(always)]
    pub const fn denali_ctl_60(&self) -> &DenaliCtl60 {
        &self.denali_ctl_60
    }
    #[doc = "0xf4 - "]
    #[inline(always)]
    pub const fn denali_ctl_61(&self) -> &DenaliCtl61 {
        &self.denali_ctl_61
    }
    #[doc = "0xf8 - "]
    #[inline(always)]
    pub const fn denali_ctl_62(&self) -> &DenaliCtl62 {
        &self.denali_ctl_62
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn denali_ctl_63(&self) -> &DenaliCtl63 {
        &self.denali_ctl_63
    }
    #[doc = "0x100 - "]
    #[inline(always)]
    pub const fn denali_ctl_64(&self) -> &DenaliCtl64 {
        &self.denali_ctl_64
    }
    #[doc = "0x104 - "]
    #[inline(always)]
    pub const fn denali_ctl_65(&self) -> &DenaliCtl65 {
        &self.denali_ctl_65
    }
    #[doc = "0x108 - "]
    #[inline(always)]
    pub const fn denali_ctl_66(&self) -> &DenaliCtl66 {
        &self.denali_ctl_66
    }
    #[doc = "0x10c - "]
    #[inline(always)]
    pub const fn denali_ctl_67(&self) -> &DenaliCtl67 {
        &self.denali_ctl_67
    }
    #[doc = "0x110 - "]
    #[inline(always)]
    pub const fn denali_ctl_68(&self) -> &DenaliCtl68 {
        &self.denali_ctl_68
    }
    #[doc = "0x114 - "]
    #[inline(always)]
    pub const fn denali_ctl_69(&self) -> &DenaliCtl69 {
        &self.denali_ctl_69
    }
    #[doc = "0x118 - "]
    #[inline(always)]
    pub const fn denali_ctl_70(&self) -> &DenaliCtl70 {
        &self.denali_ctl_70
    }
    #[doc = "0x11c - "]
    #[inline(always)]
    pub const fn denali_ctl_71(&self) -> &DenaliCtl71 {
        &self.denali_ctl_71
    }
    #[doc = "0x120 - "]
    #[inline(always)]
    pub const fn denali_ctl_72(&self) -> &DenaliCtl72 {
        &self.denali_ctl_72
    }
    #[doc = "0x124 - "]
    #[inline(always)]
    pub const fn denali_ctl_73(&self) -> &DenaliCtl73 {
        &self.denali_ctl_73
    }
    #[doc = "0x12c - "]
    #[inline(always)]
    pub const fn denali_ctl_75(&self) -> &DenaliCtl75 {
        &self.denali_ctl_75
    }
    #[doc = "0x130 - "]
    #[inline(always)]
    pub const fn denali_ctl_76(&self) -> &DenaliCtl76 {
        &self.denali_ctl_76
    }
    #[doc = "0x134 - "]
    #[inline(always)]
    pub const fn denali_ctl_77(&self) -> &DenaliCtl77 {
        &self.denali_ctl_77
    }
    #[doc = "0x138 - "]
    #[inline(always)]
    pub const fn denali_ctl_78(&self) -> &DenaliCtl78 {
        &self.denali_ctl_78
    }
    #[doc = "0x13c - "]
    #[inline(always)]
    pub const fn denali_ctl_79(&self) -> &DenaliCtl79 {
        &self.denali_ctl_79
    }
    #[doc = "0x140 - "]
    #[inline(always)]
    pub const fn denali_ctl_80(&self) -> &DenaliCtl80 {
        &self.denali_ctl_80
    }
    #[doc = "0x144 - "]
    #[inline(always)]
    pub const fn denali_ctl_81(&self) -> &DenaliCtl81 {
        &self.denali_ctl_81
    }
    #[doc = "0x148 - "]
    #[inline(always)]
    pub const fn denali_ctl_82(&self) -> &DenaliCtl82 {
        &self.denali_ctl_82
    }
    #[doc = "0x14c - "]
    #[inline(always)]
    pub const fn denali_ctl_83(&self) -> &DenaliCtl83 {
        &self.denali_ctl_83
    }
    #[doc = "0x150 - "]
    #[inline(always)]
    pub const fn denali_ctl_84(&self) -> &DenaliCtl84 {
        &self.denali_ctl_84
    }
    #[doc = "0x154 - "]
    #[inline(always)]
    pub const fn denali_ctl_85(&self) -> &DenaliCtl85 {
        &self.denali_ctl_85
    }
    #[doc = "0x158 - "]
    #[inline(always)]
    pub const fn denali_ctl_86(&self) -> &DenaliCtl86 {
        &self.denali_ctl_86
    }
    #[doc = "0x15c - "]
    #[inline(always)]
    pub const fn denali_ctl_87(&self) -> &DenaliCtl87 {
        &self.denali_ctl_87
    }
    #[doc = "0x160 - "]
    #[inline(always)]
    pub const fn denali_ctl_88(&self) -> &DenaliCtl88 {
        &self.denali_ctl_88
    }
    #[doc = "0x164 - "]
    #[inline(always)]
    pub const fn denali_ctl_89(&self) -> &DenaliCtl89 {
        &self.denali_ctl_89
    }
    #[doc = "0x168 - "]
    #[inline(always)]
    pub const fn denali_ctl_90(&self) -> &DenaliCtl90 {
        &self.denali_ctl_90
    }
    #[doc = "0x16c - "]
    #[inline(always)]
    pub const fn denali_ctl_91(&self) -> &DenaliCtl91 {
        &self.denali_ctl_91
    }
    #[doc = "0x170 - "]
    #[inline(always)]
    pub const fn denali_ctl_92(&self) -> &DenaliCtl92 {
        &self.denali_ctl_92
    }
    #[doc = "0x174 - "]
    #[inline(always)]
    pub const fn denali_ctl_93(&self) -> &DenaliCtl93 {
        &self.denali_ctl_93
    }
    #[doc = "0x178 - "]
    #[inline(always)]
    pub const fn denali_ctl_94(&self) -> &DenaliCtl94 {
        &self.denali_ctl_94
    }
    #[doc = "0x17c - "]
    #[inline(always)]
    pub const fn denali_ctl_95(&self) -> &DenaliCtl95 {
        &self.denali_ctl_95
    }
    #[doc = "0x180 - "]
    #[inline(always)]
    pub const fn denali_ctl_96(&self) -> &DenaliCtl96 {
        &self.denali_ctl_96
    }
    #[doc = "0x184 - "]
    #[inline(always)]
    pub const fn denali_ctl_97(&self) -> &DenaliCtl97 {
        &self.denali_ctl_97
    }
    #[doc = "0x188 - "]
    #[inline(always)]
    pub const fn denali_ctl_98(&self) -> &DenaliCtl98 {
        &self.denali_ctl_98
    }
    #[doc = "0x18c - "]
    #[inline(always)]
    pub const fn denali_ctl_99(&self) -> &DenaliCtl99 {
        &self.denali_ctl_99
    }
    #[doc = "0x190 - "]
    #[inline(always)]
    pub const fn denali_ctl_100(&self) -> &DenaliCtl100 {
        &self.denali_ctl_100
    }
    #[doc = "0x194 - "]
    #[inline(always)]
    pub const fn denali_ctl_101(&self) -> &DenaliCtl101 {
        &self.denali_ctl_101
    }
    #[doc = "0x198 - "]
    #[inline(always)]
    pub const fn denali_ctl_102(&self) -> &DenaliCtl102 {
        &self.denali_ctl_102
    }
    #[doc = "0x19c - "]
    #[inline(always)]
    pub const fn denali_ctl_103(&self) -> &DenaliCtl103 {
        &self.denali_ctl_103
    }
    #[doc = "0x1a0 - "]
    #[inline(always)]
    pub const fn denali_ctl_104(&self) -> &DenaliCtl104 {
        &self.denali_ctl_104
    }
    #[doc = "0x1a4 - "]
    #[inline(always)]
    pub const fn denali_ctl_105(&self) -> &DenaliCtl105 {
        &self.denali_ctl_105
    }
    #[doc = "0x1a8 - "]
    #[inline(always)]
    pub const fn denali_ctl_106(&self) -> &DenaliCtl106 {
        &self.denali_ctl_106
    }
    #[doc = "0x1ac - "]
    #[inline(always)]
    pub const fn denali_ctl_107(&self) -> &DenaliCtl107 {
        &self.denali_ctl_107
    }
    #[doc = "0x1b0 - "]
    #[inline(always)]
    pub const fn denali_ctl_108(&self) -> &DenaliCtl108 {
        &self.denali_ctl_108
    }
    #[doc = "0x1b4 - "]
    #[inline(always)]
    pub const fn denali_ctl_109(&self) -> &DenaliCtl109 {
        &self.denali_ctl_109
    }
    #[doc = "0x1b8 - "]
    #[inline(always)]
    pub const fn denali_ctl_110(&self) -> &DenaliCtl110 {
        &self.denali_ctl_110
    }
    #[doc = "0x1bc - "]
    #[inline(always)]
    pub const fn denali_ctl_111(&self) -> &DenaliCtl111 {
        &self.denali_ctl_111
    }
    #[doc = "0x1c0 - "]
    #[inline(always)]
    pub const fn denali_ctl_112(&self) -> &DenaliCtl112 {
        &self.denali_ctl_112
    }
    #[doc = "0x1c4 - "]
    #[inline(always)]
    pub const fn denali_ctl_113(&self) -> &DenaliCtl113 {
        &self.denali_ctl_113
    }
    #[doc = "0x1c8 - "]
    #[inline(always)]
    pub const fn denali_ctl_114(&self) -> &DenaliCtl114 {
        &self.denali_ctl_114
    }
    #[doc = "0x1cc - "]
    #[inline(always)]
    pub const fn denali_ctl_115(&self) -> &DenaliCtl115 {
        &self.denali_ctl_115
    }
    #[doc = "0x1d0 - "]
    #[inline(always)]
    pub const fn denali_ctl_116(&self) -> &DenaliCtl116 {
        &self.denali_ctl_116
    }
    #[doc = "0x1d4 - "]
    #[inline(always)]
    pub const fn denali_ctl_117(&self) -> &DenaliCtl117 {
        &self.denali_ctl_117
    }
    #[doc = "0x1d8 - "]
    #[inline(always)]
    pub const fn denali_ctl_118(&self) -> &DenaliCtl118 {
        &self.denali_ctl_118
    }
    #[doc = "0x1dc - "]
    #[inline(always)]
    pub const fn denali_ctl_119(&self) -> &DenaliCtl119 {
        &self.denali_ctl_119
    }
    #[doc = "0x1e0 - "]
    #[inline(always)]
    pub const fn denali_ctl_120(&self) -> &DenaliCtl120 {
        &self.denali_ctl_120
    }
    #[doc = "0x1e4 - "]
    #[inline(always)]
    pub const fn denali_ctl_121(&self) -> &DenaliCtl121 {
        &self.denali_ctl_121
    }
    #[doc = "0x1e8 - "]
    #[inline(always)]
    pub const fn denali_ctl_122(&self) -> &DenaliCtl122 {
        &self.denali_ctl_122
    }
    #[doc = "0x1ec - "]
    #[inline(always)]
    pub const fn denali_ctl_123(&self) -> &DenaliCtl123 {
        &self.denali_ctl_123
    }
    #[doc = "0x1f0 - "]
    #[inline(always)]
    pub const fn denali_ctl_124(&self) -> &DenaliCtl124 {
        &self.denali_ctl_124
    }
    #[doc = "0x1f4 - "]
    #[inline(always)]
    pub const fn denali_ctl_125(&self) -> &DenaliCtl125 {
        &self.denali_ctl_125
    }
    #[doc = "0x1f8 - "]
    #[inline(always)]
    pub const fn denali_ctl_126(&self) -> &DenaliCtl126 {
        &self.denali_ctl_126
    }
    #[doc = "0x1fc - "]
    #[inline(always)]
    pub const fn denali_ctl_127(&self) -> &DenaliCtl127 {
        &self.denali_ctl_127
    }
    #[doc = "0x200 - "]
    #[inline(always)]
    pub const fn denali_ctl_128(&self) -> &DenaliCtl128 {
        &self.denali_ctl_128
    }
    #[doc = "0x204 - "]
    #[inline(always)]
    pub const fn denali_ctl_129(&self) -> &DenaliCtl129 {
        &self.denali_ctl_129
    }
    #[doc = "0x208 - "]
    #[inline(always)]
    pub const fn denali_ctl_130(&self) -> &DenaliCtl130 {
        &self.denali_ctl_130
    }
    #[doc = "0x20c - "]
    #[inline(always)]
    pub const fn denali_ctl_131(&self) -> &DenaliCtl131 {
        &self.denali_ctl_131
    }
    #[doc = "0x210 - "]
    #[inline(always)]
    pub const fn denali_ctl_132(&self) -> &DenaliCtl132 {
        &self.denali_ctl_132
    }
    #[doc = "0x214 - "]
    #[inline(always)]
    pub const fn denali_ctl_133(&self) -> &DenaliCtl133 {
        &self.denali_ctl_133
    }
    #[doc = "0x218 - "]
    #[inline(always)]
    pub const fn denali_ctl_134(&self) -> &DenaliCtl134 {
        &self.denali_ctl_134
    }
    #[doc = "0x21c - "]
    #[inline(always)]
    pub const fn denali_ctl_135(&self) -> &DenaliCtl135 {
        &self.denali_ctl_135
    }
    #[doc = "0x220 - "]
    #[inline(always)]
    pub const fn denali_ctl_136(&self) -> &DenaliCtl136 {
        &self.denali_ctl_136
    }
    #[doc = "0x224 - "]
    #[inline(always)]
    pub const fn denali_ctl_137(&self) -> &DenaliCtl137 {
        &self.denali_ctl_137
    }
    #[doc = "0x228 - "]
    #[inline(always)]
    pub const fn denali_ctl_138(&self) -> &DenaliCtl138 {
        &self.denali_ctl_138
    }
    #[doc = "0x22c - "]
    #[inline(always)]
    pub const fn denali_ctl_139(&self) -> &DenaliCtl139 {
        &self.denali_ctl_139
    }
    #[doc = "0x230 - "]
    #[inline(always)]
    pub const fn denali_ctl_140(&self) -> &DenaliCtl140 {
        &self.denali_ctl_140
    }
    #[doc = "0x234 - "]
    #[inline(always)]
    pub const fn denali_ctl_141(&self) -> &DenaliCtl141 {
        &self.denali_ctl_141
    }
    #[doc = "0x238 - "]
    #[inline(always)]
    pub const fn denali_ctl_142(&self) -> &DenaliCtl142 {
        &self.denali_ctl_142
    }
    #[doc = "0x23c - "]
    #[inline(always)]
    pub const fn denali_ctl_143(&self) -> &DenaliCtl143 {
        &self.denali_ctl_143
    }
    #[doc = "0x240 - "]
    #[inline(always)]
    pub const fn denali_ctl_144(&self) -> &DenaliCtl144 {
        &self.denali_ctl_144
    }
    #[doc = "0x244 - "]
    #[inline(always)]
    pub const fn denali_ctl_145(&self) -> &DenaliCtl145 {
        &self.denali_ctl_145
    }
    #[doc = "0x248 - "]
    #[inline(always)]
    pub const fn denali_ctl_146(&self) -> &DenaliCtl146 {
        &self.denali_ctl_146
    }
    #[doc = "0x24c - "]
    #[inline(always)]
    pub const fn denali_ctl_147(&self) -> &DenaliCtl147 {
        &self.denali_ctl_147
    }
    #[doc = "0x250 - "]
    #[inline(always)]
    pub const fn denali_ctl_148(&self) -> &DenaliCtl148 {
        &self.denali_ctl_148
    }
    #[doc = "0x254 - "]
    #[inline(always)]
    pub const fn denali_ctl_149(&self) -> &DenaliCtl149 {
        &self.denali_ctl_149
    }
    #[doc = "0x258 - "]
    #[inline(always)]
    pub const fn denali_ctl_150(&self) -> &DenaliCtl150 {
        &self.denali_ctl_150
    }
    #[doc = "0x25c - "]
    #[inline(always)]
    pub const fn denali_ctl_151(&self) -> &DenaliCtl151 {
        &self.denali_ctl_151
    }
    #[doc = "0x260 - "]
    #[inline(always)]
    pub const fn denali_ctl_152(&self) -> &DenaliCtl152 {
        &self.denali_ctl_152
    }
    #[doc = "0x264 - "]
    #[inline(always)]
    pub const fn denali_ctl_153(&self) -> &DenaliCtl153 {
        &self.denali_ctl_153
    }
    #[doc = "0x268 - "]
    #[inline(always)]
    pub const fn denali_ctl_154(&self) -> &DenaliCtl154 {
        &self.denali_ctl_154
    }
    #[doc = "0x26c - "]
    #[inline(always)]
    pub const fn denali_ctl_155(&self) -> &DenaliCtl155 {
        &self.denali_ctl_155
    }
    #[doc = "0x270 - "]
    #[inline(always)]
    pub const fn denali_ctl_156(&self) -> &DenaliCtl156 {
        &self.denali_ctl_156
    }
    #[doc = "0x274 - "]
    #[inline(always)]
    pub const fn denali_ctl_157(&self) -> &DenaliCtl157 {
        &self.denali_ctl_157
    }
    #[doc = "0x278 - "]
    #[inline(always)]
    pub const fn denali_ctl_158(&self) -> &DenaliCtl158 {
        &self.denali_ctl_158
    }
    #[doc = "0x27c - "]
    #[inline(always)]
    pub const fn denali_ctl_159(&self) -> &DenaliCtl159 {
        &self.denali_ctl_159
    }
    #[doc = "0x280 - "]
    #[inline(always)]
    pub const fn denali_ctl_160(&self) -> &DenaliCtl160 {
        &self.denali_ctl_160
    }
    #[doc = "0x284 - "]
    #[inline(always)]
    pub const fn denali_ctl_161(&self) -> &DenaliCtl161 {
        &self.denali_ctl_161
    }
    #[doc = "0x288 - "]
    #[inline(always)]
    pub const fn denali_ctl_162(&self) -> &DenaliCtl162 {
        &self.denali_ctl_162
    }
    #[doc = "0x28c - "]
    #[inline(always)]
    pub const fn denali_ctl_163(&self) -> &DenaliCtl163 {
        &self.denali_ctl_163
    }
    #[doc = "0x290 - "]
    #[inline(always)]
    pub const fn denali_ctl_164(&self) -> &DenaliCtl164 {
        &self.denali_ctl_164
    }
    #[doc = "0x294 - "]
    #[inline(always)]
    pub const fn denali_ctl_165(&self) -> &DenaliCtl165 {
        &self.denali_ctl_165
    }
    #[doc = "0x298 - "]
    #[inline(always)]
    pub const fn denali_ctl_166(&self) -> &DenaliCtl166 {
        &self.denali_ctl_166
    }
    #[doc = "0x29c - "]
    #[inline(always)]
    pub const fn denali_ctl_167(&self) -> &DenaliCtl167 {
        &self.denali_ctl_167
    }
    #[doc = "0x2a0 - "]
    #[inline(always)]
    pub const fn denali_ctl_168(&self) -> &DenaliCtl168 {
        &self.denali_ctl_168
    }
    #[doc = "0x2a4 - "]
    #[inline(always)]
    pub const fn denali_ctl_169(&self) -> &DenaliCtl169 {
        &self.denali_ctl_169
    }
    #[doc = "0x2a8 - "]
    #[inline(always)]
    pub const fn denali_ctl_170(&self) -> &DenaliCtl170 {
        &self.denali_ctl_170
    }
    #[doc = "0x2ac - "]
    #[inline(always)]
    pub const fn denali_ctl_171(&self) -> &DenaliCtl171 {
        &self.denali_ctl_171
    }
    #[doc = "0x2b0 - "]
    #[inline(always)]
    pub const fn denali_ctl_172(&self) -> &DenaliCtl172 {
        &self.denali_ctl_172
    }
    #[doc = "0x2b4 - "]
    #[inline(always)]
    pub const fn denali_ctl_173(&self) -> &DenaliCtl173 {
        &self.denali_ctl_173
    }
    #[doc = "0x2b8 - "]
    #[inline(always)]
    pub const fn denali_ctl_174(&self) -> &DenaliCtl174 {
        &self.denali_ctl_174
    }
    #[doc = "0x2bc - "]
    #[inline(always)]
    pub const fn denali_ctl_175(&self) -> &DenaliCtl175 {
        &self.denali_ctl_175
    }
    #[doc = "0x2c0 - "]
    #[inline(always)]
    pub const fn denali_ctl_176(&self) -> &DenaliCtl176 {
        &self.denali_ctl_176
    }
    #[doc = "0x2c4 - "]
    #[inline(always)]
    pub const fn denali_ctl_177(&self) -> &DenaliCtl177 {
        &self.denali_ctl_177
    }
    #[doc = "0x2c8 - "]
    #[inline(always)]
    pub const fn denali_ctl_178(&self) -> &DenaliCtl178 {
        &self.denali_ctl_178
    }
    #[doc = "0x2cc - "]
    #[inline(always)]
    pub const fn denali_ctl_179(&self) -> &DenaliCtl179 {
        &self.denali_ctl_179
    }
    #[doc = "0x2d0 - "]
    #[inline(always)]
    pub const fn denali_ctl_180(&self) -> &DenaliCtl180 {
        &self.denali_ctl_180
    }
    #[doc = "0x2d4 - "]
    #[inline(always)]
    pub const fn denali_ctl_181(&self) -> &DenaliCtl181 {
        &self.denali_ctl_181
    }
    #[doc = "0x2d8 - "]
    #[inline(always)]
    pub const fn denali_ctl_182(&self) -> &DenaliCtl182 {
        &self.denali_ctl_182
    }
    #[doc = "0x2dc - "]
    #[inline(always)]
    pub const fn denali_ctl_183(&self) -> &DenaliCtl183 {
        &self.denali_ctl_183
    }
    #[doc = "0x2e0 - "]
    #[inline(always)]
    pub const fn denali_ctl_184(&self) -> &DenaliCtl184 {
        &self.denali_ctl_184
    }
    #[doc = "0x2e4 - "]
    #[inline(always)]
    pub const fn denali_ctl_185(&self) -> &DenaliCtl185 {
        &self.denali_ctl_185
    }
    #[doc = "0x2e8 - "]
    #[inline(always)]
    pub const fn denali_ctl_186(&self) -> &DenaliCtl186 {
        &self.denali_ctl_186
    }
    #[doc = "0x2ec - "]
    #[inline(always)]
    pub const fn denali_ctl_187(&self) -> &DenaliCtl187 {
        &self.denali_ctl_187
    }
    #[doc = "0x2f0 - "]
    #[inline(always)]
    pub const fn denali_ctl_188(&self) -> &DenaliCtl188 {
        &self.denali_ctl_188
    }
    #[doc = "0x2f4 - "]
    #[inline(always)]
    pub const fn denali_ctl_189(&self) -> &DenaliCtl189 {
        &self.denali_ctl_189
    }
    #[doc = "0x2f8 - "]
    #[inline(always)]
    pub const fn denali_ctl_190(&self) -> &DenaliCtl190 {
        &self.denali_ctl_190
    }
    #[doc = "0x2fc - "]
    #[inline(always)]
    pub const fn denali_ctl_191(&self) -> &DenaliCtl191 {
        &self.denali_ctl_191
    }
    #[doc = "0x300 - "]
    #[inline(always)]
    pub const fn denali_ctl_192(&self) -> &DenaliCtl192 {
        &self.denali_ctl_192
    }
    #[doc = "0x304 - "]
    #[inline(always)]
    pub const fn denali_ctl_193(&self) -> &DenaliCtl193 {
        &self.denali_ctl_193
    }
    #[doc = "0x308 - "]
    #[inline(always)]
    pub const fn denali_ctl_194(&self) -> &DenaliCtl194 {
        &self.denali_ctl_194
    }
    #[doc = "0x30c - "]
    #[inline(always)]
    pub const fn denali_ctl_195(&self) -> &DenaliCtl195 {
        &self.denali_ctl_195
    }
    #[doc = "0x310 - "]
    #[inline(always)]
    pub const fn denali_ctl_196(&self) -> &DenaliCtl196 {
        &self.denali_ctl_196
    }
    #[doc = "0x314 - "]
    #[inline(always)]
    pub const fn denali_ctl_197(&self) -> &DenaliCtl197 {
        &self.denali_ctl_197
    }
    #[doc = "0x318 - "]
    #[inline(always)]
    pub const fn denali_ctl_198(&self) -> &DenaliCtl198 {
        &self.denali_ctl_198
    }
    #[doc = "0x31c - "]
    #[inline(always)]
    pub const fn denali_ctl_199(&self) -> &DenaliCtl199 {
        &self.denali_ctl_199
    }
    #[doc = "0x320 - "]
    #[inline(always)]
    pub const fn denali_ctl_200(&self) -> &DenaliCtl200 {
        &self.denali_ctl_200
    }
    #[doc = "0x324 - "]
    #[inline(always)]
    pub const fn denali_ctl_201(&self) -> &DenaliCtl201 {
        &self.denali_ctl_201
    }
    #[doc = "0x328 - "]
    #[inline(always)]
    pub const fn denali_ctl_202(&self) -> &DenaliCtl202 {
        &self.denali_ctl_202
    }
    #[doc = "0x32c - "]
    #[inline(always)]
    pub const fn denali_ctl_203(&self) -> &DenaliCtl203 {
        &self.denali_ctl_203
    }
    #[doc = "0x330 - "]
    #[inline(always)]
    pub const fn denali_ctl_204(&self) -> &DenaliCtl204 {
        &self.denali_ctl_204
    }
    #[doc = "0x334 - "]
    #[inline(always)]
    pub const fn denali_ctl_205(&self) -> &DenaliCtl205 {
        &self.denali_ctl_205
    }
    #[doc = "0x338 - "]
    #[inline(always)]
    pub const fn denali_ctl_206(&self) -> &DenaliCtl206 {
        &self.denali_ctl_206
    }
    #[doc = "0x33c - "]
    #[inline(always)]
    pub const fn denali_ctl_207(&self) -> &DenaliCtl207 {
        &self.denali_ctl_207
    }
    #[doc = "0x340 - "]
    #[inline(always)]
    pub const fn denali_ctl_208(&self) -> &DenaliCtl208 {
        &self.denali_ctl_208
    }
    #[doc = "0x344 - "]
    #[inline(always)]
    pub const fn denali_ctl_209(&self) -> &DenaliCtl209 {
        &self.denali_ctl_209
    }
    #[doc = "0x348 - "]
    #[inline(always)]
    pub const fn denali_ctl_210(&self) -> &DenaliCtl210 {
        &self.denali_ctl_210
    }
    #[doc = "0x34c - "]
    #[inline(always)]
    pub const fn denali_ctl_211(&self) -> &DenaliCtl211 {
        &self.denali_ctl_211
    }
    #[doc = "0x350 - "]
    #[inline(always)]
    pub const fn denali_ctl_212(&self) -> &DenaliCtl212 {
        &self.denali_ctl_212
    }
    #[doc = "0x354 - "]
    #[inline(always)]
    pub const fn denali_ctl_213(&self) -> &DenaliCtl213 {
        &self.denali_ctl_213
    }
    #[doc = "0x358 - "]
    #[inline(always)]
    pub const fn denali_ctl_214(&self) -> &DenaliCtl214 {
        &self.denali_ctl_214
    }
    #[doc = "0x35c - "]
    #[inline(always)]
    pub const fn denali_ctl_215(&self) -> &DenaliCtl215 {
        &self.denali_ctl_215
    }
    #[doc = "0x360 - "]
    #[inline(always)]
    pub const fn denali_ctl_216(&self) -> &DenaliCtl216 {
        &self.denali_ctl_216
    }
    #[doc = "0x364 - "]
    #[inline(always)]
    pub const fn denali_ctl_217(&self) -> &DenaliCtl217 {
        &self.denali_ctl_217
    }
    #[doc = "0x368 - "]
    #[inline(always)]
    pub const fn denali_ctl_218(&self) -> &DenaliCtl218 {
        &self.denali_ctl_218
    }
    #[doc = "0x36c - "]
    #[inline(always)]
    pub const fn denali_ctl_219(&self) -> &DenaliCtl219 {
        &self.denali_ctl_219
    }
    #[doc = "0x370 - "]
    #[inline(always)]
    pub const fn denali_ctl_220(&self) -> &DenaliCtl220 {
        &self.denali_ctl_220
    }
    #[doc = "0x374 - "]
    #[inline(always)]
    pub const fn denali_ctl_221(&self) -> &DenaliCtl221 {
        &self.denali_ctl_221
    }
    #[doc = "0x378 - "]
    #[inline(always)]
    pub const fn denali_ctl_222(&self) -> &DenaliCtl222 {
        &self.denali_ctl_222
    }
    #[doc = "0x37c - "]
    #[inline(always)]
    pub const fn denali_ctl_223(&self) -> &DenaliCtl223 {
        &self.denali_ctl_223
    }
    #[doc = "0x380 - "]
    #[inline(always)]
    pub const fn denali_ctl_224(&self) -> &DenaliCtl224 {
        &self.denali_ctl_224
    }
    #[doc = "0x384 - "]
    #[inline(always)]
    pub const fn denali_ctl_225(&self) -> &DenaliCtl225 {
        &self.denali_ctl_225
    }
    #[doc = "0x388 - "]
    #[inline(always)]
    pub const fn denali_ctl_226(&self) -> &DenaliCtl226 {
        &self.denali_ctl_226
    }
    #[doc = "0x38c - "]
    #[inline(always)]
    pub const fn denali_ctl_227(&self) -> &DenaliCtl227 {
        &self.denali_ctl_227
    }
    #[doc = "0x390 - "]
    #[inline(always)]
    pub const fn denali_ctl_228(&self) -> &DenaliCtl228 {
        &self.denali_ctl_228
    }
    #[doc = "0x394 - "]
    #[inline(always)]
    pub const fn denali_ctl_229(&self) -> &DenaliCtl229 {
        &self.denali_ctl_229
    }
    #[doc = "0x398 - "]
    #[inline(always)]
    pub const fn denali_ctl_230(&self) -> &DenaliCtl230 {
        &self.denali_ctl_230
    }
    #[doc = "0x39c - "]
    #[inline(always)]
    pub const fn denali_ctl_231(&self) -> &DenaliCtl231 {
        &self.denali_ctl_231
    }
    #[doc = "0x3a0 - "]
    #[inline(always)]
    pub const fn denali_ctl_232(&self) -> &DenaliCtl232 {
        &self.denali_ctl_232
    }
    #[doc = "0x3a4 - "]
    #[inline(always)]
    pub const fn denali_ctl_233(&self) -> &DenaliCtl233 {
        &self.denali_ctl_233
    }
    #[doc = "0x3a8 - "]
    #[inline(always)]
    pub const fn denali_ctl_234(&self) -> &DenaliCtl234 {
        &self.denali_ctl_234
    }
    #[doc = "0x3ac - "]
    #[inline(always)]
    pub const fn denali_ctl_235(&self) -> &DenaliCtl235 {
        &self.denali_ctl_235
    }
    #[doc = "0x3b0 - "]
    #[inline(always)]
    pub const fn denali_ctl_236(&self) -> &DenaliCtl236 {
        &self.denali_ctl_236
    }
    #[doc = "0x3b4 - "]
    #[inline(always)]
    pub const fn denali_ctl_237(&self) -> &DenaliCtl237 {
        &self.denali_ctl_237
    }
    #[doc = "0x3b8 - "]
    #[inline(always)]
    pub const fn denali_ctl_238(&self) -> &DenaliCtl238 {
        &self.denali_ctl_238
    }
    #[doc = "0x3bc - "]
    #[inline(always)]
    pub const fn denali_ctl_239(&self) -> &DenaliCtl239 {
        &self.denali_ctl_239
    }
    #[doc = "0x3c0 - "]
    #[inline(always)]
    pub const fn denali_ctl_240(&self) -> &DenaliCtl240 {
        &self.denali_ctl_240
    }
    #[doc = "0x3c4 - "]
    #[inline(always)]
    pub const fn denali_ctl_241(&self) -> &DenaliCtl241 {
        &self.denali_ctl_241
    }
    #[doc = "0x3c8 - "]
    #[inline(always)]
    pub const fn denali_ctl_242(&self) -> &DenaliCtl242 {
        &self.denali_ctl_242
    }
    #[doc = "0x3cc - "]
    #[inline(always)]
    pub const fn denali_ctl_243(&self) -> &DenaliCtl243 {
        &self.denali_ctl_243
    }
    #[doc = "0x3d0 - "]
    #[inline(always)]
    pub const fn denali_ctl_244(&self) -> &DenaliCtl244 {
        &self.denali_ctl_244
    }
    #[doc = "0x3d4 - "]
    #[inline(always)]
    pub const fn denali_ctl_245(&self) -> &DenaliCtl245 {
        &self.denali_ctl_245
    }
    #[doc = "0x3d8 - "]
    #[inline(always)]
    pub const fn denali_ctl_246(&self) -> &DenaliCtl246 {
        &self.denali_ctl_246
    }
    #[doc = "0x3dc - "]
    #[inline(always)]
    pub const fn denali_ctl_247(&self) -> &DenaliCtl247 {
        &self.denali_ctl_247
    }
    #[doc = "0x3e0 - "]
    #[inline(always)]
    pub const fn denali_ctl_248(&self) -> &DenaliCtl248 {
        &self.denali_ctl_248
    }
    #[doc = "0x3e4 - "]
    #[inline(always)]
    pub const fn denali_ctl_249(&self) -> &DenaliCtl249 {
        &self.denali_ctl_249
    }
    #[doc = "0x3e8 - "]
    #[inline(always)]
    pub const fn denali_ctl_250(&self) -> &DenaliCtl250 {
        &self.denali_ctl_250
    }
    #[doc = "0x3ec - "]
    #[inline(always)]
    pub const fn denali_ctl_251(&self) -> &DenaliCtl251 {
        &self.denali_ctl_251
    }
    #[doc = "0x3f0 - "]
    #[inline(always)]
    pub const fn denali_ctl_252(&self) -> &DenaliCtl252 {
        &self.denali_ctl_252
    }
    #[doc = "0x3f4 - "]
    #[inline(always)]
    pub const fn denali_ctl_253(&self) -> &DenaliCtl253 {
        &self.denali_ctl_253
    }
    #[doc = "0x3f8 - "]
    #[inline(always)]
    pub const fn denali_ctl_254(&self) -> &DenaliCtl254 {
        &self.denali_ctl_254
    }
    #[doc = "0x3fc - "]
    #[inline(always)]
    pub const fn denali_ctl_255(&self) -> &DenaliCtl255 {
        &self.denali_ctl_255
    }
    #[doc = "0x400 - "]
    #[inline(always)]
    pub const fn denali_ctl_256(&self) -> &DenaliCtl256 {
        &self.denali_ctl_256
    }
    #[doc = "0x404 - "]
    #[inline(always)]
    pub const fn denali_ctl_257(&self) -> &DenaliCtl257 {
        &self.denali_ctl_257
    }
    #[doc = "0x408 - "]
    #[inline(always)]
    pub const fn denali_ctl_258(&self) -> &DenaliCtl258 {
        &self.denali_ctl_258
    }
    #[doc = "0x40c - "]
    #[inline(always)]
    pub const fn denali_ctl_259(&self) -> &DenaliCtl259 {
        &self.denali_ctl_259
    }
    #[doc = "0x410 - "]
    #[inline(always)]
    pub const fn denali_ctl_260(&self) -> &DenaliCtl260 {
        &self.denali_ctl_260
    }
    #[doc = "0x414 - "]
    #[inline(always)]
    pub const fn denali_ctl_261(&self) -> &DenaliCtl261 {
        &self.denali_ctl_261
    }
    #[doc = "0x418 - "]
    #[inline(always)]
    pub const fn denali_ctl_262(&self) -> &DenaliCtl262 {
        &self.denali_ctl_262
    }
    #[doc = "0x41c - "]
    #[inline(always)]
    pub const fn denali_ctl_263(&self) -> &DenaliCtl263 {
        &self.denali_ctl_263
    }
    #[doc = "0x420 - "]
    #[inline(always)]
    pub const fn denali_ctl_264(&self) -> &DenaliCtl264 {
        &self.denali_ctl_264
    }
    #[doc = "0x424 - "]
    #[inline(always)]
    pub const fn denali_ctl_265(&self) -> &DenaliCtl265 {
        &self.denali_ctl_265
    }
    #[doc = "0x428 - "]
    #[inline(always)]
    pub const fn denali_ctl_266(&self) -> &DenaliCtl266 {
        &self.denali_ctl_266
    }
    #[doc = "0x42c - "]
    #[inline(always)]
    pub const fn denali_ctl_267(&self) -> &DenaliCtl267 {
        &self.denali_ctl_267
    }
    #[doc = "0x430 - "]
    #[inline(always)]
    pub const fn denali_ctl_268(&self) -> &DenaliCtl268 {
        &self.denali_ctl_268
    }
    #[doc = "0x434 - "]
    #[inline(always)]
    pub const fn denali_ctl_269(&self) -> &DenaliCtl269 {
        &self.denali_ctl_269
    }
    #[doc = "0x438 - "]
    #[inline(always)]
    pub const fn denali_ctl_270(&self) -> &DenaliCtl270 {
        &self.denali_ctl_270
    }
    #[doc = "0x43c - "]
    #[inline(always)]
    pub const fn denali_ctl_271(&self) -> &DenaliCtl271 {
        &self.denali_ctl_271
    }
    #[doc = "0x440 - "]
    #[inline(always)]
    pub const fn denali_ctl_272(&self) -> &DenaliCtl272 {
        &self.denali_ctl_272
    }
    #[doc = "0x444 - "]
    #[inline(always)]
    pub const fn denali_ctl_273(&self) -> &DenaliCtl273 {
        &self.denali_ctl_273
    }
    #[doc = "0x448 - "]
    #[inline(always)]
    pub const fn denali_ctl_274(&self) -> &DenaliCtl274 {
        &self.denali_ctl_274
    }
    #[doc = "0x44c - "]
    #[inline(always)]
    pub const fn denali_ctl_275(&self) -> &DenaliCtl275 {
        &self.denali_ctl_275
    }
    #[doc = "0x450 - "]
    #[inline(always)]
    pub const fn denali_ctl_276(&self) -> &DenaliCtl276 {
        &self.denali_ctl_276
    }
    #[doc = "0x454 - "]
    #[inline(always)]
    pub const fn denali_ctl_277(&self) -> &DenaliCtl277 {
        &self.denali_ctl_277
    }
    #[doc = "0x458 - "]
    #[inline(always)]
    pub const fn denali_ctl_278(&self) -> &DenaliCtl278 {
        &self.denali_ctl_278
    }
    #[doc = "0x45c - "]
    #[inline(always)]
    pub const fn denali_ctl_279(&self) -> &DenaliCtl279 {
        &self.denali_ctl_279
    }
    #[doc = "0x460 - "]
    #[inline(always)]
    pub const fn denali_ctl_280(&self) -> &DenaliCtl280 {
        &self.denali_ctl_280
    }
    #[doc = "0x464 - "]
    #[inline(always)]
    pub const fn denali_ctl_281(&self) -> &DenaliCtl281 {
        &self.denali_ctl_281
    }
    #[doc = "0x468 - "]
    #[inline(always)]
    pub const fn denali_ctl_282(&self) -> &DenaliCtl282 {
        &self.denali_ctl_282
    }
    #[doc = "0x46c - "]
    #[inline(always)]
    pub const fn denali_ctl_283(&self) -> &DenaliCtl283 {
        &self.denali_ctl_283
    }
    #[doc = "0x470 - "]
    #[inline(always)]
    pub const fn denali_ctl_284(&self) -> &DenaliCtl284 {
        &self.denali_ctl_284
    }
    #[doc = "0x474 - "]
    #[inline(always)]
    pub const fn denali_ctl_285(&self) -> &DenaliCtl285 {
        &self.denali_ctl_285
    }
    #[doc = "0x478 - "]
    #[inline(always)]
    pub const fn denali_ctl_286(&self) -> &DenaliCtl286 {
        &self.denali_ctl_286
    }
    #[doc = "0x47c - "]
    #[inline(always)]
    pub const fn denali_ctl_287(&self) -> &DenaliCtl287 {
        &self.denali_ctl_287
    }
    #[doc = "0x480 - "]
    #[inline(always)]
    pub const fn denali_ctl_288(&self) -> &DenaliCtl288 {
        &self.denali_ctl_288
    }
    #[doc = "0x484 - "]
    #[inline(always)]
    pub const fn denali_ctl_289(&self) -> &DenaliCtl289 {
        &self.denali_ctl_289
    }
    #[doc = "0x488 - "]
    #[inline(always)]
    pub const fn denali_ctl_290(&self) -> &DenaliCtl290 {
        &self.denali_ctl_290
    }
    #[doc = "0x48c - "]
    #[inline(always)]
    pub const fn denali_ctl_291(&self) -> &DenaliCtl291 {
        &self.denali_ctl_291
    }
    #[doc = "0x490 - "]
    #[inline(always)]
    pub const fn denali_ctl_292(&self) -> &DenaliCtl292 {
        &self.denali_ctl_292
    }
    #[doc = "0x494 - "]
    #[inline(always)]
    pub const fn denali_ctl_293(&self) -> &DenaliCtl293 {
        &self.denali_ctl_293
    }
    #[doc = "0x498 - "]
    #[inline(always)]
    pub const fn denali_ctl_294(&self) -> &DenaliCtl294 {
        &self.denali_ctl_294
    }
    #[doc = "0x49c - "]
    #[inline(always)]
    pub const fn denali_ctl_295(&self) -> &DenaliCtl295 {
        &self.denali_ctl_295
    }
    #[doc = "0x4a0 - "]
    #[inline(always)]
    pub const fn denali_ctl_296(&self) -> &DenaliCtl296 {
        &self.denali_ctl_296
    }
    #[doc = "0x4a4 - "]
    #[inline(always)]
    pub const fn denali_ctl_297(&self) -> &DenaliCtl297 {
        &self.denali_ctl_297
    }
    #[doc = "0x4a8 - "]
    #[inline(always)]
    pub const fn denali_ctl_298(&self) -> &DenaliCtl298 {
        &self.denali_ctl_298
    }
    #[doc = "0x4ac - "]
    #[inline(always)]
    pub const fn denali_ctl_299(&self) -> &DenaliCtl299 {
        &self.denali_ctl_299
    }
    #[doc = "0x4b0 - "]
    #[inline(always)]
    pub const fn denali_ctl_300(&self) -> &DenaliCtl300 {
        &self.denali_ctl_300
    }
    #[doc = "0x4b4 - "]
    #[inline(always)]
    pub const fn denali_ctl_301(&self) -> &DenaliCtl301 {
        &self.denali_ctl_301
    }
    #[doc = "0x4b8 - "]
    #[inline(always)]
    pub const fn denali_ctl_302(&self) -> &DenaliCtl302 {
        &self.denali_ctl_302
    }
    #[doc = "0x4bc - "]
    #[inline(always)]
    pub const fn denali_ctl_303(&self) -> &DenaliCtl303 {
        &self.denali_ctl_303
    }
    #[doc = "0x4c0 - "]
    #[inline(always)]
    pub const fn denali_ctl_304(&self) -> &DenaliCtl304 {
        &self.denali_ctl_304
    }
    #[doc = "0x4c4 - "]
    #[inline(always)]
    pub const fn denali_ctl_305(&self) -> &DenaliCtl305 {
        &self.denali_ctl_305
    }
    #[doc = "0x4c8 - "]
    #[inline(always)]
    pub const fn denali_ctl_306(&self) -> &DenaliCtl306 {
        &self.denali_ctl_306
    }
    #[doc = "0x4cc - "]
    #[inline(always)]
    pub const fn denali_ctl_307(&self) -> &DenaliCtl307 {
        &self.denali_ctl_307
    }
    #[doc = "0x4d0 - "]
    #[inline(always)]
    pub const fn denali_ctl_308(&self) -> &DenaliCtl308 {
        &self.denali_ctl_308
    }
    #[doc = "0x4d4 - "]
    #[inline(always)]
    pub const fn denali_ctl_309(&self) -> &DenaliCtl309 {
        &self.denali_ctl_309
    }
    #[doc = "0x4d8 - "]
    #[inline(always)]
    pub const fn denali_ctl_310(&self) -> &DenaliCtl310 {
        &self.denali_ctl_310
    }
    #[doc = "0x4dc - "]
    #[inline(always)]
    pub const fn denali_ctl_311(&self) -> &DenaliCtl311 {
        &self.denali_ctl_311
    }
    #[doc = "0x4e0 - "]
    #[inline(always)]
    pub const fn denali_ctl_312(&self) -> &DenaliCtl312 {
        &self.denali_ctl_312
    }
    #[doc = "0x4e4 - "]
    #[inline(always)]
    pub const fn denali_ctl_313(&self) -> &DenaliCtl313 {
        &self.denali_ctl_313
    }
    #[doc = "0x4e8 - "]
    #[inline(always)]
    pub const fn denali_ctl_314(&self) -> &DenaliCtl314 {
        &self.denali_ctl_314
    }
    #[doc = "0x4ec - "]
    #[inline(always)]
    pub const fn denali_ctl_315(&self) -> &DenaliCtl315 {
        &self.denali_ctl_315
    }
    #[doc = "0x4f0 - "]
    #[inline(always)]
    pub const fn denali_ctl_316(&self) -> &DenaliCtl316 {
        &self.denali_ctl_316
    }
    #[doc = "0x4f4 - "]
    #[inline(always)]
    pub const fn denali_ctl_317(&self) -> &DenaliCtl317 {
        &self.denali_ctl_317
    }
    #[doc = "0x4f8 - "]
    #[inline(always)]
    pub const fn denali_ctl_318(&self) -> &DenaliCtl318 {
        &self.denali_ctl_318
    }
    #[doc = "0x4fc - "]
    #[inline(always)]
    pub const fn denali_ctl_319(&self) -> &DenaliCtl319 {
        &self.denali_ctl_319
    }
    #[doc = "0x500 - "]
    #[inline(always)]
    pub const fn denali_ctl_320(&self) -> &DenaliCtl320 {
        &self.denali_ctl_320
    }
    #[doc = "0x504 - "]
    #[inline(always)]
    pub const fn denali_ctl_321(&self) -> &DenaliCtl321 {
        &self.denali_ctl_321
    }
    #[doc = "0x508 - "]
    #[inline(always)]
    pub const fn denali_ctl_322(&self) -> &DenaliCtl322 {
        &self.denali_ctl_322
    }
    #[doc = "0x50c - "]
    #[inline(always)]
    pub const fn denali_ctl_323(&self) -> &DenaliCtl323 {
        &self.denali_ctl_323
    }
    #[doc = "0x510 - "]
    #[inline(always)]
    pub const fn denali_ctl_324(&self) -> &DenaliCtl324 {
        &self.denali_ctl_324
    }
    #[doc = "0x800 - DDR PHY Independent Register 0"]
    #[inline(always)]
    pub const fn ddr_pi_reg_0(&self) -> &DdrPiReg0 {
        &self.ddr_pi_reg_0
    }
    #[doc = "0x804 - DDR PHY Independent Register 1"]
    #[inline(always)]
    pub const fn ddr_pi_reg_1(&self) -> &DdrPiReg1 {
        &self.ddr_pi_reg_1
    }
    #[doc = "0x808 - DDR PHY Independent Register 2"]
    #[inline(always)]
    pub const fn ddr_pi_reg_2(&self) -> &DdrPiReg2 {
        &self.ddr_pi_reg_2
    }
    #[doc = "0x80c - DDR PHY Independent Register 3"]
    #[inline(always)]
    pub const fn ddr_pi_reg_3(&self) -> &DdrPiReg3 {
        &self.ddr_pi_reg_3
    }
    #[doc = "0x810 - DDR PHY Independent Register 4"]
    #[inline(always)]
    pub const fn ddr_pi_reg_4(&self) -> &DdrPiReg4 {
        &self.ddr_pi_reg_4
    }
    #[doc = "0x814 - DDR PHY Independent Register 5"]
    #[inline(always)]
    pub const fn ddr_pi_reg_5(&self) -> &DdrPiReg5 {
        &self.ddr_pi_reg_5
    }
    #[doc = "0x818 - DDR PHY Independent Register 6"]
    #[inline(always)]
    pub const fn ddr_pi_reg_6(&self) -> &DdrPiReg6 {
        &self.ddr_pi_reg_6
    }
    #[doc = "0x81c - DDR PHY Independent Register 7"]
    #[inline(always)]
    pub const fn ddr_pi_reg_7(&self) -> &DdrPiReg7 {
        &self.ddr_pi_reg_7
    }
    #[doc = "0x820 - DDR PHY Independent Register 8"]
    #[inline(always)]
    pub const fn ddr_pi_reg_8(&self) -> &DdrPiReg8 {
        &self.ddr_pi_reg_8
    }
    #[doc = "0x824 - DDR PHY Independent Register 9"]
    #[inline(always)]
    pub const fn ddr_pi_reg_9(&self) -> &DdrPiReg9 {
        &self.ddr_pi_reg_9
    }
    #[doc = "0x828 - DDR PHY Independent Register 10"]
    #[inline(always)]
    pub const fn ddr_pi_reg_10(&self) -> &DdrPiReg10 {
        &self.ddr_pi_reg_10
    }
    #[doc = "0x82c - DDR PHY Independent Register 11"]
    #[inline(always)]
    pub const fn ddr_pi_reg_11(&self) -> &DdrPiReg11 {
        &self.ddr_pi_reg_11
    }
    #[doc = "0x830 - DDR PHY Independent Register 12"]
    #[inline(always)]
    pub const fn ddr_pi_reg_12(&self) -> &DdrPiReg12 {
        &self.ddr_pi_reg_12
    }
    #[doc = "0x834 - DDR PHY Independent Register 13"]
    #[inline(always)]
    pub const fn ddr_pi_reg_13(&self) -> &DdrPiReg13 {
        &self.ddr_pi_reg_13
    }
    #[doc = "0x838 - DDR PHY Independent Register 14"]
    #[inline(always)]
    pub const fn ddr_pi_reg_14(&self) -> &DdrPiReg14 {
        &self.ddr_pi_reg_14
    }
    #[doc = "0x83c - DDR PHY Independent Register 15"]
    #[inline(always)]
    pub const fn ddr_pi_reg_15(&self) -> &DdrPiReg15 {
        &self.ddr_pi_reg_15
    }
    #[doc = "0x840 - DDR PHY Independent Register 16"]
    #[inline(always)]
    pub const fn ddr_pi_reg_16(&self) -> &DdrPiReg16 {
        &self.ddr_pi_reg_16
    }
    #[doc = "0x844 - DDR PHY Independent Register 17"]
    #[inline(always)]
    pub const fn ddr_pi_reg_17(&self) -> &DdrPiReg17 {
        &self.ddr_pi_reg_17
    }
    #[doc = "0x848 - DDR PHY Independent Register 18"]
    #[inline(always)]
    pub const fn ddr_pi_reg_18(&self) -> &DdrPiReg18 {
        &self.ddr_pi_reg_18
    }
    #[doc = "0x84c - DDR PHY Independent Register 19"]
    #[inline(always)]
    pub const fn ddr_pi_reg_19(&self) -> &DdrPiReg19 {
        &self.ddr_pi_reg_19
    }
    #[doc = "0x850 - DDR PHY Independent Register 20"]
    #[inline(always)]
    pub const fn ddr_pi_reg_20(&self) -> &DdrPiReg20 {
        &self.ddr_pi_reg_20
    }
    #[doc = "0x854 - DDR PHY Independent Register 21"]
    #[inline(always)]
    pub const fn ddr_pi_reg_21(&self) -> &DdrPiReg21 {
        &self.ddr_pi_reg_21
    }
    #[doc = "0x858 - DDR PHY Independent Register 22"]
    #[inline(always)]
    pub const fn ddr_pi_reg_22(&self) -> &DdrPiReg22 {
        &self.ddr_pi_reg_22
    }
    #[doc = "0x85c - DDR PHY Independent Register 23"]
    #[inline(always)]
    pub const fn ddr_pi_reg_23(&self) -> &DdrPiReg23 {
        &self.ddr_pi_reg_23
    }
    #[doc = "0x860 - DDR PHY Independent Register 24"]
    #[inline(always)]
    pub const fn ddr_pi_reg_24(&self) -> &DdrPiReg24 {
        &self.ddr_pi_reg_24
    }
    #[doc = "0x864 - DDR PHY Independent Register 25"]
    #[inline(always)]
    pub const fn ddr_pi_reg_25(&self) -> &DdrPiReg25 {
        &self.ddr_pi_reg_25
    }
    #[doc = "0x868 - DDR PHY Independent Register 26"]
    #[inline(always)]
    pub const fn ddr_pi_reg_26(&self) -> &DdrPiReg26 {
        &self.ddr_pi_reg_26
    }
    #[doc = "0x86c - DDR PHY Independent Register 27"]
    #[inline(always)]
    pub const fn ddr_pi_reg_27(&self) -> &DdrPiReg27 {
        &self.ddr_pi_reg_27
    }
    #[doc = "0x870 - DDR PHY Independent Register 28"]
    #[inline(always)]
    pub const fn ddr_pi_reg_28(&self) -> &DdrPiReg28 {
        &self.ddr_pi_reg_28
    }
    #[doc = "0x874 - DDR PHY Independent Register 29"]
    #[inline(always)]
    pub const fn ddr_pi_reg_29(&self) -> &DdrPiReg29 {
        &self.ddr_pi_reg_29
    }
    #[doc = "0x878 - DDR PHY Independent Register 30"]
    #[inline(always)]
    pub const fn ddr_pi_reg_30(&self) -> &DdrPiReg30 {
        &self.ddr_pi_reg_30
    }
    #[doc = "0x87c - DDR PHY Independent Register 31"]
    #[inline(always)]
    pub const fn ddr_pi_reg_31(&self) -> &DdrPiReg31 {
        &self.ddr_pi_reg_31
    }
    #[doc = "0x880 - DDR PHY Independent Register 32"]
    #[inline(always)]
    pub const fn ddr_pi_reg_32(&self) -> &DdrPiReg32 {
        &self.ddr_pi_reg_32
    }
    #[doc = "0x884 - DDR PHY Independent Register 33"]
    #[inline(always)]
    pub const fn ddr_pi_reg_33(&self) -> &DdrPiReg33 {
        &self.ddr_pi_reg_33
    }
    #[doc = "0x888 - DDR PHY Independent Register 34"]
    #[inline(always)]
    pub const fn ddr_pi_reg_34(&self) -> &DdrPiReg34 {
        &self.ddr_pi_reg_34
    }
    #[doc = "0x88c - DDR PHY Independent Register 35"]
    #[inline(always)]
    pub const fn ddr_pi_reg_35(&self) -> &DdrPiReg35 {
        &self.ddr_pi_reg_35
    }
    #[doc = "0x890 - DDR PHY Independent Register 36"]
    #[inline(always)]
    pub const fn ddr_pi_reg_36(&self) -> &DdrPiReg36 {
        &self.ddr_pi_reg_36
    }
    #[doc = "0x894 - DDR PHY Independent Register 37"]
    #[inline(always)]
    pub const fn ddr_pi_reg_37(&self) -> &DdrPiReg37 {
        &self.ddr_pi_reg_37
    }
    #[doc = "0x898 - DDR PHY Independent Register 38"]
    #[inline(always)]
    pub const fn ddr_pi_reg_38(&self) -> &DdrPiReg38 {
        &self.ddr_pi_reg_38
    }
    #[doc = "0x89c - DDR PHY Independent Register 39"]
    #[inline(always)]
    pub const fn ddr_pi_reg_39(&self) -> &DdrPiReg39 {
        &self.ddr_pi_reg_39
    }
    #[doc = "0x8a0 - DDR PHY Independent Register 40"]
    #[inline(always)]
    pub const fn ddr_pi_reg_40(&self) -> &DdrPiReg40 {
        &self.ddr_pi_reg_40
    }
    #[doc = "0x8a4 - DDR PHY Independent Register 41"]
    #[inline(always)]
    pub const fn ddr_pi_reg_41(&self) -> &DdrPiReg41 {
        &self.ddr_pi_reg_41
    }
    #[doc = "0x8a8 - DDR PHY Independent Register 42"]
    #[inline(always)]
    pub const fn ddr_pi_reg_42(&self) -> &DdrPiReg42 {
        &self.ddr_pi_reg_42
    }
    #[doc = "0x8ac - DDR PHY Independent Register 43"]
    #[inline(always)]
    pub const fn ddr_pi_reg_43(&self) -> &DdrPiReg43 {
        &self.ddr_pi_reg_43
    }
    #[doc = "0x8b0 - DDR PHY Independent Register 44"]
    #[inline(always)]
    pub const fn ddr_pi_reg_44(&self) -> &DdrPiReg44 {
        &self.ddr_pi_reg_44
    }
    #[doc = "0x8b4 - DDR PHY Independent Register 45"]
    #[inline(always)]
    pub const fn ddr_pi_reg_45(&self) -> &DdrPiReg45 {
        &self.ddr_pi_reg_45
    }
    #[doc = "0x8b8 - DDR PHY Independent Register 46"]
    #[inline(always)]
    pub const fn ddr_pi_reg_46(&self) -> &DdrPiReg46 {
        &self.ddr_pi_reg_46
    }
    #[doc = "0x8bc - DDR PHY Independent Register 47"]
    #[inline(always)]
    pub const fn ddr_pi_reg_47(&self) -> &DdrPiReg47 {
        &self.ddr_pi_reg_47
    }
    #[doc = "0x8c0 - DDR PHY Independent Register 48"]
    #[inline(always)]
    pub const fn ddr_pi_reg_48(&self) -> &DdrPiReg48 {
        &self.ddr_pi_reg_48
    }
    #[doc = "0x8c4 - DDR PHY Independent Register 49"]
    #[inline(always)]
    pub const fn ddr_pi_reg_49(&self) -> &DdrPiReg49 {
        &self.ddr_pi_reg_49
    }
    #[doc = "0x8c8 - DDR PHY Independent Register 50"]
    #[inline(always)]
    pub const fn ddr_pi_reg_50(&self) -> &DdrPiReg50 {
        &self.ddr_pi_reg_50
    }
    #[doc = "0x8cc - DDR PHY Independent Register 51"]
    #[inline(always)]
    pub const fn ddr_pi_reg_51(&self) -> &DdrPiReg51 {
        &self.ddr_pi_reg_51
    }
    #[doc = "0x8d0 - DDR PHY Independent Register 52"]
    #[inline(always)]
    pub const fn ddr_pi_reg_52(&self) -> &DdrPiReg52 {
        &self.ddr_pi_reg_52
    }
    #[doc = "0x8d4 - DDR PHY Independent Register 53"]
    #[inline(always)]
    pub const fn ddr_pi_reg_53(&self) -> &DdrPiReg53 {
        &self.ddr_pi_reg_53
    }
    #[doc = "0x8d8 - DDR PHY Independent Register 54"]
    #[inline(always)]
    pub const fn ddr_pi_reg_54(&self) -> &DdrPiReg54 {
        &self.ddr_pi_reg_54
    }
    #[doc = "0x8dc - DDR PHY Independent Register 55"]
    #[inline(always)]
    pub const fn ddr_pi_reg_55(&self) -> &DdrPiReg55 {
        &self.ddr_pi_reg_55
    }
    #[doc = "0x8e0 - DDR PHY Independent Register 56"]
    #[inline(always)]
    pub const fn ddr_pi_reg_56(&self) -> &DdrPiReg56 {
        &self.ddr_pi_reg_56
    }
    #[doc = "0x8e4 - DDR PHY Independent Register 57"]
    #[inline(always)]
    pub const fn ddr_pi_reg_57(&self) -> &DdrPiReg57 {
        &self.ddr_pi_reg_57
    }
    #[doc = "0x8e8 - DDR PHY Independent Register 58"]
    #[inline(always)]
    pub const fn ddr_pi_reg_58(&self) -> &DdrPiReg58 {
        &self.ddr_pi_reg_58
    }
    #[doc = "0x8ec - DDR PHY Independent Register 59"]
    #[inline(always)]
    pub const fn ddr_pi_reg_59(&self) -> &DdrPiReg59 {
        &self.ddr_pi_reg_59
    }
    #[doc = "0x8f0 - DDR PHY Independent Register 60"]
    #[inline(always)]
    pub const fn ddr_pi_reg_60(&self) -> &DdrPiReg60 {
        &self.ddr_pi_reg_60
    }
    #[doc = "0x8f4 - DDR PHY Independent Register 61"]
    #[inline(always)]
    pub const fn ddr_pi_reg_61(&self) -> &DdrPiReg61 {
        &self.ddr_pi_reg_61
    }
    #[doc = "0x8f8 - DDR PHY Independent Register 62"]
    #[inline(always)]
    pub const fn ddr_pi_reg_62(&self) -> &DdrPiReg62 {
        &self.ddr_pi_reg_62
    }
    #[doc = "0x8fc - DDR PHY Independent Register 63"]
    #[inline(always)]
    pub const fn ddr_pi_reg_63(&self) -> &DdrPiReg63 {
        &self.ddr_pi_reg_63
    }
    #[doc = "0x900 - DDR PHY Independent Register 64"]
    #[inline(always)]
    pub const fn ddr_pi_reg_64(&self) -> &DdrPiReg64 {
        &self.ddr_pi_reg_64
    }
    #[doc = "0x904 - DDR PHY Independent Register 65"]
    #[inline(always)]
    pub const fn ddr_pi_reg_65(&self) -> &DdrPiReg65 {
        &self.ddr_pi_reg_65
    }
    #[doc = "0x908 - DDR PHY Independent Register 66"]
    #[inline(always)]
    pub const fn ddr_pi_reg_66(&self) -> &DdrPiReg66 {
        &self.ddr_pi_reg_66
    }
    #[doc = "0x90c - DDR PHY Independent Register 67"]
    #[inline(always)]
    pub const fn ddr_pi_reg_67(&self) -> &DdrPiReg67 {
        &self.ddr_pi_reg_67
    }
    #[doc = "0x910 - DDR PHY Independent Register 68"]
    #[inline(always)]
    pub const fn ddr_pi_reg_68(&self) -> &DdrPiReg68 {
        &self.ddr_pi_reg_68
    }
    #[doc = "0x914 - DDR PHY Independent Register 69"]
    #[inline(always)]
    pub const fn ddr_pi_reg_69(&self) -> &DdrPiReg69 {
        &self.ddr_pi_reg_69
    }
    #[doc = "0x918 - DDR PHY Independent Register 70"]
    #[inline(always)]
    pub const fn ddr_pi_reg_70(&self) -> &DdrPiReg70 {
        &self.ddr_pi_reg_70
    }
    #[doc = "0x91c - DDR PHY Independent Register 71"]
    #[inline(always)]
    pub const fn ddr_pi_reg_71(&self) -> &DdrPiReg71 {
        &self.ddr_pi_reg_71
    }
    #[doc = "0x920 - DDR PHY Independent Register 72"]
    #[inline(always)]
    pub const fn ddr_pi_reg_72(&self) -> &DdrPiReg72 {
        &self.ddr_pi_reg_72
    }
    #[doc = "0x924 - DDR PHY Independent Register 73"]
    #[inline(always)]
    pub const fn ddr_pi_reg_73(&self) -> &DdrPiReg73 {
        &self.ddr_pi_reg_73
    }
    #[doc = "0x928 - DDR PHY Independent Register 74"]
    #[inline(always)]
    pub const fn ddr_pi_reg_74(&self) -> &DdrPiReg74 {
        &self.ddr_pi_reg_74
    }
    #[doc = "0x92c - DDR PHY Independent Register 75"]
    #[inline(always)]
    pub const fn ddr_pi_reg_75(&self) -> &DdrPiReg75 {
        &self.ddr_pi_reg_75
    }
    #[doc = "0x930 - DDR PHY Independent Register 76"]
    #[inline(always)]
    pub const fn ddr_pi_reg_76(&self) -> &DdrPiReg76 {
        &self.ddr_pi_reg_76
    }
    #[doc = "0x934 - DDR PHY Independent Register 77"]
    #[inline(always)]
    pub const fn ddr_pi_reg_77(&self) -> &DdrPiReg77 {
        &self.ddr_pi_reg_77
    }
    #[doc = "0x938 - DDR PHY Independent Register 78"]
    #[inline(always)]
    pub const fn ddr_pi_reg_78(&self) -> &DdrPiReg78 {
        &self.ddr_pi_reg_78
    }
    #[doc = "0x93c - DDR PHY Independent Register 79"]
    #[inline(always)]
    pub const fn ddr_pi_reg_79(&self) -> &DdrPiReg79 {
        &self.ddr_pi_reg_79
    }
    #[doc = "0x940 - DDR PHY Independent Register 80"]
    #[inline(always)]
    pub const fn ddr_pi_reg_80(&self) -> &DdrPiReg80 {
        &self.ddr_pi_reg_80
    }
    #[doc = "0x944 - DDR PHY Independent Register 81"]
    #[inline(always)]
    pub const fn ddr_pi_reg_81(&self) -> &DdrPiReg81 {
        &self.ddr_pi_reg_81
    }
    #[doc = "0x948 - DDR PHY Independent Register 82"]
    #[inline(always)]
    pub const fn ddr_pi_reg_82(&self) -> &DdrPiReg82 {
        &self.ddr_pi_reg_82
    }
    #[doc = "0x94c - DDR PHY Independent Register 83"]
    #[inline(always)]
    pub const fn ddr_pi_reg_83(&self) -> &DdrPiReg83 {
        &self.ddr_pi_reg_83
    }
    #[doc = "0x950 - DDR PHY Independent Register 84"]
    #[inline(always)]
    pub const fn ddr_pi_reg_84(&self) -> &DdrPiReg84 {
        &self.ddr_pi_reg_84
    }
    #[doc = "0x954 - DDR PHY Independent Register 85"]
    #[inline(always)]
    pub const fn ddr_pi_reg_85(&self) -> &DdrPiReg85 {
        &self.ddr_pi_reg_85
    }
    #[doc = "0x958 - DDR PHY Independent Register 86"]
    #[inline(always)]
    pub const fn ddr_pi_reg_86(&self) -> &DdrPiReg86 {
        &self.ddr_pi_reg_86
    }
    #[doc = "0x95c - DDR PHY Independent Register 87"]
    #[inline(always)]
    pub const fn ddr_pi_reg_87(&self) -> &DdrPiReg87 {
        &self.ddr_pi_reg_87
    }
    #[doc = "0x960 - DDR PHY Independent Register 88"]
    #[inline(always)]
    pub const fn ddr_pi_reg_88(&self) -> &DdrPiReg88 {
        &self.ddr_pi_reg_88
    }
    #[doc = "0x964 - DDR PHY Independent Register 89"]
    #[inline(always)]
    pub const fn ddr_pi_reg_89(&self) -> &DdrPiReg89 {
        &self.ddr_pi_reg_89
    }
    #[doc = "0x968 - DDR PHY Independent Register 90"]
    #[inline(always)]
    pub const fn ddr_pi_reg_90(&self) -> &DdrPiReg90 {
        &self.ddr_pi_reg_90
    }
    #[doc = "0x96c - DDR PHY Independent Register 91"]
    #[inline(always)]
    pub const fn ddr_pi_reg_91(&self) -> &DdrPiReg91 {
        &self.ddr_pi_reg_91
    }
    #[doc = "0x970 - DDR PHY Independent Register 92"]
    #[inline(always)]
    pub const fn ddr_pi_reg_92(&self) -> &DdrPiReg92 {
        &self.ddr_pi_reg_92
    }
    #[doc = "0x974 - DDR PHY Independent Register 93"]
    #[inline(always)]
    pub const fn ddr_pi_reg_93(&self) -> &DdrPiReg93 {
        &self.ddr_pi_reg_93
    }
    #[doc = "0x978 - DDR PHY Independent Register 94"]
    #[inline(always)]
    pub const fn ddr_pi_reg_94(&self) -> &DdrPiReg94 {
        &self.ddr_pi_reg_94
    }
    #[doc = "0x97c - DDR PHY Independent Register 95"]
    #[inline(always)]
    pub const fn ddr_pi_reg_95(&self) -> &DdrPiReg95 {
        &self.ddr_pi_reg_95
    }
    #[doc = "0x980 - DDR PHY Independent Register 96"]
    #[inline(always)]
    pub const fn ddr_pi_reg_96(&self) -> &DdrPiReg96 {
        &self.ddr_pi_reg_96
    }
    #[doc = "0x984 - DDR PHY Independent Register 97"]
    #[inline(always)]
    pub const fn ddr_pi_reg_97(&self) -> &DdrPiReg97 {
        &self.ddr_pi_reg_97
    }
    #[doc = "0x988 - DDR PHY Independent Register 98"]
    #[inline(always)]
    pub const fn ddr_pi_reg_98(&self) -> &DdrPiReg98 {
        &self.ddr_pi_reg_98
    }
    #[doc = "0x98c - DDR PHY Independent Register 99"]
    #[inline(always)]
    pub const fn ddr_pi_reg_99(&self) -> &DdrPiReg99 {
        &self.ddr_pi_reg_99
    }
    #[doc = "0x990 - DDR PHY Independent Register 100"]
    #[inline(always)]
    pub const fn ddr_pi_reg_100(&self) -> &DdrPiReg100 {
        &self.ddr_pi_reg_100
    }
    #[doc = "0x994 - DDR PHY Independent Register 101"]
    #[inline(always)]
    pub const fn ddr_pi_reg_101(&self) -> &DdrPiReg101 {
        &self.ddr_pi_reg_101
    }
    #[doc = "0x998 - DDR PHY Independent Register 102"]
    #[inline(always)]
    pub const fn ddr_pi_reg_102(&self) -> &DdrPiReg102 {
        &self.ddr_pi_reg_102
    }
    #[doc = "0x99c - DDR PHY Independent Register 103"]
    #[inline(always)]
    pub const fn ddr_pi_reg_103(&self) -> &DdrPiReg103 {
        &self.ddr_pi_reg_103
    }
    #[doc = "0x9a0 - DDR PHY Independent Register 104"]
    #[inline(always)]
    pub const fn ddr_pi_reg_104(&self) -> &DdrPiReg104 {
        &self.ddr_pi_reg_104
    }
    #[doc = "0x9a4 - DDR PHY Independent Register 105"]
    #[inline(always)]
    pub const fn ddr_pi_reg_105(&self) -> &DdrPiReg105 {
        &self.ddr_pi_reg_105
    }
    #[doc = "0x9a8 - DDR PHY Independent Register 106"]
    #[inline(always)]
    pub const fn ddr_pi_reg_106(&self) -> &DdrPiReg106 {
        &self.ddr_pi_reg_106
    }
    #[doc = "0x9ac - DDR PHY Independent Register 107"]
    #[inline(always)]
    pub const fn ddr_pi_reg_107(&self) -> &DdrPiReg107 {
        &self.ddr_pi_reg_107
    }
    #[doc = "0x9b0 - DDR PHY Independent Register 108"]
    #[inline(always)]
    pub const fn ddr_pi_reg_108(&self) -> &DdrPiReg108 {
        &self.ddr_pi_reg_108
    }
    #[doc = "0x9b4 - DDR PHY Independent Register 109"]
    #[inline(always)]
    pub const fn ddr_pi_reg_109(&self) -> &DdrPiReg109 {
        &self.ddr_pi_reg_109
    }
    #[doc = "0x9b8 - DDR PHY Independent Register 110"]
    #[inline(always)]
    pub const fn ddr_pi_reg_110(&self) -> &DdrPiReg110 {
        &self.ddr_pi_reg_110
    }
    #[doc = "0x9bc - DDR PHY Independent Register 111"]
    #[inline(always)]
    pub const fn ddr_pi_reg_111(&self) -> &DdrPiReg111 {
        &self.ddr_pi_reg_111
    }
    #[doc = "0x9c0 - DDR PHY Independent Register 112"]
    #[inline(always)]
    pub const fn ddr_pi_reg_112(&self) -> &DdrPiReg112 {
        &self.ddr_pi_reg_112
    }
    #[doc = "0x9c4 - DDR PHY Independent Register 113"]
    #[inline(always)]
    pub const fn ddr_pi_reg_113(&self) -> &DdrPiReg113 {
        &self.ddr_pi_reg_113
    }
    #[doc = "0x9c8 - DDR PHY Independent Register 114"]
    #[inline(always)]
    pub const fn ddr_pi_reg_114(&self) -> &DdrPiReg114 {
        &self.ddr_pi_reg_114
    }
    #[doc = "0x9cc - DDR PHY Independent Register 115"]
    #[inline(always)]
    pub const fn ddr_pi_reg_115(&self) -> &DdrPiReg115 {
        &self.ddr_pi_reg_115
    }
    #[doc = "0x9d0 - DDR PHY Independent Register 116"]
    #[inline(always)]
    pub const fn ddr_pi_reg_116(&self) -> &DdrPiReg116 {
        &self.ddr_pi_reg_116
    }
    #[doc = "0x9d4 - DDR PHY Independent Register 117"]
    #[inline(always)]
    pub const fn ddr_pi_reg_117(&self) -> &DdrPiReg117 {
        &self.ddr_pi_reg_117
    }
    #[doc = "0x9d8 - DDR PHY Independent Register 118"]
    #[inline(always)]
    pub const fn ddr_pi_reg_118(&self) -> &DdrPiReg118 {
        &self.ddr_pi_reg_118
    }
    #[doc = "0x9dc - DDR PHY Independent Register 119"]
    #[inline(always)]
    pub const fn ddr_pi_reg_119(&self) -> &DdrPiReg119 {
        &self.ddr_pi_reg_119
    }
    #[doc = "0x9e0 - DDR PHY Independent Register 120"]
    #[inline(always)]
    pub const fn ddr_pi_reg_120(&self) -> &DdrPiReg120 {
        &self.ddr_pi_reg_120
    }
    #[doc = "0x9e4 - DDR PHY Independent Register 121"]
    #[inline(always)]
    pub const fn ddr_pi_reg_121(&self) -> &DdrPiReg121 {
        &self.ddr_pi_reg_121
    }
    #[doc = "0x9e8 - DDR PHY Independent Register 122"]
    #[inline(always)]
    pub const fn ddr_pi_reg_122(&self) -> &DdrPiReg122 {
        &self.ddr_pi_reg_122
    }
    #[doc = "0x9ec - DDR PHY Independent Register 123"]
    #[inline(always)]
    pub const fn ddr_pi_reg_123(&self) -> &DdrPiReg123 {
        &self.ddr_pi_reg_123
    }
    #[doc = "0x9f0 - DDR PHY Independent Register 124"]
    #[inline(always)]
    pub const fn ddr_pi_reg_124(&self) -> &DdrPiReg124 {
        &self.ddr_pi_reg_124
    }
    #[doc = "0x9f4 - DDR PHY Independent Register 125"]
    #[inline(always)]
    pub const fn ddr_pi_reg_125(&self) -> &DdrPiReg125 {
        &self.ddr_pi_reg_125
    }
    #[doc = "0x9f8 - DDR PHY Independent Register 126"]
    #[inline(always)]
    pub const fn ddr_pi_reg_126(&self) -> &DdrPiReg126 {
        &self.ddr_pi_reg_126
    }
    #[doc = "0x9fc - DDR PHY Independent Register 127"]
    #[inline(always)]
    pub const fn ddr_pi_reg_127(&self) -> &DdrPiReg127 {
        &self.ddr_pi_reg_127
    }
    #[doc = "0xa00 - DDR PHY Independent Register 128"]
    #[inline(always)]
    pub const fn ddr_pi_reg_128(&self) -> &DdrPiReg128 {
        &self.ddr_pi_reg_128
    }
    #[doc = "0xa04 - DDR PHY Independent Register 129"]
    #[inline(always)]
    pub const fn ddr_pi_reg_129(&self) -> &DdrPiReg129 {
        &self.ddr_pi_reg_129
    }
    #[doc = "0xa08 - DDR PHY Independent Register 130"]
    #[inline(always)]
    pub const fn ddr_pi_reg_130(&self) -> &DdrPiReg130 {
        &self.ddr_pi_reg_130
    }
    #[doc = "0xa0c - DDR PHY Independent Register 131"]
    #[inline(always)]
    pub const fn ddr_pi_reg_131(&self) -> &DdrPiReg131 {
        &self.ddr_pi_reg_131
    }
    #[doc = "0xa10 - DDR PHY Independent Register 132"]
    #[inline(always)]
    pub const fn ddr_pi_reg_132(&self) -> &DdrPiReg132 {
        &self.ddr_pi_reg_132
    }
    #[doc = "0xa14 - DDR PHY Independent Register 133"]
    #[inline(always)]
    pub const fn ddr_pi_reg_133(&self) -> &DdrPiReg133 {
        &self.ddr_pi_reg_133
    }
    #[doc = "0xa18 - DDR PHY Independent Register 134"]
    #[inline(always)]
    pub const fn ddr_pi_reg_134(&self) -> &DdrPiReg134 {
        &self.ddr_pi_reg_134
    }
    #[doc = "0xa1c - DDR PHY Independent Register 135"]
    #[inline(always)]
    pub const fn ddr_pi_reg_135(&self) -> &DdrPiReg135 {
        &self.ddr_pi_reg_135
    }
    #[doc = "0xa20 - DDR PHY Independent Register 136"]
    #[inline(always)]
    pub const fn ddr_pi_reg_136(&self) -> &DdrPiReg136 {
        &self.ddr_pi_reg_136
    }
    #[doc = "0xa24 - DDR PHY Independent Register 137"]
    #[inline(always)]
    pub const fn ddr_pi_reg_137(&self) -> &DdrPiReg137 {
        &self.ddr_pi_reg_137
    }
    #[doc = "0xa28 - DDR PHY Independent Register 138"]
    #[inline(always)]
    pub const fn ddr_pi_reg_138(&self) -> &DdrPiReg138 {
        &self.ddr_pi_reg_138
    }
    #[doc = "0xa2c - DDR PHY Independent Register 139"]
    #[inline(always)]
    pub const fn ddr_pi_reg_139(&self) -> &DdrPiReg139 {
        &self.ddr_pi_reg_139
    }
    #[doc = "0xa30 - DDR PHY Independent Register 140"]
    #[inline(always)]
    pub const fn ddr_pi_reg_140(&self) -> &DdrPiReg140 {
        &self.ddr_pi_reg_140
    }
    #[doc = "0xa6c - DDR PHY Independent Register 155"]
    #[inline(always)]
    pub const fn ddr_pi_reg_155(&self) -> &DdrPiReg155 {
        &self.ddr_pi_reg_155
    }
    #[doc = "0xa70 - DDR PHY Independent Register 156"]
    #[inline(always)]
    pub const fn ddr_pi_reg_156(&self) -> &DdrPiReg156 {
        &self.ddr_pi_reg_156
    }
    #[doc = "0xa74 - DDR PHY Independent Register 157"]
    #[inline(always)]
    pub const fn ddr_pi_reg_157(&self) -> &DdrPiReg157 {
        &self.ddr_pi_reg_157
    }
    #[doc = "0xa78 - DDR PHY Independent Register 158"]
    #[inline(always)]
    pub const fn ddr_pi_reg_158(&self) -> &DdrPiReg158 {
        &self.ddr_pi_reg_158
    }
    #[doc = "0xa7c - DDR PHY Independent Register 159"]
    #[inline(always)]
    pub const fn ddr_pi_reg_159(&self) -> &DdrPiReg159 {
        &self.ddr_pi_reg_159
    }
    #[doc = "0xa80 - DDR PHY Independent Register 160"]
    #[inline(always)]
    pub const fn ddr_pi_reg_160(&self) -> &DdrPiReg160 {
        &self.ddr_pi_reg_160
    }
    #[doc = "0xa84 - DDR PHY Independent Register 161"]
    #[inline(always)]
    pub const fn ddr_pi_reg_161(&self) -> &DdrPiReg161 {
        &self.ddr_pi_reg_161
    }
    #[doc = "0xa88 - DDR PHY Independent Register 162"]
    #[inline(always)]
    pub const fn ddr_pi_reg_162(&self) -> &DdrPiReg162 {
        &self.ddr_pi_reg_162
    }
    #[doc = "0xa8c - DDR PHY Independent Register 163"]
    #[inline(always)]
    pub const fn ddr_pi_reg_163(&self) -> &DdrPiReg163 {
        &self.ddr_pi_reg_163
    }
    #[doc = "0xa90 - DDR PHY Independent Register 164"]
    #[inline(always)]
    pub const fn ddr_pi_reg_164(&self) -> &DdrPiReg164 {
        &self.ddr_pi_reg_164
    }
    #[doc = "0xa94 - DDR PHY Independent Register 165"]
    #[inline(always)]
    pub const fn ddr_pi_reg_165(&self) -> &DdrPiReg165 {
        &self.ddr_pi_reg_165
    }
    #[doc = "0xa98 - DDR PHY Independent Register 166"]
    #[inline(always)]
    pub const fn ddr_pi_reg_166(&self) -> &DdrPiReg166 {
        &self.ddr_pi_reg_166
    }
    #[doc = "0xa9c - DDR PHY Independent Register 167"]
    #[inline(always)]
    pub const fn ddr_pi_reg_167(&self) -> &DdrPiReg167 {
        &self.ddr_pi_reg_167
    }
    #[doc = "0xaa0 - DDR PHY Independent Register 168"]
    #[inline(always)]
    pub const fn ddr_pi_reg_168(&self) -> &DdrPiReg168 {
        &self.ddr_pi_reg_168
    }
    #[doc = "0xaa4 - DDR PHY Independent Register 169"]
    #[inline(always)]
    pub const fn ddr_pi_reg_169(&self) -> &DdrPiReg169 {
        &self.ddr_pi_reg_169
    }
    #[doc = "0xab8 - DDR PHY Independent Register 174"]
    #[inline(always)]
    pub const fn ddr_pi_reg_174(&self) -> &DdrPiReg174 {
        &self.ddr_pi_reg_174
    }
    #[doc = "0xabc - DDR PHY Independent Register 175"]
    #[inline(always)]
    pub const fn ddr_pi_reg_175(&self) -> &DdrPiReg175 {
        &self.ddr_pi_reg_175
    }
    #[doc = "0xac0 - DDR PHY Independent Register 176"]
    #[inline(always)]
    pub const fn ddr_pi_reg_176(&self) -> &DdrPiReg176 {
        &self.ddr_pi_reg_176
    }
    #[doc = "0xae8 - DDR PHY Independent Register 186"]
    #[inline(always)]
    pub const fn ddr_pi_reg_186(&self) -> &DdrPiReg186 {
        &self.ddr_pi_reg_186
    }
    #[doc = "0xaec - DDR PHY Independent Register 187"]
    #[inline(always)]
    pub const fn ddr_pi_reg_187(&self) -> &DdrPiReg187 {
        &self.ddr_pi_reg_187
    }
    #[doc = "0xaf0 - DDR PHY Independent Register 188"]
    #[inline(always)]
    pub const fn ddr_pi_reg_188(&self) -> &DdrPiReg188 {
        &self.ddr_pi_reg_188
    }
    #[doc = "0xaf4 - DDR PHY Independent Register 189"]
    #[inline(always)]
    pub const fn ddr_pi_reg_189(&self) -> &DdrPiReg189 {
        &self.ddr_pi_reg_189
    }
    #[doc = "0xaf8 - DDR PHY Independent Register 190"]
    #[inline(always)]
    pub const fn ddr_pi_reg_190(&self) -> &DdrPiReg190 {
        &self.ddr_pi_reg_190
    }
    #[doc = "0xafc - DDR PHY Independent Register 191"]
    #[inline(always)]
    pub const fn ddr_pi_reg_191(&self) -> &DdrPiReg191 {
        &self.ddr_pi_reg_191
    }
    #[doc = "0xb00 - DDR PHY Independent Register 192"]
    #[inline(always)]
    pub const fn ddr_pi_reg_192(&self) -> &DdrPiReg192 {
        &self.ddr_pi_reg_192
    }
    #[doc = "0xb04 - DDR PHY Independent Register 193"]
    #[inline(always)]
    pub const fn ddr_pi_reg_193(&self) -> &DdrPiReg193 {
        &self.ddr_pi_reg_193
    }
    #[doc = "0xb1c - DDR PHY Independent Register 199"]
    #[inline(always)]
    pub const fn ddr_pi_reg_199(&self) -> &DdrPiReg199 {
        &self.ddr_pi_reg_199
    }
    #[doc = "0x2000 - "]
    #[inline(always)]
    pub const fn denali_phy_00(&self) -> &DenaliPhy00 {
        &self.denali_phy_00
    }
    #[doc = "0x2004 - "]
    #[inline(always)]
    pub const fn denali_phy_01(&self) -> &DenaliPhy01 {
        &self.denali_phy_01
    }
    #[doc = "0x2008 - "]
    #[inline(always)]
    pub const fn denali_phy_02(&self) -> &DenaliPhy02 {
        &self.denali_phy_02
    }
    #[doc = "0x200c - "]
    #[inline(always)]
    pub const fn denali_phy_03(&self) -> &DenaliPhy03 {
        &self.denali_phy_03
    }
    #[doc = "0x2010 - "]
    #[inline(always)]
    pub const fn denali_phy_04(&self) -> &DenaliPhy04 {
        &self.denali_phy_04
    }
    #[doc = "0x2014 - "]
    #[inline(always)]
    pub const fn denali_phy_05(&self) -> &DenaliPhy05 {
        &self.denali_phy_05
    }
    #[doc = "0x2018 - "]
    #[inline(always)]
    pub const fn denali_phy_06(&self) -> &DenaliPhy06 {
        &self.denali_phy_06
    }
    #[doc = "0x201c - "]
    #[inline(always)]
    pub const fn denali_phy_07(&self) -> &DenaliPhy07 {
        &self.denali_phy_07
    }
    #[doc = "0x2020 - "]
    #[inline(always)]
    pub const fn denali_phy_08(&self) -> &DenaliPhy08 {
        &self.denali_phy_08
    }
    #[doc = "0x2024 - "]
    #[inline(always)]
    pub const fn denali_phy_09(&self) -> &DenaliPhy09 {
        &self.denali_phy_09
    }
    #[doc = "0x2028 - "]
    #[inline(always)]
    pub const fn denali_phy_10(&self) -> &DenaliPhy10 {
        &self.denali_phy_10
    }
    #[doc = "0x202c - "]
    #[inline(always)]
    pub const fn denali_phy_11(&self) -> &DenaliPhy11 {
        &self.denali_phy_11
    }
    #[doc = "0x2030 - "]
    #[inline(always)]
    pub const fn denali_phy_12(&self) -> &DenaliPhy12 {
        &self.denali_phy_12
    }
    #[doc = "0x2034 - "]
    #[inline(always)]
    pub const fn denali_phy_13(&self) -> &DenaliPhy13 {
        &self.denali_phy_13
    }
    #[doc = "0x2038 - "]
    #[inline(always)]
    pub const fn denali_phy_14(&self) -> &DenaliPhy14 {
        &self.denali_phy_14
    }
    #[doc = "0x203c - "]
    #[inline(always)]
    pub const fn denali_phy_15(&self) -> &DenaliPhy15 {
        &self.denali_phy_15
    }
    #[doc = "0x2040 - "]
    #[inline(always)]
    pub const fn denali_phy_16(&self) -> &DenaliPhy16 {
        &self.denali_phy_16
    }
    #[doc = "0x2044 - "]
    #[inline(always)]
    pub const fn denali_phy_17(&self) -> &DenaliPhy17 {
        &self.denali_phy_17
    }
    #[doc = "0x2048 - "]
    #[inline(always)]
    pub const fn denali_phy_18(&self) -> &DenaliPhy18 {
        &self.denali_phy_18
    }
    #[doc = "0x204c - "]
    #[inline(always)]
    pub const fn denali_phy_19(&self) -> &DenaliPhy19 {
        &self.denali_phy_19
    }
    #[doc = "0x2050 - "]
    #[inline(always)]
    pub const fn denali_phy_20(&self) -> &DenaliPhy20 {
        &self.denali_phy_20
    }
    #[doc = "0x2054 - "]
    #[inline(always)]
    pub const fn denali_phy_21(&self) -> &DenaliPhy21 {
        &self.denali_phy_21
    }
    #[doc = "0x2058 - "]
    #[inline(always)]
    pub const fn denali_phy_22(&self) -> &DenaliPhy22 {
        &self.denali_phy_22
    }
    #[doc = "0x205c - "]
    #[inline(always)]
    pub const fn denali_phy_23(&self) -> &DenaliPhy23 {
        &self.denali_phy_23
    }
    #[doc = "0x2060 - "]
    #[inline(always)]
    pub const fn denali_phy_24(&self) -> &DenaliPhy24 {
        &self.denali_phy_24
    }
    #[doc = "0x2064 - "]
    #[inline(always)]
    pub const fn denali_phy_25(&self) -> &DenaliPhy25 {
        &self.denali_phy_25
    }
    #[doc = "0x2068 - "]
    #[inline(always)]
    pub const fn denali_phy_26(&self) -> &DenaliPhy26 {
        &self.denali_phy_26
    }
    #[doc = "0x206c - "]
    #[inline(always)]
    pub const fn denali_phy_27(&self) -> &DenaliPhy27 {
        &self.denali_phy_27
    }
    #[doc = "0x2070 - "]
    #[inline(always)]
    pub const fn denali_phy_28(&self) -> &DenaliPhy28 {
        &self.denali_phy_28
    }
    #[doc = "0x2074 - "]
    #[inline(always)]
    pub const fn denali_phy_29(&self) -> &DenaliPhy29 {
        &self.denali_phy_29
    }
    #[doc = "0x2078 - "]
    #[inline(always)]
    pub const fn denali_phy_30(&self) -> &DenaliPhy30 {
        &self.denali_phy_30
    }
    #[doc = "0x207c - "]
    #[inline(always)]
    pub const fn denali_phy_31(&self) -> &DenaliPhy31 {
        &self.denali_phy_31
    }
    #[doc = "0x2080 - "]
    #[inline(always)]
    pub const fn denali_phy_32(&self) -> &DenaliPhy32 {
        &self.denali_phy_32
    }
    #[doc = "0x2084 - "]
    #[inline(always)]
    pub const fn denali_phy_33(&self) -> &DenaliPhy33 {
        &self.denali_phy_33
    }
    #[doc = "0x2088 - "]
    #[inline(always)]
    pub const fn denali_phy_34(&self) -> &DenaliPhy34 {
        &self.denali_phy_34
    }
    #[doc = "0x208c - "]
    #[inline(always)]
    pub const fn denali_phy_35(&self) -> &DenaliPhy35 {
        &self.denali_phy_35
    }
    #[doc = "0x2090 - "]
    #[inline(always)]
    pub const fn denali_phy_36(&self) -> &DenaliPhy36 {
        &self.denali_phy_36
    }
    #[doc = "0x2094 - "]
    #[inline(always)]
    pub const fn denali_phy_37(&self) -> &DenaliPhy37 {
        &self.denali_phy_37
    }
    #[doc = "0x2098 - "]
    #[inline(always)]
    pub const fn denali_phy_38(&self) -> &DenaliPhy38 {
        &self.denali_phy_38
    }
    #[doc = "0x209c - "]
    #[inline(always)]
    pub const fn denali_phy_39(&self) -> &DenaliPhy39 {
        &self.denali_phy_39
    }
    #[doc = "0x20a0 - "]
    #[inline(always)]
    pub const fn denali_phy_40(&self) -> &DenaliPhy40 {
        &self.denali_phy_40
    }
    #[doc = "0x20a4 - "]
    #[inline(always)]
    pub const fn denali_phy_41(&self) -> &DenaliPhy41 {
        &self.denali_phy_41
    }
    #[doc = "0x20a8 - "]
    #[inline(always)]
    pub const fn denali_phy_42(&self) -> &DenaliPhy42 {
        &self.denali_phy_42
    }
    #[doc = "0x20ac - "]
    #[inline(always)]
    pub const fn denali_phy_43(&self) -> &DenaliPhy43 {
        &self.denali_phy_43
    }
    #[doc = "0x20b0 - "]
    #[inline(always)]
    pub const fn denali_phy_44(&self) -> &DenaliPhy44 {
        &self.denali_phy_44
    }
    #[doc = "0x20b4 - "]
    #[inline(always)]
    pub const fn denali_phy_45(&self) -> &DenaliPhy45 {
        &self.denali_phy_45
    }
    #[doc = "0x20b8 - "]
    #[inline(always)]
    pub const fn denali_phy_46(&self) -> &DenaliPhy46 {
        &self.denali_phy_46
    }
    #[doc = "0x20bc - "]
    #[inline(always)]
    pub const fn denali_phy_47(&self) -> &DenaliPhy47 {
        &self.denali_phy_47
    }
    #[doc = "0x20c0 - "]
    #[inline(always)]
    pub const fn denali_phy_48(&self) -> &DenaliPhy48 {
        &self.denali_phy_48
    }
    #[doc = "0x20c4 - "]
    #[inline(always)]
    pub const fn denali_phy_49(&self) -> &DenaliPhy49 {
        &self.denali_phy_49
    }
    #[doc = "0x20c8 - "]
    #[inline(always)]
    pub const fn denali_phy_50(&self) -> &DenaliPhy50 {
        &self.denali_phy_50
    }
    #[doc = "0x20cc - "]
    #[inline(always)]
    pub const fn denali_phy_51(&self) -> &DenaliPhy51 {
        &self.denali_phy_51
    }
    #[doc = "0x20d0 - "]
    #[inline(always)]
    pub const fn denali_phy_52(&self) -> &DenaliPhy52 {
        &self.denali_phy_52
    }
    #[doc = "0x20d4 - "]
    #[inline(always)]
    pub const fn denali_phy_53(&self) -> &DenaliPhy53 {
        &self.denali_phy_53
    }
    #[doc = "0x20d8 - "]
    #[inline(always)]
    pub const fn denali_phy_54(&self) -> &DenaliPhy54 {
        &self.denali_phy_54
    }
    #[doc = "0x20dc - "]
    #[inline(always)]
    pub const fn denali_phy_55(&self) -> &DenaliPhy55 {
        &self.denali_phy_55
    }
    #[doc = "0x20e0 - "]
    #[inline(always)]
    pub const fn denali_phy_56(&self) -> &DenaliPhy56 {
        &self.denali_phy_56
    }
    #[doc = "0x20e4 - "]
    #[inline(always)]
    pub const fn denali_phy_57(&self) -> &DenaliPhy57 {
        &self.denali_phy_57
    }
    #[doc = "0x20e8 - "]
    #[inline(always)]
    pub const fn denali_phy_58(&self) -> &DenaliPhy58 {
        &self.denali_phy_58
    }
    #[doc = "0x20ec - "]
    #[inline(always)]
    pub const fn denali_phy_59(&self) -> &DenaliPhy59 {
        &self.denali_phy_59
    }
    #[doc = "0x20f0 - "]
    #[inline(always)]
    pub const fn denali_phy_60(&self) -> &DenaliPhy60 {
        &self.denali_phy_60
    }
    #[doc = "0x20f4 - "]
    #[inline(always)]
    pub const fn denali_phy_61(&self) -> &DenaliPhy61 {
        &self.denali_phy_61
    }
    #[doc = "0x20f8 - "]
    #[inline(always)]
    pub const fn denali_phy_62(&self) -> &DenaliPhy62 {
        &self.denali_phy_62
    }
    #[doc = "0x20fc - "]
    #[inline(always)]
    pub const fn denali_phy_63(&self) -> &DenaliPhy63 {
        &self.denali_phy_63
    }
    #[doc = "0x2100 - "]
    #[inline(always)]
    pub const fn denali_phy_64(&self) -> &DenaliPhy64 {
        &self.denali_phy_64
    }
    #[doc = "0x2104 - "]
    #[inline(always)]
    pub const fn denali_phy_65(&self) -> &DenaliPhy65 {
        &self.denali_phy_65
    }
    #[doc = "0x2108 - "]
    #[inline(always)]
    pub const fn denali_phy_66(&self) -> &DenaliPhy66 {
        &self.denali_phy_66
    }
    #[doc = "0x210c - "]
    #[inline(always)]
    pub const fn denali_phy_67(&self) -> &DenaliPhy67 {
        &self.denali_phy_67
    }
    #[doc = "0x2110 - "]
    #[inline(always)]
    pub const fn denali_phy_68(&self) -> &DenaliPhy68 {
        &self.denali_phy_68
    }
    #[doc = "0x2114 - "]
    #[inline(always)]
    pub const fn denali_phy_69(&self) -> &DenaliPhy69 {
        &self.denali_phy_69
    }
    #[doc = "0x2118 - "]
    #[inline(always)]
    pub const fn denali_phy_70(&self) -> &DenaliPhy70 {
        &self.denali_phy_70
    }
    #[doc = "0x211c - "]
    #[inline(always)]
    pub const fn denali_phy_71(&self) -> &DenaliPhy71 {
        &self.denali_phy_71
    }
    #[doc = "0x2120 - "]
    #[inline(always)]
    pub const fn denali_phy_72(&self) -> &DenaliPhy72 {
        &self.denali_phy_72
    }
    #[doc = "0x2124 - "]
    #[inline(always)]
    pub const fn denali_phy_73(&self) -> &DenaliPhy73 {
        &self.denali_phy_73
    }
    #[doc = "0x2128 - "]
    #[inline(always)]
    pub const fn denali_phy_74(&self) -> &DenaliPhy74 {
        &self.denali_phy_74
    }
    #[doc = "0x212c - "]
    #[inline(always)]
    pub const fn denali_phy_75(&self) -> &DenaliPhy75 {
        &self.denali_phy_75
    }
    #[doc = "0x2130 - "]
    #[inline(always)]
    pub const fn denali_phy_76(&self) -> &DenaliPhy76 {
        &self.denali_phy_76
    }
    #[doc = "0x2134 - "]
    #[inline(always)]
    pub const fn denali_phy_77(&self) -> &DenaliPhy77 {
        &self.denali_phy_77
    }
    #[doc = "0x2138 - "]
    #[inline(always)]
    pub const fn denali_phy_78(&self) -> &DenaliPhy78 {
        &self.denali_phy_78
    }
    #[doc = "0x213c - "]
    #[inline(always)]
    pub const fn denali_phy_79(&self) -> &DenaliPhy79 {
        &self.denali_phy_79
    }
    #[doc = "0x2140 - "]
    #[inline(always)]
    pub const fn denali_phy_80(&self) -> &DenaliPhy80 {
        &self.denali_phy_80
    }
    #[doc = "0x2144 - "]
    #[inline(always)]
    pub const fn denali_phy_81(&self) -> &DenaliPhy81 {
        &self.denali_phy_81
    }
    #[doc = "0x214c - "]
    #[inline(always)]
    pub const fn denali_phy_83(&self) -> &DenaliPhy83 {
        &self.denali_phy_83
    }
    #[doc = "0x2150 - "]
    #[inline(always)]
    pub const fn denali_phy_84(&self) -> &DenaliPhy84 {
        &self.denali_phy_84
    }
    #[doc = "0x2154 - "]
    #[inline(always)]
    pub const fn denali_phy_85(&self) -> &DenaliPhy85 {
        &self.denali_phy_85
    }
    #[doc = "0x2158 - "]
    #[inline(always)]
    pub const fn denali_phy_86(&self) -> &DenaliPhy86 {
        &self.denali_phy_86
    }
    #[doc = "0x215c - "]
    #[inline(always)]
    pub const fn denali_phy_87(&self) -> &DenaliPhy87 {
        &self.denali_phy_87
    }
    #[doc = "0x2160 - "]
    #[inline(always)]
    pub const fn denali_phy_88(&self) -> &DenaliPhy88 {
        &self.denali_phy_88
    }
    #[doc = "0x2164 - "]
    #[inline(always)]
    pub const fn denali_phy_89(&self) -> &DenaliPhy89 {
        &self.denali_phy_89
    }
    #[doc = "0x2168 - "]
    #[inline(always)]
    pub const fn denali_phy_90(&self) -> &DenaliPhy90 {
        &self.denali_phy_90
    }
    #[doc = "0x2200 - "]
    #[inline(always)]
    pub const fn denali_phy_128(&self) -> &DenaliPhy128 {
        &self.denali_phy_128
    }
    #[doc = "0x2204 - "]
    #[inline(always)]
    pub const fn denali_phy_129(&self) -> &DenaliPhy129 {
        &self.denali_phy_129
    }
    #[doc = "0x2208 - "]
    #[inline(always)]
    pub const fn denali_phy_130(&self) -> &DenaliPhy130 {
        &self.denali_phy_130
    }
    #[doc = "0x220c - "]
    #[inline(always)]
    pub const fn denali_phy_131(&self) -> &DenaliPhy131 {
        &self.denali_phy_131
    }
    #[doc = "0x2210 - "]
    #[inline(always)]
    pub const fn denali_phy_132(&self) -> &DenaliPhy132 {
        &self.denali_phy_132
    }
    #[doc = "0x2214 - "]
    #[inline(always)]
    pub const fn denali_phy_133(&self) -> &DenaliPhy133 {
        &self.denali_phy_133
    }
    #[doc = "0x2218 - "]
    #[inline(always)]
    pub const fn denali_phy_134(&self) -> &DenaliPhy134 {
        &self.denali_phy_134
    }
    #[doc = "0x221c - "]
    #[inline(always)]
    pub const fn denali_phy_135(&self) -> &DenaliPhy135 {
        &self.denali_phy_135
    }
    #[doc = "0x2220 - "]
    #[inline(always)]
    pub const fn denali_phy_136(&self) -> &DenaliPhy136 {
        &self.denali_phy_136
    }
    #[doc = "0x2224 - "]
    #[inline(always)]
    pub const fn denali_phy_137(&self) -> &DenaliPhy137 {
        &self.denali_phy_137
    }
    #[doc = "0x2228 - "]
    #[inline(always)]
    pub const fn denali_phy_138(&self) -> &DenaliPhy138 {
        &self.denali_phy_138
    }
    #[doc = "0x222c - "]
    #[inline(always)]
    pub const fn denali_phy_139(&self) -> &DenaliPhy139 {
        &self.denali_phy_139
    }
    #[doc = "0x2230 - "]
    #[inline(always)]
    pub const fn denali_phy_140(&self) -> &DenaliPhy140 {
        &self.denali_phy_140
    }
    #[doc = "0x2234 - "]
    #[inline(always)]
    pub const fn denali_phy_141(&self) -> &DenaliPhy141 {
        &self.denali_phy_141
    }
    #[doc = "0x2238 - "]
    #[inline(always)]
    pub const fn denali_phy_142(&self) -> &DenaliPhy142 {
        &self.denali_phy_142
    }
    #[doc = "0x223c - "]
    #[inline(always)]
    pub const fn denali_phy_143(&self) -> &DenaliPhy143 {
        &self.denali_phy_143
    }
    #[doc = "0x2240 - "]
    #[inline(always)]
    pub const fn denali_phy_144(&self) -> &DenaliPhy144 {
        &self.denali_phy_144
    }
    #[doc = "0x2244 - "]
    #[inline(always)]
    pub const fn denali_phy_145(&self) -> &DenaliPhy145 {
        &self.denali_phy_145
    }
    #[doc = "0x2248 - "]
    #[inline(always)]
    pub const fn denali_phy_146(&self) -> &DenaliPhy146 {
        &self.denali_phy_146
    }
    #[doc = "0x224c - "]
    #[inline(always)]
    pub const fn denali_phy_147(&self) -> &DenaliPhy147 {
        &self.denali_phy_147
    }
    #[doc = "0x2250 - "]
    #[inline(always)]
    pub const fn denali_phy_148(&self) -> &DenaliPhy148 {
        &self.denali_phy_148
    }
    #[doc = "0x2254 - "]
    #[inline(always)]
    pub const fn denali_phy_149(&self) -> &DenaliPhy149 {
        &self.denali_phy_149
    }
    #[doc = "0x2258 - "]
    #[inline(always)]
    pub const fn denali_phy_150(&self) -> &DenaliPhy150 {
        &self.denali_phy_150
    }
    #[doc = "0x225c - "]
    #[inline(always)]
    pub const fn denali_phy_151(&self) -> &DenaliPhy151 {
        &self.denali_phy_151
    }
    #[doc = "0x2260 - "]
    #[inline(always)]
    pub const fn denali_phy_152(&self) -> &DenaliPhy152 {
        &self.denali_phy_152
    }
    #[doc = "0x2264 - "]
    #[inline(always)]
    pub const fn denali_phy_153(&self) -> &DenaliPhy153 {
        &self.denali_phy_153
    }
    #[doc = "0x2268 - "]
    #[inline(always)]
    pub const fn denali_phy_154(&self) -> &DenaliPhy154 {
        &self.denali_phy_154
    }
    #[doc = "0x226c - "]
    #[inline(always)]
    pub const fn denali_phy_155(&self) -> &DenaliPhy155 {
        &self.denali_phy_155
    }
    #[doc = "0x2270 - "]
    #[inline(always)]
    pub const fn denali_phy_156(&self) -> &DenaliPhy156 {
        &self.denali_phy_156
    }
    #[doc = "0x2274 - "]
    #[inline(always)]
    pub const fn denali_phy_157(&self) -> &DenaliPhy157 {
        &self.denali_phy_157
    }
    #[doc = "0x2278 - "]
    #[inline(always)]
    pub const fn denali_phy_158(&self) -> &DenaliPhy158 {
        &self.denali_phy_158
    }
    #[doc = "0x227c - "]
    #[inline(always)]
    pub const fn denali_phy_159(&self) -> &DenaliPhy159 {
        &self.denali_phy_159
    }
    #[doc = "0x2280 - "]
    #[inline(always)]
    pub const fn denali_phy_160(&self) -> &DenaliPhy160 {
        &self.denali_phy_160
    }
    #[doc = "0x2284 - "]
    #[inline(always)]
    pub const fn denali_phy_161(&self) -> &DenaliPhy161 {
        &self.denali_phy_161
    }
    #[doc = "0x2288 - "]
    #[inline(always)]
    pub const fn denali_phy_162(&self) -> &DenaliPhy162 {
        &self.denali_phy_162
    }
    #[doc = "0x228c - "]
    #[inline(always)]
    pub const fn denali_phy_163(&self) -> &DenaliPhy163 {
        &self.denali_phy_163
    }
    #[doc = "0x2290 - "]
    #[inline(always)]
    pub const fn denali_phy_164(&self) -> &DenaliPhy164 {
        &self.denali_phy_164
    }
    #[doc = "0x2294 - "]
    #[inline(always)]
    pub const fn denali_phy_165(&self) -> &DenaliPhy165 {
        &self.denali_phy_165
    }
    #[doc = "0x2298 - "]
    #[inline(always)]
    pub const fn denali_phy_166(&self) -> &DenaliPhy166 {
        &self.denali_phy_166
    }
    #[doc = "0x229c - "]
    #[inline(always)]
    pub const fn denali_phy_167(&self) -> &DenaliPhy167 {
        &self.denali_phy_167
    }
    #[doc = "0x22a0 - "]
    #[inline(always)]
    pub const fn denali_phy_168(&self) -> &DenaliPhy168 {
        &self.denali_phy_168
    }
    #[doc = "0x22a4 - "]
    #[inline(always)]
    pub const fn denali_phy_169(&self) -> &DenaliPhy169 {
        &self.denali_phy_169
    }
    #[doc = "0x22a8 - "]
    #[inline(always)]
    pub const fn denali_phy_170(&self) -> &DenaliPhy170 {
        &self.denali_phy_170
    }
    #[doc = "0x22ac - "]
    #[inline(always)]
    pub const fn denali_phy_171(&self) -> &DenaliPhy171 {
        &self.denali_phy_171
    }
    #[doc = "0x22b0 - "]
    #[inline(always)]
    pub const fn denali_phy_172(&self) -> &DenaliPhy172 {
        &self.denali_phy_172
    }
    #[doc = "0x22b4 - "]
    #[inline(always)]
    pub const fn denali_phy_173(&self) -> &DenaliPhy173 {
        &self.denali_phy_173
    }
    #[doc = "0x22b8 - "]
    #[inline(always)]
    pub const fn denali_phy_174(&self) -> &DenaliPhy174 {
        &self.denali_phy_174
    }
    #[doc = "0x22bc - "]
    #[inline(always)]
    pub const fn denali_phy_175(&self) -> &DenaliPhy175 {
        &self.denali_phy_175
    }
    #[doc = "0x22c0 - "]
    #[inline(always)]
    pub const fn denali_phy_176(&self) -> &DenaliPhy176 {
        &self.denali_phy_176
    }
    #[doc = "0x22c4 - "]
    #[inline(always)]
    pub const fn denali_phy_177(&self) -> &DenaliPhy177 {
        &self.denali_phy_177
    }
    #[doc = "0x22c8 - "]
    #[inline(always)]
    pub const fn denali_phy_178(&self) -> &DenaliPhy178 {
        &self.denali_phy_178
    }
    #[doc = "0x22cc - "]
    #[inline(always)]
    pub const fn denali_phy_179(&self) -> &DenaliPhy179 {
        &self.denali_phy_179
    }
    #[doc = "0x22d0 - "]
    #[inline(always)]
    pub const fn denali_phy_180(&self) -> &DenaliPhy180 {
        &self.denali_phy_180
    }
    #[doc = "0x22d4 - "]
    #[inline(always)]
    pub const fn denali_phy_181(&self) -> &DenaliPhy181 {
        &self.denali_phy_181
    }
    #[doc = "0x22d8 - "]
    #[inline(always)]
    pub const fn denali_phy_182(&self) -> &DenaliPhy182 {
        &self.denali_phy_182
    }
    #[doc = "0x22dc - "]
    #[inline(always)]
    pub const fn denali_phy_183(&self) -> &DenaliPhy183 {
        &self.denali_phy_183
    }
    #[doc = "0x22e0 - "]
    #[inline(always)]
    pub const fn denali_phy_184(&self) -> &DenaliPhy184 {
        &self.denali_phy_184
    }
    #[doc = "0x22e4 - "]
    #[inline(always)]
    pub const fn denali_phy_185(&self) -> &DenaliPhy185 {
        &self.denali_phy_185
    }
    #[doc = "0x22e8 - "]
    #[inline(always)]
    pub const fn denali_phy_186(&self) -> &DenaliPhy186 {
        &self.denali_phy_186
    }
    #[doc = "0x22ec - "]
    #[inline(always)]
    pub const fn denali_phy_187(&self) -> &DenaliPhy187 {
        &self.denali_phy_187
    }
    #[doc = "0x22f0 - "]
    #[inline(always)]
    pub const fn denali_phy_188(&self) -> &DenaliPhy188 {
        &self.denali_phy_188
    }
    #[doc = "0x22f4 - "]
    #[inline(always)]
    pub const fn denali_phy_189(&self) -> &DenaliPhy189 {
        &self.denali_phy_189
    }
    #[doc = "0x22f8 - "]
    #[inline(always)]
    pub const fn denali_phy_190(&self) -> &DenaliPhy190 {
        &self.denali_phy_190
    }
    #[doc = "0x22fc - "]
    #[inline(always)]
    pub const fn denali_phy_191(&self) -> &DenaliPhy191 {
        &self.denali_phy_191
    }
    #[doc = "0x2300 - "]
    #[inline(always)]
    pub const fn denali_phy_192(&self) -> &DenaliPhy192 {
        &self.denali_phy_192
    }
    #[doc = "0x2304 - "]
    #[inline(always)]
    pub const fn denali_phy_193(&self) -> &DenaliPhy193 {
        &self.denali_phy_193
    }
    #[doc = "0x2308 - "]
    #[inline(always)]
    pub const fn denali_phy_194(&self) -> &DenaliPhy194 {
        &self.denali_phy_194
    }
    #[doc = "0x230c - "]
    #[inline(always)]
    pub const fn denali_phy_195(&self) -> &DenaliPhy195 {
        &self.denali_phy_195
    }
    #[doc = "0x2310 - "]
    #[inline(always)]
    pub const fn denali_phy_196(&self) -> &DenaliPhy196 {
        &self.denali_phy_196
    }
    #[doc = "0x2314 - "]
    #[inline(always)]
    pub const fn denali_phy_197(&self) -> &DenaliPhy197 {
        &self.denali_phy_197
    }
    #[doc = "0x2318 - "]
    #[inline(always)]
    pub const fn denali_phy_198(&self) -> &DenaliPhy198 {
        &self.denali_phy_198
    }
    #[doc = "0x231c - "]
    #[inline(always)]
    pub const fn denali_phy_199(&self) -> &DenaliPhy199 {
        &self.denali_phy_199
    }
    #[doc = "0x2320 - "]
    #[inline(always)]
    pub const fn denali_phy_200(&self) -> &DenaliPhy200 {
        &self.denali_phy_200
    }
    #[doc = "0x2324 - "]
    #[inline(always)]
    pub const fn denali_phy_201(&self) -> &DenaliPhy201 {
        &self.denali_phy_201
    }
    #[doc = "0x2328 - "]
    #[inline(always)]
    pub const fn denali_phy_202(&self) -> &DenaliPhy202 {
        &self.denali_phy_202
    }
    #[doc = "0x232c - "]
    #[inline(always)]
    pub const fn denali_phy_203(&self) -> &DenaliPhy203 {
        &self.denali_phy_203
    }
    #[doc = "0x2330 - "]
    #[inline(always)]
    pub const fn denali_phy_204(&self) -> &DenaliPhy204 {
        &self.denali_phy_204
    }
    #[doc = "0x2334 - "]
    #[inline(always)]
    pub const fn denali_phy_205(&self) -> &DenaliPhy205 {
        &self.denali_phy_205
    }
    #[doc = "0x2338 - "]
    #[inline(always)]
    pub const fn denali_phy_206(&self) -> &DenaliPhy206 {
        &self.denali_phy_206
    }
    #[doc = "0x233c - "]
    #[inline(always)]
    pub const fn denali_phy_207(&self) -> &DenaliPhy207 {
        &self.denali_phy_207
    }
    #[doc = "0x2340 - "]
    #[inline(always)]
    pub const fn denali_phy_208(&self) -> &DenaliPhy208 {
        &self.denali_phy_208
    }
    #[doc = "0x2344 - "]
    #[inline(always)]
    pub const fn denali_phy_209(&self) -> &DenaliPhy209 {
        &self.denali_phy_209
    }
    #[doc = "0x234c - "]
    #[inline(always)]
    pub const fn denali_phy_211(&self) -> &DenaliPhy211 {
        &self.denali_phy_211
    }
    #[doc = "0x2350 - "]
    #[inline(always)]
    pub const fn denali_phy_212(&self) -> &DenaliPhy212 {
        &self.denali_phy_212
    }
    #[doc = "0x2354 - "]
    #[inline(always)]
    pub const fn denali_phy_213(&self) -> &DenaliPhy213 {
        &self.denali_phy_213
    }
    #[doc = "0x2358 - "]
    #[inline(always)]
    pub const fn denali_phy_214(&self) -> &DenaliPhy214 {
        &self.denali_phy_214
    }
    #[doc = "0x235c - "]
    #[inline(always)]
    pub const fn denali_phy_215(&self) -> &DenaliPhy215 {
        &self.denali_phy_215
    }
    #[doc = "0x2360 - "]
    #[inline(always)]
    pub const fn denali_phy_216(&self) -> &DenaliPhy216 {
        &self.denali_phy_216
    }
    #[doc = "0x2364 - "]
    #[inline(always)]
    pub const fn denali_phy_217(&self) -> &DenaliPhy217 {
        &self.denali_phy_217
    }
    #[doc = "0x2368 - "]
    #[inline(always)]
    pub const fn denali_phy_218(&self) -> &DenaliPhy218 {
        &self.denali_phy_218
    }
    #[doc = "0x2400 - "]
    #[inline(always)]
    pub const fn denali_phy_256(&self) -> &DenaliPhy256 {
        &self.denali_phy_256
    }
    #[doc = "0x2404 - "]
    #[inline(always)]
    pub const fn denali_phy_257(&self) -> &DenaliPhy257 {
        &self.denali_phy_257
    }
    #[doc = "0x2408 - "]
    #[inline(always)]
    pub const fn denali_phy_258(&self) -> &DenaliPhy258 {
        &self.denali_phy_258
    }
    #[doc = "0x240c - "]
    #[inline(always)]
    pub const fn denali_phy_259(&self) -> &DenaliPhy259 {
        &self.denali_phy_259
    }
    #[doc = "0x2410 - "]
    #[inline(always)]
    pub const fn denali_phy_260(&self) -> &DenaliPhy260 {
        &self.denali_phy_260
    }
    #[doc = "0x2414 - "]
    #[inline(always)]
    pub const fn denali_phy_261(&self) -> &DenaliPhy261 {
        &self.denali_phy_261
    }
    #[doc = "0x2418 - "]
    #[inline(always)]
    pub const fn denali_phy_262(&self) -> &DenaliPhy262 {
        &self.denali_phy_262
    }
    #[doc = "0x241c - "]
    #[inline(always)]
    pub const fn denali_phy_263(&self) -> &DenaliPhy263 {
        &self.denali_phy_263
    }
    #[doc = "0x2420 - "]
    #[inline(always)]
    pub const fn denali_phy_264(&self) -> &DenaliPhy264 {
        &self.denali_phy_264
    }
    #[doc = "0x2424 - "]
    #[inline(always)]
    pub const fn denali_phy_265(&self) -> &DenaliPhy265 {
        &self.denali_phy_265
    }
    #[doc = "0x2428 - "]
    #[inline(always)]
    pub const fn denali_phy_266(&self) -> &DenaliPhy266 {
        &self.denali_phy_266
    }
    #[doc = "0x242c - "]
    #[inline(always)]
    pub const fn denali_phy_267(&self) -> &DenaliPhy267 {
        &self.denali_phy_267
    }
    #[doc = "0x2430 - "]
    #[inline(always)]
    pub const fn denali_phy_268(&self) -> &DenaliPhy268 {
        &self.denali_phy_268
    }
    #[doc = "0x2434 - "]
    #[inline(always)]
    pub const fn denali_phy_269(&self) -> &DenaliPhy269 {
        &self.denali_phy_269
    }
    #[doc = "0x2438 - "]
    #[inline(always)]
    pub const fn denali_phy_270(&self) -> &DenaliPhy270 {
        &self.denali_phy_270
    }
    #[doc = "0x243c - "]
    #[inline(always)]
    pub const fn denali_phy_271(&self) -> &DenaliPhy271 {
        &self.denali_phy_271
    }
    #[doc = "0x2440 - "]
    #[inline(always)]
    pub const fn denali_phy_272(&self) -> &DenaliPhy272 {
        &self.denali_phy_272
    }
    #[doc = "0x2444 - "]
    #[inline(always)]
    pub const fn denali_phy_273(&self) -> &DenaliPhy273 {
        &self.denali_phy_273
    }
    #[doc = "0x2448 - "]
    #[inline(always)]
    pub const fn denali_phy_274(&self) -> &DenaliPhy274 {
        &self.denali_phy_274
    }
    #[doc = "0x244c - "]
    #[inline(always)]
    pub const fn denali_phy_275(&self) -> &DenaliPhy275 {
        &self.denali_phy_275
    }
    #[doc = "0x2450 - "]
    #[inline(always)]
    pub const fn denali_phy_276(&self) -> &DenaliPhy276 {
        &self.denali_phy_276
    }
    #[doc = "0x2454 - "]
    #[inline(always)]
    pub const fn denali_phy_277(&self) -> &DenaliPhy277 {
        &self.denali_phy_277
    }
    #[doc = "0x2458 - "]
    #[inline(always)]
    pub const fn denali_phy_278(&self) -> &DenaliPhy278 {
        &self.denali_phy_278
    }
    #[doc = "0x245c - "]
    #[inline(always)]
    pub const fn denali_phy_279(&self) -> &DenaliPhy279 {
        &self.denali_phy_279
    }
    #[doc = "0x2460 - "]
    #[inline(always)]
    pub const fn denali_phy_280(&self) -> &DenaliPhy280 {
        &self.denali_phy_280
    }
    #[doc = "0x2464 - "]
    #[inline(always)]
    pub const fn denali_phy_281(&self) -> &DenaliPhy281 {
        &self.denali_phy_281
    }
    #[doc = "0x2468 - "]
    #[inline(always)]
    pub const fn denali_phy_282(&self) -> &DenaliPhy282 {
        &self.denali_phy_282
    }
    #[doc = "0x246c - "]
    #[inline(always)]
    pub const fn denali_phy_283(&self) -> &DenaliPhy283 {
        &self.denali_phy_283
    }
    #[doc = "0x2470 - "]
    #[inline(always)]
    pub const fn denali_phy_284(&self) -> &DenaliPhy284 {
        &self.denali_phy_284
    }
    #[doc = "0x2474 - "]
    #[inline(always)]
    pub const fn denali_phy_285(&self) -> &DenaliPhy285 {
        &self.denali_phy_285
    }
    #[doc = "0x2478 - "]
    #[inline(always)]
    pub const fn denali_phy_286(&self) -> &DenaliPhy286 {
        &self.denali_phy_286
    }
    #[doc = "0x247c - "]
    #[inline(always)]
    pub const fn denali_phy_287(&self) -> &DenaliPhy287 {
        &self.denali_phy_287
    }
    #[doc = "0x2480 - "]
    #[inline(always)]
    pub const fn denali_phy_288(&self) -> &DenaliPhy288 {
        &self.denali_phy_288
    }
    #[doc = "0x2484 - "]
    #[inline(always)]
    pub const fn denali_phy_289(&self) -> &DenaliPhy289 {
        &self.denali_phy_289
    }
    #[doc = "0x2488 - "]
    #[inline(always)]
    pub const fn denali_phy_290(&self) -> &DenaliPhy290 {
        &self.denali_phy_290
    }
    #[doc = "0x248c - "]
    #[inline(always)]
    pub const fn denali_phy_291(&self) -> &DenaliPhy291 {
        &self.denali_phy_291
    }
    #[doc = "0x2490 - "]
    #[inline(always)]
    pub const fn denali_phy_292(&self) -> &DenaliPhy292 {
        &self.denali_phy_292
    }
    #[doc = "0x2494 - "]
    #[inline(always)]
    pub const fn denali_phy_293(&self) -> &DenaliPhy293 {
        &self.denali_phy_293
    }
    #[doc = "0x2498 - "]
    #[inline(always)]
    pub const fn denali_phy_294(&self) -> &DenaliPhy294 {
        &self.denali_phy_294
    }
    #[doc = "0x249c - "]
    #[inline(always)]
    pub const fn denali_phy_295(&self) -> &DenaliPhy295 {
        &self.denali_phy_295
    }
    #[doc = "0x24a0 - "]
    #[inline(always)]
    pub const fn denali_phy_296(&self) -> &DenaliPhy296 {
        &self.denali_phy_296
    }
    #[doc = "0x24a4 - "]
    #[inline(always)]
    pub const fn denali_phy_297(&self) -> &DenaliPhy297 {
        &self.denali_phy_297
    }
    #[doc = "0x24a8 - "]
    #[inline(always)]
    pub const fn denali_phy_298(&self) -> &DenaliPhy298 {
        &self.denali_phy_298
    }
    #[doc = "0x24ac - "]
    #[inline(always)]
    pub const fn denali_phy_299(&self) -> &DenaliPhy299 {
        &self.denali_phy_299
    }
    #[doc = "0x24b0 - "]
    #[inline(always)]
    pub const fn denali_phy_300(&self) -> &DenaliPhy300 {
        &self.denali_phy_300
    }
    #[doc = "0x24b4 - "]
    #[inline(always)]
    pub const fn denali_phy_301(&self) -> &DenaliPhy301 {
        &self.denali_phy_301
    }
    #[doc = "0x24b8 - "]
    #[inline(always)]
    pub const fn denali_phy_302(&self) -> &DenaliPhy302 {
        &self.denali_phy_302
    }
    #[doc = "0x24bc - "]
    #[inline(always)]
    pub const fn denali_phy_303(&self) -> &DenaliPhy303 {
        &self.denali_phy_303
    }
    #[doc = "0x24c0 - "]
    #[inline(always)]
    pub const fn denali_phy_304(&self) -> &DenaliPhy304 {
        &self.denali_phy_304
    }
    #[doc = "0x24c4 - "]
    #[inline(always)]
    pub const fn denali_phy_305(&self) -> &DenaliPhy305 {
        &self.denali_phy_305
    }
    #[doc = "0x24c8 - "]
    #[inline(always)]
    pub const fn denali_phy_306(&self) -> &DenaliPhy306 {
        &self.denali_phy_306
    }
    #[doc = "0x24cc - "]
    #[inline(always)]
    pub const fn denali_phy_307(&self) -> &DenaliPhy307 {
        &self.denali_phy_307
    }
    #[doc = "0x24d0 - "]
    #[inline(always)]
    pub const fn denali_phy_308(&self) -> &DenaliPhy308 {
        &self.denali_phy_308
    }
    #[doc = "0x24d4 - "]
    #[inline(always)]
    pub const fn denali_phy_309(&self) -> &DenaliPhy309 {
        &self.denali_phy_309
    }
    #[doc = "0x24d8 - "]
    #[inline(always)]
    pub const fn denali_phy_310(&self) -> &DenaliPhy310 {
        &self.denali_phy_310
    }
    #[doc = "0x24dc - "]
    #[inline(always)]
    pub const fn denali_phy_311(&self) -> &DenaliPhy311 {
        &self.denali_phy_311
    }
    #[doc = "0x24e0 - "]
    #[inline(always)]
    pub const fn denali_phy_312(&self) -> &DenaliPhy312 {
        &self.denali_phy_312
    }
    #[doc = "0x24e4 - "]
    #[inline(always)]
    pub const fn denali_phy_313(&self) -> &DenaliPhy313 {
        &self.denali_phy_313
    }
    #[doc = "0x24e8 - "]
    #[inline(always)]
    pub const fn denali_phy_314(&self) -> &DenaliPhy314 {
        &self.denali_phy_314
    }
    #[doc = "0x24ec - "]
    #[inline(always)]
    pub const fn denali_phy_315(&self) -> &DenaliPhy315 {
        &self.denali_phy_315
    }
    #[doc = "0x24f0 - "]
    #[inline(always)]
    pub const fn denali_phy_316(&self) -> &DenaliPhy316 {
        &self.denali_phy_316
    }
    #[doc = "0x24f4 - "]
    #[inline(always)]
    pub const fn denali_phy_317(&self) -> &DenaliPhy317 {
        &self.denali_phy_317
    }
    #[doc = "0x24f8 - "]
    #[inline(always)]
    pub const fn denali_phy_318(&self) -> &DenaliPhy318 {
        &self.denali_phy_318
    }
    #[doc = "0x24fc - "]
    #[inline(always)]
    pub const fn denali_phy_319(&self) -> &DenaliPhy319 {
        &self.denali_phy_319
    }
    #[doc = "0x2500 - "]
    #[inline(always)]
    pub const fn denali_phy_320(&self) -> &DenaliPhy320 {
        &self.denali_phy_320
    }
    #[doc = "0x2504 - "]
    #[inline(always)]
    pub const fn denali_phy_321(&self) -> &DenaliPhy321 {
        &self.denali_phy_321
    }
    #[doc = "0x2508 - "]
    #[inline(always)]
    pub const fn denali_phy_322(&self) -> &DenaliPhy322 {
        &self.denali_phy_322
    }
    #[doc = "0x250c - "]
    #[inline(always)]
    pub const fn denali_phy_323(&self) -> &DenaliPhy323 {
        &self.denali_phy_323
    }
    #[doc = "0x2510 - "]
    #[inline(always)]
    pub const fn denali_phy_324(&self) -> &DenaliPhy324 {
        &self.denali_phy_324
    }
    #[doc = "0x2514 - "]
    #[inline(always)]
    pub const fn denali_phy_325(&self) -> &DenaliPhy325 {
        &self.denali_phy_325
    }
    #[doc = "0x2518 - "]
    #[inline(always)]
    pub const fn denali_phy_326(&self) -> &DenaliPhy326 {
        &self.denali_phy_326
    }
    #[doc = "0x251c - "]
    #[inline(always)]
    pub const fn denali_phy_327(&self) -> &DenaliPhy327 {
        &self.denali_phy_327
    }
    #[doc = "0x2520 - "]
    #[inline(always)]
    pub const fn denali_phy_328(&self) -> &DenaliPhy328 {
        &self.denali_phy_328
    }
    #[doc = "0x2524 - "]
    #[inline(always)]
    pub const fn denali_phy_329(&self) -> &DenaliPhy329 {
        &self.denali_phy_329
    }
    #[doc = "0x2528 - "]
    #[inline(always)]
    pub const fn denali_phy_330(&self) -> &DenaliPhy330 {
        &self.denali_phy_330
    }
    #[doc = "0x252c - "]
    #[inline(always)]
    pub const fn denali_phy_331(&self) -> &DenaliPhy331 {
        &self.denali_phy_331
    }
    #[doc = "0x2530 - "]
    #[inline(always)]
    pub const fn denali_phy_332(&self) -> &DenaliPhy332 {
        &self.denali_phy_332
    }
    #[doc = "0x2534 - "]
    #[inline(always)]
    pub const fn denali_phy_333(&self) -> &DenaliPhy333 {
        &self.denali_phy_333
    }
    #[doc = "0x2538 - "]
    #[inline(always)]
    pub const fn denali_phy_334(&self) -> &DenaliPhy334 {
        &self.denali_phy_334
    }
    #[doc = "0x253c - "]
    #[inline(always)]
    pub const fn denali_phy_335(&self) -> &DenaliPhy335 {
        &self.denali_phy_335
    }
    #[doc = "0x2540 - "]
    #[inline(always)]
    pub const fn denali_phy_336(&self) -> &DenaliPhy336 {
        &self.denali_phy_336
    }
    #[doc = "0x2544 - "]
    #[inline(always)]
    pub const fn denali_phy_337(&self) -> &DenaliPhy337 {
        &self.denali_phy_337
    }
    #[doc = "0x254c - "]
    #[inline(always)]
    pub const fn denali_phy_339(&self) -> &DenaliPhy339 {
        &self.denali_phy_339
    }
    #[doc = "0x2550 - "]
    #[inline(always)]
    pub const fn denali_phy_340(&self) -> &DenaliPhy340 {
        &self.denali_phy_340
    }
    #[doc = "0x2554 - "]
    #[inline(always)]
    pub const fn denali_phy_341(&self) -> &DenaliPhy341 {
        &self.denali_phy_341
    }
    #[doc = "0x2558 - "]
    #[inline(always)]
    pub const fn denali_phy_342(&self) -> &DenaliPhy342 {
        &self.denali_phy_342
    }
    #[doc = "0x255c - "]
    #[inline(always)]
    pub const fn denali_phy_343(&self) -> &DenaliPhy343 {
        &self.denali_phy_343
    }
    #[doc = "0x2560 - "]
    #[inline(always)]
    pub const fn denali_phy_344(&self) -> &DenaliPhy344 {
        &self.denali_phy_344
    }
    #[doc = "0x2564 - "]
    #[inline(always)]
    pub const fn denali_phy_345(&self) -> &DenaliPhy345 {
        &self.denali_phy_345
    }
    #[doc = "0x2568 - "]
    #[inline(always)]
    pub const fn denali_phy_346(&self) -> &DenaliPhy346 {
        &self.denali_phy_346
    }
    #[doc = "0x2600 - "]
    #[inline(always)]
    pub const fn denali_phy_384(&self) -> &DenaliPhy384 {
        &self.denali_phy_384
    }
    #[doc = "0x2604 - "]
    #[inline(always)]
    pub const fn denali_phy_385(&self) -> &DenaliPhy385 {
        &self.denali_phy_385
    }
    #[doc = "0x2608 - "]
    #[inline(always)]
    pub const fn denali_phy_386(&self) -> &DenaliPhy386 {
        &self.denali_phy_386
    }
    #[doc = "0x260c - "]
    #[inline(always)]
    pub const fn denali_phy_387(&self) -> &DenaliPhy387 {
        &self.denali_phy_387
    }
    #[doc = "0x2610 - "]
    #[inline(always)]
    pub const fn denali_phy_388(&self) -> &DenaliPhy388 {
        &self.denali_phy_388
    }
    #[doc = "0x2614 - "]
    #[inline(always)]
    pub const fn denali_phy_389(&self) -> &DenaliPhy389 {
        &self.denali_phy_389
    }
    #[doc = "0x2618 - "]
    #[inline(always)]
    pub const fn denali_phy_390(&self) -> &DenaliPhy390 {
        &self.denali_phy_390
    }
    #[doc = "0x261c - "]
    #[inline(always)]
    pub const fn denali_phy_391(&self) -> &DenaliPhy391 {
        &self.denali_phy_391
    }
    #[doc = "0x2620 - "]
    #[inline(always)]
    pub const fn denali_phy_392(&self) -> &DenaliPhy392 {
        &self.denali_phy_392
    }
    #[doc = "0x2624 - "]
    #[inline(always)]
    pub const fn denali_phy_393(&self) -> &DenaliPhy393 {
        &self.denali_phy_393
    }
    #[doc = "0x2628 - "]
    #[inline(always)]
    pub const fn denali_phy_394(&self) -> &DenaliPhy394 {
        &self.denali_phy_394
    }
    #[doc = "0x262c - "]
    #[inline(always)]
    pub const fn denali_phy_395(&self) -> &DenaliPhy395 {
        &self.denali_phy_395
    }
    #[doc = "0x2630 - "]
    #[inline(always)]
    pub const fn denali_phy_396(&self) -> &DenaliPhy396 {
        &self.denali_phy_396
    }
    #[doc = "0x2634 - "]
    #[inline(always)]
    pub const fn denali_phy_397(&self) -> &DenaliPhy397 {
        &self.denali_phy_397
    }
    #[doc = "0x2638 - "]
    #[inline(always)]
    pub const fn denali_phy_398(&self) -> &DenaliPhy398 {
        &self.denali_phy_398
    }
    #[doc = "0x263c - "]
    #[inline(always)]
    pub const fn denali_phy_399(&self) -> &DenaliPhy399 {
        &self.denali_phy_399
    }
    #[doc = "0x2640 - "]
    #[inline(always)]
    pub const fn denali_phy_400(&self) -> &DenaliPhy400 {
        &self.denali_phy_400
    }
    #[doc = "0x2644 - "]
    #[inline(always)]
    pub const fn denali_phy_401(&self) -> &DenaliPhy401 {
        &self.denali_phy_401
    }
    #[doc = "0x2648 - "]
    #[inline(always)]
    pub const fn denali_phy_402(&self) -> &DenaliPhy402 {
        &self.denali_phy_402
    }
    #[doc = "0x264c - "]
    #[inline(always)]
    pub const fn denali_phy_403(&self) -> &DenaliPhy403 {
        &self.denali_phy_403
    }
    #[doc = "0x2650 - "]
    #[inline(always)]
    pub const fn denali_phy_404(&self) -> &DenaliPhy404 {
        &self.denali_phy_404
    }
    #[doc = "0x2654 - "]
    #[inline(always)]
    pub const fn denali_phy_405(&self) -> &DenaliPhy405 {
        &self.denali_phy_405
    }
    #[doc = "0x2658 - "]
    #[inline(always)]
    pub const fn denali_phy_406(&self) -> &DenaliPhy406 {
        &self.denali_phy_406
    }
    #[doc = "0x265c - "]
    #[inline(always)]
    pub const fn denali_phy_407(&self) -> &DenaliPhy407 {
        &self.denali_phy_407
    }
    #[doc = "0x2660 - "]
    #[inline(always)]
    pub const fn denali_phy_408(&self) -> &DenaliPhy408 {
        &self.denali_phy_408
    }
    #[doc = "0x2664 - "]
    #[inline(always)]
    pub const fn denali_phy_409(&self) -> &DenaliPhy409 {
        &self.denali_phy_409
    }
    #[doc = "0x2668 - "]
    #[inline(always)]
    pub const fn denali_phy_410(&self) -> &DenaliPhy410 {
        &self.denali_phy_410
    }
    #[doc = "0x266c - "]
    #[inline(always)]
    pub const fn denali_phy_411(&self) -> &DenaliPhy411 {
        &self.denali_phy_411
    }
    #[doc = "0x2670 - "]
    #[inline(always)]
    pub const fn denali_phy_412(&self) -> &DenaliPhy412 {
        &self.denali_phy_412
    }
    #[doc = "0x2674 - "]
    #[inline(always)]
    pub const fn denali_phy_413(&self) -> &DenaliPhy413 {
        &self.denali_phy_413
    }
    #[doc = "0x2678 - "]
    #[inline(always)]
    pub const fn denali_phy_414(&self) -> &DenaliPhy414 {
        &self.denali_phy_414
    }
    #[doc = "0x267c - "]
    #[inline(always)]
    pub const fn denali_phy_415(&self) -> &DenaliPhy415 {
        &self.denali_phy_415
    }
    #[doc = "0x2680 - "]
    #[inline(always)]
    pub const fn denali_phy_416(&self) -> &DenaliPhy416 {
        &self.denali_phy_416
    }
    #[doc = "0x2684 - "]
    #[inline(always)]
    pub const fn denali_phy_417(&self) -> &DenaliPhy417 {
        &self.denali_phy_417
    }
    #[doc = "0x2688 - "]
    #[inline(always)]
    pub const fn denali_phy_418(&self) -> &DenaliPhy418 {
        &self.denali_phy_418
    }
    #[doc = "0x268c - "]
    #[inline(always)]
    pub const fn denali_phy_419(&self) -> &DenaliPhy419 {
        &self.denali_phy_419
    }
    #[doc = "0x2690 - "]
    #[inline(always)]
    pub const fn denali_phy_420(&self) -> &DenaliPhy420 {
        &self.denali_phy_420
    }
    #[doc = "0x2694 - "]
    #[inline(always)]
    pub const fn denali_phy_421(&self) -> &DenaliPhy421 {
        &self.denali_phy_421
    }
    #[doc = "0x2698 - "]
    #[inline(always)]
    pub const fn denali_phy_422(&self) -> &DenaliPhy422 {
        &self.denali_phy_422
    }
    #[doc = "0x269c - "]
    #[inline(always)]
    pub const fn denali_phy_423(&self) -> &DenaliPhy423 {
        &self.denali_phy_423
    }
    #[doc = "0x26a0 - "]
    #[inline(always)]
    pub const fn denali_phy_424(&self) -> &DenaliPhy424 {
        &self.denali_phy_424
    }
    #[doc = "0x26a4 - "]
    #[inline(always)]
    pub const fn denali_phy_425(&self) -> &DenaliPhy425 {
        &self.denali_phy_425
    }
    #[doc = "0x26a8 - "]
    #[inline(always)]
    pub const fn denali_phy_426(&self) -> &DenaliPhy426 {
        &self.denali_phy_426
    }
    #[doc = "0x26ac - "]
    #[inline(always)]
    pub const fn denali_phy_427(&self) -> &DenaliPhy427 {
        &self.denali_phy_427
    }
    #[doc = "0x26b0 - "]
    #[inline(always)]
    pub const fn denali_phy_428(&self) -> &DenaliPhy428 {
        &self.denali_phy_428
    }
    #[doc = "0x26b4 - "]
    #[inline(always)]
    pub const fn denali_phy_429(&self) -> &DenaliPhy429 {
        &self.denali_phy_429
    }
    #[doc = "0x26b8 - "]
    #[inline(always)]
    pub const fn denali_phy_430(&self) -> &DenaliPhy430 {
        &self.denali_phy_430
    }
    #[doc = "0x26bc - "]
    #[inline(always)]
    pub const fn denali_phy_431(&self) -> &DenaliPhy431 {
        &self.denali_phy_431
    }
    #[doc = "0x26c0 - "]
    #[inline(always)]
    pub const fn denali_phy_432(&self) -> &DenaliPhy432 {
        &self.denali_phy_432
    }
    #[doc = "0x26c4 - "]
    #[inline(always)]
    pub const fn denali_phy_433(&self) -> &DenaliPhy433 {
        &self.denali_phy_433
    }
    #[doc = "0x26c8 - "]
    #[inline(always)]
    pub const fn denali_phy_434(&self) -> &DenaliPhy434 {
        &self.denali_phy_434
    }
    #[doc = "0x26cc - "]
    #[inline(always)]
    pub const fn denali_phy_435(&self) -> &DenaliPhy435 {
        &self.denali_phy_435
    }
    #[doc = "0x26d0 - "]
    #[inline(always)]
    pub const fn denali_phy_436(&self) -> &DenaliPhy436 {
        &self.denali_phy_436
    }
    #[doc = "0x26d4 - "]
    #[inline(always)]
    pub const fn denali_phy_437(&self) -> &DenaliPhy437 {
        &self.denali_phy_437
    }
    #[doc = "0x26d8 - "]
    #[inline(always)]
    pub const fn denali_phy_438(&self) -> &DenaliPhy438 {
        &self.denali_phy_438
    }
    #[doc = "0x26dc - "]
    #[inline(always)]
    pub const fn denali_phy_439(&self) -> &DenaliPhy439 {
        &self.denali_phy_439
    }
    #[doc = "0x26e0 - "]
    #[inline(always)]
    pub const fn denali_phy_440(&self) -> &DenaliPhy440 {
        &self.denali_phy_440
    }
    #[doc = "0x26e4 - "]
    #[inline(always)]
    pub const fn denali_phy_441(&self) -> &DenaliPhy441 {
        &self.denali_phy_441
    }
    #[doc = "0x26e8 - "]
    #[inline(always)]
    pub const fn denali_phy_442(&self) -> &DenaliPhy442 {
        &self.denali_phy_442
    }
    #[doc = "0x26ec - "]
    #[inline(always)]
    pub const fn denali_phy_443(&self) -> &DenaliPhy443 {
        &self.denali_phy_443
    }
    #[doc = "0x26f0 - "]
    #[inline(always)]
    pub const fn denali_phy_444(&self) -> &DenaliPhy444 {
        &self.denali_phy_444
    }
    #[doc = "0x26f4 - "]
    #[inline(always)]
    pub const fn denali_phy_445(&self) -> &DenaliPhy445 {
        &self.denali_phy_445
    }
    #[doc = "0x26f8 - "]
    #[inline(always)]
    pub const fn denali_phy_446(&self) -> &DenaliPhy446 {
        &self.denali_phy_446
    }
    #[doc = "0x26fc - "]
    #[inline(always)]
    pub const fn denali_phy_447(&self) -> &DenaliPhy447 {
        &self.denali_phy_447
    }
    #[doc = "0x2700 - "]
    #[inline(always)]
    pub const fn denali_phy_448(&self) -> &DenaliPhy448 {
        &self.denali_phy_448
    }
    #[doc = "0x2704 - "]
    #[inline(always)]
    pub const fn denali_phy_449(&self) -> &DenaliPhy449 {
        &self.denali_phy_449
    }
    #[doc = "0x2708 - "]
    #[inline(always)]
    pub const fn denali_phy_450(&self) -> &DenaliPhy450 {
        &self.denali_phy_450
    }
    #[doc = "0x270c - "]
    #[inline(always)]
    pub const fn denali_phy_451(&self) -> &DenaliPhy451 {
        &self.denali_phy_451
    }
    #[doc = "0x2710 - "]
    #[inline(always)]
    pub const fn denali_phy_452(&self) -> &DenaliPhy452 {
        &self.denali_phy_452
    }
    #[doc = "0x2714 - "]
    #[inline(always)]
    pub const fn denali_phy_453(&self) -> &DenaliPhy453 {
        &self.denali_phy_453
    }
    #[doc = "0x2718 - "]
    #[inline(always)]
    pub const fn denali_phy_454(&self) -> &DenaliPhy454 {
        &self.denali_phy_454
    }
    #[doc = "0x271c - "]
    #[inline(always)]
    pub const fn denali_phy_455(&self) -> &DenaliPhy455 {
        &self.denali_phy_455
    }
    #[doc = "0x2720 - "]
    #[inline(always)]
    pub const fn denali_phy_456(&self) -> &DenaliPhy456 {
        &self.denali_phy_456
    }
    #[doc = "0x2724 - "]
    #[inline(always)]
    pub const fn denali_phy_457(&self) -> &DenaliPhy457 {
        &self.denali_phy_457
    }
    #[doc = "0x2728 - "]
    #[inline(always)]
    pub const fn denali_phy_458(&self) -> &DenaliPhy458 {
        &self.denali_phy_458
    }
    #[doc = "0x272c - "]
    #[inline(always)]
    pub const fn denali_phy_459(&self) -> &DenaliPhy459 {
        &self.denali_phy_459
    }
    #[doc = "0x2730 - "]
    #[inline(always)]
    pub const fn denali_phy_460(&self) -> &DenaliPhy460 {
        &self.denali_phy_460
    }
    #[doc = "0x2734 - "]
    #[inline(always)]
    pub const fn denali_phy_461(&self) -> &DenaliPhy461 {
        &self.denali_phy_461
    }
    #[doc = "0x2738 - "]
    #[inline(always)]
    pub const fn denali_phy_462(&self) -> &DenaliPhy462 {
        &self.denali_phy_462
    }
    #[doc = "0x273c - "]
    #[inline(always)]
    pub const fn denali_phy_463(&self) -> &DenaliPhy463 {
        &self.denali_phy_463
    }
    #[doc = "0x2740 - "]
    #[inline(always)]
    pub const fn denali_phy_464(&self) -> &DenaliPhy464 {
        &self.denali_phy_464
    }
    #[doc = "0x2744 - "]
    #[inline(always)]
    pub const fn denali_phy_465(&self) -> &DenaliPhy465 {
        &self.denali_phy_465
    }
    #[doc = "0x274c - "]
    #[inline(always)]
    pub const fn denali_phy_467(&self) -> &DenaliPhy467 {
        &self.denali_phy_467
    }
    #[doc = "0x2750 - "]
    #[inline(always)]
    pub const fn denali_phy_468(&self) -> &DenaliPhy468 {
        &self.denali_phy_468
    }
    #[doc = "0x2754 - "]
    #[inline(always)]
    pub const fn denali_phy_469(&self) -> &DenaliPhy469 {
        &self.denali_phy_469
    }
    #[doc = "0x2758 - "]
    #[inline(always)]
    pub const fn denali_phy_470(&self) -> &DenaliPhy470 {
        &self.denali_phy_470
    }
    #[doc = "0x275c - "]
    #[inline(always)]
    pub const fn denali_phy_471(&self) -> &DenaliPhy471 {
        &self.denali_phy_471
    }
    #[doc = "0x2760 - "]
    #[inline(always)]
    pub const fn denali_phy_472(&self) -> &DenaliPhy472 {
        &self.denali_phy_472
    }
    #[doc = "0x2764 - "]
    #[inline(always)]
    pub const fn denali_phy_473(&self) -> &DenaliPhy473 {
        &self.denali_phy_473
    }
    #[doc = "0x2768 - "]
    #[inline(always)]
    pub const fn denali_phy_474(&self) -> &DenaliPhy474 {
        &self.denali_phy_474
    }
    #[doc = "0x2800 - "]
    #[inline(always)]
    pub const fn denali_phy_512(&self) -> &DenaliPhy512 {
        &self.denali_phy_512
    }
    #[doc = "0x2804 - "]
    #[inline(always)]
    pub const fn denali_phy_513(&self) -> &DenaliPhy513 {
        &self.denali_phy_513
    }
    #[doc = "0x2808 - "]
    #[inline(always)]
    pub const fn denali_phy_514(&self) -> &DenaliPhy514 {
        &self.denali_phy_514
    }
    #[doc = "0x280c - "]
    #[inline(always)]
    pub const fn denali_phy_515(&self) -> &DenaliPhy515 {
        &self.denali_phy_515
    }
    #[doc = "0x2810 - "]
    #[inline(always)]
    pub const fn denali_phy_516(&self) -> &DenaliPhy516 {
        &self.denali_phy_516
    }
    #[doc = "0x2814 - "]
    #[inline(always)]
    pub const fn denali_phy_517(&self) -> &DenaliPhy517 {
        &self.denali_phy_517
    }
    #[doc = "0x2818 - "]
    #[inline(always)]
    pub const fn denali_phy_518(&self) -> &DenaliPhy518 {
        &self.denali_phy_518
    }
    #[doc = "0x281c - "]
    #[inline(always)]
    pub const fn denali_phy_519(&self) -> &DenaliPhy519 {
        &self.denali_phy_519
    }
    #[doc = "0x2820 - "]
    #[inline(always)]
    pub const fn denali_phy_520(&self) -> &DenaliPhy520 {
        &self.denali_phy_520
    }
    #[doc = "0x2824 - "]
    #[inline(always)]
    pub const fn denali_phy_521(&self) -> &DenaliPhy521 {
        &self.denali_phy_521
    }
    #[doc = "0x2828 - "]
    #[inline(always)]
    pub const fn denali_phy_522(&self) -> &DenaliPhy522 {
        &self.denali_phy_522
    }
    #[doc = "0x282c - "]
    #[inline(always)]
    pub const fn denali_phy_523(&self) -> &DenaliPhy523 {
        &self.denali_phy_523
    }
    #[doc = "0x2830 - "]
    #[inline(always)]
    pub const fn denali_phy_524(&self) -> &DenaliPhy524 {
        &self.denali_phy_524
    }
    #[doc = "0x2834 - "]
    #[inline(always)]
    pub const fn denali_phy_525(&self) -> &DenaliPhy525 {
        &self.denali_phy_525
    }
    #[doc = "0x2838 - "]
    #[inline(always)]
    pub const fn denali_phy_526(&self) -> &DenaliPhy526 {
        &self.denali_phy_526
    }
    #[doc = "0x283c - "]
    #[inline(always)]
    pub const fn denali_phy_527(&self) -> &DenaliPhy527 {
        &self.denali_phy_527
    }
    #[doc = "0x2840 - "]
    #[inline(always)]
    pub const fn denali_phy_528(&self) -> &DenaliPhy528 {
        &self.denali_phy_528
    }
    #[doc = "0x2844 - "]
    #[inline(always)]
    pub const fn denali_phy_529(&self) -> &DenaliPhy529 {
        &self.denali_phy_529
    }
    #[doc = "0x2848 - "]
    #[inline(always)]
    pub const fn denali_phy_530(&self) -> &DenaliPhy530 {
        &self.denali_phy_530
    }
    #[doc = "0x284c - "]
    #[inline(always)]
    pub const fn denali_phy_531(&self) -> &DenaliPhy531 {
        &self.denali_phy_531
    }
    #[doc = "0x2850 - "]
    #[inline(always)]
    pub const fn denali_phy_532(&self) -> &DenaliPhy532 {
        &self.denali_phy_532
    }
    #[doc = "0x2854 - "]
    #[inline(always)]
    pub const fn denali_phy_533(&self) -> &DenaliPhy533 {
        &self.denali_phy_533
    }
    #[doc = "0x2858 - "]
    #[inline(always)]
    pub const fn denali_phy_534(&self) -> &DenaliPhy534 {
        &self.denali_phy_534
    }
    #[doc = "0x285c - "]
    #[inline(always)]
    pub const fn denali_phy_535(&self) -> &DenaliPhy535 {
        &self.denali_phy_535
    }
    #[doc = "0x2860 - "]
    #[inline(always)]
    pub const fn denali_phy_536(&self) -> &DenaliPhy536 {
        &self.denali_phy_536
    }
    #[doc = "0x2864 - "]
    #[inline(always)]
    pub const fn denali_phy_537(&self) -> &DenaliPhy537 {
        &self.denali_phy_537
    }
    #[doc = "0x2868 - "]
    #[inline(always)]
    pub const fn denali_phy_538(&self) -> &DenaliPhy538 {
        &self.denali_phy_538
    }
    #[doc = "0x286c - "]
    #[inline(always)]
    pub const fn denali_phy_539(&self) -> &DenaliPhy539 {
        &self.denali_phy_539
    }
    #[doc = "0x2870 - "]
    #[inline(always)]
    pub const fn denali_phy_540(&self) -> &DenaliPhy540 {
        &self.denali_phy_540
    }
    #[doc = "0x2874 - "]
    #[inline(always)]
    pub const fn denali_phy_541(&self) -> &DenaliPhy541 {
        &self.denali_phy_541
    }
    #[doc = "0x2878 - "]
    #[inline(always)]
    pub const fn denali_phy_542(&self) -> &DenaliPhy542 {
        &self.denali_phy_542
    }
    #[doc = "0x287c - "]
    #[inline(always)]
    pub const fn denali_phy_543(&self) -> &DenaliPhy543 {
        &self.denali_phy_543
    }
    #[doc = "0x2880 - "]
    #[inline(always)]
    pub const fn denali_phy_544(&self) -> &DenaliPhy544 {
        &self.denali_phy_544
    }
    #[doc = "0x2884 - "]
    #[inline(always)]
    pub const fn denali_phy_545(&self) -> &DenaliPhy545 {
        &self.denali_phy_545
    }
    #[doc = "0x2888 - "]
    #[inline(always)]
    pub const fn denali_phy_546(&self) -> &DenaliPhy546 {
        &self.denali_phy_546
    }
    #[doc = "0x288c - "]
    #[inline(always)]
    pub const fn denali_phy_547(&self) -> &DenaliPhy547 {
        &self.denali_phy_547
    }
    #[doc = "0x2890 - "]
    #[inline(always)]
    pub const fn denali_phy_548(&self) -> &DenaliPhy548 {
        &self.denali_phy_548
    }
    #[doc = "0x2894 - "]
    #[inline(always)]
    pub const fn denali_phy_549(&self) -> &DenaliPhy549 {
        &self.denali_phy_549
    }
    #[doc = "0x2a00 - "]
    #[inline(always)]
    pub const fn denali_phy_640(&self) -> &DenaliPhy640 {
        &self.denali_phy_640
    }
    #[doc = "0x2a04 - "]
    #[inline(always)]
    pub const fn denali_phy_641(&self) -> &DenaliPhy641 {
        &self.denali_phy_641
    }
    #[doc = "0x2a08 - "]
    #[inline(always)]
    pub const fn denali_phy_642(&self) -> &DenaliPhy642 {
        &self.denali_phy_642
    }
    #[doc = "0x2a0c - "]
    #[inline(always)]
    pub const fn denali_phy_643(&self) -> &DenaliPhy643 {
        &self.denali_phy_643
    }
    #[doc = "0x2a10 - "]
    #[inline(always)]
    pub const fn denali_phy_644(&self) -> &DenaliPhy644 {
        &self.denali_phy_644
    }
    #[doc = "0x2a14 - "]
    #[inline(always)]
    pub const fn denali_phy_645(&self) -> &DenaliPhy645 {
        &self.denali_phy_645
    }
    #[doc = "0x2a18 - "]
    #[inline(always)]
    pub const fn denali_phy_646(&self) -> &DenaliPhy646 {
        &self.denali_phy_646
    }
    #[doc = "0x2a1c - "]
    #[inline(always)]
    pub const fn denali_phy_647(&self) -> &DenaliPhy647 {
        &self.denali_phy_647
    }
    #[doc = "0x2a20 - "]
    #[inline(always)]
    pub const fn denali_phy_648(&self) -> &DenaliPhy648 {
        &self.denali_phy_648
    }
    #[doc = "0x2a24 - "]
    #[inline(always)]
    pub const fn denali_phy_649(&self) -> &DenaliPhy649 {
        &self.denali_phy_649
    }
    #[doc = "0x2a28 - "]
    #[inline(always)]
    pub const fn denali_phy_650(&self) -> &DenaliPhy650 {
        &self.denali_phy_650
    }
    #[doc = "0x2a2c - "]
    #[inline(always)]
    pub const fn denali_phy_651(&self) -> &DenaliPhy651 {
        &self.denali_phy_651
    }
    #[doc = "0x2a30 - "]
    #[inline(always)]
    pub const fn denali_phy_652(&self) -> &DenaliPhy652 {
        &self.denali_phy_652
    }
    #[doc = "0x2a34 - "]
    #[inline(always)]
    pub const fn denali_phy_653(&self) -> &DenaliPhy653 {
        &self.denali_phy_653
    }
    #[doc = "0x2a38 - "]
    #[inline(always)]
    pub const fn denali_phy_654(&self) -> &DenaliPhy654 {
        &self.denali_phy_654
    }
    #[doc = "0x2a3c - "]
    #[inline(always)]
    pub const fn denali_phy_655(&self) -> &DenaliPhy655 {
        &self.denali_phy_655
    }
    #[doc = "0x2a40 - "]
    #[inline(always)]
    pub const fn denali_phy_656(&self) -> &DenaliPhy656 {
        &self.denali_phy_656
    }
    #[doc = "0x2a44 - "]
    #[inline(always)]
    pub const fn denali_phy_657(&self) -> &DenaliPhy657 {
        &self.denali_phy_657
    }
    #[doc = "0x2a48 - "]
    #[inline(always)]
    pub const fn denali_phy_658(&self) -> &DenaliPhy658 {
        &self.denali_phy_658
    }
    #[doc = "0x2a4c - "]
    #[inline(always)]
    pub const fn denali_phy_659(&self) -> &DenaliPhy659 {
        &self.denali_phy_659
    }
    #[doc = "0x2a50 - "]
    #[inline(always)]
    pub const fn denali_phy_660(&self) -> &DenaliPhy660 {
        &self.denali_phy_660
    }
    #[doc = "0x2a54 - "]
    #[inline(always)]
    pub const fn denali_phy_661(&self) -> &DenaliPhy661 {
        &self.denali_phy_661
    }
    #[doc = "0x2a58 - "]
    #[inline(always)]
    pub const fn denali_phy_662(&self) -> &DenaliPhy662 {
        &self.denali_phy_662
    }
    #[doc = "0x2a5c - "]
    #[inline(always)]
    pub const fn denali_phy_663(&self) -> &DenaliPhy663 {
        &self.denali_phy_663
    }
    #[doc = "0x2a60 - "]
    #[inline(always)]
    pub const fn denali_phy_664(&self) -> &DenaliPhy664 {
        &self.denali_phy_664
    }
    #[doc = "0x2a64 - "]
    #[inline(always)]
    pub const fn denali_phy_665(&self) -> &DenaliPhy665 {
        &self.denali_phy_665
    }
    #[doc = "0x2a68 - "]
    #[inline(always)]
    pub const fn denali_phy_666(&self) -> &DenaliPhy666 {
        &self.denali_phy_666
    }
    #[doc = "0x2a6c - "]
    #[inline(always)]
    pub const fn denali_phy_667(&self) -> &DenaliPhy667 {
        &self.denali_phy_667
    }
    #[doc = "0x2a70 - "]
    #[inline(always)]
    pub const fn denali_phy_668(&self) -> &DenaliPhy668 {
        &self.denali_phy_668
    }
    #[doc = "0x2a74 - "]
    #[inline(always)]
    pub const fn denali_phy_669(&self) -> &DenaliPhy669 {
        &self.denali_phy_669
    }
    #[doc = "0x2a78 - "]
    #[inline(always)]
    pub const fn denali_phy_670(&self) -> &DenaliPhy670 {
        &self.denali_phy_670
    }
    #[doc = "0x2a7c - "]
    #[inline(always)]
    pub const fn denali_phy_671(&self) -> &DenaliPhy671 {
        &self.denali_phy_671
    }
    #[doc = "0x2a80 - "]
    #[inline(always)]
    pub const fn denali_phy_672(&self) -> &DenaliPhy672 {
        &self.denali_phy_672
    }
    #[doc = "0x2a84 - "]
    #[inline(always)]
    pub const fn denali_phy_673(&self) -> &DenaliPhy673 {
        &self.denali_phy_673
    }
    #[doc = "0x2a88 - "]
    #[inline(always)]
    pub const fn denali_phy_674(&self) -> &DenaliPhy674 {
        &self.denali_phy_674
    }
    #[doc = "0x2a8c - "]
    #[inline(always)]
    pub const fn denali_phy_675(&self) -> &DenaliPhy675 {
        &self.denali_phy_675
    }
    #[doc = "0x2a90 - "]
    #[inline(always)]
    pub const fn denali_phy_676(&self) -> &DenaliPhy676 {
        &self.denali_phy_676
    }
    #[doc = "0x2a94 - "]
    #[inline(always)]
    pub const fn denali_phy_677(&self) -> &DenaliPhy677 {
        &self.denali_phy_677
    }
    #[doc = "0x2c00 - "]
    #[inline(always)]
    pub const fn denali_phy_768(&self) -> &DenaliPhy768 {
        &self.denali_phy_768
    }
    #[doc = "0x2c04 - "]
    #[inline(always)]
    pub const fn denali_phy_769(&self) -> &DenaliPhy769 {
        &self.denali_phy_769
    }
    #[doc = "0x2c08 - "]
    #[inline(always)]
    pub const fn denali_phy_770(&self) -> &DenaliPhy770 {
        &self.denali_phy_770
    }
    #[doc = "0x2c0c - "]
    #[inline(always)]
    pub const fn denali_phy_771(&self) -> &DenaliPhy771 {
        &self.denali_phy_771
    }
    #[doc = "0x2c10 - "]
    #[inline(always)]
    pub const fn denali_phy_772(&self) -> &DenaliPhy772 {
        &self.denali_phy_772
    }
    #[doc = "0x2c14 - "]
    #[inline(always)]
    pub const fn denali_phy_773(&self) -> &DenaliPhy773 {
        &self.denali_phy_773
    }
    #[doc = "0x2c18 - "]
    #[inline(always)]
    pub const fn denali_phy_774(&self) -> &DenaliPhy774 {
        &self.denali_phy_774
    }
    #[doc = "0x2c1c - "]
    #[inline(always)]
    pub const fn denali_phy_775(&self) -> &DenaliPhy775 {
        &self.denali_phy_775
    }
    #[doc = "0x2c20 - "]
    #[inline(always)]
    pub const fn denali_phy_776(&self) -> &DenaliPhy776 {
        &self.denali_phy_776
    }
    #[doc = "0x2c24 - "]
    #[inline(always)]
    pub const fn denali_phy_777(&self) -> &DenaliPhy777 {
        &self.denali_phy_777
    }
    #[doc = "0x2c28 - "]
    #[inline(always)]
    pub const fn denali_phy_778(&self) -> &DenaliPhy778 {
        &self.denali_phy_778
    }
    #[doc = "0x2c2c - "]
    #[inline(always)]
    pub const fn denali_phy_779(&self) -> &DenaliPhy779 {
        &self.denali_phy_779
    }
    #[doc = "0x2c30 - "]
    #[inline(always)]
    pub const fn denali_phy_780(&self) -> &DenaliPhy780 {
        &self.denali_phy_780
    }
    #[doc = "0x2c34 - "]
    #[inline(always)]
    pub const fn denali_phy_781(&self) -> &DenaliPhy781 {
        &self.denali_phy_781
    }
    #[doc = "0x2c38 - "]
    #[inline(always)]
    pub const fn denali_phy_782(&self) -> &DenaliPhy782 {
        &self.denali_phy_782
    }
    #[doc = "0x2c3c - "]
    #[inline(always)]
    pub const fn denali_phy_783(&self) -> &DenaliPhy783 {
        &self.denali_phy_783
    }
    #[doc = "0x2c40 - "]
    #[inline(always)]
    pub const fn denali_phy_784(&self) -> &DenaliPhy784 {
        &self.denali_phy_784
    }
    #[doc = "0x2c44 - "]
    #[inline(always)]
    pub const fn denali_phy_785(&self) -> &DenaliPhy785 {
        &self.denali_phy_785
    }
    #[doc = "0x2c48 - "]
    #[inline(always)]
    pub const fn denali_phy_786(&self) -> &DenaliPhy786 {
        &self.denali_phy_786
    }
    #[doc = "0x2c4c - "]
    #[inline(always)]
    pub const fn denali_phy_787(&self) -> &DenaliPhy787 {
        &self.denali_phy_787
    }
    #[doc = "0x2c50 - "]
    #[inline(always)]
    pub const fn denali_phy_788(&self) -> &DenaliPhy788 {
        &self.denali_phy_788
    }
    #[doc = "0x2c54 - "]
    #[inline(always)]
    pub const fn denali_phy_789(&self) -> &DenaliPhy789 {
        &self.denali_phy_789
    }
    #[doc = "0x2c58 - "]
    #[inline(always)]
    pub const fn denali_phy_790(&self) -> &DenaliPhy790 {
        &self.denali_phy_790
    }
    #[doc = "0x2c5c - "]
    #[inline(always)]
    pub const fn denali_phy_791(&self) -> &DenaliPhy791 {
        &self.denali_phy_791
    }
    #[doc = "0x2c60 - "]
    #[inline(always)]
    pub const fn denali_phy_792(&self) -> &DenaliPhy792 {
        &self.denali_phy_792
    }
    #[doc = "0x2c64 - "]
    #[inline(always)]
    pub const fn denali_phy_793(&self) -> &DenaliPhy793 {
        &self.denali_phy_793
    }
    #[doc = "0x2c68 - "]
    #[inline(always)]
    pub const fn denali_phy_794(&self) -> &DenaliPhy794 {
        &self.denali_phy_794
    }
    #[doc = "0x2c6c - "]
    #[inline(always)]
    pub const fn denali_phy_795(&self) -> &DenaliPhy795 {
        &self.denali_phy_795
    }
    #[doc = "0x2c70 - "]
    #[inline(always)]
    pub const fn denali_phy_796(&self) -> &DenaliPhy796 {
        &self.denali_phy_796
    }
    #[doc = "0x2c74 - "]
    #[inline(always)]
    pub const fn denali_phy_797(&self) -> &DenaliPhy797 {
        &self.denali_phy_797
    }
    #[doc = "0x2c78 - "]
    #[inline(always)]
    pub const fn denali_phy_798(&self) -> &DenaliPhy798 {
        &self.denali_phy_798
    }
    #[doc = "0x2c7c - "]
    #[inline(always)]
    pub const fn denali_phy_799(&self) -> &DenaliPhy799 {
        &self.denali_phy_799
    }
    #[doc = "0x2c80 - "]
    #[inline(always)]
    pub const fn denali_phy_800(&self) -> &DenaliPhy800 {
        &self.denali_phy_800
    }
    #[doc = "0x2c84 - "]
    #[inline(always)]
    pub const fn denali_phy_801(&self) -> &DenaliPhy801 {
        &self.denali_phy_801
    }
    #[doc = "0x2c88 - "]
    #[inline(always)]
    pub const fn denali_phy_802(&self) -> &DenaliPhy802 {
        &self.denali_phy_802
    }
    #[doc = "0x2c8c - "]
    #[inline(always)]
    pub const fn denali_phy_803(&self) -> &DenaliPhy803 {
        &self.denali_phy_803
    }
    #[doc = "0x2c90 - "]
    #[inline(always)]
    pub const fn denali_phy_804(&self) -> &DenaliPhy804 {
        &self.denali_phy_804
    }
    #[doc = "0x2c94 - "]
    #[inline(always)]
    pub const fn denali_phy_805(&self) -> &DenaliPhy805 {
        &self.denali_phy_805
    }
    #[doc = "0x2e00 - "]
    #[inline(always)]
    pub const fn denali_phy_896(&self) -> &DenaliPhy896 {
        &self.denali_phy_896
    }
    #[doc = "0x2e04 - "]
    #[inline(always)]
    pub const fn denali_phy_897(&self) -> &DenaliPhy897 {
        &self.denali_phy_897
    }
    #[doc = "0x2e08 - "]
    #[inline(always)]
    pub const fn denali_phy_898(&self) -> &DenaliPhy898 {
        &self.denali_phy_898
    }
    #[doc = "0x2e0c - "]
    #[inline(always)]
    pub const fn denali_phy_899(&self) -> &DenaliPhy899 {
        &self.denali_phy_899
    }
    #[doc = "0x2e10 - "]
    #[inline(always)]
    pub const fn denali_phy_900(&self) -> &DenaliPhy900 {
        &self.denali_phy_900
    }
    #[doc = "0x2e14 - "]
    #[inline(always)]
    pub const fn denali_phy_901(&self) -> &DenaliPhy901 {
        &self.denali_phy_901
    }
    #[doc = "0x2e18 - "]
    #[inline(always)]
    pub const fn denali_phy_902(&self) -> &DenaliPhy902 {
        &self.denali_phy_902
    }
    #[doc = "0x2e1c - "]
    #[inline(always)]
    pub const fn denali_phy_903(&self) -> &DenaliPhy903 {
        &self.denali_phy_903
    }
    #[doc = "0x2e20 - "]
    #[inline(always)]
    pub const fn denali_phy_904(&self) -> &DenaliPhy904 {
        &self.denali_phy_904
    }
    #[doc = "0x2e24 - "]
    #[inline(always)]
    pub const fn denali_phy_905(&self) -> &DenaliPhy905 {
        &self.denali_phy_905
    }
    #[doc = "0x2e28 - "]
    #[inline(always)]
    pub const fn denali_phy_906(&self) -> &DenaliPhy906 {
        &self.denali_phy_906
    }
    #[doc = "0x2e2c - "]
    #[inline(always)]
    pub const fn denali_phy_907(&self) -> &DenaliPhy907 {
        &self.denali_phy_907
    }
    #[doc = "0x2e30 - "]
    #[inline(always)]
    pub const fn denali_phy_908(&self) -> &DenaliPhy908 {
        &self.denali_phy_908
    }
    #[doc = "0x2e38 - "]
    #[inline(always)]
    pub const fn denali_phy_910(&self) -> &DenaliPhy910 {
        &self.denali_phy_910
    }
    #[doc = "0x2e3c - "]
    #[inline(always)]
    pub const fn denali_phy_911(&self) -> &DenaliPhy911 {
        &self.denali_phy_911
    }
    #[doc = "0x2e40 - "]
    #[inline(always)]
    pub const fn denali_phy_912(&self) -> &DenaliPhy912 {
        &self.denali_phy_912
    }
    #[doc = "0x2e44 - "]
    #[inline(always)]
    pub const fn denali_phy_913(&self) -> &DenaliPhy913 {
        &self.denali_phy_913
    }
    #[doc = "0x2e48 - "]
    #[inline(always)]
    pub const fn denali_phy_914(&self) -> &DenaliPhy914 {
        &self.denali_phy_914
    }
    #[doc = "0x2e4c - "]
    #[inline(always)]
    pub const fn denali_phy_915(&self) -> &DenaliPhy915 {
        &self.denali_phy_915
    }
    #[doc = "0x2e50 - "]
    #[inline(always)]
    pub const fn denali_phy_916(&self) -> &DenaliPhy916 {
        &self.denali_phy_916
    }
    #[doc = "0x2e54 - "]
    #[inline(always)]
    pub const fn denali_phy_917(&self) -> &DenaliPhy917 {
        &self.denali_phy_917
    }
    #[doc = "0x2e58 - "]
    #[inline(always)]
    pub const fn denali_phy_918(&self) -> &DenaliPhy918 {
        &self.denali_phy_918
    }
    #[doc = "0x2e5c - "]
    #[inline(always)]
    pub const fn denali_phy_919(&self) -> &DenaliPhy919 {
        &self.denali_phy_919
    }
    #[doc = "0x2e60 - "]
    #[inline(always)]
    pub const fn denali_phy_920(&self) -> &DenaliPhy920 {
        &self.denali_phy_920
    }
    #[doc = "0x2e64 - "]
    #[inline(always)]
    pub const fn denali_phy_921(&self) -> &DenaliPhy921 {
        &self.denali_phy_921
    }
    #[doc = "0x2e68 - "]
    #[inline(always)]
    pub const fn denali_phy_922(&self) -> &DenaliPhy922 {
        &self.denali_phy_922
    }
    #[doc = "0x2e6c - "]
    #[inline(always)]
    pub const fn denali_phy_923(&self) -> &DenaliPhy923 {
        &self.denali_phy_923
    }
    #[doc = "0x2e70 - "]
    #[inline(always)]
    pub const fn denali_phy_924(&self) -> &DenaliPhy924 {
        &self.denali_phy_924
    }
    #[doc = "0x2e74 - "]
    #[inline(always)]
    pub const fn denali_phy_925(&self) -> &DenaliPhy925 {
        &self.denali_phy_925
    }
    #[doc = "0x2e78 - "]
    #[inline(always)]
    pub const fn denali_phy_926(&self) -> &DenaliPhy926 {
        &self.denali_phy_926
    }
    #[doc = "0x2e7c - "]
    #[inline(always)]
    pub const fn denali_phy_927(&self) -> &DenaliPhy927 {
        &self.denali_phy_927
    }
    #[doc = "0x2e80 - "]
    #[inline(always)]
    pub const fn denali_phy_928(&self) -> &DenaliPhy928 {
        &self.denali_phy_928
    }
    #[doc = "0x2e84 - "]
    #[inline(always)]
    pub const fn denali_phy_929(&self) -> &DenaliPhy929 {
        &self.denali_phy_929
    }
    #[doc = "0x2e88 - "]
    #[inline(always)]
    pub const fn denali_phy_930(&self) -> &DenaliPhy930 {
        &self.denali_phy_930
    }
    #[doc = "0x2e8c - "]
    #[inline(always)]
    pub const fn denali_phy_931(&self) -> &DenaliPhy931 {
        &self.denali_phy_931
    }
    #[doc = "0x2e90 - "]
    #[inline(always)]
    pub const fn denali_phy_932(&self) -> &DenaliPhy932 {
        &self.denali_phy_932
    }
    #[doc = "0x2e94 - "]
    #[inline(always)]
    pub const fn denali_phy_933(&self) -> &DenaliPhy933 {
        &self.denali_phy_933
    }
    #[doc = "0x2e98 - "]
    #[inline(always)]
    pub const fn denali_phy_934(&self) -> &DenaliPhy934 {
        &self.denali_phy_934
    }
    #[doc = "0x2e9c - "]
    #[inline(always)]
    pub const fn denali_phy_935(&self) -> &DenaliPhy935 {
        &self.denali_phy_935
    }
    #[doc = "0x2ea0 - "]
    #[inline(always)]
    pub const fn denali_phy_936(&self) -> &DenaliPhy936 {
        &self.denali_phy_936
    }
    #[doc = "0x2ea4 - "]
    #[inline(always)]
    pub const fn denali_phy_937(&self) -> &DenaliPhy937 {
        &self.denali_phy_937
    }
    #[doc = "0x2ea8 - "]
    #[inline(always)]
    pub const fn denali_phy_938(&self) -> &DenaliPhy938 {
        &self.denali_phy_938
    }
    #[doc = "0x2eac - "]
    #[inline(always)]
    pub const fn denali_phy_939(&self) -> &DenaliPhy939 {
        &self.denali_phy_939
    }
    #[doc = "0x2eb0 - "]
    #[inline(always)]
    pub const fn denali_phy_940(&self) -> &DenaliPhy940 {
        &self.denali_phy_940
    }
    #[doc = "0x2eb4 - "]
    #[inline(always)]
    pub const fn denali_phy_941(&self) -> &DenaliPhy941 {
        &self.denali_phy_941
    }
    #[doc = "0x2eb8 - "]
    #[inline(always)]
    pub const fn denali_phy_942(&self) -> &DenaliPhy942 {
        &self.denali_phy_942
    }
    #[doc = "0x2ebc - "]
    #[inline(always)]
    pub const fn denali_phy_943(&self) -> &DenaliPhy943 {
        &self.denali_phy_943
    }
    #[doc = "0x2ec0 - "]
    #[inline(always)]
    pub const fn denali_phy_944(&self) -> &DenaliPhy944 {
        &self.denali_phy_944
    }
    #[doc = "0x2ec4 - "]
    #[inline(always)]
    pub const fn denali_phy_945(&self) -> &DenaliPhy945 {
        &self.denali_phy_945
    }
    #[doc = "0x2ec8 - "]
    #[inline(always)]
    pub const fn denali_phy_946(&self) -> &DenaliPhy946 {
        &self.denali_phy_946
    }
    #[doc = "0x2ecc - "]
    #[inline(always)]
    pub const fn denali_phy_947(&self) -> &DenaliPhy947 {
        &self.denali_phy_947
    }
    #[doc = "0x2ed0 - "]
    #[inline(always)]
    pub const fn denali_phy_948(&self) -> &DenaliPhy948 {
        &self.denali_phy_948
    }
    #[doc = "0x2ed4 - "]
    #[inline(always)]
    pub const fn denali_phy_949(&self) -> &DenaliPhy949 {
        &self.denali_phy_949
    }
    #[doc = "0x2ed8 - "]
    #[inline(always)]
    pub const fn denali_phy_950(&self) -> &DenaliPhy950 {
        &self.denali_phy_950
    }
    #[doc = "0x2edc - "]
    #[inline(always)]
    pub const fn denali_phy_951(&self) -> &DenaliPhy951 {
        &self.denali_phy_951
    }
    #[doc = "0x2ee0 - "]
    #[inline(always)]
    pub const fn denali_phy_952(&self) -> &DenaliPhy952 {
        &self.denali_phy_952
    }
    #[doc = "0x2ee4 - "]
    #[inline(always)]
    pub const fn denali_phy_953(&self) -> &DenaliPhy953 {
        &self.denali_phy_953
    }
    #[doc = "0x2ee8 - "]
    #[inline(always)]
    pub const fn denali_phy_954(&self) -> &DenaliPhy954 {
        &self.denali_phy_954
    }
    #[doc = "0x2eec - "]
    #[inline(always)]
    pub const fn denali_phy_955(&self) -> &DenaliPhy955 {
        &self.denali_phy_955
    }
    #[doc = "0x2ef0 - "]
    #[inline(always)]
    pub const fn denali_phy_956(&self) -> &DenaliPhy956 {
        &self.denali_phy_956
    }
    #[doc = "0x2ef4 - "]
    #[inline(always)]
    pub const fn denali_phy_957(&self) -> &DenaliPhy957 {
        &self.denali_phy_957
    }
    #[doc = "0x2ef8 - "]
    #[inline(always)]
    pub const fn denali_phy_958(&self) -> &DenaliPhy958 {
        &self.denali_phy_958
    }
}
#[doc = "DENALI_CTL_00 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_00::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_00::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_00`]
module"]
#[doc(alias = "DENALI_CTL_00")]
pub type DenaliCtl00 = crate::Reg<denali_ctl_00::DenaliCtl00Spec>;
#[doc = ""]
pub mod denali_ctl_00;
#[doc = "DENALI_CTL_01 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_01::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_01`]
module"]
#[doc(alias = "DENALI_CTL_01")]
pub type DenaliCtl01 = crate::Reg<denali_ctl_01::DenaliCtl01Spec>;
#[doc = ""]
pub mod denali_ctl_01;
#[doc = "DENALI_CTL_02 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_02::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_02`]
module"]
#[doc(alias = "DENALI_CTL_02")]
pub type DenaliCtl02 = crate::Reg<denali_ctl_02::DenaliCtl02Spec>;
#[doc = ""]
pub mod denali_ctl_02;
#[doc = "DENALI_CTL_03 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_03::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_03`]
module"]
#[doc(alias = "DENALI_CTL_03")]
pub type DenaliCtl03 = crate::Reg<denali_ctl_03::DenaliCtl03Spec>;
#[doc = ""]
pub mod denali_ctl_03;
#[doc = "DENALI_CTL_04 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_04::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_04::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_04`]
module"]
#[doc(alias = "DENALI_CTL_04")]
pub type DenaliCtl04 = crate::Reg<denali_ctl_04::DenaliCtl04Spec>;
#[doc = ""]
pub mod denali_ctl_04;
#[doc = "DENALI_CTL_05 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_05::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_05::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_05`]
module"]
#[doc(alias = "DENALI_CTL_05")]
pub type DenaliCtl05 = crate::Reg<denali_ctl_05::DenaliCtl05Spec>;
#[doc = ""]
pub mod denali_ctl_05;
#[doc = "DENALI_CTL_06 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_06::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_06::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_06`]
module"]
#[doc(alias = "DENALI_CTL_06")]
pub type DenaliCtl06 = crate::Reg<denali_ctl_06::DenaliCtl06Spec>;
#[doc = ""]
pub mod denali_ctl_06;
#[doc = "DENALI_CTL_07 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_07::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_07::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_07`]
module"]
#[doc(alias = "DENALI_CTL_07")]
pub type DenaliCtl07 = crate::Reg<denali_ctl_07::DenaliCtl07Spec>;
#[doc = ""]
pub mod denali_ctl_07;
#[doc = "DENALI_CTL_08 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_08::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_08::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_08`]
module"]
#[doc(alias = "DENALI_CTL_08")]
pub type DenaliCtl08 = crate::Reg<denali_ctl_08::DenaliCtl08Spec>;
#[doc = ""]
pub mod denali_ctl_08;
#[doc = "DENALI_CTL_09 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_09::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_09::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_09`]
module"]
#[doc(alias = "DENALI_CTL_09")]
pub type DenaliCtl09 = crate::Reg<denali_ctl_09::DenaliCtl09Spec>;
#[doc = ""]
pub mod denali_ctl_09;
#[doc = "DENALI_CTL_10 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_10`]
module"]
#[doc(alias = "DENALI_CTL_10")]
pub type DenaliCtl10 = crate::Reg<denali_ctl_10::DenaliCtl10Spec>;
#[doc = ""]
pub mod denali_ctl_10;
#[doc = "DENALI_CTL_11 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_11`]
module"]
#[doc(alias = "DENALI_CTL_11")]
pub type DenaliCtl11 = crate::Reg<denali_ctl_11::DenaliCtl11Spec>;
#[doc = ""]
pub mod denali_ctl_11;
#[doc = "DENALI_CTL_12 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_12`]
module"]
#[doc(alias = "DENALI_CTL_12")]
pub type DenaliCtl12 = crate::Reg<denali_ctl_12::DenaliCtl12Spec>;
#[doc = ""]
pub mod denali_ctl_12;
#[doc = "DENALI_CTL_13 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_13`]
module"]
#[doc(alias = "DENALI_CTL_13")]
pub type DenaliCtl13 = crate::Reg<denali_ctl_13::DenaliCtl13Spec>;
#[doc = ""]
pub mod denali_ctl_13;
#[doc = "DENALI_CTL_14 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_14`]
module"]
#[doc(alias = "DENALI_CTL_14")]
pub type DenaliCtl14 = crate::Reg<denali_ctl_14::DenaliCtl14Spec>;
#[doc = ""]
pub mod denali_ctl_14;
#[doc = "DENALI_CTL_15 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_15`]
module"]
#[doc(alias = "DENALI_CTL_15")]
pub type DenaliCtl15 = crate::Reg<denali_ctl_15::DenaliCtl15Spec>;
#[doc = ""]
pub mod denali_ctl_15;
#[doc = "DENALI_CTL_16 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_16`]
module"]
#[doc(alias = "DENALI_CTL_16")]
pub type DenaliCtl16 = crate::Reg<denali_ctl_16::DenaliCtl16Spec>;
#[doc = ""]
pub mod denali_ctl_16;
#[doc = "DENALI_CTL_17 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_17`]
module"]
#[doc(alias = "DENALI_CTL_17")]
pub type DenaliCtl17 = crate::Reg<denali_ctl_17::DenaliCtl17Spec>;
#[doc = ""]
pub mod denali_ctl_17;
#[doc = "DENALI_CTL_18 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_18`]
module"]
#[doc(alias = "DENALI_CTL_18")]
pub type DenaliCtl18 = crate::Reg<denali_ctl_18::DenaliCtl18Spec>;
#[doc = ""]
pub mod denali_ctl_18;
#[doc = "DENALI_CTL_19 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_19`]
module"]
#[doc(alias = "DENALI_CTL_19")]
pub type DenaliCtl19 = crate::Reg<denali_ctl_19::DenaliCtl19Spec>;
#[doc = ""]
pub mod denali_ctl_19;
#[doc = "DENALI_CTL_20 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_20`]
module"]
#[doc(alias = "DENALI_CTL_20")]
pub type DenaliCtl20 = crate::Reg<denali_ctl_20::DenaliCtl20Spec>;
#[doc = ""]
pub mod denali_ctl_20;
#[doc = "DENALI_CTL_21 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_21`]
module"]
#[doc(alias = "DENALI_CTL_21")]
pub type DenaliCtl21 = crate::Reg<denali_ctl_21::DenaliCtl21Spec>;
#[doc = ""]
pub mod denali_ctl_21;
#[doc = "DENALI_CTL_22 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_22`]
module"]
#[doc(alias = "DENALI_CTL_22")]
pub type DenaliCtl22 = crate::Reg<denali_ctl_22::DenaliCtl22Spec>;
#[doc = ""]
pub mod denali_ctl_22;
#[doc = "DENALI_CTL_23 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_23`]
module"]
#[doc(alias = "DENALI_CTL_23")]
pub type DenaliCtl23 = crate::Reg<denali_ctl_23::DenaliCtl23Spec>;
#[doc = ""]
pub mod denali_ctl_23;
#[doc = "DENALI_CTL_24 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_24`]
module"]
#[doc(alias = "DENALI_CTL_24")]
pub type DenaliCtl24 = crate::Reg<denali_ctl_24::DenaliCtl24Spec>;
#[doc = ""]
pub mod denali_ctl_24;
#[doc = "DENALI_CTL_25 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_25`]
module"]
#[doc(alias = "DENALI_CTL_25")]
pub type DenaliCtl25 = crate::Reg<denali_ctl_25::DenaliCtl25Spec>;
#[doc = ""]
pub mod denali_ctl_25;
#[doc = "DENALI_CTL_26 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_26`]
module"]
#[doc(alias = "DENALI_CTL_26")]
pub type DenaliCtl26 = crate::Reg<denali_ctl_26::DenaliCtl26Spec>;
#[doc = ""]
pub mod denali_ctl_26;
#[doc = "DENALI_CTL_27 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_27`]
module"]
#[doc(alias = "DENALI_CTL_27")]
pub type DenaliCtl27 = crate::Reg<denali_ctl_27::DenaliCtl27Spec>;
#[doc = ""]
pub mod denali_ctl_27;
#[doc = "DENALI_CTL_28 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_28`]
module"]
#[doc(alias = "DENALI_CTL_28")]
pub type DenaliCtl28 = crate::Reg<denali_ctl_28::DenaliCtl28Spec>;
#[doc = ""]
pub mod denali_ctl_28;
#[doc = "DENALI_CTL_29 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_29`]
module"]
#[doc(alias = "DENALI_CTL_29")]
pub type DenaliCtl29 = crate::Reg<denali_ctl_29::DenaliCtl29Spec>;
#[doc = ""]
pub mod denali_ctl_29;
#[doc = "DENALI_CTL_30 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_30`]
module"]
#[doc(alias = "DENALI_CTL_30")]
pub type DenaliCtl30 = crate::Reg<denali_ctl_30::DenaliCtl30Spec>;
#[doc = ""]
pub mod denali_ctl_30;
#[doc = "DENALI_CTL_31 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_31`]
module"]
#[doc(alias = "DENALI_CTL_31")]
pub type DenaliCtl31 = crate::Reg<denali_ctl_31::DenaliCtl31Spec>;
#[doc = ""]
pub mod denali_ctl_31;
#[doc = "DENALI_CTL_32 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_32`]
module"]
#[doc(alias = "DENALI_CTL_32")]
pub type DenaliCtl32 = crate::Reg<denali_ctl_32::DenaliCtl32Spec>;
#[doc = ""]
pub mod denali_ctl_32;
#[doc = "DENALI_CTL_33 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_33`]
module"]
#[doc(alias = "DENALI_CTL_33")]
pub type DenaliCtl33 = crate::Reg<denali_ctl_33::DenaliCtl33Spec>;
#[doc = ""]
pub mod denali_ctl_33;
#[doc = "DENALI_CTL_34 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_34`]
module"]
#[doc(alias = "DENALI_CTL_34")]
pub type DenaliCtl34 = crate::Reg<denali_ctl_34::DenaliCtl34Spec>;
#[doc = ""]
pub mod denali_ctl_34;
#[doc = "DENALI_CTL_35 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_35`]
module"]
#[doc(alias = "DENALI_CTL_35")]
pub type DenaliCtl35 = crate::Reg<denali_ctl_35::DenaliCtl35Spec>;
#[doc = ""]
pub mod denali_ctl_35;
#[doc = "DENALI_CTL_36 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_36`]
module"]
#[doc(alias = "DENALI_CTL_36")]
pub type DenaliCtl36 = crate::Reg<denali_ctl_36::DenaliCtl36Spec>;
#[doc = ""]
pub mod denali_ctl_36;
#[doc = "DENALI_CTL_37 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_37`]
module"]
#[doc(alias = "DENALI_CTL_37")]
pub type DenaliCtl37 = crate::Reg<denali_ctl_37::DenaliCtl37Spec>;
#[doc = ""]
pub mod denali_ctl_37;
#[doc = "DENALI_CTL_38 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_38`]
module"]
#[doc(alias = "DENALI_CTL_38")]
pub type DenaliCtl38 = crate::Reg<denali_ctl_38::DenaliCtl38Spec>;
#[doc = ""]
pub mod denali_ctl_38;
#[doc = "DENALI_CTL_39 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_39`]
module"]
#[doc(alias = "DENALI_CTL_39")]
pub type DenaliCtl39 = crate::Reg<denali_ctl_39::DenaliCtl39Spec>;
#[doc = ""]
pub mod denali_ctl_39;
#[doc = "DENALI_CTL_40 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_40`]
module"]
#[doc(alias = "DENALI_CTL_40")]
pub type DenaliCtl40 = crate::Reg<denali_ctl_40::DenaliCtl40Spec>;
#[doc = ""]
pub mod denali_ctl_40;
#[doc = "DENALI_CTL_41 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_41`]
module"]
#[doc(alias = "DENALI_CTL_41")]
pub type DenaliCtl41 = crate::Reg<denali_ctl_41::DenaliCtl41Spec>;
#[doc = ""]
pub mod denali_ctl_41;
#[doc = "DENALI_CTL_42 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_42`]
module"]
#[doc(alias = "DENALI_CTL_42")]
pub type DenaliCtl42 = crate::Reg<denali_ctl_42::DenaliCtl42Spec>;
#[doc = ""]
pub mod denali_ctl_42;
#[doc = "DENALI_CTL_43 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_43`]
module"]
#[doc(alias = "DENALI_CTL_43")]
pub type DenaliCtl43 = crate::Reg<denali_ctl_43::DenaliCtl43Spec>;
#[doc = ""]
pub mod denali_ctl_43;
#[doc = "DENALI_CTL_44 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_44`]
module"]
#[doc(alias = "DENALI_CTL_44")]
pub type DenaliCtl44 = crate::Reg<denali_ctl_44::DenaliCtl44Spec>;
#[doc = ""]
pub mod denali_ctl_44;
#[doc = "DENALI_CTL_45 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_45`]
module"]
#[doc(alias = "DENALI_CTL_45")]
pub type DenaliCtl45 = crate::Reg<denali_ctl_45::DenaliCtl45Spec>;
#[doc = ""]
pub mod denali_ctl_45;
#[doc = "DENALI_CTL_46 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_46`]
module"]
#[doc(alias = "DENALI_CTL_46")]
pub type DenaliCtl46 = crate::Reg<denali_ctl_46::DenaliCtl46Spec>;
#[doc = ""]
pub mod denali_ctl_46;
#[doc = "DENALI_CTL_47 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_47`]
module"]
#[doc(alias = "DENALI_CTL_47")]
pub type DenaliCtl47 = crate::Reg<denali_ctl_47::DenaliCtl47Spec>;
#[doc = ""]
pub mod denali_ctl_47;
#[doc = "DENALI_CTL_48 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_48`]
module"]
#[doc(alias = "DENALI_CTL_48")]
pub type DenaliCtl48 = crate::Reg<denali_ctl_48::DenaliCtl48Spec>;
#[doc = ""]
pub mod denali_ctl_48;
#[doc = "DENALI_CTL_49 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_49`]
module"]
#[doc(alias = "DENALI_CTL_49")]
pub type DenaliCtl49 = crate::Reg<denali_ctl_49::DenaliCtl49Spec>;
#[doc = ""]
pub mod denali_ctl_49;
#[doc = "DENALI_CTL_50 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_50::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_50`]
module"]
#[doc(alias = "DENALI_CTL_50")]
pub type DenaliCtl50 = crate::Reg<denali_ctl_50::DenaliCtl50Spec>;
#[doc = ""]
pub mod denali_ctl_50;
#[doc = "DENALI_CTL_51 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_51`]
module"]
#[doc(alias = "DENALI_CTL_51")]
pub type DenaliCtl51 = crate::Reg<denali_ctl_51::DenaliCtl51Spec>;
#[doc = ""]
pub mod denali_ctl_51;
#[doc = "DENALI_CTL_52 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_52`]
module"]
#[doc(alias = "DENALI_CTL_52")]
pub type DenaliCtl52 = crate::Reg<denali_ctl_52::DenaliCtl52Spec>;
#[doc = ""]
pub mod denali_ctl_52;
#[doc = "DENALI_CTL_53 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_53`]
module"]
#[doc(alias = "DENALI_CTL_53")]
pub type DenaliCtl53 = crate::Reg<denali_ctl_53::DenaliCtl53Spec>;
#[doc = ""]
pub mod denali_ctl_53;
#[doc = "DENALI_CTL_54 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_54::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_54`]
module"]
#[doc(alias = "DENALI_CTL_54")]
pub type DenaliCtl54 = crate::Reg<denali_ctl_54::DenaliCtl54Spec>;
#[doc = ""]
pub mod denali_ctl_54;
#[doc = "DENALI_CTL_55 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_55`]
module"]
#[doc(alias = "DENALI_CTL_55")]
pub type DenaliCtl55 = crate::Reg<denali_ctl_55::DenaliCtl55Spec>;
#[doc = ""]
pub mod denali_ctl_55;
#[doc = "DENALI_CTL_56 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_56::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_56`]
module"]
#[doc(alias = "DENALI_CTL_56")]
pub type DenaliCtl56 = crate::Reg<denali_ctl_56::DenaliCtl56Spec>;
#[doc = ""]
pub mod denali_ctl_56;
#[doc = "DENALI_CTL_57 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_57::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_57::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_57`]
module"]
#[doc(alias = "DENALI_CTL_57")]
pub type DenaliCtl57 = crate::Reg<denali_ctl_57::DenaliCtl57Spec>;
#[doc = ""]
pub mod denali_ctl_57;
#[doc = "DENALI_CTL_58 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_58::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_58::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_58`]
module"]
#[doc(alias = "DENALI_CTL_58")]
pub type DenaliCtl58 = crate::Reg<denali_ctl_58::DenaliCtl58Spec>;
#[doc = ""]
pub mod denali_ctl_58;
#[doc = "DENALI_CTL_59 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_59::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_59`]
module"]
#[doc(alias = "DENALI_CTL_59")]
pub type DenaliCtl59 = crate::Reg<denali_ctl_59::DenaliCtl59Spec>;
#[doc = ""]
pub mod denali_ctl_59;
#[doc = "DENALI_CTL_60 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_60::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_60`]
module"]
#[doc(alias = "DENALI_CTL_60")]
pub type DenaliCtl60 = crate::Reg<denali_ctl_60::DenaliCtl60Spec>;
#[doc = ""]
pub mod denali_ctl_60;
#[doc = "DENALI_CTL_61 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_61::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_61::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_61`]
module"]
#[doc(alias = "DENALI_CTL_61")]
pub type DenaliCtl61 = crate::Reg<denali_ctl_61::DenaliCtl61Spec>;
#[doc = ""]
pub mod denali_ctl_61;
#[doc = "DENALI_CTL_62 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_62::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_62::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_62`]
module"]
#[doc(alias = "DENALI_CTL_62")]
pub type DenaliCtl62 = crate::Reg<denali_ctl_62::DenaliCtl62Spec>;
#[doc = ""]
pub mod denali_ctl_62;
#[doc = "DENALI_CTL_63 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_63::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_63`]
module"]
#[doc(alias = "DENALI_CTL_63")]
pub type DenaliCtl63 = crate::Reg<denali_ctl_63::DenaliCtl63Spec>;
#[doc = ""]
pub mod denali_ctl_63;
#[doc = "DENALI_CTL_64 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_64`]
module"]
#[doc(alias = "DENALI_CTL_64")]
pub type DenaliCtl64 = crate::Reg<denali_ctl_64::DenaliCtl64Spec>;
#[doc = ""]
pub mod denali_ctl_64;
#[doc = "DENALI_CTL_65 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_65::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_65::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_65`]
module"]
#[doc(alias = "DENALI_CTL_65")]
pub type DenaliCtl65 = crate::Reg<denali_ctl_65::DenaliCtl65Spec>;
#[doc = ""]
pub mod denali_ctl_65;
#[doc = "DENALI_CTL_66 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_66::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_66::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_66`]
module"]
#[doc(alias = "DENALI_CTL_66")]
pub type DenaliCtl66 = crate::Reg<denali_ctl_66::DenaliCtl66Spec>;
#[doc = ""]
pub mod denali_ctl_66;
#[doc = "DENALI_CTL_67 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_67`]
module"]
#[doc(alias = "DENALI_CTL_67")]
pub type DenaliCtl67 = crate::Reg<denali_ctl_67::DenaliCtl67Spec>;
#[doc = ""]
pub mod denali_ctl_67;
#[doc = "DENALI_CTL_68 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_68::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_68::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_68`]
module"]
#[doc(alias = "DENALI_CTL_68")]
pub type DenaliCtl68 = crate::Reg<denali_ctl_68::DenaliCtl68Spec>;
#[doc = ""]
pub mod denali_ctl_68;
#[doc = "DENALI_CTL_69 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_69::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_69::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_69`]
module"]
#[doc(alias = "DENALI_CTL_69")]
pub type DenaliCtl69 = crate::Reg<denali_ctl_69::DenaliCtl69Spec>;
#[doc = ""]
pub mod denali_ctl_69;
#[doc = "DENALI_CTL_70 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_70::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_70::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_70`]
module"]
#[doc(alias = "DENALI_CTL_70")]
pub type DenaliCtl70 = crate::Reg<denali_ctl_70::DenaliCtl70Spec>;
#[doc = ""]
pub mod denali_ctl_70;
#[doc = "DENALI_CTL_71 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_71::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_71::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_71`]
module"]
#[doc(alias = "DENALI_CTL_71")]
pub type DenaliCtl71 = crate::Reg<denali_ctl_71::DenaliCtl71Spec>;
#[doc = ""]
pub mod denali_ctl_71;
#[doc = "DENALI_CTL_72 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_72::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_72::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_72`]
module"]
#[doc(alias = "DENALI_CTL_72")]
pub type DenaliCtl72 = crate::Reg<denali_ctl_72::DenaliCtl72Spec>;
#[doc = ""]
pub mod denali_ctl_72;
#[doc = "DENALI_CTL_73 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_73::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_73`]
module"]
#[doc(alias = "DENALI_CTL_73")]
pub type DenaliCtl73 = crate::Reg<denali_ctl_73::DenaliCtl73Spec>;
#[doc = ""]
pub mod denali_ctl_73;
#[doc = "DENALI_CTL_75 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_75::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_75::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_75`]
module"]
#[doc(alias = "DENALI_CTL_75")]
pub type DenaliCtl75 = crate::Reg<denali_ctl_75::DenaliCtl75Spec>;
#[doc = ""]
pub mod denali_ctl_75;
#[doc = "DENALI_CTL_76 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_76::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_76::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_76`]
module"]
#[doc(alias = "DENALI_CTL_76")]
pub type DenaliCtl76 = crate::Reg<denali_ctl_76::DenaliCtl76Spec>;
#[doc = ""]
pub mod denali_ctl_76;
#[doc = "DENALI_CTL_77 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_77::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_77::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_77`]
module"]
#[doc(alias = "DENALI_CTL_77")]
pub type DenaliCtl77 = crate::Reg<denali_ctl_77::DenaliCtl77Spec>;
#[doc = ""]
pub mod denali_ctl_77;
#[doc = "DENALI_CTL_78 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_78::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_78::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_78`]
module"]
#[doc(alias = "DENALI_CTL_78")]
pub type DenaliCtl78 = crate::Reg<denali_ctl_78::DenaliCtl78Spec>;
#[doc = ""]
pub mod denali_ctl_78;
#[doc = "DENALI_CTL_79 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_79::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_79::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_79`]
module"]
#[doc(alias = "DENALI_CTL_79")]
pub type DenaliCtl79 = crate::Reg<denali_ctl_79::DenaliCtl79Spec>;
#[doc = ""]
pub mod denali_ctl_79;
#[doc = "DENALI_CTL_80 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_80::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_80::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_80`]
module"]
#[doc(alias = "DENALI_CTL_80")]
pub type DenaliCtl80 = crate::Reg<denali_ctl_80::DenaliCtl80Spec>;
#[doc = ""]
pub mod denali_ctl_80;
#[doc = "DENALI_CTL_81 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_81::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_81::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_81`]
module"]
#[doc(alias = "DENALI_CTL_81")]
pub type DenaliCtl81 = crate::Reg<denali_ctl_81::DenaliCtl81Spec>;
#[doc = ""]
pub mod denali_ctl_81;
#[doc = "DENALI_CTL_82 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_82::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_82::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_82`]
module"]
#[doc(alias = "DENALI_CTL_82")]
pub type DenaliCtl82 = crate::Reg<denali_ctl_82::DenaliCtl82Spec>;
#[doc = ""]
pub mod denali_ctl_82;
#[doc = "DENALI_CTL_83 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_83::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_83::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_83`]
module"]
#[doc(alias = "DENALI_CTL_83")]
pub type DenaliCtl83 = crate::Reg<denali_ctl_83::DenaliCtl83Spec>;
#[doc = ""]
pub mod denali_ctl_83;
#[doc = "DENALI_CTL_84 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_84::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_84::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_84`]
module"]
#[doc(alias = "DENALI_CTL_84")]
pub type DenaliCtl84 = crate::Reg<denali_ctl_84::DenaliCtl84Spec>;
#[doc = ""]
pub mod denali_ctl_84;
#[doc = "DENALI_CTL_85 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_85::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_85::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_85`]
module"]
#[doc(alias = "DENALI_CTL_85")]
pub type DenaliCtl85 = crate::Reg<denali_ctl_85::DenaliCtl85Spec>;
#[doc = ""]
pub mod denali_ctl_85;
#[doc = "DENALI_CTL_86 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_86::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_86::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_86`]
module"]
#[doc(alias = "DENALI_CTL_86")]
pub type DenaliCtl86 = crate::Reg<denali_ctl_86::DenaliCtl86Spec>;
#[doc = ""]
pub mod denali_ctl_86;
#[doc = "DENALI_CTL_87 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_87::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_87::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_87`]
module"]
#[doc(alias = "DENALI_CTL_87")]
pub type DenaliCtl87 = crate::Reg<denali_ctl_87::DenaliCtl87Spec>;
#[doc = ""]
pub mod denali_ctl_87;
#[doc = "DENALI_CTL_88 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_88::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_88::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_88`]
module"]
#[doc(alias = "DENALI_CTL_88")]
pub type DenaliCtl88 = crate::Reg<denali_ctl_88::DenaliCtl88Spec>;
#[doc = ""]
pub mod denali_ctl_88;
#[doc = "DENALI_CTL_89 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_89::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_89::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_89`]
module"]
#[doc(alias = "DENALI_CTL_89")]
pub type DenaliCtl89 = crate::Reg<denali_ctl_89::DenaliCtl89Spec>;
#[doc = ""]
pub mod denali_ctl_89;
#[doc = "DENALI_CTL_90 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_90::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_90::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_90`]
module"]
#[doc(alias = "DENALI_CTL_90")]
pub type DenaliCtl90 = crate::Reg<denali_ctl_90::DenaliCtl90Spec>;
#[doc = ""]
pub mod denali_ctl_90;
#[doc = "DENALI_CTL_91 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_91::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_91::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_91`]
module"]
#[doc(alias = "DENALI_CTL_91")]
pub type DenaliCtl91 = crate::Reg<denali_ctl_91::DenaliCtl91Spec>;
#[doc = ""]
pub mod denali_ctl_91;
#[doc = "DENALI_CTL_92 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_92::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_92::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_92`]
module"]
#[doc(alias = "DENALI_CTL_92")]
pub type DenaliCtl92 = crate::Reg<denali_ctl_92::DenaliCtl92Spec>;
#[doc = ""]
pub mod denali_ctl_92;
#[doc = "DENALI_CTL_93 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_93::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_93::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_93`]
module"]
#[doc(alias = "DENALI_CTL_93")]
pub type DenaliCtl93 = crate::Reg<denali_ctl_93::DenaliCtl93Spec>;
#[doc = ""]
pub mod denali_ctl_93;
#[doc = "DENALI_CTL_94 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_94::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_94::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_94`]
module"]
#[doc(alias = "DENALI_CTL_94")]
pub type DenaliCtl94 = crate::Reg<denali_ctl_94::DenaliCtl94Spec>;
#[doc = ""]
pub mod denali_ctl_94;
#[doc = "DENALI_CTL_95 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_95::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_95::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_95`]
module"]
#[doc(alias = "DENALI_CTL_95")]
pub type DenaliCtl95 = crate::Reg<denali_ctl_95::DenaliCtl95Spec>;
#[doc = ""]
pub mod denali_ctl_95;
#[doc = "DENALI_CTL_96 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_96::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_96::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_96`]
module"]
#[doc(alias = "DENALI_CTL_96")]
pub type DenaliCtl96 = crate::Reg<denali_ctl_96::DenaliCtl96Spec>;
#[doc = ""]
pub mod denali_ctl_96;
#[doc = "DENALI_CTL_97 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_97::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_97::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_97`]
module"]
#[doc(alias = "DENALI_CTL_97")]
pub type DenaliCtl97 = crate::Reg<denali_ctl_97::DenaliCtl97Spec>;
#[doc = ""]
pub mod denali_ctl_97;
#[doc = "DENALI_CTL_98 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_98::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_98::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_98`]
module"]
#[doc(alias = "DENALI_CTL_98")]
pub type DenaliCtl98 = crate::Reg<denali_ctl_98::DenaliCtl98Spec>;
#[doc = ""]
pub mod denali_ctl_98;
#[doc = "DENALI_CTL_99 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_99::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_99::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_99`]
module"]
#[doc(alias = "DENALI_CTL_99")]
pub type DenaliCtl99 = crate::Reg<denali_ctl_99::DenaliCtl99Spec>;
#[doc = ""]
pub mod denali_ctl_99;
#[doc = "DENALI_CTL_100 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_100::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_100::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_100`]
module"]
#[doc(alias = "DENALI_CTL_100")]
pub type DenaliCtl100 = crate::Reg<denali_ctl_100::DenaliCtl100Spec>;
#[doc = ""]
pub mod denali_ctl_100;
#[doc = "DENALI_CTL_101 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_101::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_101::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_101`]
module"]
#[doc(alias = "DENALI_CTL_101")]
pub type DenaliCtl101 = crate::Reg<denali_ctl_101::DenaliCtl101Spec>;
#[doc = ""]
pub mod denali_ctl_101;
#[doc = "DENALI_CTL_102 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_102::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_102::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_102`]
module"]
#[doc(alias = "DENALI_CTL_102")]
pub type DenaliCtl102 = crate::Reg<denali_ctl_102::DenaliCtl102Spec>;
#[doc = ""]
pub mod denali_ctl_102;
#[doc = "DENALI_CTL_103 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_103::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_103::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_103`]
module"]
#[doc(alias = "DENALI_CTL_103")]
pub type DenaliCtl103 = crate::Reg<denali_ctl_103::DenaliCtl103Spec>;
#[doc = ""]
pub mod denali_ctl_103;
#[doc = "DENALI_CTL_104 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_104::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_104::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_104`]
module"]
#[doc(alias = "DENALI_CTL_104")]
pub type DenaliCtl104 = crate::Reg<denali_ctl_104::DenaliCtl104Spec>;
#[doc = ""]
pub mod denali_ctl_104;
#[doc = "DENALI_CTL_105 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_105::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_105::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_105`]
module"]
#[doc(alias = "DENALI_CTL_105")]
pub type DenaliCtl105 = crate::Reg<denali_ctl_105::DenaliCtl105Spec>;
#[doc = ""]
pub mod denali_ctl_105;
#[doc = "DENALI_CTL_106 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_106::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_106::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_106`]
module"]
#[doc(alias = "DENALI_CTL_106")]
pub type DenaliCtl106 = crate::Reg<denali_ctl_106::DenaliCtl106Spec>;
#[doc = ""]
pub mod denali_ctl_106;
#[doc = "DENALI_CTL_107 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_107::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_107::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_107`]
module"]
#[doc(alias = "DENALI_CTL_107")]
pub type DenaliCtl107 = crate::Reg<denali_ctl_107::DenaliCtl107Spec>;
#[doc = ""]
pub mod denali_ctl_107;
#[doc = "DENALI_CTL_108 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_108::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_108::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_108`]
module"]
#[doc(alias = "DENALI_CTL_108")]
pub type DenaliCtl108 = crate::Reg<denali_ctl_108::DenaliCtl108Spec>;
#[doc = ""]
pub mod denali_ctl_108;
#[doc = "DENALI_CTL_109 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_109::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_109::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_109`]
module"]
#[doc(alias = "DENALI_CTL_109")]
pub type DenaliCtl109 = crate::Reg<denali_ctl_109::DenaliCtl109Spec>;
#[doc = ""]
pub mod denali_ctl_109;
#[doc = "DENALI_CTL_110 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_110::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_110::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_110`]
module"]
#[doc(alias = "DENALI_CTL_110")]
pub type DenaliCtl110 = crate::Reg<denali_ctl_110::DenaliCtl110Spec>;
#[doc = ""]
pub mod denali_ctl_110;
#[doc = "DENALI_CTL_111 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_111::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_111::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_111`]
module"]
#[doc(alias = "DENALI_CTL_111")]
pub type DenaliCtl111 = crate::Reg<denali_ctl_111::DenaliCtl111Spec>;
#[doc = ""]
pub mod denali_ctl_111;
#[doc = "DENALI_CTL_112 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_112::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_112::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_112`]
module"]
#[doc(alias = "DENALI_CTL_112")]
pub type DenaliCtl112 = crate::Reg<denali_ctl_112::DenaliCtl112Spec>;
#[doc = ""]
pub mod denali_ctl_112;
#[doc = "DENALI_CTL_113 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_113::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_113::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_113`]
module"]
#[doc(alias = "DENALI_CTL_113")]
pub type DenaliCtl113 = crate::Reg<denali_ctl_113::DenaliCtl113Spec>;
#[doc = ""]
pub mod denali_ctl_113;
#[doc = "DENALI_CTL_114 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_114::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_114::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_114`]
module"]
#[doc(alias = "DENALI_CTL_114")]
pub type DenaliCtl114 = crate::Reg<denali_ctl_114::DenaliCtl114Spec>;
#[doc = ""]
pub mod denali_ctl_114;
#[doc = "DENALI_CTL_115 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_115::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_115::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_115`]
module"]
#[doc(alias = "DENALI_CTL_115")]
pub type DenaliCtl115 = crate::Reg<denali_ctl_115::DenaliCtl115Spec>;
#[doc = ""]
pub mod denali_ctl_115;
#[doc = "DENALI_CTL_116 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_116::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_116::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_116`]
module"]
#[doc(alias = "DENALI_CTL_116")]
pub type DenaliCtl116 = crate::Reg<denali_ctl_116::DenaliCtl116Spec>;
#[doc = ""]
pub mod denali_ctl_116;
#[doc = "DENALI_CTL_117 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_117::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_117::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_117`]
module"]
#[doc(alias = "DENALI_CTL_117")]
pub type DenaliCtl117 = crate::Reg<denali_ctl_117::DenaliCtl117Spec>;
#[doc = ""]
pub mod denali_ctl_117;
#[doc = "DENALI_CTL_118 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_118::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_118::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_118`]
module"]
#[doc(alias = "DENALI_CTL_118")]
pub type DenaliCtl118 = crate::Reg<denali_ctl_118::DenaliCtl118Spec>;
#[doc = ""]
pub mod denali_ctl_118;
#[doc = "DENALI_CTL_119 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_119::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_119`]
module"]
#[doc(alias = "DENALI_CTL_119")]
pub type DenaliCtl119 = crate::Reg<denali_ctl_119::DenaliCtl119Spec>;
#[doc = ""]
pub mod denali_ctl_119;
#[doc = "DENALI_CTL_120 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_120::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_120`]
module"]
#[doc(alias = "DENALI_CTL_120")]
pub type DenaliCtl120 = crate::Reg<denali_ctl_120::DenaliCtl120Spec>;
#[doc = ""]
pub mod denali_ctl_120;
#[doc = "DENALI_CTL_121 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_121::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_121::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_121`]
module"]
#[doc(alias = "DENALI_CTL_121")]
pub type DenaliCtl121 = crate::Reg<denali_ctl_121::DenaliCtl121Spec>;
#[doc = ""]
pub mod denali_ctl_121;
#[doc = "DENALI_CTL_122 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_122::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_122::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_122`]
module"]
#[doc(alias = "DENALI_CTL_122")]
pub type DenaliCtl122 = crate::Reg<denali_ctl_122::DenaliCtl122Spec>;
#[doc = ""]
pub mod denali_ctl_122;
#[doc = "DENALI_CTL_123 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_123::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_123::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_123`]
module"]
#[doc(alias = "DENALI_CTL_123")]
pub type DenaliCtl123 = crate::Reg<denali_ctl_123::DenaliCtl123Spec>;
#[doc = ""]
pub mod denali_ctl_123;
#[doc = "DENALI_CTL_124 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_124::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_124::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_124`]
module"]
#[doc(alias = "DENALI_CTL_124")]
pub type DenaliCtl124 = crate::Reg<denali_ctl_124::DenaliCtl124Spec>;
#[doc = ""]
pub mod denali_ctl_124;
#[doc = "DENALI_CTL_125 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_125::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_125::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_125`]
module"]
#[doc(alias = "DENALI_CTL_125")]
pub type DenaliCtl125 = crate::Reg<denali_ctl_125::DenaliCtl125Spec>;
#[doc = ""]
pub mod denali_ctl_125;
#[doc = "DENALI_CTL_126 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_126::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_126::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_126`]
module"]
#[doc(alias = "DENALI_CTL_126")]
pub type DenaliCtl126 = crate::Reg<denali_ctl_126::DenaliCtl126Spec>;
#[doc = ""]
pub mod denali_ctl_126;
#[doc = "DENALI_CTL_127 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_127::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_127::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_127`]
module"]
#[doc(alias = "DENALI_CTL_127")]
pub type DenaliCtl127 = crate::Reg<denali_ctl_127::DenaliCtl127Spec>;
#[doc = ""]
pub mod denali_ctl_127;
#[doc = "DENALI_CTL_128 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_128::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_128::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_128`]
module"]
#[doc(alias = "DENALI_CTL_128")]
pub type DenaliCtl128 = crate::Reg<denali_ctl_128::DenaliCtl128Spec>;
#[doc = ""]
pub mod denali_ctl_128;
#[doc = "DENALI_CTL_129 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_129::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_129::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_129`]
module"]
#[doc(alias = "DENALI_CTL_129")]
pub type DenaliCtl129 = crate::Reg<denali_ctl_129::DenaliCtl129Spec>;
#[doc = ""]
pub mod denali_ctl_129;
#[doc = "DENALI_CTL_130 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_130::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_130::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_130`]
module"]
#[doc(alias = "DENALI_CTL_130")]
pub type DenaliCtl130 = crate::Reg<denali_ctl_130::DenaliCtl130Spec>;
#[doc = ""]
pub mod denali_ctl_130;
#[doc = "DENALI_CTL_131 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_131::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_131::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_131`]
module"]
#[doc(alias = "DENALI_CTL_131")]
pub type DenaliCtl131 = crate::Reg<denali_ctl_131::DenaliCtl131Spec>;
#[doc = ""]
pub mod denali_ctl_131;
#[doc = "DENALI_CTL_132 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_132::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_132::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_132`]
module"]
#[doc(alias = "DENALI_CTL_132")]
pub type DenaliCtl132 = crate::Reg<denali_ctl_132::DenaliCtl132Spec>;
#[doc = ""]
pub mod denali_ctl_132;
#[doc = "DENALI_CTL_133 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_133::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_133::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_133`]
module"]
#[doc(alias = "DENALI_CTL_133")]
pub type DenaliCtl133 = crate::Reg<denali_ctl_133::DenaliCtl133Spec>;
#[doc = ""]
pub mod denali_ctl_133;
#[doc = "DENALI_CTL_134 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_134::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_134::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_134`]
module"]
#[doc(alias = "DENALI_CTL_134")]
pub type DenaliCtl134 = crate::Reg<denali_ctl_134::DenaliCtl134Spec>;
#[doc = ""]
pub mod denali_ctl_134;
#[doc = "DENALI_CTL_135 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_135::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_135::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_135`]
module"]
#[doc(alias = "DENALI_CTL_135")]
pub type DenaliCtl135 = crate::Reg<denali_ctl_135::DenaliCtl135Spec>;
#[doc = ""]
pub mod denali_ctl_135;
#[doc = "DENALI_CTL_136 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_136::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_136::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_136`]
module"]
#[doc(alias = "DENALI_CTL_136")]
pub type DenaliCtl136 = crate::Reg<denali_ctl_136::DenaliCtl136Spec>;
#[doc = ""]
pub mod denali_ctl_136;
#[doc = "DENALI_CTL_137 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_137::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_137::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_137`]
module"]
#[doc(alias = "DENALI_CTL_137")]
pub type DenaliCtl137 = crate::Reg<denali_ctl_137::DenaliCtl137Spec>;
#[doc = ""]
pub mod denali_ctl_137;
#[doc = "DENALI_CTL_138 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_138::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_138::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_138`]
module"]
#[doc(alias = "DENALI_CTL_138")]
pub type DenaliCtl138 = crate::Reg<denali_ctl_138::DenaliCtl138Spec>;
#[doc = ""]
pub mod denali_ctl_138;
#[doc = "DENALI_CTL_139 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_139::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_139::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_139`]
module"]
#[doc(alias = "DENALI_CTL_139")]
pub type DenaliCtl139 = crate::Reg<denali_ctl_139::DenaliCtl139Spec>;
#[doc = ""]
pub mod denali_ctl_139;
#[doc = "DENALI_CTL_140 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_140::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_140::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_140`]
module"]
#[doc(alias = "DENALI_CTL_140")]
pub type DenaliCtl140 = crate::Reg<denali_ctl_140::DenaliCtl140Spec>;
#[doc = ""]
pub mod denali_ctl_140;
#[doc = "DENALI_CTL_141 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_141::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_141::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_141`]
module"]
#[doc(alias = "DENALI_CTL_141")]
pub type DenaliCtl141 = crate::Reg<denali_ctl_141::DenaliCtl141Spec>;
#[doc = ""]
pub mod denali_ctl_141;
#[doc = "DENALI_CTL_142 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_142::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_142::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_142`]
module"]
#[doc(alias = "DENALI_CTL_142")]
pub type DenaliCtl142 = crate::Reg<denali_ctl_142::DenaliCtl142Spec>;
#[doc = ""]
pub mod denali_ctl_142;
#[doc = "DENALI_CTL_143 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_143::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_143::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_143`]
module"]
#[doc(alias = "DENALI_CTL_143")]
pub type DenaliCtl143 = crate::Reg<denali_ctl_143::DenaliCtl143Spec>;
#[doc = ""]
pub mod denali_ctl_143;
#[doc = "DENALI_CTL_144 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_144::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_144::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_144`]
module"]
#[doc(alias = "DENALI_CTL_144")]
pub type DenaliCtl144 = crate::Reg<denali_ctl_144::DenaliCtl144Spec>;
#[doc = ""]
pub mod denali_ctl_144;
#[doc = "DENALI_CTL_145 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_145::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_145::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_145`]
module"]
#[doc(alias = "DENALI_CTL_145")]
pub type DenaliCtl145 = crate::Reg<denali_ctl_145::DenaliCtl145Spec>;
#[doc = ""]
pub mod denali_ctl_145;
#[doc = "DENALI_CTL_146 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_146::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_146::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_146`]
module"]
#[doc(alias = "DENALI_CTL_146")]
pub type DenaliCtl146 = crate::Reg<denali_ctl_146::DenaliCtl146Spec>;
#[doc = ""]
pub mod denali_ctl_146;
#[doc = "DENALI_CTL_147 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_147::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_147::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_147`]
module"]
#[doc(alias = "DENALI_CTL_147")]
pub type DenaliCtl147 = crate::Reg<denali_ctl_147::DenaliCtl147Spec>;
#[doc = ""]
pub mod denali_ctl_147;
#[doc = "DENALI_CTL_148 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_148::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_148::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_148`]
module"]
#[doc(alias = "DENALI_CTL_148")]
pub type DenaliCtl148 = crate::Reg<denali_ctl_148::DenaliCtl148Spec>;
#[doc = ""]
pub mod denali_ctl_148;
#[doc = "DENALI_CTL_149 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_149::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_149::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_149`]
module"]
#[doc(alias = "DENALI_CTL_149")]
pub type DenaliCtl149 = crate::Reg<denali_ctl_149::DenaliCtl149Spec>;
#[doc = ""]
pub mod denali_ctl_149;
#[doc = "DENALI_CTL_150 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_150::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_150::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_150`]
module"]
#[doc(alias = "DENALI_CTL_150")]
pub type DenaliCtl150 = crate::Reg<denali_ctl_150::DenaliCtl150Spec>;
#[doc = ""]
pub mod denali_ctl_150;
#[doc = "DENALI_CTL_151 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_151::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_151::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_151`]
module"]
#[doc(alias = "DENALI_CTL_151")]
pub type DenaliCtl151 = crate::Reg<denali_ctl_151::DenaliCtl151Spec>;
#[doc = ""]
pub mod denali_ctl_151;
#[doc = "DENALI_CTL_152 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_152::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_152::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_152`]
module"]
#[doc(alias = "DENALI_CTL_152")]
pub type DenaliCtl152 = crate::Reg<denali_ctl_152::DenaliCtl152Spec>;
#[doc = ""]
pub mod denali_ctl_152;
#[doc = "DENALI_CTL_153 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_153::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_153::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_153`]
module"]
#[doc(alias = "DENALI_CTL_153")]
pub type DenaliCtl153 = crate::Reg<denali_ctl_153::DenaliCtl153Spec>;
#[doc = ""]
pub mod denali_ctl_153;
#[doc = "DENALI_CTL_154 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_154::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_154::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_154`]
module"]
#[doc(alias = "DENALI_CTL_154")]
pub type DenaliCtl154 = crate::Reg<denali_ctl_154::DenaliCtl154Spec>;
#[doc = ""]
pub mod denali_ctl_154;
#[doc = "DENALI_CTL_155 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_155::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_155::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_155`]
module"]
#[doc(alias = "DENALI_CTL_155")]
pub type DenaliCtl155 = crate::Reg<denali_ctl_155::DenaliCtl155Spec>;
#[doc = ""]
pub mod denali_ctl_155;
#[doc = "DENALI_CTL_156 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_156::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_156::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_156`]
module"]
#[doc(alias = "DENALI_CTL_156")]
pub type DenaliCtl156 = crate::Reg<denali_ctl_156::DenaliCtl156Spec>;
#[doc = ""]
pub mod denali_ctl_156;
#[doc = "DENALI_CTL_157 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_157::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_157::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_157`]
module"]
#[doc(alias = "DENALI_CTL_157")]
pub type DenaliCtl157 = crate::Reg<denali_ctl_157::DenaliCtl157Spec>;
#[doc = ""]
pub mod denali_ctl_157;
#[doc = "DENALI_CTL_158 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_158::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_158::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_158`]
module"]
#[doc(alias = "DENALI_CTL_158")]
pub type DenaliCtl158 = crate::Reg<denali_ctl_158::DenaliCtl158Spec>;
#[doc = ""]
pub mod denali_ctl_158;
#[doc = "DENALI_CTL_159 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_159::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_159::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_159`]
module"]
#[doc(alias = "DENALI_CTL_159")]
pub type DenaliCtl159 = crate::Reg<denali_ctl_159::DenaliCtl159Spec>;
#[doc = ""]
pub mod denali_ctl_159;
#[doc = "DENALI_CTL_160 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_160::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_160::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_160`]
module"]
#[doc(alias = "DENALI_CTL_160")]
pub type DenaliCtl160 = crate::Reg<denali_ctl_160::DenaliCtl160Spec>;
#[doc = ""]
pub mod denali_ctl_160;
#[doc = "DENALI_CTL_161 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_161::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_161::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_161`]
module"]
#[doc(alias = "DENALI_CTL_161")]
pub type DenaliCtl161 = crate::Reg<denali_ctl_161::DenaliCtl161Spec>;
#[doc = ""]
pub mod denali_ctl_161;
#[doc = "DENALI_CTL_162 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_162::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_162::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_162`]
module"]
#[doc(alias = "DENALI_CTL_162")]
pub type DenaliCtl162 = crate::Reg<denali_ctl_162::DenaliCtl162Spec>;
#[doc = ""]
pub mod denali_ctl_162;
#[doc = "DENALI_CTL_163 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_163::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_163::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_163`]
module"]
#[doc(alias = "DENALI_CTL_163")]
pub type DenaliCtl163 = crate::Reg<denali_ctl_163::DenaliCtl163Spec>;
#[doc = ""]
pub mod denali_ctl_163;
#[doc = "DENALI_CTL_164 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_164::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_164::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_164`]
module"]
#[doc(alias = "DENALI_CTL_164")]
pub type DenaliCtl164 = crate::Reg<denali_ctl_164::DenaliCtl164Spec>;
#[doc = ""]
pub mod denali_ctl_164;
#[doc = "DENALI_CTL_165 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_165::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_165::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_165`]
module"]
#[doc(alias = "DENALI_CTL_165")]
pub type DenaliCtl165 = crate::Reg<denali_ctl_165::DenaliCtl165Spec>;
#[doc = ""]
pub mod denali_ctl_165;
#[doc = "DENALI_CTL_166 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_166::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_166::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_166`]
module"]
#[doc(alias = "DENALI_CTL_166")]
pub type DenaliCtl166 = crate::Reg<denali_ctl_166::DenaliCtl166Spec>;
#[doc = ""]
pub mod denali_ctl_166;
#[doc = "DENALI_CTL_167 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_167::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_167::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_167`]
module"]
#[doc(alias = "DENALI_CTL_167")]
pub type DenaliCtl167 = crate::Reg<denali_ctl_167::DenaliCtl167Spec>;
#[doc = ""]
pub mod denali_ctl_167;
#[doc = "DENALI_CTL_168 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_168::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_168::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_168`]
module"]
#[doc(alias = "DENALI_CTL_168")]
pub type DenaliCtl168 = crate::Reg<denali_ctl_168::DenaliCtl168Spec>;
#[doc = ""]
pub mod denali_ctl_168;
#[doc = "DENALI_CTL_169 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_169::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_169::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_169`]
module"]
#[doc(alias = "DENALI_CTL_169")]
pub type DenaliCtl169 = crate::Reg<denali_ctl_169::DenaliCtl169Spec>;
#[doc = ""]
pub mod denali_ctl_169;
#[doc = "DENALI_CTL_170 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_170::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_170::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_170`]
module"]
#[doc(alias = "DENALI_CTL_170")]
pub type DenaliCtl170 = crate::Reg<denali_ctl_170::DenaliCtl170Spec>;
#[doc = ""]
pub mod denali_ctl_170;
#[doc = "DENALI_CTL_171 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_171::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_171::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_171`]
module"]
#[doc(alias = "DENALI_CTL_171")]
pub type DenaliCtl171 = crate::Reg<denali_ctl_171::DenaliCtl171Spec>;
#[doc = ""]
pub mod denali_ctl_171;
#[doc = "DENALI_CTL_172 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_172::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_172::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_172`]
module"]
#[doc(alias = "DENALI_CTL_172")]
pub type DenaliCtl172 = crate::Reg<denali_ctl_172::DenaliCtl172Spec>;
#[doc = ""]
pub mod denali_ctl_172;
#[doc = "DENALI_CTL_173 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_173::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_173::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_173`]
module"]
#[doc(alias = "DENALI_CTL_173")]
pub type DenaliCtl173 = crate::Reg<denali_ctl_173::DenaliCtl173Spec>;
#[doc = ""]
pub mod denali_ctl_173;
#[doc = "DENALI_CTL_174 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_174::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_174::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_174`]
module"]
#[doc(alias = "DENALI_CTL_174")]
pub type DenaliCtl174 = crate::Reg<denali_ctl_174::DenaliCtl174Spec>;
#[doc = ""]
pub mod denali_ctl_174;
#[doc = "DENALI_CTL_175 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_175::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_175::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_175`]
module"]
#[doc(alias = "DENALI_CTL_175")]
pub type DenaliCtl175 = crate::Reg<denali_ctl_175::DenaliCtl175Spec>;
#[doc = ""]
pub mod denali_ctl_175;
#[doc = "DENALI_CTL_176 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_176::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_176::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_176`]
module"]
#[doc(alias = "DENALI_CTL_176")]
pub type DenaliCtl176 = crate::Reg<denali_ctl_176::DenaliCtl176Spec>;
#[doc = ""]
pub mod denali_ctl_176;
#[doc = "DENALI_CTL_177 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_177::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_177::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_177`]
module"]
#[doc(alias = "DENALI_CTL_177")]
pub type DenaliCtl177 = crate::Reg<denali_ctl_177::DenaliCtl177Spec>;
#[doc = ""]
pub mod denali_ctl_177;
#[doc = "DENALI_CTL_178 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_178::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_178::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_178`]
module"]
#[doc(alias = "DENALI_CTL_178")]
pub type DenaliCtl178 = crate::Reg<denali_ctl_178::DenaliCtl178Spec>;
#[doc = ""]
pub mod denali_ctl_178;
#[doc = "DENALI_CTL_179 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_179::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_179::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_179`]
module"]
#[doc(alias = "DENALI_CTL_179")]
pub type DenaliCtl179 = crate::Reg<denali_ctl_179::DenaliCtl179Spec>;
#[doc = ""]
pub mod denali_ctl_179;
#[doc = "DENALI_CTL_180 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_180::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_180::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_180`]
module"]
#[doc(alias = "DENALI_CTL_180")]
pub type DenaliCtl180 = crate::Reg<denali_ctl_180::DenaliCtl180Spec>;
#[doc = ""]
pub mod denali_ctl_180;
#[doc = "DENALI_CTL_181 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_181::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_181::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_181`]
module"]
#[doc(alias = "DENALI_CTL_181")]
pub type DenaliCtl181 = crate::Reg<denali_ctl_181::DenaliCtl181Spec>;
#[doc = ""]
pub mod denali_ctl_181;
#[doc = "DENALI_CTL_182 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_182::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_182::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_182`]
module"]
#[doc(alias = "DENALI_CTL_182")]
pub type DenaliCtl182 = crate::Reg<denali_ctl_182::DenaliCtl182Spec>;
#[doc = ""]
pub mod denali_ctl_182;
#[doc = "DENALI_CTL_183 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_183::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_183::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_183`]
module"]
#[doc(alias = "DENALI_CTL_183")]
pub type DenaliCtl183 = crate::Reg<denali_ctl_183::DenaliCtl183Spec>;
#[doc = ""]
pub mod denali_ctl_183;
#[doc = "DENALI_CTL_184 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_184::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_184::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_184`]
module"]
#[doc(alias = "DENALI_CTL_184")]
pub type DenaliCtl184 = crate::Reg<denali_ctl_184::DenaliCtl184Spec>;
#[doc = ""]
pub mod denali_ctl_184;
#[doc = "DENALI_CTL_185 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_185::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_185::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_185`]
module"]
#[doc(alias = "DENALI_CTL_185")]
pub type DenaliCtl185 = crate::Reg<denali_ctl_185::DenaliCtl185Spec>;
#[doc = ""]
pub mod denali_ctl_185;
#[doc = "DENALI_CTL_186 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_186::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_186::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_186`]
module"]
#[doc(alias = "DENALI_CTL_186")]
pub type DenaliCtl186 = crate::Reg<denali_ctl_186::DenaliCtl186Spec>;
#[doc = ""]
pub mod denali_ctl_186;
#[doc = "DENALI_CTL_187 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_187::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_187::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_187`]
module"]
#[doc(alias = "DENALI_CTL_187")]
pub type DenaliCtl187 = crate::Reg<denali_ctl_187::DenaliCtl187Spec>;
#[doc = ""]
pub mod denali_ctl_187;
#[doc = "DENALI_CTL_188 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_188::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_188::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_188`]
module"]
#[doc(alias = "DENALI_CTL_188")]
pub type DenaliCtl188 = crate::Reg<denali_ctl_188::DenaliCtl188Spec>;
#[doc = ""]
pub mod denali_ctl_188;
#[doc = "DENALI_CTL_189 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_189::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_189::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_189`]
module"]
#[doc(alias = "DENALI_CTL_189")]
pub type DenaliCtl189 = crate::Reg<denali_ctl_189::DenaliCtl189Spec>;
#[doc = ""]
pub mod denali_ctl_189;
#[doc = "DENALI_CTL_190 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_190::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_190::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_190`]
module"]
#[doc(alias = "DENALI_CTL_190")]
pub type DenaliCtl190 = crate::Reg<denali_ctl_190::DenaliCtl190Spec>;
#[doc = ""]
pub mod denali_ctl_190;
#[doc = "DENALI_CTL_191 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_191::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_191::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_191`]
module"]
#[doc(alias = "DENALI_CTL_191")]
pub type DenaliCtl191 = crate::Reg<denali_ctl_191::DenaliCtl191Spec>;
#[doc = ""]
pub mod denali_ctl_191;
#[doc = "DENALI_CTL_192 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_192::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_192::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_192`]
module"]
#[doc(alias = "DENALI_CTL_192")]
pub type DenaliCtl192 = crate::Reg<denali_ctl_192::DenaliCtl192Spec>;
#[doc = ""]
pub mod denali_ctl_192;
#[doc = "DENALI_CTL_193 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_193::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_193::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_193`]
module"]
#[doc(alias = "DENALI_CTL_193")]
pub type DenaliCtl193 = crate::Reg<denali_ctl_193::DenaliCtl193Spec>;
#[doc = ""]
pub mod denali_ctl_193;
#[doc = "DENALI_CTL_194 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_194::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_194::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_194`]
module"]
#[doc(alias = "DENALI_CTL_194")]
pub type DenaliCtl194 = crate::Reg<denali_ctl_194::DenaliCtl194Spec>;
#[doc = ""]
pub mod denali_ctl_194;
#[doc = "DENALI_CTL_195 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_195::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_195::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_195`]
module"]
#[doc(alias = "DENALI_CTL_195")]
pub type DenaliCtl195 = crate::Reg<denali_ctl_195::DenaliCtl195Spec>;
#[doc = ""]
pub mod denali_ctl_195;
#[doc = "DENALI_CTL_196 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_196::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_196::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_196`]
module"]
#[doc(alias = "DENALI_CTL_196")]
pub type DenaliCtl196 = crate::Reg<denali_ctl_196::DenaliCtl196Spec>;
#[doc = ""]
pub mod denali_ctl_196;
#[doc = "DENALI_CTL_197 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_197::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_197::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_197`]
module"]
#[doc(alias = "DENALI_CTL_197")]
pub type DenaliCtl197 = crate::Reg<denali_ctl_197::DenaliCtl197Spec>;
#[doc = ""]
pub mod denali_ctl_197;
#[doc = "DENALI_CTL_198 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_198::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_198::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_198`]
module"]
#[doc(alias = "DENALI_CTL_198")]
pub type DenaliCtl198 = crate::Reg<denali_ctl_198::DenaliCtl198Spec>;
#[doc = ""]
pub mod denali_ctl_198;
#[doc = "DENALI_CTL_199 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_199::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_199::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_199`]
module"]
#[doc(alias = "DENALI_CTL_199")]
pub type DenaliCtl199 = crate::Reg<denali_ctl_199::DenaliCtl199Spec>;
#[doc = ""]
pub mod denali_ctl_199;
#[doc = "DENALI_CTL_200 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_200::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_200::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_200`]
module"]
#[doc(alias = "DENALI_CTL_200")]
pub type DenaliCtl200 = crate::Reg<denali_ctl_200::DenaliCtl200Spec>;
#[doc = ""]
pub mod denali_ctl_200;
#[doc = "DENALI_CTL_201 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_201::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_201::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_201`]
module"]
#[doc(alias = "DENALI_CTL_201")]
pub type DenaliCtl201 = crate::Reg<denali_ctl_201::DenaliCtl201Spec>;
#[doc = ""]
pub mod denali_ctl_201;
#[doc = "DENALI_CTL_202 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_202::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_202`]
module"]
#[doc(alias = "DENALI_CTL_202")]
pub type DenaliCtl202 = crate::Reg<denali_ctl_202::DenaliCtl202Spec>;
#[doc = ""]
pub mod denali_ctl_202;
#[doc = "DENALI_CTL_203 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_203::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_203`]
module"]
#[doc(alias = "DENALI_CTL_203")]
pub type DenaliCtl203 = crate::Reg<denali_ctl_203::DenaliCtl203Spec>;
#[doc = ""]
pub mod denali_ctl_203;
#[doc = "DENALI_CTL_204 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_204::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_204`]
module"]
#[doc(alias = "DENALI_CTL_204")]
pub type DenaliCtl204 = crate::Reg<denali_ctl_204::DenaliCtl204Spec>;
#[doc = ""]
pub mod denali_ctl_204;
#[doc = "DENALI_CTL_205 (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_205::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_205`]
module"]
#[doc(alias = "DENALI_CTL_205")]
pub type DenaliCtl205 = crate::Reg<denali_ctl_205::DenaliCtl205Spec>;
#[doc = ""]
pub mod denali_ctl_205;
#[doc = "DENALI_CTL_206 (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_206::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_206`]
module"]
#[doc(alias = "DENALI_CTL_206")]
pub type DenaliCtl206 = crate::Reg<denali_ctl_206::DenaliCtl206Spec>;
#[doc = ""]
pub mod denali_ctl_206;
#[doc = "DENALI_CTL_207 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_207::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_207::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_207`]
module"]
#[doc(alias = "DENALI_CTL_207")]
pub type DenaliCtl207 = crate::Reg<denali_ctl_207::DenaliCtl207Spec>;
#[doc = ""]
pub mod denali_ctl_207;
#[doc = "DENALI_CTL_208 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_208::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_208::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_208`]
module"]
#[doc(alias = "DENALI_CTL_208")]
pub type DenaliCtl208 = crate::Reg<denali_ctl_208::DenaliCtl208Spec>;
#[doc = ""]
pub mod denali_ctl_208;
#[doc = "DENALI_CTL_209 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_209::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_209`]
module"]
#[doc(alias = "DENALI_CTL_209")]
pub type DenaliCtl209 = crate::Reg<denali_ctl_209::DenaliCtl209Spec>;
#[doc = ""]
pub mod denali_ctl_209;
#[doc = "DENALI_CTL_210 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_210::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_210`]
module"]
#[doc(alias = "DENALI_CTL_210")]
pub type DenaliCtl210 = crate::Reg<denali_ctl_210::DenaliCtl210Spec>;
#[doc = ""]
pub mod denali_ctl_210;
#[doc = "DENALI_CTL_211 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_211::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_211::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_211`]
module"]
#[doc(alias = "DENALI_CTL_211")]
pub type DenaliCtl211 = crate::Reg<denali_ctl_211::DenaliCtl211Spec>;
#[doc = ""]
pub mod denali_ctl_211;
#[doc = "DENALI_CTL_212 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_212::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_212::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_212`]
module"]
#[doc(alias = "DENALI_CTL_212")]
pub type DenaliCtl212 = crate::Reg<denali_ctl_212::DenaliCtl212Spec>;
#[doc = ""]
pub mod denali_ctl_212;
#[doc = "DENALI_CTL_213 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_213::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_213::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_213`]
module"]
#[doc(alias = "DENALI_CTL_213")]
pub type DenaliCtl213 = crate::Reg<denali_ctl_213::DenaliCtl213Spec>;
#[doc = ""]
pub mod denali_ctl_213;
#[doc = "DENALI_CTL_214 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_214::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_214::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_214`]
module"]
#[doc(alias = "DENALI_CTL_214")]
pub type DenaliCtl214 = crate::Reg<denali_ctl_214::DenaliCtl214Spec>;
#[doc = ""]
pub mod denali_ctl_214;
#[doc = "DENALI_CTL_215 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_215::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_215::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_215`]
module"]
#[doc(alias = "DENALI_CTL_215")]
pub type DenaliCtl215 = crate::Reg<denali_ctl_215::DenaliCtl215Spec>;
#[doc = ""]
pub mod denali_ctl_215;
#[doc = "DENALI_CTL_216 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_216::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_216::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_216`]
module"]
#[doc(alias = "DENALI_CTL_216")]
pub type DenaliCtl216 = crate::Reg<denali_ctl_216::DenaliCtl216Spec>;
#[doc = ""]
pub mod denali_ctl_216;
#[doc = "DENALI_CTL_217 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_217::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_217::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_217`]
module"]
#[doc(alias = "DENALI_CTL_217")]
pub type DenaliCtl217 = crate::Reg<denali_ctl_217::DenaliCtl217Spec>;
#[doc = ""]
pub mod denali_ctl_217;
#[doc = "DENALI_CTL_218 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_218::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_218::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_218`]
module"]
#[doc(alias = "DENALI_CTL_218")]
pub type DenaliCtl218 = crate::Reg<denali_ctl_218::DenaliCtl218Spec>;
#[doc = ""]
pub mod denali_ctl_218;
#[doc = "DENALI_CTL_219 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_219::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_219::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_219`]
module"]
#[doc(alias = "DENALI_CTL_219")]
pub type DenaliCtl219 = crate::Reg<denali_ctl_219::DenaliCtl219Spec>;
#[doc = ""]
pub mod denali_ctl_219;
#[doc = "DENALI_CTL_220 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_220::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_220::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_220`]
module"]
#[doc(alias = "DENALI_CTL_220")]
pub type DenaliCtl220 = crate::Reg<denali_ctl_220::DenaliCtl220Spec>;
#[doc = ""]
pub mod denali_ctl_220;
#[doc = "DENALI_CTL_221 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_221::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_221::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_221`]
module"]
#[doc(alias = "DENALI_CTL_221")]
pub type DenaliCtl221 = crate::Reg<denali_ctl_221::DenaliCtl221Spec>;
#[doc = ""]
pub mod denali_ctl_221;
#[doc = "DENALI_CTL_222 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_222::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_222::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_222`]
module"]
#[doc(alias = "DENALI_CTL_222")]
pub type DenaliCtl222 = crate::Reg<denali_ctl_222::DenaliCtl222Spec>;
#[doc = ""]
pub mod denali_ctl_222;
#[doc = "DENALI_CTL_223 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_223::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_223::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_223`]
module"]
#[doc(alias = "DENALI_CTL_223")]
pub type DenaliCtl223 = crate::Reg<denali_ctl_223::DenaliCtl223Spec>;
#[doc = ""]
pub mod denali_ctl_223;
#[doc = "DENALI_CTL_224 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_224::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_224`]
module"]
#[doc(alias = "DENALI_CTL_224")]
pub type DenaliCtl224 = crate::Reg<denali_ctl_224::DenaliCtl224Spec>;
#[doc = ""]
pub mod denali_ctl_224;
#[doc = "DENALI_CTL_225 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_225::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_225::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_225`]
module"]
#[doc(alias = "DENALI_CTL_225")]
pub type DenaliCtl225 = crate::Reg<denali_ctl_225::DenaliCtl225Spec>;
#[doc = ""]
pub mod denali_ctl_225;
#[doc = "DENALI_CTL_226 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_226::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_226::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_226`]
module"]
#[doc(alias = "DENALI_CTL_226")]
pub type DenaliCtl226 = crate::Reg<denali_ctl_226::DenaliCtl226Spec>;
#[doc = ""]
pub mod denali_ctl_226;
#[doc = "DENALI_CTL_227 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_227::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_227::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_227`]
module"]
#[doc(alias = "DENALI_CTL_227")]
pub type DenaliCtl227 = crate::Reg<denali_ctl_227::DenaliCtl227Spec>;
#[doc = ""]
pub mod denali_ctl_227;
#[doc = "DENALI_CTL_228 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_228::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_228::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_228`]
module"]
#[doc(alias = "DENALI_CTL_228")]
pub type DenaliCtl228 = crate::Reg<denali_ctl_228::DenaliCtl228Spec>;
#[doc = ""]
pub mod denali_ctl_228;
#[doc = "DENALI_CTL_229 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_229::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_229::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_229`]
module"]
#[doc(alias = "DENALI_CTL_229")]
pub type DenaliCtl229 = crate::Reg<denali_ctl_229::DenaliCtl229Spec>;
#[doc = ""]
pub mod denali_ctl_229;
#[doc = "DENALI_CTL_230 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_230::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_230::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_230`]
module"]
#[doc(alias = "DENALI_CTL_230")]
pub type DenaliCtl230 = crate::Reg<denali_ctl_230::DenaliCtl230Spec>;
#[doc = ""]
pub mod denali_ctl_230;
#[doc = "DENALI_CTL_231 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_231::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_231::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_231`]
module"]
#[doc(alias = "DENALI_CTL_231")]
pub type DenaliCtl231 = crate::Reg<denali_ctl_231::DenaliCtl231Spec>;
#[doc = ""]
pub mod denali_ctl_231;
#[doc = "DENALI_CTL_232 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_232::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_232::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_232`]
module"]
#[doc(alias = "DENALI_CTL_232")]
pub type DenaliCtl232 = crate::Reg<denali_ctl_232::DenaliCtl232Spec>;
#[doc = ""]
pub mod denali_ctl_232;
#[doc = "DENALI_CTL_233 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_233::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_233::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_233`]
module"]
#[doc(alias = "DENALI_CTL_233")]
pub type DenaliCtl233 = crate::Reg<denali_ctl_233::DenaliCtl233Spec>;
#[doc = ""]
pub mod denali_ctl_233;
#[doc = "DENALI_CTL_234 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_234::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_234::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_234`]
module"]
#[doc(alias = "DENALI_CTL_234")]
pub type DenaliCtl234 = crate::Reg<denali_ctl_234::DenaliCtl234Spec>;
#[doc = ""]
pub mod denali_ctl_234;
#[doc = "DENALI_CTL_235 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_235::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_235::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_235`]
module"]
#[doc(alias = "DENALI_CTL_235")]
pub type DenaliCtl235 = crate::Reg<denali_ctl_235::DenaliCtl235Spec>;
#[doc = ""]
pub mod denali_ctl_235;
#[doc = "DENALI_CTL_236 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_236::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_236::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_236`]
module"]
#[doc(alias = "DENALI_CTL_236")]
pub type DenaliCtl236 = crate::Reg<denali_ctl_236::DenaliCtl236Spec>;
#[doc = ""]
pub mod denali_ctl_236;
#[doc = "DENALI_CTL_237 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_237::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_237::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_237`]
module"]
#[doc(alias = "DENALI_CTL_237")]
pub type DenaliCtl237 = crate::Reg<denali_ctl_237::DenaliCtl237Spec>;
#[doc = ""]
pub mod denali_ctl_237;
#[doc = "DENALI_CTL_238 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_238::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_238::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_238`]
module"]
#[doc(alias = "DENALI_CTL_238")]
pub type DenaliCtl238 = crate::Reg<denali_ctl_238::DenaliCtl238Spec>;
#[doc = ""]
pub mod denali_ctl_238;
#[doc = "DENALI_CTL_239 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_239::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_239::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_239`]
module"]
#[doc(alias = "DENALI_CTL_239")]
pub type DenaliCtl239 = crate::Reg<denali_ctl_239::DenaliCtl239Spec>;
#[doc = ""]
pub mod denali_ctl_239;
#[doc = "DENALI_CTL_240 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_240::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_240::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_240`]
module"]
#[doc(alias = "DENALI_CTL_240")]
pub type DenaliCtl240 = crate::Reg<denali_ctl_240::DenaliCtl240Spec>;
#[doc = ""]
pub mod denali_ctl_240;
#[doc = "DENALI_CTL_241 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_241::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_241::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_241`]
module"]
#[doc(alias = "DENALI_CTL_241")]
pub type DenaliCtl241 = crate::Reg<denali_ctl_241::DenaliCtl241Spec>;
#[doc = ""]
pub mod denali_ctl_241;
#[doc = "DENALI_CTL_242 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_242::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_242::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_242`]
module"]
#[doc(alias = "DENALI_CTL_242")]
pub type DenaliCtl242 = crate::Reg<denali_ctl_242::DenaliCtl242Spec>;
#[doc = ""]
pub mod denali_ctl_242;
#[doc = "DENALI_CTL_243 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_243::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_243::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_243`]
module"]
#[doc(alias = "DENALI_CTL_243")]
pub type DenaliCtl243 = crate::Reg<denali_ctl_243::DenaliCtl243Spec>;
#[doc = ""]
pub mod denali_ctl_243;
#[doc = "DENALI_CTL_244 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_244::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_244::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_244`]
module"]
#[doc(alias = "DENALI_CTL_244")]
pub type DenaliCtl244 = crate::Reg<denali_ctl_244::DenaliCtl244Spec>;
#[doc = ""]
pub mod denali_ctl_244;
#[doc = "DENALI_CTL_245 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_245::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_245::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_245`]
module"]
#[doc(alias = "DENALI_CTL_245")]
pub type DenaliCtl245 = crate::Reg<denali_ctl_245::DenaliCtl245Spec>;
#[doc = ""]
pub mod denali_ctl_245;
#[doc = "DENALI_CTL_246 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_246::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_246::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_246`]
module"]
#[doc(alias = "DENALI_CTL_246")]
pub type DenaliCtl246 = crate::Reg<denali_ctl_246::DenaliCtl246Spec>;
#[doc = ""]
pub mod denali_ctl_246;
#[doc = "DENALI_CTL_247 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_247::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_247::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_247`]
module"]
#[doc(alias = "DENALI_CTL_247")]
pub type DenaliCtl247 = crate::Reg<denali_ctl_247::DenaliCtl247Spec>;
#[doc = ""]
pub mod denali_ctl_247;
#[doc = "DENALI_CTL_248 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_248::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_248::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_248`]
module"]
#[doc(alias = "DENALI_CTL_248")]
pub type DenaliCtl248 = crate::Reg<denali_ctl_248::DenaliCtl248Spec>;
#[doc = ""]
pub mod denali_ctl_248;
#[doc = "DENALI_CTL_249 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_249::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_249::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_249`]
module"]
#[doc(alias = "DENALI_CTL_249")]
pub type DenaliCtl249 = crate::Reg<denali_ctl_249::DenaliCtl249Spec>;
#[doc = ""]
pub mod denali_ctl_249;
#[doc = "DENALI_CTL_250 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_250::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_250::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_250`]
module"]
#[doc(alias = "DENALI_CTL_250")]
pub type DenaliCtl250 = crate::Reg<denali_ctl_250::DenaliCtl250Spec>;
#[doc = ""]
pub mod denali_ctl_250;
#[doc = "DENALI_CTL_251 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_251::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_251::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_251`]
module"]
#[doc(alias = "DENALI_CTL_251")]
pub type DenaliCtl251 = crate::Reg<denali_ctl_251::DenaliCtl251Spec>;
#[doc = ""]
pub mod denali_ctl_251;
#[doc = "DENALI_CTL_252 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_252::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_252::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_252`]
module"]
#[doc(alias = "DENALI_CTL_252")]
pub type DenaliCtl252 = crate::Reg<denali_ctl_252::DenaliCtl252Spec>;
#[doc = ""]
pub mod denali_ctl_252;
#[doc = "DENALI_CTL_253 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_253::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_253::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_253`]
module"]
#[doc(alias = "DENALI_CTL_253")]
pub type DenaliCtl253 = crate::Reg<denali_ctl_253::DenaliCtl253Spec>;
#[doc = ""]
pub mod denali_ctl_253;
#[doc = "DENALI_CTL_254 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_254::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_254::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_254`]
module"]
#[doc(alias = "DENALI_CTL_254")]
pub type DenaliCtl254 = crate::Reg<denali_ctl_254::DenaliCtl254Spec>;
#[doc = ""]
pub mod denali_ctl_254;
#[doc = "DENALI_CTL_255 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_255::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_255::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_255`]
module"]
#[doc(alias = "DENALI_CTL_255")]
pub type DenaliCtl255 = crate::Reg<denali_ctl_255::DenaliCtl255Spec>;
#[doc = ""]
pub mod denali_ctl_255;
#[doc = "DENALI_CTL_256 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_256::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_256::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_256`]
module"]
#[doc(alias = "DENALI_CTL_256")]
pub type DenaliCtl256 = crate::Reg<denali_ctl_256::DenaliCtl256Spec>;
#[doc = ""]
pub mod denali_ctl_256;
#[doc = "DENALI_CTL_257 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_257::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_257::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_257`]
module"]
#[doc(alias = "DENALI_CTL_257")]
pub type DenaliCtl257 = crate::Reg<denali_ctl_257::DenaliCtl257Spec>;
#[doc = ""]
pub mod denali_ctl_257;
#[doc = "DENALI_CTL_258 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_258::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_258::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_258`]
module"]
#[doc(alias = "DENALI_CTL_258")]
pub type DenaliCtl258 = crate::Reg<denali_ctl_258::DenaliCtl258Spec>;
#[doc = ""]
pub mod denali_ctl_258;
#[doc = "DENALI_CTL_259 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_259::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_259::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_259`]
module"]
#[doc(alias = "DENALI_CTL_259")]
pub type DenaliCtl259 = crate::Reg<denali_ctl_259::DenaliCtl259Spec>;
#[doc = ""]
pub mod denali_ctl_259;
#[doc = "DENALI_CTL_260 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_260::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_260::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_260`]
module"]
#[doc(alias = "DENALI_CTL_260")]
pub type DenaliCtl260 = crate::Reg<denali_ctl_260::DenaliCtl260Spec>;
#[doc = ""]
pub mod denali_ctl_260;
#[doc = "DENALI_CTL_261 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_261::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_261::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_261`]
module"]
#[doc(alias = "DENALI_CTL_261")]
pub type DenaliCtl261 = crate::Reg<denali_ctl_261::DenaliCtl261Spec>;
#[doc = ""]
pub mod denali_ctl_261;
#[doc = "DENALI_CTL_262 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_262::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_262::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_262`]
module"]
#[doc(alias = "DENALI_CTL_262")]
pub type DenaliCtl262 = crate::Reg<denali_ctl_262::DenaliCtl262Spec>;
#[doc = ""]
pub mod denali_ctl_262;
#[doc = "DENALI_CTL_263 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_263::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_263::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_263`]
module"]
#[doc(alias = "DENALI_CTL_263")]
pub type DenaliCtl263 = crate::Reg<denali_ctl_263::DenaliCtl263Spec>;
#[doc = ""]
pub mod denali_ctl_263;
#[doc = "DENALI_CTL_264 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_264::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_264::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_264`]
module"]
#[doc(alias = "DENALI_CTL_264")]
pub type DenaliCtl264 = crate::Reg<denali_ctl_264::DenaliCtl264Spec>;
#[doc = ""]
pub mod denali_ctl_264;
#[doc = "DENALI_CTL_265 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_265::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_265::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_265`]
module"]
#[doc(alias = "DENALI_CTL_265")]
pub type DenaliCtl265 = crate::Reg<denali_ctl_265::DenaliCtl265Spec>;
#[doc = ""]
pub mod denali_ctl_265;
#[doc = "DENALI_CTL_266 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_266::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_266::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_266`]
module"]
#[doc(alias = "DENALI_CTL_266")]
pub type DenaliCtl266 = crate::Reg<denali_ctl_266::DenaliCtl266Spec>;
#[doc = ""]
pub mod denali_ctl_266;
#[doc = "DENALI_CTL_267 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_267::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_267::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_267`]
module"]
#[doc(alias = "DENALI_CTL_267")]
pub type DenaliCtl267 = crate::Reg<denali_ctl_267::DenaliCtl267Spec>;
#[doc = ""]
pub mod denali_ctl_267;
#[doc = "DENALI_CTL_268 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_268::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_268::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_268`]
module"]
#[doc(alias = "DENALI_CTL_268")]
pub type DenaliCtl268 = crate::Reg<denali_ctl_268::DenaliCtl268Spec>;
#[doc = ""]
pub mod denali_ctl_268;
#[doc = "DENALI_CTL_269 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_269::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_269::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_269`]
module"]
#[doc(alias = "DENALI_CTL_269")]
pub type DenaliCtl269 = crate::Reg<denali_ctl_269::DenaliCtl269Spec>;
#[doc = ""]
pub mod denali_ctl_269;
#[doc = "DENALI_CTL_270 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_270::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_270::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_270`]
module"]
#[doc(alias = "DENALI_CTL_270")]
pub type DenaliCtl270 = crate::Reg<denali_ctl_270::DenaliCtl270Spec>;
#[doc = ""]
pub mod denali_ctl_270;
#[doc = "DENALI_CTL_271 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_271::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_271::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_271`]
module"]
#[doc(alias = "DENALI_CTL_271")]
pub type DenaliCtl271 = crate::Reg<denali_ctl_271::DenaliCtl271Spec>;
#[doc = ""]
pub mod denali_ctl_271;
#[doc = "DENALI_CTL_272 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_272::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_272::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_272`]
module"]
#[doc(alias = "DENALI_CTL_272")]
pub type DenaliCtl272 = crate::Reg<denali_ctl_272::DenaliCtl272Spec>;
#[doc = ""]
pub mod denali_ctl_272;
#[doc = "DENALI_CTL_273 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_273::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_273::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_273`]
module"]
#[doc(alias = "DENALI_CTL_273")]
pub type DenaliCtl273 = crate::Reg<denali_ctl_273::DenaliCtl273Spec>;
#[doc = ""]
pub mod denali_ctl_273;
#[doc = "DENALI_CTL_274 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_274::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_274::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_274`]
module"]
#[doc(alias = "DENALI_CTL_274")]
pub type DenaliCtl274 = crate::Reg<denali_ctl_274::DenaliCtl274Spec>;
#[doc = ""]
pub mod denali_ctl_274;
#[doc = "DENALI_CTL_275 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_275::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_275::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_275`]
module"]
#[doc(alias = "DENALI_CTL_275")]
pub type DenaliCtl275 = crate::Reg<denali_ctl_275::DenaliCtl275Spec>;
#[doc = ""]
pub mod denali_ctl_275;
#[doc = "DENALI_CTL_276 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_276::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_276::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_276`]
module"]
#[doc(alias = "DENALI_CTL_276")]
pub type DenaliCtl276 = crate::Reg<denali_ctl_276::DenaliCtl276Spec>;
#[doc = ""]
pub mod denali_ctl_276;
#[doc = "DENALI_CTL_277 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_277::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_277::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_277`]
module"]
#[doc(alias = "DENALI_CTL_277")]
pub type DenaliCtl277 = crate::Reg<denali_ctl_277::DenaliCtl277Spec>;
#[doc = ""]
pub mod denali_ctl_277;
#[doc = "DENALI_CTL_278 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_278::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_278::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_278`]
module"]
#[doc(alias = "DENALI_CTL_278")]
pub type DenaliCtl278 = crate::Reg<denali_ctl_278::DenaliCtl278Spec>;
#[doc = ""]
pub mod denali_ctl_278;
#[doc = "DENALI_CTL_279 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_279::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_279::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_279`]
module"]
#[doc(alias = "DENALI_CTL_279")]
pub type DenaliCtl279 = crate::Reg<denali_ctl_279::DenaliCtl279Spec>;
#[doc = ""]
pub mod denali_ctl_279;
#[doc = "DENALI_CTL_280 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_280::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_280::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_280`]
module"]
#[doc(alias = "DENALI_CTL_280")]
pub type DenaliCtl280 = crate::Reg<denali_ctl_280::DenaliCtl280Spec>;
#[doc = ""]
pub mod denali_ctl_280;
#[doc = "DENALI_CTL_281 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_281::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_281::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_281`]
module"]
#[doc(alias = "DENALI_CTL_281")]
pub type DenaliCtl281 = crate::Reg<denali_ctl_281::DenaliCtl281Spec>;
#[doc = ""]
pub mod denali_ctl_281;
#[doc = "DENALI_CTL_282 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_282::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_282::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_282`]
module"]
#[doc(alias = "DENALI_CTL_282")]
pub type DenaliCtl282 = crate::Reg<denali_ctl_282::DenaliCtl282Spec>;
#[doc = ""]
pub mod denali_ctl_282;
#[doc = "DENALI_CTL_283 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_283::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_283::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_283`]
module"]
#[doc(alias = "DENALI_CTL_283")]
pub type DenaliCtl283 = crate::Reg<denali_ctl_283::DenaliCtl283Spec>;
#[doc = ""]
pub mod denali_ctl_283;
#[doc = "DENALI_CTL_284 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_284::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_284::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_284`]
module"]
#[doc(alias = "DENALI_CTL_284")]
pub type DenaliCtl284 = crate::Reg<denali_ctl_284::DenaliCtl284Spec>;
#[doc = ""]
pub mod denali_ctl_284;
#[doc = "DENALI_CTL_285 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_285::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_285::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_285`]
module"]
#[doc(alias = "DENALI_CTL_285")]
pub type DenaliCtl285 = crate::Reg<denali_ctl_285::DenaliCtl285Spec>;
#[doc = ""]
pub mod denali_ctl_285;
#[doc = "DENALI_CTL_286 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_286::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_286::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_286`]
module"]
#[doc(alias = "DENALI_CTL_286")]
pub type DenaliCtl286 = crate::Reg<denali_ctl_286::DenaliCtl286Spec>;
#[doc = ""]
pub mod denali_ctl_286;
#[doc = "DENALI_CTL_287 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_287::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_287::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_287`]
module"]
#[doc(alias = "DENALI_CTL_287")]
pub type DenaliCtl287 = crate::Reg<denali_ctl_287::DenaliCtl287Spec>;
#[doc = ""]
pub mod denali_ctl_287;
#[doc = "DENALI_CTL_288 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_288::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_288::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_288`]
module"]
#[doc(alias = "DENALI_CTL_288")]
pub type DenaliCtl288 = crate::Reg<denali_ctl_288::DenaliCtl288Spec>;
#[doc = ""]
pub mod denali_ctl_288;
#[doc = "DENALI_CTL_289 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_289::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_289::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_289`]
module"]
#[doc(alias = "DENALI_CTL_289")]
pub type DenaliCtl289 = crate::Reg<denali_ctl_289::DenaliCtl289Spec>;
#[doc = ""]
pub mod denali_ctl_289;
#[doc = "DENALI_CTL_290 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_290::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_290::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_290`]
module"]
#[doc(alias = "DENALI_CTL_290")]
pub type DenaliCtl290 = crate::Reg<denali_ctl_290::DenaliCtl290Spec>;
#[doc = ""]
pub mod denali_ctl_290;
#[doc = "DENALI_CTL_291 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_291::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_291::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_291`]
module"]
#[doc(alias = "DENALI_CTL_291")]
pub type DenaliCtl291 = crate::Reg<denali_ctl_291::DenaliCtl291Spec>;
#[doc = ""]
pub mod denali_ctl_291;
#[doc = "DENALI_CTL_292 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_292::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_292::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_292`]
module"]
#[doc(alias = "DENALI_CTL_292")]
pub type DenaliCtl292 = crate::Reg<denali_ctl_292::DenaliCtl292Spec>;
#[doc = ""]
pub mod denali_ctl_292;
#[doc = "DENALI_CTL_293 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_293::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_293::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_293`]
module"]
#[doc(alias = "DENALI_CTL_293")]
pub type DenaliCtl293 = crate::Reg<denali_ctl_293::DenaliCtl293Spec>;
#[doc = ""]
pub mod denali_ctl_293;
#[doc = "DENALI_CTL_294 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_294::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_294::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_294`]
module"]
#[doc(alias = "DENALI_CTL_294")]
pub type DenaliCtl294 = crate::Reg<denali_ctl_294::DenaliCtl294Spec>;
#[doc = ""]
pub mod denali_ctl_294;
#[doc = "DENALI_CTL_295 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_295::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_295::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_295`]
module"]
#[doc(alias = "DENALI_CTL_295")]
pub type DenaliCtl295 = crate::Reg<denali_ctl_295::DenaliCtl295Spec>;
#[doc = ""]
pub mod denali_ctl_295;
#[doc = "DENALI_CTL_296 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_296::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_296::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_296`]
module"]
#[doc(alias = "DENALI_CTL_296")]
pub type DenaliCtl296 = crate::Reg<denali_ctl_296::DenaliCtl296Spec>;
#[doc = ""]
pub mod denali_ctl_296;
#[doc = "DENALI_CTL_297 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_297::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_297::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_297`]
module"]
#[doc(alias = "DENALI_CTL_297")]
pub type DenaliCtl297 = crate::Reg<denali_ctl_297::DenaliCtl297Spec>;
#[doc = ""]
pub mod denali_ctl_297;
#[doc = "DENALI_CTL_298 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_298::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_298::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_298`]
module"]
#[doc(alias = "DENALI_CTL_298")]
pub type DenaliCtl298 = crate::Reg<denali_ctl_298::DenaliCtl298Spec>;
#[doc = ""]
pub mod denali_ctl_298;
#[doc = "DENALI_CTL_299 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_299::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_299::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_299`]
module"]
#[doc(alias = "DENALI_CTL_299")]
pub type DenaliCtl299 = crate::Reg<denali_ctl_299::DenaliCtl299Spec>;
#[doc = ""]
pub mod denali_ctl_299;
#[doc = "DENALI_CTL_300 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_300::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_300::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_300`]
module"]
#[doc(alias = "DENALI_CTL_300")]
pub type DenaliCtl300 = crate::Reg<denali_ctl_300::DenaliCtl300Spec>;
#[doc = ""]
pub mod denali_ctl_300;
#[doc = "DENALI_CTL_301 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_301::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_301::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_301`]
module"]
#[doc(alias = "DENALI_CTL_301")]
pub type DenaliCtl301 = crate::Reg<denali_ctl_301::DenaliCtl301Spec>;
#[doc = ""]
pub mod denali_ctl_301;
#[doc = "DENALI_CTL_302 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_302::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_302::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_302`]
module"]
#[doc(alias = "DENALI_CTL_302")]
pub type DenaliCtl302 = crate::Reg<denali_ctl_302::DenaliCtl302Spec>;
#[doc = ""]
pub mod denali_ctl_302;
#[doc = "DENALI_CTL_303 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_303::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_303::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_303`]
module"]
#[doc(alias = "DENALI_CTL_303")]
pub type DenaliCtl303 = crate::Reg<denali_ctl_303::DenaliCtl303Spec>;
#[doc = ""]
pub mod denali_ctl_303;
#[doc = "DENALI_CTL_304 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_304::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_304::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_304`]
module"]
#[doc(alias = "DENALI_CTL_304")]
pub type DenaliCtl304 = crate::Reg<denali_ctl_304::DenaliCtl304Spec>;
#[doc = ""]
pub mod denali_ctl_304;
#[doc = "DENALI_CTL_305 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_305::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_305::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_305`]
module"]
#[doc(alias = "DENALI_CTL_305")]
pub type DenaliCtl305 = crate::Reg<denali_ctl_305::DenaliCtl305Spec>;
#[doc = ""]
pub mod denali_ctl_305;
#[doc = "DENALI_CTL_306 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_306::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_306::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_306`]
module"]
#[doc(alias = "DENALI_CTL_306")]
pub type DenaliCtl306 = crate::Reg<denali_ctl_306::DenaliCtl306Spec>;
#[doc = ""]
pub mod denali_ctl_306;
#[doc = "DENALI_CTL_307 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_307::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_307::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_307`]
module"]
#[doc(alias = "DENALI_CTL_307")]
pub type DenaliCtl307 = crate::Reg<denali_ctl_307::DenaliCtl307Spec>;
#[doc = ""]
pub mod denali_ctl_307;
#[doc = "DENALI_CTL_308 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_308::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_308::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_308`]
module"]
#[doc(alias = "DENALI_CTL_308")]
pub type DenaliCtl308 = crate::Reg<denali_ctl_308::DenaliCtl308Spec>;
#[doc = ""]
pub mod denali_ctl_308;
#[doc = "DENALI_CTL_309 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_309::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_309::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_309`]
module"]
#[doc(alias = "DENALI_CTL_309")]
pub type DenaliCtl309 = crate::Reg<denali_ctl_309::DenaliCtl309Spec>;
#[doc = ""]
pub mod denali_ctl_309;
#[doc = "DENALI_CTL_310 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_310::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_310::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_310`]
module"]
#[doc(alias = "DENALI_CTL_310")]
pub type DenaliCtl310 = crate::Reg<denali_ctl_310::DenaliCtl310Spec>;
#[doc = ""]
pub mod denali_ctl_310;
#[doc = "DENALI_CTL_311 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_311::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_311::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_311`]
module"]
#[doc(alias = "DENALI_CTL_311")]
pub type DenaliCtl311 = crate::Reg<denali_ctl_311::DenaliCtl311Spec>;
#[doc = ""]
pub mod denali_ctl_311;
#[doc = "DENALI_CTL_312 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_312::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_312::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_312`]
module"]
#[doc(alias = "DENALI_CTL_312")]
pub type DenaliCtl312 = crate::Reg<denali_ctl_312::DenaliCtl312Spec>;
#[doc = ""]
pub mod denali_ctl_312;
#[doc = "DENALI_CTL_313 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_313::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_313::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_313`]
module"]
#[doc(alias = "DENALI_CTL_313")]
pub type DenaliCtl313 = crate::Reg<denali_ctl_313::DenaliCtl313Spec>;
#[doc = ""]
pub mod denali_ctl_313;
#[doc = "DENALI_CTL_314 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_314::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_314::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_314`]
module"]
#[doc(alias = "DENALI_CTL_314")]
pub type DenaliCtl314 = crate::Reg<denali_ctl_314::DenaliCtl314Spec>;
#[doc = ""]
pub mod denali_ctl_314;
#[doc = "DENALI_CTL_315 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_315::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_315::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_315`]
module"]
#[doc(alias = "DENALI_CTL_315")]
pub type DenaliCtl315 = crate::Reg<denali_ctl_315::DenaliCtl315Spec>;
#[doc = ""]
pub mod denali_ctl_315;
#[doc = "DENALI_CTL_316 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_316::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_316::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_316`]
module"]
#[doc(alias = "DENALI_CTL_316")]
pub type DenaliCtl316 = crate::Reg<denali_ctl_316::DenaliCtl316Spec>;
#[doc = ""]
pub mod denali_ctl_316;
#[doc = "DENALI_CTL_317 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_317::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_317`]
module"]
#[doc(alias = "DENALI_CTL_317")]
pub type DenaliCtl317 = crate::Reg<denali_ctl_317::DenaliCtl317Spec>;
#[doc = ""]
pub mod denali_ctl_317;
#[doc = "DENALI_CTL_318 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_318::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_318::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_318`]
module"]
#[doc(alias = "DENALI_CTL_318")]
pub type DenaliCtl318 = crate::Reg<denali_ctl_318::DenaliCtl318Spec>;
#[doc = ""]
pub mod denali_ctl_318;
#[doc = "DENALI_CTL_319 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_319::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_319::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_319`]
module"]
#[doc(alias = "DENALI_CTL_319")]
pub type DenaliCtl319 = crate::Reg<denali_ctl_319::DenaliCtl319Spec>;
#[doc = ""]
pub mod denali_ctl_319;
#[doc = "DENALI_CTL_320 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_320::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_320::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_320`]
module"]
#[doc(alias = "DENALI_CTL_320")]
pub type DenaliCtl320 = crate::Reg<denali_ctl_320::DenaliCtl320Spec>;
#[doc = ""]
pub mod denali_ctl_320;
#[doc = "DENALI_CTL_321 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_321::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_321::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_321`]
module"]
#[doc(alias = "DENALI_CTL_321")]
pub type DenaliCtl321 = crate::Reg<denali_ctl_321::DenaliCtl321Spec>;
#[doc = ""]
pub mod denali_ctl_321;
#[doc = "DENALI_CTL_322 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_322::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_322::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_322`]
module"]
#[doc(alias = "DENALI_CTL_322")]
pub type DenaliCtl322 = crate::Reg<denali_ctl_322::DenaliCtl322Spec>;
#[doc = ""]
pub mod denali_ctl_322;
#[doc = "DENALI_CTL_323 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_323::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_323::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_323`]
module"]
#[doc(alias = "DENALI_CTL_323")]
pub type DenaliCtl323 = crate::Reg<denali_ctl_323::DenaliCtl323Spec>;
#[doc = ""]
pub mod denali_ctl_323;
#[doc = "DENALI_CTL_324 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_324::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_324::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_ctl_324`]
module"]
#[doc(alias = "DENALI_CTL_324")]
pub type DenaliCtl324 = crate::Reg<denali_ctl_324::DenaliCtl324Spec>;
#[doc = ""]
pub mod denali_ctl_324;
#[doc = "DENALI_PHY_00 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_00::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_00::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_00`]
module"]
#[doc(alias = "DENALI_PHY_00")]
pub type DenaliPhy00 = crate::Reg<denali_phy_00::DenaliPhy00Spec>;
#[doc = ""]
pub mod denali_phy_00;
#[doc = "DENALI_PHY_01 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_01`]
module"]
#[doc(alias = "DENALI_PHY_01")]
pub type DenaliPhy01 = crate::Reg<denali_phy_01::DenaliPhy01Spec>;
#[doc = ""]
pub mod denali_phy_01;
#[doc = "DENALI_PHY_02 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_02::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_02::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_02`]
module"]
#[doc(alias = "DENALI_PHY_02")]
pub type DenaliPhy02 = crate::Reg<denali_phy_02::DenaliPhy02Spec>;
#[doc = ""]
pub mod denali_phy_02;
#[doc = "DENALI_PHY_03 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_03::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_03::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_03`]
module"]
#[doc(alias = "DENALI_PHY_03")]
pub type DenaliPhy03 = crate::Reg<denali_phy_03::DenaliPhy03Spec>;
#[doc = ""]
pub mod denali_phy_03;
#[doc = "DENALI_PHY_04 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_04::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_04::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_04`]
module"]
#[doc(alias = "DENALI_PHY_04")]
pub type DenaliPhy04 = crate::Reg<denali_phy_04::DenaliPhy04Spec>;
#[doc = ""]
pub mod denali_phy_04;
#[doc = "DENALI_PHY_05 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_05::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_05::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_05`]
module"]
#[doc(alias = "DENALI_PHY_05")]
pub type DenaliPhy05 = crate::Reg<denali_phy_05::DenaliPhy05Spec>;
#[doc = ""]
pub mod denali_phy_05;
#[doc = "DENALI_PHY_06 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_06::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_06::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_06`]
module"]
#[doc(alias = "DENALI_PHY_06")]
pub type DenaliPhy06 = crate::Reg<denali_phy_06::DenaliPhy06Spec>;
#[doc = ""]
pub mod denali_phy_06;
#[doc = "DENALI_PHY_07 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_07::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_07::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_07`]
module"]
#[doc(alias = "DENALI_PHY_07")]
pub type DenaliPhy07 = crate::Reg<denali_phy_07::DenaliPhy07Spec>;
#[doc = ""]
pub mod denali_phy_07;
#[doc = "DENALI_PHY_08 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_08::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_08::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_08`]
module"]
#[doc(alias = "DENALI_PHY_08")]
pub type DenaliPhy08 = crate::Reg<denali_phy_08::DenaliPhy08Spec>;
#[doc = ""]
pub mod denali_phy_08;
#[doc = "DENALI_PHY_09 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_09::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_09::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_09`]
module"]
#[doc(alias = "DENALI_PHY_09")]
pub type DenaliPhy09 = crate::Reg<denali_phy_09::DenaliPhy09Spec>;
#[doc = ""]
pub mod denali_phy_09;
#[doc = "DENALI_PHY_10 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_10`]
module"]
#[doc(alias = "DENALI_PHY_10")]
pub type DenaliPhy10 = crate::Reg<denali_phy_10::DenaliPhy10Spec>;
#[doc = ""]
pub mod denali_phy_10;
#[doc = "DENALI_PHY_11 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_11`]
module"]
#[doc(alias = "DENALI_PHY_11")]
pub type DenaliPhy11 = crate::Reg<denali_phy_11::DenaliPhy11Spec>;
#[doc = ""]
pub mod denali_phy_11;
#[doc = "DENALI_PHY_12 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_12`]
module"]
#[doc(alias = "DENALI_PHY_12")]
pub type DenaliPhy12 = crate::Reg<denali_phy_12::DenaliPhy12Spec>;
#[doc = ""]
pub mod denali_phy_12;
#[doc = "DENALI_PHY_13 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_13`]
module"]
#[doc(alias = "DENALI_PHY_13")]
pub type DenaliPhy13 = crate::Reg<denali_phy_13::DenaliPhy13Spec>;
#[doc = ""]
pub mod denali_phy_13;
#[doc = "DENALI_PHY_14 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_14::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_14`]
module"]
#[doc(alias = "DENALI_PHY_14")]
pub type DenaliPhy14 = crate::Reg<denali_phy_14::DenaliPhy14Spec>;
#[doc = ""]
pub mod denali_phy_14;
#[doc = "DENALI_PHY_15 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_15`]
module"]
#[doc(alias = "DENALI_PHY_15")]
pub type DenaliPhy15 = crate::Reg<denali_phy_15::DenaliPhy15Spec>;
#[doc = ""]
pub mod denali_phy_15;
#[doc = "DENALI_PHY_16 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_16`]
module"]
#[doc(alias = "DENALI_PHY_16")]
pub type DenaliPhy16 = crate::Reg<denali_phy_16::DenaliPhy16Spec>;
#[doc = ""]
pub mod denali_phy_16;
#[doc = "DENALI_PHY_17 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_17`]
module"]
#[doc(alias = "DENALI_PHY_17")]
pub type DenaliPhy17 = crate::Reg<denali_phy_17::DenaliPhy17Spec>;
#[doc = ""]
pub mod denali_phy_17;
#[doc = "DENALI_PHY_18 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_18`]
module"]
#[doc(alias = "DENALI_PHY_18")]
pub type DenaliPhy18 = crate::Reg<denali_phy_18::DenaliPhy18Spec>;
#[doc = ""]
pub mod denali_phy_18;
#[doc = "DENALI_PHY_19 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_19`]
module"]
#[doc(alias = "DENALI_PHY_19")]
pub type DenaliPhy19 = crate::Reg<denali_phy_19::DenaliPhy19Spec>;
#[doc = ""]
pub mod denali_phy_19;
#[doc = "DENALI_PHY_20 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_20`]
module"]
#[doc(alias = "DENALI_PHY_20")]
pub type DenaliPhy20 = crate::Reg<denali_phy_20::DenaliPhy20Spec>;
#[doc = ""]
pub mod denali_phy_20;
#[doc = "DENALI_PHY_21 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_21`]
module"]
#[doc(alias = "DENALI_PHY_21")]
pub type DenaliPhy21 = crate::Reg<denali_phy_21::DenaliPhy21Spec>;
#[doc = ""]
pub mod denali_phy_21;
#[doc = "DENALI_PHY_22 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_22`]
module"]
#[doc(alias = "DENALI_PHY_22")]
pub type DenaliPhy22 = crate::Reg<denali_phy_22::DenaliPhy22Spec>;
#[doc = ""]
pub mod denali_phy_22;
#[doc = "DENALI_PHY_23 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_23`]
module"]
#[doc(alias = "DENALI_PHY_23")]
pub type DenaliPhy23 = crate::Reg<denali_phy_23::DenaliPhy23Spec>;
#[doc = ""]
pub mod denali_phy_23;
#[doc = "DENALI_PHY_24 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_24`]
module"]
#[doc(alias = "DENALI_PHY_24")]
pub type DenaliPhy24 = crate::Reg<denali_phy_24::DenaliPhy24Spec>;
#[doc = ""]
pub mod denali_phy_24;
#[doc = "DENALI_PHY_25 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_25`]
module"]
#[doc(alias = "DENALI_PHY_25")]
pub type DenaliPhy25 = crate::Reg<denali_phy_25::DenaliPhy25Spec>;
#[doc = ""]
pub mod denali_phy_25;
#[doc = "DENALI_PHY_26 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_26`]
module"]
#[doc(alias = "DENALI_PHY_26")]
pub type DenaliPhy26 = crate::Reg<denali_phy_26::DenaliPhy26Spec>;
#[doc = ""]
pub mod denali_phy_26;
#[doc = "DENALI_PHY_27 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_27`]
module"]
#[doc(alias = "DENALI_PHY_27")]
pub type DenaliPhy27 = crate::Reg<denali_phy_27::DenaliPhy27Spec>;
#[doc = ""]
pub mod denali_phy_27;
#[doc = "DENALI_PHY_28 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_28`]
module"]
#[doc(alias = "DENALI_PHY_28")]
pub type DenaliPhy28 = crate::Reg<denali_phy_28::DenaliPhy28Spec>;
#[doc = ""]
pub mod denali_phy_28;
#[doc = "DENALI_PHY_29 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_29`]
module"]
#[doc(alias = "DENALI_PHY_29")]
pub type DenaliPhy29 = crate::Reg<denali_phy_29::DenaliPhy29Spec>;
#[doc = ""]
pub mod denali_phy_29;
#[doc = "DENALI_PHY_30 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_30`]
module"]
#[doc(alias = "DENALI_PHY_30")]
pub type DenaliPhy30 = crate::Reg<denali_phy_30::DenaliPhy30Spec>;
#[doc = ""]
pub mod denali_phy_30;
#[doc = "DENALI_PHY_31 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_31`]
module"]
#[doc(alias = "DENALI_PHY_31")]
pub type DenaliPhy31 = crate::Reg<denali_phy_31::DenaliPhy31Spec>;
#[doc = ""]
pub mod denali_phy_31;
#[doc = "DENALI_PHY_32 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_32`]
module"]
#[doc(alias = "DENALI_PHY_32")]
pub type DenaliPhy32 = crate::Reg<denali_phy_32::DenaliPhy32Spec>;
#[doc = ""]
pub mod denali_phy_32;
#[doc = "DENALI_PHY_33 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_33::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_33`]
module"]
#[doc(alias = "DENALI_PHY_33")]
pub type DenaliPhy33 = crate::Reg<denali_phy_33::DenaliPhy33Spec>;
#[doc = ""]
pub mod denali_phy_33;
#[doc = "DENALI_PHY_34 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_34::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_34`]
module"]
#[doc(alias = "DENALI_PHY_34")]
pub type DenaliPhy34 = crate::Reg<denali_phy_34::DenaliPhy34Spec>;
#[doc = ""]
pub mod denali_phy_34;
#[doc = "DENALI_PHY_35 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_35::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_35`]
module"]
#[doc(alias = "DENALI_PHY_35")]
pub type DenaliPhy35 = crate::Reg<denali_phy_35::DenaliPhy35Spec>;
#[doc = ""]
pub mod denali_phy_35;
#[doc = "DENALI_PHY_36 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_36::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_36`]
module"]
#[doc(alias = "DENALI_PHY_36")]
pub type DenaliPhy36 = crate::Reg<denali_phy_36::DenaliPhy36Spec>;
#[doc = ""]
pub mod denali_phy_36;
#[doc = "DENALI_PHY_37 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_37::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_37`]
module"]
#[doc(alias = "DENALI_PHY_37")]
pub type DenaliPhy37 = crate::Reg<denali_phy_37::DenaliPhy37Spec>;
#[doc = ""]
pub mod denali_phy_37;
#[doc = "DENALI_PHY_38 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_38::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_38`]
module"]
#[doc(alias = "DENALI_PHY_38")]
pub type DenaliPhy38 = crate::Reg<denali_phy_38::DenaliPhy38Spec>;
#[doc = ""]
pub mod denali_phy_38;
#[doc = "DENALI_PHY_39 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_39::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_39`]
module"]
#[doc(alias = "DENALI_PHY_39")]
pub type DenaliPhy39 = crate::Reg<denali_phy_39::DenaliPhy39Spec>;
#[doc = ""]
pub mod denali_phy_39;
#[doc = "DENALI_PHY_40 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_40::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_40`]
module"]
#[doc(alias = "DENALI_PHY_40")]
pub type DenaliPhy40 = crate::Reg<denali_phy_40::DenaliPhy40Spec>;
#[doc = ""]
pub mod denali_phy_40;
#[doc = "DENALI_PHY_41 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_41::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_41`]
module"]
#[doc(alias = "DENALI_PHY_41")]
pub type DenaliPhy41 = crate::Reg<denali_phy_41::DenaliPhy41Spec>;
#[doc = ""]
pub mod denali_phy_41;
#[doc = "DENALI_PHY_42 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_42::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_42`]
module"]
#[doc(alias = "DENALI_PHY_42")]
pub type DenaliPhy42 = crate::Reg<denali_phy_42::DenaliPhy42Spec>;
#[doc = ""]
pub mod denali_phy_42;
#[doc = "DENALI_PHY_43 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_43::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_43`]
module"]
#[doc(alias = "DENALI_PHY_43")]
pub type DenaliPhy43 = crate::Reg<denali_phy_43::DenaliPhy43Spec>;
#[doc = ""]
pub mod denali_phy_43;
#[doc = "DENALI_PHY_44 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_44::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_44`]
module"]
#[doc(alias = "DENALI_PHY_44")]
pub type DenaliPhy44 = crate::Reg<denali_phy_44::DenaliPhy44Spec>;
#[doc = ""]
pub mod denali_phy_44;
#[doc = "DENALI_PHY_45 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_45::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_45`]
module"]
#[doc(alias = "DENALI_PHY_45")]
pub type DenaliPhy45 = crate::Reg<denali_phy_45::DenaliPhy45Spec>;
#[doc = ""]
pub mod denali_phy_45;
#[doc = "DENALI_PHY_46 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_46::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_46`]
module"]
#[doc(alias = "DENALI_PHY_46")]
pub type DenaliPhy46 = crate::Reg<denali_phy_46::DenaliPhy46Spec>;
#[doc = ""]
pub mod denali_phy_46;
#[doc = "DENALI_PHY_47 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_47::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_47`]
module"]
#[doc(alias = "DENALI_PHY_47")]
pub type DenaliPhy47 = crate::Reg<denali_phy_47::DenaliPhy47Spec>;
#[doc = ""]
pub mod denali_phy_47;
#[doc = "DENALI_PHY_48 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_48::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_48`]
module"]
#[doc(alias = "DENALI_PHY_48")]
pub type DenaliPhy48 = crate::Reg<denali_phy_48::DenaliPhy48Spec>;
#[doc = ""]
pub mod denali_phy_48;
#[doc = "DENALI_PHY_49 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_49`]
module"]
#[doc(alias = "DENALI_PHY_49")]
pub type DenaliPhy49 = crate::Reg<denali_phy_49::DenaliPhy49Spec>;
#[doc = ""]
pub mod denali_phy_49;
#[doc = "DENALI_PHY_50 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_50::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_50`]
module"]
#[doc(alias = "DENALI_PHY_50")]
pub type DenaliPhy50 = crate::Reg<denali_phy_50::DenaliPhy50Spec>;
#[doc = ""]
pub mod denali_phy_50;
#[doc = "DENALI_PHY_51 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_51::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_51`]
module"]
#[doc(alias = "DENALI_PHY_51")]
pub type DenaliPhy51 = crate::Reg<denali_phy_51::DenaliPhy51Spec>;
#[doc = ""]
pub mod denali_phy_51;
#[doc = "DENALI_PHY_52 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_52`]
module"]
#[doc(alias = "DENALI_PHY_52")]
pub type DenaliPhy52 = crate::Reg<denali_phy_52::DenaliPhy52Spec>;
#[doc = ""]
pub mod denali_phy_52;
#[doc = "DENALI_PHY_53 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_53`]
module"]
#[doc(alias = "DENALI_PHY_53")]
pub type DenaliPhy53 = crate::Reg<denali_phy_53::DenaliPhy53Spec>;
#[doc = ""]
pub mod denali_phy_53;
#[doc = "DENALI_PHY_54 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_54::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_54`]
module"]
#[doc(alias = "DENALI_PHY_54")]
pub type DenaliPhy54 = crate::Reg<denali_phy_54::DenaliPhy54Spec>;
#[doc = ""]
pub mod denali_phy_54;
#[doc = "DENALI_PHY_55 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_55`]
module"]
#[doc(alias = "DENALI_PHY_55")]
pub type DenaliPhy55 = crate::Reg<denali_phy_55::DenaliPhy55Spec>;
#[doc = ""]
pub mod denali_phy_55;
#[doc = "DENALI_PHY_56 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_56::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_56`]
module"]
#[doc(alias = "DENALI_PHY_56")]
pub type DenaliPhy56 = crate::Reg<denali_phy_56::DenaliPhy56Spec>;
#[doc = ""]
pub mod denali_phy_56;
#[doc = "DENALI_PHY_57 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_57::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_57::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_57`]
module"]
#[doc(alias = "DENALI_PHY_57")]
pub type DenaliPhy57 = crate::Reg<denali_phy_57::DenaliPhy57Spec>;
#[doc = ""]
pub mod denali_phy_57;
#[doc = "DENALI_PHY_58 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_58::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_58::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_58`]
module"]
#[doc(alias = "DENALI_PHY_58")]
pub type DenaliPhy58 = crate::Reg<denali_phy_58::DenaliPhy58Spec>;
#[doc = ""]
pub mod denali_phy_58;
#[doc = "DENALI_PHY_59 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_59::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_59`]
module"]
#[doc(alias = "DENALI_PHY_59")]
pub type DenaliPhy59 = crate::Reg<denali_phy_59::DenaliPhy59Spec>;
#[doc = ""]
pub mod denali_phy_59;
#[doc = "DENALI_PHY_60 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_60::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_60`]
module"]
#[doc(alias = "DENALI_PHY_60")]
pub type DenaliPhy60 = crate::Reg<denali_phy_60::DenaliPhy60Spec>;
#[doc = ""]
pub mod denali_phy_60;
#[doc = "DENALI_PHY_61 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_61::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_61::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_61`]
module"]
#[doc(alias = "DENALI_PHY_61")]
pub type DenaliPhy61 = crate::Reg<denali_phy_61::DenaliPhy61Spec>;
#[doc = ""]
pub mod denali_phy_61;
#[doc = "DENALI_PHY_62 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_62::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_62::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_62`]
module"]
#[doc(alias = "DENALI_PHY_62")]
pub type DenaliPhy62 = crate::Reg<denali_phy_62::DenaliPhy62Spec>;
#[doc = ""]
pub mod denali_phy_62;
#[doc = "DENALI_PHY_63 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_63::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_63`]
module"]
#[doc(alias = "DENALI_PHY_63")]
pub type DenaliPhy63 = crate::Reg<denali_phy_63::DenaliPhy63Spec>;
#[doc = ""]
pub mod denali_phy_63;
#[doc = "DENALI_PHY_64 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_64`]
module"]
#[doc(alias = "DENALI_PHY_64")]
pub type DenaliPhy64 = crate::Reg<denali_phy_64::DenaliPhy64Spec>;
#[doc = ""]
pub mod denali_phy_64;
#[doc = "DENALI_PHY_65 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_65::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_65::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_65`]
module"]
#[doc(alias = "DENALI_PHY_65")]
pub type DenaliPhy65 = crate::Reg<denali_phy_65::DenaliPhy65Spec>;
#[doc = ""]
pub mod denali_phy_65;
#[doc = "DENALI_PHY_66 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_66::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_66::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_66`]
module"]
#[doc(alias = "DENALI_PHY_66")]
pub type DenaliPhy66 = crate::Reg<denali_phy_66::DenaliPhy66Spec>;
#[doc = ""]
pub mod denali_phy_66;
#[doc = "DENALI_PHY_67 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_67`]
module"]
#[doc(alias = "DENALI_PHY_67")]
pub type DenaliPhy67 = crate::Reg<denali_phy_67::DenaliPhy67Spec>;
#[doc = ""]
pub mod denali_phy_67;
#[doc = "DENALI_PHY_68 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_68::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_68::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_68`]
module"]
#[doc(alias = "DENALI_PHY_68")]
pub type DenaliPhy68 = crate::Reg<denali_phy_68::DenaliPhy68Spec>;
#[doc = ""]
pub mod denali_phy_68;
#[doc = "DENALI_PHY_69 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_69::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_69::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_69`]
module"]
#[doc(alias = "DENALI_PHY_69")]
pub type DenaliPhy69 = crate::Reg<denali_phy_69::DenaliPhy69Spec>;
#[doc = ""]
pub mod denali_phy_69;
#[doc = "DENALI_PHY_70 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_70::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_70::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_70`]
module"]
#[doc(alias = "DENALI_PHY_70")]
pub type DenaliPhy70 = crate::Reg<denali_phy_70::DenaliPhy70Spec>;
#[doc = ""]
pub mod denali_phy_70;
#[doc = "DENALI_PHY_71 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_71::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_71::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_71`]
module"]
#[doc(alias = "DENALI_PHY_71")]
pub type DenaliPhy71 = crate::Reg<denali_phy_71::DenaliPhy71Spec>;
#[doc = ""]
pub mod denali_phy_71;
#[doc = "DENALI_PHY_72 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_72::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_72::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_72`]
module"]
#[doc(alias = "DENALI_PHY_72")]
pub type DenaliPhy72 = crate::Reg<denali_phy_72::DenaliPhy72Spec>;
#[doc = ""]
pub mod denali_phy_72;
#[doc = "DENALI_PHY_73 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_73::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_73::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_73`]
module"]
#[doc(alias = "DENALI_PHY_73")]
pub type DenaliPhy73 = crate::Reg<denali_phy_73::DenaliPhy73Spec>;
#[doc = ""]
pub mod denali_phy_73;
#[doc = "DENALI_PHY_74 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_74::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_74::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_74`]
module"]
#[doc(alias = "DENALI_PHY_74")]
pub type DenaliPhy74 = crate::Reg<denali_phy_74::DenaliPhy74Spec>;
#[doc = ""]
pub mod denali_phy_74;
#[doc = "DENALI_PHY_75 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_75::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_75::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_75`]
module"]
#[doc(alias = "DENALI_PHY_75")]
pub type DenaliPhy75 = crate::Reg<denali_phy_75::DenaliPhy75Spec>;
#[doc = ""]
pub mod denali_phy_75;
#[doc = "DENALI_PHY_76 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_76::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_76::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_76`]
module"]
#[doc(alias = "DENALI_PHY_76")]
pub type DenaliPhy76 = crate::Reg<denali_phy_76::DenaliPhy76Spec>;
#[doc = ""]
pub mod denali_phy_76;
#[doc = "DENALI_PHY_77 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_77::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_77::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_77`]
module"]
#[doc(alias = "DENALI_PHY_77")]
pub type DenaliPhy77 = crate::Reg<denali_phy_77::DenaliPhy77Spec>;
#[doc = ""]
pub mod denali_phy_77;
#[doc = "DENALI_PHY_78 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_78::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_78::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_78`]
module"]
#[doc(alias = "DENALI_PHY_78")]
pub type DenaliPhy78 = crate::Reg<denali_phy_78::DenaliPhy78Spec>;
#[doc = ""]
pub mod denali_phy_78;
#[doc = "DENALI_PHY_79 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_79::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_79::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_79`]
module"]
#[doc(alias = "DENALI_PHY_79")]
pub type DenaliPhy79 = crate::Reg<denali_phy_79::DenaliPhy79Spec>;
#[doc = ""]
pub mod denali_phy_79;
#[doc = "DENALI_PHY_80 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_80::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_80::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_80`]
module"]
#[doc(alias = "DENALI_PHY_80")]
pub type DenaliPhy80 = crate::Reg<denali_phy_80::DenaliPhy80Spec>;
#[doc = ""]
pub mod denali_phy_80;
#[doc = "DENALI_PHY_81 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_81::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_81::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_81`]
module"]
#[doc(alias = "DENALI_PHY_81")]
pub type DenaliPhy81 = crate::Reg<denali_phy_81::DenaliPhy81Spec>;
#[doc = ""]
pub mod denali_phy_81;
#[doc = "DENALI_PHY_83 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_83::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_83::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_83`]
module"]
#[doc(alias = "DENALI_PHY_83")]
pub type DenaliPhy83 = crate::Reg<denali_phy_83::DenaliPhy83Spec>;
#[doc = ""]
pub mod denali_phy_83;
#[doc = "DENALI_PHY_84 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_84::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_84::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_84`]
module"]
#[doc(alias = "DENALI_PHY_84")]
pub type DenaliPhy84 = crate::Reg<denali_phy_84::DenaliPhy84Spec>;
#[doc = ""]
pub mod denali_phy_84;
#[doc = "DENALI_PHY_85 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_85::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_85::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_85`]
module"]
#[doc(alias = "DENALI_PHY_85")]
pub type DenaliPhy85 = crate::Reg<denali_phy_85::DenaliPhy85Spec>;
#[doc = ""]
pub mod denali_phy_85;
#[doc = "DENALI_PHY_86 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_86::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_86::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_86`]
module"]
#[doc(alias = "DENALI_PHY_86")]
pub type DenaliPhy86 = crate::Reg<denali_phy_86::DenaliPhy86Spec>;
#[doc = ""]
pub mod denali_phy_86;
#[doc = "DENALI_PHY_87 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_87::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_87::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_87`]
module"]
#[doc(alias = "DENALI_PHY_87")]
pub type DenaliPhy87 = crate::Reg<denali_phy_87::DenaliPhy87Spec>;
#[doc = ""]
pub mod denali_phy_87;
#[doc = "DENALI_PHY_88 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_88::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_88::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_88`]
module"]
#[doc(alias = "DENALI_PHY_88")]
pub type DenaliPhy88 = crate::Reg<denali_phy_88::DenaliPhy88Spec>;
#[doc = ""]
pub mod denali_phy_88;
#[doc = "DENALI_PHY_89 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_89::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_89::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_89`]
module"]
#[doc(alias = "DENALI_PHY_89")]
pub type DenaliPhy89 = crate::Reg<denali_phy_89::DenaliPhy89Spec>;
#[doc = ""]
pub mod denali_phy_89;
#[doc = "DENALI_PHY_90 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_90::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_90::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_90`]
module"]
#[doc(alias = "DENALI_PHY_90")]
pub type DenaliPhy90 = crate::Reg<denali_phy_90::DenaliPhy90Spec>;
#[doc = ""]
pub mod denali_phy_90;
#[doc = "DENALI_PHY_128 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_128::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_128::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_128`]
module"]
#[doc(alias = "DENALI_PHY_128")]
pub type DenaliPhy128 = crate::Reg<denali_phy_128::DenaliPhy128Spec>;
#[doc = ""]
pub mod denali_phy_128;
#[doc = "DENALI_PHY_129 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_129::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_129::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_129`]
module"]
#[doc(alias = "DENALI_PHY_129")]
pub type DenaliPhy129 = crate::Reg<denali_phy_129::DenaliPhy129Spec>;
#[doc = ""]
pub mod denali_phy_129;
#[doc = "DENALI_PHY_130 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_130::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_130::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_130`]
module"]
#[doc(alias = "DENALI_PHY_130")]
pub type DenaliPhy130 = crate::Reg<denali_phy_130::DenaliPhy130Spec>;
#[doc = ""]
pub mod denali_phy_130;
#[doc = "DENALI_PHY_131 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_131::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_131::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_131`]
module"]
#[doc(alias = "DENALI_PHY_131")]
pub type DenaliPhy131 = crate::Reg<denali_phy_131::DenaliPhy131Spec>;
#[doc = ""]
pub mod denali_phy_131;
#[doc = "DENALI_PHY_132 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_132::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_132::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_132`]
module"]
#[doc(alias = "DENALI_PHY_132")]
pub type DenaliPhy132 = crate::Reg<denali_phy_132::DenaliPhy132Spec>;
#[doc = ""]
pub mod denali_phy_132;
#[doc = "DENALI_PHY_133 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_133::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_133::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_133`]
module"]
#[doc(alias = "DENALI_PHY_133")]
pub type DenaliPhy133 = crate::Reg<denali_phy_133::DenaliPhy133Spec>;
#[doc = ""]
pub mod denali_phy_133;
#[doc = "DENALI_PHY_134 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_134::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_134::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_134`]
module"]
#[doc(alias = "DENALI_PHY_134")]
pub type DenaliPhy134 = crate::Reg<denali_phy_134::DenaliPhy134Spec>;
#[doc = ""]
pub mod denali_phy_134;
#[doc = "DENALI_PHY_135 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_135::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_135::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_135`]
module"]
#[doc(alias = "DENALI_PHY_135")]
pub type DenaliPhy135 = crate::Reg<denali_phy_135::DenaliPhy135Spec>;
#[doc = ""]
pub mod denali_phy_135;
#[doc = "DENALI_PHY_136 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_136::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_136::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_136`]
module"]
#[doc(alias = "DENALI_PHY_136")]
pub type DenaliPhy136 = crate::Reg<denali_phy_136::DenaliPhy136Spec>;
#[doc = ""]
pub mod denali_phy_136;
#[doc = "DENALI_PHY_137 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_137::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_137::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_137`]
module"]
#[doc(alias = "DENALI_PHY_137")]
pub type DenaliPhy137 = crate::Reg<denali_phy_137::DenaliPhy137Spec>;
#[doc = ""]
pub mod denali_phy_137;
#[doc = "DENALI_PHY_138 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_138::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_138::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_138`]
module"]
#[doc(alias = "DENALI_PHY_138")]
pub type DenaliPhy138 = crate::Reg<denali_phy_138::DenaliPhy138Spec>;
#[doc = ""]
pub mod denali_phy_138;
#[doc = "DENALI_PHY_139 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_139::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_139::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_139`]
module"]
#[doc(alias = "DENALI_PHY_139")]
pub type DenaliPhy139 = crate::Reg<denali_phy_139::DenaliPhy139Spec>;
#[doc = ""]
pub mod denali_phy_139;
#[doc = "DENALI_PHY_140 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_140::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_140::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_140`]
module"]
#[doc(alias = "DENALI_PHY_140")]
pub type DenaliPhy140 = crate::Reg<denali_phy_140::DenaliPhy140Spec>;
#[doc = ""]
pub mod denali_phy_140;
#[doc = "DENALI_PHY_141 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_141::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_141::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_141`]
module"]
#[doc(alias = "DENALI_PHY_141")]
pub type DenaliPhy141 = crate::Reg<denali_phy_141::DenaliPhy141Spec>;
#[doc = ""]
pub mod denali_phy_141;
#[doc = "DENALI_PHY_142 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_142::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_142`]
module"]
#[doc(alias = "DENALI_PHY_142")]
pub type DenaliPhy142 = crate::Reg<denali_phy_142::DenaliPhy142Spec>;
#[doc = ""]
pub mod denali_phy_142;
#[doc = "DENALI_PHY_143 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_143::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_143::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_143`]
module"]
#[doc(alias = "DENALI_PHY_143")]
pub type DenaliPhy143 = crate::Reg<denali_phy_143::DenaliPhy143Spec>;
#[doc = ""]
pub mod denali_phy_143;
#[doc = "DENALI_PHY_144 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_144::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_144::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_144`]
module"]
#[doc(alias = "DENALI_PHY_144")]
pub type DenaliPhy144 = crate::Reg<denali_phy_144::DenaliPhy144Spec>;
#[doc = ""]
pub mod denali_phy_144;
#[doc = "DENALI_PHY_145 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_145::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_145::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_145`]
module"]
#[doc(alias = "DENALI_PHY_145")]
pub type DenaliPhy145 = crate::Reg<denali_phy_145::DenaliPhy145Spec>;
#[doc = ""]
pub mod denali_phy_145;
#[doc = "DENALI_PHY_146 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_146::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_146::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_146`]
module"]
#[doc(alias = "DENALI_PHY_146")]
pub type DenaliPhy146 = crate::Reg<denali_phy_146::DenaliPhy146Spec>;
#[doc = ""]
pub mod denali_phy_146;
#[doc = "DENALI_PHY_147 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_147::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_147::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_147`]
module"]
#[doc(alias = "DENALI_PHY_147")]
pub type DenaliPhy147 = crate::Reg<denali_phy_147::DenaliPhy147Spec>;
#[doc = ""]
pub mod denali_phy_147;
#[doc = "DENALI_PHY_148 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_148::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_148::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_148`]
module"]
#[doc(alias = "DENALI_PHY_148")]
pub type DenaliPhy148 = crate::Reg<denali_phy_148::DenaliPhy148Spec>;
#[doc = ""]
pub mod denali_phy_148;
#[doc = "DENALI_PHY_149 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_149::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_149::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_149`]
module"]
#[doc(alias = "DENALI_PHY_149")]
pub type DenaliPhy149 = crate::Reg<denali_phy_149::DenaliPhy149Spec>;
#[doc = ""]
pub mod denali_phy_149;
#[doc = "DENALI_PHY_150 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_150::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_150::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_150`]
module"]
#[doc(alias = "DENALI_PHY_150")]
pub type DenaliPhy150 = crate::Reg<denali_phy_150::DenaliPhy150Spec>;
#[doc = ""]
pub mod denali_phy_150;
#[doc = "DENALI_PHY_151 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_151::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_151::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_151`]
module"]
#[doc(alias = "DENALI_PHY_151")]
pub type DenaliPhy151 = crate::Reg<denali_phy_151::DenaliPhy151Spec>;
#[doc = ""]
pub mod denali_phy_151;
#[doc = "DENALI_PHY_152 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_152::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_152::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_152`]
module"]
#[doc(alias = "DENALI_PHY_152")]
pub type DenaliPhy152 = crate::Reg<denali_phy_152::DenaliPhy152Spec>;
#[doc = ""]
pub mod denali_phy_152;
#[doc = "DENALI_PHY_153 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_153::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_153::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_153`]
module"]
#[doc(alias = "DENALI_PHY_153")]
pub type DenaliPhy153 = crate::Reg<denali_phy_153::DenaliPhy153Spec>;
#[doc = ""]
pub mod denali_phy_153;
#[doc = "DENALI_PHY_154 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_154::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_154::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_154`]
module"]
#[doc(alias = "DENALI_PHY_154")]
pub type DenaliPhy154 = crate::Reg<denali_phy_154::DenaliPhy154Spec>;
#[doc = ""]
pub mod denali_phy_154;
#[doc = "DENALI_PHY_155 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_155::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_155::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_155`]
module"]
#[doc(alias = "DENALI_PHY_155")]
pub type DenaliPhy155 = crate::Reg<denali_phy_155::DenaliPhy155Spec>;
#[doc = ""]
pub mod denali_phy_155;
#[doc = "DENALI_PHY_156 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_156::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_156::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_156`]
module"]
#[doc(alias = "DENALI_PHY_156")]
pub type DenaliPhy156 = crate::Reg<denali_phy_156::DenaliPhy156Spec>;
#[doc = ""]
pub mod denali_phy_156;
#[doc = "DENALI_PHY_157 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_157::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_157::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_157`]
module"]
#[doc(alias = "DENALI_PHY_157")]
pub type DenaliPhy157 = crate::Reg<denali_phy_157::DenaliPhy157Spec>;
#[doc = ""]
pub mod denali_phy_157;
#[doc = "DENALI_PHY_158 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_158::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_158::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_158`]
module"]
#[doc(alias = "DENALI_PHY_158")]
pub type DenaliPhy158 = crate::Reg<denali_phy_158::DenaliPhy158Spec>;
#[doc = ""]
pub mod denali_phy_158;
#[doc = "DENALI_PHY_159 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_159::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_159::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_159`]
module"]
#[doc(alias = "DENALI_PHY_159")]
pub type DenaliPhy159 = crate::Reg<denali_phy_159::DenaliPhy159Spec>;
#[doc = ""]
pub mod denali_phy_159;
#[doc = "DENALI_PHY_160 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_160::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_160::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_160`]
module"]
#[doc(alias = "DENALI_PHY_160")]
pub type DenaliPhy160 = crate::Reg<denali_phy_160::DenaliPhy160Spec>;
#[doc = ""]
pub mod denali_phy_160;
#[doc = "DENALI_PHY_161 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_161::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_161`]
module"]
#[doc(alias = "DENALI_PHY_161")]
pub type DenaliPhy161 = crate::Reg<denali_phy_161::DenaliPhy161Spec>;
#[doc = ""]
pub mod denali_phy_161;
#[doc = "DENALI_PHY_162 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_162::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_162`]
module"]
#[doc(alias = "DENALI_PHY_162")]
pub type DenaliPhy162 = crate::Reg<denali_phy_162::DenaliPhy162Spec>;
#[doc = ""]
pub mod denali_phy_162;
#[doc = "DENALI_PHY_163 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_163::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_163`]
module"]
#[doc(alias = "DENALI_PHY_163")]
pub type DenaliPhy163 = crate::Reg<denali_phy_163::DenaliPhy163Spec>;
#[doc = ""]
pub mod denali_phy_163;
#[doc = "DENALI_PHY_164 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_164::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_164`]
module"]
#[doc(alias = "DENALI_PHY_164")]
pub type DenaliPhy164 = crate::Reg<denali_phy_164::DenaliPhy164Spec>;
#[doc = ""]
pub mod denali_phy_164;
#[doc = "DENALI_PHY_165 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_165::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_165`]
module"]
#[doc(alias = "DENALI_PHY_165")]
pub type DenaliPhy165 = crate::Reg<denali_phy_165::DenaliPhy165Spec>;
#[doc = ""]
pub mod denali_phy_165;
#[doc = "DENALI_PHY_166 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_166::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_166`]
module"]
#[doc(alias = "DENALI_PHY_166")]
pub type DenaliPhy166 = crate::Reg<denali_phy_166::DenaliPhy166Spec>;
#[doc = ""]
pub mod denali_phy_166;
#[doc = "DENALI_PHY_167 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_167::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_167`]
module"]
#[doc(alias = "DENALI_PHY_167")]
pub type DenaliPhy167 = crate::Reg<denali_phy_167::DenaliPhy167Spec>;
#[doc = ""]
pub mod denali_phy_167;
#[doc = "DENALI_PHY_168 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_168::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_168`]
module"]
#[doc(alias = "DENALI_PHY_168")]
pub type DenaliPhy168 = crate::Reg<denali_phy_168::DenaliPhy168Spec>;
#[doc = ""]
pub mod denali_phy_168;
#[doc = "DENALI_PHY_169 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_169::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_169`]
module"]
#[doc(alias = "DENALI_PHY_169")]
pub type DenaliPhy169 = crate::Reg<denali_phy_169::DenaliPhy169Spec>;
#[doc = ""]
pub mod denali_phy_169;
#[doc = "DENALI_PHY_170 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_170::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_170`]
module"]
#[doc(alias = "DENALI_PHY_170")]
pub type DenaliPhy170 = crate::Reg<denali_phy_170::DenaliPhy170Spec>;
#[doc = ""]
pub mod denali_phy_170;
#[doc = "DENALI_PHY_171 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_171::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_171`]
module"]
#[doc(alias = "DENALI_PHY_171")]
pub type DenaliPhy171 = crate::Reg<denali_phy_171::DenaliPhy171Spec>;
#[doc = ""]
pub mod denali_phy_171;
#[doc = "DENALI_PHY_172 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_172::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_172`]
module"]
#[doc(alias = "DENALI_PHY_172")]
pub type DenaliPhy172 = crate::Reg<denali_phy_172::DenaliPhy172Spec>;
#[doc = ""]
pub mod denali_phy_172;
#[doc = "DENALI_PHY_173 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_173::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_173`]
module"]
#[doc(alias = "DENALI_PHY_173")]
pub type DenaliPhy173 = crate::Reg<denali_phy_173::DenaliPhy173Spec>;
#[doc = ""]
pub mod denali_phy_173;
#[doc = "DENALI_PHY_174 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_174::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_174`]
module"]
#[doc(alias = "DENALI_PHY_174")]
pub type DenaliPhy174 = crate::Reg<denali_phy_174::DenaliPhy174Spec>;
#[doc = ""]
pub mod denali_phy_174;
#[doc = "DENALI_PHY_175 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_175::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_175`]
module"]
#[doc(alias = "DENALI_PHY_175")]
pub type DenaliPhy175 = crate::Reg<denali_phy_175::DenaliPhy175Spec>;
#[doc = ""]
pub mod denali_phy_175;
#[doc = "DENALI_PHY_176 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_176::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_176`]
module"]
#[doc(alias = "DENALI_PHY_176")]
pub type DenaliPhy176 = crate::Reg<denali_phy_176::DenaliPhy176Spec>;
#[doc = ""]
pub mod denali_phy_176;
#[doc = "DENALI_PHY_177 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_177::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_177::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_177`]
module"]
#[doc(alias = "DENALI_PHY_177")]
pub type DenaliPhy177 = crate::Reg<denali_phy_177::DenaliPhy177Spec>;
#[doc = ""]
pub mod denali_phy_177;
#[doc = "DENALI_PHY_178 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_178::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_178`]
module"]
#[doc(alias = "DENALI_PHY_178")]
pub type DenaliPhy178 = crate::Reg<denali_phy_178::DenaliPhy178Spec>;
#[doc = ""]
pub mod denali_phy_178;
#[doc = "DENALI_PHY_179 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_179::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_179`]
module"]
#[doc(alias = "DENALI_PHY_179")]
pub type DenaliPhy179 = crate::Reg<denali_phy_179::DenaliPhy179Spec>;
#[doc = ""]
pub mod denali_phy_179;
#[doc = "DENALI_PHY_180 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_180::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_180::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_180`]
module"]
#[doc(alias = "DENALI_PHY_180")]
pub type DenaliPhy180 = crate::Reg<denali_phy_180::DenaliPhy180Spec>;
#[doc = ""]
pub mod denali_phy_180;
#[doc = "DENALI_PHY_181 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_181::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_181::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_181`]
module"]
#[doc(alias = "DENALI_PHY_181")]
pub type DenaliPhy181 = crate::Reg<denali_phy_181::DenaliPhy181Spec>;
#[doc = ""]
pub mod denali_phy_181;
#[doc = "DENALI_PHY_182 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_182::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_182::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_182`]
module"]
#[doc(alias = "DENALI_PHY_182")]
pub type DenaliPhy182 = crate::Reg<denali_phy_182::DenaliPhy182Spec>;
#[doc = ""]
pub mod denali_phy_182;
#[doc = "DENALI_PHY_183 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_183::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_183::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_183`]
module"]
#[doc(alias = "DENALI_PHY_183")]
pub type DenaliPhy183 = crate::Reg<denali_phy_183::DenaliPhy183Spec>;
#[doc = ""]
pub mod denali_phy_183;
#[doc = "DENALI_PHY_184 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_184::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_184::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_184`]
module"]
#[doc(alias = "DENALI_PHY_184")]
pub type DenaliPhy184 = crate::Reg<denali_phy_184::DenaliPhy184Spec>;
#[doc = ""]
pub mod denali_phy_184;
#[doc = "DENALI_PHY_185 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_185::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_185::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_185`]
module"]
#[doc(alias = "DENALI_PHY_185")]
pub type DenaliPhy185 = crate::Reg<denali_phy_185::DenaliPhy185Spec>;
#[doc = ""]
pub mod denali_phy_185;
#[doc = "DENALI_PHY_186 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_186::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_186::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_186`]
module"]
#[doc(alias = "DENALI_PHY_186")]
pub type DenaliPhy186 = crate::Reg<denali_phy_186::DenaliPhy186Spec>;
#[doc = ""]
pub mod denali_phy_186;
#[doc = "DENALI_PHY_187 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_187::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_187::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_187`]
module"]
#[doc(alias = "DENALI_PHY_187")]
pub type DenaliPhy187 = crate::Reg<denali_phy_187::DenaliPhy187Spec>;
#[doc = ""]
pub mod denali_phy_187;
#[doc = "DENALI_PHY_188 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_188::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_188::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_188`]
module"]
#[doc(alias = "DENALI_PHY_188")]
pub type DenaliPhy188 = crate::Reg<denali_phy_188::DenaliPhy188Spec>;
#[doc = ""]
pub mod denali_phy_188;
#[doc = "DENALI_PHY_189 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_189::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_189::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_189`]
module"]
#[doc(alias = "DENALI_PHY_189")]
pub type DenaliPhy189 = crate::Reg<denali_phy_189::DenaliPhy189Spec>;
#[doc = ""]
pub mod denali_phy_189;
#[doc = "DENALI_PHY_190 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_190::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_190::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_190`]
module"]
#[doc(alias = "DENALI_PHY_190")]
pub type DenaliPhy190 = crate::Reg<denali_phy_190::DenaliPhy190Spec>;
#[doc = ""]
pub mod denali_phy_190;
#[doc = "DENALI_PHY_191 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_191::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_191::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_191`]
module"]
#[doc(alias = "DENALI_PHY_191")]
pub type DenaliPhy191 = crate::Reg<denali_phy_191::DenaliPhy191Spec>;
#[doc = ""]
pub mod denali_phy_191;
#[doc = "DENALI_PHY_192 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_192::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_192::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_192`]
module"]
#[doc(alias = "DENALI_PHY_192")]
pub type DenaliPhy192 = crate::Reg<denali_phy_192::DenaliPhy192Spec>;
#[doc = ""]
pub mod denali_phy_192;
#[doc = "DENALI_PHY_193 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_193::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_193::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_193`]
module"]
#[doc(alias = "DENALI_PHY_193")]
pub type DenaliPhy193 = crate::Reg<denali_phy_193::DenaliPhy193Spec>;
#[doc = ""]
pub mod denali_phy_193;
#[doc = "DENALI_PHY_194 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_194::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_194::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_194`]
module"]
#[doc(alias = "DENALI_PHY_194")]
pub type DenaliPhy194 = crate::Reg<denali_phy_194::DenaliPhy194Spec>;
#[doc = ""]
pub mod denali_phy_194;
#[doc = "DENALI_PHY_195 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_195::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_195::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_195`]
module"]
#[doc(alias = "DENALI_PHY_195")]
pub type DenaliPhy195 = crate::Reg<denali_phy_195::DenaliPhy195Spec>;
#[doc = ""]
pub mod denali_phy_195;
#[doc = "DENALI_PHY_196 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_196::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_196::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_196`]
module"]
#[doc(alias = "DENALI_PHY_196")]
pub type DenaliPhy196 = crate::Reg<denali_phy_196::DenaliPhy196Spec>;
#[doc = ""]
pub mod denali_phy_196;
#[doc = "DENALI_PHY_197 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_197::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_197::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_197`]
module"]
#[doc(alias = "DENALI_PHY_197")]
pub type DenaliPhy197 = crate::Reg<denali_phy_197::DenaliPhy197Spec>;
#[doc = ""]
pub mod denali_phy_197;
#[doc = "DENALI_PHY_198 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_198::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_198::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_198`]
module"]
#[doc(alias = "DENALI_PHY_198")]
pub type DenaliPhy198 = crate::Reg<denali_phy_198::DenaliPhy198Spec>;
#[doc = ""]
pub mod denali_phy_198;
#[doc = "DENALI_PHY_199 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_199::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_199::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_199`]
module"]
#[doc(alias = "DENALI_PHY_199")]
pub type DenaliPhy199 = crate::Reg<denali_phy_199::DenaliPhy199Spec>;
#[doc = ""]
pub mod denali_phy_199;
#[doc = "DENALI_PHY_200 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_200::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_200::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_200`]
module"]
#[doc(alias = "DENALI_PHY_200")]
pub type DenaliPhy200 = crate::Reg<denali_phy_200::DenaliPhy200Spec>;
#[doc = ""]
pub mod denali_phy_200;
#[doc = "DENALI_PHY_201 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_201::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_201::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_201`]
module"]
#[doc(alias = "DENALI_PHY_201")]
pub type DenaliPhy201 = crate::Reg<denali_phy_201::DenaliPhy201Spec>;
#[doc = ""]
pub mod denali_phy_201;
#[doc = "DENALI_PHY_202 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_202::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_202::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_202`]
module"]
#[doc(alias = "DENALI_PHY_202")]
pub type DenaliPhy202 = crate::Reg<denali_phy_202::DenaliPhy202Spec>;
#[doc = ""]
pub mod denali_phy_202;
#[doc = "DENALI_PHY_203 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_203::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_203::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_203`]
module"]
#[doc(alias = "DENALI_PHY_203")]
pub type DenaliPhy203 = crate::Reg<denali_phy_203::DenaliPhy203Spec>;
#[doc = ""]
pub mod denali_phy_203;
#[doc = "DENALI_PHY_204 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_204::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_204::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_204`]
module"]
#[doc(alias = "DENALI_PHY_204")]
pub type DenaliPhy204 = crate::Reg<denali_phy_204::DenaliPhy204Spec>;
#[doc = ""]
pub mod denali_phy_204;
#[doc = "DENALI_PHY_205 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_205::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_205::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_205`]
module"]
#[doc(alias = "DENALI_PHY_205")]
pub type DenaliPhy205 = crate::Reg<denali_phy_205::DenaliPhy205Spec>;
#[doc = ""]
pub mod denali_phy_205;
#[doc = "DENALI_PHY_206 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_206::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_206::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_206`]
module"]
#[doc(alias = "DENALI_PHY_206")]
pub type DenaliPhy206 = crate::Reg<denali_phy_206::DenaliPhy206Spec>;
#[doc = ""]
pub mod denali_phy_206;
#[doc = "DENALI_PHY_207 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_207::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_207::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_207`]
module"]
#[doc(alias = "DENALI_PHY_207")]
pub type DenaliPhy207 = crate::Reg<denali_phy_207::DenaliPhy207Spec>;
#[doc = ""]
pub mod denali_phy_207;
#[doc = "DENALI_PHY_208 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_208::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_208::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_208`]
module"]
#[doc(alias = "DENALI_PHY_208")]
pub type DenaliPhy208 = crate::Reg<denali_phy_208::DenaliPhy208Spec>;
#[doc = ""]
pub mod denali_phy_208;
#[doc = "DENALI_PHY_209 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_209::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_209::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_209`]
module"]
#[doc(alias = "DENALI_PHY_209")]
pub type DenaliPhy209 = crate::Reg<denali_phy_209::DenaliPhy209Spec>;
#[doc = ""]
pub mod denali_phy_209;
#[doc = "DENALI_PHY_211 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_211::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_211::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_211`]
module"]
#[doc(alias = "DENALI_PHY_211")]
pub type DenaliPhy211 = crate::Reg<denali_phy_211::DenaliPhy211Spec>;
#[doc = ""]
pub mod denali_phy_211;
#[doc = "DENALI_PHY_212 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_212::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_212::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_212`]
module"]
#[doc(alias = "DENALI_PHY_212")]
pub type DenaliPhy212 = crate::Reg<denali_phy_212::DenaliPhy212Spec>;
#[doc = ""]
pub mod denali_phy_212;
#[doc = "DENALI_PHY_213 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_213::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_213::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_213`]
module"]
#[doc(alias = "DENALI_PHY_213")]
pub type DenaliPhy213 = crate::Reg<denali_phy_213::DenaliPhy213Spec>;
#[doc = ""]
pub mod denali_phy_213;
#[doc = "DENALI_PHY_214 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_214::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_214::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_214`]
module"]
#[doc(alias = "DENALI_PHY_214")]
pub type DenaliPhy214 = crate::Reg<denali_phy_214::DenaliPhy214Spec>;
#[doc = ""]
pub mod denali_phy_214;
#[doc = "DENALI_PHY_215 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_215::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_215::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_215`]
module"]
#[doc(alias = "DENALI_PHY_215")]
pub type DenaliPhy215 = crate::Reg<denali_phy_215::DenaliPhy215Spec>;
#[doc = ""]
pub mod denali_phy_215;
#[doc = "DENALI_PHY_216 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_216::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_216::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_216`]
module"]
#[doc(alias = "DENALI_PHY_216")]
pub type DenaliPhy216 = crate::Reg<denali_phy_216::DenaliPhy216Spec>;
#[doc = ""]
pub mod denali_phy_216;
#[doc = "DENALI_PHY_217 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_217::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_217::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_217`]
module"]
#[doc(alias = "DENALI_PHY_217")]
pub type DenaliPhy217 = crate::Reg<denali_phy_217::DenaliPhy217Spec>;
#[doc = ""]
pub mod denali_phy_217;
#[doc = "DENALI_PHY_218 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_218::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_218::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_218`]
module"]
#[doc(alias = "DENALI_PHY_218")]
pub type DenaliPhy218 = crate::Reg<denali_phy_218::DenaliPhy218Spec>;
#[doc = ""]
pub mod denali_phy_218;
#[doc = "DENALI_PHY_256 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_256::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_256::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_256`]
module"]
#[doc(alias = "DENALI_PHY_256")]
pub type DenaliPhy256 = crate::Reg<denali_phy_256::DenaliPhy256Spec>;
#[doc = ""]
pub mod denali_phy_256;
#[doc = "DENALI_PHY_257 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_257::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_257::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_257`]
module"]
#[doc(alias = "DENALI_PHY_257")]
pub type DenaliPhy257 = crate::Reg<denali_phy_257::DenaliPhy257Spec>;
#[doc = ""]
pub mod denali_phy_257;
#[doc = "DENALI_PHY_258 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_258::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_258::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_258`]
module"]
#[doc(alias = "DENALI_PHY_258")]
pub type DenaliPhy258 = crate::Reg<denali_phy_258::DenaliPhy258Spec>;
#[doc = ""]
pub mod denali_phy_258;
#[doc = "DENALI_PHY_259 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_259::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_259::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_259`]
module"]
#[doc(alias = "DENALI_PHY_259")]
pub type DenaliPhy259 = crate::Reg<denali_phy_259::DenaliPhy259Spec>;
#[doc = ""]
pub mod denali_phy_259;
#[doc = "DENALI_PHY_260 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_260::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_260::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_260`]
module"]
#[doc(alias = "DENALI_PHY_260")]
pub type DenaliPhy260 = crate::Reg<denali_phy_260::DenaliPhy260Spec>;
#[doc = ""]
pub mod denali_phy_260;
#[doc = "DENALI_PHY_261 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_261::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_261::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_261`]
module"]
#[doc(alias = "DENALI_PHY_261")]
pub type DenaliPhy261 = crate::Reg<denali_phy_261::DenaliPhy261Spec>;
#[doc = ""]
pub mod denali_phy_261;
#[doc = "DENALI_PHY_262 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_262::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_262::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_262`]
module"]
#[doc(alias = "DENALI_PHY_262")]
pub type DenaliPhy262 = crate::Reg<denali_phy_262::DenaliPhy262Spec>;
#[doc = ""]
pub mod denali_phy_262;
#[doc = "DENALI_PHY_263 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_263::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_263::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_263`]
module"]
#[doc(alias = "DENALI_PHY_263")]
pub type DenaliPhy263 = crate::Reg<denali_phy_263::DenaliPhy263Spec>;
#[doc = ""]
pub mod denali_phy_263;
#[doc = "DENALI_PHY_264 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_264::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_264::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_264`]
module"]
#[doc(alias = "DENALI_PHY_264")]
pub type DenaliPhy264 = crate::Reg<denali_phy_264::DenaliPhy264Spec>;
#[doc = ""]
pub mod denali_phy_264;
#[doc = "DENALI_PHY_265 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_265::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_265::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_265`]
module"]
#[doc(alias = "DENALI_PHY_265")]
pub type DenaliPhy265 = crate::Reg<denali_phy_265::DenaliPhy265Spec>;
#[doc = ""]
pub mod denali_phy_265;
#[doc = "DENALI_PHY_266 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_266::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_266::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_266`]
module"]
#[doc(alias = "DENALI_PHY_266")]
pub type DenaliPhy266 = crate::Reg<denali_phy_266::DenaliPhy266Spec>;
#[doc = ""]
pub mod denali_phy_266;
#[doc = "DENALI_PHY_267 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_267::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_267::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_267`]
module"]
#[doc(alias = "DENALI_PHY_267")]
pub type DenaliPhy267 = crate::Reg<denali_phy_267::DenaliPhy267Spec>;
#[doc = ""]
pub mod denali_phy_267;
#[doc = "DENALI_PHY_268 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_268::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_268::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_268`]
module"]
#[doc(alias = "DENALI_PHY_268")]
pub type DenaliPhy268 = crate::Reg<denali_phy_268::DenaliPhy268Spec>;
#[doc = ""]
pub mod denali_phy_268;
#[doc = "DENALI_PHY_269 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_269::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_269::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_269`]
module"]
#[doc(alias = "DENALI_PHY_269")]
pub type DenaliPhy269 = crate::Reg<denali_phy_269::DenaliPhy269Spec>;
#[doc = ""]
pub mod denali_phy_269;
#[doc = "DENALI_PHY_270 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_270::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_270`]
module"]
#[doc(alias = "DENALI_PHY_270")]
pub type DenaliPhy270 = crate::Reg<denali_phy_270::DenaliPhy270Spec>;
#[doc = ""]
pub mod denali_phy_270;
#[doc = "DENALI_PHY_271 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_271::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_271::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_271`]
module"]
#[doc(alias = "DENALI_PHY_271")]
pub type DenaliPhy271 = crate::Reg<denali_phy_271::DenaliPhy271Spec>;
#[doc = ""]
pub mod denali_phy_271;
#[doc = "DENALI_PHY_272 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_272::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_272::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_272`]
module"]
#[doc(alias = "DENALI_PHY_272")]
pub type DenaliPhy272 = crate::Reg<denali_phy_272::DenaliPhy272Spec>;
#[doc = ""]
pub mod denali_phy_272;
#[doc = "DENALI_PHY_273 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_273::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_273::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_273`]
module"]
#[doc(alias = "DENALI_PHY_273")]
pub type DenaliPhy273 = crate::Reg<denali_phy_273::DenaliPhy273Spec>;
#[doc = ""]
pub mod denali_phy_273;
#[doc = "DENALI_PHY_274 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_274::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_274::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_274`]
module"]
#[doc(alias = "DENALI_PHY_274")]
pub type DenaliPhy274 = crate::Reg<denali_phy_274::DenaliPhy274Spec>;
#[doc = ""]
pub mod denali_phy_274;
#[doc = "DENALI_PHY_275 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_275::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_275::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_275`]
module"]
#[doc(alias = "DENALI_PHY_275")]
pub type DenaliPhy275 = crate::Reg<denali_phy_275::DenaliPhy275Spec>;
#[doc = ""]
pub mod denali_phy_275;
#[doc = "DENALI_PHY_276 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_276::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_276::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_276`]
module"]
#[doc(alias = "DENALI_PHY_276")]
pub type DenaliPhy276 = crate::Reg<denali_phy_276::DenaliPhy276Spec>;
#[doc = ""]
pub mod denali_phy_276;
#[doc = "DENALI_PHY_277 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_277::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_277::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_277`]
module"]
#[doc(alias = "DENALI_PHY_277")]
pub type DenaliPhy277 = crate::Reg<denali_phy_277::DenaliPhy277Spec>;
#[doc = ""]
pub mod denali_phy_277;
#[doc = "DENALI_PHY_278 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_278::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_278::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_278`]
module"]
#[doc(alias = "DENALI_PHY_278")]
pub type DenaliPhy278 = crate::Reg<denali_phy_278::DenaliPhy278Spec>;
#[doc = ""]
pub mod denali_phy_278;
#[doc = "DENALI_PHY_279 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_279::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_279::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_279`]
module"]
#[doc(alias = "DENALI_PHY_279")]
pub type DenaliPhy279 = crate::Reg<denali_phy_279::DenaliPhy279Spec>;
#[doc = ""]
pub mod denali_phy_279;
#[doc = "DENALI_PHY_280 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_280::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_280::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_280`]
module"]
#[doc(alias = "DENALI_PHY_280")]
pub type DenaliPhy280 = crate::Reg<denali_phy_280::DenaliPhy280Spec>;
#[doc = ""]
pub mod denali_phy_280;
#[doc = "DENALI_PHY_281 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_281::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_281::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_281`]
module"]
#[doc(alias = "DENALI_PHY_281")]
pub type DenaliPhy281 = crate::Reg<denali_phy_281::DenaliPhy281Spec>;
#[doc = ""]
pub mod denali_phy_281;
#[doc = "DENALI_PHY_282 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_282::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_282::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_282`]
module"]
#[doc(alias = "DENALI_PHY_282")]
pub type DenaliPhy282 = crate::Reg<denali_phy_282::DenaliPhy282Spec>;
#[doc = ""]
pub mod denali_phy_282;
#[doc = "DENALI_PHY_283 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_283::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_283::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_283`]
module"]
#[doc(alias = "DENALI_PHY_283")]
pub type DenaliPhy283 = crate::Reg<denali_phy_283::DenaliPhy283Spec>;
#[doc = ""]
pub mod denali_phy_283;
#[doc = "DENALI_PHY_284 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_284::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_284::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_284`]
module"]
#[doc(alias = "DENALI_PHY_284")]
pub type DenaliPhy284 = crate::Reg<denali_phy_284::DenaliPhy284Spec>;
#[doc = ""]
pub mod denali_phy_284;
#[doc = "DENALI_PHY_285 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_285::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_285::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_285`]
module"]
#[doc(alias = "DENALI_PHY_285")]
pub type DenaliPhy285 = crate::Reg<denali_phy_285::DenaliPhy285Spec>;
#[doc = ""]
pub mod denali_phy_285;
#[doc = "DENALI_PHY_286 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_286::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_286::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_286`]
module"]
#[doc(alias = "DENALI_PHY_286")]
pub type DenaliPhy286 = crate::Reg<denali_phy_286::DenaliPhy286Spec>;
#[doc = ""]
pub mod denali_phy_286;
#[doc = "DENALI_PHY_287 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_287::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_287::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_287`]
module"]
#[doc(alias = "DENALI_PHY_287")]
pub type DenaliPhy287 = crate::Reg<denali_phy_287::DenaliPhy287Spec>;
#[doc = ""]
pub mod denali_phy_287;
#[doc = "DENALI_PHY_288 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_288::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_288::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_288`]
module"]
#[doc(alias = "DENALI_PHY_288")]
pub type DenaliPhy288 = crate::Reg<denali_phy_288::DenaliPhy288Spec>;
#[doc = ""]
pub mod denali_phy_288;
#[doc = "DENALI_PHY_289 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_289::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_289`]
module"]
#[doc(alias = "DENALI_PHY_289")]
pub type DenaliPhy289 = crate::Reg<denali_phy_289::DenaliPhy289Spec>;
#[doc = ""]
pub mod denali_phy_289;
#[doc = "DENALI_PHY_290 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_290::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_290`]
module"]
#[doc(alias = "DENALI_PHY_290")]
pub type DenaliPhy290 = crate::Reg<denali_phy_290::DenaliPhy290Spec>;
#[doc = ""]
pub mod denali_phy_290;
#[doc = "DENALI_PHY_291 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_291::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_291`]
module"]
#[doc(alias = "DENALI_PHY_291")]
pub type DenaliPhy291 = crate::Reg<denali_phy_291::DenaliPhy291Spec>;
#[doc = ""]
pub mod denali_phy_291;
#[doc = "DENALI_PHY_292 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_292::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_292`]
module"]
#[doc(alias = "DENALI_PHY_292")]
pub type DenaliPhy292 = crate::Reg<denali_phy_292::DenaliPhy292Spec>;
#[doc = ""]
pub mod denali_phy_292;
#[doc = "DENALI_PHY_293 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_293::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_293`]
module"]
#[doc(alias = "DENALI_PHY_293")]
pub type DenaliPhy293 = crate::Reg<denali_phy_293::DenaliPhy293Spec>;
#[doc = ""]
pub mod denali_phy_293;
#[doc = "DENALI_PHY_294 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_294::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_294`]
module"]
#[doc(alias = "DENALI_PHY_294")]
pub type DenaliPhy294 = crate::Reg<denali_phy_294::DenaliPhy294Spec>;
#[doc = ""]
pub mod denali_phy_294;
#[doc = "DENALI_PHY_295 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_295::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_295`]
module"]
#[doc(alias = "DENALI_PHY_295")]
pub type DenaliPhy295 = crate::Reg<denali_phy_295::DenaliPhy295Spec>;
#[doc = ""]
pub mod denali_phy_295;
#[doc = "DENALI_PHY_296 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_296::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_296`]
module"]
#[doc(alias = "DENALI_PHY_296")]
pub type DenaliPhy296 = crate::Reg<denali_phy_296::DenaliPhy296Spec>;
#[doc = ""]
pub mod denali_phy_296;
#[doc = "DENALI_PHY_297 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_297::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_297`]
module"]
#[doc(alias = "DENALI_PHY_297")]
pub type DenaliPhy297 = crate::Reg<denali_phy_297::DenaliPhy297Spec>;
#[doc = ""]
pub mod denali_phy_297;
#[doc = "DENALI_PHY_298 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_298::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_298`]
module"]
#[doc(alias = "DENALI_PHY_298")]
pub type DenaliPhy298 = crate::Reg<denali_phy_298::DenaliPhy298Spec>;
#[doc = ""]
pub mod denali_phy_298;
#[doc = "DENALI_PHY_299 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_299::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_299`]
module"]
#[doc(alias = "DENALI_PHY_299")]
pub type DenaliPhy299 = crate::Reg<denali_phy_299::DenaliPhy299Spec>;
#[doc = ""]
pub mod denali_phy_299;
#[doc = "DENALI_PHY_300 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_300::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_300`]
module"]
#[doc(alias = "DENALI_PHY_300")]
pub type DenaliPhy300 = crate::Reg<denali_phy_300::DenaliPhy300Spec>;
#[doc = ""]
pub mod denali_phy_300;
#[doc = "DENALI_PHY_301 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_301::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_301`]
module"]
#[doc(alias = "DENALI_PHY_301")]
pub type DenaliPhy301 = crate::Reg<denali_phy_301::DenaliPhy301Spec>;
#[doc = ""]
pub mod denali_phy_301;
#[doc = "DENALI_PHY_302 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_302::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_302`]
module"]
#[doc(alias = "DENALI_PHY_302")]
pub type DenaliPhy302 = crate::Reg<denali_phy_302::DenaliPhy302Spec>;
#[doc = ""]
pub mod denali_phy_302;
#[doc = "DENALI_PHY_303 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_303::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_303`]
module"]
#[doc(alias = "DENALI_PHY_303")]
pub type DenaliPhy303 = crate::Reg<denali_phy_303::DenaliPhy303Spec>;
#[doc = ""]
pub mod denali_phy_303;
#[doc = "DENALI_PHY_304 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_304::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_304`]
module"]
#[doc(alias = "DENALI_PHY_304")]
pub type DenaliPhy304 = crate::Reg<denali_phy_304::DenaliPhy304Spec>;
#[doc = ""]
pub mod denali_phy_304;
#[doc = "DENALI_PHY_305 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_305::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_305::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_305`]
module"]
#[doc(alias = "DENALI_PHY_305")]
pub type DenaliPhy305 = crate::Reg<denali_phy_305::DenaliPhy305Spec>;
#[doc = ""]
pub mod denali_phy_305;
#[doc = "DENALI_PHY_306 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_306::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_306`]
module"]
#[doc(alias = "DENALI_PHY_306")]
pub type DenaliPhy306 = crate::Reg<denali_phy_306::DenaliPhy306Spec>;
#[doc = ""]
pub mod denali_phy_306;
#[doc = "DENALI_PHY_307 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_307::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_307`]
module"]
#[doc(alias = "DENALI_PHY_307")]
pub type DenaliPhy307 = crate::Reg<denali_phy_307::DenaliPhy307Spec>;
#[doc = ""]
pub mod denali_phy_307;
#[doc = "DENALI_PHY_308 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_308::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_308::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_308`]
module"]
#[doc(alias = "DENALI_PHY_308")]
pub type DenaliPhy308 = crate::Reg<denali_phy_308::DenaliPhy308Spec>;
#[doc = ""]
pub mod denali_phy_308;
#[doc = "DENALI_PHY_309 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_309::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_309::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_309`]
module"]
#[doc(alias = "DENALI_PHY_309")]
pub type DenaliPhy309 = crate::Reg<denali_phy_309::DenaliPhy309Spec>;
#[doc = ""]
pub mod denali_phy_309;
#[doc = "DENALI_PHY_310 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_310::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_310::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_310`]
module"]
#[doc(alias = "DENALI_PHY_310")]
pub type DenaliPhy310 = crate::Reg<denali_phy_310::DenaliPhy310Spec>;
#[doc = ""]
pub mod denali_phy_310;
#[doc = "DENALI_PHY_311 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_311::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_311::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_311`]
module"]
#[doc(alias = "DENALI_PHY_311")]
pub type DenaliPhy311 = crate::Reg<denali_phy_311::DenaliPhy311Spec>;
#[doc = ""]
pub mod denali_phy_311;
#[doc = "DENALI_PHY_312 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_312::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_312::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_312`]
module"]
#[doc(alias = "DENALI_PHY_312")]
pub type DenaliPhy312 = crate::Reg<denali_phy_312::DenaliPhy312Spec>;
#[doc = ""]
pub mod denali_phy_312;
#[doc = "DENALI_PHY_313 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_313::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_313::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_313`]
module"]
#[doc(alias = "DENALI_PHY_313")]
pub type DenaliPhy313 = crate::Reg<denali_phy_313::DenaliPhy313Spec>;
#[doc = ""]
pub mod denali_phy_313;
#[doc = "DENALI_PHY_314 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_314::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_314::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_314`]
module"]
#[doc(alias = "DENALI_PHY_314")]
pub type DenaliPhy314 = crate::Reg<denali_phy_314::DenaliPhy314Spec>;
#[doc = ""]
pub mod denali_phy_314;
#[doc = "DENALI_PHY_315 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_315::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_315::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_315`]
module"]
#[doc(alias = "DENALI_PHY_315")]
pub type DenaliPhy315 = crate::Reg<denali_phy_315::DenaliPhy315Spec>;
#[doc = ""]
pub mod denali_phy_315;
#[doc = "DENALI_PHY_316 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_316::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_316::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_316`]
module"]
#[doc(alias = "DENALI_PHY_316")]
pub type DenaliPhy316 = crate::Reg<denali_phy_316::DenaliPhy316Spec>;
#[doc = ""]
pub mod denali_phy_316;
#[doc = "DENALI_PHY_317 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_317::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_317::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_317`]
module"]
#[doc(alias = "DENALI_PHY_317")]
pub type DenaliPhy317 = crate::Reg<denali_phy_317::DenaliPhy317Spec>;
#[doc = ""]
pub mod denali_phy_317;
#[doc = "DENALI_PHY_318 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_318::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_318::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_318`]
module"]
#[doc(alias = "DENALI_PHY_318")]
pub type DenaliPhy318 = crate::Reg<denali_phy_318::DenaliPhy318Spec>;
#[doc = ""]
pub mod denali_phy_318;
#[doc = "DENALI_PHY_319 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_319::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_319::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_319`]
module"]
#[doc(alias = "DENALI_PHY_319")]
pub type DenaliPhy319 = crate::Reg<denali_phy_319::DenaliPhy319Spec>;
#[doc = ""]
pub mod denali_phy_319;
#[doc = "DENALI_PHY_320 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_320::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_320::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_320`]
module"]
#[doc(alias = "DENALI_PHY_320")]
pub type DenaliPhy320 = crate::Reg<denali_phy_320::DenaliPhy320Spec>;
#[doc = ""]
pub mod denali_phy_320;
#[doc = "DENALI_PHY_321 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_321::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_321::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_321`]
module"]
#[doc(alias = "DENALI_PHY_321")]
pub type DenaliPhy321 = crate::Reg<denali_phy_321::DenaliPhy321Spec>;
#[doc = ""]
pub mod denali_phy_321;
#[doc = "DENALI_PHY_322 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_322::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_322::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_322`]
module"]
#[doc(alias = "DENALI_PHY_322")]
pub type DenaliPhy322 = crate::Reg<denali_phy_322::DenaliPhy322Spec>;
#[doc = ""]
pub mod denali_phy_322;
#[doc = "DENALI_PHY_323 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_323::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_323::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_323`]
module"]
#[doc(alias = "DENALI_PHY_323")]
pub type DenaliPhy323 = crate::Reg<denali_phy_323::DenaliPhy323Spec>;
#[doc = ""]
pub mod denali_phy_323;
#[doc = "DENALI_PHY_324 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_324::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_324::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_324`]
module"]
#[doc(alias = "DENALI_PHY_324")]
pub type DenaliPhy324 = crate::Reg<denali_phy_324::DenaliPhy324Spec>;
#[doc = ""]
pub mod denali_phy_324;
#[doc = "DENALI_PHY_325 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_325::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_325::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_325`]
module"]
#[doc(alias = "DENALI_PHY_325")]
pub type DenaliPhy325 = crate::Reg<denali_phy_325::DenaliPhy325Spec>;
#[doc = ""]
pub mod denali_phy_325;
#[doc = "DENALI_PHY_326 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_326::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_326::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_326`]
module"]
#[doc(alias = "DENALI_PHY_326")]
pub type DenaliPhy326 = crate::Reg<denali_phy_326::DenaliPhy326Spec>;
#[doc = ""]
pub mod denali_phy_326;
#[doc = "DENALI_PHY_327 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_327::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_327::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_327`]
module"]
#[doc(alias = "DENALI_PHY_327")]
pub type DenaliPhy327 = crate::Reg<denali_phy_327::DenaliPhy327Spec>;
#[doc = ""]
pub mod denali_phy_327;
#[doc = "DENALI_PHY_328 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_328::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_328::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_328`]
module"]
#[doc(alias = "DENALI_PHY_328")]
pub type DenaliPhy328 = crate::Reg<denali_phy_328::DenaliPhy328Spec>;
#[doc = ""]
pub mod denali_phy_328;
#[doc = "DENALI_PHY_329 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_329::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_329::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_329`]
module"]
#[doc(alias = "DENALI_PHY_329")]
pub type DenaliPhy329 = crate::Reg<denali_phy_329::DenaliPhy329Spec>;
#[doc = ""]
pub mod denali_phy_329;
#[doc = "DENALI_PHY_330 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_330::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_330::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_330`]
module"]
#[doc(alias = "DENALI_PHY_330")]
pub type DenaliPhy330 = crate::Reg<denali_phy_330::DenaliPhy330Spec>;
#[doc = ""]
pub mod denali_phy_330;
#[doc = "DENALI_PHY_331 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_331::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_331::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_331`]
module"]
#[doc(alias = "DENALI_PHY_331")]
pub type DenaliPhy331 = crate::Reg<denali_phy_331::DenaliPhy331Spec>;
#[doc = ""]
pub mod denali_phy_331;
#[doc = "DENALI_PHY_332 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_332::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_332::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_332`]
module"]
#[doc(alias = "DENALI_PHY_332")]
pub type DenaliPhy332 = crate::Reg<denali_phy_332::DenaliPhy332Spec>;
#[doc = ""]
pub mod denali_phy_332;
#[doc = "DENALI_PHY_333 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_333::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_333::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_333`]
module"]
#[doc(alias = "DENALI_PHY_333")]
pub type DenaliPhy333 = crate::Reg<denali_phy_333::DenaliPhy333Spec>;
#[doc = ""]
pub mod denali_phy_333;
#[doc = "DENALI_PHY_334 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_334::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_334::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_334`]
module"]
#[doc(alias = "DENALI_PHY_334")]
pub type DenaliPhy334 = crate::Reg<denali_phy_334::DenaliPhy334Spec>;
#[doc = ""]
pub mod denali_phy_334;
#[doc = "DENALI_PHY_335 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_335::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_335::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_335`]
module"]
#[doc(alias = "DENALI_PHY_335")]
pub type DenaliPhy335 = crate::Reg<denali_phy_335::DenaliPhy335Spec>;
#[doc = ""]
pub mod denali_phy_335;
#[doc = "DENALI_PHY_336 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_336::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_336::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_336`]
module"]
#[doc(alias = "DENALI_PHY_336")]
pub type DenaliPhy336 = crate::Reg<denali_phy_336::DenaliPhy336Spec>;
#[doc = ""]
pub mod denali_phy_336;
#[doc = "DENALI_PHY_337 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_337::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_337::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_337`]
module"]
#[doc(alias = "DENALI_PHY_337")]
pub type DenaliPhy337 = crate::Reg<denali_phy_337::DenaliPhy337Spec>;
#[doc = ""]
pub mod denali_phy_337;
#[doc = "DENALI_PHY_339 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_339::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_339::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_339`]
module"]
#[doc(alias = "DENALI_PHY_339")]
pub type DenaliPhy339 = crate::Reg<denali_phy_339::DenaliPhy339Spec>;
#[doc = ""]
pub mod denali_phy_339;
#[doc = "DENALI_PHY_340 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_340::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_340::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_340`]
module"]
#[doc(alias = "DENALI_PHY_340")]
pub type DenaliPhy340 = crate::Reg<denali_phy_340::DenaliPhy340Spec>;
#[doc = ""]
pub mod denali_phy_340;
#[doc = "DENALI_PHY_341 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_341::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_341::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_341`]
module"]
#[doc(alias = "DENALI_PHY_341")]
pub type DenaliPhy341 = crate::Reg<denali_phy_341::DenaliPhy341Spec>;
#[doc = ""]
pub mod denali_phy_341;
#[doc = "DENALI_PHY_342 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_342::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_342::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_342`]
module"]
#[doc(alias = "DENALI_PHY_342")]
pub type DenaliPhy342 = crate::Reg<denali_phy_342::DenaliPhy342Spec>;
#[doc = ""]
pub mod denali_phy_342;
#[doc = "DENALI_PHY_343 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_343::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_343::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_343`]
module"]
#[doc(alias = "DENALI_PHY_343")]
pub type DenaliPhy343 = crate::Reg<denali_phy_343::DenaliPhy343Spec>;
#[doc = ""]
pub mod denali_phy_343;
#[doc = "DENALI_PHY_344 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_344::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_344::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_344`]
module"]
#[doc(alias = "DENALI_PHY_344")]
pub type DenaliPhy344 = crate::Reg<denali_phy_344::DenaliPhy344Spec>;
#[doc = ""]
pub mod denali_phy_344;
#[doc = "DENALI_PHY_345 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_345::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_345::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_345`]
module"]
#[doc(alias = "DENALI_PHY_345")]
pub type DenaliPhy345 = crate::Reg<denali_phy_345::DenaliPhy345Spec>;
#[doc = ""]
pub mod denali_phy_345;
#[doc = "DENALI_PHY_346 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_346::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_346::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_346`]
module"]
#[doc(alias = "DENALI_PHY_346")]
pub type DenaliPhy346 = crate::Reg<denali_phy_346::DenaliPhy346Spec>;
#[doc = ""]
pub mod denali_phy_346;
#[doc = "DENALI_PHY_384 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_384::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_384::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_384`]
module"]
#[doc(alias = "DENALI_PHY_384")]
pub type DenaliPhy384 = crate::Reg<denali_phy_384::DenaliPhy384Spec>;
#[doc = ""]
pub mod denali_phy_384;
#[doc = "DENALI_PHY_385 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_385::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_385::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_385`]
module"]
#[doc(alias = "DENALI_PHY_385")]
pub type DenaliPhy385 = crate::Reg<denali_phy_385::DenaliPhy385Spec>;
#[doc = ""]
pub mod denali_phy_385;
#[doc = "DENALI_PHY_386 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_386::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_386::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_386`]
module"]
#[doc(alias = "DENALI_PHY_386")]
pub type DenaliPhy386 = crate::Reg<denali_phy_386::DenaliPhy386Spec>;
#[doc = ""]
pub mod denali_phy_386;
#[doc = "DENALI_PHY_387 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_387::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_387::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_387`]
module"]
#[doc(alias = "DENALI_PHY_387")]
pub type DenaliPhy387 = crate::Reg<denali_phy_387::DenaliPhy387Spec>;
#[doc = ""]
pub mod denali_phy_387;
#[doc = "DENALI_PHY_388 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_388::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_388::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_388`]
module"]
#[doc(alias = "DENALI_PHY_388")]
pub type DenaliPhy388 = crate::Reg<denali_phy_388::DenaliPhy388Spec>;
#[doc = ""]
pub mod denali_phy_388;
#[doc = "DENALI_PHY_389 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_389::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_389::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_389`]
module"]
#[doc(alias = "DENALI_PHY_389")]
pub type DenaliPhy389 = crate::Reg<denali_phy_389::DenaliPhy389Spec>;
#[doc = ""]
pub mod denali_phy_389;
#[doc = "DENALI_PHY_390 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_390::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_390::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_390`]
module"]
#[doc(alias = "DENALI_PHY_390")]
pub type DenaliPhy390 = crate::Reg<denali_phy_390::DenaliPhy390Spec>;
#[doc = ""]
pub mod denali_phy_390;
#[doc = "DENALI_PHY_391 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_391::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_391::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_391`]
module"]
#[doc(alias = "DENALI_PHY_391")]
pub type DenaliPhy391 = crate::Reg<denali_phy_391::DenaliPhy391Spec>;
#[doc = ""]
pub mod denali_phy_391;
#[doc = "DENALI_PHY_392 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_392::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_392::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_392`]
module"]
#[doc(alias = "DENALI_PHY_392")]
pub type DenaliPhy392 = crate::Reg<denali_phy_392::DenaliPhy392Spec>;
#[doc = ""]
pub mod denali_phy_392;
#[doc = "DENALI_PHY_393 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_393::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_393::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_393`]
module"]
#[doc(alias = "DENALI_PHY_393")]
pub type DenaliPhy393 = crate::Reg<denali_phy_393::DenaliPhy393Spec>;
#[doc = ""]
pub mod denali_phy_393;
#[doc = "DENALI_PHY_394 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_394::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_394::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_394`]
module"]
#[doc(alias = "DENALI_PHY_394")]
pub type DenaliPhy394 = crate::Reg<denali_phy_394::DenaliPhy394Spec>;
#[doc = ""]
pub mod denali_phy_394;
#[doc = "DENALI_PHY_395 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_395::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_395::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_395`]
module"]
#[doc(alias = "DENALI_PHY_395")]
pub type DenaliPhy395 = crate::Reg<denali_phy_395::DenaliPhy395Spec>;
#[doc = ""]
pub mod denali_phy_395;
#[doc = "DENALI_PHY_396 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_396::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_396::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_396`]
module"]
#[doc(alias = "DENALI_PHY_396")]
pub type DenaliPhy396 = crate::Reg<denali_phy_396::DenaliPhy396Spec>;
#[doc = ""]
pub mod denali_phy_396;
#[doc = "DENALI_PHY_397 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_397::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_397::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_397`]
module"]
#[doc(alias = "DENALI_PHY_397")]
pub type DenaliPhy397 = crate::Reg<denali_phy_397::DenaliPhy397Spec>;
#[doc = ""]
pub mod denali_phy_397;
#[doc = "DENALI_PHY_398 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_398::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_398`]
module"]
#[doc(alias = "DENALI_PHY_398")]
pub type DenaliPhy398 = crate::Reg<denali_phy_398::DenaliPhy398Spec>;
#[doc = ""]
pub mod denali_phy_398;
#[doc = "DENALI_PHY_399 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_399::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_399::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_399`]
module"]
#[doc(alias = "DENALI_PHY_399")]
pub type DenaliPhy399 = crate::Reg<denali_phy_399::DenaliPhy399Spec>;
#[doc = ""]
pub mod denali_phy_399;
#[doc = "DENALI_PHY_400 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_400::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_400::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_400`]
module"]
#[doc(alias = "DENALI_PHY_400")]
pub type DenaliPhy400 = crate::Reg<denali_phy_400::DenaliPhy400Spec>;
#[doc = ""]
pub mod denali_phy_400;
#[doc = "DENALI_PHY_401 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_401::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_401::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_401`]
module"]
#[doc(alias = "DENALI_PHY_401")]
pub type DenaliPhy401 = crate::Reg<denali_phy_401::DenaliPhy401Spec>;
#[doc = ""]
pub mod denali_phy_401;
#[doc = "DENALI_PHY_402 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_402::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_402::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_402`]
module"]
#[doc(alias = "DENALI_PHY_402")]
pub type DenaliPhy402 = crate::Reg<denali_phy_402::DenaliPhy402Spec>;
#[doc = ""]
pub mod denali_phy_402;
#[doc = "DENALI_PHY_403 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_403::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_403::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_403`]
module"]
#[doc(alias = "DENALI_PHY_403")]
pub type DenaliPhy403 = crate::Reg<denali_phy_403::DenaliPhy403Spec>;
#[doc = ""]
pub mod denali_phy_403;
#[doc = "DENALI_PHY_404 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_404::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_404::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_404`]
module"]
#[doc(alias = "DENALI_PHY_404")]
pub type DenaliPhy404 = crate::Reg<denali_phy_404::DenaliPhy404Spec>;
#[doc = ""]
pub mod denali_phy_404;
#[doc = "DENALI_PHY_405 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_405::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_405::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_405`]
module"]
#[doc(alias = "DENALI_PHY_405")]
pub type DenaliPhy405 = crate::Reg<denali_phy_405::DenaliPhy405Spec>;
#[doc = ""]
pub mod denali_phy_405;
#[doc = "DENALI_PHY_406 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_406::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_406::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_406`]
module"]
#[doc(alias = "DENALI_PHY_406")]
pub type DenaliPhy406 = crate::Reg<denali_phy_406::DenaliPhy406Spec>;
#[doc = ""]
pub mod denali_phy_406;
#[doc = "DENALI_PHY_407 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_407::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_407::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_407`]
module"]
#[doc(alias = "DENALI_PHY_407")]
pub type DenaliPhy407 = crate::Reg<denali_phy_407::DenaliPhy407Spec>;
#[doc = ""]
pub mod denali_phy_407;
#[doc = "DENALI_PHY_408 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_408::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_408::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_408`]
module"]
#[doc(alias = "DENALI_PHY_408")]
pub type DenaliPhy408 = crate::Reg<denali_phy_408::DenaliPhy408Spec>;
#[doc = ""]
pub mod denali_phy_408;
#[doc = "DENALI_PHY_409 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_409::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_409::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_409`]
module"]
#[doc(alias = "DENALI_PHY_409")]
pub type DenaliPhy409 = crate::Reg<denali_phy_409::DenaliPhy409Spec>;
#[doc = ""]
pub mod denali_phy_409;
#[doc = "DENALI_PHY_410 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_410::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_410::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_410`]
module"]
#[doc(alias = "DENALI_PHY_410")]
pub type DenaliPhy410 = crate::Reg<denali_phy_410::DenaliPhy410Spec>;
#[doc = ""]
pub mod denali_phy_410;
#[doc = "DENALI_PHY_411 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_411::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_411::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_411`]
module"]
#[doc(alias = "DENALI_PHY_411")]
pub type DenaliPhy411 = crate::Reg<denali_phy_411::DenaliPhy411Spec>;
#[doc = ""]
pub mod denali_phy_411;
#[doc = "DENALI_PHY_412 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_412::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_412::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_412`]
module"]
#[doc(alias = "DENALI_PHY_412")]
pub type DenaliPhy412 = crate::Reg<denali_phy_412::DenaliPhy412Spec>;
#[doc = ""]
pub mod denali_phy_412;
#[doc = "DENALI_PHY_413 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_413::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_413::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_413`]
module"]
#[doc(alias = "DENALI_PHY_413")]
pub type DenaliPhy413 = crate::Reg<denali_phy_413::DenaliPhy413Spec>;
#[doc = ""]
pub mod denali_phy_413;
#[doc = "DENALI_PHY_414 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_414::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_414::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_414`]
module"]
#[doc(alias = "DENALI_PHY_414")]
pub type DenaliPhy414 = crate::Reg<denali_phy_414::DenaliPhy414Spec>;
#[doc = ""]
pub mod denali_phy_414;
#[doc = "DENALI_PHY_415 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_415::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_415::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_415`]
module"]
#[doc(alias = "DENALI_PHY_415")]
pub type DenaliPhy415 = crate::Reg<denali_phy_415::DenaliPhy415Spec>;
#[doc = ""]
pub mod denali_phy_415;
#[doc = "DENALI_PHY_416 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_416::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_416::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_416`]
module"]
#[doc(alias = "DENALI_PHY_416")]
pub type DenaliPhy416 = crate::Reg<denali_phy_416::DenaliPhy416Spec>;
#[doc = ""]
pub mod denali_phy_416;
#[doc = "DENALI_PHY_417 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_417::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_417`]
module"]
#[doc(alias = "DENALI_PHY_417")]
pub type DenaliPhy417 = crate::Reg<denali_phy_417::DenaliPhy417Spec>;
#[doc = ""]
pub mod denali_phy_417;
#[doc = "DENALI_PHY_418 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_418::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_418`]
module"]
#[doc(alias = "DENALI_PHY_418")]
pub type DenaliPhy418 = crate::Reg<denali_phy_418::DenaliPhy418Spec>;
#[doc = ""]
pub mod denali_phy_418;
#[doc = "DENALI_PHY_419 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_419::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_419`]
module"]
#[doc(alias = "DENALI_PHY_419")]
pub type DenaliPhy419 = crate::Reg<denali_phy_419::DenaliPhy419Spec>;
#[doc = ""]
pub mod denali_phy_419;
#[doc = "DENALI_PHY_420 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_420::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_420`]
module"]
#[doc(alias = "DENALI_PHY_420")]
pub type DenaliPhy420 = crate::Reg<denali_phy_420::DenaliPhy420Spec>;
#[doc = ""]
pub mod denali_phy_420;
#[doc = "DENALI_PHY_421 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_421::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_421`]
module"]
#[doc(alias = "DENALI_PHY_421")]
pub type DenaliPhy421 = crate::Reg<denali_phy_421::DenaliPhy421Spec>;
#[doc = ""]
pub mod denali_phy_421;
#[doc = "DENALI_PHY_422 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_422::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_422`]
module"]
#[doc(alias = "DENALI_PHY_422")]
pub type DenaliPhy422 = crate::Reg<denali_phy_422::DenaliPhy422Spec>;
#[doc = ""]
pub mod denali_phy_422;
#[doc = "DENALI_PHY_423 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_423::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_423`]
module"]
#[doc(alias = "DENALI_PHY_423")]
pub type DenaliPhy423 = crate::Reg<denali_phy_423::DenaliPhy423Spec>;
#[doc = ""]
pub mod denali_phy_423;
#[doc = "DENALI_PHY_424 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_424::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_424`]
module"]
#[doc(alias = "DENALI_PHY_424")]
pub type DenaliPhy424 = crate::Reg<denali_phy_424::DenaliPhy424Spec>;
#[doc = ""]
pub mod denali_phy_424;
#[doc = "DENALI_PHY_425 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_425::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_425`]
module"]
#[doc(alias = "DENALI_PHY_425")]
pub type DenaliPhy425 = crate::Reg<denali_phy_425::DenaliPhy425Spec>;
#[doc = ""]
pub mod denali_phy_425;
#[doc = "DENALI_PHY_426 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_426::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_426`]
module"]
#[doc(alias = "DENALI_PHY_426")]
pub type DenaliPhy426 = crate::Reg<denali_phy_426::DenaliPhy426Spec>;
#[doc = ""]
pub mod denali_phy_426;
#[doc = "DENALI_PHY_427 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_427::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_427`]
module"]
#[doc(alias = "DENALI_PHY_427")]
pub type DenaliPhy427 = crate::Reg<denali_phy_427::DenaliPhy427Spec>;
#[doc = ""]
pub mod denali_phy_427;
#[doc = "DENALI_PHY_428 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_428::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_428`]
module"]
#[doc(alias = "DENALI_PHY_428")]
pub type DenaliPhy428 = crate::Reg<denali_phy_428::DenaliPhy428Spec>;
#[doc = ""]
pub mod denali_phy_428;
#[doc = "DENALI_PHY_429 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_429::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_429`]
module"]
#[doc(alias = "DENALI_PHY_429")]
pub type DenaliPhy429 = crate::Reg<denali_phy_429::DenaliPhy429Spec>;
#[doc = ""]
pub mod denali_phy_429;
#[doc = "DENALI_PHY_430 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_430::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_430`]
module"]
#[doc(alias = "DENALI_PHY_430")]
pub type DenaliPhy430 = crate::Reg<denali_phy_430::DenaliPhy430Spec>;
#[doc = ""]
pub mod denali_phy_430;
#[doc = "DENALI_PHY_431 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_431::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_431`]
module"]
#[doc(alias = "DENALI_PHY_431")]
pub type DenaliPhy431 = crate::Reg<denali_phy_431::DenaliPhy431Spec>;
#[doc = ""]
pub mod denali_phy_431;
#[doc = "DENALI_PHY_432 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_432::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_432`]
module"]
#[doc(alias = "DENALI_PHY_432")]
pub type DenaliPhy432 = crate::Reg<denali_phy_432::DenaliPhy432Spec>;
#[doc = ""]
pub mod denali_phy_432;
#[doc = "DENALI_PHY_433 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_433::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_433::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_433`]
module"]
#[doc(alias = "DENALI_PHY_433")]
pub type DenaliPhy433 = crate::Reg<denali_phy_433::DenaliPhy433Spec>;
#[doc = ""]
pub mod denali_phy_433;
#[doc = "DENALI_PHY_434 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_434::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_434`]
module"]
#[doc(alias = "DENALI_PHY_434")]
pub type DenaliPhy434 = crate::Reg<denali_phy_434::DenaliPhy434Spec>;
#[doc = ""]
pub mod denali_phy_434;
#[doc = "DENALI_PHY_435 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_435::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_435`]
module"]
#[doc(alias = "DENALI_PHY_435")]
pub type DenaliPhy435 = crate::Reg<denali_phy_435::DenaliPhy435Spec>;
#[doc = ""]
pub mod denali_phy_435;
#[doc = "DENALI_PHY_436 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_436::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_436::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_436`]
module"]
#[doc(alias = "DENALI_PHY_436")]
pub type DenaliPhy436 = crate::Reg<denali_phy_436::DenaliPhy436Spec>;
#[doc = ""]
pub mod denali_phy_436;
#[doc = "DENALI_PHY_437 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_437::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_437::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_437`]
module"]
#[doc(alias = "DENALI_PHY_437")]
pub type DenaliPhy437 = crate::Reg<denali_phy_437::DenaliPhy437Spec>;
#[doc = ""]
pub mod denali_phy_437;
#[doc = "DENALI_PHY_438 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_438::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_438::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_438`]
module"]
#[doc(alias = "DENALI_PHY_438")]
pub type DenaliPhy438 = crate::Reg<denali_phy_438::DenaliPhy438Spec>;
#[doc = ""]
pub mod denali_phy_438;
#[doc = "DENALI_PHY_439 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_439::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_439::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_439`]
module"]
#[doc(alias = "DENALI_PHY_439")]
pub type DenaliPhy439 = crate::Reg<denali_phy_439::DenaliPhy439Spec>;
#[doc = ""]
pub mod denali_phy_439;
#[doc = "DENALI_PHY_440 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_440::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_440::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_440`]
module"]
#[doc(alias = "DENALI_PHY_440")]
pub type DenaliPhy440 = crate::Reg<denali_phy_440::DenaliPhy440Spec>;
#[doc = ""]
pub mod denali_phy_440;
#[doc = "DENALI_PHY_441 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_441::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_441::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_441`]
module"]
#[doc(alias = "DENALI_PHY_441")]
pub type DenaliPhy441 = crate::Reg<denali_phy_441::DenaliPhy441Spec>;
#[doc = ""]
pub mod denali_phy_441;
#[doc = "DENALI_PHY_442 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_442::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_442::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_442`]
module"]
#[doc(alias = "DENALI_PHY_442")]
pub type DenaliPhy442 = crate::Reg<denali_phy_442::DenaliPhy442Spec>;
#[doc = ""]
pub mod denali_phy_442;
#[doc = "DENALI_PHY_443 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_443::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_443::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_443`]
module"]
#[doc(alias = "DENALI_PHY_443")]
pub type DenaliPhy443 = crate::Reg<denali_phy_443::DenaliPhy443Spec>;
#[doc = ""]
pub mod denali_phy_443;
#[doc = "DENALI_PHY_444 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_444::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_444::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_444`]
module"]
#[doc(alias = "DENALI_PHY_444")]
pub type DenaliPhy444 = crate::Reg<denali_phy_444::DenaliPhy444Spec>;
#[doc = ""]
pub mod denali_phy_444;
#[doc = "DENALI_PHY_445 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_445::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_445::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_445`]
module"]
#[doc(alias = "DENALI_PHY_445")]
pub type DenaliPhy445 = crate::Reg<denali_phy_445::DenaliPhy445Spec>;
#[doc = ""]
pub mod denali_phy_445;
#[doc = "DENALI_PHY_446 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_446::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_446::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_446`]
module"]
#[doc(alias = "DENALI_PHY_446")]
pub type DenaliPhy446 = crate::Reg<denali_phy_446::DenaliPhy446Spec>;
#[doc = ""]
pub mod denali_phy_446;
#[doc = "DENALI_PHY_447 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_447::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_447::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_447`]
module"]
#[doc(alias = "DENALI_PHY_447")]
pub type DenaliPhy447 = crate::Reg<denali_phy_447::DenaliPhy447Spec>;
#[doc = ""]
pub mod denali_phy_447;
#[doc = "DENALI_PHY_448 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_448::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_448::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_448`]
module"]
#[doc(alias = "DENALI_PHY_448")]
pub type DenaliPhy448 = crate::Reg<denali_phy_448::DenaliPhy448Spec>;
#[doc = ""]
pub mod denali_phy_448;
#[doc = "DENALI_PHY_449 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_449::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_449::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_449`]
module"]
#[doc(alias = "DENALI_PHY_449")]
pub type DenaliPhy449 = crate::Reg<denali_phy_449::DenaliPhy449Spec>;
#[doc = ""]
pub mod denali_phy_449;
#[doc = "DENALI_PHY_450 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_450::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_450::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_450`]
module"]
#[doc(alias = "DENALI_PHY_450")]
pub type DenaliPhy450 = crate::Reg<denali_phy_450::DenaliPhy450Spec>;
#[doc = ""]
pub mod denali_phy_450;
#[doc = "DENALI_PHY_451 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_451::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_451::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_451`]
module"]
#[doc(alias = "DENALI_PHY_451")]
pub type DenaliPhy451 = crate::Reg<denali_phy_451::DenaliPhy451Spec>;
#[doc = ""]
pub mod denali_phy_451;
#[doc = "DENALI_PHY_452 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_452::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_452::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_452`]
module"]
#[doc(alias = "DENALI_PHY_452")]
pub type DenaliPhy452 = crate::Reg<denali_phy_452::DenaliPhy452Spec>;
#[doc = ""]
pub mod denali_phy_452;
#[doc = "DENALI_PHY_453 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_453::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_453::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_453`]
module"]
#[doc(alias = "DENALI_PHY_453")]
pub type DenaliPhy453 = crate::Reg<denali_phy_453::DenaliPhy453Spec>;
#[doc = ""]
pub mod denali_phy_453;
#[doc = "DENALI_PHY_454 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_454::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_454::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_454`]
module"]
#[doc(alias = "DENALI_PHY_454")]
pub type DenaliPhy454 = crate::Reg<denali_phy_454::DenaliPhy454Spec>;
#[doc = ""]
pub mod denali_phy_454;
#[doc = "DENALI_PHY_455 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_455::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_455::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_455`]
module"]
#[doc(alias = "DENALI_PHY_455")]
pub type DenaliPhy455 = crate::Reg<denali_phy_455::DenaliPhy455Spec>;
#[doc = ""]
pub mod denali_phy_455;
#[doc = "DENALI_PHY_456 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_456::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_456::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_456`]
module"]
#[doc(alias = "DENALI_PHY_456")]
pub type DenaliPhy456 = crate::Reg<denali_phy_456::DenaliPhy456Spec>;
#[doc = ""]
pub mod denali_phy_456;
#[doc = "DENALI_PHY_457 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_457::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_457::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_457`]
module"]
#[doc(alias = "DENALI_PHY_457")]
pub type DenaliPhy457 = crate::Reg<denali_phy_457::DenaliPhy457Spec>;
#[doc = ""]
pub mod denali_phy_457;
#[doc = "DENALI_PHY_458 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_458::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_458::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_458`]
module"]
#[doc(alias = "DENALI_PHY_458")]
pub type DenaliPhy458 = crate::Reg<denali_phy_458::DenaliPhy458Spec>;
#[doc = ""]
pub mod denali_phy_458;
#[doc = "DENALI_PHY_459 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_459::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_459::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_459`]
module"]
#[doc(alias = "DENALI_PHY_459")]
pub type DenaliPhy459 = crate::Reg<denali_phy_459::DenaliPhy459Spec>;
#[doc = ""]
pub mod denali_phy_459;
#[doc = "DENALI_PHY_460 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_460::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_460::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_460`]
module"]
#[doc(alias = "DENALI_PHY_460")]
pub type DenaliPhy460 = crate::Reg<denali_phy_460::DenaliPhy460Spec>;
#[doc = ""]
pub mod denali_phy_460;
#[doc = "DENALI_PHY_461 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_461::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_461::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_461`]
module"]
#[doc(alias = "DENALI_PHY_461")]
pub type DenaliPhy461 = crate::Reg<denali_phy_461::DenaliPhy461Spec>;
#[doc = ""]
pub mod denali_phy_461;
#[doc = "DENALI_PHY_462 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_462::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_462::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_462`]
module"]
#[doc(alias = "DENALI_PHY_462")]
pub type DenaliPhy462 = crate::Reg<denali_phy_462::DenaliPhy462Spec>;
#[doc = ""]
pub mod denali_phy_462;
#[doc = "DENALI_PHY_463 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_463::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_463::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_463`]
module"]
#[doc(alias = "DENALI_PHY_463")]
pub type DenaliPhy463 = crate::Reg<denali_phy_463::DenaliPhy463Spec>;
#[doc = ""]
pub mod denali_phy_463;
#[doc = "DENALI_PHY_464 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_464::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_464::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_464`]
module"]
#[doc(alias = "DENALI_PHY_464")]
pub type DenaliPhy464 = crate::Reg<denali_phy_464::DenaliPhy464Spec>;
#[doc = ""]
pub mod denali_phy_464;
#[doc = "DENALI_PHY_465 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_465::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_465::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_465`]
module"]
#[doc(alias = "DENALI_PHY_465")]
pub type DenaliPhy465 = crate::Reg<denali_phy_465::DenaliPhy465Spec>;
#[doc = ""]
pub mod denali_phy_465;
#[doc = "DENALI_PHY_467 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_467::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_467::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_467`]
module"]
#[doc(alias = "DENALI_PHY_467")]
pub type DenaliPhy467 = crate::Reg<denali_phy_467::DenaliPhy467Spec>;
#[doc = ""]
pub mod denali_phy_467;
#[doc = "DENALI_PHY_468 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_468::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_468::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_468`]
module"]
#[doc(alias = "DENALI_PHY_468")]
pub type DenaliPhy468 = crate::Reg<denali_phy_468::DenaliPhy468Spec>;
#[doc = ""]
pub mod denali_phy_468;
#[doc = "DENALI_PHY_469 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_469::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_469::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_469`]
module"]
#[doc(alias = "DENALI_PHY_469")]
pub type DenaliPhy469 = crate::Reg<denali_phy_469::DenaliPhy469Spec>;
#[doc = ""]
pub mod denali_phy_469;
#[doc = "DENALI_PHY_470 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_470::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_470::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_470`]
module"]
#[doc(alias = "DENALI_PHY_470")]
pub type DenaliPhy470 = crate::Reg<denali_phy_470::DenaliPhy470Spec>;
#[doc = ""]
pub mod denali_phy_470;
#[doc = "DENALI_PHY_471 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_471::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_471::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_471`]
module"]
#[doc(alias = "DENALI_PHY_471")]
pub type DenaliPhy471 = crate::Reg<denali_phy_471::DenaliPhy471Spec>;
#[doc = ""]
pub mod denali_phy_471;
#[doc = "DENALI_PHY_472 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_472::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_472::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_472`]
module"]
#[doc(alias = "DENALI_PHY_472")]
pub type DenaliPhy472 = crate::Reg<denali_phy_472::DenaliPhy472Spec>;
#[doc = ""]
pub mod denali_phy_472;
#[doc = "DENALI_PHY_473 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_473::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_473::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_473`]
module"]
#[doc(alias = "DENALI_PHY_473")]
pub type DenaliPhy473 = crate::Reg<denali_phy_473::DenaliPhy473Spec>;
#[doc = ""]
pub mod denali_phy_473;
#[doc = "DENALI_PHY_474 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_474::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_474::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_474`]
module"]
#[doc(alias = "DENALI_PHY_474")]
pub type DenaliPhy474 = crate::Reg<denali_phy_474::DenaliPhy474Spec>;
#[doc = ""]
pub mod denali_phy_474;
#[doc = "DENALI_PHY_512 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_512::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_512::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_512`]
module"]
#[doc(alias = "DENALI_PHY_512")]
pub type DenaliPhy512 = crate::Reg<denali_phy_512::DenaliPhy512Spec>;
#[doc = ""]
pub mod denali_phy_512;
#[doc = "DENALI_PHY_513 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_513::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_513::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_513`]
module"]
#[doc(alias = "DENALI_PHY_513")]
pub type DenaliPhy513 = crate::Reg<denali_phy_513::DenaliPhy513Spec>;
#[doc = ""]
pub mod denali_phy_513;
#[doc = "DENALI_PHY_514 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_514::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_514::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_514`]
module"]
#[doc(alias = "DENALI_PHY_514")]
pub type DenaliPhy514 = crate::Reg<denali_phy_514::DenaliPhy514Spec>;
#[doc = ""]
pub mod denali_phy_514;
#[doc = "DENALI_PHY_515 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_515::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_515`]
module"]
#[doc(alias = "DENALI_PHY_515")]
pub type DenaliPhy515 = crate::Reg<denali_phy_515::DenaliPhy515Spec>;
#[doc = ""]
pub mod denali_phy_515;
#[doc = "DENALI_PHY_516 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_516::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_516::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_516`]
module"]
#[doc(alias = "DENALI_PHY_516")]
pub type DenaliPhy516 = crate::Reg<denali_phy_516::DenaliPhy516Spec>;
#[doc = ""]
pub mod denali_phy_516;
#[doc = "DENALI_PHY_517 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_517::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_517::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_517`]
module"]
#[doc(alias = "DENALI_PHY_517")]
pub type DenaliPhy517 = crate::Reg<denali_phy_517::DenaliPhy517Spec>;
#[doc = ""]
pub mod denali_phy_517;
#[doc = "DENALI_PHY_518 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_518::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_518::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_518`]
module"]
#[doc(alias = "DENALI_PHY_518")]
pub type DenaliPhy518 = crate::Reg<denali_phy_518::DenaliPhy518Spec>;
#[doc = ""]
pub mod denali_phy_518;
#[doc = "DENALI_PHY_519 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_519::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_519::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_519`]
module"]
#[doc(alias = "DENALI_PHY_519")]
pub type DenaliPhy519 = crate::Reg<denali_phy_519::DenaliPhy519Spec>;
#[doc = ""]
pub mod denali_phy_519;
#[doc = "DENALI_PHY_520 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_520::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_520::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_520`]
module"]
#[doc(alias = "DENALI_PHY_520")]
pub type DenaliPhy520 = crate::Reg<denali_phy_520::DenaliPhy520Spec>;
#[doc = ""]
pub mod denali_phy_520;
#[doc = "DENALI_PHY_521 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_521::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_521`]
module"]
#[doc(alias = "DENALI_PHY_521")]
pub type DenaliPhy521 = crate::Reg<denali_phy_521::DenaliPhy521Spec>;
#[doc = ""]
pub mod denali_phy_521;
#[doc = "DENALI_PHY_522 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_522::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_522`]
module"]
#[doc(alias = "DENALI_PHY_522")]
pub type DenaliPhy522 = crate::Reg<denali_phy_522::DenaliPhy522Spec>;
#[doc = ""]
pub mod denali_phy_522;
#[doc = "DENALI_PHY_523 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_523::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_523::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_523`]
module"]
#[doc(alias = "DENALI_PHY_523")]
pub type DenaliPhy523 = crate::Reg<denali_phy_523::DenaliPhy523Spec>;
#[doc = ""]
pub mod denali_phy_523;
#[doc = "DENALI_PHY_524 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_524::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_524::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_524`]
module"]
#[doc(alias = "DENALI_PHY_524")]
pub type DenaliPhy524 = crate::Reg<denali_phy_524::DenaliPhy524Spec>;
#[doc = ""]
pub mod denali_phy_524;
#[doc = "DENALI_PHY_525 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_525::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_525::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_525`]
module"]
#[doc(alias = "DENALI_PHY_525")]
pub type DenaliPhy525 = crate::Reg<denali_phy_525::DenaliPhy525Spec>;
#[doc = ""]
pub mod denali_phy_525;
#[doc = "DENALI_PHY_526 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_526::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_526::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_526`]
module"]
#[doc(alias = "DENALI_PHY_526")]
pub type DenaliPhy526 = crate::Reg<denali_phy_526::DenaliPhy526Spec>;
#[doc = ""]
pub mod denali_phy_526;
#[doc = "DENALI_PHY_527 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_527::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_527::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_527`]
module"]
#[doc(alias = "DENALI_PHY_527")]
pub type DenaliPhy527 = crate::Reg<denali_phy_527::DenaliPhy527Spec>;
#[doc = ""]
pub mod denali_phy_527;
#[doc = "DENALI_PHY_528 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_528::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_528::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_528`]
module"]
#[doc(alias = "DENALI_PHY_528")]
pub type DenaliPhy528 = crate::Reg<denali_phy_528::DenaliPhy528Spec>;
#[doc = ""]
pub mod denali_phy_528;
#[doc = "DENALI_PHY_529 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_529::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_529::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_529`]
module"]
#[doc(alias = "DENALI_PHY_529")]
pub type DenaliPhy529 = crate::Reg<denali_phy_529::DenaliPhy529Spec>;
#[doc = ""]
pub mod denali_phy_529;
#[doc = "DENALI_PHY_530 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_530::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_530::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_530`]
module"]
#[doc(alias = "DENALI_PHY_530")]
pub type DenaliPhy530 = crate::Reg<denali_phy_530::DenaliPhy530Spec>;
#[doc = ""]
pub mod denali_phy_530;
#[doc = "DENALI_PHY_531 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_531::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_531`]
module"]
#[doc(alias = "DENALI_PHY_531")]
pub type DenaliPhy531 = crate::Reg<denali_phy_531::DenaliPhy531Spec>;
#[doc = ""]
pub mod denali_phy_531;
#[doc = "DENALI_PHY_532 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_532::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_532`]
module"]
#[doc(alias = "DENALI_PHY_532")]
pub type DenaliPhy532 = crate::Reg<denali_phy_532::DenaliPhy532Spec>;
#[doc = ""]
pub mod denali_phy_532;
#[doc = "DENALI_PHY_533 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_533::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_533::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_533`]
module"]
#[doc(alias = "DENALI_PHY_533")]
pub type DenaliPhy533 = crate::Reg<denali_phy_533::DenaliPhy533Spec>;
#[doc = ""]
pub mod denali_phy_533;
#[doc = "DENALI_PHY_534 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_534::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_534::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_534`]
module"]
#[doc(alias = "DENALI_PHY_534")]
pub type DenaliPhy534 = crate::Reg<denali_phy_534::DenaliPhy534Spec>;
#[doc = ""]
pub mod denali_phy_534;
#[doc = "DENALI_PHY_535 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_535::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_535::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_535`]
module"]
#[doc(alias = "DENALI_PHY_535")]
pub type DenaliPhy535 = crate::Reg<denali_phy_535::DenaliPhy535Spec>;
#[doc = ""]
pub mod denali_phy_535;
#[doc = "DENALI_PHY_536 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_536::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_536::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_536`]
module"]
#[doc(alias = "DENALI_PHY_536")]
pub type DenaliPhy536 = crate::Reg<denali_phy_536::DenaliPhy536Spec>;
#[doc = ""]
pub mod denali_phy_536;
#[doc = "DENALI_PHY_537 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_537::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_537::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_537`]
module"]
#[doc(alias = "DENALI_PHY_537")]
pub type DenaliPhy537 = crate::Reg<denali_phy_537::DenaliPhy537Spec>;
#[doc = ""]
pub mod denali_phy_537;
#[doc = "DENALI_PHY_538 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_538::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_538::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_538`]
module"]
#[doc(alias = "DENALI_PHY_538")]
pub type DenaliPhy538 = crate::Reg<denali_phy_538::DenaliPhy538Spec>;
#[doc = ""]
pub mod denali_phy_538;
#[doc = "DENALI_PHY_539 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_539::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_539::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_539`]
module"]
#[doc(alias = "DENALI_PHY_539")]
pub type DenaliPhy539 = crate::Reg<denali_phy_539::DenaliPhy539Spec>;
#[doc = ""]
pub mod denali_phy_539;
#[doc = "DENALI_PHY_540 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_540::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_540::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_540`]
module"]
#[doc(alias = "DENALI_PHY_540")]
pub type DenaliPhy540 = crate::Reg<denali_phy_540::DenaliPhy540Spec>;
#[doc = ""]
pub mod denali_phy_540;
#[doc = "DENALI_PHY_541 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_541::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_541::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_541`]
module"]
#[doc(alias = "DENALI_PHY_541")]
pub type DenaliPhy541 = crate::Reg<denali_phy_541::DenaliPhy541Spec>;
#[doc = ""]
pub mod denali_phy_541;
#[doc = "DENALI_PHY_542 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_542::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_542::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_542`]
module"]
#[doc(alias = "DENALI_PHY_542")]
pub type DenaliPhy542 = crate::Reg<denali_phy_542::DenaliPhy542Spec>;
#[doc = ""]
pub mod denali_phy_542;
#[doc = "DENALI_PHY_543 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_543::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_543::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_543`]
module"]
#[doc(alias = "DENALI_PHY_543")]
pub type DenaliPhy543 = crate::Reg<denali_phy_543::DenaliPhy543Spec>;
#[doc = ""]
pub mod denali_phy_543;
#[doc = "DENALI_PHY_544 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_544::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_544::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_544`]
module"]
#[doc(alias = "DENALI_PHY_544")]
pub type DenaliPhy544 = crate::Reg<denali_phy_544::DenaliPhy544Spec>;
#[doc = ""]
pub mod denali_phy_544;
#[doc = "DENALI_PHY_545 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_545::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_545::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_545`]
module"]
#[doc(alias = "DENALI_PHY_545")]
pub type DenaliPhy545 = crate::Reg<denali_phy_545::DenaliPhy545Spec>;
#[doc = ""]
pub mod denali_phy_545;
#[doc = "DENALI_PHY_546 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_546::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_546::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_546`]
module"]
#[doc(alias = "DENALI_PHY_546")]
pub type DenaliPhy546 = crate::Reg<denali_phy_546::DenaliPhy546Spec>;
#[doc = ""]
pub mod denali_phy_546;
#[doc = "DENALI_PHY_547 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_547::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_547::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_547`]
module"]
#[doc(alias = "DENALI_PHY_547")]
pub type DenaliPhy547 = crate::Reg<denali_phy_547::DenaliPhy547Spec>;
#[doc = ""]
pub mod denali_phy_547;
#[doc = "DENALI_PHY_548 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_548::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_548::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_548`]
module"]
#[doc(alias = "DENALI_PHY_548")]
pub type DenaliPhy548 = crate::Reg<denali_phy_548::DenaliPhy548Spec>;
#[doc = ""]
pub mod denali_phy_548;
#[doc = "DENALI_PHY_549 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_549::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_549::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_549`]
module"]
#[doc(alias = "DENALI_PHY_549")]
pub type DenaliPhy549 = crate::Reg<denali_phy_549::DenaliPhy549Spec>;
#[doc = ""]
pub mod denali_phy_549;
#[doc = "DENALI_PHY_640 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_640::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_640::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_640`]
module"]
#[doc(alias = "DENALI_PHY_640")]
pub type DenaliPhy640 = crate::Reg<denali_phy_640::DenaliPhy640Spec>;
#[doc = ""]
pub mod denali_phy_640;
#[doc = "DENALI_PHY_641 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_641::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_641::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_641`]
module"]
#[doc(alias = "DENALI_PHY_641")]
pub type DenaliPhy641 = crate::Reg<denali_phy_641::DenaliPhy641Spec>;
#[doc = ""]
pub mod denali_phy_641;
#[doc = "DENALI_PHY_642 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_642::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_642::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_642`]
module"]
#[doc(alias = "DENALI_PHY_642")]
pub type DenaliPhy642 = crate::Reg<denali_phy_642::DenaliPhy642Spec>;
#[doc = ""]
pub mod denali_phy_642;
#[doc = "DENALI_PHY_643 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_643::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_643`]
module"]
#[doc(alias = "DENALI_PHY_643")]
pub type DenaliPhy643 = crate::Reg<denali_phy_643::DenaliPhy643Spec>;
#[doc = ""]
pub mod denali_phy_643;
#[doc = "DENALI_PHY_644 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_644::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_644::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_644`]
module"]
#[doc(alias = "DENALI_PHY_644")]
pub type DenaliPhy644 = crate::Reg<denali_phy_644::DenaliPhy644Spec>;
#[doc = ""]
pub mod denali_phy_644;
#[doc = "DENALI_PHY_645 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_645::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_645::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_645`]
module"]
#[doc(alias = "DENALI_PHY_645")]
pub type DenaliPhy645 = crate::Reg<denali_phy_645::DenaliPhy645Spec>;
#[doc = ""]
pub mod denali_phy_645;
#[doc = "DENALI_PHY_646 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_646::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_646::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_646`]
module"]
#[doc(alias = "DENALI_PHY_646")]
pub type DenaliPhy646 = crate::Reg<denali_phy_646::DenaliPhy646Spec>;
#[doc = ""]
pub mod denali_phy_646;
#[doc = "DENALI_PHY_647 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_647::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_647::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_647`]
module"]
#[doc(alias = "DENALI_PHY_647")]
pub type DenaliPhy647 = crate::Reg<denali_phy_647::DenaliPhy647Spec>;
#[doc = ""]
pub mod denali_phy_647;
#[doc = "DENALI_PHY_648 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_648::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_648::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_648`]
module"]
#[doc(alias = "DENALI_PHY_648")]
pub type DenaliPhy648 = crate::Reg<denali_phy_648::DenaliPhy648Spec>;
#[doc = ""]
pub mod denali_phy_648;
#[doc = "DENALI_PHY_649 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_649::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_649`]
module"]
#[doc(alias = "DENALI_PHY_649")]
pub type DenaliPhy649 = crate::Reg<denali_phy_649::DenaliPhy649Spec>;
#[doc = ""]
pub mod denali_phy_649;
#[doc = "DENALI_PHY_650 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_650::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_650`]
module"]
#[doc(alias = "DENALI_PHY_650")]
pub type DenaliPhy650 = crate::Reg<denali_phy_650::DenaliPhy650Spec>;
#[doc = ""]
pub mod denali_phy_650;
#[doc = "DENALI_PHY_651 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_651::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_651::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_651`]
module"]
#[doc(alias = "DENALI_PHY_651")]
pub type DenaliPhy651 = crate::Reg<denali_phy_651::DenaliPhy651Spec>;
#[doc = ""]
pub mod denali_phy_651;
#[doc = "DENALI_PHY_652 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_652::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_652::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_652`]
module"]
#[doc(alias = "DENALI_PHY_652")]
pub type DenaliPhy652 = crate::Reg<denali_phy_652::DenaliPhy652Spec>;
#[doc = ""]
pub mod denali_phy_652;
#[doc = "DENALI_PHY_653 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_653::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_653::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_653`]
module"]
#[doc(alias = "DENALI_PHY_653")]
pub type DenaliPhy653 = crate::Reg<denali_phy_653::DenaliPhy653Spec>;
#[doc = ""]
pub mod denali_phy_653;
#[doc = "DENALI_PHY_654 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_654::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_654::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_654`]
module"]
#[doc(alias = "DENALI_PHY_654")]
pub type DenaliPhy654 = crate::Reg<denali_phy_654::DenaliPhy654Spec>;
#[doc = ""]
pub mod denali_phy_654;
#[doc = "DENALI_PHY_655 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_655::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_655::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_655`]
module"]
#[doc(alias = "DENALI_PHY_655")]
pub type DenaliPhy655 = crate::Reg<denali_phy_655::DenaliPhy655Spec>;
#[doc = ""]
pub mod denali_phy_655;
#[doc = "DENALI_PHY_656 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_656::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_656::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_656`]
module"]
#[doc(alias = "DENALI_PHY_656")]
pub type DenaliPhy656 = crate::Reg<denali_phy_656::DenaliPhy656Spec>;
#[doc = ""]
pub mod denali_phy_656;
#[doc = "DENALI_PHY_657 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_657::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_657::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_657`]
module"]
#[doc(alias = "DENALI_PHY_657")]
pub type DenaliPhy657 = crate::Reg<denali_phy_657::DenaliPhy657Spec>;
#[doc = ""]
pub mod denali_phy_657;
#[doc = "DENALI_PHY_658 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_658::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_658::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_658`]
module"]
#[doc(alias = "DENALI_PHY_658")]
pub type DenaliPhy658 = crate::Reg<denali_phy_658::DenaliPhy658Spec>;
#[doc = ""]
pub mod denali_phy_658;
#[doc = "DENALI_PHY_659 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_659::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_659`]
module"]
#[doc(alias = "DENALI_PHY_659")]
pub type DenaliPhy659 = crate::Reg<denali_phy_659::DenaliPhy659Spec>;
#[doc = ""]
pub mod denali_phy_659;
#[doc = "DENALI_PHY_660 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_660::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_660`]
module"]
#[doc(alias = "DENALI_PHY_660")]
pub type DenaliPhy660 = crate::Reg<denali_phy_660::DenaliPhy660Spec>;
#[doc = ""]
pub mod denali_phy_660;
#[doc = "DENALI_PHY_661 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_661::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_661::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_661`]
module"]
#[doc(alias = "DENALI_PHY_661")]
pub type DenaliPhy661 = crate::Reg<denali_phy_661::DenaliPhy661Spec>;
#[doc = ""]
pub mod denali_phy_661;
#[doc = "DENALI_PHY_662 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_662::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_662::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_662`]
module"]
#[doc(alias = "DENALI_PHY_662")]
pub type DenaliPhy662 = crate::Reg<denali_phy_662::DenaliPhy662Spec>;
#[doc = ""]
pub mod denali_phy_662;
#[doc = "DENALI_PHY_663 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_663::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_663::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_663`]
module"]
#[doc(alias = "DENALI_PHY_663")]
pub type DenaliPhy663 = crate::Reg<denali_phy_663::DenaliPhy663Spec>;
#[doc = ""]
pub mod denali_phy_663;
#[doc = "DENALI_PHY_664 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_664::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_664::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_664`]
module"]
#[doc(alias = "DENALI_PHY_664")]
pub type DenaliPhy664 = crate::Reg<denali_phy_664::DenaliPhy664Spec>;
#[doc = ""]
pub mod denali_phy_664;
#[doc = "DENALI_PHY_665 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_665::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_665::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_665`]
module"]
#[doc(alias = "DENALI_PHY_665")]
pub type DenaliPhy665 = crate::Reg<denali_phy_665::DenaliPhy665Spec>;
#[doc = ""]
pub mod denali_phy_665;
#[doc = "DENALI_PHY_666 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_666::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_666::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_666`]
module"]
#[doc(alias = "DENALI_PHY_666")]
pub type DenaliPhy666 = crate::Reg<denali_phy_666::DenaliPhy666Spec>;
#[doc = ""]
pub mod denali_phy_666;
#[doc = "DENALI_PHY_667 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_667::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_667::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_667`]
module"]
#[doc(alias = "DENALI_PHY_667")]
pub type DenaliPhy667 = crate::Reg<denali_phy_667::DenaliPhy667Spec>;
#[doc = ""]
pub mod denali_phy_667;
#[doc = "DENALI_PHY_668 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_668::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_668::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_668`]
module"]
#[doc(alias = "DENALI_PHY_668")]
pub type DenaliPhy668 = crate::Reg<denali_phy_668::DenaliPhy668Spec>;
#[doc = ""]
pub mod denali_phy_668;
#[doc = "DENALI_PHY_669 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_669::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_669::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_669`]
module"]
#[doc(alias = "DENALI_PHY_669")]
pub type DenaliPhy669 = crate::Reg<denali_phy_669::DenaliPhy669Spec>;
#[doc = ""]
pub mod denali_phy_669;
#[doc = "DENALI_PHY_670 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_670::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_670::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_670`]
module"]
#[doc(alias = "DENALI_PHY_670")]
pub type DenaliPhy670 = crate::Reg<denali_phy_670::DenaliPhy670Spec>;
#[doc = ""]
pub mod denali_phy_670;
#[doc = "DENALI_PHY_671 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_671::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_671::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_671`]
module"]
#[doc(alias = "DENALI_PHY_671")]
pub type DenaliPhy671 = crate::Reg<denali_phy_671::DenaliPhy671Spec>;
#[doc = ""]
pub mod denali_phy_671;
#[doc = "DENALI_PHY_672 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_672::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_672::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_672`]
module"]
#[doc(alias = "DENALI_PHY_672")]
pub type DenaliPhy672 = crate::Reg<denali_phy_672::DenaliPhy672Spec>;
#[doc = ""]
pub mod denali_phy_672;
#[doc = "DENALI_PHY_673 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_673::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_673::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_673`]
module"]
#[doc(alias = "DENALI_PHY_673")]
pub type DenaliPhy673 = crate::Reg<denali_phy_673::DenaliPhy673Spec>;
#[doc = ""]
pub mod denali_phy_673;
#[doc = "DENALI_PHY_674 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_674::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_674::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_674`]
module"]
#[doc(alias = "DENALI_PHY_674")]
pub type DenaliPhy674 = crate::Reg<denali_phy_674::DenaliPhy674Spec>;
#[doc = ""]
pub mod denali_phy_674;
#[doc = "DENALI_PHY_675 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_675::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_675::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_675`]
module"]
#[doc(alias = "DENALI_PHY_675")]
pub type DenaliPhy675 = crate::Reg<denali_phy_675::DenaliPhy675Spec>;
#[doc = ""]
pub mod denali_phy_675;
#[doc = "DENALI_PHY_676 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_676::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_676::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_676`]
module"]
#[doc(alias = "DENALI_PHY_676")]
pub type DenaliPhy676 = crate::Reg<denali_phy_676::DenaliPhy676Spec>;
#[doc = ""]
pub mod denali_phy_676;
#[doc = "DENALI_PHY_677 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_677::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_677::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_677`]
module"]
#[doc(alias = "DENALI_PHY_677")]
pub type DenaliPhy677 = crate::Reg<denali_phy_677::DenaliPhy677Spec>;
#[doc = ""]
pub mod denali_phy_677;
#[doc = "DENALI_PHY_768 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_768::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_768::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_768`]
module"]
#[doc(alias = "DENALI_PHY_768")]
pub type DenaliPhy768 = crate::Reg<denali_phy_768::DenaliPhy768Spec>;
#[doc = ""]
pub mod denali_phy_768;
#[doc = "DENALI_PHY_769 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_769::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_769::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_769`]
module"]
#[doc(alias = "DENALI_PHY_769")]
pub type DenaliPhy769 = crate::Reg<denali_phy_769::DenaliPhy769Spec>;
#[doc = ""]
pub mod denali_phy_769;
#[doc = "DENALI_PHY_770 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_770::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_770::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_770`]
module"]
#[doc(alias = "DENALI_PHY_770")]
pub type DenaliPhy770 = crate::Reg<denali_phy_770::DenaliPhy770Spec>;
#[doc = ""]
pub mod denali_phy_770;
#[doc = "DENALI_PHY_771 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_771::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_771`]
module"]
#[doc(alias = "DENALI_PHY_771")]
pub type DenaliPhy771 = crate::Reg<denali_phy_771::DenaliPhy771Spec>;
#[doc = ""]
pub mod denali_phy_771;
#[doc = "DENALI_PHY_772 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_772::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_772::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_772`]
module"]
#[doc(alias = "DENALI_PHY_772")]
pub type DenaliPhy772 = crate::Reg<denali_phy_772::DenaliPhy772Spec>;
#[doc = ""]
pub mod denali_phy_772;
#[doc = "DENALI_PHY_773 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_773::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_773::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_773`]
module"]
#[doc(alias = "DENALI_PHY_773")]
pub type DenaliPhy773 = crate::Reg<denali_phy_773::DenaliPhy773Spec>;
#[doc = ""]
pub mod denali_phy_773;
#[doc = "DENALI_PHY_774 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_774::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_774::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_774`]
module"]
#[doc(alias = "DENALI_PHY_774")]
pub type DenaliPhy774 = crate::Reg<denali_phy_774::DenaliPhy774Spec>;
#[doc = ""]
pub mod denali_phy_774;
#[doc = "DENALI_PHY_775 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_775::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_775::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_775`]
module"]
#[doc(alias = "DENALI_PHY_775")]
pub type DenaliPhy775 = crate::Reg<denali_phy_775::DenaliPhy775Spec>;
#[doc = ""]
pub mod denali_phy_775;
#[doc = "DENALI_PHY_776 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_776::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_776::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_776`]
module"]
#[doc(alias = "DENALI_PHY_776")]
pub type DenaliPhy776 = crate::Reg<denali_phy_776::DenaliPhy776Spec>;
#[doc = ""]
pub mod denali_phy_776;
#[doc = "DENALI_PHY_777 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_777::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_777`]
module"]
#[doc(alias = "DENALI_PHY_777")]
pub type DenaliPhy777 = crate::Reg<denali_phy_777::DenaliPhy777Spec>;
#[doc = ""]
pub mod denali_phy_777;
#[doc = "DENALI_PHY_778 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_778::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_778`]
module"]
#[doc(alias = "DENALI_PHY_778")]
pub type DenaliPhy778 = crate::Reg<denali_phy_778::DenaliPhy778Spec>;
#[doc = ""]
pub mod denali_phy_778;
#[doc = "DENALI_PHY_779 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_779::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_779::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_779`]
module"]
#[doc(alias = "DENALI_PHY_779")]
pub type DenaliPhy779 = crate::Reg<denali_phy_779::DenaliPhy779Spec>;
#[doc = ""]
pub mod denali_phy_779;
#[doc = "DENALI_PHY_780 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_780::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_780::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_780`]
module"]
#[doc(alias = "DENALI_PHY_780")]
pub type DenaliPhy780 = crate::Reg<denali_phy_780::DenaliPhy780Spec>;
#[doc = ""]
pub mod denali_phy_780;
#[doc = "DENALI_PHY_781 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_781::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_781::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_781`]
module"]
#[doc(alias = "DENALI_PHY_781")]
pub type DenaliPhy781 = crate::Reg<denali_phy_781::DenaliPhy781Spec>;
#[doc = ""]
pub mod denali_phy_781;
#[doc = "DENALI_PHY_782 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_782::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_782::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_782`]
module"]
#[doc(alias = "DENALI_PHY_782")]
pub type DenaliPhy782 = crate::Reg<denali_phy_782::DenaliPhy782Spec>;
#[doc = ""]
pub mod denali_phy_782;
#[doc = "DENALI_PHY_783 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_783::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_783::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_783`]
module"]
#[doc(alias = "DENALI_PHY_783")]
pub type DenaliPhy783 = crate::Reg<denali_phy_783::DenaliPhy783Spec>;
#[doc = ""]
pub mod denali_phy_783;
#[doc = "DENALI_PHY_784 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_784::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_784::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_784`]
module"]
#[doc(alias = "DENALI_PHY_784")]
pub type DenaliPhy784 = crate::Reg<denali_phy_784::DenaliPhy784Spec>;
#[doc = ""]
pub mod denali_phy_784;
#[doc = "DENALI_PHY_785 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_785::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_785::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_785`]
module"]
#[doc(alias = "DENALI_PHY_785")]
pub type DenaliPhy785 = crate::Reg<denali_phy_785::DenaliPhy785Spec>;
#[doc = ""]
pub mod denali_phy_785;
#[doc = "DENALI_PHY_786 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_786::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_786::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_786`]
module"]
#[doc(alias = "DENALI_PHY_786")]
pub type DenaliPhy786 = crate::Reg<denali_phy_786::DenaliPhy786Spec>;
#[doc = ""]
pub mod denali_phy_786;
#[doc = "DENALI_PHY_787 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_787::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_787`]
module"]
#[doc(alias = "DENALI_PHY_787")]
pub type DenaliPhy787 = crate::Reg<denali_phy_787::DenaliPhy787Spec>;
#[doc = ""]
pub mod denali_phy_787;
#[doc = "DENALI_PHY_788 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_788::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_788`]
module"]
#[doc(alias = "DENALI_PHY_788")]
pub type DenaliPhy788 = crate::Reg<denali_phy_788::DenaliPhy788Spec>;
#[doc = ""]
pub mod denali_phy_788;
#[doc = "DENALI_PHY_789 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_789::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_789::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_789`]
module"]
#[doc(alias = "DENALI_PHY_789")]
pub type DenaliPhy789 = crate::Reg<denali_phy_789::DenaliPhy789Spec>;
#[doc = ""]
pub mod denali_phy_789;
#[doc = "DENALI_PHY_790 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_790::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_790::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_790`]
module"]
#[doc(alias = "DENALI_PHY_790")]
pub type DenaliPhy790 = crate::Reg<denali_phy_790::DenaliPhy790Spec>;
#[doc = ""]
pub mod denali_phy_790;
#[doc = "DENALI_PHY_791 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_791::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_791::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_791`]
module"]
#[doc(alias = "DENALI_PHY_791")]
pub type DenaliPhy791 = crate::Reg<denali_phy_791::DenaliPhy791Spec>;
#[doc = ""]
pub mod denali_phy_791;
#[doc = "DENALI_PHY_792 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_792::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_792::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_792`]
module"]
#[doc(alias = "DENALI_PHY_792")]
pub type DenaliPhy792 = crate::Reg<denali_phy_792::DenaliPhy792Spec>;
#[doc = ""]
pub mod denali_phy_792;
#[doc = "DENALI_PHY_793 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_793::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_793::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_793`]
module"]
#[doc(alias = "DENALI_PHY_793")]
pub type DenaliPhy793 = crate::Reg<denali_phy_793::DenaliPhy793Spec>;
#[doc = ""]
pub mod denali_phy_793;
#[doc = "DENALI_PHY_794 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_794::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_794::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_794`]
module"]
#[doc(alias = "DENALI_PHY_794")]
pub type DenaliPhy794 = crate::Reg<denali_phy_794::DenaliPhy794Spec>;
#[doc = ""]
pub mod denali_phy_794;
#[doc = "DENALI_PHY_795 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_795::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_795::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_795`]
module"]
#[doc(alias = "DENALI_PHY_795")]
pub type DenaliPhy795 = crate::Reg<denali_phy_795::DenaliPhy795Spec>;
#[doc = ""]
pub mod denali_phy_795;
#[doc = "DENALI_PHY_796 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_796::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_796::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_796`]
module"]
#[doc(alias = "DENALI_PHY_796")]
pub type DenaliPhy796 = crate::Reg<denali_phy_796::DenaliPhy796Spec>;
#[doc = ""]
pub mod denali_phy_796;
#[doc = "DENALI_PHY_797 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_797::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_797::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_797`]
module"]
#[doc(alias = "DENALI_PHY_797")]
pub type DenaliPhy797 = crate::Reg<denali_phy_797::DenaliPhy797Spec>;
#[doc = ""]
pub mod denali_phy_797;
#[doc = "DENALI_PHY_798 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_798::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_798::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_798`]
module"]
#[doc(alias = "DENALI_PHY_798")]
pub type DenaliPhy798 = crate::Reg<denali_phy_798::DenaliPhy798Spec>;
#[doc = ""]
pub mod denali_phy_798;
#[doc = "DENALI_PHY_799 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_799::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_799::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_799`]
module"]
#[doc(alias = "DENALI_PHY_799")]
pub type DenaliPhy799 = crate::Reg<denali_phy_799::DenaliPhy799Spec>;
#[doc = ""]
pub mod denali_phy_799;
#[doc = "DENALI_PHY_800 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_800::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_800::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_800`]
module"]
#[doc(alias = "DENALI_PHY_800")]
pub type DenaliPhy800 = crate::Reg<denali_phy_800::DenaliPhy800Spec>;
#[doc = ""]
pub mod denali_phy_800;
#[doc = "DENALI_PHY_801 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_801::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_801::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_801`]
module"]
#[doc(alias = "DENALI_PHY_801")]
pub type DenaliPhy801 = crate::Reg<denali_phy_801::DenaliPhy801Spec>;
#[doc = ""]
pub mod denali_phy_801;
#[doc = "DENALI_PHY_802 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_802::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_802::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_802`]
module"]
#[doc(alias = "DENALI_PHY_802")]
pub type DenaliPhy802 = crate::Reg<denali_phy_802::DenaliPhy802Spec>;
#[doc = ""]
pub mod denali_phy_802;
#[doc = "DENALI_PHY_803 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_803::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_803::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_803`]
module"]
#[doc(alias = "DENALI_PHY_803")]
pub type DenaliPhy803 = crate::Reg<denali_phy_803::DenaliPhy803Spec>;
#[doc = ""]
pub mod denali_phy_803;
#[doc = "DENALI_PHY_804 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_804::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_804::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_804`]
module"]
#[doc(alias = "DENALI_PHY_804")]
pub type DenaliPhy804 = crate::Reg<denali_phy_804::DenaliPhy804Spec>;
#[doc = ""]
pub mod denali_phy_804;
#[doc = "DENALI_PHY_805 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_805::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_805::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_805`]
module"]
#[doc(alias = "DENALI_PHY_805")]
pub type DenaliPhy805 = crate::Reg<denali_phy_805::DenaliPhy805Spec>;
#[doc = ""]
pub mod denali_phy_805;
#[doc = "DENALI_PHY_896 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_896::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_896::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_896`]
module"]
#[doc(alias = "DENALI_PHY_896")]
pub type DenaliPhy896 = crate::Reg<denali_phy_896::DenaliPhy896Spec>;
#[doc = ""]
pub mod denali_phy_896;
#[doc = "DENALI_PHY_897 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_897::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_897::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_897`]
module"]
#[doc(alias = "DENALI_PHY_897")]
pub type DenaliPhy897 = crate::Reg<denali_phy_897::DenaliPhy897Spec>;
#[doc = ""]
pub mod denali_phy_897;
#[doc = "DENALI_PHY_898 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_898::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_898::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_898`]
module"]
#[doc(alias = "DENALI_PHY_898")]
pub type DenaliPhy898 = crate::Reg<denali_phy_898::DenaliPhy898Spec>;
#[doc = ""]
pub mod denali_phy_898;
#[doc = "DENALI_PHY_899 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_899::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_899::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_899`]
module"]
#[doc(alias = "DENALI_PHY_899")]
pub type DenaliPhy899 = crate::Reg<denali_phy_899::DenaliPhy899Spec>;
#[doc = ""]
pub mod denali_phy_899;
#[doc = "DENALI_PHY_900 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_900::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_900::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_900`]
module"]
#[doc(alias = "DENALI_PHY_900")]
pub type DenaliPhy900 = crate::Reg<denali_phy_900::DenaliPhy900Spec>;
#[doc = ""]
pub mod denali_phy_900;
#[doc = "DENALI_PHY_901 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_901::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_901::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_901`]
module"]
#[doc(alias = "DENALI_PHY_901")]
pub type DenaliPhy901 = crate::Reg<denali_phy_901::DenaliPhy901Spec>;
#[doc = ""]
pub mod denali_phy_901;
#[doc = "DENALI_PHY_902 (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_902::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_902`]
module"]
#[doc(alias = "DENALI_PHY_902")]
pub type DenaliPhy902 = crate::Reg<denali_phy_902::DenaliPhy902Spec>;
#[doc = ""]
pub mod denali_phy_902;
#[doc = "DENALI_PHY_903 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_903::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_903`]
module"]
#[doc(alias = "DENALI_PHY_903")]
pub type DenaliPhy903 = crate::Reg<denali_phy_903::DenaliPhy903Spec>;
#[doc = ""]
pub mod denali_phy_903;
#[doc = "DENALI_PHY_904 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_904::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_904`]
module"]
#[doc(alias = "DENALI_PHY_904")]
pub type DenaliPhy904 = crate::Reg<denali_phy_904::DenaliPhy904Spec>;
#[doc = ""]
pub mod denali_phy_904;
#[doc = "DENALI_PHY_905 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_905::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_905::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_905`]
module"]
#[doc(alias = "DENALI_PHY_905")]
pub type DenaliPhy905 = crate::Reg<denali_phy_905::DenaliPhy905Spec>;
#[doc = ""]
pub mod denali_phy_905;
#[doc = "DENALI_PHY_906 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_906::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_906::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_906`]
module"]
#[doc(alias = "DENALI_PHY_906")]
pub type DenaliPhy906 = crate::Reg<denali_phy_906::DenaliPhy906Spec>;
#[doc = ""]
pub mod denali_phy_906;
#[doc = "DENALI_PHY_907 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_907::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_907::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_907`]
module"]
#[doc(alias = "DENALI_PHY_907")]
pub type DenaliPhy907 = crate::Reg<denali_phy_907::DenaliPhy907Spec>;
#[doc = ""]
pub mod denali_phy_907;
#[doc = "DENALI_PHY_908 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_908::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_908::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_908`]
module"]
#[doc(alias = "DENALI_PHY_908")]
pub type DenaliPhy908 = crate::Reg<denali_phy_908::DenaliPhy908Spec>;
#[doc = ""]
pub mod denali_phy_908;
#[doc = "DENALI_PHY_910 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_910::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_910::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_910`]
module"]
#[doc(alias = "DENALI_PHY_910")]
pub type DenaliPhy910 = crate::Reg<denali_phy_910::DenaliPhy910Spec>;
#[doc = ""]
pub mod denali_phy_910;
#[doc = "DENALI_PHY_911 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_911::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_911::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_911`]
module"]
#[doc(alias = "DENALI_PHY_911")]
pub type DenaliPhy911 = crate::Reg<denali_phy_911::DenaliPhy911Spec>;
#[doc = ""]
pub mod denali_phy_911;
#[doc = "DENALI_PHY_912 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_912::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_912::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_912`]
module"]
#[doc(alias = "DENALI_PHY_912")]
pub type DenaliPhy912 = crate::Reg<denali_phy_912::DenaliPhy912Spec>;
#[doc = ""]
pub mod denali_phy_912;
#[doc = "DENALI_PHY_913 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_913::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_913::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_913`]
module"]
#[doc(alias = "DENALI_PHY_913")]
pub type DenaliPhy913 = crate::Reg<denali_phy_913::DenaliPhy913Spec>;
#[doc = ""]
pub mod denali_phy_913;
#[doc = "DENALI_PHY_914 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_914::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_914::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_914`]
module"]
#[doc(alias = "DENALI_PHY_914")]
pub type DenaliPhy914 = crate::Reg<denali_phy_914::DenaliPhy914Spec>;
#[doc = ""]
pub mod denali_phy_914;
#[doc = "DENALI_PHY_915 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_915::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_915::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_915`]
module"]
#[doc(alias = "DENALI_PHY_915")]
pub type DenaliPhy915 = crate::Reg<denali_phy_915::DenaliPhy915Spec>;
#[doc = ""]
pub mod denali_phy_915;
#[doc = "DENALI_PHY_916 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_916::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_916::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_916`]
module"]
#[doc(alias = "DENALI_PHY_916")]
pub type DenaliPhy916 = crate::Reg<denali_phy_916::DenaliPhy916Spec>;
#[doc = ""]
pub mod denali_phy_916;
#[doc = "DENALI_PHY_917 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_917::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_917::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_917`]
module"]
#[doc(alias = "DENALI_PHY_917")]
pub type DenaliPhy917 = crate::Reg<denali_phy_917::DenaliPhy917Spec>;
#[doc = ""]
pub mod denali_phy_917;
#[doc = "DENALI_PHY_918 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_918::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_918::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_918`]
module"]
#[doc(alias = "DENALI_PHY_918")]
pub type DenaliPhy918 = crate::Reg<denali_phy_918::DenaliPhy918Spec>;
#[doc = ""]
pub mod denali_phy_918;
#[doc = "DENALI_PHY_919 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_919::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_919::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_919`]
module"]
#[doc(alias = "DENALI_PHY_919")]
pub type DenaliPhy919 = crate::Reg<denali_phy_919::DenaliPhy919Spec>;
#[doc = ""]
pub mod denali_phy_919;
#[doc = "DENALI_PHY_920 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_920::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_920::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_920`]
module"]
#[doc(alias = "DENALI_PHY_920")]
pub type DenaliPhy920 = crate::Reg<denali_phy_920::DenaliPhy920Spec>;
#[doc = ""]
pub mod denali_phy_920;
#[doc = "DENALI_PHY_921 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_921::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_921`]
module"]
#[doc(alias = "DENALI_PHY_921")]
pub type DenaliPhy921 = crate::Reg<denali_phy_921::DenaliPhy921Spec>;
#[doc = ""]
pub mod denali_phy_921;
#[doc = "DENALI_PHY_922 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_922::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_922::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_922`]
module"]
#[doc(alias = "DENALI_PHY_922")]
pub type DenaliPhy922 = crate::Reg<denali_phy_922::DenaliPhy922Spec>;
#[doc = ""]
pub mod denali_phy_922;
#[doc = "DENALI_PHY_923 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_923::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_923::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_923`]
module"]
#[doc(alias = "DENALI_PHY_923")]
pub type DenaliPhy923 = crate::Reg<denali_phy_923::DenaliPhy923Spec>;
#[doc = ""]
pub mod denali_phy_923;
#[doc = "DENALI_PHY_924 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_924::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_924::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_924`]
module"]
#[doc(alias = "DENALI_PHY_924")]
pub type DenaliPhy924 = crate::Reg<denali_phy_924::DenaliPhy924Spec>;
#[doc = ""]
pub mod denali_phy_924;
#[doc = "DENALI_PHY_925 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_925::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_925::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_925`]
module"]
#[doc(alias = "DENALI_PHY_925")]
pub type DenaliPhy925 = crate::Reg<denali_phy_925::DenaliPhy925Spec>;
#[doc = ""]
pub mod denali_phy_925;
#[doc = "DENALI_PHY_926 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_926::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_926::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_926`]
module"]
#[doc(alias = "DENALI_PHY_926")]
pub type DenaliPhy926 = crate::Reg<denali_phy_926::DenaliPhy926Spec>;
#[doc = ""]
pub mod denali_phy_926;
#[doc = "DENALI_PHY_927 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_927::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_927::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_927`]
module"]
#[doc(alias = "DENALI_PHY_927")]
pub type DenaliPhy927 = crate::Reg<denali_phy_927::DenaliPhy927Spec>;
#[doc = ""]
pub mod denali_phy_927;
#[doc = "DENALI_PHY_928 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_928::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_928::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_928`]
module"]
#[doc(alias = "DENALI_PHY_928")]
pub type DenaliPhy928 = crate::Reg<denali_phy_928::DenaliPhy928Spec>;
#[doc = ""]
pub mod denali_phy_928;
#[doc = "DENALI_PHY_929 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_929::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_929::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_929`]
module"]
#[doc(alias = "DENALI_PHY_929")]
pub type DenaliPhy929 = crate::Reg<denali_phy_929::DenaliPhy929Spec>;
#[doc = ""]
pub mod denali_phy_929;
#[doc = "DENALI_PHY_930 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_930::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_930::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_930`]
module"]
#[doc(alias = "DENALI_PHY_930")]
pub type DenaliPhy930 = crate::Reg<denali_phy_930::DenaliPhy930Spec>;
#[doc = ""]
pub mod denali_phy_930;
#[doc = "DENALI_PHY_931 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_931::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_931::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_931`]
module"]
#[doc(alias = "DENALI_PHY_931")]
pub type DenaliPhy931 = crate::Reg<denali_phy_931::DenaliPhy931Spec>;
#[doc = ""]
pub mod denali_phy_931;
#[doc = "DENALI_PHY_932 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_932::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_932::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_932`]
module"]
#[doc(alias = "DENALI_PHY_932")]
pub type DenaliPhy932 = crate::Reg<denali_phy_932::DenaliPhy932Spec>;
#[doc = ""]
pub mod denali_phy_932;
#[doc = "DENALI_PHY_933 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_933::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_933::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_933`]
module"]
#[doc(alias = "DENALI_PHY_933")]
pub type DenaliPhy933 = crate::Reg<denali_phy_933::DenaliPhy933Spec>;
#[doc = ""]
pub mod denali_phy_933;
#[doc = "DENALI_PHY_934 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_934::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_934::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_934`]
module"]
#[doc(alias = "DENALI_PHY_934")]
pub type DenaliPhy934 = crate::Reg<denali_phy_934::DenaliPhy934Spec>;
#[doc = ""]
pub mod denali_phy_934;
#[doc = "DENALI_PHY_935 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_935::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_935::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_935`]
module"]
#[doc(alias = "DENALI_PHY_935")]
pub type DenaliPhy935 = crate::Reg<denali_phy_935::DenaliPhy935Spec>;
#[doc = ""]
pub mod denali_phy_935;
#[doc = "DENALI_PHY_936 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_936::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_936::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_936`]
module"]
#[doc(alias = "DENALI_PHY_936")]
pub type DenaliPhy936 = crate::Reg<denali_phy_936::DenaliPhy936Spec>;
#[doc = ""]
pub mod denali_phy_936;
#[doc = "DENALI_PHY_937 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_937::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_937::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_937`]
module"]
#[doc(alias = "DENALI_PHY_937")]
pub type DenaliPhy937 = crate::Reg<denali_phy_937::DenaliPhy937Spec>;
#[doc = ""]
pub mod denali_phy_937;
#[doc = "DENALI_PHY_938 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_938::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_938::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_938`]
module"]
#[doc(alias = "DENALI_PHY_938")]
pub type DenaliPhy938 = crate::Reg<denali_phy_938::DenaliPhy938Spec>;
#[doc = ""]
pub mod denali_phy_938;
#[doc = "DENALI_PHY_939 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_939::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_939::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_939`]
module"]
#[doc(alias = "DENALI_PHY_939")]
pub type DenaliPhy939 = crate::Reg<denali_phy_939::DenaliPhy939Spec>;
#[doc = ""]
pub mod denali_phy_939;
#[doc = "DENALI_PHY_940 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_940::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_940::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_940`]
module"]
#[doc(alias = "DENALI_PHY_940")]
pub type DenaliPhy940 = crate::Reg<denali_phy_940::DenaliPhy940Spec>;
#[doc = ""]
pub mod denali_phy_940;
#[doc = "DENALI_PHY_941 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_941::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_941::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_941`]
module"]
#[doc(alias = "DENALI_PHY_941")]
pub type DenaliPhy941 = crate::Reg<denali_phy_941::DenaliPhy941Spec>;
#[doc = ""]
pub mod denali_phy_941;
#[doc = "DENALI_PHY_942 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_942::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_942::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_942`]
module"]
#[doc(alias = "DENALI_PHY_942")]
pub type DenaliPhy942 = crate::Reg<denali_phy_942::DenaliPhy942Spec>;
#[doc = ""]
pub mod denali_phy_942;
#[doc = "DENALI_PHY_943 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_943::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_943::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_943`]
module"]
#[doc(alias = "DENALI_PHY_943")]
pub type DenaliPhy943 = crate::Reg<denali_phy_943::DenaliPhy943Spec>;
#[doc = ""]
pub mod denali_phy_943;
#[doc = "DENALI_PHY_944 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_944::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_944::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_944`]
module"]
#[doc(alias = "DENALI_PHY_944")]
pub type DenaliPhy944 = crate::Reg<denali_phy_944::DenaliPhy944Spec>;
#[doc = ""]
pub mod denali_phy_944;
#[doc = "DENALI_PHY_945 (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_945::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_945`]
module"]
#[doc(alias = "DENALI_PHY_945")]
pub type DenaliPhy945 = crate::Reg<denali_phy_945::DenaliPhy945Spec>;
#[doc = ""]
pub mod denali_phy_945;
#[doc = "DENALI_PHY_946 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_946::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_946::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_946`]
module"]
#[doc(alias = "DENALI_PHY_946")]
pub type DenaliPhy946 = crate::Reg<denali_phy_946::DenaliPhy946Spec>;
#[doc = ""]
pub mod denali_phy_946;
#[doc = "DENALI_PHY_947 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_947::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_947::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_947`]
module"]
#[doc(alias = "DENALI_PHY_947")]
pub type DenaliPhy947 = crate::Reg<denali_phy_947::DenaliPhy947Spec>;
#[doc = ""]
pub mod denali_phy_947;
#[doc = "DENALI_PHY_948 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_948::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_948`]
module"]
#[doc(alias = "DENALI_PHY_948")]
pub type DenaliPhy948 = crate::Reg<denali_phy_948::DenaliPhy948Spec>;
#[doc = ""]
pub mod denali_phy_948;
#[doc = "DENALI_PHY_949 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_949::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_949`]
module"]
#[doc(alias = "DENALI_PHY_949")]
pub type DenaliPhy949 = crate::Reg<denali_phy_949::DenaliPhy949Spec>;
#[doc = ""]
pub mod denali_phy_949;
#[doc = "DENALI_PHY_950 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_950::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_950::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_950`]
module"]
#[doc(alias = "DENALI_PHY_950")]
pub type DenaliPhy950 = crate::Reg<denali_phy_950::DenaliPhy950Spec>;
#[doc = ""]
pub mod denali_phy_950;
#[doc = "DENALI_PHY_951 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_951::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_951::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_951`]
module"]
#[doc(alias = "DENALI_PHY_951")]
pub type DenaliPhy951 = crate::Reg<denali_phy_951::DenaliPhy951Spec>;
#[doc = ""]
pub mod denali_phy_951;
#[doc = "DENALI_PHY_952 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_952::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_952`]
module"]
#[doc(alias = "DENALI_PHY_952")]
pub type DenaliPhy952 = crate::Reg<denali_phy_952::DenaliPhy952Spec>;
#[doc = ""]
pub mod denali_phy_952;
#[doc = "DENALI_PHY_953 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_953::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_953::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_953`]
module"]
#[doc(alias = "DENALI_PHY_953")]
pub type DenaliPhy953 = crate::Reg<denali_phy_953::DenaliPhy953Spec>;
#[doc = ""]
pub mod denali_phy_953;
#[doc = "DENALI_PHY_954 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_954::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_954::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_954`]
module"]
#[doc(alias = "DENALI_PHY_954")]
pub type DenaliPhy954 = crate::Reg<denali_phy_954::DenaliPhy954Spec>;
#[doc = ""]
pub mod denali_phy_954;
#[doc = "DENALI_PHY_955 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_955::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_955::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_955`]
module"]
#[doc(alias = "DENALI_PHY_955")]
pub type DenaliPhy955 = crate::Reg<denali_phy_955::DenaliPhy955Spec>;
#[doc = ""]
pub mod denali_phy_955;
#[doc = "DENALI_PHY_956 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_956::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_956::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_956`]
module"]
#[doc(alias = "DENALI_PHY_956")]
pub type DenaliPhy956 = crate::Reg<denali_phy_956::DenaliPhy956Spec>;
#[doc = ""]
pub mod denali_phy_956;
#[doc = "DENALI_PHY_957 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_957::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_957::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_957`]
module"]
#[doc(alias = "DENALI_PHY_957")]
pub type DenaliPhy957 = crate::Reg<denali_phy_957::DenaliPhy957Spec>;
#[doc = ""]
pub mod denali_phy_957;
#[doc = "DENALI_PHY_958 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_958::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_958`]
module"]
#[doc(alias = "DENALI_PHY_958")]
pub type DenaliPhy958 = crate::Reg<denali_phy_958::DenaliPhy958Spec>;
#[doc = ""]
pub mod denali_phy_958;
#[doc = "DDR_PI_REG_0 (rw) register accessor: DDR PHY Independent Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_0`]
module"]
#[doc(alias = "DDR_PI_REG_0")]
pub type DdrPiReg0 = crate::Reg<ddr_pi_reg_0::DdrPiReg0Spec>;
#[doc = "DDR PHY Independent Register 0"]
pub mod ddr_pi_reg_0;
#[doc = "DDR_PI_REG_1 (rw) register accessor: DDR PHY Independent Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_1`]
module"]
#[doc(alias = "DDR_PI_REG_1")]
pub type DdrPiReg1 = crate::Reg<ddr_pi_reg_1::DdrPiReg1Spec>;
#[doc = "DDR PHY Independent Register 1"]
pub mod ddr_pi_reg_1;
#[doc = "DDR_PI_REG_2 (rw) register accessor: DDR PHY Independent Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_2`]
module"]
#[doc(alias = "DDR_PI_REG_2")]
pub type DdrPiReg2 = crate::Reg<ddr_pi_reg_2::DdrPiReg2Spec>;
#[doc = "DDR PHY Independent Register 2"]
pub mod ddr_pi_reg_2;
#[doc = "DDR_PI_REG_3 (rw) register accessor: DDR PHY Independent Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_3`]
module"]
#[doc(alias = "DDR_PI_REG_3")]
pub type DdrPiReg3 = crate::Reg<ddr_pi_reg_3::DdrPiReg3Spec>;
#[doc = "DDR PHY Independent Register 3"]
pub mod ddr_pi_reg_3;
#[doc = "DDR_PI_REG_4 (rw) register accessor: DDR PHY Independent Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_4`]
module"]
#[doc(alias = "DDR_PI_REG_4")]
pub type DdrPiReg4 = crate::Reg<ddr_pi_reg_4::DdrPiReg4Spec>;
#[doc = "DDR PHY Independent Register 4"]
pub mod ddr_pi_reg_4;
#[doc = "DDR_PI_REG_5 (rw) register accessor: DDR PHY Independent Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_5`]
module"]
#[doc(alias = "DDR_PI_REG_5")]
pub type DdrPiReg5 = crate::Reg<ddr_pi_reg_5::DdrPiReg5Spec>;
#[doc = "DDR PHY Independent Register 5"]
pub mod ddr_pi_reg_5;
#[doc = "DDR_PI_REG_6 (rw) register accessor: DDR PHY Independent Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_6`]
module"]
#[doc(alias = "DDR_PI_REG_6")]
pub type DdrPiReg6 = crate::Reg<ddr_pi_reg_6::DdrPiReg6Spec>;
#[doc = "DDR PHY Independent Register 6"]
pub mod ddr_pi_reg_6;
#[doc = "DDR_PI_REG_7 (rw) register accessor: DDR PHY Independent Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_7`]
module"]
#[doc(alias = "DDR_PI_REG_7")]
pub type DdrPiReg7 = crate::Reg<ddr_pi_reg_7::DdrPiReg7Spec>;
#[doc = "DDR PHY Independent Register 7"]
pub mod ddr_pi_reg_7;
#[doc = "DDR_PI_REG_8 (rw) register accessor: DDR PHY Independent Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_8`]
module"]
#[doc(alias = "DDR_PI_REG_8")]
pub type DdrPiReg8 = crate::Reg<ddr_pi_reg_8::DdrPiReg8Spec>;
#[doc = "DDR PHY Independent Register 8"]
pub mod ddr_pi_reg_8;
#[doc = "DDR_PI_REG_9 (rw) register accessor: DDR PHY Independent Register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_9`]
module"]
#[doc(alias = "DDR_PI_REG_9")]
pub type DdrPiReg9 = crate::Reg<ddr_pi_reg_9::DdrPiReg9Spec>;
#[doc = "DDR PHY Independent Register 9"]
pub mod ddr_pi_reg_9;
#[doc = "DDR_PI_REG_10 (rw) register accessor: DDR PHY Independent Register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_10`]
module"]
#[doc(alias = "DDR_PI_REG_10")]
pub type DdrPiReg10 = crate::Reg<ddr_pi_reg_10::DdrPiReg10Spec>;
#[doc = "DDR PHY Independent Register 10"]
pub mod ddr_pi_reg_10;
#[doc = "DDR_PI_REG_11 (rw) register accessor: DDR PHY Independent Register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_11`]
module"]
#[doc(alias = "DDR_PI_REG_11")]
pub type DdrPiReg11 = crate::Reg<ddr_pi_reg_11::DdrPiReg11Spec>;
#[doc = "DDR PHY Independent Register 11"]
pub mod ddr_pi_reg_11;
#[doc = "DDR_PI_REG_12 (rw) register accessor: DDR PHY Independent Register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_12`]
module"]
#[doc(alias = "DDR_PI_REG_12")]
pub type DdrPiReg12 = crate::Reg<ddr_pi_reg_12::DdrPiReg12Spec>;
#[doc = "DDR PHY Independent Register 12"]
pub mod ddr_pi_reg_12;
#[doc = "DDR_PI_REG_13 (rw) register accessor: DDR PHY Independent Register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_13`]
module"]
#[doc(alias = "DDR_PI_REG_13")]
pub type DdrPiReg13 = crate::Reg<ddr_pi_reg_13::DdrPiReg13Spec>;
#[doc = "DDR PHY Independent Register 13"]
pub mod ddr_pi_reg_13;
#[doc = "DDR_PI_REG_14 (rw) register accessor: DDR PHY Independent Register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_14`]
module"]
#[doc(alias = "DDR_PI_REG_14")]
pub type DdrPiReg14 = crate::Reg<ddr_pi_reg_14::DdrPiReg14Spec>;
#[doc = "DDR PHY Independent Register 14"]
pub mod ddr_pi_reg_14;
#[doc = "DDR_PI_REG_15 (rw) register accessor: DDR PHY Independent Register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_15`]
module"]
#[doc(alias = "DDR_PI_REG_15")]
pub type DdrPiReg15 = crate::Reg<ddr_pi_reg_15::DdrPiReg15Spec>;
#[doc = "DDR PHY Independent Register 15"]
pub mod ddr_pi_reg_15;
#[doc = "DDR_PI_REG_16 (rw) register accessor: DDR PHY Independent Register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_16`]
module"]
#[doc(alias = "DDR_PI_REG_16")]
pub type DdrPiReg16 = crate::Reg<ddr_pi_reg_16::DdrPiReg16Spec>;
#[doc = "DDR PHY Independent Register 16"]
pub mod ddr_pi_reg_16;
#[doc = "DDR_PI_REG_17 (rw) register accessor: DDR PHY Independent Register 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_17`]
module"]
#[doc(alias = "DDR_PI_REG_17")]
pub type DdrPiReg17 = crate::Reg<ddr_pi_reg_17::DdrPiReg17Spec>;
#[doc = "DDR PHY Independent Register 17"]
pub mod ddr_pi_reg_17;
#[doc = "DDR_PI_REG_18 (rw) register accessor: DDR PHY Independent Register 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_18`]
module"]
#[doc(alias = "DDR_PI_REG_18")]
pub type DdrPiReg18 = crate::Reg<ddr_pi_reg_18::DdrPiReg18Spec>;
#[doc = "DDR PHY Independent Register 18"]
pub mod ddr_pi_reg_18;
#[doc = "DDR_PI_REG_19 (rw) register accessor: DDR PHY Independent Register 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_19`]
module"]
#[doc(alias = "DDR_PI_REG_19")]
pub type DdrPiReg19 = crate::Reg<ddr_pi_reg_19::DdrPiReg19Spec>;
#[doc = "DDR PHY Independent Register 19"]
pub mod ddr_pi_reg_19;
#[doc = "DDR_PI_REG_20 (rw) register accessor: DDR PHY Independent Register 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_20`]
module"]
#[doc(alias = "DDR_PI_REG_20")]
pub type DdrPiReg20 = crate::Reg<ddr_pi_reg_20::DdrPiReg20Spec>;
#[doc = "DDR PHY Independent Register 20"]
pub mod ddr_pi_reg_20;
#[doc = "DDR_PI_REG_21 (rw) register accessor: DDR PHY Independent Register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_21`]
module"]
#[doc(alias = "DDR_PI_REG_21")]
pub type DdrPiReg21 = crate::Reg<ddr_pi_reg_21::DdrPiReg21Spec>;
#[doc = "DDR PHY Independent Register 21"]
pub mod ddr_pi_reg_21;
#[doc = "DDR_PI_REG_22 (rw) register accessor: DDR PHY Independent Register 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_22`]
module"]
#[doc(alias = "DDR_PI_REG_22")]
pub type DdrPiReg22 = crate::Reg<ddr_pi_reg_22::DdrPiReg22Spec>;
#[doc = "DDR PHY Independent Register 22"]
pub mod ddr_pi_reg_22;
#[doc = "DDR_PI_REG_23 (rw) register accessor: DDR PHY Independent Register 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_23`]
module"]
#[doc(alias = "DDR_PI_REG_23")]
pub type DdrPiReg23 = crate::Reg<ddr_pi_reg_23::DdrPiReg23Spec>;
#[doc = "DDR PHY Independent Register 23"]
pub mod ddr_pi_reg_23;
#[doc = "DDR_PI_REG_24 (rw) register accessor: DDR PHY Independent Register 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_24`]
module"]
#[doc(alias = "DDR_PI_REG_24")]
pub type DdrPiReg24 = crate::Reg<ddr_pi_reg_24::DdrPiReg24Spec>;
#[doc = "DDR PHY Independent Register 24"]
pub mod ddr_pi_reg_24;
#[doc = "DDR_PI_REG_25 (rw) register accessor: DDR PHY Independent Register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_25`]
module"]
#[doc(alias = "DDR_PI_REG_25")]
pub type DdrPiReg25 = crate::Reg<ddr_pi_reg_25::DdrPiReg25Spec>;
#[doc = "DDR PHY Independent Register 25"]
pub mod ddr_pi_reg_25;
#[doc = "DDR_PI_REG_26 (rw) register accessor: DDR PHY Independent Register 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_26`]
module"]
#[doc(alias = "DDR_PI_REG_26")]
pub type DdrPiReg26 = crate::Reg<ddr_pi_reg_26::DdrPiReg26Spec>;
#[doc = "DDR PHY Independent Register 26"]
pub mod ddr_pi_reg_26;
#[doc = "DDR_PI_REG_27 (rw) register accessor: DDR PHY Independent Register 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_27`]
module"]
#[doc(alias = "DDR_PI_REG_27")]
pub type DdrPiReg27 = crate::Reg<ddr_pi_reg_27::DdrPiReg27Spec>;
#[doc = "DDR PHY Independent Register 27"]
pub mod ddr_pi_reg_27;
#[doc = "DDR_PI_REG_28 (rw) register accessor: DDR PHY Independent Register 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_28`]
module"]
#[doc(alias = "DDR_PI_REG_28")]
pub type DdrPiReg28 = crate::Reg<ddr_pi_reg_28::DdrPiReg28Spec>;
#[doc = "DDR PHY Independent Register 28"]
pub mod ddr_pi_reg_28;
#[doc = "DDR_PI_REG_29 (rw) register accessor: DDR PHY Independent Register 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_29`]
module"]
#[doc(alias = "DDR_PI_REG_29")]
pub type DdrPiReg29 = crate::Reg<ddr_pi_reg_29::DdrPiReg29Spec>;
#[doc = "DDR PHY Independent Register 29"]
pub mod ddr_pi_reg_29;
#[doc = "DDR_PI_REG_30 (rw) register accessor: DDR PHY Independent Register 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_30`]
module"]
#[doc(alias = "DDR_PI_REG_30")]
pub type DdrPiReg30 = crate::Reg<ddr_pi_reg_30::DdrPiReg30Spec>;
#[doc = "DDR PHY Independent Register 30"]
pub mod ddr_pi_reg_30;
#[doc = "DDR_PI_REG_31 (rw) register accessor: DDR PHY Independent Register 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_31`]
module"]
#[doc(alias = "DDR_PI_REG_31")]
pub type DdrPiReg31 = crate::Reg<ddr_pi_reg_31::DdrPiReg31Spec>;
#[doc = "DDR PHY Independent Register 31"]
pub mod ddr_pi_reg_31;
#[doc = "DDR_PI_REG_32 (rw) register accessor: DDR PHY Independent Register 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_32`]
module"]
#[doc(alias = "DDR_PI_REG_32")]
pub type DdrPiReg32 = crate::Reg<ddr_pi_reg_32::DdrPiReg32Spec>;
#[doc = "DDR PHY Independent Register 32"]
pub mod ddr_pi_reg_32;
#[doc = "DDR_PI_REG_33 (rw) register accessor: DDR PHY Independent Register 33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_33`]
module"]
#[doc(alias = "DDR_PI_REG_33")]
pub type DdrPiReg33 = crate::Reg<ddr_pi_reg_33::DdrPiReg33Spec>;
#[doc = "DDR PHY Independent Register 33"]
pub mod ddr_pi_reg_33;
#[doc = "DDR_PI_REG_34 (rw) register accessor: DDR PHY Independent Register 34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_34`]
module"]
#[doc(alias = "DDR_PI_REG_34")]
pub type DdrPiReg34 = crate::Reg<ddr_pi_reg_34::DdrPiReg34Spec>;
#[doc = "DDR PHY Independent Register 34"]
pub mod ddr_pi_reg_34;
#[doc = "DDR_PI_REG_35 (rw) register accessor: DDR PHY Independent Register 35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_35`]
module"]
#[doc(alias = "DDR_PI_REG_35")]
pub type DdrPiReg35 = crate::Reg<ddr_pi_reg_35::DdrPiReg35Spec>;
#[doc = "DDR PHY Independent Register 35"]
pub mod ddr_pi_reg_35;
#[doc = "DDR_PI_REG_36 (rw) register accessor: DDR PHY Independent Register 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_36`]
module"]
#[doc(alias = "DDR_PI_REG_36")]
pub type DdrPiReg36 = crate::Reg<ddr_pi_reg_36::DdrPiReg36Spec>;
#[doc = "DDR PHY Independent Register 36"]
pub mod ddr_pi_reg_36;
#[doc = "DDR_PI_REG_37 (rw) register accessor: DDR PHY Independent Register 37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_37`]
module"]
#[doc(alias = "DDR_PI_REG_37")]
pub type DdrPiReg37 = crate::Reg<ddr_pi_reg_37::DdrPiReg37Spec>;
#[doc = "DDR PHY Independent Register 37"]
pub mod ddr_pi_reg_37;
#[doc = "DDR_PI_REG_38 (rw) register accessor: DDR PHY Independent Register 38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_38`]
module"]
#[doc(alias = "DDR_PI_REG_38")]
pub type DdrPiReg38 = crate::Reg<ddr_pi_reg_38::DdrPiReg38Spec>;
#[doc = "DDR PHY Independent Register 38"]
pub mod ddr_pi_reg_38;
#[doc = "DDR_PI_REG_39 (rw) register accessor: DDR PHY Independent Register 39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_39`]
module"]
#[doc(alias = "DDR_PI_REG_39")]
pub type DdrPiReg39 = crate::Reg<ddr_pi_reg_39::DdrPiReg39Spec>;
#[doc = "DDR PHY Independent Register 39"]
pub mod ddr_pi_reg_39;
#[doc = "DDR_PI_REG_40 (rw) register accessor: DDR PHY Independent Register 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_40`]
module"]
#[doc(alias = "DDR_PI_REG_40")]
pub type DdrPiReg40 = crate::Reg<ddr_pi_reg_40::DdrPiReg40Spec>;
#[doc = "DDR PHY Independent Register 40"]
pub mod ddr_pi_reg_40;
#[doc = "DDR_PI_REG_41 (rw) register accessor: DDR PHY Independent Register 41\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_41`]
module"]
#[doc(alias = "DDR_PI_REG_41")]
pub type DdrPiReg41 = crate::Reg<ddr_pi_reg_41::DdrPiReg41Spec>;
#[doc = "DDR PHY Independent Register 41"]
pub mod ddr_pi_reg_41;
#[doc = "DDR_PI_REG_42 (rw) register accessor: DDR PHY Independent Register 42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_42`]
module"]
#[doc(alias = "DDR_PI_REG_42")]
pub type DdrPiReg42 = crate::Reg<ddr_pi_reg_42::DdrPiReg42Spec>;
#[doc = "DDR PHY Independent Register 42"]
pub mod ddr_pi_reg_42;
#[doc = "DDR_PI_REG_43 (rw) register accessor: DDR PHY Independent Register 43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_43`]
module"]
#[doc(alias = "DDR_PI_REG_43")]
pub type DdrPiReg43 = crate::Reg<ddr_pi_reg_43::DdrPiReg43Spec>;
#[doc = "DDR PHY Independent Register 43"]
pub mod ddr_pi_reg_43;
#[doc = "DDR_PI_REG_44 (rw) register accessor: DDR PHY Independent Register 44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_44`]
module"]
#[doc(alias = "DDR_PI_REG_44")]
pub type DdrPiReg44 = crate::Reg<ddr_pi_reg_44::DdrPiReg44Spec>;
#[doc = "DDR PHY Independent Register 44"]
pub mod ddr_pi_reg_44;
#[doc = "DDR_PI_REG_45 (rw) register accessor: DDR PHY Independent Register 45\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_45`]
module"]
#[doc(alias = "DDR_PI_REG_45")]
pub type DdrPiReg45 = crate::Reg<ddr_pi_reg_45::DdrPiReg45Spec>;
#[doc = "DDR PHY Independent Register 45"]
pub mod ddr_pi_reg_45;
#[doc = "DDR_PI_REG_46 (rw) register accessor: DDR PHY Independent Register 46\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_46`]
module"]
#[doc(alias = "DDR_PI_REG_46")]
pub type DdrPiReg46 = crate::Reg<ddr_pi_reg_46::DdrPiReg46Spec>;
#[doc = "DDR PHY Independent Register 46"]
pub mod ddr_pi_reg_46;
#[doc = "DDR_PI_REG_47 (rw) register accessor: DDR PHY Independent Register 47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_47`]
module"]
#[doc(alias = "DDR_PI_REG_47")]
pub type DdrPiReg47 = crate::Reg<ddr_pi_reg_47::DdrPiReg47Spec>;
#[doc = "DDR PHY Independent Register 47"]
pub mod ddr_pi_reg_47;
#[doc = "DDR_PI_REG_48 (rw) register accessor: DDR PHY Independent Register 48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_48`]
module"]
#[doc(alias = "DDR_PI_REG_48")]
pub type DdrPiReg48 = crate::Reg<ddr_pi_reg_48::DdrPiReg48Spec>;
#[doc = "DDR PHY Independent Register 48"]
pub mod ddr_pi_reg_48;
#[doc = "DDR_PI_REG_49 (rw) register accessor: DDR PHY Independent Register 49\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_49`]
module"]
#[doc(alias = "DDR_PI_REG_49")]
pub type DdrPiReg49 = crate::Reg<ddr_pi_reg_49::DdrPiReg49Spec>;
#[doc = "DDR PHY Independent Register 49"]
pub mod ddr_pi_reg_49;
#[doc = "DDR_PI_REG_50 (r) register accessor: DDR PHY Independent Register 50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_50::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_50`]
module"]
#[doc(alias = "DDR_PI_REG_50")]
pub type DdrPiReg50 = crate::Reg<ddr_pi_reg_50::DdrPiReg50Spec>;
#[doc = "DDR PHY Independent Register 50"]
pub mod ddr_pi_reg_50;
#[doc = "DDR_PI_REG_51 (r) register accessor: DDR PHY Independent Register 51\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_51::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_51`]
module"]
#[doc(alias = "DDR_PI_REG_51")]
pub type DdrPiReg51 = crate::Reg<ddr_pi_reg_51::DdrPiReg51Spec>;
#[doc = "DDR PHY Independent Register 51"]
pub mod ddr_pi_reg_51;
#[doc = "DDR_PI_REG_52 (rw) register accessor: DDR PHY Independent Register 52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_52`]
module"]
#[doc(alias = "DDR_PI_REG_52")]
pub type DdrPiReg52 = crate::Reg<ddr_pi_reg_52::DdrPiReg52Spec>;
#[doc = "DDR PHY Independent Register 52"]
pub mod ddr_pi_reg_52;
#[doc = "DDR_PI_REG_53 (rw) register accessor: DDR PHY Independent Register 53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_53`]
module"]
#[doc(alias = "DDR_PI_REG_53")]
pub type DdrPiReg53 = crate::Reg<ddr_pi_reg_53::DdrPiReg53Spec>;
#[doc = "DDR PHY Independent Register 53"]
pub mod ddr_pi_reg_53;
#[doc = "DDR_PI_REG_54 (rw) register accessor: DDR PHY Independent Register 54\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_54::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_54`]
module"]
#[doc(alias = "DDR_PI_REG_54")]
pub type DdrPiReg54 = crate::Reg<ddr_pi_reg_54::DdrPiReg54Spec>;
#[doc = "DDR PHY Independent Register 54"]
pub mod ddr_pi_reg_54;
#[doc = "DDR_PI_REG_55 (rw) register accessor: DDR PHY Independent Register 55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_55`]
module"]
#[doc(alias = "DDR_PI_REG_55")]
pub type DdrPiReg55 = crate::Reg<ddr_pi_reg_55::DdrPiReg55Spec>;
#[doc = "DDR PHY Independent Register 55"]
pub mod ddr_pi_reg_55;
#[doc = "DDR_PI_REG_56 (rw) register accessor: DDR PHY Independent Register 56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_56::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_56`]
module"]
#[doc(alias = "DDR_PI_REG_56")]
pub type DdrPiReg56 = crate::Reg<ddr_pi_reg_56::DdrPiReg56Spec>;
#[doc = "DDR PHY Independent Register 56"]
pub mod ddr_pi_reg_56;
#[doc = "DDR_PI_REG_57 (rw) register accessor: DDR PHY Independent Register 57\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_57::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_57::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_57`]
module"]
#[doc(alias = "DDR_PI_REG_57")]
pub type DdrPiReg57 = crate::Reg<ddr_pi_reg_57::DdrPiReg57Spec>;
#[doc = "DDR PHY Independent Register 57"]
pub mod ddr_pi_reg_57;
#[doc = "DDR_PI_REG_58 (rw) register accessor: DDR PHY Independent Register 58\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_58::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_58::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_58`]
module"]
#[doc(alias = "DDR_PI_REG_58")]
pub type DdrPiReg58 = crate::Reg<ddr_pi_reg_58::DdrPiReg58Spec>;
#[doc = "DDR PHY Independent Register 58"]
pub mod ddr_pi_reg_58;
#[doc = "DDR_PI_REG_59 (rw) register accessor: DDR PHY Independent Register 59\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_59::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_59`]
module"]
#[doc(alias = "DDR_PI_REG_59")]
pub type DdrPiReg59 = crate::Reg<ddr_pi_reg_59::DdrPiReg59Spec>;
#[doc = "DDR PHY Independent Register 59"]
pub mod ddr_pi_reg_59;
#[doc = "DDR_PI_REG_60 (rw) register accessor: DDR PHY Independent Register 60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_60::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_60`]
module"]
#[doc(alias = "DDR_PI_REG_60")]
pub type DdrPiReg60 = crate::Reg<ddr_pi_reg_60::DdrPiReg60Spec>;
#[doc = "DDR PHY Independent Register 60"]
pub mod ddr_pi_reg_60;
#[doc = "DDR_PI_REG_61 (rw) register accessor: DDR PHY Independent Register 61\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_61::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_61::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_61`]
module"]
#[doc(alias = "DDR_PI_REG_61")]
pub type DdrPiReg61 = crate::Reg<ddr_pi_reg_61::DdrPiReg61Spec>;
#[doc = "DDR PHY Independent Register 61"]
pub mod ddr_pi_reg_61;
#[doc = "DDR_PI_REG_62 (rw) register accessor: DDR PHY Independent Register 62\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_62::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_62::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_62`]
module"]
#[doc(alias = "DDR_PI_REG_62")]
pub type DdrPiReg62 = crate::Reg<ddr_pi_reg_62::DdrPiReg62Spec>;
#[doc = "DDR PHY Independent Register 62"]
pub mod ddr_pi_reg_62;
#[doc = "DDR_PI_REG_63 (rw) register accessor: DDR PHY Independent Register 63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_63::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_63`]
module"]
#[doc(alias = "DDR_PI_REG_63")]
pub type DdrPiReg63 = crate::Reg<ddr_pi_reg_63::DdrPiReg63Spec>;
#[doc = "DDR PHY Independent Register 63"]
pub mod ddr_pi_reg_63;
#[doc = "DDR_PI_REG_64 (rw) register accessor: DDR PHY Independent Register 64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_64`]
module"]
#[doc(alias = "DDR_PI_REG_64")]
pub type DdrPiReg64 = crate::Reg<ddr_pi_reg_64::DdrPiReg64Spec>;
#[doc = "DDR PHY Independent Register 64"]
pub mod ddr_pi_reg_64;
#[doc = "DDR_PI_REG_65 (rw) register accessor: DDR PHY Independent Register 65\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_65::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_65::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_65`]
module"]
#[doc(alias = "DDR_PI_REG_65")]
pub type DdrPiReg65 = crate::Reg<ddr_pi_reg_65::DdrPiReg65Spec>;
#[doc = "DDR PHY Independent Register 65"]
pub mod ddr_pi_reg_65;
#[doc = "DDR_PI_REG_66 (rw) register accessor: DDR PHY Independent Register 66\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_66::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_66::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_66`]
module"]
#[doc(alias = "DDR_PI_REG_66")]
pub type DdrPiReg66 = crate::Reg<ddr_pi_reg_66::DdrPiReg66Spec>;
#[doc = "DDR PHY Independent Register 66"]
pub mod ddr_pi_reg_66;
#[doc = "DDR_PI_REG_67 (rw) register accessor: DDR PHY Independent Register 67\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_67::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_67::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_67`]
module"]
#[doc(alias = "DDR_PI_REG_67")]
pub type DdrPiReg67 = crate::Reg<ddr_pi_reg_67::DdrPiReg67Spec>;
#[doc = "DDR PHY Independent Register 67"]
pub mod ddr_pi_reg_67;
#[doc = "DDR_PI_REG_68 (rw) register accessor: DDR PHY Independent Register 68\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_68::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_68::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_68`]
module"]
#[doc(alias = "DDR_PI_REG_68")]
pub type DdrPiReg68 = crate::Reg<ddr_pi_reg_68::DdrPiReg68Spec>;
#[doc = "DDR PHY Independent Register 68"]
pub mod ddr_pi_reg_68;
#[doc = "DDR_PI_REG_69 (rw) register accessor: DDR PHY Independent Register 69\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_69::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_69::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_69`]
module"]
#[doc(alias = "DDR_PI_REG_69")]
pub type DdrPiReg69 = crate::Reg<ddr_pi_reg_69::DdrPiReg69Spec>;
#[doc = "DDR PHY Independent Register 69"]
pub mod ddr_pi_reg_69;
#[doc = "DDR_PI_REG_70 (rw) register accessor: DDR PHY Independent Register 70\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_70::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_70::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_70`]
module"]
#[doc(alias = "DDR_PI_REG_70")]
pub type DdrPiReg70 = crate::Reg<ddr_pi_reg_70::DdrPiReg70Spec>;
#[doc = "DDR PHY Independent Register 70"]
pub mod ddr_pi_reg_70;
#[doc = "DDR_PI_REG_71 (rw) register accessor: DDR PHY Independent Register 71\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_71::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_71::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_71`]
module"]
#[doc(alias = "DDR_PI_REG_71")]
pub type DdrPiReg71 = crate::Reg<ddr_pi_reg_71::DdrPiReg71Spec>;
#[doc = "DDR PHY Independent Register 71"]
pub mod ddr_pi_reg_71;
#[doc = "DDR_PI_REG_72 (rw) register accessor: DDR PHY Independent Register 72\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_72::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_72::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_72`]
module"]
#[doc(alias = "DDR_PI_REG_72")]
pub type DdrPiReg72 = crate::Reg<ddr_pi_reg_72::DdrPiReg72Spec>;
#[doc = "DDR PHY Independent Register 72"]
pub mod ddr_pi_reg_72;
#[doc = "DDR_PI_REG_73 (rw) register accessor: DDR PHY Independent Register 73\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_73::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_73::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_73`]
module"]
#[doc(alias = "DDR_PI_REG_73")]
pub type DdrPiReg73 = crate::Reg<ddr_pi_reg_73::DdrPiReg73Spec>;
#[doc = "DDR PHY Independent Register 73"]
pub mod ddr_pi_reg_73;
#[doc = "DDR_PI_REG_74 (rw) register accessor: DDR PHY Independent Register 74\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_74::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_74::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_74`]
module"]
#[doc(alias = "DDR_PI_REG_74")]
pub type DdrPiReg74 = crate::Reg<ddr_pi_reg_74::DdrPiReg74Spec>;
#[doc = "DDR PHY Independent Register 74"]
pub mod ddr_pi_reg_74;
#[doc = "DDR_PI_REG_75 (rw) register accessor: DDR PHY Independent Register 75\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_75::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_75::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_75`]
module"]
#[doc(alias = "DDR_PI_REG_75")]
pub type DdrPiReg75 = crate::Reg<ddr_pi_reg_75::DdrPiReg75Spec>;
#[doc = "DDR PHY Independent Register 75"]
pub mod ddr_pi_reg_75;
#[doc = "DDR_PI_REG_76 (rw) register accessor: DDR PHY Independent Register 76\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_76::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_76::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_76`]
module"]
#[doc(alias = "DDR_PI_REG_76")]
pub type DdrPiReg76 = crate::Reg<ddr_pi_reg_76::DdrPiReg76Spec>;
#[doc = "DDR PHY Independent Register 76"]
pub mod ddr_pi_reg_76;
#[doc = "DDR_PI_REG_77 (rw) register accessor: DDR PHY Independent Register 77\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_77::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_77::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_77`]
module"]
#[doc(alias = "DDR_PI_REG_77")]
pub type DdrPiReg77 = crate::Reg<ddr_pi_reg_77::DdrPiReg77Spec>;
#[doc = "DDR PHY Independent Register 77"]
pub mod ddr_pi_reg_77;
#[doc = "DDR_PI_REG_78 (rw) register accessor: DDR PHY Independent Register 78\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_78::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_78::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_78`]
module"]
#[doc(alias = "DDR_PI_REG_78")]
pub type DdrPiReg78 = crate::Reg<ddr_pi_reg_78::DdrPiReg78Spec>;
#[doc = "DDR PHY Independent Register 78"]
pub mod ddr_pi_reg_78;
#[doc = "DDR_PI_REG_79 (rw) register accessor: DDR PHY Independent Register 79\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_79::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_79::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_79`]
module"]
#[doc(alias = "DDR_PI_REG_79")]
pub type DdrPiReg79 = crate::Reg<ddr_pi_reg_79::DdrPiReg79Spec>;
#[doc = "DDR PHY Independent Register 79"]
pub mod ddr_pi_reg_79;
#[doc = "DDR_PI_REG_80 (rw) register accessor: DDR PHY Independent Register 80\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_80::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_80::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_80`]
module"]
#[doc(alias = "DDR_PI_REG_80")]
pub type DdrPiReg80 = crate::Reg<ddr_pi_reg_80::DdrPiReg80Spec>;
#[doc = "DDR PHY Independent Register 80"]
pub mod ddr_pi_reg_80;
#[doc = "DDR_PI_REG_81 (rw) register accessor: DDR PHY Independent Register 81\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_81::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_81::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_81`]
module"]
#[doc(alias = "DDR_PI_REG_81")]
pub type DdrPiReg81 = crate::Reg<ddr_pi_reg_81::DdrPiReg81Spec>;
#[doc = "DDR PHY Independent Register 81"]
pub mod ddr_pi_reg_81;
#[doc = "DDR_PI_REG_82 (rw) register accessor: DDR PHY Independent Register 82\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_82::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_82::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_82`]
module"]
#[doc(alias = "DDR_PI_REG_82")]
pub type DdrPiReg82 = crate::Reg<ddr_pi_reg_82::DdrPiReg82Spec>;
#[doc = "DDR PHY Independent Register 82"]
pub mod ddr_pi_reg_82;
#[doc = "DDR_PI_REG_83 (rw) register accessor: DDR PHY Independent Register 83\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_83::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_83::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_83`]
module"]
#[doc(alias = "DDR_PI_REG_83")]
pub type DdrPiReg83 = crate::Reg<ddr_pi_reg_83::DdrPiReg83Spec>;
#[doc = "DDR PHY Independent Register 83"]
pub mod ddr_pi_reg_83;
#[doc = "DDR_PI_REG_84 (rw) register accessor: DDR PHY Independent Register 84\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_84::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_84::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_84`]
module"]
#[doc(alias = "DDR_PI_REG_84")]
pub type DdrPiReg84 = crate::Reg<ddr_pi_reg_84::DdrPiReg84Spec>;
#[doc = "DDR PHY Independent Register 84"]
pub mod ddr_pi_reg_84;
#[doc = "DDR_PI_REG_85 (rw) register accessor: DDR PHY Independent Register 85\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_85::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_85::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_85`]
module"]
#[doc(alias = "DDR_PI_REG_85")]
pub type DdrPiReg85 = crate::Reg<ddr_pi_reg_85::DdrPiReg85Spec>;
#[doc = "DDR PHY Independent Register 85"]
pub mod ddr_pi_reg_85;
#[doc = "DDR_PI_REG_86 (rw) register accessor: DDR PHY Independent Register 86\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_86::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_86::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_86`]
module"]
#[doc(alias = "DDR_PI_REG_86")]
pub type DdrPiReg86 = crate::Reg<ddr_pi_reg_86::DdrPiReg86Spec>;
#[doc = "DDR PHY Independent Register 86"]
pub mod ddr_pi_reg_86;
#[doc = "DDR_PI_REG_87 (rw) register accessor: DDR PHY Independent Register 87\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_87::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_87::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_87`]
module"]
#[doc(alias = "DDR_PI_REG_87")]
pub type DdrPiReg87 = crate::Reg<ddr_pi_reg_87::DdrPiReg87Spec>;
#[doc = "DDR PHY Independent Register 87"]
pub mod ddr_pi_reg_87;
#[doc = "DDR_PI_REG_88 (rw) register accessor: DDR PHY Independent Register 88\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_88::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_88::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_88`]
module"]
#[doc(alias = "DDR_PI_REG_88")]
pub type DdrPiReg88 = crate::Reg<ddr_pi_reg_88::DdrPiReg88Spec>;
#[doc = "DDR PHY Independent Register 88"]
pub mod ddr_pi_reg_88;
#[doc = "DDR_PI_REG_89 (rw) register accessor: DDR PHY Independent Register 89\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_89::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_89::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_89`]
module"]
#[doc(alias = "DDR_PI_REG_89")]
pub type DdrPiReg89 = crate::Reg<ddr_pi_reg_89::DdrPiReg89Spec>;
#[doc = "DDR PHY Independent Register 89"]
pub mod ddr_pi_reg_89;
#[doc = "DDR_PI_REG_90 (rw) register accessor: DDR PHY Independent Register 90\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_90::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_90::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_90`]
module"]
#[doc(alias = "DDR_PI_REG_90")]
pub type DdrPiReg90 = crate::Reg<ddr_pi_reg_90::DdrPiReg90Spec>;
#[doc = "DDR PHY Independent Register 90"]
pub mod ddr_pi_reg_90;
#[doc = "DDR_PI_REG_91 (rw) register accessor: DDR PHY Independent Register 91\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_91::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_91::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_91`]
module"]
#[doc(alias = "DDR_PI_REG_91")]
pub type DdrPiReg91 = crate::Reg<ddr_pi_reg_91::DdrPiReg91Spec>;
#[doc = "DDR PHY Independent Register 91"]
pub mod ddr_pi_reg_91;
#[doc = "DDR_PI_REG_92 (rw) register accessor: DDR PHY Independent Register 92\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_92::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_92::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_92`]
module"]
#[doc(alias = "DDR_PI_REG_92")]
pub type DdrPiReg92 = crate::Reg<ddr_pi_reg_92::DdrPiReg92Spec>;
#[doc = "DDR PHY Independent Register 92"]
pub mod ddr_pi_reg_92;
#[doc = "DDR_PI_REG_93 (rw) register accessor: DDR PHY Independent Register 93\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_93::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_93::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_93`]
module"]
#[doc(alias = "DDR_PI_REG_93")]
pub type DdrPiReg93 = crate::Reg<ddr_pi_reg_93::DdrPiReg93Spec>;
#[doc = "DDR PHY Independent Register 93"]
pub mod ddr_pi_reg_93;
#[doc = "DDR_PI_REG_94 (rw) register accessor: DDR PHY Independent Register 94\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_94::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_94::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_94`]
module"]
#[doc(alias = "DDR_PI_REG_94")]
pub type DdrPiReg94 = crate::Reg<ddr_pi_reg_94::DdrPiReg94Spec>;
#[doc = "DDR PHY Independent Register 94"]
pub mod ddr_pi_reg_94;
#[doc = "DDR_PI_REG_95 (rw) register accessor: DDR PHY Independent Register 95\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_95::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_95::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_95`]
module"]
#[doc(alias = "DDR_PI_REG_95")]
pub type DdrPiReg95 = crate::Reg<ddr_pi_reg_95::DdrPiReg95Spec>;
#[doc = "DDR PHY Independent Register 95"]
pub mod ddr_pi_reg_95;
#[doc = "DDR_PI_REG_96 (rw) register accessor: DDR PHY Independent Register 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_96::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_96::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_96`]
module"]
#[doc(alias = "DDR_PI_REG_96")]
pub type DdrPiReg96 = crate::Reg<ddr_pi_reg_96::DdrPiReg96Spec>;
#[doc = "DDR PHY Independent Register 96"]
pub mod ddr_pi_reg_96;
#[doc = "DDR_PI_REG_97 (rw) register accessor: DDR PHY Independent Register 97\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_97::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_97::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_97`]
module"]
#[doc(alias = "DDR_PI_REG_97")]
pub type DdrPiReg97 = crate::Reg<ddr_pi_reg_97::DdrPiReg97Spec>;
#[doc = "DDR PHY Independent Register 97"]
pub mod ddr_pi_reg_97;
#[doc = "DDR_PI_REG_98 (rw) register accessor: DDR PHY Independent Register 98\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_98::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_98::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_98`]
module"]
#[doc(alias = "DDR_PI_REG_98")]
pub type DdrPiReg98 = crate::Reg<ddr_pi_reg_98::DdrPiReg98Spec>;
#[doc = "DDR PHY Independent Register 98"]
pub mod ddr_pi_reg_98;
#[doc = "DDR_PI_REG_99 (rw) register accessor: DDR PHY Independent Register 99\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_99::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_99::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_99`]
module"]
#[doc(alias = "DDR_PI_REG_99")]
pub type DdrPiReg99 = crate::Reg<ddr_pi_reg_99::DdrPiReg99Spec>;
#[doc = "DDR PHY Independent Register 99"]
pub mod ddr_pi_reg_99;
#[doc = "DDR_PI_REG_100 (rw) register accessor: DDR PHY Independent Register 100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_100::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_100::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_100`]
module"]
#[doc(alias = "DDR_PI_REG_100")]
pub type DdrPiReg100 = crate::Reg<ddr_pi_reg_100::DdrPiReg100Spec>;
#[doc = "DDR PHY Independent Register 100"]
pub mod ddr_pi_reg_100;
#[doc = "DDR_PI_REG_101 (rw) register accessor: DDR PHY Independent Register 101\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_101::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_101::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_101`]
module"]
#[doc(alias = "DDR_PI_REG_101")]
pub type DdrPiReg101 = crate::Reg<ddr_pi_reg_101::DdrPiReg101Spec>;
#[doc = "DDR PHY Independent Register 101"]
pub mod ddr_pi_reg_101;
#[doc = "DDR_PI_REG_102 (rw) register accessor: DDR PHY Independent Register 102\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_102::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_102::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_102`]
module"]
#[doc(alias = "DDR_PI_REG_102")]
pub type DdrPiReg102 = crate::Reg<ddr_pi_reg_102::DdrPiReg102Spec>;
#[doc = "DDR PHY Independent Register 102"]
pub mod ddr_pi_reg_102;
#[doc = "DDR_PI_REG_103 (rw) register accessor: DDR PHY Independent Register 103\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_103::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_103::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_103`]
module"]
#[doc(alias = "DDR_PI_REG_103")]
pub type DdrPiReg103 = crate::Reg<ddr_pi_reg_103::DdrPiReg103Spec>;
#[doc = "DDR PHY Independent Register 103"]
pub mod ddr_pi_reg_103;
#[doc = "DDR_PI_REG_104 (rw) register accessor: DDR PHY Independent Register 104\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_104::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_104::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_104`]
module"]
#[doc(alias = "DDR_PI_REG_104")]
pub type DdrPiReg104 = crate::Reg<ddr_pi_reg_104::DdrPiReg104Spec>;
#[doc = "DDR PHY Independent Register 104"]
pub mod ddr_pi_reg_104;
#[doc = "DDR_PI_REG_105 (rw) register accessor: DDR PHY Independent Register 105\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_105::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_105::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_105`]
module"]
#[doc(alias = "DDR_PI_REG_105")]
pub type DdrPiReg105 = crate::Reg<ddr_pi_reg_105::DdrPiReg105Spec>;
#[doc = "DDR PHY Independent Register 105"]
pub mod ddr_pi_reg_105;
#[doc = "DDR_PI_REG_106 (rw) register accessor: DDR PHY Independent Register 106\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_106::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_106::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_106`]
module"]
#[doc(alias = "DDR_PI_REG_106")]
pub type DdrPiReg106 = crate::Reg<ddr_pi_reg_106::DdrPiReg106Spec>;
#[doc = "DDR PHY Independent Register 106"]
pub mod ddr_pi_reg_106;
#[doc = "DDR_PI_REG_107 (rw) register accessor: DDR PHY Independent Register 107\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_107::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_107::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_107`]
module"]
#[doc(alias = "DDR_PI_REG_107")]
pub type DdrPiReg107 = crate::Reg<ddr_pi_reg_107::DdrPiReg107Spec>;
#[doc = "DDR PHY Independent Register 107"]
pub mod ddr_pi_reg_107;
#[doc = "DDR_PI_REG_108 (rw) register accessor: DDR PHY Independent Register 108\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_108::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_108::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_108`]
module"]
#[doc(alias = "DDR_PI_REG_108")]
pub type DdrPiReg108 = crate::Reg<ddr_pi_reg_108::DdrPiReg108Spec>;
#[doc = "DDR PHY Independent Register 108"]
pub mod ddr_pi_reg_108;
#[doc = "DDR_PI_REG_109 (rw) register accessor: DDR PHY Independent Register 109\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_109::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_109::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_109`]
module"]
#[doc(alias = "DDR_PI_REG_109")]
pub type DdrPiReg109 = crate::Reg<ddr_pi_reg_109::DdrPiReg109Spec>;
#[doc = "DDR PHY Independent Register 109"]
pub mod ddr_pi_reg_109;
#[doc = "DDR_PI_REG_110 (rw) register accessor: DDR PHY Independent Register 110\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_110::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_110::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_110`]
module"]
#[doc(alias = "DDR_PI_REG_110")]
pub type DdrPiReg110 = crate::Reg<ddr_pi_reg_110::DdrPiReg110Spec>;
#[doc = "DDR PHY Independent Register 110"]
pub mod ddr_pi_reg_110;
#[doc = "DDR_PI_REG_111 (rw) register accessor: DDR PHY Independent Register 111\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_111::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_111::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_111`]
module"]
#[doc(alias = "DDR_PI_REG_111")]
pub type DdrPiReg111 = crate::Reg<ddr_pi_reg_111::DdrPiReg111Spec>;
#[doc = "DDR PHY Independent Register 111"]
pub mod ddr_pi_reg_111;
#[doc = "DDR_PI_REG_112 (rw) register accessor: DDR PHY Independent Register 112\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_112::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_112::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_112`]
module"]
#[doc(alias = "DDR_PI_REG_112")]
pub type DdrPiReg112 = crate::Reg<ddr_pi_reg_112::DdrPiReg112Spec>;
#[doc = "DDR PHY Independent Register 112"]
pub mod ddr_pi_reg_112;
#[doc = "DDR_PI_REG_113 (rw) register accessor: DDR PHY Independent Register 113\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_113::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_113::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_113`]
module"]
#[doc(alias = "DDR_PI_REG_113")]
pub type DdrPiReg113 = crate::Reg<ddr_pi_reg_113::DdrPiReg113Spec>;
#[doc = "DDR PHY Independent Register 113"]
pub mod ddr_pi_reg_113;
#[doc = "DDR_PI_REG_114 (rw) register accessor: DDR PHY Independent Register 114\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_114::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_114::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_114`]
module"]
#[doc(alias = "DDR_PI_REG_114")]
pub type DdrPiReg114 = crate::Reg<ddr_pi_reg_114::DdrPiReg114Spec>;
#[doc = "DDR PHY Independent Register 114"]
pub mod ddr_pi_reg_114;
#[doc = "DDR_PI_REG_115 (rw) register accessor: DDR PHY Independent Register 115\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_115::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_115::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_115`]
module"]
#[doc(alias = "DDR_PI_REG_115")]
pub type DdrPiReg115 = crate::Reg<ddr_pi_reg_115::DdrPiReg115Spec>;
#[doc = "DDR PHY Independent Register 115"]
pub mod ddr_pi_reg_115;
#[doc = "DDR_PI_REG_116 (rw) register accessor: DDR PHY Independent Register 116\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_116::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_116::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_116`]
module"]
#[doc(alias = "DDR_PI_REG_116")]
pub type DdrPiReg116 = crate::Reg<ddr_pi_reg_116::DdrPiReg116Spec>;
#[doc = "DDR PHY Independent Register 116"]
pub mod ddr_pi_reg_116;
#[doc = "DDR_PI_REG_117 (rw) register accessor: DDR PHY Independent Register 117\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_117::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_117::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_117`]
module"]
#[doc(alias = "DDR_PI_REG_117")]
pub type DdrPiReg117 = crate::Reg<ddr_pi_reg_117::DdrPiReg117Spec>;
#[doc = "DDR PHY Independent Register 117"]
pub mod ddr_pi_reg_117;
#[doc = "DDR_PI_REG_118 (rw) register accessor: DDR PHY Independent Register 118\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_118::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_118::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_118`]
module"]
#[doc(alias = "DDR_PI_REG_118")]
pub type DdrPiReg118 = crate::Reg<ddr_pi_reg_118::DdrPiReg118Spec>;
#[doc = "DDR PHY Independent Register 118"]
pub mod ddr_pi_reg_118;
#[doc = "DDR_PI_REG_119 (rw) register accessor: DDR PHY Independent Register 119\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_119::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_119::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_119`]
module"]
#[doc(alias = "DDR_PI_REG_119")]
pub type DdrPiReg119 = crate::Reg<ddr_pi_reg_119::DdrPiReg119Spec>;
#[doc = "DDR PHY Independent Register 119"]
pub mod ddr_pi_reg_119;
#[doc = "DDR_PI_REG_120 (rw) register accessor: DDR PHY Independent Register 120\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_120::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_120::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_120`]
module"]
#[doc(alias = "DDR_PI_REG_120")]
pub type DdrPiReg120 = crate::Reg<ddr_pi_reg_120::DdrPiReg120Spec>;
#[doc = "DDR PHY Independent Register 120"]
pub mod ddr_pi_reg_120;
#[doc = "DDR_PI_REG_121 (rw) register accessor: DDR PHY Independent Register 121\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_121::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_121::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_121`]
module"]
#[doc(alias = "DDR_PI_REG_121")]
pub type DdrPiReg121 = crate::Reg<ddr_pi_reg_121::DdrPiReg121Spec>;
#[doc = "DDR PHY Independent Register 121"]
pub mod ddr_pi_reg_121;
#[doc = "DDR_PI_REG_122 (rw) register accessor: DDR PHY Independent Register 122\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_122::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_122::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_122`]
module"]
#[doc(alias = "DDR_PI_REG_122")]
pub type DdrPiReg122 = crate::Reg<ddr_pi_reg_122::DdrPiReg122Spec>;
#[doc = "DDR PHY Independent Register 122"]
pub mod ddr_pi_reg_122;
#[doc = "DDR_PI_REG_123 (rw) register accessor: DDR PHY Independent Register 123\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_123::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_123::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_123`]
module"]
#[doc(alias = "DDR_PI_REG_123")]
pub type DdrPiReg123 = crate::Reg<ddr_pi_reg_123::DdrPiReg123Spec>;
#[doc = "DDR PHY Independent Register 123"]
pub mod ddr_pi_reg_123;
#[doc = "DDR_PI_REG_124 (rw) register accessor: DDR PHY Independent Register 124\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_124::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_124::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_124`]
module"]
#[doc(alias = "DDR_PI_REG_124")]
pub type DdrPiReg124 = crate::Reg<ddr_pi_reg_124::DdrPiReg124Spec>;
#[doc = "DDR PHY Independent Register 124"]
pub mod ddr_pi_reg_124;
#[doc = "DDR_PI_REG_125 (rw) register accessor: DDR PHY Independent Register 125\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_125::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_125::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_125`]
module"]
#[doc(alias = "DDR_PI_REG_125")]
pub type DdrPiReg125 = crate::Reg<ddr_pi_reg_125::DdrPiReg125Spec>;
#[doc = "DDR PHY Independent Register 125"]
pub mod ddr_pi_reg_125;
#[doc = "DDR_PI_REG_126 (rw) register accessor: DDR PHY Independent Register 126\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_126::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_126::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_126`]
module"]
#[doc(alias = "DDR_PI_REG_126")]
pub type DdrPiReg126 = crate::Reg<ddr_pi_reg_126::DdrPiReg126Spec>;
#[doc = "DDR PHY Independent Register 126"]
pub mod ddr_pi_reg_126;
#[doc = "DDR_PI_REG_127 (rw) register accessor: DDR PHY Independent Register 127\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_127::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_127::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_127`]
module"]
#[doc(alias = "DDR_PI_REG_127")]
pub type DdrPiReg127 = crate::Reg<ddr_pi_reg_127::DdrPiReg127Spec>;
#[doc = "DDR PHY Independent Register 127"]
pub mod ddr_pi_reg_127;
#[doc = "DDR_PI_REG_128 (rw) register accessor: DDR PHY Independent Register 128\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_128::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_128::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_128`]
module"]
#[doc(alias = "DDR_PI_REG_128")]
pub type DdrPiReg128 = crate::Reg<ddr_pi_reg_128::DdrPiReg128Spec>;
#[doc = "DDR PHY Independent Register 128"]
pub mod ddr_pi_reg_128;
#[doc = "DDR_PI_REG_129 (rw) register accessor: DDR PHY Independent Register 129\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_129::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_129::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_129`]
module"]
#[doc(alias = "DDR_PI_REG_129")]
pub type DdrPiReg129 = crate::Reg<ddr_pi_reg_129::DdrPiReg129Spec>;
#[doc = "DDR PHY Independent Register 129"]
pub mod ddr_pi_reg_129;
#[doc = "DDR_PI_REG_130 (rw) register accessor: DDR PHY Independent Register 130\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_130::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_130::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_130`]
module"]
#[doc(alias = "DDR_PI_REG_130")]
pub type DdrPiReg130 = crate::Reg<ddr_pi_reg_130::DdrPiReg130Spec>;
#[doc = "DDR PHY Independent Register 130"]
pub mod ddr_pi_reg_130;
#[doc = "DDR_PI_REG_131 (rw) register accessor: DDR PHY Independent Register 131\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_131::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_131::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_131`]
module"]
#[doc(alias = "DDR_PI_REG_131")]
pub type DdrPiReg131 = crate::Reg<ddr_pi_reg_131::DdrPiReg131Spec>;
#[doc = "DDR PHY Independent Register 131"]
pub mod ddr_pi_reg_131;
#[doc = "DDR_PI_REG_132 (rw) register accessor: DDR PHY Independent Register 132\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_132::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_132::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_132`]
module"]
#[doc(alias = "DDR_PI_REG_132")]
pub type DdrPiReg132 = crate::Reg<ddr_pi_reg_132::DdrPiReg132Spec>;
#[doc = "DDR PHY Independent Register 132"]
pub mod ddr_pi_reg_132;
#[doc = "DDR_PI_REG_133 (rw) register accessor: DDR PHY Independent Register 133\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_133::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_133::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_133`]
module"]
#[doc(alias = "DDR_PI_REG_133")]
pub type DdrPiReg133 = crate::Reg<ddr_pi_reg_133::DdrPiReg133Spec>;
#[doc = "DDR PHY Independent Register 133"]
pub mod ddr_pi_reg_133;
#[doc = "DDR_PI_REG_134 (rw) register accessor: DDR PHY Independent Register 134\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_134::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_134::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_134`]
module"]
#[doc(alias = "DDR_PI_REG_134")]
pub type DdrPiReg134 = crate::Reg<ddr_pi_reg_134::DdrPiReg134Spec>;
#[doc = "DDR PHY Independent Register 134"]
pub mod ddr_pi_reg_134;
#[doc = "DDR_PI_REG_135 (rw) register accessor: DDR PHY Independent Register 135\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_135::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_135::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_135`]
module"]
#[doc(alias = "DDR_PI_REG_135")]
pub type DdrPiReg135 = crate::Reg<ddr_pi_reg_135::DdrPiReg135Spec>;
#[doc = "DDR PHY Independent Register 135"]
pub mod ddr_pi_reg_135;
#[doc = "DDR_PI_REG_136 (rw) register accessor: DDR PHY Independent Register 136\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_136::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_136::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_136`]
module"]
#[doc(alias = "DDR_PI_REG_136")]
pub type DdrPiReg136 = crate::Reg<ddr_pi_reg_136::DdrPiReg136Spec>;
#[doc = "DDR PHY Independent Register 136"]
pub mod ddr_pi_reg_136;
#[doc = "DDR_PI_REG_137 (rw) register accessor: DDR PHY Independent Register 137\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_137::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_137::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_137`]
module"]
#[doc(alias = "DDR_PI_REG_137")]
pub type DdrPiReg137 = crate::Reg<ddr_pi_reg_137::DdrPiReg137Spec>;
#[doc = "DDR PHY Independent Register 137"]
pub mod ddr_pi_reg_137;
#[doc = "DDR_PI_REG_138 (rw) register accessor: DDR PHY Independent Register 138\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_138::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_138::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_138`]
module"]
#[doc(alias = "DDR_PI_REG_138")]
pub type DdrPiReg138 = crate::Reg<ddr_pi_reg_138::DdrPiReg138Spec>;
#[doc = "DDR PHY Independent Register 138"]
pub mod ddr_pi_reg_138;
#[doc = "DDR_PI_REG_139 (rw) register accessor: DDR PHY Independent Register 139\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_139::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_139::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_139`]
module"]
#[doc(alias = "DDR_PI_REG_139")]
pub type DdrPiReg139 = crate::Reg<ddr_pi_reg_139::DdrPiReg139Spec>;
#[doc = "DDR PHY Independent Register 139"]
pub mod ddr_pi_reg_139;
#[doc = "DDR_PI_REG_140 (rw) register accessor: DDR PHY Independent Register 140\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_140::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_140::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_140`]
module"]
#[doc(alias = "DDR_PI_REG_140")]
pub type DdrPiReg140 = crate::Reg<ddr_pi_reg_140::DdrPiReg140Spec>;
#[doc = "DDR PHY Independent Register 140"]
pub mod ddr_pi_reg_140;
#[doc = "DDR_PI_REG_155 (rw) register accessor: DDR PHY Independent Register 155\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_155::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_155::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_155`]
module"]
#[doc(alias = "DDR_PI_REG_155")]
pub type DdrPiReg155 = crate::Reg<ddr_pi_reg_155::DdrPiReg155Spec>;
#[doc = "DDR PHY Independent Register 155"]
pub mod ddr_pi_reg_155;
#[doc = "DDR_PI_REG_156 (rw) register accessor: DDR PHY Independent Register 156\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_156::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_156::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_156`]
module"]
#[doc(alias = "DDR_PI_REG_156")]
pub type DdrPiReg156 = crate::Reg<ddr_pi_reg_156::DdrPiReg156Spec>;
#[doc = "DDR PHY Independent Register 156"]
pub mod ddr_pi_reg_156;
#[doc = "DDR_PI_REG_157 (rw) register accessor: DDR PHY Independent Register 157\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_157::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_157::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_157`]
module"]
#[doc(alias = "DDR_PI_REG_157")]
pub type DdrPiReg157 = crate::Reg<ddr_pi_reg_157::DdrPiReg157Spec>;
#[doc = "DDR PHY Independent Register 157"]
pub mod ddr_pi_reg_157;
#[doc = "DDR_PI_REG_158 (rw) register accessor: DDR PHY Independent Register 158\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_158::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_158::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_158`]
module"]
#[doc(alias = "DDR_PI_REG_158")]
pub type DdrPiReg158 = crate::Reg<ddr_pi_reg_158::DdrPiReg158Spec>;
#[doc = "DDR PHY Independent Register 158"]
pub mod ddr_pi_reg_158;
#[doc = "DDR_PI_REG_159 (rw) register accessor: DDR PHY Independent Register 159\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_159::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_159::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_159`]
module"]
#[doc(alias = "DDR_PI_REG_159")]
pub type DdrPiReg159 = crate::Reg<ddr_pi_reg_159::DdrPiReg159Spec>;
#[doc = "DDR PHY Independent Register 159"]
pub mod ddr_pi_reg_159;
#[doc = "DDR_PI_REG_160 (rw) register accessor: DDR PHY Independent Register 160\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_160::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_160::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_160`]
module"]
#[doc(alias = "DDR_PI_REG_160")]
pub type DdrPiReg160 = crate::Reg<ddr_pi_reg_160::DdrPiReg160Spec>;
#[doc = "DDR PHY Independent Register 160"]
pub mod ddr_pi_reg_160;
#[doc = "DDR_PI_REG_161 (rw) register accessor: DDR PHY Independent Register 161\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_161::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_161::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_161`]
module"]
#[doc(alias = "DDR_PI_REG_161")]
pub type DdrPiReg161 = crate::Reg<ddr_pi_reg_161::DdrPiReg161Spec>;
#[doc = "DDR PHY Independent Register 161"]
pub mod ddr_pi_reg_161;
#[doc = "DDR_PI_REG_162 (rw) register accessor: DDR PHY Independent Register 162\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_162::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_162::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_162`]
module"]
#[doc(alias = "DDR_PI_REG_162")]
pub type DdrPiReg162 = crate::Reg<ddr_pi_reg_162::DdrPiReg162Spec>;
#[doc = "DDR PHY Independent Register 162"]
pub mod ddr_pi_reg_162;
#[doc = "DDR_PI_REG_163 (rw) register accessor: DDR PHY Independent Register 163\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_163::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_163::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_163`]
module"]
#[doc(alias = "DDR_PI_REG_163")]
pub type DdrPiReg163 = crate::Reg<ddr_pi_reg_163::DdrPiReg163Spec>;
#[doc = "DDR PHY Independent Register 163"]
pub mod ddr_pi_reg_163;
#[doc = "DDR_PI_REG_164 (rw) register accessor: DDR PHY Independent Register 164\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_164::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_164::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_164`]
module"]
#[doc(alias = "DDR_PI_REG_164")]
pub type DdrPiReg164 = crate::Reg<ddr_pi_reg_164::DdrPiReg164Spec>;
#[doc = "DDR PHY Independent Register 164"]
pub mod ddr_pi_reg_164;
#[doc = "DDR_PI_REG_165 (rw) register accessor: DDR PHY Independent Register 165\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_165::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_165::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_165`]
module"]
#[doc(alias = "DDR_PI_REG_165")]
pub type DdrPiReg165 = crate::Reg<ddr_pi_reg_165::DdrPiReg165Spec>;
#[doc = "DDR PHY Independent Register 165"]
pub mod ddr_pi_reg_165;
#[doc = "DDR_PI_REG_166 (rw) register accessor: DDR PHY Independent Register 166\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_166::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_166::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_166`]
module"]
#[doc(alias = "DDR_PI_REG_166")]
pub type DdrPiReg166 = crate::Reg<ddr_pi_reg_166::DdrPiReg166Spec>;
#[doc = "DDR PHY Independent Register 166"]
pub mod ddr_pi_reg_166;
#[doc = "DDR_PI_REG_167 (rw) register accessor: DDR PHY Independent Register 167\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_167::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_167::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_167`]
module"]
#[doc(alias = "DDR_PI_REG_167")]
pub type DdrPiReg167 = crate::Reg<ddr_pi_reg_167::DdrPiReg167Spec>;
#[doc = "DDR PHY Independent Register 167"]
pub mod ddr_pi_reg_167;
#[doc = "DDR_PI_REG_168 (rw) register accessor: DDR PHY Independent Register 168\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_168::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_168::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_168`]
module"]
#[doc(alias = "DDR_PI_REG_168")]
pub type DdrPiReg168 = crate::Reg<ddr_pi_reg_168::DdrPiReg168Spec>;
#[doc = "DDR PHY Independent Register 168"]
pub mod ddr_pi_reg_168;
#[doc = "DDR_PI_REG_169 (rw) register accessor: DDR PHY Independent Register 169\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_169::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_169::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_169`]
module"]
#[doc(alias = "DDR_PI_REG_169")]
pub type DdrPiReg169 = crate::Reg<ddr_pi_reg_169::DdrPiReg169Spec>;
#[doc = "DDR PHY Independent Register 169"]
pub mod ddr_pi_reg_169;
#[doc = "DDR_PI_REG_174 (r) register accessor: DDR PHY Independent Register 174\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_174::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_174`]
module"]
#[doc(alias = "DDR_PI_REG_174")]
pub type DdrPiReg174 = crate::Reg<ddr_pi_reg_174::DdrPiReg174Spec>;
#[doc = "DDR PHY Independent Register 174"]
pub mod ddr_pi_reg_174;
#[doc = "DDR_PI_REG_175 (w) register accessor: DDR PHY Independent Register 175\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_175::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_175`]
module"]
#[doc(alias = "DDR_PI_REG_175")]
pub type DdrPiReg175 = crate::Reg<ddr_pi_reg_175::DdrPiReg175Spec>;
#[doc = "DDR PHY Independent Register 175"]
pub mod ddr_pi_reg_175;
#[doc = "DDR_PI_REG_176 (rw) register accessor: DDR PHY Independent Register 176\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_176::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_176::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_176`]
module"]
#[doc(alias = "DDR_PI_REG_176")]
pub type DdrPiReg176 = crate::Reg<ddr_pi_reg_176::DdrPiReg176Spec>;
#[doc = "DDR PHY Independent Register 176"]
pub mod ddr_pi_reg_176;
#[doc = "DDR_PI_REG_186 (rw) register accessor: DDR PHY Independent Register 186\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_186::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_186::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_186`]
module"]
#[doc(alias = "DDR_PI_REG_186")]
pub type DdrPiReg186 = crate::Reg<ddr_pi_reg_186::DdrPiReg186Spec>;
#[doc = "DDR PHY Independent Register 186"]
pub mod ddr_pi_reg_186;
#[doc = "DDR_PI_REG_187 (rw) register accessor: DDR PHY Independent Register 187\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_187::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_187::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_187`]
module"]
#[doc(alias = "DDR_PI_REG_187")]
pub type DdrPiReg187 = crate::Reg<ddr_pi_reg_187::DdrPiReg187Spec>;
#[doc = "DDR PHY Independent Register 187"]
pub mod ddr_pi_reg_187;
#[doc = "DDR_PI_REG_188 (rw) register accessor: DDR PHY Independent Register 188\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_188::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_188::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_188`]
module"]
#[doc(alias = "DDR_PI_REG_188")]
pub type DdrPiReg188 = crate::Reg<ddr_pi_reg_188::DdrPiReg188Spec>;
#[doc = "DDR PHY Independent Register 188"]
pub mod ddr_pi_reg_188;
#[doc = "DDR_PI_REG_189 (rw) register accessor: DDR PHY Independent Register 189\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_189::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_189::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_189`]
module"]
#[doc(alias = "DDR_PI_REG_189")]
pub type DdrPiReg189 = crate::Reg<ddr_pi_reg_189::DdrPiReg189Spec>;
#[doc = "DDR PHY Independent Register 189"]
pub mod ddr_pi_reg_189;
#[doc = "DDR_PI_REG_190 (rw) register accessor: DDR PHY Independent Register 190\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_190::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_190::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_190`]
module"]
#[doc(alias = "DDR_PI_REG_190")]
pub type DdrPiReg190 = crate::Reg<ddr_pi_reg_190::DdrPiReg190Spec>;
#[doc = "DDR PHY Independent Register 190"]
pub mod ddr_pi_reg_190;
#[doc = "DDR_PI_REG_191 (rw) register accessor: DDR PHY Independent Register 191\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_191::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_191::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_191`]
module"]
#[doc(alias = "DDR_PI_REG_191")]
pub type DdrPiReg191 = crate::Reg<ddr_pi_reg_191::DdrPiReg191Spec>;
#[doc = "DDR PHY Independent Register 191"]
pub mod ddr_pi_reg_191;
#[doc = "DDR_PI_REG_192 (rw) register accessor: DDR PHY Independent Register 192\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_192::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_192::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_192`]
module"]
#[doc(alias = "DDR_PI_REG_192")]
pub type DdrPiReg192 = crate::Reg<ddr_pi_reg_192::DdrPiReg192Spec>;
#[doc = "DDR PHY Independent Register 192"]
pub mod ddr_pi_reg_192;
#[doc = "DDR_PI_REG_193 (r) register accessor: DDR PHY Independent Register 193\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_193::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_193`]
module"]
#[doc(alias = "DDR_PI_REG_193")]
pub type DdrPiReg193 = crate::Reg<ddr_pi_reg_193::DdrPiReg193Spec>;
#[doc = "DDR PHY Independent Register 193"]
pub mod ddr_pi_reg_193;
#[doc = "DDR_PI_REG_199 (rw) register accessor: DDR PHY Independent Register 199\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_199::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_199::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_pi_reg_199`]
module"]
#[doc(alias = "DDR_PI_REG_199")]
pub type DdrPiReg199 = crate::Reg<ddr_pi_reg_199::DdrPiReg199Spec>;
#[doc = "DDR PHY Independent Register 199"]
pub mod ddr_pi_reg_199;
