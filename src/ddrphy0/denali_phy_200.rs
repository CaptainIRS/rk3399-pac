#[doc = "Register `DENALI_PHY_200` reader"]
pub type R = crate::R<DenaliPhy200Spec>;
#[doc = "Register `DENALI_PHY_200` writer"]
pub type W = crate::W<DenaliPhy200Spec>;
#[doc = "Field `PHY_RDDQS_DQ3_FALL_SLAVE_DELAY_1` reader - Falling edge read DQS slave delay setting for DQ3 for slice 1."]
pub type PhyRddqsDq3FallSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ3_FALL_SLAVE_DELAY_1` writer - Falling edge read DQS slave delay setting for DQ3 for slice 1."]
pub type PhyRddqsDq3FallSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_DQ4_RISE_SLAVE_DELAY_1` reader - Rising edge read DQS slave delay setting for DQ4 for slice 1."]
pub type PhyRddqsDq4RiseSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ4_RISE_SLAVE_DELAY_1` writer - Rising edge read DQS slave delay setting for DQ4 for slice 1."]
pub type PhyRddqsDq4RiseSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ3 for slice 1."]
    #[inline(always)]
    pub fn phy_rddqs_dq3_fall_slave_delay_1(&self) -> PhyRddqsDq3FallSlaveDelay1R {
        PhyRddqsDq3FallSlaveDelay1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ4 for slice 1."]
    #[inline(always)]
    pub fn phy_rddqs_dq4_rise_slave_delay_1(&self) -> PhyRddqsDq4RiseSlaveDelay1R {
        PhyRddqsDq4RiseSlaveDelay1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ3 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq3_fall_slave_delay_1(
        &mut self,
    ) -> PhyRddqsDq3FallSlaveDelay1W<DenaliPhy200Spec> {
        PhyRddqsDq3FallSlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ4 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq4_rise_slave_delay_1(
        &mut self,
    ) -> PhyRddqsDq4RiseSlaveDelay1W<DenaliPhy200Spec> {
        PhyRddqsDq4RiseSlaveDelay1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_200::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_200::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy200Spec;
impl crate::RegisterSpec for DenaliPhy200Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_200::R`](R) reader structure"]
impl crate::Readable for DenaliPhy200Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_200::W`](W) writer structure"]
impl crate::Writable for DenaliPhy200Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_200 to value 0"]
impl crate::Resettable for DenaliPhy200Spec {
    const RESET_VALUE: u32 = 0;
}
