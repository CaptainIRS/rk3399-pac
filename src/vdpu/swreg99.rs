#[doc = "Register `SWREG99` reader"]
pub type R = crate::R<Swreg99Spec>;
#[doc = "Register `SWREG99` writer"]
pub type W = crate::W<Swreg99Spec>;
#[doc = "Which field is more closer to current picture\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264Ref15CloserSel {
    #[doc = "0: bottom field be selected"]
    B0 = 0,
    #[doc = "1: top field be selected"]
    B1 = 1,
}
impl From<H264Ref15CloserSel> for bool {
    #[inline(always)]
    fn from(variant: H264Ref15CloserSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H264_REF15_CLOSER_SEL` reader - Which field is more closer to current picture"]
pub type H264Ref15CloserSelR = crate::BitReader<H264Ref15CloserSel>;
impl H264Ref15CloserSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264Ref15CloserSel {
        match self.bits {
            false => H264Ref15CloserSel::B0,
            true => H264Ref15CloserSel::B1,
        }
    }
    #[doc = "bottom field be selected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == H264Ref15CloserSel::B0
    }
    #[doc = "top field be selected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == H264Ref15CloserSel::B1
    }
}
#[doc = "Field `H264_REF15_CLOSER_SEL` writer - Which field is more closer to current picture"]
pub type H264Ref15CloserSelW<'a, REG> = crate::BitWriter<'a, REG, H264Ref15CloserSel>;
impl<'a, REG> H264Ref15CloserSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bottom field be selected"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(H264Ref15CloserSel::B0)
    }
    #[doc = "top field be selected"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(H264Ref15CloserSel::B1)
    }
}
#[doc = "the type Refer picture consist of\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H264Ref15FieldEn {
    #[doc = "0: frame"]
    B0 = 0,
    #[doc = "1: field"]
    B1 = 1,
}
impl From<H264Ref15FieldEn> for bool {
    #[inline(always)]
    fn from(variant: H264Ref15FieldEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H264_REF15_FIELD_EN` reader - the type Refer picture consist of"]
pub type H264Ref15FieldEnR = crate::BitReader<H264Ref15FieldEn>;
impl H264Ref15FieldEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H264Ref15FieldEn {
        match self.bits {
            false => H264Ref15FieldEn::B0,
            true => H264Ref15FieldEn::B1,
        }
    }
    #[doc = "frame"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == H264Ref15FieldEn::B0
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == H264Ref15FieldEn::B1
    }
}
#[doc = "Field `H264_REF15_FIELD_EN` writer - the type Refer picture consist of"]
pub type H264Ref15FieldEnW<'a, REG> = crate::BitWriter<'a, REG, H264Ref15FieldEn>;
impl<'a, REG> H264Ref15FieldEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "frame"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(H264Ref15FieldEn::B0)
    }
    #[doc = "field"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(H264Ref15FieldEn::B1)
    }
}
#[doc = "Field `H264_REF15_ST_ADDR` reader - the start address of referance frame15"]
pub type H264Ref15StAddrR = crate::FieldReader<u32>;
#[doc = "Field `H264_REF15_ST_ADDR` writer - the start address of referance frame15"]
pub type H264Ref15StAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - Which field is more closer to current picture"]
    #[inline(always)]
    pub fn h264_ref15_closer_sel(&self) -> H264Ref15CloserSelR {
        H264Ref15CloserSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the type Refer picture consist of"]
    #[inline(always)]
    pub fn h264_ref15_field_en(&self) -> H264Ref15FieldEnR {
        H264Ref15FieldEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - the start address of referance frame15"]
    #[inline(always)]
    pub fn h264_ref15_st_addr(&self) -> H264Ref15StAddrR {
        H264Ref15StAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Which field is more closer to current picture"]
    #[inline(always)]
    #[must_use]
    pub fn h264_ref15_closer_sel(&mut self) -> H264Ref15CloserSelW<Swreg99Spec> {
        H264Ref15CloserSelW::new(self, 0)
    }
    #[doc = "Bit 1 - the type Refer picture consist of"]
    #[inline(always)]
    #[must_use]
    pub fn h264_ref15_field_en(&mut self) -> H264Ref15FieldEnW<Swreg99Spec> {
        H264Ref15FieldEnW::new(self, 1)
    }
    #[doc = "Bits 2:31 - the start address of referance frame15"]
    #[inline(always)]
    #[must_use]
    pub fn h264_ref15_st_addr(&mut self) -> H264Ref15StAddrW<Swreg99Spec> {
        H264Ref15StAddrW::new(self, 2)
    }
}
#[doc = "referance frame15 address for h264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg99::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg99::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg99Spec;
impl crate::RegisterSpec for Swreg99Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg99::R`](R) reader structure"]
impl crate::Readable for Swreg99Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg99::W`](W) writer structure"]
impl crate::Writable for Swreg99Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG99 to value 0"]
impl crate::Resettable for Swreg99Spec {
    const RESET_VALUE: u32 = 0;
}
