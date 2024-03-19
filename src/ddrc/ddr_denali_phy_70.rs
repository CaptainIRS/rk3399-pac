#[doc = "Register `DDR_DENALI_PHY_70` reader"]
pub type R = crate::R<DdrDenaliPhy70Spec>;
#[doc = "Register `DDR_DENALI_PHY_70` writer"]
pub type W = crate::W<DdrDenaliPhy70Spec>;
#[doc = "Field `PHY_RDDQS_DQ1_FALL_SLAVE_DELAY_0` reader - Falling edge read DQS slave delay setting for DQ1 for slice 0."]
pub type PhyRddqsDq1FallSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ1_FALL_SLAVE_DELAY_0` writer - Falling edge read DQS slave delay setting for DQ1 for slice 0."]
pub type PhyRddqsDq1FallSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_DQ2_RISE_SLAVE_DELAY_0` reader - Rising edge read DQS slave delay setting for DQ2 for slice 0."]
pub type PhyRddqsDq2RiseSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ2_RISE_SLAVE_DELAY_0` writer - Rising edge read DQS slave delay setting for DQ2 for slice 0."]
pub type PhyRddqsDq2RiseSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ1 for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_dq1_fall_slave_delay_0(&self) -> PhyRddqsDq1FallSlaveDelay0R {
        PhyRddqsDq1FallSlaveDelay0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ2 for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_dq2_rise_slave_delay_0(&self) -> PhyRddqsDq2RiseSlaveDelay0R {
        PhyRddqsDq2RiseSlaveDelay0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ1 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq1_fall_slave_delay_0(
        &mut self,
    ) -> PhyRddqsDq1FallSlaveDelay0W<DdrDenaliPhy70Spec> {
        PhyRddqsDq1FallSlaveDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ2 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq2_rise_slave_delay_0(
        &mut self,
    ) -> PhyRddqsDq2RiseSlaveDelay0W<DdrDenaliPhy70Spec> {
        PhyRddqsDq2RiseSlaveDelay0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_70::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_70::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy70Spec;
impl crate::RegisterSpec for DdrDenaliPhy70Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_70::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy70Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_70::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy70Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_70 to value 0"]
impl crate::Resettable for DdrDenaliPhy70Spec {
    const RESET_VALUE: u32 = 0;
}
