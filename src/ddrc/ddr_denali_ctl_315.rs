#[doc = "Register `DDR_DENALI_CTL_315` reader"]
pub type R = crate::R<DdrDenaliCtl315Spec>;
#[doc = "Register `DDR_DENALI_CTL_315` writer"]
pub type W = crate::W<DdrDenaliCtl315Spec>;
#[doc = "Field `TDFI_RDCSLAT_F2` reader - Defines the DFI tPHY_RDCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a read command and a dfi_rddata_cs_n assertion."]
pub type TdfiRdcslatF2R = crate::FieldReader;
#[doc = "Field `TDFI_RDCSLAT_F2` writer - Defines the DFI tPHY_RDCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a read command and a dfi_rddata_cs_n assertion."]
pub type TdfiRdcslatF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDFI_WRCSLAT_F2` reader - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a write command and a dfi_wrdata_cs_n assertion."]
pub type TdfiWrcslatF2R = crate::FieldReader;
#[doc = "Field `TDFI_WRCSLAT_F2` writer - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a write command and a dfi_wrdata_cs_n assertion."]
pub type TdfiWrcslatF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDFI_WRDATA_DELAY` reader - Defines the tWRDATA_DELAY timing parameter (in DFI PHY clocks), the maximum cycles between when the dfi_wrdata_en signal is asserted and when the corresponding write data transfer is completed on the DRAM bus."]
pub type TdfiWrdataDelayR = crate::FieldReader;
#[doc = "Field `TDFI_WRDATA_DELAY` writer - Defines the tWRDATA_DELAY timing parameter (in DFI PHY clocks), the maximum cycles between when the dfi_wrdata_en signal is asserted and when the corresponding write data transfer is completed on the DRAM bus."]
pub type TdfiWrdataDelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Defines the DFI tPHY_RDCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a read command and a dfi_rddata_cs_n assertion."]
    #[inline(always)]
    pub fn tdfi_rdcslat_f2(&self) -> TdfiRdcslatF2R {
        TdfiRdcslatF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a write command and a dfi_wrdata_cs_n assertion."]
    #[inline(always)]
    pub fn tdfi_wrcslat_f2(&self) -> TdfiWrcslatF2R {
        TdfiWrcslatF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Defines the tWRDATA_DELAY timing parameter (in DFI PHY clocks), the maximum cycles between when the dfi_wrdata_en signal is asserted and when the corresponding write data transfer is completed on the DRAM bus."]
    #[inline(always)]
    pub fn tdfi_wrdata_delay(&self) -> TdfiWrdataDelayR {
        TdfiWrdataDelayR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Defines the DFI tPHY_RDCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a read command and a dfi_rddata_cs_n assertion."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_rdcslat_f2(&mut self) -> TdfiRdcslatF2W<DdrDenaliCtl315Spec> {
        TdfiRdcslatF2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Defines the DFI tPHY_WRCSLAT timing parameter (in DFI PHY clocks), the maximum cycles between a write command and a dfi_wrdata_cs_n assertion."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_wrcslat_f2(&mut self) -> TdfiWrcslatF2W<DdrDenaliCtl315Spec> {
        TdfiWrcslatF2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Defines the tWRDATA_DELAY timing parameter (in DFI PHY clocks), the maximum cycles between when the dfi_wrdata_en signal is asserted and when the corresponding write data transfer is completed on the DRAM bus."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_wrdata_delay(&mut self) -> TdfiWrdataDelayW<DdrDenaliCtl315Spec> {
        TdfiWrdataDelayW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_315::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_315::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl315Spec;
impl crate::RegisterSpec for DdrDenaliCtl315Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_315::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl315Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_315::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl315Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_315 to value 0"]
impl crate::Resettable for DdrDenaliCtl315Spec {
    const RESET_VALUE: u32 = 0;
}
