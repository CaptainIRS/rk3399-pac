#[doc = "Register `PMUGRF_SOC_CON10` reader"]
pub type R = crate::R<PmugrfSocCon10Spec>;
#[doc = "Register `PMUGRF_SOC_CON10` writer"]
pub type W = crate::W<PmugrfSocCon10Spec>;
#[doc = "Field `SDMMC_DETTIME0` reader - sdmmc_dettime\\[15:0\\]"]
pub type SdmmcDettime0R = crate::FieldReader<u16>;
#[doc = "Field `SDMMC_DETTIME0` writer - sdmmc_dettime\\[15:0\\]"]
pub type SdmmcDettime0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sdmmc_dettime\\[15:0\\]"]
    #[inline(always)]
    pub fn sdmmc_dettime0(&self) -> SdmmcDettime0R {
        SdmmcDettime0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sdmmc_dettime\\[15:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc_dettime0(&mut self) -> SdmmcDettime0W<PmugrfSocCon10Spec> {
        SdmmcDettime0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<PmugrfSocCon10Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_soc_con10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_soc_con10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfSocCon10Spec;
impl crate::RegisterSpec for PmugrfSocCon10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_soc_con10::R`](R) reader structure"]
impl crate::Readable for PmugrfSocCon10Spec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_soc_con10::W`](W) writer structure"]
impl crate::Writable for PmugrfSocCon10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_SOC_CON10 to value 0x61a8"]
impl crate::Resettable for PmugrfSocCon10Spec {
    const RESET_VALUE: u32 = 0x61a8;
}
