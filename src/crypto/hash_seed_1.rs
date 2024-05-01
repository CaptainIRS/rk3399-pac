#[doc = "Register `HASH_SEED_1` reader"]
pub type R = crate::R<HashSeed1Spec>;
#[doc = "Register `HASH_SEED_1` writer"]
pub type W = crate::W<HashSeed1Spec>;
#[doc = "Field `HASH_SEED_1` reader - Specifies PRNG Seed/HMAC Key buffer \\[127:96\\]"]
pub type HashSeed1R = crate::FieldReader<u32>;
#[doc = "Field `HASH_SEED_1` writer - Specifies PRNG Seed/HMAC Key buffer \\[127:96\\]"]
pub type HashSeed1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies PRNG Seed/HMAC Key buffer \\[127:96\\]"]
    #[inline(always)]
    pub fn hash_seed_1(&self) -> HashSeed1R {
        HashSeed1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies PRNG Seed/HMAC Key buffer \\[127:96\\]"]
    #[inline(always)]
    #[must_use]
    pub fn hash_seed_1(&mut self) -> HashSeed1W<HashSeed1Spec> {
        HashSeed1W::new(self, 0)
    }
}
#[doc = "PRNG Seed/HMAC Key Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_seed_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_seed_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashSeed1Spec;
impl crate::RegisterSpec for HashSeed1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_seed_1::R`](R) reader structure"]
impl crate::Readable for HashSeed1Spec {}
#[doc = "`write(|w| ..)` method takes [`hash_seed_1::W`](W) writer structure"]
impl crate::Writable for HashSeed1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_SEED_1 to value 0"]
impl crate::Resettable for HashSeed1Spec {
    const RESET_VALUE: u32 = 0;
}
