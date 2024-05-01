#[doc = "Register `PKA_C` reader"]
pub type R = crate::R<PkaCSpec>;
#[doc = "Register `PKA_C` writer"]
pub type W = crate::W<PkaCSpec>;
#[doc = "Field `C` reader - PKA pre-calculate data, C = 2 ^(2n+2) mod N"]
pub type CR = crate::BitReader;
#[doc = "Field `C` writer - PKA pre-calculate data, C = 2 ^(2n+2) mod N"]
pub type CW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PKA pre-calculate data, C = 2 ^(2n+2) mod N"]
    #[inline(always)]
    pub fn c(&self) -> CR {
        CR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PKA pre-calculate data, C = 2 ^(2n+2) mod N"]
    #[inline(always)]
    #[must_use]
    pub fn c(&mut self) -> CW<PkaCSpec> {
        CW::new(self, 0)
    }
}
#[doc = "PKA C value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pka_c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pka_c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaCSpec;
impl crate::RegisterSpec for PkaCSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pka_c::R`](R) reader structure"]
impl crate::Readable for PkaCSpec {}
#[doc = "`write(|w| ..)` method takes [`pka_c::W`](W) writer structure"]
impl crate::Writable for PkaCSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKA_C to value 0"]
impl crate::Resettable for PkaCSpec {
    const RESET_VALUE: u32 = 0;
}
