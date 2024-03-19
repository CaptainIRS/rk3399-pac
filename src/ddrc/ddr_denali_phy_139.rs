#[doc = "Register `DDR_DENALI_PHY_139` reader"]
pub type R = crate::R<DdrDenaliPhy139Spec>;
#[doc = "Register `DDR_DENALI_PHY_139` writer"]
pub type W = crate::W<DdrDenaliPhy139Spec>;
#[doc = "Field `PHY_RDDQS_DQ_BYPASS_SLAVE_DELAY_1` reader - Read DQS data clock bypass mode slave delay setting for slice 1."]
pub type PhyRddqsDqBypassSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ_BYPASS_SLAVE_DELAY_1` writer - Read DQS data clock bypass mode slave delay setting for slice 1."]
pub type PhyRddqsDqBypassSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_GATE_ERROR_DELAY_SELECT_1` reader - Number of cycles to wait for the DQS gate to close before flagging an error for slice 1."]
pub type PhyGateErrorDelaySelect1R = crate::FieldReader;
#[doc = "Field `PHY_GATE_ERROR_DELAY_SELECT_1` writer - Number of cycles to wait for the DQS gate to close before flagging an error for slice 1."]
pub type PhyGateErrorDelaySelect1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SC_PHY_SNAP_OBS_REGS_1` writer - Initiates a snapshot of the internal observation registers for slice 1. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhySnapObsRegs1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Read DQS data clock bypass mode slave delay setting for slice 1."]
    #[inline(always)]
    pub fn phy_rddqs_dq_bypass_slave_delay_1(&self) -> PhyRddqsDqBypassSlaveDelay1R {
        PhyRddqsDqBypassSlaveDelay1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:20 - Number of cycles to wait for the DQS gate to close before flagging an error for slice 1."]
    #[inline(always)]
    pub fn phy_gate_error_delay_select_1(&self) -> PhyGateErrorDelaySelect1R {
        PhyGateErrorDelaySelect1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQS data clock bypass mode slave delay setting for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq_bypass_slave_delay_1(
        &mut self,
    ) -> PhyRddqsDqBypassSlaveDelay1W<DdrDenaliPhy139Spec> {
        PhyRddqsDqBypassSlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:20 - Number of cycles to wait for the DQS gate to close before flagging an error for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gate_error_delay_select_1(
        &mut self,
    ) -> PhyGateErrorDelaySelect1W<DdrDenaliPhy139Spec> {
        PhyGateErrorDelaySelect1W::new(self, 16)
    }
    #[doc = "Bit 24 - Initiates a snapshot of the internal observation registers for slice 1. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_snap_obs_regs_1(&mut self) -> ScPhySnapObsRegs1W<DdrDenaliPhy139Spec> {
        ScPhySnapObsRegs1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_139::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_139::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy139Spec;
impl crate::RegisterSpec for DdrDenaliPhy139Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_139::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy139Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_139::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy139Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_139 to value 0"]
impl crate::Resettable for DdrDenaliPhy139Spec {
    const RESET_VALUE: u32 = 0;
}
