#[doc = "Register `FC_ACP11` reader"]
pub type R = crate::R<FcAcp11Spec>;
#[doc = "Register `FC_ACP11` writer"]
pub type W = crate::W<FcAcp11Spec>;
#[doc = "Field `FC_ACP11` reader - Frame Composer ACP Packet Body Configuration Register 11"]
pub type FcAcp11R = crate::FieldReader;
#[doc = "Field `FC_ACP11` writer - Frame Composer ACP Packet Body Configuration Register 11"]
pub type FcAcp11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration Register 11"]
    #[inline(always)]
    pub fn fc_acp11(&self) -> FcAcp11R {
        FcAcp11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration Register 11"]
    #[inline(always)]
    #[must_use]
    pub fn fc_acp11(&mut self) -> FcAcp11W<FcAcp11Spec> {
        FcAcp11W::new(self, 0)
    }
}
#[doc = "Frame Composer ACP Packet Body Configuration Register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAcp11Spec;
impl crate::RegisterSpec for FcAcp11Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_acp11::R`](R) reader structure"]
impl crate::Readable for FcAcp11Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_acp11::W`](W) writer structure"]
impl crate::Writable for FcAcp11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ACP11 to value 0"]
impl crate::Resettable for FcAcp11Spec {
    const RESET_VALUE: u8 = 0;
}
