#[doc = "Register `A2B_DAT_2` reader"]
pub type R = crate::R<A2bDat2Spec>;
#[doc = "Register `A2B_DAT_2` writer"]
pub type W = crate::W<A2bDat2Spec>;
#[doc = "Field `DATA` reader - data of Cortex-A53/Cortex-A72 to Cortex-M0"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - data of Cortex-A53/Cortex-A72 to Cortex-M0"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data of Cortex-A53/Cortex-A72 to Cortex-M0"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data of Cortex-A53/Cortex-A72 to Cortex-M0"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<A2bDat2Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 data 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a2b_dat_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a2b_dat_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A2bDat2Spec;
impl crate::RegisterSpec for A2bDat2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a2b_dat_2::R`](R) reader structure"]
impl crate::Readable for A2bDat2Spec {}
#[doc = "`write(|w| ..)` method takes [`a2b_dat_2::W`](W) writer structure"]
impl crate::Writable for A2bDat2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A2B_DAT_2 to value 0"]
impl crate::Resettable for A2bDat2Spec {
    const RESET_VALUE: u32 = 0;
}
