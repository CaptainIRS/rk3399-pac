#[doc = "Register `DENALI_PHY_11` reader"]
pub type R = crate::R<DenaliPhy11Spec>;
#[doc = "Register `DENALI_PHY_11` writer"]
pub type W = crate::W<DenaliPhy11Spec>;
#[doc = "Field `PHY_RDDQS_DQ_BYPASS_SLAVE_DELAY_0` reader - Read DQS data clock bypass mode slave delay setting for slice 0."]
pub type PhyRddqsDqBypassSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ_BYPASS_SLAVE_DELAY_0` writer - Read DQS data clock bypass mode slave delay setting for slice 0."]
pub type PhyRddqsDqBypassSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_GATE_ERROR_DELAY_SELECT_0` reader - Number of cycles to wait for the DQS gate to close before flagging an error for slice 0."]
pub type PhyGateErrorDelaySelect0R = crate::FieldReader;
#[doc = "Field `PHY_GATE_ERROR_DELAY_SELECT_0` writer - Number of cycles to wait for the DQS gate to close before flagging an error for slice 0."]
pub type PhyGateErrorDelaySelect0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SC_PHY_SNAP_OBS_REGS_0` writer - Initiates a snapshot of the internal observation registers for slice 0. Set to 1 to trigger."]
pub type ScPhySnapObsRegs0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Read DQS data clock bypass mode slave delay setting for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_dq_bypass_slave_delay_0(&self) -> PhyRddqsDqBypassSlaveDelay0R {
        PhyRddqsDqBypassSlaveDelay0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:20 - Number of cycles to wait for the DQS gate to close before flagging an error for slice 0."]
    #[inline(always)]
    pub fn phy_gate_error_delay_select_0(&self) -> PhyGateErrorDelaySelect0R {
        PhyGateErrorDelaySelect0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQS data clock bypass mode slave delay setting for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq_bypass_slave_delay_0(
        &mut self,
    ) -> PhyRddqsDqBypassSlaveDelay0W<DenaliPhy11Spec> {
        PhyRddqsDqBypassSlaveDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:20 - Number of cycles to wait for the DQS gate to close before flagging an error for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gate_error_delay_select_0(&mut self) -> PhyGateErrorDelaySelect0W<DenaliPhy11Spec> {
        PhyGateErrorDelaySelect0W::new(self, 16)
    }
    #[doc = "Bit 24 - Initiates a snapshot of the internal observation registers for slice 0. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_snap_obs_regs_0(&mut self) -> ScPhySnapObsRegs0W<DenaliPhy11Spec> {
        ScPhySnapObsRegs0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy11Spec;
impl crate::RegisterSpec for DenaliPhy11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_11::R`](R) reader structure"]
impl crate::Readable for DenaliPhy11Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_11::W`](W) writer structure"]
impl crate::Writable for DenaliPhy11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_11 to value 0"]
impl crate::Resettable for DenaliPhy11Spec {
    const RESET_VALUE: u32 = 0;
}
