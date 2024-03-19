#[doc = "Register `DDR_DENALI_PHY_196` reader"]
pub type R = crate::R<DdrDenaliPhy196Spec>;
#[doc = "Register `DDR_DENALI_PHY_196` writer"]
pub type W = crate::W<DdrDenaliPhy196Spec>;
#[doc = "Field `PHY_RDDM_SLAVE_DELAY_1` reader - Read DM/DBI slave delay setting for slice 1. May be used for data swap."]
pub type PhyRddmSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDM_SLAVE_DELAY_1` writer - Read DM/DBI slave delay setting for slice 1. May be used for data swap."]
pub type PhyRddmSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_DQ0_RISE_SLAVE_DELAY_1` reader - Rising edge read DQS slave delay setting for DQ0 for slice 1."]
pub type PhyRddqsDq0RiseSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ0_RISE_SLAVE_DELAY_1` writer - Rising edge read DQS slave delay setting for DQ0 for slice 1."]
pub type PhyRddqsDq0RiseSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Read DM/DBI slave delay setting for slice 1. May be used for data swap."]
    #[inline(always)]
    pub fn phy_rddm_slave_delay_1(&self) -> PhyRddmSlaveDelay1R {
        PhyRddmSlaveDelay1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ0 for slice 1."]
    #[inline(always)]
    pub fn phy_rddqs_dq0_rise_slave_delay_1(&self) -> PhyRddqsDq0RiseSlaveDelay1R {
        PhyRddqsDq0RiseSlaveDelay1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DM/DBI slave delay setting for slice 1. May be used for data swap."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddm_slave_delay_1(&mut self) -> PhyRddmSlaveDelay1W<DdrDenaliPhy196Spec> {
        PhyRddmSlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ0 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq0_rise_slave_delay_1(
        &mut self,
    ) -> PhyRddqsDq0RiseSlaveDelay1W<DdrDenaliPhy196Spec> {
        PhyRddqsDq0RiseSlaveDelay1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_196::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_196::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy196Spec;
impl crate::RegisterSpec for DdrDenaliPhy196Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_196::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy196Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_196::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy196Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_196 to value 0"]
impl crate::Resettable for DdrDenaliPhy196Spec {
    const RESET_VALUE: u32 = 0;
}
