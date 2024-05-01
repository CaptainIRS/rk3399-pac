#[doc = "Register `PKA_E` reader"]
pub type R = crate::R<PkaESpec>;
#[doc = "Register `PKA_E` writer"]
pub type W = crate::W<PkaESpec>;
#[doc = "Field `E` reader - PKA exponent."]
pub type ER = crate::FieldReader<u32>;
#[doc = "Field `E` writer - PKA exponent."]
pub type EW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PKA exponent."]
    #[inline(always)]
    pub fn e(&self) -> ER {
        ER::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PKA exponent."]
    #[inline(always)]
    #[must_use]
    pub fn e(&mut self) -> EW<PkaESpec> {
        EW::new(self, 0)
    }
}
#[doc = "PKA exponent\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pka_e::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pka_e::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaESpec;
impl crate::RegisterSpec for PkaESpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pka_e::R`](R) reader structure"]
impl crate::Readable for PkaESpec {}
#[doc = "`write(|w| ..)` method takes [`pka_e::W`](W) writer structure"]
impl crate::Writable for PkaESpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKA_E to value 0"]
impl crate::Resettable for PkaESpec {
    const RESET_VALUE: u32 = 0;
}
