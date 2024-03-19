#[doc = "Register `DDR_DENALI_PHY_328` reader"]
pub type R = crate::R<DdrDenaliPhy328Spec>;
#[doc = "Register `DDR_DENALI_PHY_328` writer"]
pub type W = crate::W<DdrDenaliPhy328Spec>;
#[doc = "Field `PHY_RDDQS_DQ3_FALL_SLAVE_DELAY_2` reader - Falling edge read DQS slave delay setting for DQ3 for slice 2."]
pub type PhyRddqsDq3FallSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ3_FALL_SLAVE_DELAY_2` writer - Falling edge read DQS slave delay setting for DQ3 for slice 2."]
pub type PhyRddqsDq3FallSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_DQ4_RISE_SLAVE_DELAY_2` reader - Rising edge read DQS slave delay setting for DQ4 for slice 2."]
pub type PhyRddqsDq4RiseSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ4_RISE_SLAVE_DELAY_2` writer - Rising edge read DQS slave delay setting for DQ4 for slice 2."]
pub type PhyRddqsDq4RiseSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ3 for slice 2."]
    #[inline(always)]
    pub fn phy_rddqs_dq3_fall_slave_delay_2(&self) -> PhyRddqsDq3FallSlaveDelay2R {
        PhyRddqsDq3FallSlaveDelay2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ4 for slice 2."]
    #[inline(always)]
    pub fn phy_rddqs_dq4_rise_slave_delay_2(&self) -> PhyRddqsDq4RiseSlaveDelay2R {
        PhyRddqsDq4RiseSlaveDelay2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Falling edge read DQS slave delay setting for DQ3 for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq3_fall_slave_delay_2(
        &mut self,
    ) -> PhyRddqsDq3FallSlaveDelay2W<DdrDenaliPhy328Spec> {
        PhyRddqsDq3FallSlaveDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ4 for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq4_rise_slave_delay_2(
        &mut self,
    ) -> PhyRddqsDq4RiseSlaveDelay2W<DdrDenaliPhy328Spec> {
        PhyRddqsDq4RiseSlaveDelay2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_328::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_328::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy328Spec;
impl crate::RegisterSpec for DdrDenaliPhy328Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_328::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy328Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_328::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy328Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_328 to value 0"]
impl crate::Resettable for DdrDenaliPhy328Spec {
    const RESET_VALUE: u32 = 0;
}
