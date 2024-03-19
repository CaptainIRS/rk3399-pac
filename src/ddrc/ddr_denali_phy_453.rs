#[doc = "Register `DDR_DENALI_PHY_453` reader"]
pub type R = crate::R<DdrDenaliPhy453Spec>;
#[doc = "Register `DDR_DENALI_PHY_453` writer"]
pub type W = crate::W<DdrDenaliPhy453Spec>;
#[doc = "Field `PHY_RDDQS_DQ0_FALL_SLAVE_DELAY_3` reader - Falling edge read DQS slave delay setting for DQ0 for slice 3."]
pub type PhyRddqsDq0FallSlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ0_FALL_SLAVE_DELAY_3` writer - Falling edge read DQS slave delay setting for DQ0 for slice 3."]
pub type PhyRddqsDq0FallSlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_DQ1_RISE_SLAVE_DELAY_3` reader - Rising edge read DQS slave delay setting for DQ1 for slice 3."]
pub type PhyRddqsDq1RiseSlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ1_RISE_SLAVE_DELAY_3` writer - Rising edge read DQS slave delay setting for DQ1 for slice 3."]
pub type PhyRddqsDq1RiseSlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ0 for slice 3."]
    #[inline(always)]
    pub fn phy_rddqs_dq0_fall_slave_delay_3(&self) -> PhyRddqsDq0FallSlaveDelay3R {
        PhyRddqsDq0FallSlaveDelay3R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ1 for slice 3."]
    #[inline(always)]
    pub fn phy_rddqs_dq1_rise_slave_delay_3(&self) -> PhyRddqsDq1RiseSlaveDelay3R {
        PhyRddqsDq1RiseSlaveDelay3R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ0 for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq0_fall_slave_delay_3(
        &mut self,
    ) -> PhyRddqsDq0FallSlaveDelay3W<DdrDenaliPhy453Spec> {
        PhyRddqsDq0FallSlaveDelay3W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ1 for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq1_rise_slave_delay_3(
        &mut self,
    ) -> PhyRddqsDq1RiseSlaveDelay3W<DdrDenaliPhy453Spec> {
        PhyRddqsDq1RiseSlaveDelay3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_453::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_453::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy453Spec;
impl crate::RegisterSpec for DdrDenaliPhy453Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_453::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy453Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_453::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy453Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_453 to value 0"]
impl crate::Resettable for DdrDenaliPhy453Spec {
    const RESET_VALUE: u32 = 0;
}
