#[doc = "Register `FC_ACP14` reader"]
pub type R = crate::R<FcAcp14Spec>;
#[doc = "Register `FC_ACP14` writer"]
pub type W = crate::W<FcAcp14Spec>;
#[doc = "Field `FC_ACP14` reader - Frame Composer ACP Packet Body Configuration\n\nRegister 14"]
pub type FcAcp14R = crate::FieldReader;
#[doc = "Field `FC_ACP14` writer - Frame Composer ACP Packet Body Configuration\n\nRegister 14"]
pub type FcAcp14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration\n\nRegister 14"]
    #[inline(always)]
    pub fn fc_acp14(&self) -> FcAcp14R {
        FcAcp14R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration\n\nRegister 14"]
    #[inline(always)]
    #[must_use]
    pub fn fc_acp14(&mut self) -> FcAcp14W<FcAcp14Spec> {
        FcAcp14W::new(self, 0)
    }
}
#[doc = "Frame Composer ACP Packet Body Configuration Register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAcp14Spec;
impl crate::RegisterSpec for FcAcp14Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_acp14::R`](R) reader structure"]
impl crate::Readable for FcAcp14Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_acp14::W`](W) writer structure"]
impl crate::Writable for FcAcp14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ACP14 to value 0"]
impl crate::Resettable for FcAcp14Spec {
    const RESET_VALUE: u8 = 0;
}
