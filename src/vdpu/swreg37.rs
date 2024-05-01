#[doc = "Register `SWREG37` reader"]
pub type R = crate::R<Swreg37Spec>;
#[doc = "Register `SWREG37` writer"]
pub type W = crate::W<Swreg37Spec>;
#[doc = "the endian mode of pp input picture when PP in standalone mode\n\nthis bit will no to be used when PP is running pipelined with the\n\ndecoder\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwPpInEndian {
    #[doc = "0: big endian (0-1-2-3)"]
    B0 = 0,
    #[doc = "1: little endian (3-2-1-0 )"]
    B1 = 1,
}
impl From<SwPpInEndian> for bool {
    #[inline(always)]
    fn from(variant: SwPpInEndian) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PP_IN_ENDIAN` reader - the endian mode of pp input picture when PP in standalone mode\n\nthis bit will no to be used when PP is running pipelined with the\n\ndecoder"]
pub type SwPpInEndianR = crate::BitReader<SwPpInEndian>;
impl SwPpInEndianR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPpInEndian {
        match self.bits {
            false => SwPpInEndian::B0,
            true => SwPpInEndian::B1,
        }
    }
    #[doc = "big endian (0-1-2-3)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPpInEndian::B0
    }
    #[doc = "little endian (3-2-1-0 )"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPpInEndian::B1
    }
}
#[doc = "Field `SW_PP_IN_ENDIAN` writer - the endian mode of pp input picture when PP in standalone mode\n\nthis bit will no to be used when PP is running pipelined with the\n\ndecoder"]
pub type SwPpInEndianW<'a, REG> = crate::BitWriter<'a, REG, SwPpInEndian>;
impl<'a, REG> SwPpInEndianW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "big endian (0-1-2-3)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInEndian::B0)
    }
    #[doc = "little endian (3-2-1-0 )"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInEndian::B1)
    }
}
#[doc = "the endian mode of input data for Alpha blend source 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwPpAbld1InEndian {
    #[doc = "0: big endian (0-1-2-3)"]
    B0 = 0,
    #[doc = "1: little endian (3-2-1-0 )"]
    B1 = 1,
}
impl From<SwPpAbld1InEndian> for bool {
    #[inline(always)]
    fn from(variant: SwPpAbld1InEndian) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PP_ABLD1_IN_ENDIAN` reader - the endian mode of input data for Alpha blend source 1"]
pub type SwPpAbld1InEndianR = crate::BitReader<SwPpAbld1InEndian>;
impl SwPpAbld1InEndianR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPpAbld1InEndian {
        match self.bits {
            false => SwPpAbld1InEndian::B0,
            true => SwPpAbld1InEndian::B1,
        }
    }
    #[doc = "big endian (0-1-2-3)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPpAbld1InEndian::B0
    }
    #[doc = "little endian (3-2-1-0 )"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPpAbld1InEndian::B1
    }
}
#[doc = "Field `SW_PP_ABLD1_IN_ENDIAN` writer - the endian mode of input data for Alpha blend source 1"]
pub type SwPpAbld1InEndianW<'a, REG> = crate::BitWriter<'a, REG, SwPpAbld1InEndian>;
impl<'a, REG> SwPpAbld1InEndianW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "big endian (0-1-2-3)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpAbld1InEndian::B0)
    }
    #[doc = "little endian (3-2-1-0 )"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpAbld1InEndian::B1)
    }
}
#[doc = "the endian select for input data of Alpha blend source 2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwPpAbld2InEndian {
    #[doc = "0: same with sw_pp_in_endian"]
    B0 = 0,
    #[doc = "1: same with sw_pp_abld1_in_endian '0' = Use PP in endian/swap definitions (sw_pp_in_endian, sw_pp_in_swap) '1' = Use Ablend source 1 endian/swap definitions"]
    B1 = 1,
}
impl From<SwPpAbld2InEndian> for bool {
    #[inline(always)]
    fn from(variant: SwPpAbld2InEndian) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PP_ABLD2_IN_ENDIAN` reader - the endian select for input data of Alpha blend source 2"]
pub type SwPpAbld2InEndianR = crate::BitReader<SwPpAbld2InEndian>;
impl SwPpAbld2InEndianR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPpAbld2InEndian {
        match self.bits {
            false => SwPpAbld2InEndian::B0,
            true => SwPpAbld2InEndian::B1,
        }
    }
    #[doc = "same with sw_pp_in_endian"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPpAbld2InEndian::B0
    }
    #[doc = "same with sw_pp_abld1_in_endian '0' = Use PP in endian/swap definitions (sw_pp_in_endian, sw_pp_in_swap) '1' = Use Ablend source 1 endian/swap definitions"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPpAbld2InEndian::B1
    }
}
#[doc = "Field `SW_PP_ABLD2_IN_ENDIAN` writer - the endian select for input data of Alpha blend source 2"]
pub type SwPpAbld2InEndianW<'a, REG> = crate::BitWriter<'a, REG, SwPpAbld2InEndian>;
impl<'a, REG> SwPpAbld2InEndianW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "same with sw_pp_in_endian"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpAbld2InEndian::B0)
    }
    #[doc = "same with sw_pp_abld1_in_endian '0' = Use PP in endian/swap definitions (sw_pp_in_endian, sw_pp_in_swap) '1' = Use Ablend source 1 endian/swap definitions"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpAbld2InEndian::B1)
    }
}
#[doc = "the endian mode of pp output\n\nfor all yuv output endian mode or any data when\n\npp_endian_mode=1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwPpOutEndian {
    #[doc = "0: big endian"]
    B0 = 0,
    #[doc = "1: little endian if pp_endian_mode=0: 16 bit RGB: this bit used as pixel swapping bit 32 bit RGB: no used"]
    B1 = 1,
}
impl From<SwPpOutEndian> for bool {
    #[inline(always)]
    fn from(variant: SwPpOutEndian) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PP_OUT_ENDIAN` reader - the endian mode of pp output\n\nfor all yuv output endian mode or any data when\n\npp_endian_mode=1"]
pub type SwPpOutEndianR = crate::BitReader<SwPpOutEndian>;
impl SwPpOutEndianR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPpOutEndian {
        match self.bits {
            false => SwPpOutEndian::B0,
            true => SwPpOutEndian::B1,
        }
    }
    #[doc = "big endian"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPpOutEndian::B0
    }
    #[doc = "little endian if pp_endian_mode=0: 16 bit RGB: this bit used as pixel swapping bit 32 bit RGB: no used"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPpOutEndian::B1
    }
}
#[doc = "Field `SW_PP_OUT_ENDIAN` writer - the endian mode of pp output\n\nfor all yuv output endian mode or any data when\n\npp_endian_mode=1"]
pub type SwPpOutEndianW<'a, REG> = crate::BitWriter<'a, REG, SwPpOutEndian>;
impl<'a, REG> SwPpOutEndianW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "big endian"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpOutEndian::B0)
    }
    #[doc = "little endian if pp_endian_mode=0: 16 bit RGB: this bit used as pixel swapping bit 32 bit RGB: no used"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpOutEndian::B1)
    }
}
#[doc = "rga bits used sel\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwRgbPixBits {
    #[doc = "0: every word have only one rga pixel"]
    B0 = 0,
    #[doc = "1: every word have two rga pixel"]
    B1 = 1,
}
impl From<SwRgbPixBits> for bool {
    #[inline(always)]
    fn from(variant: SwRgbPixBits) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_RGB_PIX_BITS` reader - rga bits used sel"]
pub type SwRgbPixBitsR = crate::BitReader<SwRgbPixBits>;
impl SwRgbPixBitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwRgbPixBits {
        match self.bits {
            false => SwRgbPixBits::B0,
            true => SwRgbPixBits::B1,
        }
    }
    #[doc = "every word have only one rga pixel"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwRgbPixBits::B0
    }
    #[doc = "every word have two rga pixel"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwRgbPixBits::B1
    }
}
#[doc = "Field `SW_RGB_PIX_BITS` writer - rga bits used sel"]
pub type SwRgbPixBitsW<'a, REG> = crate::BitWriter<'a, REG, SwRgbPixBits>;
impl<'a, REG> SwRgbPixBitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "every word have only one rga pixel"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwRgbPixBits::B0)
    }
    #[doc = "every word have two rga pixel"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwRgbPixBits::B1)
    }
}
#[doc = "the 32bit data swap for pp input data,\n\nit will be used in 64 bit environment\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwPpInWordsp {
    #[doc = "0: no swapping"]
    B0 = 0,
    #[doc = "1: swapping high and low 32bit data"]
    B1 = 1,
}
impl From<SwPpInWordsp> for bool {
    #[inline(always)]
    fn from(variant: SwPpInWordsp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PP_IN_WORDSP` reader - the 32bit data swap for pp input data,\n\nit will be used in 64 bit environment"]
pub type SwPpInWordspR = crate::BitReader<SwPpInWordsp>;
impl SwPpInWordspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPpInWordsp {
        match self.bits {
            false => SwPpInWordsp::B0,
            true => SwPpInWordsp::B1,
        }
    }
    #[doc = "no swapping"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPpInWordsp::B0
    }
    #[doc = "swapping high and low 32bit data"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPpInWordsp::B1
    }
}
#[doc = "Field `SW_PP_IN_WORDSP` writer - the 32bit data swap for pp input data,\n\nit will be used in 64 bit environment"]
pub type SwPpInWordspW<'a, REG> = crate::BitWriter<'a, REG, SwPpInWordsp>;
impl<'a, REG> SwPpInWordspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no swapping"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInWordsp::B0)
    }
    #[doc = "swapping high and low 32bit data"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInWordsp::B1)
    }
}
#[doc = "the 32bit data swap for the input 32bit data swap of Alpha blend\n\nfor Alpha blend source 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwPpAbld1InWordsp {
    #[doc = "0: no swapping"]
    B0 = 0,
    #[doc = "1: swapping high and low 32bit data"]
    B1 = 1,
}
impl From<SwPpAbld1InWordsp> for bool {
    #[inline(always)]
    fn from(variant: SwPpAbld1InWordsp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PP_ABLD1_IN_WORDSP` reader - the 32bit data swap for the input 32bit data swap of Alpha blend\n\nfor Alpha blend source 1"]
pub type SwPpAbld1InWordspR = crate::BitReader<SwPpAbld1InWordsp>;
impl SwPpAbld1InWordspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPpAbld1InWordsp {
        match self.bits {
            false => SwPpAbld1InWordsp::B0,
            true => SwPpAbld1InWordsp::B1,
        }
    }
    #[doc = "no swapping"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPpAbld1InWordsp::B0
    }
    #[doc = "swapping high and low 32bit data"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPpAbld1InWordsp::B1
    }
}
#[doc = "Field `SW_PP_ABLD1_IN_WORDSP` writer - the 32bit data swap for the input 32bit data swap of Alpha blend\n\nfor Alpha blend source 1"]
pub type SwPpAbld1InWordspW<'a, REG> = crate::BitWriter<'a, REG, SwPpAbld1InWordsp>;
impl<'a, REG> SwPpAbld1InWordspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no swapping"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpAbld1InWordsp::B0)
    }
    #[doc = "swapping high and low 32bit data"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpAbld1InWordsp::B1)
    }
}
#[doc = "the half word swap inside of word\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwPpOutHfwordsp {
    #[doc = "0: no swap"]
    B0 = 0,
    #[doc = "1: swap also be used as change pixel orders for 16 bit RGB,support all output format require pp_endian_mode=1"]
    B1 = 1,
}
impl From<SwPpOutHfwordsp> for bool {
    #[inline(always)]
    fn from(variant: SwPpOutHfwordsp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PP_OUT_HFWORDSP` reader - the half word swap inside of word"]
pub type SwPpOutHfwordspR = crate::BitReader<SwPpOutHfwordsp>;
impl SwPpOutHfwordspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPpOutHfwordsp {
        match self.bits {
            false => SwPpOutHfwordsp::B0,
            true => SwPpOutHfwordsp::B1,
        }
    }
    #[doc = "no swap"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPpOutHfwordsp::B0
    }
    #[doc = "swap also be used as change pixel orders for 16 bit RGB,support all output format require pp_endian_mode=1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPpOutHfwordsp::B1
    }
}
#[doc = "Field `SW_PP_OUT_HFWORDSP` writer - the half word swap inside of word"]
pub type SwPpOutHfwordspW<'a, REG> = crate::BitWriter<'a, REG, SwPpOutHfwordsp>;
impl<'a, REG> SwPpOutHfwordspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no swap"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpOutHfwordsp::B0)
    }
    #[doc = "swap also be used as change pixel orders for 16 bit RGB,support all output format require pp_endian_mode=1"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpOutHfwordsp::B1)
    }
}
#[doc = "the 32bit data swap for pp output data,\n\nit will be used in 64 bit environment\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwPpOutWordsp {
    #[doc = "0: no swapping"]
    B0 = 0,
    #[doc = "1: swapping high and low 32bit data"]
    B1 = 1,
}
impl From<SwPpOutWordsp> for bool {
    #[inline(always)]
    fn from(variant: SwPpOutWordsp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PP_OUT_WORDSP` reader - the 32bit data swap for pp output data,\n\nit will be used in 64 bit environment"]
pub type SwPpOutWordspR = crate::BitReader<SwPpOutWordsp>;
impl SwPpOutWordspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPpOutWordsp {
        match self.bits {
            false => SwPpOutWordsp::B0,
            true => SwPpOutWordsp::B1,
        }
    }
    #[doc = "no swapping"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPpOutWordsp::B0
    }
    #[doc = "swapping high and low 32bit data"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPpOutWordsp::B1
    }
}
#[doc = "Field `SW_PP_OUT_WORDSP` writer - the 32bit data swap for pp output data,\n\nit will be used in 64 bit environment"]
pub type SwPpOutWordspW<'a, REG> = crate::BitWriter<'a, REG, SwPpOutWordsp>;
impl<'a, REG> SwPpOutWordspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no swapping"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpOutWordsp::B0)
    }
    #[doc = "swapping high and low 32bit data"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpOutWordsp::B1)
    }
}
#[doc = "the input yuv order\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwPpInYuvOrder {
    #[doc = "0: Y0CbY0Cr / Y0CrY0Cb"]
    B0 = 0,
    #[doc = "1: CbY0CrY0 / CrY0CbY0"]
    B1 = 1,
}
impl From<SwPpInYuvOrder> for bool {
    #[inline(always)]
    fn from(variant: SwPpInYuvOrder) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PP_IN_YUV_ORDER` reader - the input yuv order"]
pub type SwPpInYuvOrderR = crate::BitReader<SwPpInYuvOrder>;
impl SwPpInYuvOrderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPpInYuvOrder {
        match self.bits {
            false => SwPpInYuvOrder::B0,
            true => SwPpInYuvOrder::B1,
        }
    }
    #[doc = "Y0CbY0Cr / Y0CrY0Cb"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPpInYuvOrder::B0
    }
    #[doc = "CbY0CrY0 / CrY0CbY0"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPpInYuvOrder::B1
    }
}
#[doc = "Field `SW_PP_IN_YUV_ORDER` writer - the input yuv order"]
pub type SwPpInYuvOrderW<'a, REG> = crate::BitWriter<'a, REG, SwPpInYuvOrder>;
impl<'a, REG> SwPpInYuvOrderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Y0CbY0Cr / Y0CrY0Cb"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInYuvOrder::B0)
    }
    #[doc = "CbY0CrY0 / CrY0CbY0"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInYuvOrder::B1)
    }
}
#[doc = "the output yuv order\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwPpOutYuvOrder {
    #[doc = "0: Y0CbY0Cr / Y0CrY0Cb"]
    B0 = 0,
    #[doc = "1: CbY0CrY0 / CrY0CbY0"]
    B1 = 1,
}
impl From<SwPpOutYuvOrder> for bool {
    #[inline(always)]
    fn from(variant: SwPpOutYuvOrder) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PP_OUT_YUV_ORDER` reader - the output yuv order"]
pub type SwPpOutYuvOrderR = crate::BitReader<SwPpOutYuvOrder>;
impl SwPpOutYuvOrderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPpOutYuvOrder {
        match self.bits {
            false => SwPpOutYuvOrder::B0,
            true => SwPpOutYuvOrder::B1,
        }
    }
    #[doc = "Y0CbY0Cr / Y0CrY0Cb"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPpOutYuvOrder::B0
    }
    #[doc = "CbY0CrY0 / CrY0CbY0"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPpOutYuvOrder::B1
    }
}
#[doc = "Field `SW_PP_OUT_YUV_ORDER` writer - the output yuv order"]
pub type SwPpOutYuvOrderW<'a, REG> = crate::BitWriter<'a, REG, SwPpOutYuvOrder>;
impl<'a, REG> SwPpOutYuvOrderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Y0CbY0Cr / Y0CrY0Cb"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpOutYuvOrder::B0)
    }
    #[doc = "CbY0CrY0 / CrY0CbY0"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpOutYuvOrder::B1)
    }
}
#[doc = "Field `SW_PP_IN_CRBF_EN` reader - in yuv422 or yuv420,cr before cb\n\nyuv422:\n\n0 : Y0CbY0Cr / CbY0CrY0\n\n1 : Y0CrY0Cb / CrY0CbY0\n\nyuv420 semiplanar chrominance:\n\n0 : CbCrCbCr\n\n1:CrCbCrCb"]
pub type SwPpInCrbfEnR = crate::BitReader;
#[doc = "Field `SW_PP_IN_CRBF_EN` writer - in yuv422 or yuv420,cr before cb\n\nyuv422:\n\n0 : Y0CbY0Cr / CbY0CrY0\n\n1 : Y0CrY0Cb / CrY0CbY0\n\nyuv420 semiplanar chrominance:\n\n0 : CbCrCbCr\n\n1:CrCbCrCb"]
pub type SwPpInCrbfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "output yuv422 or yuv420,cr before cb\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwPpOutCrbfEn {
    #[doc = "0: Y0CbY0Cr / CbY0CrY0"]
    B0 = 0,
    #[doc = "1: Y0CrY0Cb / CrY0CbY0"]
    B1 = 1,
}
impl From<SwPpOutCrbfEn> for bool {
    #[inline(always)]
    fn from(variant: SwPpOutCrbfEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_PP_OUT_CRBF_EN` reader - output yuv422 or yuv420,cr before cb"]
pub type SwPpOutCrbfEnR = crate::BitReader<SwPpOutCrbfEn>;
impl SwPpOutCrbfEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwPpOutCrbfEn {
        match self.bits {
            false => SwPpOutCrbfEn::B0,
            true => SwPpOutCrbfEn::B1,
        }
    }
    #[doc = "Y0CbY0Cr / CbY0CrY0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwPpOutCrbfEn::B0
    }
    #[doc = "Y0CrY0Cb / CrY0CbY0"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwPpOutCrbfEn::B1
    }
}
#[doc = "Field `SW_PP_OUT_CRBF_EN` writer - output yuv422 or yuv420,cr before cb"]
pub type SwPpOutCrbfEnW<'a, REG> = crate::BitWriter<'a, REG, SwPpOutCrbfEn>;
impl<'a, REG> SwPpOutCrbfEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Y0CbY0Cr / CbY0CrY0"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpOutCrbfEn::B0)
    }
    #[doc = "Y0CrY0Cb / CrY0CbY0"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpOutCrbfEn::B1)
    }
}
#[doc = "the data structure of pp input picture\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwPpInDataStrc {
    #[doc = "0: Top field"]
    D0 = 0,
    #[doc = "1: Bottom field"]
    D1 = 1,
    #[doc = "2: Interlaced field"]
    D2 = 2,
    #[doc = "3: Interlaced frame"]
    D3 = 3,
    #[doc = "4: Ripped top field"]
    D4 = 4,
    #[doc = "5: Ripped bottom field if value=0/1/2,then should read every line from the base address,if value=3/4/5,then should read every second line from the base address"]
    D5 = 5,
}
impl From<SwPpInDataStrc> for u8 {
    #[inline(always)]
    fn from(variant: SwPpInDataStrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwPpInDataStrc {
    type Ux = u8;
}
#[doc = "Field `SW_PP_IN_DATA_STRC` reader - the data structure of pp input picture"]
pub type SwPpInDataStrcR = crate::FieldReader<SwPpInDataStrc>;
impl SwPpInDataStrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwPpInDataStrc> {
        match self.bits {
            0 => Some(SwPpInDataStrc::D0),
            1 => Some(SwPpInDataStrc::D1),
            2 => Some(SwPpInDataStrc::D2),
            3 => Some(SwPpInDataStrc::D3),
            4 => Some(SwPpInDataStrc::D4),
            5 => Some(SwPpInDataStrc::D5),
            _ => None,
        }
    }
    #[doc = "Top field"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == SwPpInDataStrc::D0
    }
    #[doc = "Bottom field"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == SwPpInDataStrc::D1
    }
    #[doc = "Interlaced field"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == SwPpInDataStrc::D2
    }
    #[doc = "Interlaced frame"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == SwPpInDataStrc::D3
    }
    #[doc = "Ripped top field"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == SwPpInDataStrc::D4
    }
    #[doc = "Ripped bottom field if value=0/1/2,then should read every line from the base address,if value=3/4/5,then should read every second line from the base address"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == SwPpInDataStrc::D5
    }
}
#[doc = "Field `SW_PP_IN_DATA_STRC` writer - the data structure of pp input picture"]
pub type SwPpInDataStrcW<'a, REG> = crate::FieldWriter<'a, REG, 3, SwPpInDataStrc>;
impl<'a, REG> SwPpInDataStrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Top field"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInDataStrc::D0)
    }
    #[doc = "Bottom field"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInDataStrc::D1)
    }
    #[doc = "Interlaced field"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInDataStrc::D2)
    }
    #[doc = "Interlaced frame"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInDataStrc::D3)
    }
    #[doc = "Ripped top field"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInDataStrc::D4)
    }
    #[doc = "Ripped bottom field if value=0/1/2,then should read every line from the base address,if value=3/4/5,then should read every second line from the base address"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut crate::W<REG> {
        self.variant(SwPpInDataStrc::D5)
    }
}
impl R {
    #[doc = "Bit 0 - the endian mode of pp input picture when PP in standalone mode\n\nthis bit will no to be used when PP is running pipelined with the\n\ndecoder"]
    #[inline(always)]
    pub fn sw_pp_in_endian(&self) -> SwPpInEndianR {
        SwPpInEndianR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the endian mode of input data for Alpha blend source 1"]
    #[inline(always)]
    pub fn sw_pp_abld1_in_endian(&self) -> SwPpAbld1InEndianR {
        SwPpAbld1InEndianR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the endian select for input data of Alpha blend source 2"]
    #[inline(always)]
    pub fn sw_pp_abld2_in_endian(&self) -> SwPpAbld2InEndianR {
        SwPpAbld2InEndianR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the endian mode of pp output\n\nfor all yuv output endian mode or any data when\n\npp_endian_mode=1"]
    #[inline(always)]
    pub fn sw_pp_out_endian(&self) -> SwPpOutEndianR {
        SwPpOutEndianR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - rga bits used sel"]
    #[inline(always)]
    pub fn sw_rgb_pix_bits(&self) -> SwRgbPixBitsR {
        SwRgbPixBitsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - the 32bit data swap for pp input data,\n\nit will be used in 64 bit environment"]
    #[inline(always)]
    pub fn sw_pp_in_wordsp(&self) -> SwPpInWordspR {
        SwPpInWordspR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - the 32bit data swap for the input 32bit data swap of Alpha blend\n\nfor Alpha blend source 1"]
    #[inline(always)]
    pub fn sw_pp_abld1_in_wordsp(&self) -> SwPpAbld1InWordspR {
        SwPpAbld1InWordspR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - the half word swap inside of word"]
    #[inline(always)]
    pub fn sw_pp_out_hfwordsp(&self) -> SwPpOutHfwordspR {
        SwPpOutHfwordspR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - the 32bit data swap for pp output data,\n\nit will be used in 64 bit environment"]
    #[inline(always)]
    pub fn sw_pp_out_wordsp(&self) -> SwPpOutWordspR {
        SwPpOutWordspR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - the input yuv order"]
    #[inline(always)]
    pub fn sw_pp_in_yuv_order(&self) -> SwPpInYuvOrderR {
        SwPpInYuvOrderR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - the output yuv order"]
    #[inline(always)]
    pub fn sw_pp_out_yuv_order(&self) -> SwPpOutYuvOrderR {
        SwPpOutYuvOrderR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - in yuv422 or yuv420,cr before cb\n\nyuv422:\n\n0 : Y0CbY0Cr / CbY0CrY0\n\n1 : Y0CrY0Cb / CrY0CbY0\n\nyuv420 semiplanar chrominance:\n\n0 : CbCrCbCr\n\n1:CrCbCrCb"]
    #[inline(always)]
    pub fn sw_pp_in_crbf_en(&self) -> SwPpInCrbfEnR {
        SwPpInCrbfEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - output yuv422 or yuv420,cr before cb"]
    #[inline(always)]
    pub fn sw_pp_out_crbf_en(&self) -> SwPpOutCrbfEnR {
        SwPpOutCrbfEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:26 - the data structure of pp input picture"]
    #[inline(always)]
    pub fn sw_pp_in_data_strc(&self) -> SwPpInDataStrcR {
        SwPpInDataStrcR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - the endian mode of pp input picture when PP in standalone mode\n\nthis bit will no to be used when PP is running pipelined with the\n\ndecoder"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_in_endian(&mut self) -> SwPpInEndianW<Swreg37Spec> {
        SwPpInEndianW::new(self, 0)
    }
    #[doc = "Bit 1 - the endian mode of input data for Alpha blend source 1"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_abld1_in_endian(&mut self) -> SwPpAbld1InEndianW<Swreg37Spec> {
        SwPpAbld1InEndianW::new(self, 1)
    }
    #[doc = "Bit 2 - the endian select for input data of Alpha blend source 2"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_abld2_in_endian(&mut self) -> SwPpAbld2InEndianW<Swreg37Spec> {
        SwPpAbld2InEndianW::new(self, 2)
    }
    #[doc = "Bit 3 - the endian mode of pp output\n\nfor all yuv output endian mode or any data when\n\npp_endian_mode=1"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_out_endian(&mut self) -> SwPpOutEndianW<Swreg37Spec> {
        SwPpOutEndianW::new(self, 3)
    }
    #[doc = "Bit 4 - rga bits used sel"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rgb_pix_bits(&mut self) -> SwRgbPixBitsW<Swreg37Spec> {
        SwRgbPixBitsW::new(self, 4)
    }
    #[doc = "Bit 8 - the 32bit data swap for pp input data,\n\nit will be used in 64 bit environment"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_in_wordsp(&mut self) -> SwPpInWordspW<Swreg37Spec> {
        SwPpInWordspW::new(self, 8)
    }
    #[doc = "Bit 9 - the 32bit data swap for the input 32bit data swap of Alpha blend\n\nfor Alpha blend source 1"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_abld1_in_wordsp(&mut self) -> SwPpAbld1InWordspW<Swreg37Spec> {
        SwPpAbld1InWordspW::new(self, 9)
    }
    #[doc = "Bit 10 - the half word swap inside of word"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_out_hfwordsp(&mut self) -> SwPpOutHfwordspW<Swreg37Spec> {
        SwPpOutHfwordspW::new(self, 10)
    }
    #[doc = "Bit 11 - the 32bit data swap for pp output data,\n\nit will be used in 64 bit environment"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_out_wordsp(&mut self) -> SwPpOutWordspW<Swreg37Spec> {
        SwPpOutWordspW::new(self, 11)
    }
    #[doc = "Bit 16 - the input yuv order"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_in_yuv_order(&mut self) -> SwPpInYuvOrderW<Swreg37Spec> {
        SwPpInYuvOrderW::new(self, 16)
    }
    #[doc = "Bit 17 - the output yuv order"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_out_yuv_order(&mut self) -> SwPpOutYuvOrderW<Swreg37Spec> {
        SwPpOutYuvOrderW::new(self, 17)
    }
    #[doc = "Bit 18 - in yuv422 or yuv420,cr before cb\n\nyuv422:\n\n0 : Y0CbY0Cr / CbY0CrY0\n\n1 : Y0CrY0Cb / CrY0CbY0\n\nyuv420 semiplanar chrominance:\n\n0 : CbCrCbCr\n\n1:CrCbCrCb"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_in_crbf_en(&mut self) -> SwPpInCrbfEnW<Swreg37Spec> {
        SwPpInCrbfEnW::new(self, 18)
    }
    #[doc = "Bit 19 - output yuv422 or yuv420,cr before cb"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_out_crbf_en(&mut self) -> SwPpOutCrbfEnW<Swreg37Spec> {
        SwPpOutCrbfEnW::new(self, 19)
    }
    #[doc = "Bits 24:26 - the data structure of pp input picture"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_in_data_strc(&mut self) -> SwPpInDataStrcW<Swreg37Spec> {
        SwPpInDataStrcW::new(self, 24)
    }
}
#[doc = "PP input/output data format\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg37::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg37::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg37Spec;
impl crate::RegisterSpec for Swreg37Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg37::R`](R) reader structure"]
impl crate::Readable for Swreg37Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg37::W`](W) writer structure"]
impl crate::Writable for Swreg37Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG37 to value 0"]
impl crate::Resettable for Swreg37Spec {
    const RESET_VALUE: u32 = 0;
}
