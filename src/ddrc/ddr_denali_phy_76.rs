#[doc = "Register `DDR_DENALI_PHY_76` reader"]
pub type R = crate::R<DdrDenaliPhy76Spec>;
#[doc = "Register `DDR_DENALI_PHY_76` writer"]
pub type W = crate::W<DdrDenaliPhy76Spec>;
#[doc = "Field `PHY_RDDQS_DQ7_FALL_SLAVE_DELAY_0` reader - Falling edge read DQS slave delay setting for DQ7 for slice 0."]
pub type PhyRddqsDq7FallSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ7_FALL_SLAVE_DELAY_0` writer - Falling edge read DQS slave delay setting for DQ7 for slice 0."]
pub type PhyRddqsDq7FallSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_DM_RISE_SLAVE_DELAY_0` reader - Rising edge read DQS slave delay setting for DM for slice 0."]
pub type PhyRddqsDmRiseSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DM_RISE_SLAVE_DELAY_0` writer - Rising edge read DQS slave delay setting for DM for slice 0."]
pub type PhyRddqsDmRiseSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ7 for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_dq7_fall_slave_delay_0(&self) -> PhyRddqsDq7FallSlaveDelay0R {
        PhyRddqsDq7FallSlaveDelay0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DM for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_dm_rise_slave_delay_0(&self) -> PhyRddqsDmRiseSlaveDelay0R {
        PhyRddqsDmRiseSlaveDelay0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ7 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq7_fall_slave_delay_0(
        &mut self,
    ) -> PhyRddqsDq7FallSlaveDelay0W<DdrDenaliPhy76Spec> {
        PhyRddqsDq7FallSlaveDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DM for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dm_rise_slave_delay_0(
        &mut self,
    ) -> PhyRddqsDmRiseSlaveDelay0W<DdrDenaliPhy76Spec> {
        PhyRddqsDmRiseSlaveDelay0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_76::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_76::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy76Spec;
impl crate::RegisterSpec for DdrDenaliPhy76Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_76::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy76Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_76::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy76Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_76 to value 0"]
impl crate::Resettable for DdrDenaliPhy76Spec {
    const RESET_VALUE: u32 = 0;
}
