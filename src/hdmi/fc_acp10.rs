#[doc = "Register `FC_ACP10` reader"]
pub type R = crate::R<FcAcp10Spec>;
#[doc = "Register `FC_ACP10` writer"]
pub type W = crate::W<FcAcp10Spec>;
#[doc = "Field `FC_ACP10` reader - Frame Composer ACP Packet Body Configuration\n\nRegister 10"]
pub type FcAcp10R = crate::FieldReader;
#[doc = "Field `FC_ACP10` writer - Frame Composer ACP Packet Body Configuration\n\nRegister 10"]
pub type FcAcp10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration\n\nRegister 10"]
    #[inline(always)]
    pub fn fc_acp10(&self) -> FcAcp10R {
        FcAcp10R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration\n\nRegister 10"]
    #[inline(always)]
    #[must_use]
    pub fn fc_acp10(&mut self) -> FcAcp10W<FcAcp10Spec> {
        FcAcp10W::new(self, 0)
    }
}
#[doc = "Frame Composer ACP Packet Body Configuration Register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAcp10Spec;
impl crate::RegisterSpec for FcAcp10Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_acp10::R`](R) reader structure"]
impl crate::Readable for FcAcp10Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_acp10::W`](W) writer structure"]
impl crate::Writable for FcAcp10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ACP10 to value 0"]
impl crate::Resettable for FcAcp10Spec {
    const RESET_VALUE: u8 = 0;
}
