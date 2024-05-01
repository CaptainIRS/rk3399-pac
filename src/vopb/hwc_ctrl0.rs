#[doc = "Register `HWC_CTRL0` reader"]
pub type R = crate::R<HwcCtrl0Spec>;
#[doc = "Register `HWC_CTRL0` writer"]
pub type W = crate::W<HwcCtrl0Spec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwcEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<HwcEn> for bool {
    #[inline(always)]
    fn from(variant: HwcEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWC_EN` reader - "]
pub type HwcEnR = crate::BitReader<HwcEn>;
impl HwcEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwcEn {
        match self.bits {
            false => HwcEn::B0,
            true => HwcEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HwcEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HwcEn::B1
    }
}
#[doc = "Field `HWC_EN` writer - "]
pub type HwcEnW<'a, REG> = crate::BitWriter<'a, REG, HwcEn>;
impl<'a, REG> HwcEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HwcEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HwcEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HwcDataFmt {
    #[doc = "0: ARGB888"]
    B000 = 0,
    #[doc = "1: RGB888"]
    B001 = 1,
    #[doc = "2: RGB565"]
    B010 = 2,
    #[doc = "4: 8bpp"]
    B100 = 4,
    #[doc = "5: 4bpp"]
    B101 = 5,
    #[doc = "6: 2bpp"]
    B110 = 6,
    #[doc = "7: 1bpp"]
    B111 = 7,
}
impl From<HwcDataFmt> for u8 {
    #[inline(always)]
    fn from(variant: HwcDataFmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HwcDataFmt {
    type Ux = u8;
}
#[doc = "Field `HWC_DATA_FMT` reader - "]
pub type HwcDataFmtR = crate::FieldReader<HwcDataFmt>;
impl HwcDataFmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HwcDataFmt> {
        match self.bits {
            0 => Some(HwcDataFmt::B000),
            1 => Some(HwcDataFmt::B001),
            2 => Some(HwcDataFmt::B010),
            4 => Some(HwcDataFmt::B100),
            5 => Some(HwcDataFmt::B101),
            6 => Some(HwcDataFmt::B110),
            7 => Some(HwcDataFmt::B111),
            _ => None,
        }
    }
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == HwcDataFmt::B000
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == HwcDataFmt::B001
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == HwcDataFmt::B010
    }
    #[doc = "8bpp"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == HwcDataFmt::B100
    }
    #[doc = "4bpp"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == HwcDataFmt::B101
    }
    #[doc = "2bpp"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == HwcDataFmt::B110
    }
    #[doc = "1bpp"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == HwcDataFmt::B111
    }
}
#[doc = "Field `HWC_DATA_FMT` writer - "]
pub type HwcDataFmtW<'a, REG> = crate::FieldWriter<'a, REG, 3, HwcDataFmt>;
impl<'a, REG> HwcDataFmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(HwcDataFmt::B000)
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(HwcDataFmt::B001)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(HwcDataFmt::B010)
    }
    #[doc = "8bpp"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(HwcDataFmt::B100)
    }
    #[doc = "4bpp"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(HwcDataFmt::B101)
    }
    #[doc = "2bpp"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(HwcDataFmt::B110)
    }
    #[doc = "1bpp"]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(HwcDataFmt::B111)
    }
}
#[doc = "hwc color mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwcMode {
    #[doc = "0: normal color mode"]
    B0 = 0,
    #[doc = "1: reversed color mode"]
    B1 = 1,
}
impl From<HwcMode> for bool {
    #[inline(always)]
    fn from(variant: HwcMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWC_MODE` reader - hwc color mode"]
pub type HwcModeR = crate::BitReader<HwcMode>;
impl HwcModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwcMode {
        match self.bits {
            false => HwcMode::B0,
            true => HwcMode::B1,
        }
    }
    #[doc = "normal color mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HwcMode::B0
    }
    #[doc = "reversed color mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HwcMode::B1
    }
}
#[doc = "Field `HWC_MODE` writer - hwc color mode"]
pub type HwcModeW<'a, REG> = crate::BitWriter<'a, REG, HwcMode>;
impl<'a, REG> HwcModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal color mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HwcMode::B0)
    }
    #[doc = "reversed color mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HwcMode::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HwcSize {
    #[doc = "0: 32x32"]
    B00 = 0,
    #[doc = "1: 64x64"]
    B01 = 1,
    #[doc = "2: 96x96"]
    B10 = 2,
    #[doc = "3: 128x128"]
    B11 = 3,
}
impl From<HwcSize> for u8 {
    #[inline(always)]
    fn from(variant: HwcSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HwcSize {
    type Ux = u8;
}
#[doc = "Field `HWC_SIZE` reader - "]
pub type HwcSizeR = crate::FieldReader<HwcSize>;
impl HwcSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwcSize {
        match self.bits {
            0 => HwcSize::B00,
            1 => HwcSize::B01,
            2 => HwcSize::B10,
            3 => HwcSize::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "32x32"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == HwcSize::B00
    }
    #[doc = "64x64"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == HwcSize::B01
    }
    #[doc = "96x96"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == HwcSize::B10
    }
    #[doc = "128x128"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == HwcSize::B11
    }
}
#[doc = "Field `HWC_SIZE` writer - "]
pub type HwcSizeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, HwcSize>;
impl<'a, REG> HwcSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32x32"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(HwcSize::B00)
    }
    #[doc = "64x64"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(HwcSize::B01)
    }
    #[doc = "96x96"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(HwcSize::B10)
    }
    #[doc = "128x128"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(HwcSize::B11)
    }
}
#[doc = "hwc interlace read mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwcInterlaceRead {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<HwcInterlaceRead> for bool {
    #[inline(always)]
    fn from(variant: HwcInterlaceRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWC_INTERLACE_READ` reader - hwc interlace read mode"]
pub type HwcInterlaceReadR = crate::BitReader<HwcInterlaceRead>;
impl HwcInterlaceReadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwcInterlaceRead {
        match self.bits {
            false => HwcInterlaceRead::B0,
            true => HwcInterlaceRead::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HwcInterlaceRead::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HwcInterlaceRead::B1
    }
}
#[doc = "Field `HWC_INTERLACE_READ` writer - hwc interlace read mode"]
pub type HwcInterlaceReadW<'a, REG> = crate::BitWriter<'a, REG, HwcInterlaceRead>;
impl<'a, REG> HwcInterlaceReadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HwcInterlaceRead::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HwcInterlaceRead::B1)
    }
}
#[doc = "hwc RGB2YUV\n\nColor space conversion:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HwcCscMode {
    #[doc = "0: BT601_L"]
    B00 = 0,
    #[doc = "1: BT709_L"]
    B01 = 1,
    #[doc = "2: BT601_F"]
    B10 = 2,
    #[doc = "3: BT2020"]
    B11 = 3,
}
impl From<HwcCscMode> for u8 {
    #[inline(always)]
    fn from(variant: HwcCscMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HwcCscMode {
    type Ux = u8;
}
#[doc = "Field `HWC_CSC_MODE` reader - hwc RGB2YUV\n\nColor space conversion:"]
pub type HwcCscModeR = crate::FieldReader<HwcCscMode>;
impl HwcCscModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwcCscMode {
        match self.bits {
            0 => HwcCscMode::B00,
            1 => HwcCscMode::B01,
            2 => HwcCscMode::B10,
            3 => HwcCscMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "BT601_L"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == HwcCscMode::B00
    }
    #[doc = "BT709_L"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == HwcCscMode::B01
    }
    #[doc = "BT601_F"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == HwcCscMode::B10
    }
    #[doc = "BT2020"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == HwcCscMode::B11
    }
}
#[doc = "Field `HWC_CSC_MODE` writer - hwc RGB2YUV\n\nColor space conversion:"]
pub type HwcCscModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, HwcCscMode>;
impl<'a, REG> HwcCscModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BT601_L"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(HwcCscMode::B00)
    }
    #[doc = "BT709_L"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(HwcCscMode::B01)
    }
    #[doc = "BT601_F"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(HwcCscMode::B10)
    }
    #[doc = "BT2020"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(HwcCscMode::B11)
    }
}
#[doc = "hwc RGB Red and Blue swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwcRbSwap {
    #[doc = "0: RGB"]
    B0 = 0,
    #[doc = "1: BGR"]
    B1 = 1,
}
impl From<HwcRbSwap> for bool {
    #[inline(always)]
    fn from(variant: HwcRbSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWC_RB_SWAP` reader - hwc RGB Red and Blue swap"]
pub type HwcRbSwapR = crate::BitReader<HwcRbSwap>;
impl HwcRbSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwcRbSwap {
        match self.bits {
            false => HwcRbSwap::B0,
            true => HwcRbSwap::B1,
        }
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HwcRbSwap::B0
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HwcRbSwap::B1
    }
}
#[doc = "Field `HWC_RB_SWAP` writer - hwc RGB Red and Blue swap"]
pub type HwcRbSwapW<'a, REG> = crate::BitWriter<'a, REG, HwcRbSwap>;
impl<'a, REG> HwcRbSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HwcRbSwap::B0)
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HwcRbSwap::B1)
    }
}
#[doc = "hwc RGB alpha swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwcAlphaSwap {
    #[doc = "0: ARGB"]
    B0 = 0,
    #[doc = "1: RGBA"]
    B1 = 1,
}
impl From<HwcAlphaSwap> for bool {
    #[inline(always)]
    fn from(variant: HwcAlphaSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWC_ALPHA_SWAP` reader - hwc RGB alpha swap"]
pub type HwcAlphaSwapR = crate::BitReader<HwcAlphaSwap>;
impl HwcAlphaSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwcAlphaSwap {
        match self.bits {
            false => HwcAlphaSwap::B0,
            true => HwcAlphaSwap::B1,
        }
    }
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HwcAlphaSwap::B0
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HwcAlphaSwap::B1
    }
}
#[doc = "Field `HWC_ALPHA_SWAP` writer - hwc RGB alpha swap"]
pub type HwcAlphaSwapW<'a, REG> = crate::BitWriter<'a, REG, HwcAlphaSwap>;
impl<'a, REG> HwcAlphaSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HwcAlphaSwap::B0)
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HwcAlphaSwap::B1)
    }
}
#[doc = "hwc 8pp palette data Big-endian/ Little-endian select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwcEndianSwap {
    #[doc = "0: Big-endian"]
    B0 = 0,
    #[doc = "1: Little-endian"]
    B1 = 1,
}
impl From<HwcEndianSwap> for bool {
    #[inline(always)]
    fn from(variant: HwcEndianSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWC_ENDIAN_SWAP` reader - hwc 8pp palette data Big-endian/ Little-endian select"]
pub type HwcEndianSwapR = crate::BitReader<HwcEndianSwap>;
impl HwcEndianSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwcEndianSwap {
        match self.bits {
            false => HwcEndianSwap::B0,
            true => HwcEndianSwap::B1,
        }
    }
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HwcEndianSwap::B0
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HwcEndianSwap::B1
    }
}
#[doc = "Field `HWC_ENDIAN_SWAP` writer - hwc 8pp palette data Big-endian/ Little-endian select"]
pub type HwcEndianSwapW<'a, REG> = crate::BitWriter<'a, REG, HwcEndianSwap>;
impl<'a, REG> HwcEndianSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HwcEndianSwap::B0)
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HwcEndianSwap::B1)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hwc_en(&self) -> HwcEnR {
        HwcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn hwc_data_fmt(&self) -> HwcDataFmtR {
        HwcDataFmtR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - hwc color mode"]
    #[inline(always)]
    pub fn hwc_mode(&self) -> HwcModeR {
        HwcModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn hwc_size(&self) -> HwcSizeR {
        HwcSizeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - hwc interlace read mode"]
    #[inline(always)]
    pub fn hwc_interlace_read(&self) -> HwcInterlaceReadR {
        HwcInterlaceReadR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - hwc RGB2YUV\n\nColor space conversion:"]
    #[inline(always)]
    pub fn hwc_csc_mode(&self) -> HwcCscModeR {
        HwcCscModeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - hwc RGB Red and Blue swap"]
    #[inline(always)]
    pub fn hwc_rb_swap(&self) -> HwcRbSwapR {
        HwcRbSwapR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - hwc RGB alpha swap"]
    #[inline(always)]
    pub fn hwc_alpha_swap(&self) -> HwcAlphaSwapR {
        HwcAlphaSwapR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - hwc 8pp palette data Big-endian/ Little-endian select"]
    #[inline(always)]
    pub fn hwc_endian_swap(&self) -> HwcEndianSwapR {
        HwcEndianSwapR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_en(&mut self) -> HwcEnW<HwcCtrl0Spec> {
        HwcEnW::new(self, 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_data_fmt(&mut self) -> HwcDataFmtW<HwcCtrl0Spec> {
        HwcDataFmtW::new(self, 1)
    }
    #[doc = "Bit 4 - hwc color mode"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_mode(&mut self) -> HwcModeW<HwcCtrl0Spec> {
        HwcModeW::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_size(&mut self) -> HwcSizeW<HwcCtrl0Spec> {
        HwcSizeW::new(self, 5)
    }
    #[doc = "Bit 8 - hwc interlace read mode"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_interlace_read(&mut self) -> HwcInterlaceReadW<HwcCtrl0Spec> {
        HwcInterlaceReadW::new(self, 8)
    }
    #[doc = "Bits 10:11 - hwc RGB2YUV\n\nColor space conversion:"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_csc_mode(&mut self) -> HwcCscModeW<HwcCtrl0Spec> {
        HwcCscModeW::new(self, 10)
    }
    #[doc = "Bit 12 - hwc RGB Red and Blue swap"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_rb_swap(&mut self) -> HwcRbSwapW<HwcCtrl0Spec> {
        HwcRbSwapW::new(self, 12)
    }
    #[doc = "Bit 13 - hwc RGB alpha swap"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_alpha_swap(&mut self) -> HwcAlphaSwapW<HwcCtrl0Spec> {
        HwcAlphaSwapW::new(self, 13)
    }
    #[doc = "Bit 14 - hwc 8pp palette data Big-endian/ Little-endian select"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_endian_swap(&mut self) -> HwcEndianSwapW<HwcCtrl0Spec> {
        HwcEndianSwapW::new(self, 14)
    }
}
#[doc = "Hwc ctrl register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwc_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwc_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwcCtrl0Spec;
impl crate::RegisterSpec for HwcCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwc_ctrl0::R`](R) reader structure"]
impl crate::Readable for HwcCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`hwc_ctrl0::W`](W) writer structure"]
impl crate::Writable for HwcCtrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWC_CTRL0 to value 0"]
impl crate::Resettable for HwcCtrl0Spec {
    const RESET_VALUE: u32 = 0;
}
