#[doc = "Register `DENALI_PHY_646` reader"]
pub type R = crate::R<DenaliPhy646Spec>;
#[doc = "Register `DENALI_PHY_646` writer"]
pub type W = crate::W<DenaliPhy646Spec>;
#[doc = "Field `SC_PHY_ADR_SNAP_OBS_REGS_1` writer - Initiates a snapshot of the internal observation registers for address slice 1. Set to 1 to trigger."]
pub type ScPhyAdrSnapObsRegs1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_ADR_TSEL_ENABLE_1` reader - Enables tsel_en for address slice 1."]
pub type PhyAdrTselEnable1R = crate::BitReader;
#[doc = "Field `PHY_ADR_TSEL_ENABLE_1` writer - Enables tsel_en for address slice 1."]
pub type PhyAdrTselEnable1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_ADR_LPBK_CONTROL_1` reader - Loopback control bits for address slice 1."]
pub type PhyAdrLpbkControl1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_LPBK_CONTROL_1` writer - Loopback control bits for address slice 1."]
pub type PhyAdrLpbkControl1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_ADR_PWR_RDC_DISABLE_1` reader - adr slice power reduction disable for slice 1."]
pub type PhyAdrPwrRdcDisable1R = crate::BitReader;
#[doc = "Field `PHY_ADR_PWR_RDC_DISABLE_1` writer - adr slice power reduction disable for slice 1."]
pub type PhyAdrPwrRdcDisable1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - Enables tsel_en for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_tsel_enable_1(&self) -> PhyAdrTselEnable1R {
        PhyAdrTselEnable1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Loopback control bits for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_lpbk_control_1(&self) -> PhyAdrLpbkControl1R {
        PhyAdrLpbkControl1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - adr slice power reduction disable for slice 1."]
    #[inline(always)]
    pub fn phy_adr_pwr_rdc_disable_1(&self) -> PhyAdrPwrRdcDisable1R {
        PhyAdrPwrRdcDisable1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initiates a snapshot of the internal observation registers for address slice 1. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_adr_snap_obs_regs_1(&mut self) -> ScPhyAdrSnapObsRegs1W<DenaliPhy646Spec> {
        ScPhyAdrSnapObsRegs1W::new(self, 0)
    }
    #[doc = "Bit 8 - Enables tsel_en for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_tsel_enable_1(&mut self) -> PhyAdrTselEnable1W<DenaliPhy646Spec> {
        PhyAdrTselEnable1W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Loopback control bits for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_lpbk_control_1(&mut self) -> PhyAdrLpbkControl1W<DenaliPhy646Spec> {
        PhyAdrLpbkControl1W::new(self, 16)
    }
    #[doc = "Bit 24 - adr slice power reduction disable for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_pwr_rdc_disable_1(&mut self) -> PhyAdrPwrRdcDisable1W<DenaliPhy646Spec> {
        PhyAdrPwrRdcDisable1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_646::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_646::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy646Spec;
impl crate::RegisterSpec for DenaliPhy646Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_646::R`](R) reader structure"]
impl crate::Readable for DenaliPhy646Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_646::W`](W) writer structure"]
impl crate::Writable for DenaliPhy646Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_646 to value 0"]
impl crate::Resettable for DenaliPhy646Spec {
    const RESET_VALUE: u32 = 0;
}
