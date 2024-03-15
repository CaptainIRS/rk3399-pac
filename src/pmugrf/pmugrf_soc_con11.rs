#[doc = "Register `PMUGRF_SOC_CON11` reader"]
pub type R = crate::R<PmugrfSocCon11Spec>;
#[doc = "Register `PMUGRF_SOC_CON11` writer"]
pub type W = crate::W<PmugrfSocCon11Spec>;
#[doc = "Field `SDMMC_DETTIME1` reader - sdmmc_dettime\\[19:16\\]"]
pub type SdmmcDettime1R = crate::FieldReader;
#[doc = "Field `SDMMC_DETTIME1` writer - sdmmc_dettime\\[19:16\\]"]
pub type SdmmcDettime1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - sdmmc_dettime\\[19:16\\]"]
    #[inline(always)]
    pub fn sdmmc_dettime1(&self) -> SdmmcDettime1R {
        SdmmcDettime1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - sdmmc_dettime\\[19:16\\]"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc_dettime1(&mut self) -> SdmmcDettime1W<PmugrfSocCon11Spec> {
        SdmmcDettime1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<PmugrfSocCon11Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_soc_con11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_soc_con11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfSocCon11Spec;
impl crate::RegisterSpec for PmugrfSocCon11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_soc_con11::R`](R) reader structure"]
impl crate::Readable for PmugrfSocCon11Spec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_soc_con11::W`](W) writer structure"]
impl crate::Writable for PmugrfSocCon11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_SOC_CON11 to value 0"]
impl crate::Resettable for PmugrfSocCon11Spec {
    const RESET_VALUE: u32 = 0;
}
