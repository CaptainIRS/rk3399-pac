#[doc = "Register `FC_ACP12` reader"]
pub type R = crate::R<FcAcp12Spec>;
#[doc = "Register `FC_ACP12` writer"]
pub type W = crate::W<FcAcp12Spec>;
#[doc = "Field `FC_ACP12` reader - Frame Composer ACP Packet Body Configuration Register 12"]
pub type FcAcp12R = crate::FieldReader;
#[doc = "Field `FC_ACP12` writer - Frame Composer ACP Packet Body Configuration Register 12"]
pub type FcAcp12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration Register 12"]
    #[inline(always)]
    pub fn fc_acp12(&self) -> FcAcp12R {
        FcAcp12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration Register 12"]
    #[inline(always)]
    #[must_use]
    pub fn fc_acp12(&mut self) -> FcAcp12W<FcAcp12Spec> {
        FcAcp12W::new(self, 0)
    }
}
#[doc = "Frame Composer ACP Packet Body Configuration Register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAcp12Spec;
impl crate::RegisterSpec for FcAcp12Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_acp12::R`](R) reader structure"]
impl crate::Readable for FcAcp12Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_acp12::W`](W) writer structure"]
impl crate::Writable for FcAcp12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ACP12 to value 0"]
impl crate::Resettable for FcAcp12Spec {
    const RESET_VALUE: u8 = 0;
}
