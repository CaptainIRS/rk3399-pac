#[doc = "Register `AES_KEY_0` reader"]
pub type R = crate::R<AesKey0Spec>;
#[doc = "Register `AES_KEY_0` writer"]
pub type W = crate::W<AesKey0Spec>;
#[doc = "Field `AES_KEY_0` reader - Specifies AES key data \\[255:224\\]"]
pub type AesKey0R = crate::FieldReader<u32>;
#[doc = "Field `AES_KEY_0` writer - Specifies AES key data \\[255:224\\]"]
pub type AesKey0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES key data \\[255:224\\]"]
    #[inline(always)]
    pub fn aes_key_0(&self) -> AesKey0R {
        AesKey0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES key data \\[255:224\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_key_0(&mut self) -> AesKey0W<AesKey0Spec> {
        AesKey0W::new(self, 0)
    }
}
#[doc = "AES Key data 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_key_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesKey0Spec;
impl crate::RegisterSpec for AesKey0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_key_0::R`](R) reader structure"]
impl crate::Readable for AesKey0Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_key_0::W`](W) writer structure"]
impl crate::Writable for AesKey0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_KEY_0 to value 0"]
impl crate::Resettable for AesKey0Spec {
    const RESET_VALUE: u32 = 0;
}
