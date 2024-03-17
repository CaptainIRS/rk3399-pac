#[doc = "Register `DENALI_CTL_276` reader"]
pub type R = crate::R<DenaliCtl276Spec>;
#[doc = "Register `DENALI_CTL_276` writer"]
pub type W = crate::W<DenaliCtl276Spec>;
#[doc = "Field `TDFI_PHY_RDLAT_F2` reader - Defines the DFI tPHY_RDLAT timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion."]
pub type TdfiPhyRdlatF2R = crate::FieldReader;
#[doc = "Field `TDFI_PHY_RDLAT_F2` writer - Defines the DFI tPHY_RDLAT timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion."]
pub type TdfiPhyRdlatF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDFI_RDDATA_EN` reader - Holds the calculated DFI tRDDATA_EN timing parameter (in DFI PHY clocks), the maximum cycles between a read command and a dfi_rddata_en assertion. READ-ONLY"]
pub type TdfiRddataEnR = crate::FieldReader;
#[doc = "Field `DRAM_CLK_DISABLE` reader - Set value for the dfi_dram_clk_disable signal. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to disable."]
pub type DramClkDisableR = crate::FieldReader;
#[doc = "Field `DRAM_CLK_DISABLE` writer - Set value for the dfi_dram_clk_disable signal. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to disable."]
pub type DramClkDisableW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TDFI_CTRLUPD_MIN` reader - Reports the DFI tCTRLUPD_MIN timing parameter (in DFI clocks), the minimum cycles that dfi_ctrlupd_req must be asserted."]
pub type TdfiCtrlupdMinR = crate::FieldReader;
#[doc = "Field `TDFI_CTRLUPD_MIN` writer - Reports the DFI tCTRLUPD_MIN timing parameter (in DFI clocks), the minimum cycles that dfi_ctrlupd_req must be asserted."]
pub type TdfiCtrlupdMinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Defines the DFI tPHY_RDLAT timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion."]
    #[inline(always)]
    pub fn tdfi_phy_rdlat_f2(&self) -> TdfiPhyRdlatF2R {
        TdfiPhyRdlatF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Holds the calculated DFI tRDDATA_EN timing parameter (in DFI PHY clocks), the maximum cycles between a read command and a dfi_rddata_en assertion. READ-ONLY"]
    #[inline(always)]
    pub fn tdfi_rddata_en(&self) -> TdfiRddataEnR {
        TdfiRddataEnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Set value for the dfi_dram_clk_disable signal. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to disable."]
    #[inline(always)]
    pub fn dram_clk_disable(&self) -> DramClkDisableR {
        DramClkDisableR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Reports the DFI tCTRLUPD_MIN timing parameter (in DFI clocks), the minimum cycles that dfi_ctrlupd_req must be asserted."]
    #[inline(always)]
    pub fn tdfi_ctrlupd_min(&self) -> TdfiCtrlupdMinR {
        TdfiCtrlupdMinR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Defines the DFI tPHY_RDLAT timing parameter (in DFI PHY clocks), the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phy_rdlat_f2(&mut self) -> TdfiPhyRdlatF2W<DenaliCtl276Spec> {
        TdfiPhyRdlatF2W::new(self, 0)
    }
    #[doc = "Bits 16:17 - Set value for the dfi_dram_clk_disable signal. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn dram_clk_disable(&mut self) -> DramClkDisableW<DenaliCtl276Spec> {
        DramClkDisableW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Reports the DFI tCTRLUPD_MIN timing parameter (in DFI clocks), the minimum cycles that dfi_ctrlupd_req must be asserted."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_ctrlupd_min(&mut self) -> TdfiCtrlupdMinW<DenaliCtl276Spec> {
        TdfiCtrlupdMinW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_276::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_276::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl276Spec;
impl crate::RegisterSpec for DenaliCtl276Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_276::R`](R) reader structure"]
impl crate::Readable for DenaliCtl276Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_276::W`](W) writer structure"]
impl crate::Writable for DenaliCtl276Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_276 to value 0x06"]
impl crate::Resettable for DenaliCtl276Spec {
    const RESET_VALUE: u32 = 0x06;
}
