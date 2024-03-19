#[doc = "Register `DDR_DENALI_PHY_923` reader"]
pub type R = crate::R<DdrDenaliPhy923Spec>;
#[doc = "Register `DDR_DENALI_PHY_923` writer"]
pub type W = crate::W<DdrDenaliPhy923Spec>;
#[doc = "Field `PHY_LP4_BOOT_LOW_FREQ_SEL` reader - Control the PLL domain enter/exit from the negative clock edge for LPDDR4 boot frequency."]
pub type PhyLp4BootLowFreqSelR = crate::BitReader;
#[doc = "Field `PHY_LP4_BOOT_LOW_FREQ_SEL` writer - Control the PLL domain enter/exit from the negative clock edge for LPDDR4 boot frequency."]
pub type PhyLp4BootLowFreqSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP_WAKEUP` reader - Specifies the number of cycles the PHY takes to wakeup in low power mode."]
pub type PhyLpWakeupR = crate::FieldReader;
#[doc = "Field `PHY_LP_WAKEUP` writer - Specifies the number of cycles the PHY takes to wakeup in low power mode."]
pub type PhyLpWakeupW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_LS_IDLE_EN` reader - Indicates the Reduced Idle Power State is enabled in low power mode."]
pub type PhyLsIdleEnR = crate::BitReader;
#[doc = "Field `PHY_LS_IDLE_EN` writer - Indicates the Reduced Idle Power State is enabled in low power mode."]
pub type PhyLsIdleEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_TDFI_PHY_WRDELAY` reader - DFI timing parameter TDFI_PHY_WRDELAY."]
pub type PhyTdfiPhyWrdelayR = crate::BitReader;
#[doc = "Field `PHY_TDFI_PHY_WRDELAY` writer - DFI timing parameter TDFI_PHY_WRDELAY."]
pub type PhyTdfiPhyWrdelayW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Control the PLL domain enter/exit from the negative clock edge for LPDDR4 boot frequency."]
    #[inline(always)]
    pub fn phy_lp4_boot_low_freq_sel(&self) -> PhyLp4BootLowFreqSelR {
        PhyLp4BootLowFreqSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Specifies the number of cycles the PHY takes to wakeup in low power mode."]
    #[inline(always)]
    pub fn phy_lp_wakeup(&self) -> PhyLpWakeupR {
        PhyLpWakeupR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Indicates the Reduced Idle Power State is enabled in low power mode."]
    #[inline(always)]
    pub fn phy_ls_idle_en(&self) -> PhyLsIdleEnR {
        PhyLsIdleEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - DFI timing parameter TDFI_PHY_WRDELAY."]
    #[inline(always)]
    pub fn phy_tdfi_phy_wrdelay(&self) -> PhyTdfiPhyWrdelayR {
        PhyTdfiPhyWrdelayR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control the PLL domain enter/exit from the negative clock edge for LPDDR4 boot frequency."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_low_freq_sel(&mut self) -> PhyLp4BootLowFreqSelW<DdrDenaliPhy923Spec> {
        PhyLp4BootLowFreqSelW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Specifies the number of cycles the PHY takes to wakeup in low power mode."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp_wakeup(&mut self) -> PhyLpWakeupW<DdrDenaliPhy923Spec> {
        PhyLpWakeupW::new(self, 8)
    }
    #[doc = "Bit 16 - Indicates the Reduced Idle Power State is enabled in low power mode."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ls_idle_en(&mut self) -> PhyLsIdleEnW<DdrDenaliPhy923Spec> {
        PhyLsIdleEnW::new(self, 16)
    }
    #[doc = "Bit 24 - DFI timing parameter TDFI_PHY_WRDELAY."]
    #[inline(always)]
    #[must_use]
    pub fn phy_tdfi_phy_wrdelay(&mut self) -> PhyTdfiPhyWrdelayW<DdrDenaliPhy923Spec> {
        PhyTdfiPhyWrdelayW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_923::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_923::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy923Spec;
impl crate::RegisterSpec for DdrDenaliPhy923Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_923::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy923Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_923::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy923Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_923 to value 0"]
impl crate::Resettable for DdrDenaliPhy923Spec {
    const RESET_VALUE: u32 = 0;
}
