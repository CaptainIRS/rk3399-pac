#[doc = "Register `AES_TKEY_4` reader"]
pub type R = crate::R<AesTkey4Spec>;
#[doc = "Register `AES_TKEY_4` writer"]
pub type W = crate::W<AesTkey4Spec>;
#[doc = "Field `AES_TKEY_4` reader - Specifies AES-XTS tweak key data \\[127:96\\]"]
pub type AesTkey4R = crate::FieldReader<u32>;
#[doc = "Field `AES_TKEY_4` writer - Specifies AES-XTS tweak key data \\[127:96\\]"]
pub type AesTkey4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak key data \\[127:96\\]"]
    #[inline(always)]
    pub fn aes_tkey_4(&self) -> AesTkey4R {
        AesTkey4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak key data \\[127:96\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_tkey_4(&mut self) -> AesTkey4W<AesTkey4Spec> {
        AesTkey4W::new(self, 0)
    }
}
#[doc = "AES Tweak Key data 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_tkey_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_tkey_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesTkey4Spec;
impl crate::RegisterSpec for AesTkey4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_tkey_4::R`](R) reader structure"]
impl crate::Readable for AesTkey4Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_tkey_4::W`](W) writer structure"]
impl crate::Writable for AesTkey4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_TKEY_4 to value 0"]
impl crate::Resettable for AesTkey4Spec {
    const RESET_VALUE: u32 = 0;
}
