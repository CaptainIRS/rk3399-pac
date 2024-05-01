#[doc = "Register `AES_TWK_3` reader"]
pub type R = crate::R<AesTwk3Spec>;
#[doc = "Register `AES_TWK_3` writer"]
pub type W = crate::W<AesTwk3Spec>;
#[doc = "Field `AES_TWK_3` reader - Specifies AES-XTS tweak date\\[31:0\\]"]
pub type AesTwk3R = crate::FieldReader<u32>;
#[doc = "Field `AES_TWK_3` writer - Specifies AES-XTS tweak date\\[31:0\\]"]
pub type AesTwk3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak date\\[31:0\\]"]
    #[inline(always)]
    pub fn aes_twk_3(&self) -> AesTwk3R {
        AesTwk3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak date\\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_twk_3(&mut self) -> AesTwk3W<AesTwk3Spec> {
        AesTwk3W::new(self, 0)
    }
}
#[doc = "AES Tweak data 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_twk_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_twk_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesTwk3Spec;
impl crate::RegisterSpec for AesTwk3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_twk_3::R`](R) reader structure"]
impl crate::Readable for AesTwk3Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_twk_3::W`](W) writer structure"]
impl crate::Writable for AesTwk3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_TWK_3 to value 0"]
impl crate::Resettable for AesTwk3Spec {
    const RESET_VALUE: u32 = 0;
}
