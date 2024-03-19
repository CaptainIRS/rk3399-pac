#[doc = "Register `FC_ACP3` reader"]
pub type R = crate::R<FcAcp3Spec>;
#[doc = "Register `FC_ACP3` writer"]
pub type W = crate::W<FcAcp3Spec>;
#[doc = "Field `FC_ACP3` reader - Frame Composer ACP Packet Body Configuration\n\nRegister 3"]
pub type FcAcp3R = crate::FieldReader;
#[doc = "Field `FC_ACP3` writer - Frame Composer ACP Packet Body Configuration\n\nRegister 3"]
pub type FcAcp3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration\n\nRegister 3"]
    #[inline(always)]
    pub fn fc_acp3(&self) -> FcAcp3R {
        FcAcp3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration\n\nRegister 3"]
    #[inline(always)]
    #[must_use]
    pub fn fc_acp3(&mut self) -> FcAcp3W<FcAcp3Spec> {
        FcAcp3W::new(self, 0)
    }
}
#[doc = "Frame Composer ACP Packet Body Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAcp3Spec;
impl crate::RegisterSpec for FcAcp3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_acp3::R`](R) reader structure"]
impl crate::Readable for FcAcp3Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_acp3::W`](W) writer structure"]
impl crate::Writable for FcAcp3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ACP3 to value 0"]
impl crate::Resettable for FcAcp3Spec {
    const RESET_VALUE: u8 = 0;
}
