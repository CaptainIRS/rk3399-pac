#[doc = "Register `AES_TKEY_6` reader"]
pub type R = crate::R<AesTkey6Spec>;
#[doc = "Register `AES_TKEY_6` writer"]
pub type W = crate::W<AesTkey6Spec>;
#[doc = "Field `AES_TKEY_6` reader - Specifies AES-XTS tweak key data \\[63:32\\]"]
pub type AesTkey6R = crate::FieldReader<u32>;
#[doc = "Field `AES_TKEY_6` writer - Specifies AES-XTS tweak key data \\[63:32\\]"]
pub type AesTkey6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak key data \\[63:32\\]"]
    #[inline(always)]
    pub fn aes_tkey_6(&self) -> AesTkey6R {
        AesTkey6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak key data \\[63:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_tkey_6(&mut self) -> AesTkey6W<AesTkey6Spec> {
        AesTkey6W::new(self, 0)
    }
}
#[doc = "AES Tweak Key data 6 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_tkey_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_tkey_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesTkey6Spec;
impl crate::RegisterSpec for AesTkey6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_tkey_6::R`](R) reader structure"]
impl crate::Readable for AesTkey6Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_tkey_6::W`](W) writer structure"]
impl crate::Writable for AesTkey6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_TKEY_6 to value 0"]
impl crate::Resettable for AesTkey6Spec {
    const RESET_VALUE: u32 = 0;
}
