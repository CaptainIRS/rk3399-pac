#[doc = "Register `AES_TKEY_2` reader"]
pub type R = crate::R<AesTkey2Spec>;
#[doc = "Register `AES_TKEY_2` writer"]
pub type W = crate::W<AesTkey2Spec>;
#[doc = "Field `AES_TKEY_2` reader - Specifies AES-XTS tweak key data \\[191:160\\]"]
pub type AesTkey2R = crate::FieldReader<u32>;
#[doc = "Field `AES_TKEY_2` writer - Specifies AES-XTS tweak key data \\[191:160\\]"]
pub type AesTkey2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak key data \\[191:160\\]"]
    #[inline(always)]
    pub fn aes_tkey_2(&self) -> AesTkey2R {
        AesTkey2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES-XTS tweak key data \\[191:160\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_tkey_2(&mut self) -> AesTkey2W<AesTkey2Spec> {
        AesTkey2W::new(self, 0)
    }
}
#[doc = "AES Tweak Key data 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_tkey_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_tkey_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesTkey2Spec;
impl crate::RegisterSpec for AesTkey2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_tkey_2::R`](R) reader structure"]
impl crate::Readable for AesTkey2Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_tkey_2::W`](W) writer structure"]
impl crate::Writable for AesTkey2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_TKEY_2 to value 0"]
impl crate::Resettable for AesTkey2Spec {
    const RESET_VALUE: u32 = 0;
}
