#[doc = "Register `DENALI_PHY_73` reader"]
pub type R = crate::R<DenaliPhy73Spec>;
#[doc = "Register `DENALI_PHY_73` writer"]
pub type W = crate::W<DenaliPhy73Spec>;
#[doc = "Field `PHY_RDDQS_DQ4_FALL_SLAVE_DELAY_0` reader - Falling edge read DQS slave delay setting for DQ4 for slice 0."]
pub type PhyRddqsDq4FallSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ4_FALL_SLAVE_DELAY_0` writer - Falling edge read DQS slave delay setting for DQ4 for slice 0."]
pub type PhyRddqsDq4FallSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_DQ5_RISE_SLAVE_DELAY_0` reader - Rising edge read DQS slave delay setting for DQ5 for slice 0."]
pub type PhyRddqsDq5RiseSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ5_RISE_SLAVE_DELAY_0` writer - Rising edge read DQS slave delay setting for DQ5 for slice 0."]
pub type PhyRddqsDq5RiseSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ4 for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_dq4_fall_slave_delay_0(&self) -> PhyRddqsDq4FallSlaveDelay0R {
        PhyRddqsDq4FallSlaveDelay0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ5 for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_dq5_rise_slave_delay_0(&self) -> PhyRddqsDq5RiseSlaveDelay0R {
        PhyRddqsDq5RiseSlaveDelay0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ4 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq4_fall_slave_delay_0(
        &mut self,
    ) -> PhyRddqsDq4FallSlaveDelay0W<DenaliPhy73Spec> {
        PhyRddqsDq4FallSlaveDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ5 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq5_rise_slave_delay_0(
        &mut self,
    ) -> PhyRddqsDq5RiseSlaveDelay0W<DenaliPhy73Spec> {
        PhyRddqsDq5RiseSlaveDelay0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_73::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_73::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy73Spec;
impl crate::RegisterSpec for DenaliPhy73Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_73::R`](R) reader structure"]
impl crate::Readable for DenaliPhy73Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_73::W`](W) writer structure"]
impl crate::Writable for DenaliPhy73Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_73 to value 0"]
impl crate::Resettable for DenaliPhy73Spec {
    const RESET_VALUE: u32 = 0;
}
