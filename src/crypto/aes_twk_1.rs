#[doc = "Register `AES_TWK_1` reader"]
pub type R = crate::R<AesTwk1Spec>;
#[doc = "Register `AES_TWK_1` writer"]
pub type W = crate::W<AesTwk1Spec>;
#[doc = "Field `AES_TWK_1` reader - Specifies AES-XTS tweak date\\[95:64\\]"]
pub type AesTwk1R = crate::FieldReader<u32>;
#[doc = "Field `AES_TWK_1` writer - Specifies AES-XTS tweak date\\[95:64\\]"]
pub type AesTwk1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak date\\[95:64\\]"]
    #[inline(always)]
    pub fn aes_twk_1(&self) -> AesTwk1R {
        AesTwk1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak date\\[95:64\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_twk_1(&mut self) -> AesTwk1W<AesTwk1Spec> {
        AesTwk1W::new(self, 0)
    }
}
#[doc = "AES Tweak data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_twk_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_twk_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesTwk1Spec;
impl crate::RegisterSpec for AesTwk1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_twk_1::R`](R) reader structure"]
impl crate::Readable for AesTwk1Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_twk_1::W`](W) writer structure"]
impl crate::Writable for AesTwk1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_TWK_1 to value 0"]
impl crate::Resettable for AesTwk1Spec {
    const RESET_VALUE: u32 = 0;
}
