#[doc = "Register `DENALI_PHY_461` reader"]
pub type R = crate::R<DenaliPhy461Spec>;
#[doc = "Register `DENALI_PHY_461` writer"]
pub type W = crate::W<DenaliPhy461Spec>;
#[doc = "Field `PHY_RDDQS_DM_FALL_SLAVE_DELAY_3` reader - Falling edge read DQS slave delay setting for DM for slice 3."]
pub type PhyRddqsDmFallSlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DM_FALL_SLAVE_DELAY_3` writer - Falling edge read DQS slave delay setting for DM for slice 3."]
pub type PhyRddqsDmFallSlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_GATE_SLAVE_DELAY_3` reader - Read DQS slave delay setting for slice 3."]
pub type PhyRddqsGateSlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_GATE_SLAVE_DELAY_3` writer - Read DQS slave delay setting for slice 3."]
pub type PhyRddqsGateSlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DM for slice 3."]
    #[inline(always)]
    pub fn phy_rddqs_dm_fall_slave_delay_3(&self) -> PhyRddqsDmFallSlaveDelay3R {
        PhyRddqsDmFallSlaveDelay3R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Read DQS slave delay setting for slice 3."]
    #[inline(always)]
    pub fn phy_rddqs_gate_slave_delay_3(&self) -> PhyRddqsGateSlaveDelay3R {
        PhyRddqsGateSlaveDelay3R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DM for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dm_fall_slave_delay_3(
        &mut self,
    ) -> PhyRddqsDmFallSlaveDelay3W<DenaliPhy461Spec> {
        PhyRddqsDmFallSlaveDelay3W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read DQS slave delay setting for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_gate_slave_delay_3(&mut self) -> PhyRddqsGateSlaveDelay3W<DenaliPhy461Spec> {
        PhyRddqsGateSlaveDelay3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_461::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_461::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy461Spec;
impl crate::RegisterSpec for DenaliPhy461Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_461::R`](R) reader structure"]
impl crate::Readable for DenaliPhy461Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_461::W`](W) writer structure"]
impl crate::Writable for DenaliPhy461Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_461 to value 0"]
impl crate::Resettable for DenaliPhy461Spec {
    const RESET_VALUE: u32 = 0;
}
