#[doc = "Register `n_LOAD_COUNT1` reader"]
pub type R = crate::R<NLoadCount1Spec>;
#[doc = "Register `n_LOAD_COUNT1` writer"]
pub type W = crate::W<NLoadCount1Spec>;
#[doc = "Field `LOAD_COUNT_HIGH_BITS` reader - High 32 bits value to be loaded into Timer n."]
pub type LoadCountHighBitsR = crate::FieldReader<u32>;
#[doc = "Field `LOAD_COUNT_HIGH_BITS` writer - High 32 bits value to be loaded into Timer n."]
pub type LoadCountHighBitsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - High 32 bits value to be loaded into Timer n."]
    #[inline(always)]
    pub fn load_count_high_bits(&self) -> LoadCountHighBitsR {
        LoadCountHighBitsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - High 32 bits value to be loaded into Timer n."]
    #[inline(always)]
    #[must_use]
    pub fn load_count_high_bits(&mut self) -> LoadCountHighBitsW<NLoadCount1Spec> {
        LoadCountHighBitsW::new(self, 0)
    }
}
#[doc = "Timer n higher Load Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_load_count1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`n_load_count1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NLoadCount1Spec;
impl crate::RegisterSpec for NLoadCount1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`n_load_count1::R`](R) reader structure"]
impl crate::Readable for NLoadCount1Spec {}
#[doc = "`write(|w| ..)` method takes [`n_load_count1::W`](W) writer structure"]
impl crate::Writable for NLoadCount1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets n_LOAD_COUNT1 to value 0"]
impl crate::Resettable for NLoadCount1Spec {
    const RESET_VALUE: u32 = 0;
}
