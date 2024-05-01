#[doc = "Register `AES_KEY_1` reader"]
pub type R = crate::R<AesKey1Spec>;
#[doc = "Register `AES_KEY_1` writer"]
pub type W = crate::W<AesKey1Spec>;
#[doc = "Field `AES_KEY_1` reader - Specifies AES key data \\[223:192\\]"]
pub type AesKey1R = crate::FieldReader<u32>;
#[doc = "Field `AES_KEY_1` writer - Specifies AES key data \\[223:192\\]"]
pub type AesKey1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies AES key data \\[223:192\\]"]
    #[inline(always)]
    pub fn aes_key_1(&self) -> AesKey1R {
        AesKey1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies AES key data \\[223:192\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_key_1(&mut self) -> AesKey1W<AesKey1Spec> {
        AesKey1W::new(self, 0)
    }
}
#[doc = "AES Key data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_key_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesKey1Spec;
impl crate::RegisterSpec for AesKey1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_key_1::R`](R) reader structure"]
impl crate::Readable for AesKey1Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_key_1::W`](W) writer structure"]
impl crate::Writable for AesKey1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_KEY_1 to value 0"]
impl crate::Resettable for AesKey1Spec {
    const RESET_VALUE: u32 = 0;
}
