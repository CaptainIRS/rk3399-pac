#[doc = "Register `SWREG98` reader"]
pub type R = crate::R<Swreg98Spec>;
#[doc = "Register `SWREG98` writer"]
pub type W = crate::W<Swreg98Spec>;
#[doc = "Which field is more closer to current picture\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264Ref14CloserSel {
    #[doc = "0: bottom field be selected"]
    B0 = 0,
    #[doc = "1: top field be selected"]
    B1 = 1,
}
impl From<H264Ref14CloserSel> for bool {
    #[inline(always)]
    fn from(variant: H264Ref14CloserSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H264_REF14_CLOSER_SEL` reader - Which field is more closer to current picture"]
pub type H264Ref14CloserSelR = crate::BitReader<H264Ref14CloserSel>;
impl H264Ref14CloserSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264Ref14CloserSel {
        match self.bits {
            false => H264Ref14CloserSel::B0,
            true => H264Ref14CloserSel::B1,
        }
    }
    #[doc = "bottom field be selected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == H264Ref14CloserSel::B0
    }
    #[doc = "top field be selected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == H264Ref14CloserSel::B1
    }
}
#[doc = "Field `H264_REF14_CLOSER_SEL` writer - Which field is more closer to current picture"]
pub type H264Ref14CloserSelW<'a, REG> = crate::BitWriter<'a, REG, H264Ref14CloserSel>;
impl<'a, REG> H264Ref14CloserSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bottom field be selected"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(H264Ref14CloserSel::B0)
    }
    #[doc = "top field be selected"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(H264Ref14CloserSel::B1)
    }
}
#[doc = "the type Refer picture consist of\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264Ref14FieldEn {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<H264Ref14FieldEn> for bool {
    #[inline(always)]
    fn from(variant: H264Ref14FieldEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H264_REF14_FIELD_EN` reader - the type Refer picture consist of"]
pub type H264Ref14FieldEnR = crate::BitReader<H264Ref14FieldEn>;
impl H264Ref14FieldEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264Ref14FieldEn {
        match self.bits {
            false => H264Ref14FieldEn::B0,
            true => H264Ref14FieldEn::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == H264Ref14FieldEn::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == H264Ref14FieldEn::B1
    }
}
#[doc = "Field `H264_REF14_FIELD_EN` writer - the type Refer picture consist of"]
pub type H264Ref14FieldEnW<'a, REG> = crate::BitWriter<'a, REG, H264Ref14FieldEn>;
impl<'a, REG> H264Ref14FieldEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(H264Ref14FieldEn::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(H264Ref14FieldEn::B1)
    }
}
#[doc = "Field `H264_REF14_ST_ADDR` reader - the start address of referance frame14"]
pub type H264Ref14StAddrR = crate::FieldReader<u32>;
#[doc = "Field `H264_REF14_ST_ADDR` writer - the start address of referance frame14"]
pub type H264Ref14StAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - Which field is more closer to current picture"]
    #[inline(always)]
    pub fn h264_ref14_closer_sel(&self) -> H264Ref14CloserSelR {
        H264Ref14CloserSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the type Refer picture consist of"]
    #[inline(always)]
    pub fn h264_ref14_field_en(&self) -> H264Ref14FieldEnR {
        H264Ref14FieldEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - the start address of referance frame14"]
    #[inline(always)]
    pub fn h264_ref14_st_addr(&self) -> H264Ref14StAddrR {
        H264Ref14StAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Which field is more closer to current picture"]
    #[inline(always)]
    #[must_use]
    pub fn h264_ref14_closer_sel(&mut self) -> H264Ref14CloserSelW<Swreg98Spec> {
        H264Ref14CloserSelW::new(self, 0)
    }
    #[doc = "Bit 1 - the type Refer picture consist of"]
    #[inline(always)]
    #[must_use]
    pub fn h264_ref14_field_en(&mut self) -> H264Ref14FieldEnW<Swreg98Spec> {
        H264Ref14FieldEnW::new(self, 1)
    }
    #[doc = "Bits 2:31 - the start address of referance frame14"]
    #[inline(always)]
    #[must_use]
    pub fn h264_ref14_st_addr(&mut self) -> H264Ref14StAddrW<Swreg98Spec> {
        H264Ref14StAddrW::new(self, 2)
    }
}
#[doc = "referance frame14 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg98::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg98::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg98Spec;
impl crate::RegisterSpec for Swreg98Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg98::R`](R) reader structure"]
impl crate::Readable for Swreg98Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg98::W`](W) writer structure"]
impl crate::Writable for Swreg98Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG98 to value 0"]
impl crate::Resettable for Swreg98Spec {
    const RESET_VALUE: u32 = 0;
}
