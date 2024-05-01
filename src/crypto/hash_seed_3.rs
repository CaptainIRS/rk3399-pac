#[doc = "Register `HASH_SEED_3` reader"]
pub type R = crate::R<HashSeed3Spec>;
#[doc = "Register `HASH_SEED_3` writer"]
pub type W = crate::W<HashSeed3Spec>;
#[doc = "Field `HASH_SEED_3` reader - Specifies PRNG Seed/HMAC Key buffer \\[63:32\\]"]
pub type HashSeed3R = crate::FieldReader<u32>;
#[doc = "Field `HASH_SEED_3` writer - Specifies PRNG Seed/HMAC Key buffer \\[63:32\\]"]
pub type HashSeed3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies PRNG Seed/HMAC Key buffer \\[63:32\\]"]
    #[inline(always)]
    pub fn hash_seed_3(&self) -> HashSeed3R {
        HashSeed3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies PRNG Seed/HMAC Key buffer \\[63:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn hash_seed_3(&mut self) -> HashSeed3W<HashSeed3Spec> {
        HashSeed3W::new(self, 0)
    }
}
#[doc = "PRNG Seed/HMAC Key Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_seed_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_seed_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashSeed3Spec;
impl crate::RegisterSpec for HashSeed3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_seed_3::R`](R) reader structure"]
impl crate::Readable for HashSeed3Spec {}
#[doc = "`write(|w| ..)` method takes [`hash_seed_3::W`](W) writer structure"]
impl crate::Writable for HashSeed3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_SEED_3 to value 0"]
impl crate::Resettable for HashSeed3Spec {
    const RESET_VALUE: u32 = 0;
}
