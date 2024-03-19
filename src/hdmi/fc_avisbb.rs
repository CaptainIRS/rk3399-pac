#[doc = "Register `FC_AVISBB[%s]` reader"]
pub type R = crate::R<FcAvisbbSpec>;
#[doc = "Register `FC_AVISBB[%s]` writer"]
pub type W = crate::W<FcAvisbbSpec>;
#[doc = "Field `FC_AVISBB` reader - This register defines the AVI InfoFrame Start of\n\nBottom Bar value. For more information, refer to\n\nthe CEA-861-E specification."]
pub type FcAvisbbR = crate::FieldReader;
#[doc = "Field `FC_AVISBB` writer - This register defines the AVI InfoFrame Start of\n\nBottom Bar value. For more information, refer to\n\nthe CEA-861-E specification."]
pub type FcAvisbbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register defines the AVI InfoFrame Start of\n\nBottom Bar value. For more information, refer to\n\nthe CEA-861-E specification."]
    #[inline(always)]
    pub fn fc_avisbb(&self) -> FcAvisbbR {
        FcAvisbbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register defines the AVI InfoFrame Start of\n\nBottom Bar value. For more information, refer to\n\nthe CEA-861-E specification."]
    #[inline(always)]
    #[must_use]
    pub fn fc_avisbb(&mut self) -> FcAvisbbW<FcAvisbbSpec> {
        FcAvisbbW::new(self, 0)
    }
}
#[doc = "Frame Composer AVI Packet Start of Bottom Bar Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_avisbb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_avisbb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAvisbbSpec;
impl crate::RegisterSpec for FcAvisbbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_avisbb::R`](R) reader structure"]
impl crate::Readable for FcAvisbbSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_avisbb::W`](W) writer structure"]
impl crate::Writable for FcAvisbbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AVISBB[%s]
to value 0"]
impl crate::Resettable for FcAvisbbSpec {
    const RESET_VALUE: u8 = 0;
}
