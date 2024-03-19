#[doc = "Register `DDR_DENALI_PHY_907` reader"]
pub type R = crate::R<DdrDenaliPhy907Spec>;
#[doc = "Register `DDR_DENALI_PHY_907` writer"]
pub type W = crate::W<DdrDenaliPhy907Spec>;
#[doc = "Field `PHY_ADRCTL_SNAP_OBS_REGS` writer - Initiates a snapshot of the internal observation registers for the address/control block. Set to 1 to trigger."]
pub type PhyAdrctlSnapObsRegsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_DFI_PHYUPD_TYPE` reader - Defines the value of the dfi_phyupd_type output signal to MC."]
pub type PhyDfiPhyupdTypeR = crate::FieldReader;
#[doc = "Field `PHY_DFI_PHYUPD_TYPE` writer - Defines the value of the dfi_phyupd_type output signal to MC."]
pub type PhyDfiPhyupdTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_ADRCTL_LPDDR` reader - Adds a cycle of delay for the address/control slices to match the address slice."]
pub type PhyAdrctlLpddrR = crate::BitReader;
#[doc = "Field `PHY_ADRCTL_LPDDR` writer - Adds a cycle of delay for the address/control slices to match the address slice."]
pub type PhyAdrctlLpddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP4_ACTIVE` reader - Indicates LPDDR4 device is connected to the PHY."]
pub type PhyLp4ActiveR = crate::BitReader;
#[doc = "Field `PHY_LP4_ACTIVE` writer - Indicates LPDDR4 device is connected to the PHY."]
pub type PhyLp4ActiveW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:9 - Defines the value of the dfi_phyupd_type output signal to MC."]
    #[inline(always)]
    pub fn phy_dfi_phyupd_type(&self) -> PhyDfiPhyupdTypeR {
        PhyDfiPhyupdTypeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Adds a cycle of delay for the address/control slices to match the address slice."]
    #[inline(always)]
    pub fn phy_adrctl_lpddr(&self) -> PhyAdrctlLpddrR {
        PhyAdrctlLpddrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Indicates LPDDR4 device is connected to the PHY."]
    #[inline(always)]
    pub fn phy_lp4_active(&self) -> PhyLp4ActiveR {
        PhyLp4ActiveR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initiates a snapshot of the internal observation registers for the address/control block. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_snap_obs_regs(&mut self) -> PhyAdrctlSnapObsRegsW<DdrDenaliPhy907Spec> {
        PhyAdrctlSnapObsRegsW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Defines the value of the dfi_phyupd_type output signal to MC."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dfi_phyupd_type(&mut self) -> PhyDfiPhyupdTypeW<DdrDenaliPhy907Spec> {
        PhyDfiPhyupdTypeW::new(self, 8)
    }
    #[doc = "Bit 16 - Adds a cycle of delay for the address/control slices to match the address slice."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_lpddr(&mut self) -> PhyAdrctlLpddrW<DdrDenaliPhy907Spec> {
        PhyAdrctlLpddrW::new(self, 16)
    }
    #[doc = "Bit 24 - Indicates LPDDR4 device is connected to the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_active(&mut self) -> PhyLp4ActiveW<DdrDenaliPhy907Spec> {
        PhyLp4ActiveW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_907::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_907::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy907Spec;
impl crate::RegisterSpec for DdrDenaliPhy907Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_907::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy907Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_907::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy907Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_907 to value 0"]
impl crate::Resettable for DdrDenaliPhy907Spec {
    const RESET_VALUE: u32 = 0;
}
