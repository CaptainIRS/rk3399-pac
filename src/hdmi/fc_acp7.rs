#[doc = "Register `FC_ACP7` reader"]
pub type R = crate::R<FcAcp7Spec>;
#[doc = "Register `FC_ACP7` writer"]
pub type W = crate::W<FcAcp7Spec>;
#[doc = "Field `FC_ACP7` reader - Frame Composer ACP Packet Body Configuration Register 7"]
pub type FcAcp7R = crate::FieldReader;
#[doc = "Field `FC_ACP7` writer - Frame Composer ACP Packet Body Configuration Register 7"]
pub type FcAcp7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration Register 7"]
    #[inline(always)]
    pub fn fc_acp7(&self) -> FcAcp7R {
        FcAcp7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration Register 7"]
    #[inline(always)]
    #[must_use]
    pub fn fc_acp7(&mut self) -> FcAcp7W<FcAcp7Spec> {
        FcAcp7W::new(self, 0)
    }
}
#[doc = "Frame Composer ACP Packet Body Configuration Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAcp7Spec;
impl crate::RegisterSpec for FcAcp7Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_acp7::R`](R) reader structure"]
impl crate::Readable for FcAcp7Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_acp7::W`](W) writer structure"]
impl crate::Writable for FcAcp7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ACP7 to value 0"]
impl crate::Resettable for FcAcp7Spec {
    const RESET_VALUE: u8 = 0;
}
