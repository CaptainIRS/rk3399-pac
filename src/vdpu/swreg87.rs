#[doc = "Register `SWREG87` reader"]
pub type R = crate::R<Swreg87Spec>;
#[doc = "Register `SWREG87` writer"]
pub type W = crate::W<Swreg87Spec>;
#[doc = "Which field is more closer to current picture\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264Ref3CloserSel {
    #[doc = "0: bottom field be selected"]
    B0 = 0,
    #[doc = "1: top field be selected"]
    B1 = 1,
}
impl From<H264Ref3CloserSel> for bool {
    #[inline(always)]
    fn from(variant: H264Ref3CloserSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H264_REF3_CLOSER_SEL` reader - Which field is more closer to current picture"]
pub type H264Ref3CloserSelR = crate::BitReader<H264Ref3CloserSel>;
impl H264Ref3CloserSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264Ref3CloserSel {
        match self.bits {
            false => H264Ref3CloserSel::B0,
            true => H264Ref3CloserSel::B1,
        }
    }
    #[doc = "bottom field be selected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == H264Ref3CloserSel::B0
    }
    #[doc = "top field be selected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == H264Ref3CloserSel::B1
    }
}
#[doc = "Field `H264_REF3_CLOSER_SEL` writer - Which field is more closer to current picture"]
pub type H264Ref3CloserSelW<'a, REG> = crate::BitWriter<'a, REG, H264Ref3CloserSel>;
impl<'a, REG> H264Ref3CloserSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bottom field be selected"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(H264Ref3CloserSel::B0)
    }
    #[doc = "top field be selected"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(H264Ref3CloserSel::B1)
    }
}
#[doc = "the type Refer picture consist of\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264Ref3FieldEn {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<H264Ref3FieldEn> for bool {
    #[inline(always)]
    fn from(variant: H264Ref3FieldEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H264_REF3_FIELD_EN` reader - the type Refer picture consist of"]
pub type H264Ref3FieldEnR = crate::BitReader<H264Ref3FieldEn>;
impl H264Ref3FieldEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264Ref3FieldEn {
        match self.bits {
            false => H264Ref3FieldEn::B0,
            true => H264Ref3FieldEn::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == H264Ref3FieldEn::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == H264Ref3FieldEn::B1
    }
}
#[doc = "Field `H264_REF3_FIELD_EN` writer - the type Refer picture consist of"]
pub type H264Ref3FieldEnW<'a, REG> = crate::BitWriter<'a, REG, H264Ref3FieldEn>;
impl<'a, REG> H264Ref3FieldEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(H264Ref3FieldEn::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(H264Ref3FieldEn::B1)
    }
}
#[doc = "Field `H264_REF3_ST_ADDR` reader - the start address of referance frame3"]
pub type H264Ref3StAddrR = crate::FieldReader<u32>;
#[doc = "Field `H264_REF3_ST_ADDR` writer - the start address of referance frame3"]
pub type H264Ref3StAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - Which field is more closer to current picture"]
    #[inline(always)]
    pub fn h264_ref3_closer_sel(&self) -> H264Ref3CloserSelR {
        H264Ref3CloserSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the type Refer picture consist of"]
    #[inline(always)]
    pub fn h264_ref3_field_en(&self) -> H264Ref3FieldEnR {
        H264Ref3FieldEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - the start address of referance frame3"]
    #[inline(always)]
    pub fn h264_ref3_st_addr(&self) -> H264Ref3StAddrR {
        H264Ref3StAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Which field is more closer to current picture"]
    #[inline(always)]
    #[must_use]
    pub fn h264_ref3_closer_sel(&mut self) -> H264Ref3CloserSelW<Swreg87Spec> {
        H264Ref3CloserSelW::new(self, 0)
    }
    #[doc = "Bit 1 - the type Refer picture consist of"]
    #[inline(always)]
    #[must_use]
    pub fn h264_ref3_field_en(&mut self) -> H264Ref3FieldEnW<Swreg87Spec> {
        H264Ref3FieldEnW::new(self, 1)
    }
    #[doc = "Bits 2:31 - the start address of referance frame3"]
    #[inline(always)]
    #[must_use]
    pub fn h264_ref3_st_addr(&mut self) -> H264Ref3StAddrW<Swreg87Spec> {
        H264Ref3StAddrW::new(self, 2)
    }
}
#[doc = "referance frame3 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg87::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg87::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg87Spec;
impl crate::RegisterSpec for Swreg87Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg87::R`](R) reader structure"]
impl crate::Readable for Swreg87Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg87::W`](W) writer structure"]
impl crate::Writable for Swreg87Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG87 to value 0"]
impl crate::Resettable for Swreg87Spec {
    const RESET_VALUE: u32 = 0;
}
