#[doc = "Register `DDR_DENALI_PHY_333` reader"]
pub type R = crate::R<DdrDenaliPhy333Spec>;
#[doc = "Register `DDR_DENALI_PHY_333` writer"]
pub type W = crate::W<DdrDenaliPhy333Spec>;
#[doc = "Field `PHY_RDDQS_DM_FALL_SLAVE_DELAY_2` reader - Falling edge read DQS slave delay setting for DM for slice 2."]
pub type PhyRddqsDmFallSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DM_FALL_SLAVE_DELAY_2` writer - Falling edge read DQS slave delay setting for DM for slice 2."]
pub type PhyRddqsDmFallSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_GATE_SLAVE_DELAY_2` reader - Read DQS slave delay setting for slice 2."]
pub type PhyRddqsGateSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_GATE_SLAVE_DELAY_2` writer - Read DQS slave delay setting for slice 2."]
pub type PhyRddqsGateSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DM for slice 2."]
    #[inline(always)]
    pub fn phy_rddqs_dm_fall_slave_delay_2(&self) -> PhyRddqsDmFallSlaveDelay2R {
        PhyRddqsDmFallSlaveDelay2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Read DQS slave delay setting for slice 2."]
    #[inline(always)]
    pub fn phy_rddqs_gate_slave_delay_2(&self) -> PhyRddqsGateSlaveDelay2R {
        PhyRddqsGateSlaveDelay2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DM for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dm_fall_slave_delay_2(
        &mut self,
    ) -> PhyRddqsDmFallSlaveDelay2W<DdrDenaliPhy333Spec> {
        PhyRddqsDmFallSlaveDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read DQS slave delay setting for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_gate_slave_delay_2(
        &mut self,
    ) -> PhyRddqsGateSlaveDelay2W<DdrDenaliPhy333Spec> {
        PhyRddqsGateSlaveDelay2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_333::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_333::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy333Spec;
impl crate::RegisterSpec for DdrDenaliPhy333Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_333::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy333Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_333::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy333Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_333 to value 0"]
impl crate::Resettable for DdrDenaliPhy333Spec {
    const RESET_VALUE: u32 = 0;
}
