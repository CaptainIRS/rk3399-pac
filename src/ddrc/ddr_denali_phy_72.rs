#[doc = "Register `DDR_DENALI_PHY_72` reader"]
pub type R = crate::R<DdrDenaliPhy72Spec>;
#[doc = "Register `DDR_DENALI_PHY_72` writer"]
pub type W = crate::W<DdrDenaliPhy72Spec>;
#[doc = "Field `PHY_RDDQS_DQ3_FALL_SLAVE_DELAY_0` reader - Falling edge read DQS slave delay setting for DQ3 for slice 0."]
pub type PhyRddqsDq3FallSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ3_FALL_SLAVE_DELAY_0` writer - Falling edge read DQS slave delay setting for DQ3 for slice 0."]
pub type PhyRddqsDq3FallSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_DQ4_RISE_SLAVE_DELAY_0` reader - Rising edge read DQS slave delay setting for DQ4 for slice 0."]
pub type PhyRddqsDq4RiseSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ4_RISE_SLAVE_DELAY_0` writer - Rising edge read DQS slave delay setting for DQ4 for slice 0."]
pub type PhyRddqsDq4RiseSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ3 for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_dq3_fall_slave_delay_0(&self) -> PhyRddqsDq3FallSlaveDelay0R {
        PhyRddqsDq3FallSlaveDelay0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ4 for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_dq4_rise_slave_delay_0(&self) -> PhyRddqsDq4RiseSlaveDelay0R {
        PhyRddqsDq4RiseSlaveDelay0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ3 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq3_fall_slave_delay_0(
        &mut self,
    ) -> PhyRddqsDq3FallSlaveDelay0W<DdrDenaliPhy72Spec> {
        PhyRddqsDq3FallSlaveDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ4 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq4_rise_slave_delay_0(
        &mut self,
    ) -> PhyRddqsDq4RiseSlaveDelay0W<DdrDenaliPhy72Spec> {
        PhyRddqsDq4RiseSlaveDelay0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_72::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_72::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy72Spec;
impl crate::RegisterSpec for DdrDenaliPhy72Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_72::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy72Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_72::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy72Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_72 to value 0"]
impl crate::Resettable for DdrDenaliPhy72Spec {
    const RESET_VALUE: u32 = 0;
}
