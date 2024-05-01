#[doc = "Register `PKA_N` reader"]
pub type R = crate::R<PkaNSpec>;
#[doc = "Register `PKA_N` writer"]
pub type W = crate::W<PkaNSpec>;
#[doc = "Field `N` reader - PKA modular"]
pub type NR = crate::FieldReader<u32>;
#[doc = "Field `N` writer - PKA modular"]
pub type NW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PKA modular"]
    #[inline(always)]
    pub fn n(&self) -> NR {
        NR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PKA modular"]
    #[inline(always)]
    #[must_use]
    pub fn n(&mut self) -> NW<PkaNSpec> {
        NW::new(self, 0)
    }
}
#[doc = "PKA modular\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pka_n::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pka_n::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkaNSpec;
impl crate::RegisterSpec for PkaNSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pka_n::R`](R) reader structure"]
impl crate::Readable for PkaNSpec {}
#[doc = "`write(|w| ..)` method takes [`pka_n::W`](W) writer structure"]
impl crate::Writable for PkaNSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKA_N to value 0"]
impl crate::Resettable for PkaNSpec {
    const RESET_VALUE: u32 = 0;
}
