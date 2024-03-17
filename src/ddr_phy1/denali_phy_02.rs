#[doc = "Register `DENALI_PHY_02` reader"]
pub type R = crate::R<DenaliPhy02Spec>;
#[doc = "Register `DENALI_PHY_02` writer"]
pub type W = crate::W<DenaliPhy02Spec>;
#[doc = "Field `PHY_RDDQS_GATE_BYPASS_SLAVE_DELAY_0` reader - Read DQS bypass mode slave delay setting for slice 0."]
pub type PhyRddqsGateBypassSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_GATE_BYPASS_SLAVE_DELAY_0` writer - Read DQS bypass mode slave delay setting for slice 0."]
pub type PhyRddqsGateBypassSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_BYPASS_TWO_CYC_PREAMBLE_0` reader - PHY two_cycle_preamble for bypass mode for slice 0."]
pub type PhyBypassTwoCycPreamble0R = crate::FieldReader;
#[doc = "Field `PHY_BYPASS_TWO_CYC_PREAMBLE_0` writer - PHY two_cycle_preamble for bypass mode for slice 0."]
pub type PhyBypassTwoCycPreamble0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_CLK_BYPASS_OVERRIDE_0` reader - Bypass mode override setting for slice 0."]
pub type PhyClkBypassOverride0R = crate::BitReader;
#[doc = "Field `PHY_CLK_BYPASS_OVERRIDE_0` writer - Bypass mode override setting for slice 0."]
pub type PhyClkBypassOverride0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Read DQS bypass mode slave delay setting for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_gate_bypass_slave_delay_0(&self) -> PhyRddqsGateBypassSlaveDelay0R {
        PhyRddqsGateBypassSlaveDelay0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:17 - PHY two_cycle_preamble for bypass mode for slice 0."]
    #[inline(always)]
    pub fn phy_bypass_two_cyc_preamble_0(&self) -> PhyBypassTwoCycPreamble0R {
        PhyBypassTwoCycPreamble0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Bypass mode override setting for slice 0."]
    #[inline(always)]
    pub fn phy_clk_bypass_override_0(&self) -> PhyClkBypassOverride0R {
        PhyClkBypassOverride0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQS bypass mode slave delay setting for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_gate_bypass_slave_delay_0(
        &mut self,
    ) -> PhyRddqsGateBypassSlaveDelay0W<DenaliPhy02Spec> {
        PhyRddqsGateBypassSlaveDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:17 - PHY two_cycle_preamble for bypass mode for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_bypass_two_cyc_preamble_0(&mut self) -> PhyBypassTwoCycPreamble0W<DenaliPhy02Spec> {
        PhyBypassTwoCycPreamble0W::new(self, 16)
    }
    #[doc = "Bit 24 - Bypass mode override setting for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_bypass_override_0(&mut self) -> PhyClkBypassOverride0W<DenaliPhy02Spec> {
        PhyClkBypassOverride0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_02::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_02::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy02Spec;
impl crate::RegisterSpec for DenaliPhy02Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_02::R`](R) reader structure"]
impl crate::Readable for DenaliPhy02Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_02::W`](W) writer structure"]
impl crate::Writable for DenaliPhy02Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_02 to value 0"]
impl crate::Resettable for DenaliPhy02Spec {
    const RESET_VALUE: u32 = 0;
}
