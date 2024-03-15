#[doc = "Register `FC_ACP6` reader"]
pub type R = crate::R<FcAcp6Spec>;
#[doc = "Register `FC_ACP6` writer"]
pub type W = crate::W<FcAcp6Spec>;
#[doc = "Field `FC_ACP6` reader - Frame Composer ACP Packet Body Configuration Register 6"]
pub type FcAcp6R = crate::FieldReader;
#[doc = "Field `FC_ACP6` writer - Frame Composer ACP Packet Body Configuration Register 6"]
pub type FcAcp6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration Register 6"]
    #[inline(always)]
    pub fn fc_acp6(&self) -> FcAcp6R {
        FcAcp6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration Register 6"]
    #[inline(always)]
    #[must_use]
    pub fn fc_acp6(&mut self) -> FcAcp6W<FcAcp6Spec> {
        FcAcp6W::new(self, 0)
    }
}
#[doc = "Frame Composer ACP Packet Body Configuration Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAcp6Spec;
impl crate::RegisterSpec for FcAcp6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_acp6::R`](R) reader structure"]
impl crate::Readable for FcAcp6Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_acp6::W`](W) writer structure"]
impl crate::Writable for FcAcp6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ACP6 to value 0"]
impl crate::Resettable for FcAcp6Spec {
    const RESET_VALUE: u8 = 0;
}
