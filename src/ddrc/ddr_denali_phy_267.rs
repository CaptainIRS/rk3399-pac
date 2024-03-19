#[doc = "Register `DDR_DENALI_PHY_267` reader"]
pub type R = crate::R<DdrDenaliPhy267Spec>;
#[doc = "Register `DDR_DENALI_PHY_267` writer"]
pub type W = crate::W<DdrDenaliPhy267Spec>;
#[doc = "Field `PHY_RDDQS_DQ_BYPASS_SLAVE_DELAY_2` reader - Read DQS data clock bypass mode slave delay setting for slice 2."]
pub type PhyRddqsDqBypassSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ_BYPASS_SLAVE_DELAY_2` writer - Read DQS data clock bypass mode slave delay setting for slice 2."]
pub type PhyRddqsDqBypassSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_GATE_ERROR_DELAY_SELECT_2` reader - Number of cycles to wait for the DQS gate to close before flagging an error for slice 2."]
pub type PhyGateErrorDelaySelect2R = crate::FieldReader;
#[doc = "Field `PHY_GATE_ERROR_DELAY_SELECT_2` writer - Number of cycles to wait for the DQS gate to close before flagging an error for slice 2."]
pub type PhyGateErrorDelaySelect2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SC_PHY_SNAP_OBS_REGS_2` writer - Initiates a snapshot of the internal observation registers for slice 2. Set to 1 to trigger."]
pub type ScPhySnapObsRegs2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Read DQS data clock bypass mode slave delay setting for slice 2."]
    #[inline(always)]
    pub fn phy_rddqs_dq_bypass_slave_delay_2(&self) -> PhyRddqsDqBypassSlaveDelay2R {
        PhyRddqsDqBypassSlaveDelay2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:20 - Number of cycles to wait for the DQS gate to close before flagging an error for slice 2."]
    #[inline(always)]
    pub fn phy_gate_error_delay_select_2(&self) -> PhyGateErrorDelaySelect2R {
        PhyGateErrorDelaySelect2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQS data clock bypass mode slave delay setting for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq_bypass_slave_delay_2(
        &mut self,
    ) -> PhyRddqsDqBypassSlaveDelay2W<DdrDenaliPhy267Spec> {
        PhyRddqsDqBypassSlaveDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:20 - Number of cycles to wait for the DQS gate to close before flagging an error for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gate_error_delay_select_2(
        &mut self,
    ) -> PhyGateErrorDelaySelect2W<DdrDenaliPhy267Spec> {
        PhyGateErrorDelaySelect2W::new(self, 16)
    }
    #[doc = "Bit 24 - Initiates a snapshot of the internal observation registers for slice 2. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_snap_obs_regs_2(&mut self) -> ScPhySnapObsRegs2W<DdrDenaliPhy267Spec> {
        ScPhySnapObsRegs2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_267::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_267::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy267Spec;
impl crate::RegisterSpec for DdrDenaliPhy267Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_267::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy267Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_267::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy267Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_267 to value 0"]
impl crate::Resettable for DdrDenaliPhy267Spec {
    const RESET_VALUE: u32 = 0;
}
