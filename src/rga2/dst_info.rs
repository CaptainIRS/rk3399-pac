#[doc = "Register `DST_INFO` reader"]
pub type R = crate::R<DstInfoSpec>;
#[doc = "Register `DST_INFO` writer"]
pub type W = crate::W<DstInfoSpec>;
#[doc = "Destination bitmap data format\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwDstFmt {
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
    #[doc = "11: YUV420P If RGA has yuyv output format feature:"]
    B1011 = 11,
    #[doc = "12: YVYU422(U, LSB)"]
    B1100 = 12,
    #[doc = "13: YVYU420(U, LSB)"]
    B1101 = 13,
    #[doc = "14: VYUY422(Y, LSB)"]
    B1110 = 14,
    #[doc = "15: VYUY420(Y, LSB)"]
    B1111 = 15,
}
impl From<SwDstFmt> for u8 {
    #[inline(always)]
    fn from(variant: SwDstFmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwDstFmt {
    type Ux = u8;
}
#[doc = "Field `SW_DST_FMT` reader - Destination bitmap data format"]
pub type SwDstFmtR = crate::FieldReader<SwDstFmt>;
impl SwDstFmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwDstFmt> {
        match self.bits {
            0 => Some(SwDstFmt::B0000),
            1 => Some(SwDstFmt::B0001),
            2 => Some(SwDstFmt::B0010),
            4 => Some(SwDstFmt::B0100),
            5 => Some(SwDstFmt::B0101),
            6 => Some(SwDstFmt::B0110),
            8 => Some(SwDstFmt::B1000),
            9 => Some(SwDstFmt::B1001),
            10 => Some(SwDstFmt::B1010),
            11 => Some(SwDstFmt::B1011),
            12 => Some(SwDstFmt::B1100),
            13 => Some(SwDstFmt::B1101),
            14 => Some(SwDstFmt::B1110),
            15 => Some(SwDstFmt::B1111),
            _ => None,
        }
    }
    #[doc = "ABGR888"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == SwDstFmt::B0000
    }
    #[doc = "XBGR888"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == SwDstFmt::B0001
    }
    #[doc = "BGR packed"]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == SwDstFmt::B0010
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == SwDstFmt::B0100
    }
    #[doc = "ARGB1555"]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == SwDstFmt::B0101
    }
    #[doc = "ARGB4444"]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == SwDstFmt::B0110
    }
    #[doc = "YUV422SP"]
    #[inline(always)]
    pub fn is_b1000(&self) -> bool {
        *self == SwDstFmt::B1000
    }
    #[doc = "YUV422P"]
    #[inline(always)]
    pub fn is_b1001(&self) -> bool {
        *self == SwDstFmt::B1001
    }
    #[doc = "YUV420SP"]
    #[inline(always)]
    pub fn is_b1010(&self) -> bool {
        *self == SwDstFmt::B1010
    }
    #[doc = "YUV420P If RGA has yuyv output format feature:"]
    #[inline(always)]
    pub fn is_b1011(&self) -> bool {
        *self == SwDstFmt::B1011
    }
    #[doc = "YVYU422(U, LSB)"]
    #[inline(always)]
    pub fn is_b1100(&self) -> bool {
        *self == SwDstFmt::B1100
    }
    #[doc = "YVYU420(U, LSB)"]
    #[inline(always)]
    pub fn is_b1101(&self) -> bool {
        *self == SwDstFmt::B1101
    }
    #[doc = "VYUY422(Y, LSB)"]
    #[inline(always)]
    pub fn is_b1110(&self) -> bool {
        *self == SwDstFmt::B1110
    }
    #[doc = "VYUY420(Y, LSB)"]
    #[inline(always)]
    pub fn is_b1111(&self) -> bool {
        *self == SwDstFmt::B1111
    }
}
#[doc = "Field `SW_DST_FMT` writer - Destination bitmap data format"]
pub type SwDstFmtW<'a, REG> = crate::FieldWriter<'a, REG, 4, SwDstFmt>;
impl<'a, REG> SwDstFmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ABGR888"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFmt::B0000)
    }
    #[doc = "XBGR888"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFmt::B0001)
    }
    #[doc = "BGR packed"]
    #[inline(always)]
    pub fn b0010(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFmt::B0010)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFmt::B0100)
    }
    #[doc = "ARGB1555"]
    #[inline(always)]
    pub fn b0101(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFmt::B0101)
    }
    #[doc = "ARGB4444"]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFmt::B0110)
    }
    #[doc = "YUV422SP"]
    #[inline(always)]
    pub fn b1000(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFmt::B1000)
    }
    #[doc = "YUV422P"]
    #[inline(always)]
    pub fn b1001(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFmt::B1001)
    }
    #[doc = "YUV420SP"]
    #[inline(always)]
    pub fn b1010(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFmt::B1010)
    }
    #[doc = "YUV420P If RGA has yuyv output format feature:"]
    #[inline(always)]
    pub fn b1011(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFmt::B1011)
    }
    #[doc = "YVYU422(U, LSB)"]
    #[inline(always)]
    pub fn b1100(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFmt::B1100)
    }
    #[doc = "YVYU420(U, LSB)"]
    #[inline(always)]
    pub fn b1101(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFmt::B1101)
    }
    #[doc = "VYUY422(Y, LSB)"]
    #[inline(always)]
    pub fn b1110(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFmt::B1110)
    }
    #[doc = "VYUY420(Y, LSB)"]
    #[inline(always)]
    pub fn b1111(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFmt::B1111)
    }
}
#[doc = "Destination bitmap data RB swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDstRbswap {
    #[doc = "0: BGR"]
    B0 = 0,
    #[doc = "1: RGB"]
    B1 = 1,
}
impl From<SwDstRbswap> for bool {
    #[inline(always)]
    fn from(variant: SwDstRbswap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DST_RBSWAP` reader - Destination bitmap data RB swap"]
pub type SwDstRbswapR = crate::BitReader<SwDstRbswap>;
impl SwDstRbswapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDstRbswap {
        match self.bits {
            false => SwDstRbswap::B0,
            true => SwDstRbswap::B1,
        }
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDstRbswap::B0
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDstRbswap::B1
    }
}
#[doc = "Field `SW_DST_RBSWAP` writer - Destination bitmap data RB swap"]
pub type SwDstRbswapW<'a, REG> = crate::BitWriter<'a, REG, SwDstRbswap>;
impl<'a, REG> SwDstRbswapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BGR"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstRbswap::B0)
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstRbswap::B1)
    }
}
#[doc = "Destination bitmap data alpha swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDstAlphaSwap {
    #[doc = "0: ABGR"]
    B0 = 0,
    #[doc = "1: BGRA"]
    B1 = 1,
}
impl From<SwDstAlphaSwap> for bool {
    #[inline(always)]
    fn from(variant: SwDstAlphaSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DST_ALPHA_SWAP` reader - Destination bitmap data alpha swap"]
pub type SwDstAlphaSwapR = crate::BitReader<SwDstAlphaSwap>;
impl SwDstAlphaSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDstAlphaSwap {
        match self.bits {
            false => SwDstAlphaSwap::B0,
            true => SwDstAlphaSwap::B1,
        }
    }
    #[doc = "ABGR"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDstAlphaSwap::B0
    }
    #[doc = "BGRA"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDstAlphaSwap::B1
    }
}
#[doc = "Field `SW_DST_ALPHA_SWAP` writer - Destination bitmap data alpha swap"]
pub type SwDstAlphaSwapW<'a, REG> = crate::BitWriter<'a, REG, SwDstAlphaSwap>;
impl<'a, REG> SwDstAlphaSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ABGR"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstAlphaSwap::B0)
    }
    #[doc = "BGRA"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstAlphaSwap::B1)
    }
}
#[doc = "Destination Cb-Cr swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDstUvswap {
    #[doc = "0: CrCb"]
    B0 = 0,
    #[doc = "1: CbCr"]
    B1 = 1,
}
impl From<SwDstUvswap> for bool {
    #[inline(always)]
    fn from(variant: SwDstUvswap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DST_UVSWAP` reader - Destination Cb-Cr swap"]
pub type SwDstUvswapR = crate::BitReader<SwDstUvswap>;
impl SwDstUvswapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDstUvswap {
        match self.bits {
            false => SwDstUvswap::B0,
            true => SwDstUvswap::B1,
        }
    }
    #[doc = "CrCb"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDstUvswap::B0
    }
    #[doc = "CbCr"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDstUvswap::B1
    }
}
#[doc = "Field `SW_DST_UVSWAP` writer - Destination Cb-Cr swap"]
pub type SwDstUvswapW<'a, REG> = crate::BitWriter<'a, REG, SwDstUvswap>;
impl<'a, REG> SwDstUvswapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CrCb"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstUvswap::B0)
    }
    #[doc = "CbCr"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstUvswap::B1)
    }
}
#[doc = "Source 1 bitmap data format\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwSrc1Fmt {
    #[doc = "0: ABGR888"]
    B000 = 0,
    #[doc = "1: XBGR888"]
    B001 = 1,
    #[doc = "2: BGR packed"]
    B010 = 2,
    #[doc = "4: RGB565"]
    B100 = 4,
    #[doc = "5: ARGB1555"]
    B101 = 5,
    #[doc = "6: ARGB4444"]
    B110 = 6,
}
impl From<SwSrc1Fmt> for u8 {
    #[inline(always)]
    fn from(variant: SwSrc1Fmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwSrc1Fmt {
    type Ux = u8;
}
#[doc = "Field `SW_SRC1_FMT` reader - Source 1 bitmap data format"]
pub type SwSrc1FmtR = crate::FieldReader<SwSrc1Fmt>;
impl SwSrc1FmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwSrc1Fmt> {
        match self.bits {
            0 => Some(SwSrc1Fmt::B000),
            1 => Some(SwSrc1Fmt::B001),
            2 => Some(SwSrc1Fmt::B010),
            4 => Some(SwSrc1Fmt::B100),
            5 => Some(SwSrc1Fmt::B101),
            6 => Some(SwSrc1Fmt::B110),
            _ => None,
        }
    }
    #[doc = "ABGR888"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == SwSrc1Fmt::B000
    }
    #[doc = "XBGR888"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == SwSrc1Fmt::B001
    }
    #[doc = "BGR packed"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == SwSrc1Fmt::B010
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == SwSrc1Fmt::B100
    }
    #[doc = "ARGB1555"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == SwSrc1Fmt::B101
    }
    #[doc = "ARGB4444"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == SwSrc1Fmt::B110
    }
}
#[doc = "Field `SW_SRC1_FMT` writer - Source 1 bitmap data format"]
pub type SwSrc1FmtW<'a, REG> = crate::FieldWriter<'a, REG, 3, SwSrc1Fmt>;
impl<'a, REG> SwSrc1FmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ABGR888"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrc1Fmt::B000)
    }
    #[doc = "XBGR888"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrc1Fmt::B001)
    }
    #[doc = "BGR packed"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrc1Fmt::B010)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrc1Fmt::B100)
    }
    #[doc = "ARGB1555"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrc1Fmt::B101)
    }
    #[doc = "ARGB4444"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrc1Fmt::B110)
    }
}
#[doc = "Source 1 bitmap data RB swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrc1Rbswap {
    #[doc = "0: BGR"]
    B0 = 0,
    #[doc = "1: RGB"]
    B1 = 1,
}
impl From<SwSrc1Rbswap> for bool {
    #[inline(always)]
    fn from(variant: SwSrc1Rbswap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC1_RBSWAP` reader - Source 1 bitmap data RB swap"]
pub type SwSrc1RbswapR = crate::BitReader<SwSrc1Rbswap>;
impl SwSrc1RbswapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrc1Rbswap {
        match self.bits {
            false => SwSrc1Rbswap::B0,
            true => SwSrc1Rbswap::B1,
        }
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrc1Rbswap::B0
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrc1Rbswap::B1
    }
}
#[doc = "Field `SW_SRC1_RBSWAP` writer - Source 1 bitmap data RB swap"]
pub type SwSrc1RbswapW<'a, REG> = crate::BitWriter<'a, REG, SwSrc1Rbswap>;
impl<'a, REG> SwSrc1RbswapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BGR"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrc1Rbswap::B0)
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrc1Rbswap::B1)
    }
}
#[doc = "Source 1 bitmap data alpha swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrc1AlphaSwap {
    #[doc = "0: ABGR"]
    B0 = 0,
    #[doc = "1: BGRA"]
    B1 = 1,
}
impl From<SwSrc1AlphaSwap> for bool {
    #[inline(always)]
    fn from(variant: SwSrc1AlphaSwap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC1_ALPHA_SWAP` reader - Source 1 bitmap data alpha swap"]
pub type SwSrc1AlphaSwapR = crate::BitReader<SwSrc1AlphaSwap>;
impl SwSrc1AlphaSwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrc1AlphaSwap {
        match self.bits {
            false => SwSrc1AlphaSwap::B0,
            true => SwSrc1AlphaSwap::B1,
        }
    }
    #[doc = "ABGR"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrc1AlphaSwap::B0
    }
    #[doc = "BGRA"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrc1AlphaSwap::B1
    }
}
#[doc = "Field `SW_SRC1_ALPHA_SWAP` writer - Source 1 bitmap data alpha swap"]
pub type SwSrc1AlphaSwapW<'a, REG> = crate::BitWriter<'a, REG, SwSrc1AlphaSwap>;
impl<'a, REG> SwSrc1AlphaSwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ABGR"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrc1AlphaSwap::B0)
    }
    #[doc = "BGRA"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrc1AlphaSwap::B1)
    }
}
#[doc = "DST/SRC1 dither up enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrc1DitherUp {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwSrc1DitherUp> for bool {
    #[inline(always)]
    fn from(variant: SwSrc1DitherUp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC1_DITHER_UP` reader - DST/SRC1 dither up enable"]
pub type SwSrc1DitherUpR = crate::BitReader<SwSrc1DitherUp>;
impl SwSrc1DitherUpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrc1DitherUp {
        match self.bits {
            false => SwSrc1DitherUp::B0,
            true => SwSrc1DitherUp::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrc1DitherUp::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrc1DitherUp::B1
    }
}
#[doc = "Field `SW_SRC1_DITHER_UP` writer - DST/SRC1 dither up enable"]
pub type SwSrc1DitherUpW<'a, REG> = crate::BitWriter<'a, REG, SwSrc1DitherUp>;
impl<'a, REG> SwSrc1DitherUpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrc1DitherUp::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrc1DitherUp::B1)
    }
}
#[doc = "DST dither down enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDitherDown {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<SwDitherDown> for bool {
    #[inline(always)]
    fn from(variant: SwDitherDown) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DITHER_DOWN` reader - DST dither down enable"]
pub type SwDitherDownR = crate::BitReader<SwDitherDown>;
impl SwDitherDownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDitherDown {
        match self.bits {
            false => SwDitherDown::B0,
            true => SwDitherDown::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDitherDown::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDitherDown::B1
    }
}
#[doc = "Field `SW_DITHER_DOWN` writer - DST dither down enable"]
pub type SwDitherDownW<'a, REG> = crate::BitWriter<'a, REG, SwDitherDown>;
impl<'a, REG> SwDitherDownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDitherDown::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDitherDown::B1)
    }
}
#[doc = "DST dither down bit mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwDitherMode {
    #[doc = "0: 888 to 666"]
    B00 = 0,
    #[doc = "1: 888 to 565"]
    B01 = 1,
    #[doc = "2: 888 to 555"]
    B10 = 2,
    #[doc = "3: 888 to 444"]
    B11 = 3,
}
impl From<SwDitherMode> for u8 {
    #[inline(always)]
    fn from(variant: SwDitherMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwDitherMode {
    type Ux = u8;
}
#[doc = "Field `SW_DITHER_MODE` reader - DST dither down bit mode"]
pub type SwDitherModeR = crate::FieldReader<SwDitherMode>;
impl SwDitherModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDitherMode {
        match self.bits {
            0 => SwDitherMode::B00,
            1 => SwDitherMode::B01,
            2 => SwDitherMode::B10,
            3 => SwDitherMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "888 to 666"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SwDitherMode::B00
    }
    #[doc = "888 to 565"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SwDitherMode::B01
    }
    #[doc = "888 to 555"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SwDitherMode::B10
    }
    #[doc = "888 to 444"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == SwDitherMode::B11
    }
}
#[doc = "Field `SW_DITHER_MODE` writer - DST dither down bit mode"]
pub type SwDitherModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SwDitherMode>;
impl<'a, REG> SwDitherModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "888 to 666"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SwDitherMode::B00)
    }
    #[doc = "888 to 565"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SwDitherMode::B01)
    }
    #[doc = "888 to 555"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SwDitherMode::B10)
    }
    #[doc = "888 to 444"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(SwDitherMode::B11)
    }
}
#[doc = "DST bitmap RGB2YUV conversion mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwDstCscMode {
    #[doc = "0: Bypass"]
    B00 = 0,
    #[doc = "1: BT.601-range0"]
    B01 = 1,
    #[doc = "2: BT.601-range1"]
    B10 = 2,
    #[doc = "3: BT.709-range0"]
    B11 = 3,
}
impl From<SwDstCscMode> for u8 {
    #[inline(always)]
    fn from(variant: SwDstCscMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwDstCscMode {
    type Ux = u8;
}
#[doc = "Field `SW_DST_CSC_MODE` reader - DST bitmap RGB2YUV conversion mode"]
pub type SwDstCscModeR = crate::FieldReader<SwDstCscMode>;
impl SwDstCscModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDstCscMode {
        match self.bits {
            0 => SwDstCscMode::B00,
            1 => SwDstCscMode::B01,
            2 => SwDstCscMode::B10,
            3 => SwDstCscMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SwDstCscMode::B00
    }
    #[doc = "BT.601-range0"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SwDstCscMode::B01
    }
    #[doc = "BT.601-range1"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SwDstCscMode::B10
    }
    #[doc = "BT.709-range0"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == SwDstCscMode::B11
    }
}
#[doc = "Field `SW_DST_CSC_MODE` writer - DST bitmap RGB2YUV conversion mode"]
pub type SwDstCscModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SwDstCscMode>;
impl<'a, REG> SwDstCscModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstCscMode::B00)
    }
    #[doc = "BT.601-range0"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstCscMode::B01)
    }
    #[doc = "BT.601-range1"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstCscMode::B10)
    }
    #[doc = "BT.709-range0"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstCscMode::B11)
    }
}
#[doc = "Field `SW_DST_CSC_CLIP` reader - BGR2YUV Clip mode(from 0~255 clip to 36~235)\n\n1: clip enable; 0: unclip"]
pub type SwDstCscClipR = crate::BitReader;
#[doc = "Field `SW_DST_CSC_CLIP` writer - BGR2YUV Clip mode(from 0~255 clip to 36~235)\n\n1: clip enable; 0: unclip"]
pub type SwDstCscClipW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Destination bitmap data format"]
    #[inline(always)]
    pub fn sw_dst_fmt(&self) -> SwDstFmtR {
        SwDstFmtR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Destination bitmap data RB swap"]
    #[inline(always)]
    pub fn sw_dst_rbswap(&self) -> SwDstRbswapR {
        SwDstRbswapR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Destination bitmap data alpha swap"]
    #[inline(always)]
    pub fn sw_dst_alpha_swap(&self) -> SwDstAlphaSwapR {
        SwDstAlphaSwapR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Destination Cb-Cr swap"]
    #[inline(always)]
    pub fn sw_dst_uvswap(&self) -> SwDstUvswapR {
        SwDstUvswapR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9 - Source 1 bitmap data format"]
    #[inline(always)]
    pub fn sw_src1_fmt(&self) -> SwSrc1FmtR {
        SwSrc1FmtR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - Source 1 bitmap data RB swap"]
    #[inline(always)]
    pub fn sw_src1_rbswap(&self) -> SwSrc1RbswapR {
        SwSrc1RbswapR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Source 1 bitmap data alpha swap"]
    #[inline(always)]
    pub fn sw_src1_alpha_swap(&self) -> SwSrc1AlphaSwapR {
        SwSrc1AlphaSwapR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DST/SRC1 dither up enable"]
    #[inline(always)]
    pub fn sw_src1_dither_up(&self) -> SwSrc1DitherUpR {
        SwSrc1DitherUpR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DST dither down enable"]
    #[inline(always)]
    pub fn sw_dither_down(&self) -> SwDitherDownR {
        SwDitherDownR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - DST dither down bit mode"]
    #[inline(always)]
    pub fn sw_dither_mode(&self) -> SwDitherModeR {
        SwDitherModeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - DST bitmap RGB2YUV conversion mode"]
    #[inline(always)]
    pub fn sw_dst_csc_mode(&self) -> SwDstCscModeR {
        SwDstCscModeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - BGR2YUV Clip mode(from 0~255 clip to 36~235)\n\n1: clip enable; 0: unclip"]
    #[inline(always)]
    pub fn sw_dst_csc_clip(&self) -> SwDstCscClipR {
        SwDstCscClipR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Destination bitmap data format"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_fmt(&mut self) -> SwDstFmtW<DstInfoSpec> {
        SwDstFmtW::new(self, 0)
    }
    #[doc = "Bit 4 - Destination bitmap data RB swap"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_rbswap(&mut self) -> SwDstRbswapW<DstInfoSpec> {
        SwDstRbswapW::new(self, 4)
    }
    #[doc = "Bit 5 - Destination bitmap data alpha swap"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_alpha_swap(&mut self) -> SwDstAlphaSwapW<DstInfoSpec> {
        SwDstAlphaSwapW::new(self, 5)
    }
    #[doc = "Bit 6 - Destination Cb-Cr swap"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_uvswap(&mut self) -> SwDstUvswapW<DstInfoSpec> {
        SwDstUvswapW::new(self, 6)
    }
    #[doc = "Bits 7:9 - Source 1 bitmap data format"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src1_fmt(&mut self) -> SwSrc1FmtW<DstInfoSpec> {
        SwSrc1FmtW::new(self, 7)
    }
    #[doc = "Bit 10 - Source 1 bitmap data RB swap"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src1_rbswap(&mut self) -> SwSrc1RbswapW<DstInfoSpec> {
        SwSrc1RbswapW::new(self, 10)
    }
    #[doc = "Bit 11 - Source 1 bitmap data alpha swap"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src1_alpha_swap(&mut self) -> SwSrc1AlphaSwapW<DstInfoSpec> {
        SwSrc1AlphaSwapW::new(self, 11)
    }
    #[doc = "Bit 12 - DST/SRC1 dither up enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src1_dither_up(&mut self) -> SwSrc1DitherUpW<DstInfoSpec> {
        SwSrc1DitherUpW::new(self, 12)
    }
    #[doc = "Bit 13 - DST dither down enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dither_down(&mut self) -> SwDitherDownW<DstInfoSpec> {
        SwDitherDownW::new(self, 13)
    }
    #[doc = "Bits 14:15 - DST dither down bit mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dither_mode(&mut self) -> SwDitherModeW<DstInfoSpec> {
        SwDitherModeW::new(self, 14)
    }
    #[doc = "Bits 16:17 - DST bitmap RGB2YUV conversion mode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_csc_mode(&mut self) -> SwDstCscModeW<DstInfoSpec> {
        SwDstCscModeW::new(self, 16)
    }
    #[doc = "Bit 18 - BGR2YUV Clip mode(from 0~255 clip to 36~235)\n\n1: clip enable; 0: unclip"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_csc_clip(&mut self) -> SwDstCscClipW<DstInfoSpec> {
        SwDstCscClipW::new(self, 18)
    }
}
#[doc = "RGA destination format register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstInfoSpec;
impl crate::RegisterSpec for DstInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_info::R`](R) reader structure"]
impl crate::Readable for DstInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_info::W`](W) writer structure"]
impl crate::Writable for DstInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_INFO to value 0"]
impl crate::Resettable for DstInfoSpec {
    const RESET_VALUE: u32 = 0;
}
