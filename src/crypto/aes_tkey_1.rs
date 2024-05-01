#[doc = "Register `AES_TKEY_1` reader"]
pub type R = crate::R<AesTkey1Spec>;
#[doc = "Register `AES_TKEY_1` writer"]
pub type W = crate::W<AesTkey1Spec>;
#[doc = "Field `AES_TKEY_1` reader - Specifies AES-XTS tweak key data \\[223:192\\]"]
pub type AesTkey1R = crate::FieldReader<u32>;
#[doc = "Field `AES_TKEY_1` writer - Specifies AES-XTS tweak key data \\[223:192\\]"]
pub type AesTkey1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak key data \\[223:192\\]"]
    #[inline(always)]
    pub fn aes_tkey_1(&self) -> AesTkey1R {
        AesTkey1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak key data \\[223:192\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_tkey_1(&mut self) -> AesTkey1W<AesTkey1Spec> {
        AesTkey1W::new(self, 0)
    }
}
#[doc = "AES Tweak Key data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_tkey_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_tkey_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesTkey1Spec;
impl crate::RegisterSpec for AesTkey1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_tkey_1::R`](R) reader structure"]
impl crate::Readable for AesTkey1Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_tkey_1::W`](W) writer structure"]
impl crate::Writable for AesTkey1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_TKEY_1 to value 0"]
impl crate::Resettable for AesTkey1Spec {
    const RESET_VALUE: u32 = 0;
}
