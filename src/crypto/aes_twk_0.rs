#[doc = "Register `AES_TWK_0` reader"]
pub type R = crate::R<AesTwk0Spec>;
#[doc = "Register `AES_TWK_0` writer"]
pub type W = crate::W<AesTwk0Spec>;
#[doc = "Field `AES_TWK_0` reader - Specifies AES-XTS tweak date\\[127:96\\]"]
pub type AesTwk0R = crate::FieldReader<u32>;
#[doc = "Field `AES_TWK_0` writer - Specifies AES-XTS tweak date\\[127:96\\]"]
pub type AesTwk0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak date\\[127:96\\]"]
    #[inline(always)]
    pub fn aes_twk_0(&self) -> AesTwk0R {
        AesTwk0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak date\\[127:96\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_twk_0(&mut self) -> AesTwk0W<AesTwk0Spec> {
        AesTwk0W::new(self, 0)
    }
}
#[doc = "AES Tweak data 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_twk_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_twk_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesTwk0Spec;
impl crate::RegisterSpec for AesTwk0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_twk_0::R`](R) reader structure"]
impl crate::Readable for AesTwk0Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_twk_0::W`](W) writer structure"]
impl crate::Writable for AesTwk0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_TWK_0 to value 0"]
impl crate::Resettable for AesTwk0Spec {
    const RESET_VALUE: u32 = 0;
}
