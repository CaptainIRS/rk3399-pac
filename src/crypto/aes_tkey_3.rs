#[doc = "Register `AES_TKEY_3` reader"]
pub type R = crate::R<AesTkey3Spec>;
#[doc = "Register `AES_TKEY_3` writer"]
pub type W = crate::W<AesTkey3Spec>;
#[doc = "Field `AES_TKEY_3` reader - Specifies AES-XTS tweak key data \\[159:128\\]"]
pub type AesTkey3R = crate::FieldReader<u32>;
#[doc = "Field `AES_TKEY_3` writer - Specifies AES-XTS tweak key data \\[159:128\\]"]
pub type AesTkey3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak key data \\[159:128\\]"]
    #[inline(always)]
    pub fn aes_tkey_3(&self) -> AesTkey3R {
        AesTkey3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak key data \\[159:128\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_tkey_3(&mut self) -> AesTkey3W<AesTkey3Spec> {
        AesTkey3W::new(self, 0)
    }
}
#[doc = "AES Tweak Key data 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_tkey_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_tkey_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesTkey3Spec;
impl crate::RegisterSpec for AesTkey3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_tkey_3::R`](R) reader structure"]
impl crate::Readable for AesTkey3Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_tkey_3::W`](W) writer structure"]
impl crate::Writable for AesTkey3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_TKEY_3 to value 0"]
impl crate::Resettable for AesTkey3Spec {
    const RESET_VALUE: u32 = 0;
}
