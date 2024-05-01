#[doc = "Register `SWREG92` reader"]
pub type R = crate::R<Swreg92Spec>;
#[doc = "Register `SWREG92` writer"]
pub type W = crate::W<Swreg92Spec>;
#[doc = "Which field is more closer to current picture\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264Ref8CloserSel {
    #[doc = "0: bottom field be selected"]
    B0 = 0,
    #[doc = "1: top field be selected"]
    B1 = 1,
}
impl From<H264Ref8CloserSel> for bool {
    #[inline(always)]
    fn from(variant: H264Ref8CloserSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H264_REF8_CLOSER_SEL` reader - Which field is more closer to current picture"]
pub type H264Ref8CloserSelR = crate::BitReader<H264Ref8CloserSel>;
impl H264Ref8CloserSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264Ref8CloserSel {
        match self.bits {
            false => H264Ref8CloserSel::B0,
            true => H264Ref8CloserSel::B1,
        }
    }
    #[doc = "bottom field be selected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == H264Ref8CloserSel::B0
    }
    #[doc = "top field be selected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == H264Ref8CloserSel::B1
    }
}
#[doc = "Field `H264_REF8_CLOSER_SEL` writer - Which field is more closer to current picture"]
pub type H264Ref8CloserSelW<'a, REG> = crate::BitWriter<'a, REG, H264Ref8CloserSel>;
impl<'a, REG> H264Ref8CloserSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bottom field be selected"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(H264Ref8CloserSel::B0)
    }
    #[doc = "top field be selected"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(H264Ref8CloserSel::B1)
    }
}
#[doc = "the type Refer picture consist of\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264Ref8FieldEn {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<H264Ref8FieldEn> for bool {
    #[inline(always)]
    fn from(variant: H264Ref8FieldEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H264_REF8_FIELD_EN` reader - the type Refer picture consist of"]
pub type H264Ref8FieldEnR = crate::BitReader<H264Ref8FieldEn>;
impl H264Ref8FieldEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264Ref8FieldEn {
        match self.bits {
            false => H264Ref8FieldEn::B0,
            true => H264Ref8FieldEn::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == H264Ref8FieldEn::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == H264Ref8FieldEn::B1
    }
}
#[doc = "Field `H264_REF8_FIELD_EN` writer - the type Refer picture consist of"]
pub type H264Ref8FieldEnW<'a, REG> = crate::BitWriter<'a, REG, H264Ref8FieldEn>;
impl<'a, REG> H264Ref8FieldEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(H264Ref8FieldEn::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(H264Ref8FieldEn::B1)
    }
}
#[doc = "Field `H264_REF8_ST_ADDR` reader - the start address of referance frame8"]
pub type H264Ref8StAddrR = crate::FieldReader<u32>;
#[doc = "Field `H264_REF8_ST_ADDR` writer - the start address of referance frame8"]
pub type H264Ref8StAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - Which field is more closer to current picture"]
    #[inline(always)]
    pub fn h264_ref8_closer_sel(&self) -> H264Ref8CloserSelR {
        H264Ref8CloserSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the type Refer picture consist of"]
    #[inline(always)]
    pub fn h264_ref8_field_en(&self) -> H264Ref8FieldEnR {
        H264Ref8FieldEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - the start address of referance frame8"]
    #[inline(always)]
    pub fn h264_ref8_st_addr(&self) -> H264Ref8StAddrR {
        H264Ref8StAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Which field is more closer to current picture"]
    #[inline(always)]
    #[must_use]
    pub fn h264_ref8_closer_sel(&mut self) -> H264Ref8CloserSelW<Swreg92Spec> {
        H264Ref8CloserSelW::new(self, 0)
    }
    #[doc = "Bit 1 - the type Refer picture consist of"]
    #[inline(always)]
    #[must_use]
    pub fn h264_ref8_field_en(&mut self) -> H264Ref8FieldEnW<Swreg92Spec> {
        H264Ref8FieldEnW::new(self, 1)
    }
    #[doc = "Bits 2:31 - the start address of referance frame8"]
    #[inline(always)]
    #[must_use]
    pub fn h264_ref8_st_addr(&mut self) -> H264Ref8StAddrW<Swreg92Spec> {
        H264Ref8StAddrW::new(self, 2)
    }
}
#[doc = "referance frame8 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg92::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg92::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg92Spec;
impl crate::RegisterSpec for Swreg92Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg92::R`](R) reader structure"]
impl crate::Readable for Swreg92Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg92::W`](W) writer structure"]
impl crate::Writable for Swreg92Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG92 to value 0"]
impl crate::Resettable for Swreg92Spec {
    const RESET_VALUE: u32 = 0;
}
