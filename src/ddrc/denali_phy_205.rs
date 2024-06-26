#[doc = "Register `DENALI_PHY_205` reader"]
pub type R = crate::R<DenaliPhy205Spec>;
#[doc = "Register `DENALI_PHY_205` writer"]
pub type W = crate::W<DenaliPhy205Spec>;
#[doc = "Field `PHY_RDDQS_DM_FALL_SLAVE_DELAY_1` reader - Falling edge read DQS slave delay setting for DM for slice 1."]
pub type PhyRddqsDmFallSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DM_FALL_SLAVE_DELAY_1` writer - Falling edge read DQS slave delay setting for DM for slice 1."]
pub type PhyRddqsDmFallSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_GATE_SLAVE_DELAY_1` reader - Read DQS slave delay setting for slice 1."]
pub type PhyRddqsGateSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_GATE_SLAVE_DELAY_1` writer - Read DQS slave delay setting for slice 1."]
pub type PhyRddqsGateSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DM for slice 1."]
    #[inline(always)]
    pub fn phy_rddqs_dm_fall_slave_delay_1(&self) -> PhyRddqsDmFallSlaveDelay1R {
        PhyRddqsDmFallSlaveDelay1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Read DQS slave delay setting for slice 1."]
    #[inline(always)]
    pub fn phy_rddqs_gate_slave_delay_1(&self) -> PhyRddqsGateSlaveDelay1R {
        PhyRddqsGateSlaveDelay1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DM for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dm_fall_slave_delay_1(
        &mut self,
    ) -> PhyRddqsDmFallSlaveDelay1W<DenaliPhy205Spec> {
        PhyRddqsDmFallSlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read DQS slave delay setting for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_gate_slave_delay_1(&mut self) -> PhyRddqsGateSlaveDelay1W<DenaliPhy205Spec> {
        PhyRddqsGateSlaveDelay1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_205::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_205::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy205Spec;
impl crate::RegisterSpec for DenaliPhy205Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_205::R`](R) reader structure"]
impl crate::Readable for DenaliPhy205Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_205::W`](W) writer structure"]
impl crate::Writable for DenaliPhy205Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_205 to value 0"]
impl crate::Resettable for DenaliPhy205Spec {
    const RESET_VALUE: u32 = 0;
}
