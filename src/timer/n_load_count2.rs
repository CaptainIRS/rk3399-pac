#[doc = "Register `n_LOAD_COUNT2` reader"]
pub type R = crate::R<NLoadCount2Spec>;
#[doc = "Register `n_LOAD_COUNT2` writer"]
pub type W = crate::W<NLoadCount2Spec>;
#[doc = "Field `LOAD_COUNT_LOW_BITS` reader - Low 32 bits value to be loaded into Timer n."]
pub type LoadCountLowBitsR = crate::FieldReader<u32>;
#[doc = "Field `LOAD_COUNT_LOW_BITS` writer - Low 32 bits value to be loaded into Timer n."]
pub type LoadCountLowBitsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Low 32 bits value to be loaded into Timer n."]
    #[inline(always)]
    pub fn load_count_low_bits(&self) -> LoadCountLowBitsR {
        LoadCountLowBitsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Low 32 bits value to be loaded into Timer n."]
    #[inline(always)]
    #[must_use]
    pub fn load_count_low_bits(&mut self) -> LoadCountLowBitsW<NLoadCount2Spec> {
        LoadCountLowBitsW::new(self, 0)
    }
}
#[doc = "Timer n lower Load Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_load_count2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`n_load_count2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NLoadCount2Spec;
impl crate::RegisterSpec for NLoadCount2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`n_load_count2::R`](R) reader structure"]
impl crate::Readable for NLoadCount2Spec {}
#[doc = "`write(|w| ..)` method takes [`n_load_count2::W`](W) writer structure"]
impl crate::Writable for NLoadCount2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets n_LOAD_COUNT2 to value 0"]
impl crate::Resettable for NLoadCount2Spec {
    const RESET_VALUE: u32 = 0;
}
