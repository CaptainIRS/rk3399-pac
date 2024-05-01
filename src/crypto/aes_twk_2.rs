#[doc = "Register `AES_TWK_2` reader"]
pub type R = crate::R<AesTwk2Spec>;
#[doc = "Register `AES_TWK_2` writer"]
pub type W = crate::W<AesTwk2Spec>;
#[doc = "Field `AES_TWK_2` reader - Specifies AES-XTS tweak date\\[63:32\\]"]
pub type AesTwk2R = crate::FieldReader<u32>;
#[doc = "Field `AES_TWK_2` writer - Specifies AES-XTS tweak date\\[63:32\\]"]
pub type AesTwk2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak date\\[63:32\\]"]
    #[inline(always)]
    pub fn aes_twk_2(&self) -> AesTwk2R {
        AesTwk2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak date\\[63:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_twk_2(&mut self) -> AesTwk2W<AesTwk2Spec> {
        AesTwk2W::new(self, 0)
    }
}
#[doc = "AES Tweak data 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_twk_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_twk_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesTwk2Spec;
impl crate::RegisterSpec for AesTwk2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_twk_2::R`](R) reader structure"]
impl crate::Readable for AesTwk2Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_twk_2::W`](W) writer structure"]
impl crate::Writable for AesTwk2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_TWK_2 to value 0"]
impl crate::Resettable for AesTwk2Spec {
    const RESET_VALUE: u32 = 0;
}
