#[doc = "Register `FC_ACP16` reader"]
pub type R = crate::R<FcAcp16Spec>;
#[doc = "Register `FC_ACP16` writer"]
pub type W = crate::W<FcAcp16Spec>;
#[doc = "Field `FC_ACP16` reader - Frame Composer ACP Packet Body Configuration\n\nRegister 16"]
pub type FcAcp16R = crate::FieldReader;
#[doc = "Field `FC_ACP16` writer - Frame Composer ACP Packet Body Configuration\n\nRegister 16"]
pub type FcAcp16W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration\n\nRegister 16"]
    #[inline(always)]
    pub fn fc_acp16(&self) -> FcAcp16R {
        FcAcp16R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration\n\nRegister 16"]
    #[inline(always)]
    #[must_use]
    pub fn fc_acp16(&mut self) -> FcAcp16W<FcAcp16Spec> {
        FcAcp16W::new(self, 0)
    }
}
#[doc = "Frame Composer ACP Packet Body Configuration Register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAcp16Spec;
impl crate::RegisterSpec for FcAcp16Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_acp16::R`](R) reader structure"]
impl crate::Readable for FcAcp16Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_acp16::W`](W) writer structure"]
impl crate::Writable for FcAcp16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ACP16 to value 0"]
impl crate::Resettable for FcAcp16Spec {
    const RESET_VALUE: u8 = 0;
}
