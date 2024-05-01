#[doc = "Register `HASH_SEED_2` reader"]
pub type R = crate::R<HashSeed2Spec>;
#[doc = "Register `HASH_SEED_2` writer"]
pub type W = crate::W<HashSeed2Spec>;
#[doc = "Field `HASH_SEED_2` reader - Specifies PRNG Seed/HMAC Key buffer \\[95:64\\]"]
pub type HashSeed2R = crate::FieldReader<u32>;
#[doc = "Field `HASH_SEED_2` writer - Specifies PRNG Seed/HMAC Key buffer \\[95:64\\]"]
pub type HashSeed2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies PRNG Seed/HMAC Key buffer \\[95:64\\]"]
    #[inline(always)]
    pub fn hash_seed_2(&self) -> HashSeed2R {
        HashSeed2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies PRNG Seed/HMAC Key buffer \\[95:64\\]"]
    #[inline(always)]
    #[must_use]
    pub fn hash_seed_2(&mut self) -> HashSeed2W<HashSeed2Spec> {
        HashSeed2W::new(self, 0)
    }
}
#[doc = "PRNG Seed/HMAC Key Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_seed_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_seed_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashSeed2Spec;
impl crate::RegisterSpec for HashSeed2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_seed_2::R`](R) reader structure"]
impl crate::Readable for HashSeed2Spec {}
#[doc = "`write(|w| ..)` method takes [`hash_seed_2::W`](W) writer structure"]
impl crate::Writable for HashSeed2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_SEED_2 to value 0"]
impl crate::Resettable for HashSeed2Spec {
    const RESET_VALUE: u32 = 0;
}
