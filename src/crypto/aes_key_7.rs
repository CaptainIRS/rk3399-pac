#[doc = "Register `AES_KEY_7` reader"]
pub type R = crate::R<AesKey7Spec>;
#[doc = "Register `AES_KEY_7` writer"]
pub type W = crate::W<AesKey7Spec>;
#[doc = "Field `AES_KEY_7` reader - Specifies the key data \\[31:0\\]"]
pub type AesKey7R = crate::FieldReader<u32>;
#[doc = "Field `AES_KEY_7` writer - Specifies the key data \\[31:0\\]"]
pub type AesKey7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the key data \\[31:0\\]"]
    #[inline(always)]
    pub fn aes_key_7(&self) -> AesKey7R {
        AesKey7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies the key data \\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_key_7(&mut self) -> AesKey7W<AesKey7Spec> {
        AesKey7W::new(self, 0)
    }
}
#[doc = "AES Key data 7 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_key_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesKey7Spec;
impl crate::RegisterSpec for AesKey7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_key_7::R`](R) reader structure"]
impl crate::Readable for AesKey7Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_key_7::W`](W) writer structure"]
impl crate::Writable for AesKey7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_KEY_7 to value 0"]
impl crate::Resettable for AesKey7Spec {
    const RESET_VALUE: u32 = 0;
}
