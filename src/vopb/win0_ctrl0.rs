#[doc = "Register `WIN0_CTRL0` reader"]
pub type R = crate::R<Win0Ctrl0Spec>;
#[doc = "Register `WIN0_CTRL0` writer"]
pub type W = crate::W<Win0Ctrl0Spec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0En {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win0En> for bool {
    #[inline(always)]
    fn from(variant: Win0En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_EN` reader - "]
pub type Win0EnR = crate::BitReader<Win0En>;
impl Win0EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0En {
        match self.bits {
            false => Win0En::B0,
            true => Win0En::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0En::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0En::B1
    }
}
#[doc = "Field `WIN0_EN` writer - "]
pub type Win0EnW<'a, REG> = crate::BitWriter<'a, REG, Win0En>;
impl<'a, REG> Win0EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0En::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0En::B1)
    }
}
#[doc = "Field `WIN0_DATA_FMT` reader - vld_reg\n\n4'b0000 : ARGB888\n\n4'b0001 : RGB888\n\n4'b0010 : RGB565\n\n4'b0100 : YcbCr420\n\n4'b0101 : YcbCr422\n\n4'b0110 : YcbCr444\n\n4'b1000: YCrYCb422\n\n4'b1001: YCrYCb420\n\n4'b1010: CrYCbY422\n\n4'b1011: CrYCbY420"]
pub type Win0DataFmtR = crate::FieldReader;
#[doc = "Field `WIN0_DATA_FMT` writer - vld_reg\n\n4'b0000 : ARGB888\n\n4'b0001 : RGB888\n\n4'b0010 : RGB565\n\n4'b0100 : YcbCr420\n\n4'b0101 : YcbCr422\n\n4'b0110 : YcbCr444\n\n4'b1000: YCrYCb422\n\n4'b1001: YCrYCb420\n\n4'b1010: CrYCbY422\n\n4'b1011: CrYCbY420"]
pub type Win0DataFmtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0Fmt10 {
    #[doc = "0: yuv 8bit fmt mode"]
    B0 = 0,
    #[doc = "1: yuv 10bit fmt mode"]
    B1 = 1,
}
impl From<Win0Fmt10> for bool {
    #[inline(always)]
    fn from(variant: Win0Fmt10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_FMT_10` reader - "]
pub type Win0Fmt10R = crate::BitReader<Win0Fmt10>;
impl Win0Fmt10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0Fmt10 {
        match self.bits {
            false => Win0Fmt10::B0,
            true => Win0Fmt10::B1,
        }
    }
    #[doc = "yuv 8bit fmt mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0Fmt10::B0
    }
    #[doc = "yuv 10bit fmt mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0Fmt10::B1
    }
}
#[doc = "Field `WIN0_FMT_10` writer - "]
pub type Win0Fmt10W<'a, REG> = crate::BitWriter<'a, REG, Win0Fmt10>;
impl<'a, REG> Win0Fmt10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "yuv 8bit fmt mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0Fmt10::B0)
    }
    #[doc = "yuv 10bit fmt mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0Fmt10::B1)
    }
}
#[doc = "Field `WIN0_LB_MODE` reader - win0 line buffer mode,calc by driver."]
pub type Win0LbModeR = crate::FieldReader;
#[doc = "Field `WIN0_LB_MODE` writer - win0 line buffer mode,calc by driver."]
pub type Win0LbModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Win0 interlace read mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0InterlaceRead {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win0InterlaceRead> for bool {
    #[inline(always)]
    fn from(variant: Win0InterlaceRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_INTERLACE_READ` reader - Win0 interlace read mode"]
pub type Win0InterlaceReadR = crate::BitReader<Win0InterlaceRead>;
impl Win0InterlaceReadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0InterlaceRead {
        match self.bits {
            false => Win0InterlaceRead::B0,
            true => Win0InterlaceRead::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0InterlaceRead::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0InterlaceRead::B1
    }
}
#[doc = "Field `WIN0_INTERLACE_READ` writer - Win0 interlace read mode"]
pub type Win0InterlaceReadW<'a, REG> = crate::BitWriter<'a, REG, Win0InterlaceRead>;
impl<'a, REG> Win0InterlaceReadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0InterlaceRead::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0InterlaceRead::B1)
    }
}
#[doc = "win0 AXI master read outstanding\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0NoOutstanding {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: disable"]
    B1 = 1,
}
impl From<Win0NoOutstanding> for bool {
    #[inline(always)]
    fn from(variant: Win0NoOutstanding) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_NO_OUTSTANDING` reader - win0 AXI master read outstanding"]
pub type Win0NoOutstandingR = crate::BitReader<Win0NoOutstanding>;
impl Win0NoOutstandingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0NoOutstanding {
        match self.bits {
            false => Win0NoOutstanding::B0,
            true => Win0NoOutstanding::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0NoOutstanding::B0
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0NoOutstanding::B1
    }
}
#[doc = "Field `WIN0_NO_OUTSTANDING` writer - win0 AXI master read outstanding"]
pub type Win0NoOutstandingW<'a, REG> = crate::BitWriter<'a, REG, Win0NoOutstanding>;
impl<'a, REG> Win0NoOutstandingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0NoOutstanding::B0)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0NoOutstanding::B1)
    }
}
#[doc = "Field `WIN0_CSC_MODE` reader - Win0 YUV2RGB or RGB2YUV\n\nColor space conversion(YUV2RGB):\n\n2'b00 : mpeg\n\n2'b01 : jpeg\n\n2'b10 : hd\n\n2'b11 : mpeg\n\nColor space conversion(RGB2YUV):\n\n2'bx0: BT601\n\n2'bx1: BT709"]
pub type Win0CscModeR = crate::FieldReader;
#[doc = "Field `WIN0_CSC_MODE` writer - Win0 YUV2RGB or RGB2YUV\n\nColor space conversion(YUV2RGB):\n\n2'b00 : mpeg\n\n2'b01 : jpeg\n\n2'b10 : hd\n\n2'b11 : mpeg\n\nColor space conversion(RGB2YUV):\n\n2'bx0: BT601\n\n2'bx1: BT709"]
pub type Win0CscModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "win0 RGB RED and BLUE swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0RbSwap {
    #[doc = "0: RGB"]
    B0 = 0,
    #[doc = "1: BGR"]
    B1 = 1,
}
impl From<Win0RbSwap> for bool {
    #[inline(always)]
    fn from(variant: Win0RbSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_RB_SWAP` reader - win0 RGB RED and BLUE swap"]
pub type Win0RbSwapR = crate::BitReader<Win0RbSwap>;
impl Win0RbSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0RbSwap {
        match self.bits {
            false => Win0RbSwap::B0,
            true => Win0RbSwap::B1,
        }
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0RbSwap::B0
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0RbSwap::B1
    }
}
#[doc = "Field `WIN0_RB_SWAP` writer - win0 RGB RED and BLUE swap"]
pub type Win0RbSwapW<'a, REG> = crate::BitWriter<'a, REG, Win0RbSwap>;
impl<'a, REG> Win0RbSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0RbSwap::B0)
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0RbSwap::B1)
    }
}
#[doc = "win0 alpha swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0AlphaSwap {
    #[doc = "0: ARGB"]
    B0 = 0,
    #[doc = "1: RGBA"]
    B1 = 1,
}
impl From<Win0AlphaSwap> for bool {
    #[inline(always)]
    fn from(variant: Win0AlphaSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_ALPHA_SWAP` reader - win0 alpha swap"]
pub type Win0AlphaSwapR = crate::BitReader<Win0AlphaSwap>;
impl Win0AlphaSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0AlphaSwap {
        match self.bits {
            false => Win0AlphaSwap::B0,
            true => Win0AlphaSwap::B1,
        }
    }
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0AlphaSwap::B0
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0AlphaSwap::B1
    }
}
#[doc = "Field `WIN0_ALPHA_SWAP` writer - win0 alpha swap"]
pub type Win0AlphaSwapW<'a, REG> = crate::BitWriter<'a, REG, Win0AlphaSwap>;
impl<'a, REG> Win0AlphaSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0AlphaSwap::B0)
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0AlphaSwap::B1)
    }
}
#[doc = "Win0 Y middle swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0MidSwap {
    #[doc = "0: Y3Y2Y1Y0"]
    B0 = 0,
    #[doc = "1: Y3Y1Y2Y0"]
    B1 = 1,
}
impl From<Win0MidSwap> for bool {
    #[inline(always)]
    fn from(variant: Win0MidSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_MID_SWAP` reader - Win0 Y middle swap"]
pub type Win0MidSwapR = crate::BitReader<Win0MidSwap>;
impl Win0MidSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0MidSwap {
        match self.bits {
            false => Win0MidSwap::B0,
            true => Win0MidSwap::B1,
        }
    }
    #[doc = "Y3Y2Y1Y0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0MidSwap::B0
    }
    #[doc = "Y3Y1Y2Y0"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0MidSwap::B1
    }
}
#[doc = "Field `WIN0_MID_SWAP` writer - Win0 Y middle swap"]
pub type Win0MidSwapW<'a, REG> = crate::BitWriter<'a, REG, Win0MidSwap>;
impl<'a, REG> Win0MidSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Y3Y2Y1Y0"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0MidSwap::B0)
    }
    #[doc = "Y3Y1Y2Y0"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0MidSwap::B1)
    }
}
#[doc = "Win0 CbCr swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0UvSwap {
    #[doc = "0: CrCb"]
    B0 = 0,
    #[doc = "1: CbCr"]
    B1 = 1,
}
impl From<Win0UvSwap> for bool {
    #[inline(always)]
    fn from(variant: Win0UvSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_UV_SWAP` reader - Win0 CbCr swap"]
pub type Win0UvSwapR = crate::BitReader<Win0UvSwap>;
impl Win0UvSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0UvSwap {
        match self.bits {
            false => Win0UvSwap::B0,
            true => Win0UvSwap::B1,
        }
    }
    #[doc = "CrCb"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0UvSwap::B0
    }
    #[doc = "CbCr"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0UvSwap::B1
    }
}
#[doc = "Field `WIN0_UV_SWAP` writer - Win0 CbCr swap"]
pub type Win0UvSwapW<'a, REG> = crate::BitWriter<'a, REG, Win0UvSwap>;
impl<'a, REG> Win0UvSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CrCb"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0UvSwap::B0)
    }
    #[doc = "CbCr"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0UvSwap::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0HwPreMulEn {
    #[doc = "0: no hardware pre multiply mode"]
    B0 = 0,
    #[doc = "1: hardware pre multiply mode"]
    B1 = 1,
}
impl From<Win0HwPreMulEn> for bool {
    #[inline(always)]
    fn from(variant: Win0HwPreMulEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_HW_PRE_MUL_EN` reader - "]
pub type Win0HwPreMulEnR = crate::BitReader<Win0HwPreMulEn>;
impl Win0HwPreMulEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0HwPreMulEn {
        match self.bits {
            false => Win0HwPreMulEn::B0,
            true => Win0HwPreMulEn::B1,
        }
    }
    #[doc = "no hardware pre multiply mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0HwPreMulEn::B0
    }
    #[doc = "hardware pre multiply mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0HwPreMulEn::B1
    }
}
#[doc = "Field `WIN0_HW_PRE_MUL_EN` writer - "]
pub type Win0HwPreMulEnW<'a, REG> = crate::BitWriter<'a, REG, Win0HwPreMulEn>;
impl<'a, REG> Win0HwPreMulEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no hardware pre multiply mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0HwPreMulEn::B0)
    }
    #[doc = "hardware pre multiply mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0HwPreMulEn::B1)
    }
}
#[doc = "Field `WIN0_YUYV` reader - win0_data_fmt\\[3\\]"]
pub type Win0YuyvR = crate::BitReader;
#[doc = "Field `WIN0_YUYV` writer - win0_data_fmt\\[3\\]"]
pub type Win0YuyvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "win0 YRGB deflick mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0YrgbDeflick {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win0YrgbDeflick> for bool {
    #[inline(always)]
    fn from(variant: Win0YrgbDeflick) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_YRGB_DEFLICK` reader - win0 YRGB deflick mode"]
pub type Win0YrgbDeflickR = crate::BitReader<Win0YrgbDeflick>;
impl Win0YrgbDeflickR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0YrgbDeflick {
        match self.bits {
            false => Win0YrgbDeflick::B0,
            true => Win0YrgbDeflick::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0YrgbDeflick::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0YrgbDeflick::B1
    }
}
#[doc = "Field `WIN0_YRGB_DEFLICK` writer - win0 YRGB deflick mode"]
pub type Win0YrgbDeflickW<'a, REG> = crate::BitWriter<'a, REG, Win0YrgbDeflick>;
impl<'a, REG> Win0YrgbDeflickW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YrgbDeflick::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YrgbDeflick::B1)
    }
}
#[doc = "Win0 Cbr deflick mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0CbrDeflick {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win0CbrDeflick> for bool {
    #[inline(always)]
    fn from(variant: Win0CbrDeflick) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_CBR_DEFLICK` reader - Win0 Cbr deflick mode"]
pub type Win0CbrDeflickR = crate::BitReader<Win0CbrDeflick>;
impl Win0CbrDeflickR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0CbrDeflick {
        match self.bits {
            false => Win0CbrDeflick::B0,
            true => Win0CbrDeflick::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0CbrDeflick::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0CbrDeflick::B1
    }
}
#[doc = "Field `WIN0_CBR_DEFLICK` writer - Win0 Cbr deflick mode"]
pub type Win0CbrDeflickW<'a, REG> = crate::BitWriter<'a, REG, Win0CbrDeflick>;
impl<'a, REG> Win0CbrDeflickW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0CbrDeflick::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0CbrDeflick::B1)
    }
}
#[doc = "YCrCb clip\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0YuvClip {
    #[doc = "0: disable, YCbCr no clip"]
    B0 = 0,
    #[doc = "1: enable, YCbCr clip before YCbCr2RGB *Y clip: 16~235, CbCr clip: 16~239"]
    B1 = 1,
}
impl From<Win0YuvClip> for bool {
    #[inline(always)]
    fn from(variant: Win0YuvClip) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_YUV_CLIP` reader - YCrCb clip"]
pub type Win0YuvClipR = crate::BitReader<Win0YuvClip>;
impl Win0YuvClipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0YuvClip {
        match self.bits {
            false => Win0YuvClip::B0,
            true => Win0YuvClip::B1,
        }
    }
    #[doc = "disable, YCbCr no clip"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0YuvClip::B0
    }
    #[doc = "enable, YCbCr clip before YCbCr2RGB *Y clip: 16~235, CbCr clip: 16~239"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0YuvClip::B1
    }
}
#[doc = "Field `WIN0_YUV_CLIP` writer - YCrCb clip"]
pub type Win0YuvClipW<'a, REG> = crate::BitWriter<'a, REG, Win0YuvClip>;
impl<'a, REG> Win0YuvClipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable, YCbCr no clip"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YuvClip::B0)
    }
    #[doc = "enable, YCbCr clip before YCbCr2RGB *Y clip: 16~235, CbCr clip: 16~239"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YuvClip::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0XMirEn {
    #[doc = "0: no x_mirror"]
    B0 = 0,
    #[doc = "1: x_mirror"]
    B1 = 1,
}
impl From<Win0XMirEn> for bool {
    #[inline(always)]
    fn from(variant: Win0XMirEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_X_MIR_EN` reader - "]
pub type Win0XMirEnR = crate::BitReader<Win0XMirEn>;
impl Win0XMirEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0XMirEn {
        match self.bits {
            false => Win0XMirEn::B0,
            true => Win0XMirEn::B1,
        }
    }
    #[doc = "no x_mirror"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0XMirEn::B0
    }
    #[doc = "x_mirror"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0XMirEn::B1
    }
}
#[doc = "Field `WIN0_X_MIR_EN` writer - "]
pub type Win0XMirEnW<'a, REG> = crate::BitWriter<'a, REG, Win0XMirEn>;
impl<'a, REG> Win0XMirEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no x_mirror"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0XMirEn::B0)
    }
    #[doc = "x_mirror"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0XMirEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0YMirEn {
    #[doc = "0: no y_mirror"]
    B0 = 0,
    #[doc = "1: y_mirror"]
    B1 = 1,
}
impl From<Win0YMirEn> for bool {
    #[inline(always)]
    fn from(variant: Win0YMirEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_Y_MIR_EN` reader - "]
pub type Win0YMirEnR = crate::BitReader<Win0YMirEn>;
impl Win0YMirEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0YMirEn {
        match self.bits {
            false => Win0YMirEn::B0,
            true => Win0YMirEn::B1,
        }
    }
    #[doc = "no y_mirror"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0YMirEn::B0
    }
    #[doc = "y_mirror"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0YMirEn::B1
    }
}
#[doc = "Field `WIN0_Y_MIR_EN` writer - "]
pub type Win0YMirEnW<'a, REG> = crate::BitWriter<'a, REG, Win0YMirEn>;
impl<'a, REG> Win0YMirEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no y_mirror"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YMirEn::B0)
    }
    #[doc = "y_mirror"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YMirEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0AxiMaxOutstandingEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win0AxiMaxOutstandingEn> for bool {
    #[inline(always)]
    fn from(variant: Win0AxiMaxOutstandingEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_AXI_MAX_OUTSTANDING_EN` reader - "]
pub type Win0AxiMaxOutstandingEnR = crate::BitReader<Win0AxiMaxOutstandingEn>;
impl Win0AxiMaxOutstandingEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0AxiMaxOutstandingEn {
        match self.bits {
            false => Win0AxiMaxOutstandingEn::B0,
            true => Win0AxiMaxOutstandingEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0AxiMaxOutstandingEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0AxiMaxOutstandingEn::B1
    }
}
#[doc = "Field `WIN0_AXI_MAX_OUTSTANDING_EN` writer - "]
pub type Win0AxiMaxOutstandingEnW<'a, REG> = crate::BitWriter<'a, REG, Win0AxiMaxOutstandingEn>;
impl<'a, REG> Win0AxiMaxOutstandingEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0AxiMaxOutstandingEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0AxiMaxOutstandingEn::B1)
    }
}
#[doc = "Field `WIN0_AXI_OUTSTANDING_MAX_NUM` reader - win0 out standing max number"]
pub type Win0AxiOutstandingMaxNumR = crate::FieldReader;
#[doc = "Field `WIN0_AXI_OUTSTANDING_MAX_NUM` writer - win0 out standing max number"]
pub type Win0AxiOutstandingMaxNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "WIN0 DMA read Burst length\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win0DmaBurstLength {
    #[doc = "0: burst16 (burst 15 in rgb888 pack mode)"]
    B00 = 0,
    #[doc = "1: burst8 (burst 12 in rgb888 pack mode)"]
    B01 = 1,
    #[doc = "2: burst4 (burst 6 in rgb888 pack mode)"]
    B10 = 2,
}
impl From<Win0DmaBurstLength> for u8 {
    #[inline(always)]
    fn from(variant: Win0DmaBurstLength) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win0DmaBurstLength {
    type Ux = u8;
}
#[doc = "Field `WIN0_DMA_BURST_LENGTH` reader - WIN0 DMA read Burst length"]
pub type Win0DmaBurstLengthR = crate::FieldReader<Win0DmaBurstLength>;
impl Win0DmaBurstLengthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Win0DmaBurstLength> {
        match self.bits {
            0 => Some(Win0DmaBurstLength::B00),
            1 => Some(Win0DmaBurstLength::B01),
            2 => Some(Win0DmaBurstLength::B10),
            _ => None,
        }
    }
    #[doc = "burst16 (burst 15 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win0DmaBurstLength::B00
    }
    #[doc = "burst8 (burst 12 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win0DmaBurstLength::B01
    }
    #[doc = "burst4 (burst 6 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win0DmaBurstLength::B10
    }
}
#[doc = "Field `WIN0_DMA_BURST_LENGTH` writer - WIN0 DMA read Burst length"]
pub type Win0DmaBurstLengthW<'a, REG> = crate::FieldWriter<'a, REG, 2, Win0DmaBurstLength>;
impl<'a, REG> Win0DmaBurstLengthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "burst16 (burst 15 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win0DmaBurstLength::B00)
    }
    #[doc = "burst8 (burst 12 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win0DmaBurstLength::B01)
    }
    #[doc = "burst4 (burst 6 in rgb888 pack mode)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win0DmaBurstLength::B10)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn win0_en(&self) -> Win0EnR {
        Win0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - vld_reg\n\n4'b0000 : ARGB888\n\n4'b0001 : RGB888\n\n4'b0010 : RGB565\n\n4'b0100 : YcbCr420\n\n4'b0101 : YcbCr422\n\n4'b0110 : YcbCr444\n\n4'b1000: YCrYCb422\n\n4'b1001: YCrYCb420\n\n4'b1010: CrYCbY422\n\n4'b1011: CrYCbY420"]
    #[inline(always)]
    pub fn win0_data_fmt(&self) -> Win0DataFmtR {
        Win0DataFmtR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn win0_fmt_10(&self) -> Win0Fmt10R {
        Win0Fmt10R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - win0 line buffer mode,calc by driver."]
    #[inline(always)]
    pub fn win0_lb_mode(&self) -> Win0LbModeR {
        Win0LbModeR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Win0 interlace read mode"]
    #[inline(always)]
    pub fn win0_interlace_read(&self) -> Win0InterlaceReadR {
        Win0InterlaceReadR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - win0 AXI master read outstanding"]
    #[inline(always)]
    pub fn win0_no_outstanding(&self) -> Win0NoOutstandingR {
        Win0NoOutstandingR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Win0 YUV2RGB or RGB2YUV\n\nColor space conversion(YUV2RGB):\n\n2'b00 : mpeg\n\n2'b01 : jpeg\n\n2'b10 : hd\n\n2'b11 : mpeg\n\nColor space conversion(RGB2YUV):\n\n2'bx0: BT601\n\n2'bx1: BT709"]
    #[inline(always)]
    pub fn win0_csc_mode(&self) -> Win0CscModeR {
        Win0CscModeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - win0 RGB RED and BLUE swap"]
    #[inline(always)]
    pub fn win0_rb_swap(&self) -> Win0RbSwapR {
        Win0RbSwapR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - win0 alpha swap"]
    #[inline(always)]
    pub fn win0_alpha_swap(&self) -> Win0AlphaSwapR {
        Win0AlphaSwapR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Win0 Y middle swap"]
    #[inline(always)]
    pub fn win0_mid_swap(&self) -> Win0MidSwapR {
        Win0MidSwapR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Win0 CbCr swap"]
    #[inline(always)]
    pub fn win0_uv_swap(&self) -> Win0UvSwapR {
        Win0UvSwapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn win0_hw_pre_mul_en(&self) -> Win0HwPreMulEnR {
        Win0HwPreMulEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - win0_data_fmt\\[3\\]"]
    #[inline(always)]
    pub fn win0_yuyv(&self) -> Win0YuyvR {
        Win0YuyvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - win0 YRGB deflick mode"]
    #[inline(always)]
    pub fn win0_yrgb_deflick(&self) -> Win0YrgbDeflickR {
        Win0YrgbDeflickR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Win0 Cbr deflick mode"]
    #[inline(always)]
    pub fn win0_cbr_deflick(&self) -> Win0CbrDeflickR {
        Win0CbrDeflickR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - YCrCb clip"]
    #[inline(always)]
    pub fn win0_yuv_clip(&self) -> Win0YuvClipR {
        Win0YuvClipR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn win0_x_mir_en(&self) -> Win0XMirEnR {
        Win0XMirEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn win0_y_mir_en(&self) -> Win0YMirEnR {
        Win0YMirEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn win0_axi_max_outstanding_en(&self) -> Win0AxiMaxOutstandingEnR {
        Win0AxiMaxOutstandingEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - win0 out standing max number"]
    #[inline(always)]
    pub fn win0_axi_outstanding_max_num(&self) -> Win0AxiOutstandingMaxNumR {
        Win0AxiOutstandingMaxNumR::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 30:31 - WIN0 DMA read Burst length"]
    #[inline(always)]
    pub fn win0_dma_burst_length(&self) -> Win0DmaBurstLengthR {
        Win0DmaBurstLengthR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn win0_en(&mut self) -> Win0EnW<Win0Ctrl0Spec> {
        Win0EnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - vld_reg\n\n4'b0000 : ARGB888\n\n4'b0001 : RGB888\n\n4'b0010 : RGB565\n\n4'b0100 : YcbCr420\n\n4'b0101 : YcbCr422\n\n4'b0110 : YcbCr444\n\n4'b1000: YCrYCb422\n\n4'b1001: YCrYCb420\n\n4'b1010: CrYCbY422\n\n4'b1011: CrYCbY420"]
    #[inline(always)]
    #[must_use]
    pub fn win0_data_fmt(&mut self) -> Win0DataFmtW<Win0Ctrl0Spec> {
        Win0DataFmtW::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn win0_fmt_10(&mut self) -> Win0Fmt10W<Win0Ctrl0Spec> {
        Win0Fmt10W::new(self, 4)
    }
    #[doc = "Bits 5:7 - win0 line buffer mode,calc by driver."]
    #[inline(always)]
    #[must_use]
    pub fn win0_lb_mode(&mut self) -> Win0LbModeW<Win0Ctrl0Spec> {
        Win0LbModeW::new(self, 5)
    }
    #[doc = "Bit 8 - Win0 interlace read mode"]
    #[inline(always)]
    #[must_use]
    pub fn win0_interlace_read(&mut self) -> Win0InterlaceReadW<Win0Ctrl0Spec> {
        Win0InterlaceReadW::new(self, 8)
    }
    #[doc = "Bit 9 - win0 AXI master read outstanding"]
    #[inline(always)]
    #[must_use]
    pub fn win0_no_outstanding(&mut self) -> Win0NoOutstandingW<Win0Ctrl0Spec> {
        Win0NoOutstandingW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Win0 YUV2RGB or RGB2YUV\n\nColor space conversion(YUV2RGB):\n\n2'b00 : mpeg\n\n2'b01 : jpeg\n\n2'b10 : hd\n\n2'b11 : mpeg\n\nColor space conversion(RGB2YUV):\n\n2'bx0: BT601\n\n2'bx1: BT709"]
    #[inline(always)]
    #[must_use]
    pub fn win0_csc_mode(&mut self) -> Win0CscModeW<Win0Ctrl0Spec> {
        Win0CscModeW::new(self, 10)
    }
    #[doc = "Bit 12 - win0 RGB RED and BLUE swap"]
    #[inline(always)]
    #[must_use]
    pub fn win0_rb_swap(&mut self) -> Win0RbSwapW<Win0Ctrl0Spec> {
        Win0RbSwapW::new(self, 12)
    }
    #[doc = "Bit 13 - win0 alpha swap"]
    #[inline(always)]
    #[must_use]
    pub fn win0_alpha_swap(&mut self) -> Win0AlphaSwapW<Win0Ctrl0Spec> {
        Win0AlphaSwapW::new(self, 13)
    }
    #[doc = "Bit 14 - Win0 Y middle swap"]
    #[inline(always)]
    #[must_use]
    pub fn win0_mid_swap(&mut self) -> Win0MidSwapW<Win0Ctrl0Spec> {
        Win0MidSwapW::new(self, 14)
    }
    #[doc = "Bit 15 - Win0 CbCr swap"]
    #[inline(always)]
    #[must_use]
    pub fn win0_uv_swap(&mut self) -> Win0UvSwapW<Win0Ctrl0Spec> {
        Win0UvSwapW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn win0_hw_pre_mul_en(&mut self) -> Win0HwPreMulEnW<Win0Ctrl0Spec> {
        Win0HwPreMulEnW::new(self, 16)
    }
    #[doc = "Bit 17 - win0_data_fmt\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn win0_yuyv(&mut self) -> Win0YuyvW<Win0Ctrl0Spec> {
        Win0YuyvW::new(self, 17)
    }
    #[doc = "Bit 18 - win0 YRGB deflick mode"]
    #[inline(always)]
    #[must_use]
    pub fn win0_yrgb_deflick(&mut self) -> Win0YrgbDeflickW<Win0Ctrl0Spec> {
        Win0YrgbDeflickW::new(self, 18)
    }
    #[doc = "Bit 19 - Win0 Cbr deflick mode"]
    #[inline(always)]
    #[must_use]
    pub fn win0_cbr_deflick(&mut self) -> Win0CbrDeflickW<Win0Ctrl0Spec> {
        Win0CbrDeflickW::new(self, 19)
    }
    #[doc = "Bit 20 - YCrCb clip"]
    #[inline(always)]
    #[must_use]
    pub fn win0_yuv_clip(&mut self) -> Win0YuvClipW<Win0Ctrl0Spec> {
        Win0YuvClipW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn win0_x_mir_en(&mut self) -> Win0XMirEnW<Win0Ctrl0Spec> {
        Win0XMirEnW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn win0_y_mir_en(&mut self) -> Win0YMirEnW<Win0Ctrl0Spec> {
        Win0YMirEnW::new(self, 22)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn win0_axi_max_outstanding_en(&mut self) -> Win0AxiMaxOutstandingEnW<Win0Ctrl0Spec> {
        Win0AxiMaxOutstandingEnW::new(self, 24)
    }
    #[doc = "Bits 25:29 - win0 out standing max number"]
    #[inline(always)]
    #[must_use]
    pub fn win0_axi_outstanding_max_num(&mut self) -> Win0AxiOutstandingMaxNumW<Win0Ctrl0Spec> {
        Win0AxiOutstandingMaxNumW::new(self, 25)
    }
    #[doc = "Bits 30:31 - WIN0 DMA read Burst length"]
    #[inline(always)]
    #[must_use]
    pub fn win0_dma_burst_length(&mut self) -> Win0DmaBurstLengthW<Win0Ctrl0Spec> {
        Win0DmaBurstLengthW::new(self, 30)
    }
}
#[doc = "Win0 ctrl register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win0Ctrl0Spec;
impl crate::RegisterSpec for Win0Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win0_ctrl0::R`](R) reader structure"]
impl crate::Readable for Win0Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`win0_ctrl0::W`](W) writer structure"]
impl crate::Writable for Win0Ctrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN0_CTRL0 to value 0x3a00_0040"]
impl crate::Resettable for Win0Ctrl0Spec {
    const RESET_VALUE: u32 = 0x3a00_0040;
}
