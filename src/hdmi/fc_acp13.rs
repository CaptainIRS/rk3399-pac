#[doc = "Register `FC_ACP13` reader"]
pub type R = crate::R<FcAcp13Spec>;
#[doc = "Register `FC_ACP13` writer"]
pub type W = crate::W<FcAcp13Spec>;
#[doc = "Field `FC_ACP13` reader - Frame Composer ACP Packet Body Configuration\n\nRegister 13"]
pub type FcAcp13R = crate::FieldReader;
#[doc = "Field `FC_ACP13` writer - Frame Composer ACP Packet Body Configuration\n\nRegister 13"]
pub type FcAcp13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration\n\nRegister 13"]
    #[inline(always)]
    pub fn fc_acp13(&self) -> FcAcp13R {
        FcAcp13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration\n\nRegister 13"]
    #[inline(always)]
    #[must_use]
    pub fn fc_acp13(&mut self) -> FcAcp13W<FcAcp13Spec> {
        FcAcp13W::new(self, 0)
    }
}
#[doc = "Frame Composer ACP Packet Body Configuration Register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAcp13Spec;
impl crate::RegisterSpec for FcAcp13Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_acp13::R`](R) reader structure"]
impl crate::Readable for FcAcp13Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_acp13::W`](W) writer structure"]
impl crate::Writable for FcAcp13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ACP13 to value 0"]
impl crate::Resettable for FcAcp13Spec {
    const RESET_VALUE: u8 = 0;
}
