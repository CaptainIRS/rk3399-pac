#[doc = "Register `AES_KEY_5` reader"]
pub type R = crate::R<AesKey5Spec>;
#[doc = "Register `AES_KEY_5` writer"]
pub type W = crate::W<AesKey5Spec>;
#[doc = "Field `AES_KEY_5` reader - Specifies the key data \\[95:64\\]"]
pub type AesKey5R = crate::FieldReader<u32>;
#[doc = "Field `AES_KEY_5` writer - Specifies the key data \\[95:64\\]"]
pub type AesKey5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the key data \\[95:64\\]"]
    #[inline(always)]
    pub fn aes_key_5(&self) -> AesKey5R {
        AesKey5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies the key data \\[95:64\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_key_5(&mut self) -> AesKey5W<AesKey5Spec> {
        AesKey5W::new(self, 0)
    }
}
#[doc = "AES Key data 5 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_key_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesKey5Spec;
impl crate::RegisterSpec for AesKey5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_key_5::R`](R) reader structure"]
impl crate::Readable for AesKey5Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_key_5::W`](W) writer structure"]
impl crate::Writable for AesKey5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_KEY_5 to value 0"]
impl crate::Resettable for AesKey5Spec {
    const RESET_VALUE: u32 = 0;
}
