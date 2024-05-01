#[doc = "Register `SWREG_98` reader"]
pub type R = crate::R<Swreg98Spec>;
#[doc = "Register `SWREG_98` writer"]
pub type W = crate::W<Swreg98Spec>;
#[doc = "Field `RCMPT_MASK_POSTITION` reader - the mask msb bit position of rgb R-component\n\nrange : 0~31"]
pub type RcmptMaskPostitionR = crate::FieldReader;
#[doc = "Field `RCMPT_MASK_POSTITION` writer - the mask msb bit position of rgb R-component\n\nrange : 0~31"]
pub type RcmptMaskPostitionW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `GCMPT_MASK_POSTITION` reader - the mask msb bit position of rgb G-component\n\nrange : 0~31"]
pub type GcmptMaskPostitionR = crate::FieldReader;
#[doc = "Field `GCMPT_MASK_POSTITION` writer - the mask msb bit position of rgb G-component\n\nrange : 0~31"]
pub type GcmptMaskPostitionW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BCMPT_MASK_POSTITION` reader - the mask msb bit position of rgb B-component\n\nrange : 0~31"]
pub type BcmptMaskPostitionR = crate::FieldReader;
#[doc = "Field `BCMPT_MASK_POSTITION` writer - the mask msb bit position of rgb B-component\n\nrange : 0~31"]
pub type BcmptMaskPostitionW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - the mask msb bit position of rgb R-component\n\nrange : 0~31"]
    #[inline(always)]
    pub fn rcmpt_mask_postition(&self) -> RcmptMaskPostitionR {
        RcmptMaskPostitionR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - the mask msb bit position of rgb G-component\n\nrange : 0~31"]
    #[inline(always)]
    pub fn gcmpt_mask_postition(&self) -> GcmptMaskPostitionR {
        GcmptMaskPostitionR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - the mask msb bit position of rgb B-component\n\nrange : 0~31"]
    #[inline(always)]
    pub fn bcmpt_mask_postition(&self) -> BcmptMaskPostitionR {
        BcmptMaskPostitionR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - the mask msb bit position of rgb R-component\n\nrange : 0~31"]
    #[inline(always)]
    #[must_use]
    pub fn rcmpt_mask_postition(&mut self) -> RcmptMaskPostitionW<Swreg98Spec> {
        RcmptMaskPostitionW::new(self, 0)
    }
    #[doc = "Bits 8:12 - the mask msb bit position of rgb G-component\n\nrange : 0~31"]
    #[inline(always)]
    #[must_use]
    pub fn gcmpt_mask_postition(&mut self) -> GcmptMaskPostitionW<Swreg98Spec> {
        GcmptMaskPostitionW::new(self, 8)
    }
    #[doc = "Bits 16:20 - the mask msb bit position of rgb B-component\n\nrange : 0~31"]
    #[inline(always)]
    #[must_use]
    pub fn bcmpt_mask_postition(&mut self) -> BcmptMaskPostitionW<Swreg98Spec> {
        BcmptMaskPostitionW::new(self, 16)
    }
}
#[doc = "RGA MASK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_98::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_98::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg98Spec;
impl crate::RegisterSpec for Swreg98Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_98::R`](R) reader structure"]
impl crate::Readable for Swreg98Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_98::W`](W) writer structure"]
impl crate::Writable for Swreg98Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_98 to value 0"]
impl crate::Resettable for Swreg98Spec {
    const RESET_VALUE: u32 = 0;
}
