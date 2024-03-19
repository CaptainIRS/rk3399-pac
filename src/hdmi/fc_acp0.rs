#[doc = "Register `FC_ACP0` reader"]
pub type R = crate::R<FcAcp0Spec>;
#[doc = "Register `FC_ACP0` writer"]
pub type W = crate::W<FcAcp0Spec>;
#[doc = "Field `ACPTYPE` reader - Configures the ACP packet type."]
pub type AcptypeR = crate::FieldReader;
#[doc = "Field `ACPTYPE` writer - Configures the ACP packet type."]
pub type AcptypeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configures the ACP packet type."]
    #[inline(always)]
    pub fn acptype(&self) -> AcptypeR {
        AcptypeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the ACP packet type."]
    #[inline(always)]
    #[must_use]
    pub fn acptype(&mut self) -> AcptypeW<FcAcp0Spec> {
        AcptypeW::new(self, 0)
    }
}
#[doc = "Frame Composer ACP Packet Type Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAcp0Spec;
impl crate::RegisterSpec for FcAcp0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_acp0::R`](R) reader structure"]
impl crate::Readable for FcAcp0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_acp0::W`](W) writer structure"]
impl crate::Writable for FcAcp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ACP0 to value 0"]
impl crate::Resettable for FcAcp0Spec {
    const RESET_VALUE: u8 = 0;
}
