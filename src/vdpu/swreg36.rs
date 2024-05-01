#[doc = "Register `SWREG36` reader"]
pub type R = crate::R<Swreg36Spec>;
#[doc = "Register `SWREG36` writer"]
pub type W = crate::W<Swreg36Spec>;
#[doc = "the dither mode for R-component\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwDitherModeR {
    #[doc = "0: no use dithering"]
    D0 = 0,
    #[doc = "1: 4-bits dither matrix be used"]
    D1 = 1,
    #[doc = "2: 5-bits dither matrix be used"]
    D2 = 2,
    #[doc = "3: 6-bits dither matrix be used"]
    D3 = 3,
}
impl From<SwDitherModeR> for u8 {
    #[inline(always)]
    fn from(variant: SwDitherModeR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwDitherModeR {
    type Ux = u8;
}
#[doc = "Field `SW_DITHER_MODE_R` reader - the dither mode for R-component"]
pub type SwDitherModeRR = crate::FieldReader<SwDitherModeR>;
impl SwDitherModeRR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDitherModeR {
        match self.bits {
            0 => SwDitherModeR::D0,
            1 => SwDitherModeR::D1,
            2 => SwDitherModeR::D2,
            3 => SwDitherModeR::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "no use dithering"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == SwDitherModeR::D0
    }
    #[doc = "4-bits dither matrix be used"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == SwDitherModeR::D1
    }
    #[doc = "5-bits dither matrix be used"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == SwDitherModeR::D2
    }
    #[doc = "6-bits dither matrix be used"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == SwDitherModeR::D3
    }
}
#[doc = "Field `SW_DITHER_MODE_R` writer - the dither mode for R-component"]
pub type SwDitherModeRW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SwDitherModeR>;
impl<'a, REG> SwDitherModeRW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no use dithering"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDitherModeR::D0)
    }
    #[doc = "4-bits dither matrix be used"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDitherModeR::D1)
    }
    #[doc = "5-bits dither matrix be used"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(SwDitherModeR::D2)
    }
    #[doc = "6-bits dither matrix be used"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(SwDitherModeR::D3)
    }
}
#[doc = "the dither mode for G-component\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwDitherModeG {
    #[doc = "0: no use dithering"]
    D0 = 0,
    #[doc = "1: 4-bits dither matrix be used"]
    D1 = 1,
    #[doc = "2: 5-bits dither matrix be used"]
    D2 = 2,
    #[doc = "3: 6-bits dither matrix be used"]
    D3 = 3,
}
impl From<SwDitherModeG> for u8 {
    #[inline(always)]
    fn from(variant: SwDitherModeG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwDitherModeG {
    type Ux = u8;
}
#[doc = "Field `SW_DITHER_MODE_G` reader - the dither mode for G-component"]
pub type SwDitherModeGR = crate::FieldReader<SwDitherModeG>;
impl SwDitherModeGR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDitherModeG {
        match self.bits {
            0 => SwDitherModeG::D0,
            1 => SwDitherModeG::D1,
            2 => SwDitherModeG::D2,
            3 => SwDitherModeG::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "no use dithering"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == SwDitherModeG::D0
    }
    #[doc = "4-bits dither matrix be used"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == SwDitherModeG::D1
    }
    #[doc = "5-bits dither matrix be used"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == SwDitherModeG::D2
    }
    #[doc = "6-bits dither matrix be used"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == SwDitherModeG::D3
    }
}
#[doc = "Field `SW_DITHER_MODE_G` writer - the dither mode for G-component"]
pub type SwDitherModeGW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SwDitherModeG>;
impl<'a, REG> SwDitherModeGW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no use dithering"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDitherModeG::D0)
    }
    #[doc = "4-bits dither matrix be used"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDitherModeG::D1)
    }
    #[doc = "5-bits dither matrix be used"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(SwDitherModeG::D2)
    }
    #[doc = "6-bits dither matrix be used"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(SwDitherModeG::D3)
    }
}
#[doc = "the dither mode for B-component\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwDitherModeB {
    #[doc = "0: no use dithering"]
    D0 = 0,
    #[doc = "1: 4-bits dither matrix be used"]
    D1 = 1,
    #[doc = "2: 5-bits dither matrix be used"]
    D2 = 2,
    #[doc = "3: 6-bits dither matrix be used"]
    D3 = 3,
}
impl From<SwDitherModeB> for u8 {
    #[inline(always)]
    fn from(variant: SwDitherModeB) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwDitherModeB {
    type Ux = u8;
}
#[doc = "Field `SW_DITHER_MODE_B` reader - the dither mode for B-component"]
pub type SwDitherModeBR = crate::FieldReader<SwDitherModeB>;
impl SwDitherModeBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDitherModeB {
        match self.bits {
            0 => SwDitherModeB::D0,
            1 => SwDitherModeB::D1,
            2 => SwDitherModeB::D2,
            3 => SwDitherModeB::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "no use dithering"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == SwDitherModeB::D0
    }
    #[doc = "4-bits dither matrix be used"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == SwDitherModeB::D1
    }
    #[doc = "5-bits dither matrix be used"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == SwDitherModeB::D2
    }
    #[doc = "6-bits dither matrix be used"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == SwDitherModeB::D3
    }
}
#[doc = "Field `SW_DITHER_MODE_B` writer - the dither mode for B-component"]
pub type SwDitherModeBW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SwDitherModeB>;
impl<'a, REG> SwDitherModeBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no use dithering"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDitherModeB::D0)
    }
    #[doc = "4-bits dither matrix be used"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDitherModeB::D1)
    }
    #[doc = "5-bits dither matrix be used"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(SwDitherModeB::D2)
    }
    #[doc = "6-bits dither matrix be used"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(SwDitherModeB::D3)
    }
}
impl R {
    #[doc = "Bits 0:1 - the dither mode for R-component"]
    #[inline(always)]
    pub fn sw_dither_mode_r(&self) -> SwDitherModeRR {
        SwDitherModeRR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the dither mode for G-component"]
    #[inline(always)]
    pub fn sw_dither_mode_g(&self) -> SwDitherModeGR {
        SwDitherModeGR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the dither mode for B-component"]
    #[inline(always)]
    pub fn sw_dither_mode_b(&self) -> SwDitherModeBR {
        SwDitherModeBR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - the dither mode for R-component"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dither_mode_r(&mut self) -> SwDitherModeRW<Swreg36Spec> {
        SwDitherModeRW::new(self, 0)
    }
    #[doc = "Bits 2:3 - the dither mode for G-component"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dither_mode_g(&mut self) -> SwDitherModeGW<Swreg36Spec> {
        SwDitherModeGW::new(self, 2)
    }
    #[doc = "Bits 4:5 - the dither mode for B-component"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dither_mode_b(&mut self) -> SwDitherModeBW<Swreg36Spec> {
        SwDitherModeBW::new(self, 4)
    }
}
#[doc = "the dither mode for RGB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg36::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg36::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg36Spec;
impl crate::RegisterSpec for Swreg36Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg36::R`](R) reader structure"]
impl crate::Readable for Swreg36Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg36::W`](W) writer structure"]
impl crate::Writable for Swreg36Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG36 to value 0"]
impl crate::Resettable for Swreg36Spec {
    const RESET_VALUE: u32 = 0;
}
