#[doc = "Register `HASH_SEED_0` reader"]
pub type R = crate::R<HashSeed0Spec>;
#[doc = "Register `HASH_SEED_0` writer"]
pub type W = crate::W<HashSeed0Spec>;
#[doc = "Field `HASH_SEED_0` reader - Specifies PRNG Seed/HMAC Key buffer \\[159:128\\]"]
pub type HashSeed0R = crate::FieldReader<u32>;
#[doc = "Field `HASH_SEED_0` writer - Specifies PRNG Seed/HMAC Key buffer \\[159:128\\]"]
pub type HashSeed0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies PRNG Seed/HMAC Key buffer \\[159:128\\]"]
    #[inline(always)]
    pub fn hash_seed_0(&self) -> HashSeed0R {
        HashSeed0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies PRNG Seed/HMAC Key buffer \\[159:128\\]"]
    #[inline(always)]
    #[must_use]
    pub fn hash_seed_0(&mut self) -> HashSeed0W<HashSeed0Spec> {
        HashSeed0W::new(self, 0)
    }
}
#[doc = "PRNG Seed/HMAC Key Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_seed_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_seed_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashSeed0Spec;
impl crate::RegisterSpec for HashSeed0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_seed_0::R`](R) reader structure"]
impl crate::Readable for HashSeed0Spec {}
#[doc = "`write(|w| ..)` method takes [`hash_seed_0::W`](W) writer structure"]
impl crate::Writable for HashSeed0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_SEED_0 to value 0"]
impl crate::Resettable for HashSeed0Spec {
    const RESET_VALUE: u32 = 0;
}
