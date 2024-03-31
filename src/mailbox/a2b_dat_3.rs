#[doc = "Register `A2B_DAT_3` reader"]
pub type R = crate::R<A2bDat3Spec>;
#[doc = "Register `A2B_DAT_3` writer"]
pub type W = crate::W<A2bDat3Spec>;
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
    pub fn data(&mut self) -> DataW<A2bDat3Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Cortex-A53/Cortex-A72 to Cortex-M0 data 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a2b_dat_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a2b_dat_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A2bDat3Spec;
impl crate::RegisterSpec for A2bDat3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a2b_dat_3::R`](R) reader structure"]
impl crate::Readable for A2bDat3Spec {}
#[doc = "`write(|w| ..)` method takes [`a2b_dat_3::W`](W) writer structure"]
impl crate::Writable for A2bDat3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A2B_DAT_3 to value 0"]
impl crate::Resettable for A2bDat3Spec {
    const RESET_VALUE: u32 = 0;
}
