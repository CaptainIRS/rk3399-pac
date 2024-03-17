#[doc = "Register `DENALI_PHY_386` reader"]
pub type R = crate::R<DenaliPhy386Spec>;
#[doc = "Register `DENALI_PHY_386` writer"]
pub type W = crate::W<DenaliPhy386Spec>;
#[doc = "Field `PHY_RDDQS_GATE_BYPASS_SLAVE_DELAY_3` reader - Read DQS bypass mode slave delay setting for slice 3."]
pub type PhyRddqsGateBypassSlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_GATE_BYPASS_SLAVE_DELAY_3` writer - Read DQS bypass mode slave delay setting for slice 3."]
pub type PhyRddqsGateBypassSlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_BYPASS_TWO_CYC_PREAMBLE_3` reader - PHY two_cycle_preamble for bypass mode for slice 3."]
pub type PhyBypassTwoCycPreamble3R = crate::FieldReader;
#[doc = "Field `PHY_BYPASS_TWO_CYC_PREAMBLE_3` writer - PHY two_cycle_preamble for bypass mode for slice 3."]
pub type PhyBypassTwoCycPreamble3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_CLK_BYPASS_OVERRIDE_3` reader - Bypass mode override setting for slice 3."]
pub type PhyClkBypassOverride3R = crate::BitReader;
#[doc = "Field `PHY_CLK_BYPASS_OVERRIDE_3` writer - Bypass mode override setting for slice 3."]
pub type PhyClkBypassOverride3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Read DQS bypass mode slave delay setting for slice 3."]
    #[inline(always)]
    pub fn phy_rddqs_gate_bypass_slave_delay_3(&self) -> PhyRddqsGateBypassSlaveDelay3R {
        PhyRddqsGateBypassSlaveDelay3R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:17 - PHY two_cycle_preamble for bypass mode for slice 3."]
    #[inline(always)]
    pub fn phy_bypass_two_cyc_preamble_3(&self) -> PhyBypassTwoCycPreamble3R {
        PhyBypassTwoCycPreamble3R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Bypass mode override setting for slice 3."]
    #[inline(always)]
    pub fn phy_clk_bypass_override_3(&self) -> PhyClkBypassOverride3R {
        PhyClkBypassOverride3R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQS bypass mode slave delay setting for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_gate_bypass_slave_delay_3(
        &mut self,
    ) -> PhyRddqsGateBypassSlaveDelay3W<DenaliPhy386Spec> {
        PhyRddqsGateBypassSlaveDelay3W::new(self, 0)
    }
    #[doc = "Bits 16:17 - PHY two_cycle_preamble for bypass mode for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_bypass_two_cyc_preamble_3(&mut self) -> PhyBypassTwoCycPreamble3W<DenaliPhy386Spec> {
        PhyBypassTwoCycPreamble3W::new(self, 16)
    }
    #[doc = "Bit 24 - Bypass mode override setting for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_bypass_override_3(&mut self) -> PhyClkBypassOverride3W<DenaliPhy386Spec> {
        PhyClkBypassOverride3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_386::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_386::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy386Spec;
impl crate::RegisterSpec for DenaliPhy386Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_386::R`](R) reader structure"]
impl crate::Readable for DenaliPhy386Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_386::W`](W) writer structure"]
impl crate::Writable for DenaliPhy386Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_386 to value 0"]
impl crate::Resettable for DenaliPhy386Spec {
    const RESET_VALUE: u32 = 0;
}
