#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
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
    denali_phy_82: DenaliPhy82,
    denali_phy_83: DenaliPhy83,
    denali_phy_84: DenaliPhy84,
    denali_phy_85: DenaliPhy85,
    denali_phy_86: DenaliPhy86,
    denali_phy_87: DenaliPhy87,
    denali_phy_88: DenaliPhy88,
    denali_phy_89: DenaliPhy89,
    denali_phy_90: DenaliPhy90,
    _reserved91: [u8; 0x94],
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
    denali_phy_210: DenaliPhy210,
    denali_phy_211: DenaliPhy211,
    denali_phy_212: DenaliPhy212,
    denali_phy_213: DenaliPhy213,
    denali_phy_214: DenaliPhy214,
    denali_phy_215: DenaliPhy215,
    denali_phy_216: DenaliPhy216,
    denali_phy_217: DenaliPhy217,
    denali_phy_218: DenaliPhy218,
    _reserved182: [u8; 0x94],
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
    denali_phy_338: DenaliPhy338,
    denali_phy_339: DenaliPhy339,
    denali_phy_340: DenaliPhy340,
    denali_phy_341: DenaliPhy341,
    denali_phy_342: DenaliPhy342,
    denali_phy_343: DenaliPhy343,
    denali_phy_344: DenaliPhy344,
    denali_phy_345: DenaliPhy345,
    denali_phy_346: DenaliPhy346,
    _reserved273: [u8; 0x94],
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
    denali_phy_466: DenaliPhy466,
    denali_phy_467: DenaliPhy467,
    denali_phy_468: DenaliPhy468,
    denali_phy_469: DenaliPhy469,
    denali_phy_470: DenaliPhy470,
    denali_phy_471: DenaliPhy471,
    denali_phy_472: DenaliPhy472,
    denali_phy_473: DenaliPhy473,
    denali_phy_474: DenaliPhy474,
    _reserved364: [u8; 0x94],
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
    _reserved402: [u8; 0x0168],
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
    _reserved440: [u8; 0x0168],
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
    _reserved478: [u8; 0x0168],
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
    denali_phy_909: DenaliPhy909,
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
    pub const fn denali_phy_00(&self) -> &DenaliPhy00 {
        &self.denali_phy_00
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn denali_phy_01(&self) -> &DenaliPhy01 {
        &self.denali_phy_01
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn denali_phy_02(&self) -> &DenaliPhy02 {
        &self.denali_phy_02
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn denali_phy_03(&self) -> &DenaliPhy03 {
        &self.denali_phy_03
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn denali_phy_04(&self) -> &DenaliPhy04 {
        &self.denali_phy_04
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn denali_phy_05(&self) -> &DenaliPhy05 {
        &self.denali_phy_05
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn denali_phy_06(&self) -> &DenaliPhy06 {
        &self.denali_phy_06
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn denali_phy_07(&self) -> &DenaliPhy07 {
        &self.denali_phy_07
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn denali_phy_08(&self) -> &DenaliPhy08 {
        &self.denali_phy_08
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn denali_phy_09(&self) -> &DenaliPhy09 {
        &self.denali_phy_09
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn denali_phy_10(&self) -> &DenaliPhy10 {
        &self.denali_phy_10
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn denali_phy_11(&self) -> &DenaliPhy11 {
        &self.denali_phy_11
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn denali_phy_12(&self) -> &DenaliPhy12 {
        &self.denali_phy_12
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn denali_phy_13(&self) -> &DenaliPhy13 {
        &self.denali_phy_13
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn denali_phy_14(&self) -> &DenaliPhy14 {
        &self.denali_phy_14
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn denali_phy_15(&self) -> &DenaliPhy15 {
        &self.denali_phy_15
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn denali_phy_16(&self) -> &DenaliPhy16 {
        &self.denali_phy_16
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn denali_phy_17(&self) -> &DenaliPhy17 {
        &self.denali_phy_17
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn denali_phy_18(&self) -> &DenaliPhy18 {
        &self.denali_phy_18
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn denali_phy_19(&self) -> &DenaliPhy19 {
        &self.denali_phy_19
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn denali_phy_20(&self) -> &DenaliPhy20 {
        &self.denali_phy_20
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn denali_phy_21(&self) -> &DenaliPhy21 {
        &self.denali_phy_21
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn denali_phy_22(&self) -> &DenaliPhy22 {
        &self.denali_phy_22
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn denali_phy_23(&self) -> &DenaliPhy23 {
        &self.denali_phy_23
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn denali_phy_24(&self) -> &DenaliPhy24 {
        &self.denali_phy_24
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn denali_phy_25(&self) -> &DenaliPhy25 {
        &self.denali_phy_25
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn denali_phy_26(&self) -> &DenaliPhy26 {
        &self.denali_phy_26
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn denali_phy_27(&self) -> &DenaliPhy27 {
        &self.denali_phy_27
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn denali_phy_28(&self) -> &DenaliPhy28 {
        &self.denali_phy_28
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn denali_phy_29(&self) -> &DenaliPhy29 {
        &self.denali_phy_29
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn denali_phy_30(&self) -> &DenaliPhy30 {
        &self.denali_phy_30
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn denali_phy_31(&self) -> &DenaliPhy31 {
        &self.denali_phy_31
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn denali_phy_32(&self) -> &DenaliPhy32 {
        &self.denali_phy_32
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn denali_phy_33(&self) -> &DenaliPhy33 {
        &self.denali_phy_33
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn denali_phy_34(&self) -> &DenaliPhy34 {
        &self.denali_phy_34
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn denali_phy_35(&self) -> &DenaliPhy35 {
        &self.denali_phy_35
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn denali_phy_36(&self) -> &DenaliPhy36 {
        &self.denali_phy_36
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn denali_phy_37(&self) -> &DenaliPhy37 {
        &self.denali_phy_37
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn denali_phy_38(&self) -> &DenaliPhy38 {
        &self.denali_phy_38
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn denali_phy_39(&self) -> &DenaliPhy39 {
        &self.denali_phy_39
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn denali_phy_40(&self) -> &DenaliPhy40 {
        &self.denali_phy_40
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn denali_phy_41(&self) -> &DenaliPhy41 {
        &self.denali_phy_41
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn denali_phy_42(&self) -> &DenaliPhy42 {
        &self.denali_phy_42
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn denali_phy_43(&self) -> &DenaliPhy43 {
        &self.denali_phy_43
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn denali_phy_44(&self) -> &DenaliPhy44 {
        &self.denali_phy_44
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn denali_phy_45(&self) -> &DenaliPhy45 {
        &self.denali_phy_45
    }
    #[doc = "0xb8 - "]
    #[inline(always)]
    pub const fn denali_phy_46(&self) -> &DenaliPhy46 {
        &self.denali_phy_46
    }
    #[doc = "0xbc - "]
    #[inline(always)]
    pub const fn denali_phy_47(&self) -> &DenaliPhy47 {
        &self.denali_phy_47
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn denali_phy_48(&self) -> &DenaliPhy48 {
        &self.denali_phy_48
    }
    #[doc = "0xc4 - "]
    #[inline(always)]
    pub const fn denali_phy_49(&self) -> &DenaliPhy49 {
        &self.denali_phy_49
    }
    #[doc = "0xc8 - "]
    #[inline(always)]
    pub const fn denali_phy_50(&self) -> &DenaliPhy50 {
        &self.denali_phy_50
    }
    #[doc = "0xcc - "]
    #[inline(always)]
    pub const fn denali_phy_51(&self) -> &DenaliPhy51 {
        &self.denali_phy_51
    }
    #[doc = "0xd0 - "]
    #[inline(always)]
    pub const fn denali_phy_52(&self) -> &DenaliPhy52 {
        &self.denali_phy_52
    }
    #[doc = "0xd4 - "]
    #[inline(always)]
    pub const fn denali_phy_53(&self) -> &DenaliPhy53 {
        &self.denali_phy_53
    }
    #[doc = "0xd8 - "]
    #[inline(always)]
    pub const fn denali_phy_54(&self) -> &DenaliPhy54 {
        &self.denali_phy_54
    }
    #[doc = "0xdc - "]
    #[inline(always)]
    pub const fn denali_phy_55(&self) -> &DenaliPhy55 {
        &self.denali_phy_55
    }
    #[doc = "0xe0 - "]
    #[inline(always)]
    pub const fn denali_phy_56(&self) -> &DenaliPhy56 {
        &self.denali_phy_56
    }
    #[doc = "0xe4 - "]
    #[inline(always)]
    pub const fn denali_phy_57(&self) -> &DenaliPhy57 {
        &self.denali_phy_57
    }
    #[doc = "0xe8 - "]
    #[inline(always)]
    pub const fn denali_phy_58(&self) -> &DenaliPhy58 {
        &self.denali_phy_58
    }
    #[doc = "0xec - "]
    #[inline(always)]
    pub const fn denali_phy_59(&self) -> &DenaliPhy59 {
        &self.denali_phy_59
    }
    #[doc = "0xf0 - "]
    #[inline(always)]
    pub const fn denali_phy_60(&self) -> &DenaliPhy60 {
        &self.denali_phy_60
    }
    #[doc = "0xf4 - "]
    #[inline(always)]
    pub const fn denali_phy_61(&self) -> &DenaliPhy61 {
        &self.denali_phy_61
    }
    #[doc = "0xf8 - "]
    #[inline(always)]
    pub const fn denali_phy_62(&self) -> &DenaliPhy62 {
        &self.denali_phy_62
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn denali_phy_63(&self) -> &DenaliPhy63 {
        &self.denali_phy_63
    }
    #[doc = "0x100 - "]
    #[inline(always)]
    pub const fn denali_phy_64(&self) -> &DenaliPhy64 {
        &self.denali_phy_64
    }
    #[doc = "0x104 - "]
    #[inline(always)]
    pub const fn denali_phy_65(&self) -> &DenaliPhy65 {
        &self.denali_phy_65
    }
    #[doc = "0x108 - "]
    #[inline(always)]
    pub const fn denali_phy_66(&self) -> &DenaliPhy66 {
        &self.denali_phy_66
    }
    #[doc = "0x10c - "]
    #[inline(always)]
    pub const fn denali_phy_67(&self) -> &DenaliPhy67 {
        &self.denali_phy_67
    }
    #[doc = "0x110 - "]
    #[inline(always)]
    pub const fn denali_phy_68(&self) -> &DenaliPhy68 {
        &self.denali_phy_68
    }
    #[doc = "0x114 - "]
    #[inline(always)]
    pub const fn denali_phy_69(&self) -> &DenaliPhy69 {
        &self.denali_phy_69
    }
    #[doc = "0x118 - "]
    #[inline(always)]
    pub const fn denali_phy_70(&self) -> &DenaliPhy70 {
        &self.denali_phy_70
    }
    #[doc = "0x11c - "]
    #[inline(always)]
    pub const fn denali_phy_71(&self) -> &DenaliPhy71 {
        &self.denali_phy_71
    }
    #[doc = "0x120 - "]
    #[inline(always)]
    pub const fn denali_phy_72(&self) -> &DenaliPhy72 {
        &self.denali_phy_72
    }
    #[doc = "0x124 - "]
    #[inline(always)]
    pub const fn denali_phy_73(&self) -> &DenaliPhy73 {
        &self.denali_phy_73
    }
    #[doc = "0x128 - "]
    #[inline(always)]
    pub const fn denali_phy_74(&self) -> &DenaliPhy74 {
        &self.denali_phy_74
    }
    #[doc = "0x12c - "]
    #[inline(always)]
    pub const fn denali_phy_75(&self) -> &DenaliPhy75 {
        &self.denali_phy_75
    }
    #[doc = "0x130 - "]
    #[inline(always)]
    pub const fn denali_phy_76(&self) -> &DenaliPhy76 {
        &self.denali_phy_76
    }
    #[doc = "0x134 - "]
    #[inline(always)]
    pub const fn denali_phy_77(&self) -> &DenaliPhy77 {
        &self.denali_phy_77
    }
    #[doc = "0x138 - "]
    #[inline(always)]
    pub const fn denali_phy_78(&self) -> &DenaliPhy78 {
        &self.denali_phy_78
    }
    #[doc = "0x13c - "]
    #[inline(always)]
    pub const fn denali_phy_79(&self) -> &DenaliPhy79 {
        &self.denali_phy_79
    }
    #[doc = "0x140 - "]
    #[inline(always)]
    pub const fn denali_phy_80(&self) -> &DenaliPhy80 {
        &self.denali_phy_80
    }
    #[doc = "0x144 - "]
    #[inline(always)]
    pub const fn denali_phy_81(&self) -> &DenaliPhy81 {
        &self.denali_phy_81
    }
    #[doc = "0x148 - "]
    #[inline(always)]
    pub const fn denali_phy_82(&self) -> &DenaliPhy82 {
        &self.denali_phy_82
    }
    #[doc = "0x14c - "]
    #[inline(always)]
    pub const fn denali_phy_83(&self) -> &DenaliPhy83 {
        &self.denali_phy_83
    }
    #[doc = "0x150 - "]
    #[inline(always)]
    pub const fn denali_phy_84(&self) -> &DenaliPhy84 {
        &self.denali_phy_84
    }
    #[doc = "0x154 - "]
    #[inline(always)]
    pub const fn denali_phy_85(&self) -> &DenaliPhy85 {
        &self.denali_phy_85
    }
    #[doc = "0x158 - "]
    #[inline(always)]
    pub const fn denali_phy_86(&self) -> &DenaliPhy86 {
        &self.denali_phy_86
    }
    #[doc = "0x15c - "]
    #[inline(always)]
    pub const fn denali_phy_87(&self) -> &DenaliPhy87 {
        &self.denali_phy_87
    }
    #[doc = "0x160 - "]
    #[inline(always)]
    pub const fn denali_phy_88(&self) -> &DenaliPhy88 {
        &self.denali_phy_88
    }
    #[doc = "0x164 - "]
    #[inline(always)]
    pub const fn denali_phy_89(&self) -> &DenaliPhy89 {
        &self.denali_phy_89
    }
    #[doc = "0x168 - "]
    #[inline(always)]
    pub const fn denali_phy_90(&self) -> &DenaliPhy90 {
        &self.denali_phy_90
    }
    #[doc = "0x200 - "]
    #[inline(always)]
    pub const fn denali_phy_128(&self) -> &DenaliPhy128 {
        &self.denali_phy_128
    }
    #[doc = "0x204 - "]
    #[inline(always)]
    pub const fn denali_phy_129(&self) -> &DenaliPhy129 {
        &self.denali_phy_129
    }
    #[doc = "0x208 - "]
    #[inline(always)]
    pub const fn denali_phy_130(&self) -> &DenaliPhy130 {
        &self.denali_phy_130
    }
    #[doc = "0x20c - "]
    #[inline(always)]
    pub const fn denali_phy_131(&self) -> &DenaliPhy131 {
        &self.denali_phy_131
    }
    #[doc = "0x210 - "]
    #[inline(always)]
    pub const fn denali_phy_132(&self) -> &DenaliPhy132 {
        &self.denali_phy_132
    }
    #[doc = "0x214 - "]
    #[inline(always)]
    pub const fn denali_phy_133(&self) -> &DenaliPhy133 {
        &self.denali_phy_133
    }
    #[doc = "0x218 - "]
    #[inline(always)]
    pub const fn denali_phy_134(&self) -> &DenaliPhy134 {
        &self.denali_phy_134
    }
    #[doc = "0x21c - "]
    #[inline(always)]
    pub const fn denali_phy_135(&self) -> &DenaliPhy135 {
        &self.denali_phy_135
    }
    #[doc = "0x220 - "]
    #[inline(always)]
    pub const fn denali_phy_136(&self) -> &DenaliPhy136 {
        &self.denali_phy_136
    }
    #[doc = "0x224 - "]
    #[inline(always)]
    pub const fn denali_phy_137(&self) -> &DenaliPhy137 {
        &self.denali_phy_137
    }
    #[doc = "0x228 - "]
    #[inline(always)]
    pub const fn denali_phy_138(&self) -> &DenaliPhy138 {
        &self.denali_phy_138
    }
    #[doc = "0x22c - "]
    #[inline(always)]
    pub const fn denali_phy_139(&self) -> &DenaliPhy139 {
        &self.denali_phy_139
    }
    #[doc = "0x230 - "]
    #[inline(always)]
    pub const fn denali_phy_140(&self) -> &DenaliPhy140 {
        &self.denali_phy_140
    }
    #[doc = "0x234 - "]
    #[inline(always)]
    pub const fn denali_phy_141(&self) -> &DenaliPhy141 {
        &self.denali_phy_141
    }
    #[doc = "0x238 - "]
    #[inline(always)]
    pub const fn denali_phy_142(&self) -> &DenaliPhy142 {
        &self.denali_phy_142
    }
    #[doc = "0x23c - "]
    #[inline(always)]
    pub const fn denali_phy_143(&self) -> &DenaliPhy143 {
        &self.denali_phy_143
    }
    #[doc = "0x240 - "]
    #[inline(always)]
    pub const fn denali_phy_144(&self) -> &DenaliPhy144 {
        &self.denali_phy_144
    }
    #[doc = "0x244 - "]
    #[inline(always)]
    pub const fn denali_phy_145(&self) -> &DenaliPhy145 {
        &self.denali_phy_145
    }
    #[doc = "0x248 - "]
    #[inline(always)]
    pub const fn denali_phy_146(&self) -> &DenaliPhy146 {
        &self.denali_phy_146
    }
    #[doc = "0x24c - "]
    #[inline(always)]
    pub const fn denali_phy_147(&self) -> &DenaliPhy147 {
        &self.denali_phy_147
    }
    #[doc = "0x250 - "]
    #[inline(always)]
    pub const fn denali_phy_148(&self) -> &DenaliPhy148 {
        &self.denali_phy_148
    }
    #[doc = "0x254 - "]
    #[inline(always)]
    pub const fn denali_phy_149(&self) -> &DenaliPhy149 {
        &self.denali_phy_149
    }
    #[doc = "0x258 - "]
    #[inline(always)]
    pub const fn denali_phy_150(&self) -> &DenaliPhy150 {
        &self.denali_phy_150
    }
    #[doc = "0x25c - "]
    #[inline(always)]
    pub const fn denali_phy_151(&self) -> &DenaliPhy151 {
        &self.denali_phy_151
    }
    #[doc = "0x260 - "]
    #[inline(always)]
    pub const fn denali_phy_152(&self) -> &DenaliPhy152 {
        &self.denali_phy_152
    }
    #[doc = "0x264 - "]
    #[inline(always)]
    pub const fn denali_phy_153(&self) -> &DenaliPhy153 {
        &self.denali_phy_153
    }
    #[doc = "0x268 - "]
    #[inline(always)]
    pub const fn denali_phy_154(&self) -> &DenaliPhy154 {
        &self.denali_phy_154
    }
    #[doc = "0x26c - "]
    #[inline(always)]
    pub const fn denali_phy_155(&self) -> &DenaliPhy155 {
        &self.denali_phy_155
    }
    #[doc = "0x270 - "]
    #[inline(always)]
    pub const fn denali_phy_156(&self) -> &DenaliPhy156 {
        &self.denali_phy_156
    }
    #[doc = "0x274 - "]
    #[inline(always)]
    pub const fn denali_phy_157(&self) -> &DenaliPhy157 {
        &self.denali_phy_157
    }
    #[doc = "0x278 - "]
    #[inline(always)]
    pub const fn denali_phy_158(&self) -> &DenaliPhy158 {
        &self.denali_phy_158
    }
    #[doc = "0x27c - "]
    #[inline(always)]
    pub const fn denali_phy_159(&self) -> &DenaliPhy159 {
        &self.denali_phy_159
    }
    #[doc = "0x280 - "]
    #[inline(always)]
    pub const fn denali_phy_160(&self) -> &DenaliPhy160 {
        &self.denali_phy_160
    }
    #[doc = "0x284 - "]
    #[inline(always)]
    pub const fn denali_phy_161(&self) -> &DenaliPhy161 {
        &self.denali_phy_161
    }
    #[doc = "0x288 - "]
    #[inline(always)]
    pub const fn denali_phy_162(&self) -> &DenaliPhy162 {
        &self.denali_phy_162
    }
    #[doc = "0x28c - "]
    #[inline(always)]
    pub const fn denali_phy_163(&self) -> &DenaliPhy163 {
        &self.denali_phy_163
    }
    #[doc = "0x290 - "]
    #[inline(always)]
    pub const fn denali_phy_164(&self) -> &DenaliPhy164 {
        &self.denali_phy_164
    }
    #[doc = "0x294 - "]
    #[inline(always)]
    pub const fn denali_phy_165(&self) -> &DenaliPhy165 {
        &self.denali_phy_165
    }
    #[doc = "0x298 - "]
    #[inline(always)]
    pub const fn denali_phy_166(&self) -> &DenaliPhy166 {
        &self.denali_phy_166
    }
    #[doc = "0x29c - "]
    #[inline(always)]
    pub const fn denali_phy_167(&self) -> &DenaliPhy167 {
        &self.denali_phy_167
    }
    #[doc = "0x2a0 - "]
    #[inline(always)]
    pub const fn denali_phy_168(&self) -> &DenaliPhy168 {
        &self.denali_phy_168
    }
    #[doc = "0x2a4 - "]
    #[inline(always)]
    pub const fn denali_phy_169(&self) -> &DenaliPhy169 {
        &self.denali_phy_169
    }
    #[doc = "0x2a8 - "]
    #[inline(always)]
    pub const fn denali_phy_170(&self) -> &DenaliPhy170 {
        &self.denali_phy_170
    }
    #[doc = "0x2ac - "]
    #[inline(always)]
    pub const fn denali_phy_171(&self) -> &DenaliPhy171 {
        &self.denali_phy_171
    }
    #[doc = "0x2b0 - "]
    #[inline(always)]
    pub const fn denali_phy_172(&self) -> &DenaliPhy172 {
        &self.denali_phy_172
    }
    #[doc = "0x2b4 - "]
    #[inline(always)]
    pub const fn denali_phy_173(&self) -> &DenaliPhy173 {
        &self.denali_phy_173
    }
    #[doc = "0x2b8 - "]
    #[inline(always)]
    pub const fn denali_phy_174(&self) -> &DenaliPhy174 {
        &self.denali_phy_174
    }
    #[doc = "0x2bc - "]
    #[inline(always)]
    pub const fn denali_phy_175(&self) -> &DenaliPhy175 {
        &self.denali_phy_175
    }
    #[doc = "0x2c0 - "]
    #[inline(always)]
    pub const fn denali_phy_176(&self) -> &DenaliPhy176 {
        &self.denali_phy_176
    }
    #[doc = "0x2c4 - "]
    #[inline(always)]
    pub const fn denali_phy_177(&self) -> &DenaliPhy177 {
        &self.denali_phy_177
    }
    #[doc = "0x2c8 - "]
    #[inline(always)]
    pub const fn denali_phy_178(&self) -> &DenaliPhy178 {
        &self.denali_phy_178
    }
    #[doc = "0x2cc - "]
    #[inline(always)]
    pub const fn denali_phy_179(&self) -> &DenaliPhy179 {
        &self.denali_phy_179
    }
    #[doc = "0x2d0 - "]
    #[inline(always)]
    pub const fn denali_phy_180(&self) -> &DenaliPhy180 {
        &self.denali_phy_180
    }
    #[doc = "0x2d4 - "]
    #[inline(always)]
    pub const fn denali_phy_181(&self) -> &DenaliPhy181 {
        &self.denali_phy_181
    }
    #[doc = "0x2d8 - "]
    #[inline(always)]
    pub const fn denali_phy_182(&self) -> &DenaliPhy182 {
        &self.denali_phy_182
    }
    #[doc = "0x2dc - "]
    #[inline(always)]
    pub const fn denali_phy_183(&self) -> &DenaliPhy183 {
        &self.denali_phy_183
    }
    #[doc = "0x2e0 - "]
    #[inline(always)]
    pub const fn denali_phy_184(&self) -> &DenaliPhy184 {
        &self.denali_phy_184
    }
    #[doc = "0x2e4 - "]
    #[inline(always)]
    pub const fn denali_phy_185(&self) -> &DenaliPhy185 {
        &self.denali_phy_185
    }
    #[doc = "0x2e8 - "]
    #[inline(always)]
    pub const fn denali_phy_186(&self) -> &DenaliPhy186 {
        &self.denali_phy_186
    }
    #[doc = "0x2ec - "]
    #[inline(always)]
    pub const fn denali_phy_187(&self) -> &DenaliPhy187 {
        &self.denali_phy_187
    }
    #[doc = "0x2f0 - "]
    #[inline(always)]
    pub const fn denali_phy_188(&self) -> &DenaliPhy188 {
        &self.denali_phy_188
    }
    #[doc = "0x2f4 - "]
    #[inline(always)]
    pub const fn denali_phy_189(&self) -> &DenaliPhy189 {
        &self.denali_phy_189
    }
    #[doc = "0x2f8 - "]
    #[inline(always)]
    pub const fn denali_phy_190(&self) -> &DenaliPhy190 {
        &self.denali_phy_190
    }
    #[doc = "0x2fc - "]
    #[inline(always)]
    pub const fn denali_phy_191(&self) -> &DenaliPhy191 {
        &self.denali_phy_191
    }
    #[doc = "0x300 - "]
    #[inline(always)]
    pub const fn denali_phy_192(&self) -> &DenaliPhy192 {
        &self.denali_phy_192
    }
    #[doc = "0x304 - "]
    #[inline(always)]
    pub const fn denali_phy_193(&self) -> &DenaliPhy193 {
        &self.denali_phy_193
    }
    #[doc = "0x308 - "]
    #[inline(always)]
    pub const fn denali_phy_194(&self) -> &DenaliPhy194 {
        &self.denali_phy_194
    }
    #[doc = "0x30c - "]
    #[inline(always)]
    pub const fn denali_phy_195(&self) -> &DenaliPhy195 {
        &self.denali_phy_195
    }
    #[doc = "0x310 - "]
    #[inline(always)]
    pub const fn denali_phy_196(&self) -> &DenaliPhy196 {
        &self.denali_phy_196
    }
    #[doc = "0x314 - "]
    #[inline(always)]
    pub const fn denali_phy_197(&self) -> &DenaliPhy197 {
        &self.denali_phy_197
    }
    #[doc = "0x318 - "]
    #[inline(always)]
    pub const fn denali_phy_198(&self) -> &DenaliPhy198 {
        &self.denali_phy_198
    }
    #[doc = "0x31c - "]
    #[inline(always)]
    pub const fn denali_phy_199(&self) -> &DenaliPhy199 {
        &self.denali_phy_199
    }
    #[doc = "0x320 - "]
    #[inline(always)]
    pub const fn denali_phy_200(&self) -> &DenaliPhy200 {
        &self.denali_phy_200
    }
    #[doc = "0x324 - "]
    #[inline(always)]
    pub const fn denali_phy_201(&self) -> &DenaliPhy201 {
        &self.denali_phy_201
    }
    #[doc = "0x328 - "]
    #[inline(always)]
    pub const fn denali_phy_202(&self) -> &DenaliPhy202 {
        &self.denali_phy_202
    }
    #[doc = "0x32c - "]
    #[inline(always)]
    pub const fn denali_phy_203(&self) -> &DenaliPhy203 {
        &self.denali_phy_203
    }
    #[doc = "0x330 - "]
    #[inline(always)]
    pub const fn denali_phy_204(&self) -> &DenaliPhy204 {
        &self.denali_phy_204
    }
    #[doc = "0x334 - "]
    #[inline(always)]
    pub const fn denali_phy_205(&self) -> &DenaliPhy205 {
        &self.denali_phy_205
    }
    #[doc = "0x338 - "]
    #[inline(always)]
    pub const fn denali_phy_206(&self) -> &DenaliPhy206 {
        &self.denali_phy_206
    }
    #[doc = "0x33c - "]
    #[inline(always)]
    pub const fn denali_phy_207(&self) -> &DenaliPhy207 {
        &self.denali_phy_207
    }
    #[doc = "0x340 - "]
    #[inline(always)]
    pub const fn denali_phy_208(&self) -> &DenaliPhy208 {
        &self.denali_phy_208
    }
    #[doc = "0x344 - "]
    #[inline(always)]
    pub const fn denali_phy_209(&self) -> &DenaliPhy209 {
        &self.denali_phy_209
    }
    #[doc = "0x348 - "]
    #[inline(always)]
    pub const fn denali_phy_210(&self) -> &DenaliPhy210 {
        &self.denali_phy_210
    }
    #[doc = "0x34c - "]
    #[inline(always)]
    pub const fn denali_phy_211(&self) -> &DenaliPhy211 {
        &self.denali_phy_211
    }
    #[doc = "0x350 - "]
    #[inline(always)]
    pub const fn denali_phy_212(&self) -> &DenaliPhy212 {
        &self.denali_phy_212
    }
    #[doc = "0x354 - "]
    #[inline(always)]
    pub const fn denali_phy_213(&self) -> &DenaliPhy213 {
        &self.denali_phy_213
    }
    #[doc = "0x358 - "]
    #[inline(always)]
    pub const fn denali_phy_214(&self) -> &DenaliPhy214 {
        &self.denali_phy_214
    }
    #[doc = "0x35c - "]
    #[inline(always)]
    pub const fn denali_phy_215(&self) -> &DenaliPhy215 {
        &self.denali_phy_215
    }
    #[doc = "0x360 - "]
    #[inline(always)]
    pub const fn denali_phy_216(&self) -> &DenaliPhy216 {
        &self.denali_phy_216
    }
    #[doc = "0x364 - "]
    #[inline(always)]
    pub const fn denali_phy_217(&self) -> &DenaliPhy217 {
        &self.denali_phy_217
    }
    #[doc = "0x368 - "]
    #[inline(always)]
    pub const fn denali_phy_218(&self) -> &DenaliPhy218 {
        &self.denali_phy_218
    }
    #[doc = "0x400 - "]
    #[inline(always)]
    pub const fn denali_phy_256(&self) -> &DenaliPhy256 {
        &self.denali_phy_256
    }
    #[doc = "0x404 - "]
    #[inline(always)]
    pub const fn denali_phy_257(&self) -> &DenaliPhy257 {
        &self.denali_phy_257
    }
    #[doc = "0x408 - "]
    #[inline(always)]
    pub const fn denali_phy_258(&self) -> &DenaliPhy258 {
        &self.denali_phy_258
    }
    #[doc = "0x40c - "]
    #[inline(always)]
    pub const fn denali_phy_259(&self) -> &DenaliPhy259 {
        &self.denali_phy_259
    }
    #[doc = "0x410 - "]
    #[inline(always)]
    pub const fn denali_phy_260(&self) -> &DenaliPhy260 {
        &self.denali_phy_260
    }
    #[doc = "0x414 - "]
    #[inline(always)]
    pub const fn denali_phy_261(&self) -> &DenaliPhy261 {
        &self.denali_phy_261
    }
    #[doc = "0x418 - "]
    #[inline(always)]
    pub const fn denali_phy_262(&self) -> &DenaliPhy262 {
        &self.denali_phy_262
    }
    #[doc = "0x41c - "]
    #[inline(always)]
    pub const fn denali_phy_263(&self) -> &DenaliPhy263 {
        &self.denali_phy_263
    }
    #[doc = "0x420 - "]
    #[inline(always)]
    pub const fn denali_phy_264(&self) -> &DenaliPhy264 {
        &self.denali_phy_264
    }
    #[doc = "0x424 - "]
    #[inline(always)]
    pub const fn denali_phy_265(&self) -> &DenaliPhy265 {
        &self.denali_phy_265
    }
    #[doc = "0x428 - "]
    #[inline(always)]
    pub const fn denali_phy_266(&self) -> &DenaliPhy266 {
        &self.denali_phy_266
    }
    #[doc = "0x42c - "]
    #[inline(always)]
    pub const fn denali_phy_267(&self) -> &DenaliPhy267 {
        &self.denali_phy_267
    }
    #[doc = "0x430 - "]
    #[inline(always)]
    pub const fn denali_phy_268(&self) -> &DenaliPhy268 {
        &self.denali_phy_268
    }
    #[doc = "0x434 - "]
    #[inline(always)]
    pub const fn denali_phy_269(&self) -> &DenaliPhy269 {
        &self.denali_phy_269
    }
    #[doc = "0x438 - "]
    #[inline(always)]
    pub const fn denali_phy_270(&self) -> &DenaliPhy270 {
        &self.denali_phy_270
    }
    #[doc = "0x43c - "]
    #[inline(always)]
    pub const fn denali_phy_271(&self) -> &DenaliPhy271 {
        &self.denali_phy_271
    }
    #[doc = "0x440 - "]
    #[inline(always)]
    pub const fn denali_phy_272(&self) -> &DenaliPhy272 {
        &self.denali_phy_272
    }
    #[doc = "0x444 - "]
    #[inline(always)]
    pub const fn denali_phy_273(&self) -> &DenaliPhy273 {
        &self.denali_phy_273
    }
    #[doc = "0x448 - "]
    #[inline(always)]
    pub const fn denali_phy_274(&self) -> &DenaliPhy274 {
        &self.denali_phy_274
    }
    #[doc = "0x44c - "]
    #[inline(always)]
    pub const fn denali_phy_275(&self) -> &DenaliPhy275 {
        &self.denali_phy_275
    }
    #[doc = "0x450 - "]
    #[inline(always)]
    pub const fn denali_phy_276(&self) -> &DenaliPhy276 {
        &self.denali_phy_276
    }
    #[doc = "0x454 - "]
    #[inline(always)]
    pub const fn denali_phy_277(&self) -> &DenaliPhy277 {
        &self.denali_phy_277
    }
    #[doc = "0x458 - "]
    #[inline(always)]
    pub const fn denali_phy_278(&self) -> &DenaliPhy278 {
        &self.denali_phy_278
    }
    #[doc = "0x45c - "]
    #[inline(always)]
    pub const fn denali_phy_279(&self) -> &DenaliPhy279 {
        &self.denali_phy_279
    }
    #[doc = "0x460 - "]
    #[inline(always)]
    pub const fn denali_phy_280(&self) -> &DenaliPhy280 {
        &self.denali_phy_280
    }
    #[doc = "0x464 - "]
    #[inline(always)]
    pub const fn denali_phy_281(&self) -> &DenaliPhy281 {
        &self.denali_phy_281
    }
    #[doc = "0x468 - "]
    #[inline(always)]
    pub const fn denali_phy_282(&self) -> &DenaliPhy282 {
        &self.denali_phy_282
    }
    #[doc = "0x46c - "]
    #[inline(always)]
    pub const fn denali_phy_283(&self) -> &DenaliPhy283 {
        &self.denali_phy_283
    }
    #[doc = "0x470 - "]
    #[inline(always)]
    pub const fn denali_phy_284(&self) -> &DenaliPhy284 {
        &self.denali_phy_284
    }
    #[doc = "0x474 - "]
    #[inline(always)]
    pub const fn denali_phy_285(&self) -> &DenaliPhy285 {
        &self.denali_phy_285
    }
    #[doc = "0x478 - "]
    #[inline(always)]
    pub const fn denali_phy_286(&self) -> &DenaliPhy286 {
        &self.denali_phy_286
    }
    #[doc = "0x47c - "]
    #[inline(always)]
    pub const fn denali_phy_287(&self) -> &DenaliPhy287 {
        &self.denali_phy_287
    }
    #[doc = "0x480 - "]
    #[inline(always)]
    pub const fn denali_phy_288(&self) -> &DenaliPhy288 {
        &self.denali_phy_288
    }
    #[doc = "0x484 - "]
    #[inline(always)]
    pub const fn denali_phy_289(&self) -> &DenaliPhy289 {
        &self.denali_phy_289
    }
    #[doc = "0x488 - "]
    #[inline(always)]
    pub const fn denali_phy_290(&self) -> &DenaliPhy290 {
        &self.denali_phy_290
    }
    #[doc = "0x48c - "]
    #[inline(always)]
    pub const fn denali_phy_291(&self) -> &DenaliPhy291 {
        &self.denali_phy_291
    }
    #[doc = "0x490 - "]
    #[inline(always)]
    pub const fn denali_phy_292(&self) -> &DenaliPhy292 {
        &self.denali_phy_292
    }
    #[doc = "0x494 - "]
    #[inline(always)]
    pub const fn denali_phy_293(&self) -> &DenaliPhy293 {
        &self.denali_phy_293
    }
    #[doc = "0x498 - "]
    #[inline(always)]
    pub const fn denali_phy_294(&self) -> &DenaliPhy294 {
        &self.denali_phy_294
    }
    #[doc = "0x49c - "]
    #[inline(always)]
    pub const fn denali_phy_295(&self) -> &DenaliPhy295 {
        &self.denali_phy_295
    }
    #[doc = "0x4a0 - "]
    #[inline(always)]
    pub const fn denali_phy_296(&self) -> &DenaliPhy296 {
        &self.denali_phy_296
    }
    #[doc = "0x4a4 - "]
    #[inline(always)]
    pub const fn denali_phy_297(&self) -> &DenaliPhy297 {
        &self.denali_phy_297
    }
    #[doc = "0x4a8 - "]
    #[inline(always)]
    pub const fn denali_phy_298(&self) -> &DenaliPhy298 {
        &self.denali_phy_298
    }
    #[doc = "0x4ac - "]
    #[inline(always)]
    pub const fn denali_phy_299(&self) -> &DenaliPhy299 {
        &self.denali_phy_299
    }
    #[doc = "0x4b0 - "]
    #[inline(always)]
    pub const fn denali_phy_300(&self) -> &DenaliPhy300 {
        &self.denali_phy_300
    }
    #[doc = "0x4b4 - "]
    #[inline(always)]
    pub const fn denali_phy_301(&self) -> &DenaliPhy301 {
        &self.denali_phy_301
    }
    #[doc = "0x4b8 - "]
    #[inline(always)]
    pub const fn denali_phy_302(&self) -> &DenaliPhy302 {
        &self.denali_phy_302
    }
    #[doc = "0x4bc - "]
    #[inline(always)]
    pub const fn denali_phy_303(&self) -> &DenaliPhy303 {
        &self.denali_phy_303
    }
    #[doc = "0x4c0 - "]
    #[inline(always)]
    pub const fn denali_phy_304(&self) -> &DenaliPhy304 {
        &self.denali_phy_304
    }
    #[doc = "0x4c4 - "]
    #[inline(always)]
    pub const fn denali_phy_305(&self) -> &DenaliPhy305 {
        &self.denali_phy_305
    }
    #[doc = "0x4c8 - "]
    #[inline(always)]
    pub const fn denali_phy_306(&self) -> &DenaliPhy306 {
        &self.denali_phy_306
    }
    #[doc = "0x4cc - "]
    #[inline(always)]
    pub const fn denali_phy_307(&self) -> &DenaliPhy307 {
        &self.denali_phy_307
    }
    #[doc = "0x4d0 - "]
    #[inline(always)]
    pub const fn denali_phy_308(&self) -> &DenaliPhy308 {
        &self.denali_phy_308
    }
    #[doc = "0x4d4 - "]
    #[inline(always)]
    pub const fn denali_phy_309(&self) -> &DenaliPhy309 {
        &self.denali_phy_309
    }
    #[doc = "0x4d8 - "]
    #[inline(always)]
    pub const fn denali_phy_310(&self) -> &DenaliPhy310 {
        &self.denali_phy_310
    }
    #[doc = "0x4dc - "]
    #[inline(always)]
    pub const fn denali_phy_311(&self) -> &DenaliPhy311 {
        &self.denali_phy_311
    }
    #[doc = "0x4e0 - "]
    #[inline(always)]
    pub const fn denali_phy_312(&self) -> &DenaliPhy312 {
        &self.denali_phy_312
    }
    #[doc = "0x4e4 - "]
    #[inline(always)]
    pub const fn denali_phy_313(&self) -> &DenaliPhy313 {
        &self.denali_phy_313
    }
    #[doc = "0x4e8 - "]
    #[inline(always)]
    pub const fn denali_phy_314(&self) -> &DenaliPhy314 {
        &self.denali_phy_314
    }
    #[doc = "0x4ec - "]
    #[inline(always)]
    pub const fn denali_phy_315(&self) -> &DenaliPhy315 {
        &self.denali_phy_315
    }
    #[doc = "0x4f0 - "]
    #[inline(always)]
    pub const fn denali_phy_316(&self) -> &DenaliPhy316 {
        &self.denali_phy_316
    }
    #[doc = "0x4f4 - "]
    #[inline(always)]
    pub const fn denali_phy_317(&self) -> &DenaliPhy317 {
        &self.denali_phy_317
    }
    #[doc = "0x4f8 - "]
    #[inline(always)]
    pub const fn denali_phy_318(&self) -> &DenaliPhy318 {
        &self.denali_phy_318
    }
    #[doc = "0x4fc - "]
    #[inline(always)]
    pub const fn denali_phy_319(&self) -> &DenaliPhy319 {
        &self.denali_phy_319
    }
    #[doc = "0x500 - "]
    #[inline(always)]
    pub const fn denali_phy_320(&self) -> &DenaliPhy320 {
        &self.denali_phy_320
    }
    #[doc = "0x504 - "]
    #[inline(always)]
    pub const fn denali_phy_321(&self) -> &DenaliPhy321 {
        &self.denali_phy_321
    }
    #[doc = "0x508 - "]
    #[inline(always)]
    pub const fn denali_phy_322(&self) -> &DenaliPhy322 {
        &self.denali_phy_322
    }
    #[doc = "0x50c - "]
    #[inline(always)]
    pub const fn denali_phy_323(&self) -> &DenaliPhy323 {
        &self.denali_phy_323
    }
    #[doc = "0x510 - "]
    #[inline(always)]
    pub const fn denali_phy_324(&self) -> &DenaliPhy324 {
        &self.denali_phy_324
    }
    #[doc = "0x514 - "]
    #[inline(always)]
    pub const fn denali_phy_325(&self) -> &DenaliPhy325 {
        &self.denali_phy_325
    }
    #[doc = "0x518 - "]
    #[inline(always)]
    pub const fn denali_phy_326(&self) -> &DenaliPhy326 {
        &self.denali_phy_326
    }
    #[doc = "0x51c - "]
    #[inline(always)]
    pub const fn denali_phy_327(&self) -> &DenaliPhy327 {
        &self.denali_phy_327
    }
    #[doc = "0x520 - "]
    #[inline(always)]
    pub const fn denali_phy_328(&self) -> &DenaliPhy328 {
        &self.denali_phy_328
    }
    #[doc = "0x524 - "]
    #[inline(always)]
    pub const fn denali_phy_329(&self) -> &DenaliPhy329 {
        &self.denali_phy_329
    }
    #[doc = "0x528 - "]
    #[inline(always)]
    pub const fn denali_phy_330(&self) -> &DenaliPhy330 {
        &self.denali_phy_330
    }
    #[doc = "0x52c - "]
    #[inline(always)]
    pub const fn denali_phy_331(&self) -> &DenaliPhy331 {
        &self.denali_phy_331
    }
    #[doc = "0x530 - "]
    #[inline(always)]
    pub const fn denali_phy_332(&self) -> &DenaliPhy332 {
        &self.denali_phy_332
    }
    #[doc = "0x534 - "]
    #[inline(always)]
    pub const fn denali_phy_333(&self) -> &DenaliPhy333 {
        &self.denali_phy_333
    }
    #[doc = "0x538 - "]
    #[inline(always)]
    pub const fn denali_phy_334(&self) -> &DenaliPhy334 {
        &self.denali_phy_334
    }
    #[doc = "0x53c - "]
    #[inline(always)]
    pub const fn denali_phy_335(&self) -> &DenaliPhy335 {
        &self.denali_phy_335
    }
    #[doc = "0x540 - "]
    #[inline(always)]
    pub const fn denali_phy_336(&self) -> &DenaliPhy336 {
        &self.denali_phy_336
    }
    #[doc = "0x544 - "]
    #[inline(always)]
    pub const fn denali_phy_337(&self) -> &DenaliPhy337 {
        &self.denali_phy_337
    }
    #[doc = "0x548 - "]
    #[inline(always)]
    pub const fn denali_phy_338(&self) -> &DenaliPhy338 {
        &self.denali_phy_338
    }
    #[doc = "0x54c - "]
    #[inline(always)]
    pub const fn denali_phy_339(&self) -> &DenaliPhy339 {
        &self.denali_phy_339
    }
    #[doc = "0x550 - "]
    #[inline(always)]
    pub const fn denali_phy_340(&self) -> &DenaliPhy340 {
        &self.denali_phy_340
    }
    #[doc = "0x554 - "]
    #[inline(always)]
    pub const fn denali_phy_341(&self) -> &DenaliPhy341 {
        &self.denali_phy_341
    }
    #[doc = "0x558 - "]
    #[inline(always)]
    pub const fn denali_phy_342(&self) -> &DenaliPhy342 {
        &self.denali_phy_342
    }
    #[doc = "0x55c - "]
    #[inline(always)]
    pub const fn denali_phy_343(&self) -> &DenaliPhy343 {
        &self.denali_phy_343
    }
    #[doc = "0x560 - "]
    #[inline(always)]
    pub const fn denali_phy_344(&self) -> &DenaliPhy344 {
        &self.denali_phy_344
    }
    #[doc = "0x564 - "]
    #[inline(always)]
    pub const fn denali_phy_345(&self) -> &DenaliPhy345 {
        &self.denali_phy_345
    }
    #[doc = "0x568 - "]
    #[inline(always)]
    pub const fn denali_phy_346(&self) -> &DenaliPhy346 {
        &self.denali_phy_346
    }
    #[doc = "0x600 - "]
    #[inline(always)]
    pub const fn denali_phy_384(&self) -> &DenaliPhy384 {
        &self.denali_phy_384
    }
    #[doc = "0x604 - "]
    #[inline(always)]
    pub const fn denali_phy_385(&self) -> &DenaliPhy385 {
        &self.denali_phy_385
    }
    #[doc = "0x608 - "]
    #[inline(always)]
    pub const fn denali_phy_386(&self) -> &DenaliPhy386 {
        &self.denali_phy_386
    }
    #[doc = "0x60c - "]
    #[inline(always)]
    pub const fn denali_phy_387(&self) -> &DenaliPhy387 {
        &self.denali_phy_387
    }
    #[doc = "0x610 - "]
    #[inline(always)]
    pub const fn denali_phy_388(&self) -> &DenaliPhy388 {
        &self.denali_phy_388
    }
    #[doc = "0x614 - "]
    #[inline(always)]
    pub const fn denali_phy_389(&self) -> &DenaliPhy389 {
        &self.denali_phy_389
    }
    #[doc = "0x618 - "]
    #[inline(always)]
    pub const fn denali_phy_390(&self) -> &DenaliPhy390 {
        &self.denali_phy_390
    }
    #[doc = "0x61c - "]
    #[inline(always)]
    pub const fn denali_phy_391(&self) -> &DenaliPhy391 {
        &self.denali_phy_391
    }
    #[doc = "0x620 - "]
    #[inline(always)]
    pub const fn denali_phy_392(&self) -> &DenaliPhy392 {
        &self.denali_phy_392
    }
    #[doc = "0x624 - "]
    #[inline(always)]
    pub const fn denali_phy_393(&self) -> &DenaliPhy393 {
        &self.denali_phy_393
    }
    #[doc = "0x628 - "]
    #[inline(always)]
    pub const fn denali_phy_394(&self) -> &DenaliPhy394 {
        &self.denali_phy_394
    }
    #[doc = "0x62c - "]
    #[inline(always)]
    pub const fn denali_phy_395(&self) -> &DenaliPhy395 {
        &self.denali_phy_395
    }
    #[doc = "0x630 - "]
    #[inline(always)]
    pub const fn denali_phy_396(&self) -> &DenaliPhy396 {
        &self.denali_phy_396
    }
    #[doc = "0x634 - "]
    #[inline(always)]
    pub const fn denali_phy_397(&self) -> &DenaliPhy397 {
        &self.denali_phy_397
    }
    #[doc = "0x638 - "]
    #[inline(always)]
    pub const fn denali_phy_398(&self) -> &DenaliPhy398 {
        &self.denali_phy_398
    }
    #[doc = "0x63c - "]
    #[inline(always)]
    pub const fn denali_phy_399(&self) -> &DenaliPhy399 {
        &self.denali_phy_399
    }
    #[doc = "0x640 - "]
    #[inline(always)]
    pub const fn denali_phy_400(&self) -> &DenaliPhy400 {
        &self.denali_phy_400
    }
    #[doc = "0x644 - "]
    #[inline(always)]
    pub const fn denali_phy_401(&self) -> &DenaliPhy401 {
        &self.denali_phy_401
    }
    #[doc = "0x648 - "]
    #[inline(always)]
    pub const fn denali_phy_402(&self) -> &DenaliPhy402 {
        &self.denali_phy_402
    }
    #[doc = "0x64c - "]
    #[inline(always)]
    pub const fn denali_phy_403(&self) -> &DenaliPhy403 {
        &self.denali_phy_403
    }
    #[doc = "0x650 - "]
    #[inline(always)]
    pub const fn denali_phy_404(&self) -> &DenaliPhy404 {
        &self.denali_phy_404
    }
    #[doc = "0x654 - "]
    #[inline(always)]
    pub const fn denali_phy_405(&self) -> &DenaliPhy405 {
        &self.denali_phy_405
    }
    #[doc = "0x658 - "]
    #[inline(always)]
    pub const fn denali_phy_406(&self) -> &DenaliPhy406 {
        &self.denali_phy_406
    }
    #[doc = "0x65c - "]
    #[inline(always)]
    pub const fn denali_phy_407(&self) -> &DenaliPhy407 {
        &self.denali_phy_407
    }
    #[doc = "0x660 - "]
    #[inline(always)]
    pub const fn denali_phy_408(&self) -> &DenaliPhy408 {
        &self.denali_phy_408
    }
    #[doc = "0x664 - "]
    #[inline(always)]
    pub const fn denali_phy_409(&self) -> &DenaliPhy409 {
        &self.denali_phy_409
    }
    #[doc = "0x668 - "]
    #[inline(always)]
    pub const fn denali_phy_410(&self) -> &DenaliPhy410 {
        &self.denali_phy_410
    }
    #[doc = "0x66c - "]
    #[inline(always)]
    pub const fn denali_phy_411(&self) -> &DenaliPhy411 {
        &self.denali_phy_411
    }
    #[doc = "0x670 - "]
    #[inline(always)]
    pub const fn denali_phy_412(&self) -> &DenaliPhy412 {
        &self.denali_phy_412
    }
    #[doc = "0x674 - "]
    #[inline(always)]
    pub const fn denali_phy_413(&self) -> &DenaliPhy413 {
        &self.denali_phy_413
    }
    #[doc = "0x678 - "]
    #[inline(always)]
    pub const fn denali_phy_414(&self) -> &DenaliPhy414 {
        &self.denali_phy_414
    }
    #[doc = "0x67c - "]
    #[inline(always)]
    pub const fn denali_phy_415(&self) -> &DenaliPhy415 {
        &self.denali_phy_415
    }
    #[doc = "0x680 - "]
    #[inline(always)]
    pub const fn denali_phy_416(&self) -> &DenaliPhy416 {
        &self.denali_phy_416
    }
    #[doc = "0x684 - "]
    #[inline(always)]
    pub const fn denali_phy_417(&self) -> &DenaliPhy417 {
        &self.denali_phy_417
    }
    #[doc = "0x688 - "]
    #[inline(always)]
    pub const fn denali_phy_418(&self) -> &DenaliPhy418 {
        &self.denali_phy_418
    }
    #[doc = "0x68c - "]
    #[inline(always)]
    pub const fn denali_phy_419(&self) -> &DenaliPhy419 {
        &self.denali_phy_419
    }
    #[doc = "0x690 - "]
    #[inline(always)]
    pub const fn denali_phy_420(&self) -> &DenaliPhy420 {
        &self.denali_phy_420
    }
    #[doc = "0x694 - "]
    #[inline(always)]
    pub const fn denali_phy_421(&self) -> &DenaliPhy421 {
        &self.denali_phy_421
    }
    #[doc = "0x698 - "]
    #[inline(always)]
    pub const fn denali_phy_422(&self) -> &DenaliPhy422 {
        &self.denali_phy_422
    }
    #[doc = "0x69c - "]
    #[inline(always)]
    pub const fn denali_phy_423(&self) -> &DenaliPhy423 {
        &self.denali_phy_423
    }
    #[doc = "0x6a0 - "]
    #[inline(always)]
    pub const fn denali_phy_424(&self) -> &DenaliPhy424 {
        &self.denali_phy_424
    }
    #[doc = "0x6a4 - "]
    #[inline(always)]
    pub const fn denali_phy_425(&self) -> &DenaliPhy425 {
        &self.denali_phy_425
    }
    #[doc = "0x6a8 - "]
    #[inline(always)]
    pub const fn denali_phy_426(&self) -> &DenaliPhy426 {
        &self.denali_phy_426
    }
    #[doc = "0x6ac - "]
    #[inline(always)]
    pub const fn denali_phy_427(&self) -> &DenaliPhy427 {
        &self.denali_phy_427
    }
    #[doc = "0x6b0 - "]
    #[inline(always)]
    pub const fn denali_phy_428(&self) -> &DenaliPhy428 {
        &self.denali_phy_428
    }
    #[doc = "0x6b4 - "]
    #[inline(always)]
    pub const fn denali_phy_429(&self) -> &DenaliPhy429 {
        &self.denali_phy_429
    }
    #[doc = "0x6b8 - "]
    #[inline(always)]
    pub const fn denali_phy_430(&self) -> &DenaliPhy430 {
        &self.denali_phy_430
    }
    #[doc = "0x6bc - "]
    #[inline(always)]
    pub const fn denali_phy_431(&self) -> &DenaliPhy431 {
        &self.denali_phy_431
    }
    #[doc = "0x6c0 - "]
    #[inline(always)]
    pub const fn denali_phy_432(&self) -> &DenaliPhy432 {
        &self.denali_phy_432
    }
    #[doc = "0x6c4 - "]
    #[inline(always)]
    pub const fn denali_phy_433(&self) -> &DenaliPhy433 {
        &self.denali_phy_433
    }
    #[doc = "0x6c8 - "]
    #[inline(always)]
    pub const fn denali_phy_434(&self) -> &DenaliPhy434 {
        &self.denali_phy_434
    }
    #[doc = "0x6cc - "]
    #[inline(always)]
    pub const fn denali_phy_435(&self) -> &DenaliPhy435 {
        &self.denali_phy_435
    }
    #[doc = "0x6d0 - "]
    #[inline(always)]
    pub const fn denali_phy_436(&self) -> &DenaliPhy436 {
        &self.denali_phy_436
    }
    #[doc = "0x6d4 - "]
    #[inline(always)]
    pub const fn denali_phy_437(&self) -> &DenaliPhy437 {
        &self.denali_phy_437
    }
    #[doc = "0x6d8 - "]
    #[inline(always)]
    pub const fn denali_phy_438(&self) -> &DenaliPhy438 {
        &self.denali_phy_438
    }
    #[doc = "0x6dc - "]
    #[inline(always)]
    pub const fn denali_phy_439(&self) -> &DenaliPhy439 {
        &self.denali_phy_439
    }
    #[doc = "0x6e0 - "]
    #[inline(always)]
    pub const fn denali_phy_440(&self) -> &DenaliPhy440 {
        &self.denali_phy_440
    }
    #[doc = "0x6e4 - "]
    #[inline(always)]
    pub const fn denali_phy_441(&self) -> &DenaliPhy441 {
        &self.denali_phy_441
    }
    #[doc = "0x6e8 - "]
    #[inline(always)]
    pub const fn denali_phy_442(&self) -> &DenaliPhy442 {
        &self.denali_phy_442
    }
    #[doc = "0x6ec - "]
    #[inline(always)]
    pub const fn denali_phy_443(&self) -> &DenaliPhy443 {
        &self.denali_phy_443
    }
    #[doc = "0x6f0 - "]
    #[inline(always)]
    pub const fn denali_phy_444(&self) -> &DenaliPhy444 {
        &self.denali_phy_444
    }
    #[doc = "0x6f4 - "]
    #[inline(always)]
    pub const fn denali_phy_445(&self) -> &DenaliPhy445 {
        &self.denali_phy_445
    }
    #[doc = "0x6f8 - "]
    #[inline(always)]
    pub const fn denali_phy_446(&self) -> &DenaliPhy446 {
        &self.denali_phy_446
    }
    #[doc = "0x6fc - "]
    #[inline(always)]
    pub const fn denali_phy_447(&self) -> &DenaliPhy447 {
        &self.denali_phy_447
    }
    #[doc = "0x700 - "]
    #[inline(always)]
    pub const fn denali_phy_448(&self) -> &DenaliPhy448 {
        &self.denali_phy_448
    }
    #[doc = "0x704 - "]
    #[inline(always)]
    pub const fn denali_phy_449(&self) -> &DenaliPhy449 {
        &self.denali_phy_449
    }
    #[doc = "0x708 - "]
    #[inline(always)]
    pub const fn denali_phy_450(&self) -> &DenaliPhy450 {
        &self.denali_phy_450
    }
    #[doc = "0x70c - "]
    #[inline(always)]
    pub const fn denali_phy_451(&self) -> &DenaliPhy451 {
        &self.denali_phy_451
    }
    #[doc = "0x710 - "]
    #[inline(always)]
    pub const fn denali_phy_452(&self) -> &DenaliPhy452 {
        &self.denali_phy_452
    }
    #[doc = "0x714 - "]
    #[inline(always)]
    pub const fn denali_phy_453(&self) -> &DenaliPhy453 {
        &self.denali_phy_453
    }
    #[doc = "0x718 - "]
    #[inline(always)]
    pub const fn denali_phy_454(&self) -> &DenaliPhy454 {
        &self.denali_phy_454
    }
    #[doc = "0x71c - "]
    #[inline(always)]
    pub const fn denali_phy_455(&self) -> &DenaliPhy455 {
        &self.denali_phy_455
    }
    #[doc = "0x720 - "]
    #[inline(always)]
    pub const fn denali_phy_456(&self) -> &DenaliPhy456 {
        &self.denali_phy_456
    }
    #[doc = "0x724 - "]
    #[inline(always)]
    pub const fn denali_phy_457(&self) -> &DenaliPhy457 {
        &self.denali_phy_457
    }
    #[doc = "0x728 - "]
    #[inline(always)]
    pub const fn denali_phy_458(&self) -> &DenaliPhy458 {
        &self.denali_phy_458
    }
    #[doc = "0x72c - "]
    #[inline(always)]
    pub const fn denali_phy_459(&self) -> &DenaliPhy459 {
        &self.denali_phy_459
    }
    #[doc = "0x730 - "]
    #[inline(always)]
    pub const fn denali_phy_460(&self) -> &DenaliPhy460 {
        &self.denali_phy_460
    }
    #[doc = "0x734 - "]
    #[inline(always)]
    pub const fn denali_phy_461(&self) -> &DenaliPhy461 {
        &self.denali_phy_461
    }
    #[doc = "0x738 - "]
    #[inline(always)]
    pub const fn denali_phy_462(&self) -> &DenaliPhy462 {
        &self.denali_phy_462
    }
    #[doc = "0x73c - "]
    #[inline(always)]
    pub const fn denali_phy_463(&self) -> &DenaliPhy463 {
        &self.denali_phy_463
    }
    #[doc = "0x740 - "]
    #[inline(always)]
    pub const fn denali_phy_464(&self) -> &DenaliPhy464 {
        &self.denali_phy_464
    }
    #[doc = "0x744 - "]
    #[inline(always)]
    pub const fn denali_phy_465(&self) -> &DenaliPhy465 {
        &self.denali_phy_465
    }
    #[doc = "0x748 - "]
    #[inline(always)]
    pub const fn denali_phy_466(&self) -> &DenaliPhy466 {
        &self.denali_phy_466
    }
    #[doc = "0x74c - "]
    #[inline(always)]
    pub const fn denali_phy_467(&self) -> &DenaliPhy467 {
        &self.denali_phy_467
    }
    #[doc = "0x750 - "]
    #[inline(always)]
    pub const fn denali_phy_468(&self) -> &DenaliPhy468 {
        &self.denali_phy_468
    }
    #[doc = "0x754 - "]
    #[inline(always)]
    pub const fn denali_phy_469(&self) -> &DenaliPhy469 {
        &self.denali_phy_469
    }
    #[doc = "0x758 - "]
    #[inline(always)]
    pub const fn denali_phy_470(&self) -> &DenaliPhy470 {
        &self.denali_phy_470
    }
    #[doc = "0x75c - "]
    #[inline(always)]
    pub const fn denali_phy_471(&self) -> &DenaliPhy471 {
        &self.denali_phy_471
    }
    #[doc = "0x760 - "]
    #[inline(always)]
    pub const fn denali_phy_472(&self) -> &DenaliPhy472 {
        &self.denali_phy_472
    }
    #[doc = "0x764 - "]
    #[inline(always)]
    pub const fn denali_phy_473(&self) -> &DenaliPhy473 {
        &self.denali_phy_473
    }
    #[doc = "0x768 - "]
    #[inline(always)]
    pub const fn denali_phy_474(&self) -> &DenaliPhy474 {
        &self.denali_phy_474
    }
    #[doc = "0x800 - "]
    #[inline(always)]
    pub const fn denali_phy_512(&self) -> &DenaliPhy512 {
        &self.denali_phy_512
    }
    #[doc = "0x804 - "]
    #[inline(always)]
    pub const fn denali_phy_513(&self) -> &DenaliPhy513 {
        &self.denali_phy_513
    }
    #[doc = "0x808 - "]
    #[inline(always)]
    pub const fn denali_phy_514(&self) -> &DenaliPhy514 {
        &self.denali_phy_514
    }
    #[doc = "0x80c - "]
    #[inline(always)]
    pub const fn denali_phy_515(&self) -> &DenaliPhy515 {
        &self.denali_phy_515
    }
    #[doc = "0x810 - "]
    #[inline(always)]
    pub const fn denali_phy_516(&self) -> &DenaliPhy516 {
        &self.denali_phy_516
    }
    #[doc = "0x814 - "]
    #[inline(always)]
    pub const fn denali_phy_517(&self) -> &DenaliPhy517 {
        &self.denali_phy_517
    }
    #[doc = "0x818 - "]
    #[inline(always)]
    pub const fn denali_phy_518(&self) -> &DenaliPhy518 {
        &self.denali_phy_518
    }
    #[doc = "0x81c - "]
    #[inline(always)]
    pub const fn denali_phy_519(&self) -> &DenaliPhy519 {
        &self.denali_phy_519
    }
    #[doc = "0x820 - "]
    #[inline(always)]
    pub const fn denali_phy_520(&self) -> &DenaliPhy520 {
        &self.denali_phy_520
    }
    #[doc = "0x824 - "]
    #[inline(always)]
    pub const fn denali_phy_521(&self) -> &DenaliPhy521 {
        &self.denali_phy_521
    }
    #[doc = "0x828 - "]
    #[inline(always)]
    pub const fn denali_phy_522(&self) -> &DenaliPhy522 {
        &self.denali_phy_522
    }
    #[doc = "0x82c - "]
    #[inline(always)]
    pub const fn denali_phy_523(&self) -> &DenaliPhy523 {
        &self.denali_phy_523
    }
    #[doc = "0x830 - "]
    #[inline(always)]
    pub const fn denali_phy_524(&self) -> &DenaliPhy524 {
        &self.denali_phy_524
    }
    #[doc = "0x834 - "]
    #[inline(always)]
    pub const fn denali_phy_525(&self) -> &DenaliPhy525 {
        &self.denali_phy_525
    }
    #[doc = "0x838 - "]
    #[inline(always)]
    pub const fn denali_phy_526(&self) -> &DenaliPhy526 {
        &self.denali_phy_526
    }
    #[doc = "0x83c - "]
    #[inline(always)]
    pub const fn denali_phy_527(&self) -> &DenaliPhy527 {
        &self.denali_phy_527
    }
    #[doc = "0x840 - "]
    #[inline(always)]
    pub const fn denali_phy_528(&self) -> &DenaliPhy528 {
        &self.denali_phy_528
    }
    #[doc = "0x844 - "]
    #[inline(always)]
    pub const fn denali_phy_529(&self) -> &DenaliPhy529 {
        &self.denali_phy_529
    }
    #[doc = "0x848 - "]
    #[inline(always)]
    pub const fn denali_phy_530(&self) -> &DenaliPhy530 {
        &self.denali_phy_530
    }
    #[doc = "0x84c - "]
    #[inline(always)]
    pub const fn denali_phy_531(&self) -> &DenaliPhy531 {
        &self.denali_phy_531
    }
    #[doc = "0x850 - "]
    #[inline(always)]
    pub const fn denali_phy_532(&self) -> &DenaliPhy532 {
        &self.denali_phy_532
    }
    #[doc = "0x854 - "]
    #[inline(always)]
    pub const fn denali_phy_533(&self) -> &DenaliPhy533 {
        &self.denali_phy_533
    }
    #[doc = "0x858 - "]
    #[inline(always)]
    pub const fn denali_phy_534(&self) -> &DenaliPhy534 {
        &self.denali_phy_534
    }
    #[doc = "0x85c - "]
    #[inline(always)]
    pub const fn denali_phy_535(&self) -> &DenaliPhy535 {
        &self.denali_phy_535
    }
    #[doc = "0x860 - "]
    #[inline(always)]
    pub const fn denali_phy_536(&self) -> &DenaliPhy536 {
        &self.denali_phy_536
    }
    #[doc = "0x864 - "]
    #[inline(always)]
    pub const fn denali_phy_537(&self) -> &DenaliPhy537 {
        &self.denali_phy_537
    }
    #[doc = "0x868 - "]
    #[inline(always)]
    pub const fn denali_phy_538(&self) -> &DenaliPhy538 {
        &self.denali_phy_538
    }
    #[doc = "0x86c - "]
    #[inline(always)]
    pub const fn denali_phy_539(&self) -> &DenaliPhy539 {
        &self.denali_phy_539
    }
    #[doc = "0x870 - "]
    #[inline(always)]
    pub const fn denali_phy_540(&self) -> &DenaliPhy540 {
        &self.denali_phy_540
    }
    #[doc = "0x874 - "]
    #[inline(always)]
    pub const fn denali_phy_541(&self) -> &DenaliPhy541 {
        &self.denali_phy_541
    }
    #[doc = "0x878 - "]
    #[inline(always)]
    pub const fn denali_phy_542(&self) -> &DenaliPhy542 {
        &self.denali_phy_542
    }
    #[doc = "0x87c - "]
    #[inline(always)]
    pub const fn denali_phy_543(&self) -> &DenaliPhy543 {
        &self.denali_phy_543
    }
    #[doc = "0x880 - "]
    #[inline(always)]
    pub const fn denali_phy_544(&self) -> &DenaliPhy544 {
        &self.denali_phy_544
    }
    #[doc = "0x884 - "]
    #[inline(always)]
    pub const fn denali_phy_545(&self) -> &DenaliPhy545 {
        &self.denali_phy_545
    }
    #[doc = "0x888 - "]
    #[inline(always)]
    pub const fn denali_phy_546(&self) -> &DenaliPhy546 {
        &self.denali_phy_546
    }
    #[doc = "0x88c - "]
    #[inline(always)]
    pub const fn denali_phy_547(&self) -> &DenaliPhy547 {
        &self.denali_phy_547
    }
    #[doc = "0x890 - "]
    #[inline(always)]
    pub const fn denali_phy_548(&self) -> &DenaliPhy548 {
        &self.denali_phy_548
    }
    #[doc = "0x894 - "]
    #[inline(always)]
    pub const fn denali_phy_549(&self) -> &DenaliPhy549 {
        &self.denali_phy_549
    }
    #[doc = "0xa00 - "]
    #[inline(always)]
    pub const fn denali_phy_640(&self) -> &DenaliPhy640 {
        &self.denali_phy_640
    }
    #[doc = "0xa04 - "]
    #[inline(always)]
    pub const fn denali_phy_641(&self) -> &DenaliPhy641 {
        &self.denali_phy_641
    }
    #[doc = "0xa08 - "]
    #[inline(always)]
    pub const fn denali_phy_642(&self) -> &DenaliPhy642 {
        &self.denali_phy_642
    }
    #[doc = "0xa0c - "]
    #[inline(always)]
    pub const fn denali_phy_643(&self) -> &DenaliPhy643 {
        &self.denali_phy_643
    }
    #[doc = "0xa10 - "]
    #[inline(always)]
    pub const fn denali_phy_644(&self) -> &DenaliPhy644 {
        &self.denali_phy_644
    }
    #[doc = "0xa14 - "]
    #[inline(always)]
    pub const fn denali_phy_645(&self) -> &DenaliPhy645 {
        &self.denali_phy_645
    }
    #[doc = "0xa18 - "]
    #[inline(always)]
    pub const fn denali_phy_646(&self) -> &DenaliPhy646 {
        &self.denali_phy_646
    }
    #[doc = "0xa1c - "]
    #[inline(always)]
    pub const fn denali_phy_647(&self) -> &DenaliPhy647 {
        &self.denali_phy_647
    }
    #[doc = "0xa20 - "]
    #[inline(always)]
    pub const fn denali_phy_648(&self) -> &DenaliPhy648 {
        &self.denali_phy_648
    }
    #[doc = "0xa24 - "]
    #[inline(always)]
    pub const fn denali_phy_649(&self) -> &DenaliPhy649 {
        &self.denali_phy_649
    }
    #[doc = "0xa28 - "]
    #[inline(always)]
    pub const fn denali_phy_650(&self) -> &DenaliPhy650 {
        &self.denali_phy_650
    }
    #[doc = "0xa2c - "]
    #[inline(always)]
    pub const fn denali_phy_651(&self) -> &DenaliPhy651 {
        &self.denali_phy_651
    }
    #[doc = "0xa30 - "]
    #[inline(always)]
    pub const fn denali_phy_652(&self) -> &DenaliPhy652 {
        &self.denali_phy_652
    }
    #[doc = "0xa34 - "]
    #[inline(always)]
    pub const fn denali_phy_653(&self) -> &DenaliPhy653 {
        &self.denali_phy_653
    }
    #[doc = "0xa38 - "]
    #[inline(always)]
    pub const fn denali_phy_654(&self) -> &DenaliPhy654 {
        &self.denali_phy_654
    }
    #[doc = "0xa3c - "]
    #[inline(always)]
    pub const fn denali_phy_655(&self) -> &DenaliPhy655 {
        &self.denali_phy_655
    }
    #[doc = "0xa40 - "]
    #[inline(always)]
    pub const fn denali_phy_656(&self) -> &DenaliPhy656 {
        &self.denali_phy_656
    }
    #[doc = "0xa44 - "]
    #[inline(always)]
    pub const fn denali_phy_657(&self) -> &DenaliPhy657 {
        &self.denali_phy_657
    }
    #[doc = "0xa48 - "]
    #[inline(always)]
    pub const fn denali_phy_658(&self) -> &DenaliPhy658 {
        &self.denali_phy_658
    }
    #[doc = "0xa4c - "]
    #[inline(always)]
    pub const fn denali_phy_659(&self) -> &DenaliPhy659 {
        &self.denali_phy_659
    }
    #[doc = "0xa50 - "]
    #[inline(always)]
    pub const fn denali_phy_660(&self) -> &DenaliPhy660 {
        &self.denali_phy_660
    }
    #[doc = "0xa54 - "]
    #[inline(always)]
    pub const fn denali_phy_661(&self) -> &DenaliPhy661 {
        &self.denali_phy_661
    }
    #[doc = "0xa58 - "]
    #[inline(always)]
    pub const fn denali_phy_662(&self) -> &DenaliPhy662 {
        &self.denali_phy_662
    }
    #[doc = "0xa5c - "]
    #[inline(always)]
    pub const fn denali_phy_663(&self) -> &DenaliPhy663 {
        &self.denali_phy_663
    }
    #[doc = "0xa60 - "]
    #[inline(always)]
    pub const fn denali_phy_664(&self) -> &DenaliPhy664 {
        &self.denali_phy_664
    }
    #[doc = "0xa64 - "]
    #[inline(always)]
    pub const fn denali_phy_665(&self) -> &DenaliPhy665 {
        &self.denali_phy_665
    }
    #[doc = "0xa68 - "]
    #[inline(always)]
    pub const fn denali_phy_666(&self) -> &DenaliPhy666 {
        &self.denali_phy_666
    }
    #[doc = "0xa6c - "]
    #[inline(always)]
    pub const fn denali_phy_667(&self) -> &DenaliPhy667 {
        &self.denali_phy_667
    }
    #[doc = "0xa70 - "]
    #[inline(always)]
    pub const fn denali_phy_668(&self) -> &DenaliPhy668 {
        &self.denali_phy_668
    }
    #[doc = "0xa74 - "]
    #[inline(always)]
    pub const fn denali_phy_669(&self) -> &DenaliPhy669 {
        &self.denali_phy_669
    }
    #[doc = "0xa78 - "]
    #[inline(always)]
    pub const fn denali_phy_670(&self) -> &DenaliPhy670 {
        &self.denali_phy_670
    }
    #[doc = "0xa7c - "]
    #[inline(always)]
    pub const fn denali_phy_671(&self) -> &DenaliPhy671 {
        &self.denali_phy_671
    }
    #[doc = "0xa80 - "]
    #[inline(always)]
    pub const fn denali_phy_672(&self) -> &DenaliPhy672 {
        &self.denali_phy_672
    }
    #[doc = "0xa84 - "]
    #[inline(always)]
    pub const fn denali_phy_673(&self) -> &DenaliPhy673 {
        &self.denali_phy_673
    }
    #[doc = "0xa88 - "]
    #[inline(always)]
    pub const fn denali_phy_674(&self) -> &DenaliPhy674 {
        &self.denali_phy_674
    }
    #[doc = "0xa8c - "]
    #[inline(always)]
    pub const fn denali_phy_675(&self) -> &DenaliPhy675 {
        &self.denali_phy_675
    }
    #[doc = "0xa90 - "]
    #[inline(always)]
    pub const fn denali_phy_676(&self) -> &DenaliPhy676 {
        &self.denali_phy_676
    }
    #[doc = "0xa94 - "]
    #[inline(always)]
    pub const fn denali_phy_677(&self) -> &DenaliPhy677 {
        &self.denali_phy_677
    }
    #[doc = "0xc00 - "]
    #[inline(always)]
    pub const fn denali_phy_768(&self) -> &DenaliPhy768 {
        &self.denali_phy_768
    }
    #[doc = "0xc04 - "]
    #[inline(always)]
    pub const fn denali_phy_769(&self) -> &DenaliPhy769 {
        &self.denali_phy_769
    }
    #[doc = "0xc08 - "]
    #[inline(always)]
    pub const fn denali_phy_770(&self) -> &DenaliPhy770 {
        &self.denali_phy_770
    }
    #[doc = "0xc0c - "]
    #[inline(always)]
    pub const fn denali_phy_771(&self) -> &DenaliPhy771 {
        &self.denali_phy_771
    }
    #[doc = "0xc10 - "]
    #[inline(always)]
    pub const fn denali_phy_772(&self) -> &DenaliPhy772 {
        &self.denali_phy_772
    }
    #[doc = "0xc14 - "]
    #[inline(always)]
    pub const fn denali_phy_773(&self) -> &DenaliPhy773 {
        &self.denali_phy_773
    }
    #[doc = "0xc18 - "]
    #[inline(always)]
    pub const fn denali_phy_774(&self) -> &DenaliPhy774 {
        &self.denali_phy_774
    }
    #[doc = "0xc1c - "]
    #[inline(always)]
    pub const fn denali_phy_775(&self) -> &DenaliPhy775 {
        &self.denali_phy_775
    }
    #[doc = "0xc20 - "]
    #[inline(always)]
    pub const fn denali_phy_776(&self) -> &DenaliPhy776 {
        &self.denali_phy_776
    }
    #[doc = "0xc24 - "]
    #[inline(always)]
    pub const fn denali_phy_777(&self) -> &DenaliPhy777 {
        &self.denali_phy_777
    }
    #[doc = "0xc28 - "]
    #[inline(always)]
    pub const fn denali_phy_778(&self) -> &DenaliPhy778 {
        &self.denali_phy_778
    }
    #[doc = "0xc2c - "]
    #[inline(always)]
    pub const fn denali_phy_779(&self) -> &DenaliPhy779 {
        &self.denali_phy_779
    }
    #[doc = "0xc30 - "]
    #[inline(always)]
    pub const fn denali_phy_780(&self) -> &DenaliPhy780 {
        &self.denali_phy_780
    }
    #[doc = "0xc34 - "]
    #[inline(always)]
    pub const fn denali_phy_781(&self) -> &DenaliPhy781 {
        &self.denali_phy_781
    }
    #[doc = "0xc38 - "]
    #[inline(always)]
    pub const fn denali_phy_782(&self) -> &DenaliPhy782 {
        &self.denali_phy_782
    }
    #[doc = "0xc3c - "]
    #[inline(always)]
    pub const fn denali_phy_783(&self) -> &DenaliPhy783 {
        &self.denali_phy_783
    }
    #[doc = "0xc40 - "]
    #[inline(always)]
    pub const fn denali_phy_784(&self) -> &DenaliPhy784 {
        &self.denali_phy_784
    }
    #[doc = "0xc44 - "]
    #[inline(always)]
    pub const fn denali_phy_785(&self) -> &DenaliPhy785 {
        &self.denali_phy_785
    }
    #[doc = "0xc48 - "]
    #[inline(always)]
    pub const fn denali_phy_786(&self) -> &DenaliPhy786 {
        &self.denali_phy_786
    }
    #[doc = "0xc4c - "]
    #[inline(always)]
    pub const fn denali_phy_787(&self) -> &DenaliPhy787 {
        &self.denali_phy_787
    }
    #[doc = "0xc50 - "]
    #[inline(always)]
    pub const fn denali_phy_788(&self) -> &DenaliPhy788 {
        &self.denali_phy_788
    }
    #[doc = "0xc54 - "]
    #[inline(always)]
    pub const fn denali_phy_789(&self) -> &DenaliPhy789 {
        &self.denali_phy_789
    }
    #[doc = "0xc58 - "]
    #[inline(always)]
    pub const fn denali_phy_790(&self) -> &DenaliPhy790 {
        &self.denali_phy_790
    }
    #[doc = "0xc5c - "]
    #[inline(always)]
    pub const fn denali_phy_791(&self) -> &DenaliPhy791 {
        &self.denali_phy_791
    }
    #[doc = "0xc60 - "]
    #[inline(always)]
    pub const fn denali_phy_792(&self) -> &DenaliPhy792 {
        &self.denali_phy_792
    }
    #[doc = "0xc64 - "]
    #[inline(always)]
    pub const fn denali_phy_793(&self) -> &DenaliPhy793 {
        &self.denali_phy_793
    }
    #[doc = "0xc68 - "]
    #[inline(always)]
    pub const fn denali_phy_794(&self) -> &DenaliPhy794 {
        &self.denali_phy_794
    }
    #[doc = "0xc6c - "]
    #[inline(always)]
    pub const fn denali_phy_795(&self) -> &DenaliPhy795 {
        &self.denali_phy_795
    }
    #[doc = "0xc70 - "]
    #[inline(always)]
    pub const fn denali_phy_796(&self) -> &DenaliPhy796 {
        &self.denali_phy_796
    }
    #[doc = "0xc74 - "]
    #[inline(always)]
    pub const fn denali_phy_797(&self) -> &DenaliPhy797 {
        &self.denali_phy_797
    }
    #[doc = "0xc78 - "]
    #[inline(always)]
    pub const fn denali_phy_798(&self) -> &DenaliPhy798 {
        &self.denali_phy_798
    }
    #[doc = "0xc7c - "]
    #[inline(always)]
    pub const fn denali_phy_799(&self) -> &DenaliPhy799 {
        &self.denali_phy_799
    }
    #[doc = "0xc80 - "]
    #[inline(always)]
    pub const fn denali_phy_800(&self) -> &DenaliPhy800 {
        &self.denali_phy_800
    }
    #[doc = "0xc84 - "]
    #[inline(always)]
    pub const fn denali_phy_801(&self) -> &DenaliPhy801 {
        &self.denali_phy_801
    }
    #[doc = "0xc88 - "]
    #[inline(always)]
    pub const fn denali_phy_802(&self) -> &DenaliPhy802 {
        &self.denali_phy_802
    }
    #[doc = "0xc8c - "]
    #[inline(always)]
    pub const fn denali_phy_803(&self) -> &DenaliPhy803 {
        &self.denali_phy_803
    }
    #[doc = "0xc90 - "]
    #[inline(always)]
    pub const fn denali_phy_804(&self) -> &DenaliPhy804 {
        &self.denali_phy_804
    }
    #[doc = "0xc94 - "]
    #[inline(always)]
    pub const fn denali_phy_805(&self) -> &DenaliPhy805 {
        &self.denali_phy_805
    }
    #[doc = "0xe00 - "]
    #[inline(always)]
    pub const fn denali_phy_896(&self) -> &DenaliPhy896 {
        &self.denali_phy_896
    }
    #[doc = "0xe04 - "]
    #[inline(always)]
    pub const fn denali_phy_897(&self) -> &DenaliPhy897 {
        &self.denali_phy_897
    }
    #[doc = "0xe08 - "]
    #[inline(always)]
    pub const fn denali_phy_898(&self) -> &DenaliPhy898 {
        &self.denali_phy_898
    }
    #[doc = "0xe0c - "]
    #[inline(always)]
    pub const fn denali_phy_899(&self) -> &DenaliPhy899 {
        &self.denali_phy_899
    }
    #[doc = "0xe10 - "]
    #[inline(always)]
    pub const fn denali_phy_900(&self) -> &DenaliPhy900 {
        &self.denali_phy_900
    }
    #[doc = "0xe14 - "]
    #[inline(always)]
    pub const fn denali_phy_901(&self) -> &DenaliPhy901 {
        &self.denali_phy_901
    }
    #[doc = "0xe18 - "]
    #[inline(always)]
    pub const fn denali_phy_902(&self) -> &DenaliPhy902 {
        &self.denali_phy_902
    }
    #[doc = "0xe1c - "]
    #[inline(always)]
    pub const fn denali_phy_903(&self) -> &DenaliPhy903 {
        &self.denali_phy_903
    }
    #[doc = "0xe20 - "]
    #[inline(always)]
    pub const fn denali_phy_904(&self) -> &DenaliPhy904 {
        &self.denali_phy_904
    }
    #[doc = "0xe24 - "]
    #[inline(always)]
    pub const fn denali_phy_905(&self) -> &DenaliPhy905 {
        &self.denali_phy_905
    }
    #[doc = "0xe28 - "]
    #[inline(always)]
    pub const fn denali_phy_906(&self) -> &DenaliPhy906 {
        &self.denali_phy_906
    }
    #[doc = "0xe2c - "]
    #[inline(always)]
    pub const fn denali_phy_907(&self) -> &DenaliPhy907 {
        &self.denali_phy_907
    }
    #[doc = "0xe30 - "]
    #[inline(always)]
    pub const fn denali_phy_908(&self) -> &DenaliPhy908 {
        &self.denali_phy_908
    }
    #[doc = "0xe34 - "]
    #[inline(always)]
    pub const fn denali_phy_909(&self) -> &DenaliPhy909 {
        &self.denali_phy_909
    }
    #[doc = "0xe38 - "]
    #[inline(always)]
    pub const fn denali_phy_910(&self) -> &DenaliPhy910 {
        &self.denali_phy_910
    }
    #[doc = "0xe3c - "]
    #[inline(always)]
    pub const fn denali_phy_911(&self) -> &DenaliPhy911 {
        &self.denali_phy_911
    }
    #[doc = "0xe40 - "]
    #[inline(always)]
    pub const fn denali_phy_912(&self) -> &DenaliPhy912 {
        &self.denali_phy_912
    }
    #[doc = "0xe44 - "]
    #[inline(always)]
    pub const fn denali_phy_913(&self) -> &DenaliPhy913 {
        &self.denali_phy_913
    }
    #[doc = "0xe48 - "]
    #[inline(always)]
    pub const fn denali_phy_914(&self) -> &DenaliPhy914 {
        &self.denali_phy_914
    }
    #[doc = "0xe4c - "]
    #[inline(always)]
    pub const fn denali_phy_915(&self) -> &DenaliPhy915 {
        &self.denali_phy_915
    }
    #[doc = "0xe50 - "]
    #[inline(always)]
    pub const fn denali_phy_916(&self) -> &DenaliPhy916 {
        &self.denali_phy_916
    }
    #[doc = "0xe54 - "]
    #[inline(always)]
    pub const fn denali_phy_917(&self) -> &DenaliPhy917 {
        &self.denali_phy_917
    }
    #[doc = "0xe58 - "]
    #[inline(always)]
    pub const fn denali_phy_918(&self) -> &DenaliPhy918 {
        &self.denali_phy_918
    }
    #[doc = "0xe5c - "]
    #[inline(always)]
    pub const fn denali_phy_919(&self) -> &DenaliPhy919 {
        &self.denali_phy_919
    }
    #[doc = "0xe60 - "]
    #[inline(always)]
    pub const fn denali_phy_920(&self) -> &DenaliPhy920 {
        &self.denali_phy_920
    }
    #[doc = "0xe64 - "]
    #[inline(always)]
    pub const fn denali_phy_921(&self) -> &DenaliPhy921 {
        &self.denali_phy_921
    }
    #[doc = "0xe68 - "]
    #[inline(always)]
    pub const fn denali_phy_922(&self) -> &DenaliPhy922 {
        &self.denali_phy_922
    }
    #[doc = "0xe6c - "]
    #[inline(always)]
    pub const fn denali_phy_923(&self) -> &DenaliPhy923 {
        &self.denali_phy_923
    }
    #[doc = "0xe70 - "]
    #[inline(always)]
    pub const fn denali_phy_924(&self) -> &DenaliPhy924 {
        &self.denali_phy_924
    }
    #[doc = "0xe74 - "]
    #[inline(always)]
    pub const fn denali_phy_925(&self) -> &DenaliPhy925 {
        &self.denali_phy_925
    }
    #[doc = "0xe78 - "]
    #[inline(always)]
    pub const fn denali_phy_926(&self) -> &DenaliPhy926 {
        &self.denali_phy_926
    }
    #[doc = "0xe7c - "]
    #[inline(always)]
    pub const fn denali_phy_927(&self) -> &DenaliPhy927 {
        &self.denali_phy_927
    }
    #[doc = "0xe80 - "]
    #[inline(always)]
    pub const fn denali_phy_928(&self) -> &DenaliPhy928 {
        &self.denali_phy_928
    }
    #[doc = "0xe84 - "]
    #[inline(always)]
    pub const fn denali_phy_929(&self) -> &DenaliPhy929 {
        &self.denali_phy_929
    }
    #[doc = "0xe88 - "]
    #[inline(always)]
    pub const fn denali_phy_930(&self) -> &DenaliPhy930 {
        &self.denali_phy_930
    }
    #[doc = "0xe8c - "]
    #[inline(always)]
    pub const fn denali_phy_931(&self) -> &DenaliPhy931 {
        &self.denali_phy_931
    }
    #[doc = "0xe90 - "]
    #[inline(always)]
    pub const fn denali_phy_932(&self) -> &DenaliPhy932 {
        &self.denali_phy_932
    }
    #[doc = "0xe94 - "]
    #[inline(always)]
    pub const fn denali_phy_933(&self) -> &DenaliPhy933 {
        &self.denali_phy_933
    }
    #[doc = "0xe98 - "]
    #[inline(always)]
    pub const fn denali_phy_934(&self) -> &DenaliPhy934 {
        &self.denali_phy_934
    }
    #[doc = "0xe9c - "]
    #[inline(always)]
    pub const fn denali_phy_935(&self) -> &DenaliPhy935 {
        &self.denali_phy_935
    }
    #[doc = "0xea0 - "]
    #[inline(always)]
    pub const fn denali_phy_936(&self) -> &DenaliPhy936 {
        &self.denali_phy_936
    }
    #[doc = "0xea4 - "]
    #[inline(always)]
    pub const fn denali_phy_937(&self) -> &DenaliPhy937 {
        &self.denali_phy_937
    }
    #[doc = "0xea8 - "]
    #[inline(always)]
    pub const fn denali_phy_938(&self) -> &DenaliPhy938 {
        &self.denali_phy_938
    }
    #[doc = "0xeac - "]
    #[inline(always)]
    pub const fn denali_phy_939(&self) -> &DenaliPhy939 {
        &self.denali_phy_939
    }
    #[doc = "0xeb0 - "]
    #[inline(always)]
    pub const fn denali_phy_940(&self) -> &DenaliPhy940 {
        &self.denali_phy_940
    }
    #[doc = "0xeb4 - "]
    #[inline(always)]
    pub const fn denali_phy_941(&self) -> &DenaliPhy941 {
        &self.denali_phy_941
    }
    #[doc = "0xeb8 - "]
    #[inline(always)]
    pub const fn denali_phy_942(&self) -> &DenaliPhy942 {
        &self.denali_phy_942
    }
    #[doc = "0xebc - "]
    #[inline(always)]
    pub const fn denali_phy_943(&self) -> &DenaliPhy943 {
        &self.denali_phy_943
    }
    #[doc = "0xec0 - "]
    #[inline(always)]
    pub const fn denali_phy_944(&self) -> &DenaliPhy944 {
        &self.denali_phy_944
    }
    #[doc = "0xec4 - "]
    #[inline(always)]
    pub const fn denali_phy_945(&self) -> &DenaliPhy945 {
        &self.denali_phy_945
    }
    #[doc = "0xec8 - "]
    #[inline(always)]
    pub const fn denali_phy_946(&self) -> &DenaliPhy946 {
        &self.denali_phy_946
    }
    #[doc = "0xecc - "]
    #[inline(always)]
    pub const fn denali_phy_947(&self) -> &DenaliPhy947 {
        &self.denali_phy_947
    }
    #[doc = "0xed0 - "]
    #[inline(always)]
    pub const fn denali_phy_948(&self) -> &DenaliPhy948 {
        &self.denali_phy_948
    }
    #[doc = "0xed4 - "]
    #[inline(always)]
    pub const fn denali_phy_949(&self) -> &DenaliPhy949 {
        &self.denali_phy_949
    }
    #[doc = "0xed8 - "]
    #[inline(always)]
    pub const fn denali_phy_950(&self) -> &DenaliPhy950 {
        &self.denali_phy_950
    }
    #[doc = "0xedc - "]
    #[inline(always)]
    pub const fn denali_phy_951(&self) -> &DenaliPhy951 {
        &self.denali_phy_951
    }
    #[doc = "0xee0 - "]
    #[inline(always)]
    pub const fn denali_phy_952(&self) -> &DenaliPhy952 {
        &self.denali_phy_952
    }
    #[doc = "0xee4 - "]
    #[inline(always)]
    pub const fn denali_phy_953(&self) -> &DenaliPhy953 {
        &self.denali_phy_953
    }
    #[doc = "0xee8 - "]
    #[inline(always)]
    pub const fn denali_phy_954(&self) -> &DenaliPhy954 {
        &self.denali_phy_954
    }
    #[doc = "0xeec - "]
    #[inline(always)]
    pub const fn denali_phy_955(&self) -> &DenaliPhy955 {
        &self.denali_phy_955
    }
    #[doc = "0xef0 - "]
    #[inline(always)]
    pub const fn denali_phy_956(&self) -> &DenaliPhy956 {
        &self.denali_phy_956
    }
    #[doc = "0xef4 - "]
    #[inline(always)]
    pub const fn denali_phy_957(&self) -> &DenaliPhy957 {
        &self.denali_phy_957
    }
    #[doc = "0xef8 - "]
    #[inline(always)]
    pub const fn denali_phy_958(&self) -> &DenaliPhy958 {
        &self.denali_phy_958
    }
}
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
#[doc = "DENALI_PHY_82 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_82::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_82`]
module"]
#[doc(alias = "DENALI_PHY_82")]
pub type DenaliPhy82 = crate::Reg<denali_phy_82::DenaliPhy82Spec>;
#[doc = ""]
pub mod denali_phy_82;
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
#[doc = "DENALI_PHY_210 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_210::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_210`]
module"]
#[doc(alias = "DENALI_PHY_210")]
pub type DenaliPhy210 = crate::Reg<denali_phy_210::DenaliPhy210Spec>;
#[doc = ""]
pub mod denali_phy_210;
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
#[doc = "DENALI_PHY_338 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_338::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_338`]
module"]
#[doc(alias = "DENALI_PHY_338")]
pub type DenaliPhy338 = crate::Reg<denali_phy_338::DenaliPhy338Spec>;
#[doc = ""]
pub mod denali_phy_338;
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
#[doc = "DENALI_PHY_466 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_466::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_466`]
module"]
#[doc(alias = "DENALI_PHY_466")]
pub type DenaliPhy466 = crate::Reg<denali_phy_466::DenaliPhy466Spec>;
#[doc = ""]
pub mod denali_phy_466;
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
#[doc = "DENALI_PHY_909 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_909::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@denali_phy_909`]
module"]
#[doc(alias = "DENALI_PHY_909")]
pub type DenaliPhy909 = crate::Reg<denali_phy_909::DenaliPhy909Spec>;
#[doc = ""]
pub mod denali_phy_909;
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
