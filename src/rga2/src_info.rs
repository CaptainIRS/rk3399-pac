#[doc = "Register `SRC_INFO` reader"]
pub type R = crate::R<SrcInfoSpec>;
#[doc = "Register `SRC_INFO` writer"]
pub type W = crate::W<SrcInfoSpec>;
#[doc = "Source bitmap data format\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwSrcFmt {
    #[doc = "0: ABGR888"]
    B0000 = 0,
    #[doc = "1: XBGR888"]
    B0001 = 1,
    #[doc = "2: BGR packed"]
    B0010 = 2,
    #[doc = "4: RGB565"]
    B0100 = 4,
    #[doc = "5: ARGB1555"]
    B0101 = 5,
    #[doc = "6: ARGB4444"]
    B0110 = 6,
    #[doc = "8: YUV422SP"]
    B1000 = 8,
    #[doc = "9: YUV422P"]
    B1001 = 9,
    #[doc = "10: YUV420SP"]
    B1010 = 10,
    #[doc = "11: YUV420P"]
    B1011 = 11,
    #[doc = "12: 1BPP (color palette)"]
    B1100 = 12,
    #[doc = "13: 2BPP (color palette)"]
    B1101 = 13,
    #[doc = "14: 4BPP (color palette)"]
    B1110 = 14,
    #[doc = "15: 8BPP (color palette)"]
    B1111 = 15,
}
impl From<SwSrcFmt> for u8 {
    #[inline(always)]
    fn from(variant: SwSrcFmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwSrcFmt {
    type Ux = u8;
}
#[doc = "Field `SW_SRC_FMT` reader - Source bitmap data format"]
pub type SwSrcFmtR = crate::FieldReader<SwSrcFmt>;
impl SwSrcFmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwSrcFmt> {
        match self.bits {
            0 => Some(SwSrcFmt::B0000),
            1 => Some(SwSrcFmt::B0001),
            2 => Some(SwSrcFmt::B0010),
            4 => Some(SwSrcFmt::B0100),
            5 => Some(SwSrcFmt::B0101),
            6 => Some(SwSrcFmt::B0110),
            8 => Some(SwSrcFmt::B1000),
            9 => Some(SwSrcFmt::B1001),
            10 => Some(SwSrcFmt::B1010),
            11 => Some(SwSrcFmt::B1011),
            12 => Some(SwSrcFmt::B1100),
            13 => Some(SwSrcFmt::B1101),
            14 => Some(SwSrcFmt::B1110),
            15 => Some(SwSrcFmt::B1111),
            _ => None,
        }
    }
    #[doc = "ABGR888"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == SwSrcFmt::B0000
    }
    #[doc = "XBGR888"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == SwSrcFmt::B0001
    }
    #[doc = "BGR packed"]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == SwSrcFmt::B0010
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == SwSrcFmt::B0100
    }
    #[doc = "ARGB1555"]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == SwSrcFmt::B0101
    }
    #[doc = "ARGB4444"]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == SwSrcFmt::B0110
    }
    #[doc = "YUV422SP"]
    #[inline(always)]
    pub fn is_b1000(&self) -> bool {
        *self == SwSrcFmt::B1000
    }
    #[doc = "YUV422P"]
    #[inline(always)]
    pub fn is_b1001(&self) -> bool {
        *self == SwSrcFmt::B1001
    }
    #[doc = "YUV420SP"]
    #[inline(always)]
    pub fn is_b1010(&self) -> bool {
        *self == SwSrcFmt::B1010
    }
    #[doc = "YUV420P"]
    #[inline(always)]
    pub fn is_b1011(&self) -> bool {
        *self == SwSrcFmt::B1011
    }
    #[doc = "1BPP (color palette)"]
    #[inline(always)]
    pub fn is_b1100(&self) -> bool {
        *self == SwSrcFmt::B1100
    }
    #[doc = "2BPP (color palette)"]
    #[inline(always)]
    pub fn is_b1101(&self) -> bool {
        *self == SwSrcFmt::B1101
    }
    #[doc = "4BPP (color palette)"]
    #[inline(always)]
    pub fn is_b1110(&self) -> bool {
        *self == SwSrcFmt::B1110
    }
    #[doc = "8BPP (color palette)"]
    #[inline(always)]
    pub fn is_b1111(&self) -> bool {
        *self == SwSrcFmt::B1111
    }
}
#[doc = "Field `SW_SRC_FMT` writer - Source bitmap data format"]
pub type SwSrcFmtW<'a, REG> = crate::FieldWriter<'a, REG, 4, SwSrcFmt>;
impl<'a, REG> SwSrcFmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ABGR888"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFmt::B0000)
    }
    #[doc = "XBGR888"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFmt::B0001)
    }
    #[doc = "BGR packed"]
    #[inline(always)]
    pub fn b0010(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFmt::B0010)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFmt::B0100)
    }
    #[doc = "ARGB1555"]
    #[inline(always)]
    pub fn b0101(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFmt::B0101)
    }
    #[doc = "ARGB4444"]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFmt::B0110)
    }
    #[doc = "YUV422SP"]
    #[inline(always)]
    pub fn b1000(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFmt::B1000)
    }
    #[doc = "YUV422P"]
    #[inline(always)]
    pub fn b1001(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFmt::B1001)
    }
    #[doc = "YUV420SP"]
    #[inline(always)]
    pub fn b1010(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFmt::B1010)
    }
    #[doc = "YUV420P"]
    #[inline(always)]
    pub fn b1011(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFmt::B1011)
    }
    #[doc = "1BPP (color palette)"]
    #[inline(always)]
    pub fn b1100(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFmt::B1100)
    }
    #[doc = "2BPP (color palette)"]
    #[inline(always)]
    pub fn b1101(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFmt::B1101)
    }
    #[doc = "4BPP (color palette)"]
    #[inline(always)]
    pub fn b1110(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFmt::B1110)
    }
    #[doc = "8BPP (color palette)"]
    #[inline(always)]
    pub fn b1111(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFmt::B1111)
    }
}
#[doc = "Source bitmap data RB swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrcRbswap {
    #[doc = "0: BGR"]
    B0 = 0,
    #[doc = "1: RGB"]
    B1 = 1,
}
impl From<SwSrcRbswap> for bool {
    #[inline(always)]
    fn from(variant: SwSrcRbswap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC_RBSWAP` reader - Source bitmap data RB swap"]
pub type SwSrcRbswapR = crate::BitReader<SwSrcRbswap>;
impl SwSrcRbswapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrcRbswap {
        match self.bits {
            false => SwSrcRbswap::B0,
            true => SwSrcRbswap::B1,
        }
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrcRbswap::B0
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrcRbswap::B1
    }
}
#[doc = "Field `SW_SRC_RBSWAP` writer - Source bitmap data RB swap"]
pub type SwSrcRbswapW<'a, REG> = crate::BitWriter<'a, REG, SwSrcRbswap>;
impl<'a, REG> SwSrcRbswapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BGR"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcRbswap::B0)
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcRbswap::B1)
    }
}
#[doc = "Source bitmap data alpha swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrcAlphaSwap {
    #[doc = "0: ABGR"]
    B0 = 0,
    #[doc = "1: BGRA"]
    B1 = 1,
}
impl From<SwSrcAlphaSwap> for bool {
    #[inline(always)]
    fn from(variant: SwSrcAlphaSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC_ALPHA_SWAP` reader - Source bitmap data alpha swap"]
pub type SwSrcAlphaSwapR = crate::BitReader<SwSrcAlphaSwap>;
impl SwSrcAlphaSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrcAlphaSwap {
        match self.bits {
            false => SwSrcAlphaSwap::B0,
            true => SwSrcAlphaSwap::B1,
        }
    }
    #[doc = "ABGR"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrcAlphaSwap::B0
    }
    #[doc = "BGRA"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrcAlphaSwap::B1
    }
}
#[doc = "Field `SW_SRC_ALPHA_SWAP` writer - Source bitmap data alpha swap"]
pub type SwSrcAlphaSwapW<'a, REG> = crate::BitWriter<'a, REG, SwSrcAlphaSwap>;
impl<'a, REG> SwSrcAlphaSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ABGR"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcAlphaSwap::B0)
    }
    #[doc = "BGRA"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcAlphaSwap::B1)
    }
}
#[doc = "Source Cb-Cr swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrcUvswap {
    #[doc = "0: CrCb"]
    B0 = 0,
    #[doc = "1: CbCr"]
    B1 = 1,
}
impl From<SwSrcUvswap> for bool {
    #[inline(always)]
    fn from(variant: SwSrcUvswap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC_UVSWAP` reader - Source Cb-Cr swap"]
pub type SwSrcUvswapR = crate::BitReader<SwSrcUvswap>;
impl SwSrcUvswapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrcUvswap {
        match self.bits {
            false => SwSrcUvswap::B0,
            true => SwSrcUvswap::B1,
        }
    }
    #[doc = "CrCb"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrcUvswap::B0
    }
    #[doc = "CbCr"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrcUvswap::B1
    }
}
#[doc = "Field `SW_SRC_UVSWAP` writer - Source Cb-Cr swap"]
pub type SwSrcUvswapW<'a, REG> = crate::BitWriter<'a, REG, SwSrcUvswap>;
impl<'a, REG> SwSrcUvswapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CrCb"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcUvswap::B0)
    }
    #[doc = "CbCr"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcUvswap::B1)
    }
}
#[doc = "Source Color palette endian swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwCpEndian {
    #[doc = "0: big endian"]
    B0 = 0,
    #[doc = "1: little endian"]
    B1 = 1,
}
impl From<SwCpEndian> for bool {
    #[inline(always)]
    fn from(variant: SwCpEndian) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_CP_ENDIAN` reader - Source Color palette endian swap"]
pub type SwCpEndianR = crate::BitReader<SwCpEndian>;
impl SwCpEndianR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwCpEndian {
        match self.bits {
            false => SwCpEndian::B0,
            true => SwCpEndian::B1,
        }
    }
    #[doc = "big endian"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwCpEndian::B0
    }
    #[doc = "little endian"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwCpEndian::B1
    }
}
#[doc = "Field `SW_CP_ENDIAN` writer - Source Color palette endian swap"]
pub type SwCpEndianW<'a, REG> = crate::BitWriter<'a, REG, SwCpEndian>;
impl<'a, REG> SwCpEndianW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "big endian"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwCpEndian::B0)
    }
    #[doc = "little endian"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwCpEndian::B1)
    }
}
#[doc = "Source bitmap YUV2RGB conversion mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwSrcCscMode {
    #[doc = "0: bypass"]
    B00 = 0,
    #[doc = "1: BT.601-range0"]
    B01 = 1,
    #[doc = "2: BT.601-range1"]
    B10 = 2,
    #[doc = "3: BT.709-range0"]
    B11 = 3,
}
impl From<SwSrcCscMode> for u8 {
    #[inline(always)]
    fn from(variant: SwSrcCscMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwSrcCscMode {
    type Ux = u8;
}
#[doc = "Field `SW_SRC_CSC_MODE` reader - Source bitmap YUV2RGB conversion mode"]
pub type SwSrcCscModeR = crate::FieldReader<SwSrcCscMode>;
impl SwSrcCscModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrcCscMode {
        match self.bits {
            0 => SwSrcCscMode::B00,
            1 => SwSrcCscMode::B01,
            2 => SwSrcCscMode::B10,
            3 => SwSrcCscMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "bypass"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SwSrcCscMode::B00
    }
    #[doc = "BT.601-range0"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SwSrcCscMode::B01
    }
    #[doc = "BT.601-range1"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SwSrcCscMode::B10
    }
    #[doc = "BT.709-range0"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == SwSrcCscMode::B11
    }
}
#[doc = "Field `SW_SRC_CSC_MODE` writer - Source bitmap YUV2RGB conversion mode"]
pub type SwSrcCscModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SwSrcCscMode>;
impl<'a, REG> SwSrcCscModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "bypass"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcCscMode::B00)
    }
    #[doc = "BT.601-range0"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcCscMode::B01)
    }
    #[doc = "BT.601-range1"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcCscMode::B10)
    }
    #[doc = "BT.709-range0"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcCscMode::B11)
    }
}
#[doc = "SRC rotation mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwSrcRotMode {
    #[doc = "0: 0 degree"]
    B00 = 0,
    #[doc = "1: 90 degree"]
    B01 = 1,
    #[doc = "2: 180 degree"]
    B10 = 2,
    #[doc = "3: 270 degree"]
    B11 = 3,
}
impl From<SwSrcRotMode> for u8 {
    #[inline(always)]
    fn from(variant: SwSrcRotMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwSrcRotMode {
    type Ux = u8;
}
#[doc = "Field `SW_SRC_ROT_MODE` reader - SRC rotation mode"]
pub type SwSrcRotModeR = crate::FieldReader<SwSrcRotMode>;
impl SwSrcRotModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrcRotMode {
        match self.bits {
            0 => SwSrcRotMode::B00,
            1 => SwSrcRotMode::B01,
            2 => SwSrcRotMode::B10,
            3 => SwSrcRotMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "0 degree"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SwSrcRotMode::B00
    }
    #[doc = "90 degree"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SwSrcRotMode::B01
    }
    #[doc = "180 degree"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SwSrcRotMode::B10
    }
    #[doc = "270 degree"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == SwSrcRotMode::B11
    }
}
#[doc = "Field `SW_SRC_ROT_MODE` writer - SRC rotation mode"]
pub type SwSrcRotModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SwSrcRotMode>;
impl<'a, REG> SwSrcRotModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 degree"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcRotMode::B00)
    }
    #[doc = "90 degree"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcRotMode::B01)
    }
    #[doc = "180 degree"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcRotMode::B10)
    }
    #[doc = "270 degree"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcRotMode::B11)
    }
}
#[doc = "SRC mirror mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwSrcMirMode {
    #[doc = "0: no mirror"]
    B00 = 0,
    #[doc = "1: x mirror"]
    B01 = 1,
    #[doc = "2: y mirror"]
    B10 = 2,
    #[doc = "3: x mirror + y mirror"]
    B11 = 3,
}
impl From<SwSrcMirMode> for u8 {
    #[inline(always)]
    fn from(variant: SwSrcMirMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwSrcMirMode {
    type Ux = u8;
}
#[doc = "Field `SW_SRC_MIR_MODE` reader - SRC mirror mode"]
pub type SwSrcMirModeR = crate::FieldReader<SwSrcMirMode>;
impl SwSrcMirModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrcMirMode {
        match self.bits {
            0 => SwSrcMirMode::B00,
            1 => SwSrcMirMode::B01,
            2 => SwSrcMirMode::B10,
            3 => SwSrcMirMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "no mirror"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SwSrcMirMode::B00
    }
    #[doc = "x mirror"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SwSrcMirMode::B01
    }
    #[doc = "y mirror"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SwSrcMirMode::B10
    }
    #[doc = "x mirror + y mirror"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == SwSrcMirMode::B11
    }
}
#[doc = "Field `SW_SRC_MIR_MODE` writer - SRC mirror mode"]
pub type SwSrcMirModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SwSrcMirMode>;
impl<'a, REG> SwSrcMirModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no mirror"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcMirMode::B00)
    }
    #[doc = "x mirror"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcMirMode::B01)
    }
    #[doc = "y mirror"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcMirMode::B10)
    }
    #[doc = "x mirror + y mirror"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcMirMode::B11)
    }
}
#[doc = "SRC horizontal scaling mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwSrcHsclMode {
    #[doc = "0: no scaling"]
    B00 = 0,
    #[doc = "1: down-scaling"]
    B01 = 1,
    #[doc = "2: up-scaling"]
    B10 = 2,
}
impl From<SwSrcHsclMode> for u8 {
    #[inline(always)]
    fn from(variant: SwSrcHsclMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwSrcHsclMode {
    type Ux = u8;
}
#[doc = "Field `SW_SRC_HSCL_MODE` reader - SRC horizontal scaling mode"]
pub type SwSrcHsclModeR = crate::FieldReader<SwSrcHsclMode>;
impl SwSrcHsclModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwSrcHsclMode> {
        match self.bits {
            0 => Some(SwSrcHsclMode::B00),
            1 => Some(SwSrcHsclMode::B01),
            2 => Some(SwSrcHsclMode::B10),
            _ => None,
        }
    }
    #[doc = "no scaling"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SwSrcHsclMode::B00
    }
    #[doc = "down-scaling"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SwSrcHsclMode::B01
    }
    #[doc = "up-scaling"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SwSrcHsclMode::B10
    }
}
#[doc = "Field `SW_SRC_HSCL_MODE` writer - SRC horizontal scaling mode"]
pub type SwSrcHsclModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, SwSrcHsclMode>;
impl<'a, REG> SwSrcHsclModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no scaling"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcHsclMode::B00)
    }
    #[doc = "down-scaling"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcHsclMode::B01)
    }
    #[doc = "up-scaling"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcHsclMode::B10)
    }
}
#[doc = "SRC vertical scaling mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwSrcVsclMode {
    #[doc = "0: no scaling"]
    B00 = 0,
    #[doc = "1: down-scaling"]
    B01 = 1,
    #[doc = "2: up-scaling"]
    B10 = 2,
}
impl From<SwSrcVsclMode> for u8 {
    #[inline(always)]
    fn from(variant: SwSrcVsclMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwSrcVsclMode {
    type Ux = u8;
}
#[doc = "Field `SW_SRC_VSCL_MODE` reader - SRC vertical scaling mode"]
pub type SwSrcVsclModeR = crate::FieldReader<SwSrcVsclMode>;
impl SwSrcVsclModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwSrcVsclMode> {
        match self.bits {
            0 => Some(SwSrcVsclMode::B00),
            1 => Some(SwSrcVsclMode::B01),
            2 => Some(SwSrcVsclMode::B10),
            _ => None,
        }
    }
    #[doc = "no scaling"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SwSrcVsclMode::B00
    }
    #[doc = "down-scaling"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SwSrcVsclMode::B01
    }
    #[doc = "up-scaling"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SwSrcVsclMode::B10
    }
}
#[doc = "Field `SW_SRC_VSCL_MODE` writer - SRC vertical scaling mode"]
pub type SwSrcVsclModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, SwSrcVsclMode>;
impl<'a, REG> SwSrcVsclModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no scaling"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcVsclMode::B00)
    }
    #[doc = "down-scaling"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcVsclMode::B01)
    }
    #[doc = "up-scaling"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcVsclMode::B10)
    }
}
#[doc = "Source transparency mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrcTransMode {
    #[doc = "0: normal stencil test (color key)"]
    B0 = 0,
    #[doc = "1: inverted stencil test"]
    B1 = 1,
}
impl From<SwSrcTransMode> for bool {
    #[inline(always)]
    fn from(variant: SwSrcTransMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC_TRANS_MODE` reader - Source transparency mode"]
pub type SwSrcTransModeR = crate::BitReader<SwSrcTransMode>;
impl SwSrcTransModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrcTransMode {
        match self.bits {
            false => SwSrcTransMode::B0,
            true => SwSrcTransMode::B1,
        }
    }
    #[doc = "normal stencil test (color key)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrcTransMode::B0
    }
    #[doc = "inverted stencil test"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrcTransMode::B1
    }
}
#[doc = "Field `SW_SRC_TRANS_MODE` writer - Source transparency mode"]
pub type SwSrcTransModeW<'a, REG> = crate::BitWriter<'a, REG, SwSrcTransMode>;
impl<'a, REG> SwSrcTransModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal stencil test (color key)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcTransMode::B0)
    }
    #[doc = "inverted stencil test"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcTransMode::B1)
    }
}
#[doc = "Field `SW_SRC_TRANS_E` reader - Source transparency enable bits\n\n\\[3\\]: A value stencil test enable bit\n\n\\[2\\]: B value stencil test enable bit\n\n\\[1\\]: G value stencil test enable bit\n\n\\[0\\]: R value stencil test enable bit"]
pub type SwSrcTransER = crate::FieldReader;
#[doc = "Field `SW_SRC_TRANS_E` writer - Source transparency enable bits\n\n\\[3\\]: A value stencil test enable bit\n\n\\[2\\]: B value stencil test enable bit\n\n\\[1\\]: G value stencil test enable bit\n\n\\[0\\]: R value stencil test enable bit"]
pub type SwSrcTransEW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "SRC dither up enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrcDitherUp {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwSrcDitherUp> for bool {
    #[inline(always)]
    fn from(variant: SwSrcDitherUp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC_DITHER_UP` reader - SRC dither up enable"]
pub type SwSrcDitherUpR = crate::BitReader<SwSrcDitherUp>;
impl SwSrcDitherUpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrcDitherUp {
        match self.bits {
            false => SwSrcDitherUp::B0,
            true => SwSrcDitherUp::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrcDitherUp::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrcDitherUp::B1
    }
}
#[doc = "Field `SW_SRC_DITHER_UP` writer - SRC dither up enable"]
pub type SwSrcDitherUpW<'a, REG> = crate::BitWriter<'a, REG, SwSrcDitherUp>;
impl<'a, REG> SwSrcDitherUpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcDitherUp::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcDitherUp::B1)
    }
}
#[doc = "SRC bicubic scaling coefficient select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwBicCoeSel {
    #[doc = "0: CATROM"]
    B00 = 0,
    #[doc = "1: MITCHELL"]
    B01 = 1,
    #[doc = "2: HERMITE"]
    B10 = 2,
    #[doc = "3: B-SPLINE"]
    B11 = 3,
}
impl From<SwBicCoeSel> for u8 {
    #[inline(always)]
    fn from(variant: SwBicCoeSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwBicCoeSel {
    type Ux = u8;
}
#[doc = "Field `SW_BIC_COE_SEL` reader - SRC bicubic scaling coefficient select"]
pub type SwBicCoeSelR = crate::FieldReader<SwBicCoeSel>;
impl SwBicCoeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwBicCoeSel {
        match self.bits {
            0 => SwBicCoeSel::B00,
            1 => SwBicCoeSel::B01,
            2 => SwBicCoeSel::B10,
            3 => SwBicCoeSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "CATROM"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SwBicCoeSel::B00
    }
    #[doc = "MITCHELL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SwBicCoeSel::B01
    }
    #[doc = "HERMITE"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SwBicCoeSel::B10
    }
    #[doc = "B-SPLINE"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == SwBicCoeSel::B11
    }
}
#[doc = "Field `SW_BIC_COE_SEL` writer - SRC bicubic scaling coefficient select"]
pub type SwBicCoeSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SwBicCoeSel>;
impl<'a, REG> SwBicCoeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CATROM"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SwBicCoeSel::B00)
    }
    #[doc = "MITCHELL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SwBicCoeSel::B01)
    }
    #[doc = "HERMITE"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SwBicCoeSel::B10)
    }
    #[doc = "B-SPLINE"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(SwBicCoeSel::B11)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwVspMode {
    #[doc = "0: by-cubic"]
    B0 = 0,
    #[doc = "1: bi-linear"]
    B1 = 1,
}
impl From<SwVspMode> for bool {
    #[inline(always)]
    fn from(variant: SwVspMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_VSP_MODE` reader - "]
pub type SwVspModeR = crate::BitReader<SwVspMode>;
impl SwVspModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwVspMode {
        match self.bits {
            false => SwVspMode::B0,
            true => SwVspMode::B1,
        }
    }
    #[doc = "by-cubic"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwVspMode::B0
    }
    #[doc = "bi-linear"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwVspMode::B1
    }
}
#[doc = "Field `SW_VSP_MODE` writer - "]
pub type SwVspModeW<'a, REG> = crate::BitWriter<'a, REG, SwVspMode>;
impl<'a, REG> SwVspModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "by-cubic"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwVspMode::B0)
    }
    #[doc = "bi-linear"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwVspMode::B1)
    }
}
#[doc = "this bit valid when RGA support yuv 10bit picture input\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrcYuv10E {
    #[doc = "0: yuv 10bit disable"]
    B0 = 0,
    #[doc = "1: yuv 10bit enalbe"]
    B1 = 1,
}
impl From<SwSrcYuv10E> for bool {
    #[inline(always)]
    fn from(variant: SwSrcYuv10E) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC_YUV10_E` reader - this bit valid when RGA support yuv 10bit picture input"]
pub type SwSrcYuv10ER = crate::BitReader<SwSrcYuv10E>;
impl SwSrcYuv10ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrcYuv10E {
        match self.bits {
            false => SwSrcYuv10E::B0,
            true => SwSrcYuv10E::B1,
        }
    }
    #[doc = "yuv 10bit disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrcYuv10E::B0
    }
    #[doc = "yuv 10bit enalbe"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrcYuv10E::B1
    }
}
#[doc = "Field `SW_SRC_YUV10_E` writer - this bit valid when RGA support yuv 10bit picture input"]
pub type SwSrcYuv10EW<'a, REG> = crate::BitWriter<'a, REG, SwSrcYuv10E>;
impl<'a, REG> SwSrcYuv10EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "yuv 10bit disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcYuv10E::B0)
    }
    #[doc = "yuv 10bit enalbe"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcYuv10E::B1)
    }
}
#[doc = "this bit valid when RGA support yuv 10bit picture input\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrcYuv10RoundE {
    #[doc = "0: yuv 10bit to 8bit round disable"]
    B0 = 0,
    #[doc = "1: yuv 10bit to 8bit round enable"]
    B1 = 1,
}
impl From<SwSrcYuv10RoundE> for bool {
    #[inline(always)]
    fn from(variant: SwSrcYuv10RoundE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC_YUV10_ROUND_E` reader - this bit valid when RGA support yuv 10bit picture input"]
pub type SwSrcYuv10RoundER = crate::BitReader<SwSrcYuv10RoundE>;
impl SwSrcYuv10RoundER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrcYuv10RoundE {
        match self.bits {
            false => SwSrcYuv10RoundE::B0,
            true => SwSrcYuv10RoundE::B1,
        }
    }
    #[doc = "yuv 10bit to 8bit round disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrcYuv10RoundE::B0
    }
    #[doc = "yuv 10bit to 8bit round enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrcYuv10RoundE::B1
    }
}
#[doc = "Field `SW_SRC_YUV10_ROUND_E` writer - this bit valid when RGA support yuv 10bit picture input"]
pub type SwSrcYuv10RoundEW<'a, REG> = crate::BitWriter<'a, REG, SwSrcYuv10RoundE>;
impl<'a, REG> SwSrcYuv10RoundEW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "yuv 10bit to 8bit round disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcYuv10RoundE::B0)
    }
    #[doc = "yuv 10bit to 8bit round enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcYuv10RoundE::B1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Source bitmap data format"]
    #[inline(always)]
    pub fn sw_src_fmt(&self) -> SwSrcFmtR {
        SwSrcFmtR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Source bitmap data RB swap"]
    #[inline(always)]
    pub fn sw_src_rbswap(&self) -> SwSrcRbswapR {
        SwSrcRbswapR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Source bitmap data alpha swap"]
    #[inline(always)]
    pub fn sw_src_alpha_swap(&self) -> SwSrcAlphaSwapR {
        SwSrcAlphaSwapR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Source Cb-Cr swap"]
    #[inline(always)]
    pub fn sw_src_uvswap(&self) -> SwSrcUvswapR {
        SwSrcUvswapR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Source Color palette endian swap"]
    #[inline(always)]
    pub fn sw_cp_endian(&self) -> SwCpEndianR {
        SwCpEndianR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Source bitmap YUV2RGB conversion mode"]
    #[inline(always)]
    pub fn sw_src_csc_mode(&self) -> SwSrcCscModeR {
        SwSrcCscModeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - SRC rotation mode"]
    #[inline(always)]
    pub fn sw_src_rot_mode(&self) -> SwSrcRotModeR {
        SwSrcRotModeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - SRC mirror mode"]
    #[inline(always)]
    pub fn sw_src_mir_mode(&self) -> SwSrcMirModeR {
        SwSrcMirModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - SRC horizontal scaling mode"]
    #[inline(always)]
    pub fn sw_src_hscl_mode(&self) -> SwSrcHsclModeR {
        SwSrcHsclModeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - SRC vertical scaling mode"]
    #[inline(always)]
    pub fn sw_src_vscl_mode(&self) -> SwSrcVsclModeR {
        SwSrcVsclModeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Source transparency mode"]
    #[inline(always)]
    pub fn sw_src_trans_mode(&self) -> SwSrcTransModeR {
        SwSrcTransModeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - Source transparency enable bits\n\n\\[3\\]: A value stencil test enable bit\n\n\\[2\\]: B value stencil test enable bit\n\n\\[1\\]: G value stencil test enable bit\n\n\\[0\\]: R value stencil test enable bit"]
    #[inline(always)]
    pub fn sw_src_trans_e(&self) -> SwSrcTransER {
        SwSrcTransER::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - SRC dither up enable"]
    #[inline(always)]
    pub fn sw_src_dither_up(&self) -> SwSrcDitherUpR {
        SwSrcDitherUpR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - SRC bicubic scaling coefficient select"]
    #[inline(always)]
    pub fn sw_bic_coe_sel(&self) -> SwBicCoeSelR {
        SwBicCoeSelR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sw_vsp_mode(&self) -> SwVspModeR {
        SwVspModeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - this bit valid when RGA support yuv 10bit picture input"]
    #[inline(always)]
    pub fn sw_src_yuv10_e(&self) -> SwSrcYuv10ER {
        SwSrcYuv10ER::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - this bit valid when RGA support yuv 10bit picture input"]
    #[inline(always)]
    pub fn sw_src_yuv10_round_e(&self) -> SwSrcYuv10RoundER {
        SwSrcYuv10RoundER::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Source bitmap data format"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_fmt(&mut self) -> SwSrcFmtW<SrcInfoSpec> {
        SwSrcFmtW::new(self, 0)
    }
    #[doc = "Bit 4 - Source bitmap data RB swap"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_rbswap(&mut self) -> SwSrcRbswapW<SrcInfoSpec> {
        SwSrcRbswapW::new(self, 4)
    }
    #[doc = "Bit 5 - Source bitmap data alpha swap"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_alpha_swap(&mut self) -> SwSrcAlphaSwapW<SrcInfoSpec> {
        SwSrcAlphaSwapW::new(self, 5)
    }
    #[doc = "Bit 6 - Source Cb-Cr swap"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_uvswap(&mut self) -> SwSrcUvswapW<SrcInfoSpec> {
        SwSrcUvswapW::new(self, 6)
    }
    #[doc = "Bit 7 - Source Color palette endian swap"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cp_endian(&mut self) -> SwCpEndianW<SrcInfoSpec> {
        SwCpEndianW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Source bitmap YUV2RGB conversion mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_csc_mode(&mut self) -> SwSrcCscModeW<SrcInfoSpec> {
        SwSrcCscModeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - SRC rotation mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_rot_mode(&mut self) -> SwSrcRotModeW<SrcInfoSpec> {
        SwSrcRotModeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - SRC mirror mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_mir_mode(&mut self) -> SwSrcMirModeW<SrcInfoSpec> {
        SwSrcMirModeW::new(self, 12)
    }
    #[doc = "Bits 14:15 - SRC horizontal scaling mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_hscl_mode(&mut self) -> SwSrcHsclModeW<SrcInfoSpec> {
        SwSrcHsclModeW::new(self, 14)
    }
    #[doc = "Bits 16:17 - SRC vertical scaling mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_vscl_mode(&mut self) -> SwSrcVsclModeW<SrcInfoSpec> {
        SwSrcVsclModeW::new(self, 16)
    }
    #[doc = "Bit 18 - Source transparency mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_trans_mode(&mut self) -> SwSrcTransModeW<SrcInfoSpec> {
        SwSrcTransModeW::new(self, 18)
    }
    #[doc = "Bits 19:22 - Source transparency enable bits\n\n\\[3\\]: A value stencil test enable bit\n\n\\[2\\]: B value stencil test enable bit\n\n\\[1\\]: G value stencil test enable bit\n\n\\[0\\]: R value stencil test enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_trans_e(&mut self) -> SwSrcTransEW<SrcInfoSpec> {
        SwSrcTransEW::new(self, 19)
    }
    #[doc = "Bit 23 - SRC dither up enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_dither_up(&mut self) -> SwSrcDitherUpW<SrcInfoSpec> {
        SwSrcDitherUpW::new(self, 23)
    }
    #[doc = "Bits 24:25 - SRC bicubic scaling coefficient select"]
    #[inline(always)]
    #[must_use]
    pub fn sw_bic_coe_sel(&mut self) -> SwBicCoeSelW<SrcInfoSpec> {
        SwBicCoeSelW::new(self, 24)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vsp_mode(&mut self) -> SwVspModeW<SrcInfoSpec> {
        SwVspModeW::new(self, 26)
    }
    #[doc = "Bit 27 - this bit valid when RGA support yuv 10bit picture input"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_yuv10_e(&mut self) -> SwSrcYuv10EW<SrcInfoSpec> {
        SwSrcYuv10EW::new(self, 27)
    }
    #[doc = "Bit 28 - this bit valid when RGA support yuv 10bit picture input"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_yuv10_round_e(&mut self) -> SwSrcYuv10RoundEW<SrcInfoSpec> {
        SwSrcYuv10RoundEW::new(self, 28)
    }
}
#[doc = "RGA source information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcInfoSpec;
impl crate::RegisterSpec for SrcInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_info::R`](R) reader structure"]
impl crate::Readable for SrcInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`src_info::W`](W) writer structure"]
impl crate::Writable for SrcInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_INFO to value 0"]
impl crate::Resettable for SrcInfoSpec {
    const RESET_VALUE: u32 = 0;
}
