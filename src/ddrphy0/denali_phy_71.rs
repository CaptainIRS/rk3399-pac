#[doc = "Register `DENALI_PHY_71` reader"]
pub type R = crate::R<DenaliPhy71Spec>;
#[doc = "Register `DENALI_PHY_71` writer"]
pub type W = crate::W<DenaliPhy71Spec>;
#[doc = "Field `PHY_RDDQS_DQ2_FALL_SLAVE_DELAY_0` reader - Falling edge read DQS slave delay setting for DQ2 for slice 0."]
pub type PhyRddqsDq2FallSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ2_FALL_SLAVE_DELAY_0` writer - Falling edge read DQS slave delay setting for DQ2 for slice 0."]
pub type PhyRddqsDq2FallSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_DQ3_RISE_SLAVE_DELAY_0` reader - Rising edge read DQS slave delay setting for DQ3 for slice 0."]
pub type PhyRddqsDq3RiseSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ3_RISE_SLAVE_DELAY_0` writer - Rising edge read DQS slave delay setting for DQ3 for slice 0."]
pub type PhyRddqsDq3RiseSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ2 for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_dq2_fall_slave_delay_0(&self) -> PhyRddqsDq2FallSlaveDelay0R {
        PhyRddqsDq2FallSlaveDelay0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ3 for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_dq3_rise_slave_delay_0(&self) -> PhyRddqsDq3RiseSlaveDelay0R {
        PhyRddqsDq3RiseSlaveDelay0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ2 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq2_fall_slave_delay_0(
        &mut self,
    ) -> PhyRddqsDq2FallSlaveDelay0W<DenaliPhy71Spec> {
        PhyRddqsDq2FallSlaveDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ3 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq3_rise_slave_delay_0(
        &mut self,
    ) -> PhyRddqsDq3RiseSlaveDelay0W<DenaliPhy71Spec> {
        PhyRddqsDq3RiseSlaveDelay0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_71::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_71::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy71Spec;
impl crate::RegisterSpec for DenaliPhy71Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_71::R`](R) reader structure"]
impl crate::Readable for DenaliPhy71Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_71::W`](W) writer structure"]
impl crate::Writable for DenaliPhy71Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_71 to value 0"]
impl crate::Resettable for DenaliPhy71Spec {
    const RESET_VALUE: u32 = 0;
}
