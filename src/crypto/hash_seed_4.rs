#[doc = "Register `HASH_SEED_4` reader"]
pub type R = crate::R<HashSeed4Spec>;
#[doc = "Register `HASH_SEED_4` writer"]
pub type W = crate::W<HashSeed4Spec>;
#[doc = "Field `HASH_SEED_4` reader - Specifies PRNG Seed/HMAC Key buffer \\[31:0\\]"]
pub type HashSeed4R = crate::FieldReader<u32>;
#[doc = "Field `HASH_SEED_4` writer - Specifies PRNG Seed/HMAC Key buffer \\[31:0\\]"]
pub type HashSeed4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies PRNG Seed/HMAC Key buffer \\[31:0\\]"]
    #[inline(always)]
    pub fn hash_seed_4(&self) -> HashSeed4R {
        HashSeed4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies PRNG Seed/HMAC Key buffer \\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn hash_seed_4(&mut self) -> HashSeed4W<HashSeed4Spec> {
        HashSeed4W::new(self, 0)
    }
}
#[doc = "PRNG Seed/HMAC Key Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_seed_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_seed_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashSeed4Spec;
impl crate::RegisterSpec for HashSeed4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_seed_4::R`](R) reader structure"]
impl crate::Readable for HashSeed4Spec {}
#[doc = "`write(|w| ..)` method takes [`hash_seed_4::W`](W) writer structure"]
impl crate::Writable for HashSeed4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_SEED_4 to value 0"]
impl crate::Resettable for HashSeed4Spec {
    const RESET_VALUE: u32 = 0;
}
