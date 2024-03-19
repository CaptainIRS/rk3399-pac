#[doc = "Register `DDR_DENALI_PHY_197` reader"]
pub type R = crate::R<DdrDenaliPhy197Spec>;
#[doc = "Register `DDR_DENALI_PHY_197` writer"]
pub type W = crate::W<DdrDenaliPhy197Spec>;
#[doc = "Field `PHY_RDDQS_DQ0_FALL_SLAVE_DELAY_1` reader - Falling edge read DQS slave delay setting for DQ0 for slice 1."]
pub type PhyRddqsDq0FallSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ0_FALL_SLAVE_DELAY_1` writer - Falling edge read DQS slave delay setting for DQ0 for slice 1."]
pub type PhyRddqsDq0FallSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_DQ1_RISE_SLAVE_DELAY_1` reader - Rising edge read DQS slave delay setting for DQ1 for slice 1."]
pub type PhyRddqsDq1RiseSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ1_RISE_SLAVE_DELAY_1` writer - Rising edge read DQS slave delay setting for DQ1 for slice 1."]
pub type PhyRddqsDq1RiseSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ0 for slice 1."]
    #[inline(always)]
    pub fn phy_rddqs_dq0_fall_slave_delay_1(&self) -> PhyRddqsDq0FallSlaveDelay1R {
        PhyRddqsDq0FallSlaveDelay1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ1 for slice 1."]
    #[inline(always)]
    pub fn phy_rddqs_dq1_rise_slave_delay_1(&self) -> PhyRddqsDq1RiseSlaveDelay1R {
        PhyRddqsDq1RiseSlaveDelay1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ0 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq0_fall_slave_delay_1(
        &mut self,
    ) -> PhyRddqsDq0FallSlaveDelay1W<DdrDenaliPhy197Spec> {
        PhyRddqsDq0FallSlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ1 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq1_rise_slave_delay_1(
        &mut self,
    ) -> PhyRddqsDq1RiseSlaveDelay1W<DdrDenaliPhy197Spec> {
        PhyRddqsDq1RiseSlaveDelay1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_197::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_197::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy197Spec;
impl crate::RegisterSpec for DdrDenaliPhy197Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_197::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy197Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_197::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy197Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_197 to value 0"]
impl crate::Resettable for DdrDenaliPhy197Spec {
    const RESET_VALUE: u32 = 0;
}
