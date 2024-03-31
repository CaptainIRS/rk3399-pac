#[doc = "Register `DENALI_PHY_898` reader"]
pub type R = crate::R<DenaliPhy898Spec>;
#[doc = "Register `DENALI_PHY_898` writer"]
pub type W = crate::W<DenaliPhy898Spec>;
#[doc = "Field `PHY_SW_GRP_BYPASS_SHIFT` reader - Address/control group slice bypass mode shift settings."]
pub type PhySwGrpBypassShiftR = crate::FieldReader;
#[doc = "Field `PHY_SW_GRP_BYPASS_SHIFT` writer - Address/control group slice bypass mode shift settings."]
pub type PhySwGrpBypassShiftW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_GRP_BYPASS_OVERRIDE` reader - Address/control group slice bypass mode override setting."]
pub type PhyGrpBypassOverrideR = crate::BitReader;
#[doc = "Field `PHY_GRP_BYPASS_OVERRIDE` writer - Address/control group slice bypass mode override setting."]
pub type PhyGrpBypassOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_MANUAL_UPDATE` writer - Manual update of all slave delay line settings. Set to 1 to trigger."]
pub type ScPhyManualUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP4_BOOT_DISABLE` reader - Controls the handling of the DFI frequency. When set to 1, DFI frequency 0 is considered the first operational frequency. When cleared to 0, DFI frequency 0 is the boot frequency and other DFI frequency values are operational frequencies. Must be cleared to 0 for LPDDR3 devices operating in an LPDDR4 capable configuration."]
pub type PhyLp4BootDisableR = crate::BitReader;
#[doc = "Field `PHY_LP4_BOOT_DISABLE` writer - Controls the handling of the DFI frequency. When set to 1, DFI frequency 0 is considered the first operational frequency. When cleared to 0, DFI frequency 0 is the boot frequency and other DFI frequency values are operational frequencies. Must be cleared to 0 for LPDDR3 devices operating in an LPDDR4 capable configuration."]
pub type PhyLp4BootDisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Address/control group slice bypass mode shift settings."]
    #[inline(always)]
    pub fn phy_sw_grp_bypass_shift(&self) -> PhySwGrpBypassShiftR {
        PhySwGrpBypassShiftR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Address/control group slice bypass mode override setting."]
    #[inline(always)]
    pub fn phy_grp_bypass_override(&self) -> PhyGrpBypassOverrideR {
        PhyGrpBypassOverrideR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 24 - Controls the handling of the DFI frequency. When set to 1, DFI frequency 0 is considered the first operational frequency. When cleared to 0, DFI frequency 0 is the boot frequency and other DFI frequency values are operational frequencies. Must be cleared to 0 for LPDDR3 devices operating in an LPDDR4 capable configuration."]
    #[inline(always)]
    pub fn phy_lp4_boot_disable(&self) -> PhyLp4BootDisableR {
        PhyLp4BootDisableR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Address/control group slice bypass mode shift settings."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_grp_bypass_shift(&mut self) -> PhySwGrpBypassShiftW<DenaliPhy898Spec> {
        PhySwGrpBypassShiftW::new(self, 0)
    }
    #[doc = "Bit 8 - Address/control group slice bypass mode override setting."]
    #[inline(always)]
    #[must_use]
    pub fn phy_grp_bypass_override(&mut self) -> PhyGrpBypassOverrideW<DenaliPhy898Spec> {
        PhyGrpBypassOverrideW::new(self, 8)
    }
    #[doc = "Bit 16 - Manual update of all slave delay line settings. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_manual_update(&mut self) -> ScPhyManualUpdateW<DenaliPhy898Spec> {
        ScPhyManualUpdateW::new(self, 16)
    }
    #[doc = "Bit 24 - Controls the handling of the DFI frequency. When set to 1, DFI frequency 0 is considered the first operational frequency. When cleared to 0, DFI frequency 0 is the boot frequency and other DFI frequency values are operational frequencies. Must be cleared to 0 for LPDDR3 devices operating in an LPDDR4 capable configuration."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_disable(&mut self) -> PhyLp4BootDisableW<DenaliPhy898Spec> {
        PhyLp4BootDisableW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_898::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_898::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy898Spec;
impl crate::RegisterSpec for DenaliPhy898Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_898::R`](R) reader structure"]
impl crate::Readable for DenaliPhy898Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_898::W`](W) writer structure"]
impl crate::Writable for DenaliPhy898Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_898 to value 0"]
impl crate::Resettable for DenaliPhy898Spec {
    const RESET_VALUE: u32 = 0;
}
