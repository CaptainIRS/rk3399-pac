#[doc = "Register `CRYPTO_VER` reader"]
pub type R = crate::R<CryptoVerSpec>;
#[doc = "Register `CRYPTO_VER` writer"]
pub type W = crate::W<CryptoVerSpec>;
#[doc = "Field `CRYPTO_VER` reader - Crypto version"]
pub type CryptoVerR = crate::FieldReader<u32>;
#[doc = "Field `CRYPTO_VER` writer - Crypto version"]
pub type CryptoVerW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Crypto version"]
    #[inline(always)]
    pub fn crypto_ver(&self) -> CryptoVerR {
        CryptoVerR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Crypto version"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_ver(&mut self) -> CryptoVerW<CryptoVerSpec> {
        CryptoVerW::new(self, 0)
    }
}
#[doc = "Crypto Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crypto_ver::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_ver::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoVerSpec;
impl crate::RegisterSpec for CryptoVerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_ver::R`](R) reader structure"]
impl crate::Readable for CryptoVerSpec {}
#[doc = "`write(|w| ..)` method takes [`crypto_ver::W`](W) writer structure"]
impl crate::Writable for CryptoVerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYPTO_VER to value 0x0300"]
impl crate::Resettable for CryptoVerSpec {
    const RESET_VALUE: u32 = 0x0300;
}
