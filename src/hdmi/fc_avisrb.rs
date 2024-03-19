#[doc = "Register `FC_AVISRB[%s]` reader"]
pub type R = crate::R<FcAvisrbSpec>;
#[doc = "Register `FC_AVISRB[%s]` writer"]
pub type W = crate::W<FcAvisrbSpec>;
#[doc = "Field `FC_AVISRB` reader - This register defines the AVI InfoFrame Start of\n\nRight Bar value. For more information, refer to the\n\nCEA-861-E specification."]
pub type FcAvisrbR = crate::FieldReader;
#[doc = "Field `FC_AVISRB` writer - This register defines the AVI InfoFrame Start of\n\nRight Bar value. For more information, refer to the\n\nCEA-861-E specification."]
pub type FcAvisrbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register defines the AVI InfoFrame Start of\n\nRight Bar value. For more information, refer to the\n\nCEA-861-E specification."]
    #[inline(always)]
    pub fn fc_avisrb(&self) -> FcAvisrbR {
        FcAvisrbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register defines the AVI InfoFrame Start of\n\nRight Bar value. For more information, refer to the\n\nCEA-861-E specification."]
    #[inline(always)]
    #[must_use]
    pub fn fc_avisrb(&mut self) -> FcAvisrbW<FcAvisrbSpec> {
        FcAvisrbW::new(self, 0)
    }
}
#[doc = "Frame Composer AVI Packet Start of Right Bar Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_avisrb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_avisrb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAvisrbSpec;
impl crate::RegisterSpec for FcAvisrbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_avisrb::R`](R) reader structure"]
impl crate::Readable for FcAvisrbSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_avisrb::W`](W) writer structure"]
impl crate::Writable for FcAvisrbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AVISRB[%s]
to value 0"]
impl crate::Resettable for FcAvisrbSpec {
    const RESET_VALUE: u8 = 0;
}
