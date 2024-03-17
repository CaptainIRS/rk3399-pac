#[doc = "Register `DENALI_PHY_459` reader"]
pub type R = crate::R<DenaliPhy459Spec>;
#[doc = "Register `DENALI_PHY_459` writer"]
pub type W = crate::W<DenaliPhy459Spec>;
#[doc = "Field `PHY_RDDQS_DQ6_FALL_SLAVE_DELAY_3` reader - Falling edge read DQS slave delay setting for DQ6 for slice 3."]
pub type PhyRddqsDq6FallSlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ6_FALL_SLAVE_DELAY_3` writer - Falling edge read DQS slave delay setting for DQ6 for slice 3."]
pub type PhyRddqsDq6FallSlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_DQ7_RISE_SLAVE_DELAY_3` reader - Rising edge read DQS slave delay setting for DQ7 for slice 3."]
pub type PhyRddqsDq7RiseSlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ7_RISE_SLAVE_DELAY_3` writer - Rising edge read DQS slave delay setting for DQ7 for slice 3."]
pub type PhyRddqsDq7RiseSlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ6 for slice 3."]
    #[inline(always)]
    pub fn phy_rddqs_dq6_fall_slave_delay_3(&self) -> PhyRddqsDq6FallSlaveDelay3R {
        PhyRddqsDq6FallSlaveDelay3R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ7 for slice 3."]
    #[inline(always)]
    pub fn phy_rddqs_dq7_rise_slave_delay_3(&self) -> PhyRddqsDq7RiseSlaveDelay3R {
        PhyRddqsDq7RiseSlaveDelay3R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ6 for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq6_fall_slave_delay_3(
        &mut self,
    ) -> PhyRddqsDq6FallSlaveDelay3W<DenaliPhy459Spec> {
        PhyRddqsDq6FallSlaveDelay3W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ7 for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq7_rise_slave_delay_3(
        &mut self,
    ) -> PhyRddqsDq7RiseSlaveDelay3W<DenaliPhy459Spec> {
        PhyRddqsDq7RiseSlaveDelay3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_459::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_459::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy459Spec;
impl crate::RegisterSpec for DenaliPhy459Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_459::R`](R) reader structure"]
impl crate::Readable for DenaliPhy459Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_459::W`](W) writer structure"]
impl crate::Writable for DenaliPhy459Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_459 to value 0"]
impl crate::Resettable for DenaliPhy459Spec {
    const RESET_VALUE: u32 = 0;
}
