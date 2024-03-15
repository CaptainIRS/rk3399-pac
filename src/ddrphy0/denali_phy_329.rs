#[doc = "Register `DENALI_PHY_329` reader"]
pub type R = crate::R<DenaliPhy329Spec>;
#[doc = "Register `DENALI_PHY_329` writer"]
pub type W = crate::W<DenaliPhy329Spec>;
#[doc = "Field `PHY_RDDQS_DQ4_FALL_SLAVE_DELAY_2` reader - Falling edge read DQS slave delay setting for DQ4 for slice 2."]
pub type PhyRddqsDq4FallSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ4_FALL_SLAVE_DELAY_2` writer - Falling edge read DQS slave delay setting for DQ4 for slice 2."]
pub type PhyRddqsDq4FallSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_DQ5_RISE_SLAVE_DELAY_2` reader - Rising edge read DQS slave delay setting for DQ5 for slice 2."]
pub type PhyRddqsDq5RiseSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ5_RISE_SLAVE_DELAY_2` writer - Rising edge read DQS slave delay setting for DQ5 for slice 2."]
pub type PhyRddqsDq5RiseSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ4 for slice 2."]
    #[inline(always)]
    pub fn phy_rddqs_dq4_fall_slave_delay_2(&self) -> PhyRddqsDq4FallSlaveDelay2R {
        PhyRddqsDq4FallSlaveDelay2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ5 for slice 2."]
    #[inline(always)]
    pub fn phy_rddqs_dq5_rise_slave_delay_2(&self) -> PhyRddqsDq5RiseSlaveDelay2R {
        PhyRddqsDq5RiseSlaveDelay2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ4 for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq4_fall_slave_delay_2(
        &mut self,
    ) -> PhyRddqsDq4FallSlaveDelay2W<DenaliPhy329Spec> {
        PhyRddqsDq4FallSlaveDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ5 for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq5_rise_slave_delay_2(
        &mut self,
    ) -> PhyRddqsDq5RiseSlaveDelay2W<DenaliPhy329Spec> {
        PhyRddqsDq5RiseSlaveDelay2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_329::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_329::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy329Spec;
impl crate::RegisterSpec for DenaliPhy329Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_329::R`](R) reader structure"]
impl crate::Readable for DenaliPhy329Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_329::W`](W) writer structure"]
impl crate::Writable for DenaliPhy329Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_329 to value 0"]
impl crate::Resettable for DenaliPhy329Spec {
    const RESET_VALUE: u32 = 0;
}
