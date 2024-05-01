#[doc = "Register `KEY_SECURE` reader"]
pub type R = crate::R<KeySecureSpec>;
#[doc = "Register `KEY_SECURE` writer"]
pub type W = crate::W<KeySecureSpec>;
#[doc = "Field `KEY_SECURE` reader - 0 = AES key ,tkey and DES key are non-secure\n\n1 = AES key ,tkey and DES key are secure"]
pub type KeySecureR = crate::BitReader;
#[doc = "Field `KEY_SECURE` writer - 0 = AES key ,tkey and DES key are non-secure\n\n1 = AES key ,tkey and DES key are secure"]
pub type KeySecureW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0 = AES key ,tkey and DES key are non-secure\n\n1 = AES key ,tkey and DES key are secure"]
    #[inline(always)]
    pub fn key_secure(&self) -> KeySecureR {
        KeySecureR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0 = AES key ,tkey and DES key are non-secure\n\n1 = AES key ,tkey and DES key are secure"]
    #[inline(always)]
    #[must_use]
    pub fn key_secure(&mut self) -> KeySecureW<KeySecureSpec> {
        KeySecureW::new(self, 0)
    }
}
#[doc = "Key Secure Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_secure::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_secure::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeySecureSpec;
impl crate::RegisterSpec for KeySecureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_secure::R`](R) reader structure"]
impl crate::Readable for KeySecureSpec {}
#[doc = "`write(|w| ..)` method takes [`key_secure::W`](W) writer structure"]
impl crate::Writable for KeySecureSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY_SECURE to value 0"]
impl crate::Resettable for KeySecureSpec {
    const RESET_VALUE: u32 = 0;
}
