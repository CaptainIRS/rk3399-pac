#[doc = "Register `AES_KEY_3` reader"]
pub type R = crate::R<AesKey3Spec>;
#[doc = "Register `AES_KEY_3` writer"]
pub type W = crate::W<AesKey3Spec>;
#[doc = "Field `AES_KEY_3` reader - Specifies AES key data \\[159:128\\]"]
pub type AesKey3R = crate::FieldReader<u32>;
#[doc = "Field `AES_KEY_3` writer - Specifies AES key data \\[159:128\\]"]
pub type AesKey3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES key data \\[159:128\\]"]
    #[inline(always)]
    pub fn aes_key_3(&self) -> AesKey3R {
        AesKey3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES key data \\[159:128\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_key_3(&mut self) -> AesKey3W<AesKey3Spec> {
        AesKey3W::new(self, 0)
    }
}
#[doc = "AES Key data 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_key_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesKey3Spec;
impl crate::RegisterSpec for AesKey3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_key_3::R`](R) reader structure"]
impl crate::Readable for AesKey3Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_key_3::W`](W) writer structure"]
impl crate::Writable for AesKey3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_KEY_3 to value 0"]
impl crate::Resettable for AesKey3Spec {
    const RESET_VALUE: u32 = 0;
}
