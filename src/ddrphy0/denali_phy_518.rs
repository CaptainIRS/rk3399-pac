#[doc = "Register `DENALI_PHY_518` reader"]
pub type R = crate::R<DenaliPhy518Spec>;
#[doc = "Register `DENALI_PHY_518` writer"]
pub type W = crate::W<DenaliPhy518Spec>;
#[doc = "Field `SC_PHY_ADR_SNAP_OBS_REGS_0` writer - Initiates a snapshot of the internal observation registers for address slice 0. Set to 1 to trigger. WRITE- ONLY"]
pub type ScPhyAdrSnapObsRegs0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_ADR_TSEL_ENABLE_0` reader - Enables tsel_en for address slice 0."]
pub type PhyAdrTselEnable0R = crate::BitReader;
#[doc = "Field `PHY_ADR_TSEL_ENABLE_0` writer - Enables tsel_en for address slice 0."]
pub type PhyAdrTselEnable0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_ADR_LPBK_CONTROL_0` reader - Loopback control bits for address slice 0."]
pub type PhyAdrLpbkControl0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_LPBK_CONTROL_0` writer - Loopback control bits for address slice 0."]
pub type PhyAdrLpbkControl0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_ADR_PWR_RDC_DISABLE_0` reader - adr slice power reduction disable for slice 0."]
pub type PhyAdrPwrRdcDisable0R = crate::BitReader;
#[doc = "Field `PHY_ADR_PWR_RDC_DISABLE_0` writer - adr slice power reduction disable for slice 0."]
pub type PhyAdrPwrRdcDisable0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - Enables tsel_en for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_tsel_enable_0(&self) -> PhyAdrTselEnable0R {
        PhyAdrTselEnable0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Loopback control bits for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_lpbk_control_0(&self) -> PhyAdrLpbkControl0R {
        PhyAdrLpbkControl0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - adr slice power reduction disable for slice 0."]
    #[inline(always)]
    pub fn phy_adr_pwr_rdc_disable_0(&self) -> PhyAdrPwrRdcDisable0R {
        PhyAdrPwrRdcDisable0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initiates a snapshot of the internal observation registers for address slice 0. Set to 1 to trigger. WRITE- ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_adr_snap_obs_regs_0(&mut self) -> ScPhyAdrSnapObsRegs0W<DenaliPhy518Spec> {
        ScPhyAdrSnapObsRegs0W::new(self, 0)
    }
    #[doc = "Bit 8 - Enables tsel_en for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_tsel_enable_0(&mut self) -> PhyAdrTselEnable0W<DenaliPhy518Spec> {
        PhyAdrTselEnable0W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Loopback control bits for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_lpbk_control_0(&mut self) -> PhyAdrLpbkControl0W<DenaliPhy518Spec> {
        PhyAdrLpbkControl0W::new(self, 16)
    }
    #[doc = "Bit 24 - adr slice power reduction disable for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_pwr_rdc_disable_0(&mut self) -> PhyAdrPwrRdcDisable0W<DenaliPhy518Spec> {
        PhyAdrPwrRdcDisable0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_518::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_518::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy518Spec;
impl crate::RegisterSpec for DenaliPhy518Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_518::R`](R) reader structure"]
impl crate::Readable for DenaliPhy518Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_518::W`](W) writer structure"]
impl crate::Writable for DenaliPhy518Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_518 to value 0"]
impl crate::Resettable for DenaliPhy518Spec {
    const RESET_VALUE: u32 = 0;
}
