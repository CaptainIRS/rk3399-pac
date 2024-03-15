#[doc = "Register `FC_ACP15` reader"]
pub type R = crate::R<FcAcp15Spec>;
#[doc = "Register `FC_ACP15` writer"]
pub type W = crate::W<FcAcp15Spec>;
#[doc = "Field `FC_ACP15` reader - Frame Composer ACP Packet Body Configuration Register 15"]
pub type FcAcp15R = crate::FieldReader;
#[doc = "Field `FC_ACP15` writer - Frame Composer ACP Packet Body Configuration Register 15"]
pub type FcAcp15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration Register 15"]
    #[inline(always)]
    pub fn fc_acp15(&self) -> FcAcp15R {
        FcAcp15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration Register 15"]
    #[inline(always)]
    #[must_use]
    pub fn fc_acp15(&mut self) -> FcAcp15W<FcAcp15Spec> {
        FcAcp15W::new(self, 0)
    }
}
#[doc = "Frame Composer ACP Packet Body Configuration Register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAcp15Spec;
impl crate::RegisterSpec for FcAcp15Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_acp15::R`](R) reader structure"]
impl crate::Readable for FcAcp15Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_acp15::W`](W) writer structure"]
impl crate::Writable for FcAcp15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ACP15 to value 0"]
impl crate::Resettable for FcAcp15Spec {
    const RESET_VALUE: u8 = 0;
}
