#[doc = "Register `FC_ACP5` reader"]
pub type R = crate::R<FcAcp5Spec>;
#[doc = "Register `FC_ACP5` writer"]
pub type W = crate::W<FcAcp5Spec>;
#[doc = "Field `FC_ACP5` reader - Frame Composer ACP Packet Body Configuration\n\nRegister 5"]
pub type FcAcp5R = crate::FieldReader;
#[doc = "Field `FC_ACP5` writer - Frame Composer ACP Packet Body Configuration\n\nRegister 5"]
pub type FcAcp5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration\n\nRegister 5"]
    #[inline(always)]
    pub fn fc_acp5(&self) -> FcAcp5R {
        FcAcp5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration\n\nRegister 5"]
    #[inline(always)]
    #[must_use]
    pub fn fc_acp5(&mut self) -> FcAcp5W<FcAcp5Spec> {
        FcAcp5W::new(self, 0)
    }
}
#[doc = "Frame Composer ACP Packet Body Configuration Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAcp5Spec;
impl crate::RegisterSpec for FcAcp5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_acp5::R`](R) reader structure"]
impl crate::Readable for FcAcp5Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_acp5::W`](W) writer structure"]
impl crate::Writable for FcAcp5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ACP5 to value 0"]
impl crate::Resettable for FcAcp5Spec {
    const RESET_VALUE: u8 = 0;
}
