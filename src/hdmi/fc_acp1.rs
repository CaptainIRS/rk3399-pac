#[doc = "Register `FC_ACP1` reader"]
pub type R = crate::R<FcAcp1Spec>;
#[doc = "Register `FC_ACP1` writer"]
pub type W = crate::W<FcAcp1Spec>;
#[doc = "Field `FC_ACP1` reader - Frame Composer ACP Packet Body Configuration Register 1"]
pub type FcAcp1R = crate::FieldReader;
#[doc = "Field `FC_ACP1` writer - Frame Composer ACP Packet Body Configuration Register 1"]
pub type FcAcp1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration Register 1"]
    #[inline(always)]
    pub fn fc_acp1(&self) -> FcAcp1R {
        FcAcp1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration Register 1"]
    #[inline(always)]
    #[must_use]
    pub fn fc_acp1(&mut self) -> FcAcp1W<FcAcp1Spec> {
        FcAcp1W::new(self, 0)
    }
}
#[doc = "Frame Composer ACP Packet Body Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAcp1Spec;
impl crate::RegisterSpec for FcAcp1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_acp1::R`](R) reader structure"]
impl crate::Readable for FcAcp1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_acp1::W`](W) writer structure"]
impl crate::Writable for FcAcp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ACP1 to value 0"]
impl crate::Resettable for FcAcp1Spec {
    const RESET_VALUE: u8 = 0;
}
