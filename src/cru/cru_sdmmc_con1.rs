#[doc = "Register `CRU_SDMMC_CON1` writer"]
pub type W = crate::W<CruSdmmcCon1Spec>;
#[doc = "Field `SDMMC_CON1` writer - sdmmc con1 register\n\nrefer to chapter SDMMC"]
pub type SdmmcCon1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - sdmmc con1 register\n\nrefer to chapter SDMMC"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc_con1(&mut self) -> SdmmcCon1W<CruSdmmcCon1Spec> {
        SdmmcCon1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruSdmmcCon1Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "sdmmc control1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_sdmmc_con1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruSdmmcCon1Spec;
impl crate::RegisterSpec for CruSdmmcCon1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cru_sdmmc_con1::W`](W) writer structure"]
impl crate::Writable for CruSdmmcCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_SDMMC_CON1 to value 0"]
impl crate::Resettable for CruSdmmcCon1Spec {
    const RESET_VALUE: u32 = 0;
}
