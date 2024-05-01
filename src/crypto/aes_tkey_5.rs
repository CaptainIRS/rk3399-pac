#[doc = "Register `AES_TKEY_5` reader"]
pub type R = crate::R<AesTkey5Spec>;
#[doc = "Register `AES_TKEY_5` writer"]
pub type W = crate::W<AesTkey5Spec>;
#[doc = "Field `AES_TKEY_5` reader - Specifies AES-XTS tweak key data \\[95:64\\]"]
pub type AesTkey5R = crate::FieldReader<u32>;
#[doc = "Field `AES_TKEY_5` writer - Specifies AES-XTS tweak key data \\[95:64\\]"]
pub type AesTkey5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak key data \\[95:64\\]"]
    #[inline(always)]
    pub fn aes_tkey_5(&self) -> AesTkey5R {
        AesTkey5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak key data \\[95:64\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_tkey_5(&mut self) -> AesTkey5W<AesTkey5Spec> {
        AesTkey5W::new(self, 0)
    }
}
#[doc = "AES Tweak Key data 5 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_tkey_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_tkey_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesTkey5Spec;
impl crate::RegisterSpec for AesTkey5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_tkey_5::R`](R) reader structure"]
impl crate::Readable for AesTkey5Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_tkey_5::W`](W) writer structure"]
impl crate::Writable for AesTkey5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_TKEY_5 to value 0"]
impl crate::Resettable for AesTkey5Spec {
    const RESET_VALUE: u32 = 0;
}
