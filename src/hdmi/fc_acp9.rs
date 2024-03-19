#[doc = "Register `FC_ACP9` reader"]
pub type R = crate::R<FcAcp9Spec>;
#[doc = "Register `FC_ACP9` writer"]
pub type W = crate::W<FcAcp9Spec>;
#[doc = "Field `FC_ACP9` reader - Frame Composer ACP Packet Body Configuration\n\nRegister 9"]
pub type FcAcp9R = crate::FieldReader;
#[doc = "Field `FC_ACP9` writer - Frame Composer ACP Packet Body Configuration\n\nRegister 9"]
pub type FcAcp9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration\n\nRegister 9"]
    #[inline(always)]
    pub fn fc_acp9(&self) -> FcAcp9R {
        FcAcp9R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer ACP Packet Body Configuration\n\nRegister 9"]
    #[inline(always)]
    #[must_use]
    pub fn fc_acp9(&mut self) -> FcAcp9W<FcAcp9Spec> {
        FcAcp9W::new(self, 0)
    }
}
#[doc = "Frame Composer ACP Packet Body Configuration Register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_acp9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_acp9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAcp9Spec;
impl crate::RegisterSpec for FcAcp9Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_acp9::R`](R) reader structure"]
impl crate::Readable for FcAcp9Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_acp9::W`](W) writer structure"]
impl crate::Writable for FcAcp9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_ACP9 to value 0"]
impl crate::Resettable for FcAcp9Spec {
    const RESET_VALUE: u8 = 0;
}
