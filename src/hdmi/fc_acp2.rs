#[doc = "Register `FC_ACP2` reader"]
pub type R = crate::R<FcAcp2Spec>;
#[doc = "Register `FC_ACP2` writer"]
pub type W = crate::W<FcAcp2Spec>;
#[doc = "Field `FC_ACP2` reader - Frame Composer ACP Packet Body Configuration\n\nRegister 2"]
pub type FcAcp2R = crate::FieldReader;
#[doc = "Field `FC_ACP2` writer - Frame Composer ACP Packet Body Configuration\n\nRegister 2"]
pub type FcAcp2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration\n\nRegister 2"]
    #[inline(always)]
    pub fn fc_acp2(&self) -> FcAcp2R {
        FcAcp2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration\n\nRegister 2"]
    #[inline(always)]
    #[must_use]
    pub fn fc_acp2(&mut self) -> FcAcp2W<FcAcp2Spec> {
        FcAcp2W::new(self, 0)
    }
}
#[doc = "Frame Composer ACP Packet Body Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAcp2Spec;
impl crate::RegisterSpec for FcAcp2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_acp2::R`](R) reader structure"]
impl crate::Readable for FcAcp2Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_acp2::W`](W) writer structure"]
impl crate::Writable for FcAcp2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ACP2 to value 0"]
impl crate::Resettable for FcAcp2Spec {
    const RESET_VALUE: u8 = 0;
}
