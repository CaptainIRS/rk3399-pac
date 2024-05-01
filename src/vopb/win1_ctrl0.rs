#[doc = "Register `WIN1_CTRL0` reader"]
pub type R = crate::R<Win1Ctrl0Spec>;
#[doc = "Register `WIN1_CTRL0` writer"]
pub type W = crate::W<Win1Ctrl0Spec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1En {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win1En> for bool {
    #[inline(always)]
    fn from(variant: Win1En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_EN` reader - "]
pub type Win1EnR = crate::BitReader<Win1En>;
impl Win1EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1En {
        match self.bits {
            false => Win1En::B0,
            true => Win1En::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1En::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1En::B1
    }
}
#[doc = "Field `WIN1_EN` writer - "]
pub type Win1EnW<'a, REG> = crate::BitWriter<'a, REG, Win1En>;
impl<'a, REG> Win1EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1En::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1En::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win1DataFmt {
    #[doc = "0: ARGB888"]
    B000 = 0,
    #[doc = "1: RGB888"]
    B001 = 1,
    #[doc = "2: RGB565"]
    B010 = 2,
    #[doc = "4: YcbCr420"]
    B100 = 4,
    #[doc = "5: YcbCr422"]
    B101 = 5,
    #[doc = "6: YcbCr444"]
    B110 = 6,
}
impl From<Win1DataFmt> for u8 {
    #[inline(always)]
    fn from(variant: Win1DataFmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win1DataFmt {
    type Ux = u8;
}
#[doc = "Field `WIN1_DATA_FMT` reader - "]
pub type Win1DataFmtR = crate::FieldReader<Win1DataFmt>;
impl Win1DataFmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Win1DataFmt> {
        match self.bits {
            0 => Some(Win1DataFmt::B000),
            1 => Some(Win1DataFmt::B001),
            2 => Some(Win1DataFmt::B010),
            4 => Some(Win1DataFmt::B100),
            5 => Some(Win1DataFmt::B101),
            6 => Some(Win1DataFmt::B110),
            _ => None,
        }
    }
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Win1DataFmt::B000
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Win1DataFmt::B001
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Win1DataFmt::B010
    }
    #[doc = "YcbCr420"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Win1DataFmt::B100
    }
    #[doc = "YcbCr422"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Win1DataFmt::B101
    }
    #[doc = "YcbCr444"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Win1DataFmt::B110
    }
}
#[doc = "Field `WIN1_DATA_FMT` writer - "]
pub type Win1DataFmtW<'a, REG> = crate::FieldWriter<'a, REG, 3, Win1DataFmt>;
impl<'a, REG> Win1DataFmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Win1DataFmt::B000)
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Win1DataFmt::B001)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Win1DataFmt::B010)
    }
    #[doc = "YcbCr420"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Win1DataFmt::B100)
    }
    #[doc = "YcbCr422"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(Win1DataFmt::B101)
    }
    #[doc = "YcbCr444"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(Win1DataFmt::B110)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1Fmt10 {
    #[doc = "0: yuv 8bit fmt mode"]
    B0 = 0,
    #[doc = "1: yuv 10bit fmt mode"]
    B1 = 1,
}
impl From<Win1Fmt10> for bool {
    #[inline(always)]
    fn from(variant: Win1Fmt10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_FMT_10` reader - "]
pub type Win1Fmt10R = crate::BitReader<Win1Fmt10>;
impl Win1Fmt10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1Fmt10 {
        match self.bits {
            false => Win1Fmt10::B0,
            true => Win1Fmt10::B1,
        }
    }
    #[doc = "yuv 8bit fmt mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1Fmt10::B0
    }
    #[doc = "yuv 10bit fmt mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1Fmt10::B1
    }
}
#[doc = "Field `WIN1_FMT_10` writer - "]
pub type Win1Fmt10W<'a, REG> = crate::BitWriter<'a, REG, Win1Fmt10>;
impl<'a, REG> Win1Fmt10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "yuv 8bit fmt mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1Fmt10::B0)
    }
    #[doc = "yuv 10bit fmt mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1Fmt10::B1)
    }
}
#[doc = "Field `WIN1_LB_MODE` reader - win1 line buffer mode,calc by driver."]
pub type Win1LbModeR = crate::FieldReader;
#[doc = "Field `WIN1_LB_MODE` writer - win1 line buffer mode,calc by driver."]
pub type Win1LbModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Win1 interlace read mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1InterlaceRead {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win1InterlaceRead> for bool {
    #[inline(always)]
    fn from(variant: Win1InterlaceRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_INTERLACE_READ` reader - Win1 interlace read mode"]
pub type Win1InterlaceReadR = crate::BitReader<Win1InterlaceRead>;
impl Win1InterlaceReadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1InterlaceRead {
        match self.bits {
            false => Win1InterlaceRead::B0,
            true => Win1InterlaceRead::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1InterlaceRead::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1InterlaceRead::B1
    }
}
#[doc = "Field `WIN1_INTERLACE_READ` writer - Win1 interlace read mode"]
pub type Win1InterlaceReadW<'a, REG> = crate::BitWriter<'a, REG, Win1InterlaceRead>;
impl<'a, REG> Win1InterlaceReadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1InterlaceRead::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1InterlaceRead::B1)
    }
}
#[doc = "win1 AXI master read outstanding\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1NoOutstanding {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: disable"]
    B1 = 1,
}
impl From<Win1NoOutstanding> for bool {
    #[inline(always)]
    fn from(variant: Win1NoOutstanding) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_NO_OUTSTANDING` reader - win1 AXI master read outstanding"]
pub type Win1NoOutstandingR = crate::BitReader<Win1NoOutstanding>;
impl Win1NoOutstandingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1NoOutstanding {
        match self.bits {
            false => Win1NoOutstanding::B0,
            true => Win1NoOutstanding::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1NoOutstanding::B0
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1NoOutstanding::B1
    }
}
#[doc = "Field `WIN1_NO_OUTSTANDING` writer - win1 AXI master read outstanding"]
pub type Win1NoOutstandingW<'a, REG> = crate::BitWriter<'a, REG, Win1NoOutstanding>;
impl<'a, REG> Win1NoOutstandingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1NoOutstanding::B0)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1NoOutstanding::B1)
    }
}
#[doc = "Field `WIN1_CSC_MODE` reader - Win1 YUV2RGB or RGB2YUV\n\nColor space conversion(YUV2RGB):\n\n2'b00 : mpeg\n\n2'b01 : jpeg\n\n2'b10 : hd\n\n2'b11 : mpeg\n\nColor space conversion(RGB2YUV):\n\n2'bx0: BT601\n\n2'bx1: BT709"]
pub type Win1CscModeR = crate::FieldReader;
#[doc = "Field `WIN1_CSC_MODE` writer - Win1 YUV2RGB or RGB2YUV\n\nColor space conversion(YUV2RGB):\n\n2'b00 : mpeg\n\n2'b01 : jpeg\n\n2'b10 : hd\n\n2'b11 : mpeg\n\nColor space conversion(RGB2YUV):\n\n2'bx0: BT601\n\n2'bx1: BT709"]
pub type Win1CscModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "win1 RGB RED and BLUE swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1RbSwap {
    #[doc = "0: RGB"]
    B0 = 0,
    #[doc = "1: BGR"]
    B1 = 1,
}
impl From<Win1RbSwap> for bool {
    #[inline(always)]
    fn from(variant: Win1RbSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_RB_SWAP` reader - win1 RGB RED and BLUE swap"]
pub type Win1RbSwapR = crate::BitReader<Win1RbSwap>;
impl Win1RbSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1RbSwap {
        match self.bits {
            false => Win1RbSwap::B0,
            true => Win1RbSwap::B1,
        }
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1RbSwap::B0
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1RbSwap::B1
    }
}
#[doc = "Field `WIN1_RB_SWAP` writer - win1 RGB RED and BLUE swap"]
pub type Win1RbSwapW<'a, REG> = crate::BitWriter<'a, REG, Win1RbSwap>;
impl<'a, REG> Win1RbSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1RbSwap::B0)
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1RbSwap::B1)
    }
}
#[doc = "win1 alpha swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1AlphaSwap {
    #[doc = "0: ARGB"]
    B0 = 0,
    #[doc = "1: RGBA"]
    B1 = 1,
}
impl From<Win1AlphaSwap> for bool {
    #[inline(always)]
    fn from(variant: Win1AlphaSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_ALPHA_SWAP` reader - win1 alpha swap"]
pub type Win1AlphaSwapR = crate::BitReader<Win1AlphaSwap>;
impl Win1AlphaSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1AlphaSwap {
        match self.bits {
            false => Win1AlphaSwap::B0,
            true => Win1AlphaSwap::B1,
        }
    }
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1AlphaSwap::B0
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1AlphaSwap::B1
    }
}
#[doc = "Field `WIN1_ALPHA_SWAP` writer - win1 alpha swap"]
pub type Win1AlphaSwapW<'a, REG> = crate::BitWriter<'a, REG, Win1AlphaSwap>;
impl<'a, REG> Win1AlphaSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1AlphaSwap::B0)
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1AlphaSwap::B1)
    }
}
#[doc = "Win1 Y middle 8-bit swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1MidSwap {
    #[doc = "0: Y3Y2Y1Y0"]
    B0 = 0,
    #[doc = "1: Y3Y1Y2Y0"]
    B1 = 1,
}
impl From<Win1MidSwap> for bool {
    #[inline(always)]
    fn from(variant: Win1MidSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_MID_SWAP` reader - Win1 Y middle 8-bit swap"]
pub type Win1MidSwapR = crate::BitReader<Win1MidSwap>;
impl Win1MidSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1MidSwap {
        match self.bits {
            false => Win1MidSwap::B0,
            true => Win1MidSwap::B1,
        }
    }
    #[doc = "Y3Y2Y1Y0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1MidSwap::B0
    }
    #[doc = "Y3Y1Y2Y0"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1MidSwap::B1
    }
}
#[doc = "Field `WIN1_MID_SWAP` writer - Win1 Y middle 8-bit swap"]
pub type Win1MidSwapW<'a, REG> = crate::BitWriter<'a, REG, Win1MidSwap>;
impl<'a, REG> Win1MidSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Y3Y2Y1Y0"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1MidSwap::B0)
    }
    #[doc = "Y3Y1Y2Y0"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1MidSwap::B1)
    }
}
#[doc = "Win1 CbCr swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1UvSwap {
    #[doc = "0: CrCb"]
    B0 = 0,
    #[doc = "1: CbCr"]
    B1 = 1,
}
impl From<Win1UvSwap> for bool {
    #[inline(always)]
    fn from(variant: Win1UvSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_UV_SWAP` reader - Win1 CbCr swap"]
pub type Win1UvSwapR = crate::BitReader<Win1UvSwap>;
impl Win1UvSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1UvSwap {
        match self.bits {
            false => Win1UvSwap::B0,
            true => Win1UvSwap::B1,
        }
    }
    #[doc = "CrCb"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1UvSwap::B0
    }
    #[doc = "CbCr"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1UvSwap::B1
    }
}
#[doc = "Field `WIN1_UV_SWAP` writer - Win1 CbCr swap"]
pub type Win1UvSwapW<'a, REG> = crate::BitWriter<'a, REG, Win1UvSwap>;
impl<'a, REG> Win1UvSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CrCb"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1UvSwap::B0)
    }
    #[doc = "CbCr"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1UvSwap::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1HwPreMulEn {
    #[doc = "0: no hardware pre multiply mode"]
    B0 = 0,
    #[doc = "1: hardware pre multiply mode"]
    B1 = 1,
}
impl From<Win1HwPreMulEn> for bool {
    #[inline(always)]
    fn from(variant: Win1HwPreMulEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_HW_PRE_MUL_EN` reader - "]
pub type Win1HwPreMulEnR = crate::BitReader<Win1HwPreMulEn>;
impl Win1HwPreMulEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1HwPreMulEn {
        match self.bits {
            false => Win1HwPreMulEn::B0,
            true => Win1HwPreMulEn::B1,
        }
    }
    #[doc = "no hardware pre multiply mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1HwPreMulEn::B0
    }
    #[doc = "hardware pre multiply mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1HwPreMulEn::B1
    }
}
#[doc = "Field `WIN1_HW_PRE_MUL_EN` writer - "]
pub type Win1HwPreMulEnW<'a, REG> = crate::BitWriter<'a, REG, Win1HwPreMulEn>;
impl<'a, REG> Win1HwPreMulEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no hardware pre multiply mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1HwPreMulEn::B0)
    }
    #[doc = "hardware pre multiply mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1HwPreMulEn::B1)
    }
}
#[doc = "Field `WIN1_YUYV` reader - win1_data_fmt\\[3\\]"]
pub type Win1YuyvR = crate::BitReader;
#[doc = "Field `WIN1_YUYV` writer - win1_data_fmt\\[3\\]"]
pub type Win1YuyvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "win1 YRGB deflick mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1YrgbDeflick {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win1YrgbDeflick> for bool {
    #[inline(always)]
    fn from(variant: Win1YrgbDeflick) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_YRGB_DEFLICK` reader - win1 YRGB deflick mode"]
pub type Win1YrgbDeflickR = crate::BitReader<Win1YrgbDeflick>;
impl Win1YrgbDeflickR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1YrgbDeflick {
        match self.bits {
            false => Win1YrgbDeflick::B0,
            true => Win1YrgbDeflick::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1YrgbDeflick::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1YrgbDeflick::B1
    }
}
#[doc = "Field `WIN1_YRGB_DEFLICK` writer - win1 YRGB deflick mode"]
pub type Win1YrgbDeflickW<'a, REG> = crate::BitWriter<'a, REG, Win1YrgbDeflick>;
impl<'a, REG> Win1YrgbDeflickW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YrgbDeflick::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YrgbDeflick::B1)
    }
}
#[doc = "Win1 Cbr deflick mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1CbrDeflick {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win1CbrDeflick> for bool {
    #[inline(always)]
    fn from(variant: Win1CbrDeflick) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_CBR_DEFLICK` reader - Win1 Cbr deflick mode"]
pub type Win1CbrDeflickR = crate::BitReader<Win1CbrDeflick>;
impl Win1CbrDeflickR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1CbrDeflick {
        match self.bits {
            false => Win1CbrDeflick::B0,
            true => Win1CbrDeflick::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1CbrDeflick::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1CbrDeflick::B1
    }
}
#[doc = "Field `WIN1_CBR_DEFLICK` writer - Win1 Cbr deflick mode"]
pub type Win1CbrDeflickW<'a, REG> = crate::BitWriter<'a, REG, Win1CbrDeflick>;
impl<'a, REG> Win1CbrDeflickW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1CbrDeflick::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1CbrDeflick::B1)
    }
}
#[doc = "YCrCb clip\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1YuvClip {
    #[doc = "0: disable, YCbCr no clip"]
    B0 = 0,
    #[doc = "1: enable, YCbCr clip before YCbCr2RGB *Y clip: 16~235, CbCr clip: 16~239"]
    B1 = 1,
}
impl From<Win1YuvClip> for bool {
    #[inline(always)]
    fn from(variant: Win1YuvClip) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_YUV_CLIP` reader - YCrCb clip"]
pub type Win1YuvClipR = crate::BitReader<Win1YuvClip>;
impl Win1YuvClipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1YuvClip {
        match self.bits {
            false => Win1YuvClip::B0,
            true => Win1YuvClip::B1,
        }
    }
    #[doc = "disable, YCbCr no clip"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1YuvClip::B0
    }
    #[doc = "enable, YCbCr clip before YCbCr2RGB *Y clip: 16~235, CbCr clip: 16~239"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1YuvClip::B1
    }
}
#[doc = "Field `WIN1_YUV_CLIP` writer - YCrCb clip"]
pub type Win1YuvClipW<'a, REG> = crate::BitWriter<'a, REG, Win1YuvClip>;
impl<'a, REG> Win1YuvClipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable, YCbCr no clip"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YuvClip::B0)
    }
    #[doc = "enable, YCbCr clip before YCbCr2RGB *Y clip: 16~235, CbCr clip: 16~239"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YuvClip::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1XMirEn {
    #[doc = "0: no x_mirror"]
    B0 = 0,
    #[doc = "1: x_mirror"]
    B1 = 1,
}
impl From<Win1XMirEn> for bool {
    #[inline(always)]
    fn from(variant: Win1XMirEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_X_MIR_EN` reader - "]
pub type Win1XMirEnR = crate::BitReader<Win1XMirEn>;
impl Win1XMirEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1XMirEn {
        match self.bits {
            false => Win1XMirEn::B0,
            true => Win1XMirEn::B1,
        }
    }
    #[doc = "no x_mirror"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1XMirEn::B0
    }
    #[doc = "x_mirror"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1XMirEn::B1
    }
}
#[doc = "Field `WIN1_X_MIR_EN` writer - "]
pub type Win1XMirEnW<'a, REG> = crate::BitWriter<'a, REG, Win1XMirEn>;
impl<'a, REG> Win1XMirEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no x_mirror"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1XMirEn::B0)
    }
    #[doc = "x_mirror"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1XMirEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1YMirEn {
    #[doc = "0: no y_mirror"]
    B0 = 0,
    #[doc = "1: y_mirror"]
    B1 = 1,
}
impl From<Win1YMirEn> for bool {
    #[inline(always)]
    fn from(variant: Win1YMirEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_Y_MIR_EN` reader - "]
pub type Win1YMirEnR = crate::BitReader<Win1YMirEn>;
impl Win1YMirEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1YMirEn {
        match self.bits {
            false => Win1YMirEn::B0,
            true => Win1YMirEn::B1,
        }
    }
    #[doc = "no y_mirror"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1YMirEn::B0
    }
    #[doc = "y_mirror"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1YMirEn::B1
    }
}
#[doc = "Field `WIN1_Y_MIR_EN` writer - "]
pub type Win1YMirEnW<'a, REG> = crate::BitWriter<'a, REG, Win1YMirEn>;
impl<'a, REG> Win1YMirEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no y_mirror"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YMirEn::B0)
    }
    #[doc = "y_mirror"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YMirEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1AxiMaxOutstandingEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win1AxiMaxOutstandingEn> for bool {
    #[inline(always)]
    fn from(variant: Win1AxiMaxOutstandingEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_AXI_MAX_OUTSTANDING_EN` reader - "]
pub type Win1AxiMaxOutstandingEnR = crate::BitReader<Win1AxiMaxOutstandingEn>;
impl Win1AxiMaxOutstandingEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1AxiMaxOutstandingEn {
        match self.bits {
            false => Win1AxiMaxOutstandingEn::B0,
            true => Win1AxiMaxOutstandingEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1AxiMaxOutstandingEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1AxiMaxOutstandingEn::B1
    }
}
#[doc = "Field `WIN1_AXI_MAX_OUTSTANDING_EN` writer - "]
pub type Win1AxiMaxOutstandingEnW<'a, REG> = crate::BitWriter<'a, REG, Win1AxiMaxOutstandingEn>;
impl<'a, REG> Win1AxiMaxOutstandingEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1AxiMaxOutstandingEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1AxiMaxOutstandingEn::B1)
    }
}
#[doc = "Field `WIN1_AXI_MAX_OUTSTANDING_NUM` reader - win1 out standing max number"]
pub type Win1AxiMaxOutstandingNumR = crate::FieldReader;
#[doc = "Field `WIN1_AXI_MAX_OUTSTANDING_NUM` writer - win1 out standing max number"]
pub type Win1AxiMaxOutstandingNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "WIN1 DMA read Burst length\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win1DmaBurstLength {
    #[doc = "0: burst16 (burst 15 in rgb888 pack mode)"]
    B00 = 0,
    #[doc = "1: burst8 (burst 12 in rgb888 pack mode)"]
    B01 = 1,
    #[doc = "2: burst4 (burst 6 in rgb888 pack mode)"]
    B10 = 2,
}
impl From<Win1DmaBurstLength> for u8 {
    #[inline(always)]
    fn from(variant: Win1DmaBurstLength) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win1DmaBurstLength {
    type Ux = u8;
}
#[doc = "Field `WIN1_DMA_BURST_LENGTH` reader - WIN1 DMA read Burst length"]
pub type Win1DmaBurstLengthR = crate::FieldReader<Win1DmaBurstLength>;
impl Win1DmaBurstLengthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Win1DmaBurstLength> {
        match self.bits {
            0 => Some(Win1DmaBurstLength::B00),
            1 => Some(Win1DmaBurstLength::B01),
            2 => Some(Win1DmaBurstLength::B10),
            _ => None,
        }
    }
    #[doc = "burst16 (burst 15 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win1DmaBurstLength::B00
    }
    #[doc = "burst8 (burst 12 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win1DmaBurstLength::B01
    }
    #[doc = "burst4 (burst 6 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win1DmaBurstLength::B10
    }
}
#[doc = "Field `WIN1_DMA_BURST_LENGTH` writer - WIN1 DMA read Burst length"]
pub type Win1DmaBurstLengthW<'a, REG> = crate::FieldWriter<'a, REG, 2, Win1DmaBurstLength>;
impl<'a, REG> Win1DmaBurstLengthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "burst16 (burst 15 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win1DmaBurstLength::B00)
    }
    #[doc = "burst8 (burst 12 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win1DmaBurstLength::B01)
    }
    #[doc = "burst4 (burst 6 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win1DmaBurstLength::B10)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn win1_en(&self) -> Win1EnR {
        Win1EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn win1_data_fmt(&self) -> Win1DataFmtR {
        Win1DataFmtR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn win1_fmt_10(&self) -> Win1Fmt10R {
        Win1Fmt10R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - win1 line buffer mode,calc by driver."]
    #[inline(always)]
    pub fn win1_lb_mode(&self) -> Win1LbModeR {
        Win1LbModeR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Win1 interlace read mode"]
    #[inline(always)]
    pub fn win1_interlace_read(&self) -> Win1InterlaceReadR {
        Win1InterlaceReadR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - win1 AXI master read outstanding"]
    #[inline(always)]
    pub fn win1_no_outstanding(&self) -> Win1NoOutstandingR {
        Win1NoOutstandingR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Win1 YUV2RGB or RGB2YUV\n\nColor space conversion(YUV2RGB):\n\n2'b00 : mpeg\n\n2'b01 : jpeg\n\n2'b10 : hd\n\n2'b11 : mpeg\n\nColor space conversion(RGB2YUV):\n\n2'bx0: BT601\n\n2'bx1: BT709"]
    #[inline(always)]
    pub fn win1_csc_mode(&self) -> Win1CscModeR {
        Win1CscModeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - win1 RGB RED and BLUE swap"]
    #[inline(always)]
    pub fn win1_rb_swap(&self) -> Win1RbSwapR {
        Win1RbSwapR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - win1 alpha swap"]
    #[inline(always)]
    pub fn win1_alpha_swap(&self) -> Win1AlphaSwapR {
        Win1AlphaSwapR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Win1 Y middle 8-bit swap"]
    #[inline(always)]
    pub fn win1_mid_swap(&self) -> Win1MidSwapR {
        Win1MidSwapR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Win1 CbCr swap"]
    #[inline(always)]
    pub fn win1_uv_swap(&self) -> Win1UvSwapR {
        Win1UvSwapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn win1_hw_pre_mul_en(&self) -> Win1HwPreMulEnR {
        Win1HwPreMulEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - win1_data_fmt\\[3\\]"]
    #[inline(always)]
    pub fn win1_yuyv(&self) -> Win1YuyvR {
        Win1YuyvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - win1 YRGB deflick mode"]
    #[inline(always)]
    pub fn win1_yrgb_deflick(&self) -> Win1YrgbDeflickR {
        Win1YrgbDeflickR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Win1 Cbr deflick mode"]
    #[inline(always)]
    pub fn win1_cbr_deflick(&self) -> Win1CbrDeflickR {
        Win1CbrDeflickR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - YCrCb clip"]
    #[inline(always)]
    pub fn win1_yuv_clip(&self) -> Win1YuvClipR {
        Win1YuvClipR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn win1_x_mir_en(&self) -> Win1XMirEnR {
        Win1XMirEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn win1_y_mir_en(&self) -> Win1YMirEnR {
        Win1YMirEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn win1_axi_max_outstanding_en(&self) -> Win1AxiMaxOutstandingEnR {
        Win1AxiMaxOutstandingEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - win1 out standing max number"]
    #[inline(always)]
    pub fn win1_axi_max_outstanding_num(&self) -> Win1AxiMaxOutstandingNumR {
        Win1AxiMaxOutstandingNumR::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 30:31 - WIN1 DMA read Burst length"]
    #[inline(always)]
    pub fn win1_dma_burst_length(&self) -> Win1DmaBurstLengthR {
        Win1DmaBurstLengthR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn win1_en(&mut self) -> Win1EnW<Win1Ctrl0Spec> {
        Win1EnW::new(self, 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    #[must_use]
    pub fn win1_data_fmt(&mut self) -> Win1DataFmtW<Win1Ctrl0Spec> {
        Win1DataFmtW::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn win1_fmt_10(&mut self) -> Win1Fmt10W<Win1Ctrl0Spec> {
        Win1Fmt10W::new(self, 4)
    }
    #[doc = "Bits 5:7 - win1 line buffer mode,calc by driver."]
    #[inline(always)]
    #[must_use]
    pub fn win1_lb_mode(&mut self) -> Win1LbModeW<Win1Ctrl0Spec> {
        Win1LbModeW::new(self, 5)
    }
    #[doc = "Bit 8 - Win1 interlace read mode"]
    #[inline(always)]
    #[must_use]
    pub fn win1_interlace_read(&mut self) -> Win1InterlaceReadW<Win1Ctrl0Spec> {
        Win1InterlaceReadW::new(self, 8)
    }
    #[doc = "Bit 9 - win1 AXI master read outstanding"]
    #[inline(always)]
    #[must_use]
    pub fn win1_no_outstanding(&mut self) -> Win1NoOutstandingW<Win1Ctrl0Spec> {
        Win1NoOutstandingW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Win1 YUV2RGB or RGB2YUV\n\nColor space conversion(YUV2RGB):\n\n2'b00 : mpeg\n\n2'b01 : jpeg\n\n2'b10 : hd\n\n2'b11 : mpeg\n\nColor space conversion(RGB2YUV):\n\n2'bx0: BT601\n\n2'bx1: BT709"]
    #[inline(always)]
    #[must_use]
    pub fn win1_csc_mode(&mut self) -> Win1CscModeW<Win1Ctrl0Spec> {
        Win1CscModeW::new(self, 10)
    }
    #[doc = "Bit 12 - win1 RGB RED and BLUE swap"]
    #[inline(always)]
    #[must_use]
    pub fn win1_rb_swap(&mut self) -> Win1RbSwapW<Win1Ctrl0Spec> {
        Win1RbSwapW::new(self, 12)
    }
    #[doc = "Bit 13 - win1 alpha swap"]
    #[inline(always)]
    #[must_use]
    pub fn win1_alpha_swap(&mut self) -> Win1AlphaSwapW<Win1Ctrl0Spec> {
        Win1AlphaSwapW::new(self, 13)
    }
    #[doc = "Bit 14 - Win1 Y middle 8-bit swap"]
    #[inline(always)]
    #[must_use]
    pub fn win1_mid_swap(&mut self) -> Win1MidSwapW<Win1Ctrl0Spec> {
        Win1MidSwapW::new(self, 14)
    }
    #[doc = "Bit 15 - Win1 CbCr swap"]
    #[inline(always)]
    #[must_use]
    pub fn win1_uv_swap(&mut self) -> Win1UvSwapW<Win1Ctrl0Spec> {
        Win1UvSwapW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn win1_hw_pre_mul_en(&mut self) -> Win1HwPreMulEnW<Win1Ctrl0Spec> {
        Win1HwPreMulEnW::new(self, 16)
    }
    #[doc = "Bit 17 - win1_data_fmt\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn win1_yuyv(&mut self) -> Win1YuyvW<Win1Ctrl0Spec> {
        Win1YuyvW::new(self, 17)
    }
    #[doc = "Bit 18 - win1 YRGB deflick mode"]
    #[inline(always)]
    #[must_use]
    pub fn win1_yrgb_deflick(&mut self) -> Win1YrgbDeflickW<Win1Ctrl0Spec> {
        Win1YrgbDeflickW::new(self, 18)
    }
    #[doc = "Bit 19 - Win1 Cbr deflick mode"]
    #[inline(always)]
    #[must_use]
    pub fn win1_cbr_deflick(&mut self) -> Win1CbrDeflickW<Win1Ctrl0Spec> {
        Win1CbrDeflickW::new(self, 19)
    }
    #[doc = "Bit 20 - YCrCb clip"]
    #[inline(always)]
    #[must_use]
    pub fn win1_yuv_clip(&mut self) -> Win1YuvClipW<Win1Ctrl0Spec> {
        Win1YuvClipW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn win1_x_mir_en(&mut self) -> Win1XMirEnW<Win1Ctrl0Spec> {
        Win1XMirEnW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn win1_y_mir_en(&mut self) -> Win1YMirEnW<Win1Ctrl0Spec> {
        Win1YMirEnW::new(self, 22)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn win1_axi_max_outstanding_en(&mut self) -> Win1AxiMaxOutstandingEnW<Win1Ctrl0Spec> {
        Win1AxiMaxOutstandingEnW::new(self, 24)
    }
    #[doc = "Bits 25:29 - win1 out standing max number"]
    #[inline(always)]
    #[must_use]
    pub fn win1_axi_max_outstanding_num(&mut self) -> Win1AxiMaxOutstandingNumW<Win1Ctrl0Spec> {
        Win1AxiMaxOutstandingNumW::new(self, 25)
    }
    #[doc = "Bits 30:31 - WIN1 DMA read Burst length"]
    #[inline(always)]
    #[must_use]
    pub fn win1_dma_burst_length(&mut self) -> Win1DmaBurstLengthW<Win1Ctrl0Spec> {
        Win1DmaBurstLengthW::new(self, 30)
    }
}
#[doc = "Win1 ctrl register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win1Ctrl0Spec;
impl crate::RegisterSpec for Win1Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win1_ctrl0::R`](R) reader structure"]
impl crate::Readable for Win1Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`win1_ctrl0::W`](W) writer structure"]
impl crate::Writable for Win1Ctrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN1_CTRL0 to value 0x3a00_0040"]
impl crate::Resettable for Win1Ctrl0Spec {
    const RESET_VALUE: u32 = 0x3a00_0040;
}
