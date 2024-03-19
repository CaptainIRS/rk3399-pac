#[doc = "Register `FC_ACP8` reader"]
pub type R = crate::R<FcAcp8Spec>;
#[doc = "Register `FC_ACP8` writer"]
pub type W = crate::W<FcAcp8Spec>;
#[doc = "Field `FC_ACP8` reader - Frame Composer ACP Packet Body Configuration\n\nRegister 8"]
pub type FcAcp8R = crate::FieldReader;
#[doc = "Field `FC_ACP8` writer - Frame Composer ACP Packet Body Configuration\n\nRegister 8"]
pub type FcAcp8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration\n\nRegister 8"]
    #[inline(always)]
    pub fn fc_acp8(&self) -> FcAcp8R {
        FcAcp8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration\n\nRegister 8"]
    #[inline(always)]
    #[must_use]
    pub fn fc_acp8(&mut self) -> FcAcp8W<FcAcp8Spec> {
        FcAcp8W::new(self, 0)
    }
}
#[doc = "Frame Composer ACP Packet Body Configuration Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAcp8Spec;
impl crate::RegisterSpec for FcAcp8Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_acp8::R`](R) reader structure"]
impl crate::Readable for FcAcp8Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_acp8::W`](W) writer structure"]
impl crate::Writable for FcAcp8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ACP8 to value 0"]
impl crate::Resettable for FcAcp8Spec {
    const RESET_VALUE: u8 = 0;
}
