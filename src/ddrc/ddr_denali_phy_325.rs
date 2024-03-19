#[doc = "Register `DDR_DENALI_PHY_325` reader"]
pub type R = crate::R<DdrDenaliPhy325Spec>;
#[doc = "Register `DDR_DENALI_PHY_325` writer"]
pub type W = crate::W<DdrDenaliPhy325Spec>;
#[doc = "Field `PHY_RDDQS_DQ0_FALL_SLAVE_DELAY_2` reader - Falling edge read DQS slave delay setting for DQ0 for slice 2."]
pub type PhyRddqsDq0FallSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ0_FALL_SLAVE_DELAY_2` writer - Falling edge read DQS slave delay setting for DQ0 for slice 2."]
pub type PhyRddqsDq0FallSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_DQ1_RISE_SLAVE_DELAY_2` reader - Rising edge read DQS slave delay setting for DQ1 for slice 2."]
pub type PhyRddqsDq1RiseSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ1_RISE_SLAVE_DELAY_2` writer - Rising edge read DQS slave delay setting for DQ1 for slice 2."]
pub type PhyRddqsDq1RiseSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ0 for slice 2."]
    #[inline(always)]
    pub fn phy_rddqs_dq0_fall_slave_delay_2(&self) -> PhyRddqsDq0FallSlaveDelay2R {
        PhyRddqsDq0FallSlaveDelay2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ1 for slice 2."]
    #[inline(always)]
    pub fn phy_rddqs_dq1_rise_slave_delay_2(&self) -> PhyRddqsDq1RiseSlaveDelay2R {
        PhyRddqsDq1RiseSlaveDelay2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ0 for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq0_fall_slave_delay_2(
        &mut self,
    ) -> PhyRddqsDq0FallSlaveDelay2W<DdrDenaliPhy325Spec> {
        PhyRddqsDq0FallSlaveDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ1 for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq1_rise_slave_delay_2(
        &mut self,
    ) -> PhyRddqsDq1RiseSlaveDelay2W<DdrDenaliPhy325Spec> {
        PhyRddqsDq1RiseSlaveDelay2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_325::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_325::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy325Spec;
impl crate::RegisterSpec for DdrDenaliPhy325Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_325::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy325Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_325::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy325Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_325 to value 0"]
impl crate::Resettable for DdrDenaliPhy325Spec {
    const RESET_VALUE: u32 = 0;
}
