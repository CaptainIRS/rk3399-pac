#[doc = "Register `SWREG54` reader"]
pub type R = crate::R<Swreg54Spec>;
#[doc = "Register `SWREG54` writer"]
pub type W = crate::W<Swreg54Spec>;
#[doc = "the endian mode of dec input\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDecInEndian {
    #[doc = "0: Big endian (0-1-2-3 order)"]
    B0 = 0,
    #[doc = "1: Little endian (3-2-1-0 order) note : it no used for stream data"]
    B1 = 1,
}
impl From<SwDecInEndian> for bool {
    #[inline(always)]
    fn from(variant: SwDecInEndian) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DEC_IN_ENDIAN` reader - the endian mode of dec input"]
pub type SwDecInEndianR = crate::BitReader<SwDecInEndian>;
impl SwDecInEndianR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDecInEndian {
        match self.bits {
            false => SwDecInEndian::B0,
            true => SwDecInEndian::B1,
        }
    }
    #[doc = "Big endian (0-1-2-3 order)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDecInEndian::B0
    }
    #[doc = "Little endian (3-2-1-0 order) note : it no used for stream data"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDecInEndian::B1
    }
}
#[doc = "Field `SW_DEC_IN_ENDIAN` writer - the endian mode of dec input"]
pub type SwDecInEndianW<'a, REG> = crate::BitWriter<'a, REG, SwDecInEndian>;
impl<'a, REG> SwDecInEndianW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Big endian (0-1-2-3 order)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecInEndian::B0)
    }
    #[doc = "Little endian (3-2-1-0 order) note : it no used for stream data"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecInEndian::B1)
    }
}
#[doc = "the endian mode of dec output\n\nDecoder output endian mode:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDecOutEndian {
    #[doc = "0: Big endian (0-1-2-3 order)"]
    B0 = 0,
    #[doc = "1: Little endian (3-2-1-0 order)"]
    B1 = 1,
}
impl From<SwDecOutEndian> for bool {
    #[inline(always)]
    fn from(variant: SwDecOutEndian) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DEC_OUT_ENDIAN` reader - the endian mode of dec output\n\nDecoder output endian mode:"]
pub type SwDecOutEndianR = crate::BitReader<SwDecOutEndian>;
impl SwDecOutEndianR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDecOutEndian {
        match self.bits {
            false => SwDecOutEndian::B0,
            true => SwDecOutEndian::B1,
        }
    }
    #[doc = "Big endian (0-1-2-3 order)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDecOutEndian::B0
    }
    #[doc = "Little endian (3-2-1-0 order)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDecOutEndian::B1
    }
}
#[doc = "Field `SW_DEC_OUT_ENDIAN` writer - the endian mode of dec output\n\nDecoder output endian mode:"]
pub type SwDecOutEndianW<'a, REG> = crate::BitWriter<'a, REG, SwDecOutEndian>;
impl<'a, REG> SwDecOutEndianW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Big endian (0-1-2-3 order)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecOutEndian::B0)
    }
    #[doc = "Little endian (3-2-1-0 order)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecOutEndian::B1)
    }
}
#[doc = "the 32bit data swap for dec input data,\n\nit will be used in 64 bit environment\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDecInWordsp {
    #[doc = "0: no swapping"]
    B0 = 0,
    #[doc = "1: swapping high and low 32bit data note : it no used for stream data"]
    B1 = 1,
}
impl From<SwDecInWordsp> for bool {
    #[inline(always)]
    fn from(variant: SwDecInWordsp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DEC_IN_WORDSP` reader - the 32bit data swap for dec input data,\n\nit will be used in 64 bit environment"]
pub type SwDecInWordspR = crate::BitReader<SwDecInWordsp>;
impl SwDecInWordspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDecInWordsp {
        match self.bits {
            false => SwDecInWordsp::B0,
            true => SwDecInWordsp::B1,
        }
    }
    #[doc = "no swapping"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDecInWordsp::B0
    }
    #[doc = "swapping high and low 32bit data note : it no used for stream data"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDecInWordsp::B1
    }
}
#[doc = "Field `SW_DEC_IN_WORDSP` writer - the 32bit data swap for dec input data,\n\nit will be used in 64 bit environment"]
pub type SwDecInWordspW<'a, REG> = crate::BitWriter<'a, REG, SwDecInWordsp>;
impl<'a, REG> SwDecInWordspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no swapping"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecInWordsp::B0)
    }
    #[doc = "swapping high and low 32bit data note : it no used for stream data"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecInWordsp::B1)
    }
}
#[doc = "the 32bit data swap for dec output data,\n\nit will be used in 64 bit environment\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDecOutWordsp {
    #[doc = "0: no swapping"]
    B0 = 0,
    #[doc = "1: swapping high and low 32bit data"]
    B1 = 1,
}
impl From<SwDecOutWordsp> for bool {
    #[inline(always)]
    fn from(variant: SwDecOutWordsp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DEC_OUT_WORDSP` reader - the 32bit data swap for dec output data,\n\nit will be used in 64 bit environment"]
pub type SwDecOutWordspR = crate::BitReader<SwDecOutWordsp>;
impl SwDecOutWordspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDecOutWordsp {
        match self.bits {
            false => SwDecOutWordsp::B0,
            true => SwDecOutWordsp::B1,
        }
    }
    #[doc = "no swapping"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDecOutWordsp::B0
    }
    #[doc = "swapping high and low 32bit data"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDecOutWordsp::B1
    }
}
#[doc = "Field `SW_DEC_OUT_WORDSP` writer - the 32bit data swap for dec output data,\n\nit will be used in 64 bit environment"]
pub type SwDecOutWordspW<'a, REG> = crate::BitWriter<'a, REG, SwDecOutWordsp>;
impl<'a, REG> SwDecOutWordspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no swapping"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecOutWordsp::B0)
    }
    #[doc = "swapping high and low 32bit data"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecOutWordsp::B1)
    }
}
#[doc = "the 32bit data swap for stream data\n\nit will be used in 64 bit environment\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDecStrmWordsp {
    #[doc = "0: no swapping"]
    B0 = 0,
    #[doc = "1: swapping high and low 32bit data"]
    B1 = 1,
}
impl From<SwDecStrmWordsp> for bool {
    #[inline(always)]
    fn from(variant: SwDecStrmWordsp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DEC_STRM_WORDSP` reader - the 32bit data swap for stream data\n\nit will be used in 64 bit environment"]
pub type SwDecStrmWordspR = crate::BitReader<SwDecStrmWordsp>;
impl SwDecStrmWordspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDecStrmWordsp {
        match self.bits {
            false => SwDecStrmWordsp::B0,
            true => SwDecStrmWordsp::B1,
        }
    }
    #[doc = "no swapping"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDecStrmWordsp::B0
    }
    #[doc = "swapping high and low 32bit data"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDecStrmWordsp::B1
    }
}
#[doc = "Field `SW_DEC_STRM_WORDSP` writer - the 32bit data swap for stream data\n\nit will be used in 64 bit environment"]
pub type SwDecStrmWordspW<'a, REG> = crate::BitWriter<'a, REG, SwDecStrmWordsp>;
impl<'a, REG> SwDecStrmWordspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no swapping"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecStrmWordsp::B0)
    }
    #[doc = "swapping high and low 32bit data"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecStrmWordsp::B1)
    }
}
#[doc = "the endian mode of stream data\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDecStrendianE {
    #[doc = "0: Big endian (0-1-2-3 order)"]
    B0 = 0,
    #[doc = "1: Little endian (3-2-1-0 order)"]
    B1 = 1,
}
impl From<SwDecStrendianE> for bool {
    #[inline(always)]
    fn from(variant: SwDecStrendianE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DEC_STRENDIAN_E` reader - the endian mode of stream data"]
pub type SwDecStrendianER = crate::BitReader<SwDecStrendianE>;
impl SwDecStrendianER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDecStrendianE {
        match self.bits {
            false => SwDecStrendianE::B0,
            true => SwDecStrendianE::B1,
        }
    }
    #[doc = "Big endian (0-1-2-3 order)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDecStrendianE::B0
    }
    #[doc = "Little endian (3-2-1-0 order)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDecStrendianE::B1
    }
}
#[doc = "Field `SW_DEC_STRENDIAN_E` writer - the endian mode of stream data"]
pub type SwDecStrendianEW<'a, REG> = crate::BitWriter<'a, REG, SwDecStrendianE>;
impl<'a, REG> SwDecStrendianEW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Big endian (0-1-2-3 order)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecStrendianE::B0)
    }
    #[doc = "Little endian (3-2-1-0 order)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDecStrendianE::B1)
    }
}
impl R {
    #[doc = "Bit 0 - the endian mode of dec input"]
    #[inline(always)]
    pub fn sw_dec_in_endian(&self) -> SwDecInEndianR {
        SwDecInEndianR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the endian mode of dec output\n\nDecoder output endian mode:"]
    #[inline(always)]
    pub fn sw_dec_out_endian(&self) -> SwDecOutEndianR {
        SwDecOutEndianR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the 32bit data swap for dec input data,\n\nit will be used in 64 bit environment"]
    #[inline(always)]
    pub fn sw_dec_in_wordsp(&self) -> SwDecInWordspR {
        SwDecInWordspR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the 32bit data swap for dec output data,\n\nit will be used in 64 bit environment"]
    #[inline(always)]
    pub fn sw_dec_out_wordsp(&self) -> SwDecOutWordspR {
        SwDecOutWordspR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the 32bit data swap for stream data\n\nit will be used in 64 bit environment"]
    #[inline(always)]
    pub fn sw_dec_strm_wordsp(&self) -> SwDecStrmWordspR {
        SwDecStrmWordspR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the endian mode of stream data"]
    #[inline(always)]
    pub fn sw_dec_strendian_e(&self) -> SwDecStrendianER {
        SwDecStrendianER::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - the endian mode of dec input"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_in_endian(&mut self) -> SwDecInEndianW<Swreg54Spec> {
        SwDecInEndianW::new(self, 0)
    }
    #[doc = "Bit 1 - the endian mode of dec output\n\nDecoder output endian mode:"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_out_endian(&mut self) -> SwDecOutEndianW<Swreg54Spec> {
        SwDecOutEndianW::new(self, 1)
    }
    #[doc = "Bit 2 - the 32bit data swap for dec input data,\n\nit will be used in 64 bit environment"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_in_wordsp(&mut self) -> SwDecInWordspW<Swreg54Spec> {
        SwDecInWordspW::new(self, 2)
    }
    #[doc = "Bit 3 - the 32bit data swap for dec output data,\n\nit will be used in 64 bit environment"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_out_wordsp(&mut self) -> SwDecOutWordspW<Swreg54Spec> {
        SwDecOutWordspW::new(self, 3)
    }
    #[doc = "Bit 4 - the 32bit data swap for stream data\n\nit will be used in 64 bit environment"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_strm_wordsp(&mut self) -> SwDecStrmWordspW<Swreg54Spec> {
        SwDecStrmWordspW::new(self, 4)
    }
    #[doc = "Bit 5 - the endian mode of stream data"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dec_strendian_e(&mut self) -> SwDecStrendianEW<Swreg54Spec> {
        SwDecStrendianEW::new(self, 5)
    }
}
#[doc = "endian for input/output data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg54::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg54::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg54Spec;
impl crate::RegisterSpec for Swreg54Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg54::R`](R) reader structure"]
impl crate::Readable for Swreg54Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg54::W`](W) writer structure"]
impl crate::Writable for Swreg54Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG54 to value 0"]
impl crate::Resettable for Swreg54Spec {
    const RESET_VALUE: u32 = 0;
}
