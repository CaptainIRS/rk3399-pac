#[doc = "Register `FC_ACP4` reader"]
pub type R = crate::R<FcAcp4Spec>;
#[doc = "Register `FC_ACP4` writer"]
pub type W = crate::W<FcAcp4Spec>;
#[doc = "Field `FC_ACP4` reader - Frame Composer ACP Packet Body Configuration Register 4"]
pub type FcAcp4R = crate::FieldReader;
#[doc = "Field `FC_ACP4` writer - Frame Composer ACP Packet Body Configuration Register 4"]
pub type FcAcp4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration Register 4"]
    #[inline(always)]
    pub fn fc_acp4(&self) -> FcAcp4R {
        FcAcp4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration Register 4"]
    #[inline(always)]
    #[must_use]
    pub fn fc_acp4(&mut self) -> FcAcp4W<FcAcp4Spec> {
        FcAcp4W::new(self, 0)
    }
}
#[doc = "Frame Composer ACP Packet Body Configuration Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAcp4Spec;
impl crate::RegisterSpec for FcAcp4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_acp4::R`](R) reader structure"]
impl crate::Readable for FcAcp4Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_acp4::W`](W) writer structure"]
impl crate::Writable for FcAcp4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ACP4 to value 0"]
impl crate::Resettable for FcAcp4Spec {
    const RESET_VALUE: u8 = 0;
}
