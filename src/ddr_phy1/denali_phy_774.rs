#[doc = "Register `DENALI_PHY_774` reader"]
pub type R = crate::R<DenaliPhy774Spec>;
#[doc = "Register `DENALI_PHY_774` writer"]
pub type W = crate::W<DenaliPhy774Spec>;
#[doc = "Field `SC_PHY_ADR_SNAP_OBS_REGS_2` writer - Initiates a snapshot of the internal observation registers for address slice 2. Set to 1 to trigger. WRITE- ONLY"]
pub type ScPhyAdrSnapObsRegs2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_ADR_TSEL_ENABLE_2` reader - Enables tsel_en for address slice 2."]
pub type PhyAdrTselEnable2R = crate::BitReader;
#[doc = "Field `PHY_ADR_TSEL_ENABLE_2` writer - Enables tsel_en for address slice 2."]
pub type PhyAdrTselEnable2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_ADR_LPBK_CONTROL_2` reader - Loopback control bits for address slice 2."]
pub type PhyAdrLpbkControl2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_LPBK_CONTROL_2` writer - Loopback control bits for address slice 2."]
pub type PhyAdrLpbkControl2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_ADR_PWR_RDC_DISABLE_2` reader - adr slice power reduction disable for slice 2."]
pub type PhyAdrPwrRdcDisable2R = crate::BitReader;
#[doc = "Field `PHY_ADR_PWR_RDC_DISABLE_2` writer - adr slice power reduction disable for slice 2."]
pub type PhyAdrPwrRdcDisable2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - Enables tsel_en for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_tsel_enable_2(&self) -> PhyAdrTselEnable2R {
        PhyAdrTselEnable2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Loopback control bits for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_lpbk_control_2(&self) -> PhyAdrLpbkControl2R {
        PhyAdrLpbkControl2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - adr slice power reduction disable for slice 2."]
    #[inline(always)]
    pub fn phy_adr_pwr_rdc_disable_2(&self) -> PhyAdrPwrRdcDisable2R {
        PhyAdrPwrRdcDisable2R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initiates a snapshot of the internal observation registers for address slice 2. Set to 1 to trigger. WRITE- ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_adr_snap_obs_regs_2(&mut self) -> ScPhyAdrSnapObsRegs2W<DenaliPhy774Spec> {
        ScPhyAdrSnapObsRegs2W::new(self, 0)
    }
    #[doc = "Bit 8 - Enables tsel_en for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_tsel_enable_2(&mut self) -> PhyAdrTselEnable2W<DenaliPhy774Spec> {
        PhyAdrTselEnable2W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Loopback control bits for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_lpbk_control_2(&mut self) -> PhyAdrLpbkControl2W<DenaliPhy774Spec> {
        PhyAdrLpbkControl2W::new(self, 16)
    }
    #[doc = "Bit 24 - adr slice power reduction disable for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_pwr_rdc_disable_2(&mut self) -> PhyAdrPwrRdcDisable2W<DenaliPhy774Spec> {
        PhyAdrPwrRdcDisable2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_774::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_774::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy774Spec;
impl crate::RegisterSpec for DenaliPhy774Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_774::R`](R) reader structure"]
impl crate::Readable for DenaliPhy774Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_774::W`](W) writer structure"]
impl crate::Writable for DenaliPhy774Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_774 to value 0"]
impl crate::Resettable for DenaliPhy774Spec {
    const RESET_VALUE: u32 = 0;
}
