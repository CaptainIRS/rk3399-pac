#[doc = "Register `DDR_DENALI_PHY_452` reader"]
pub type R = crate::R<DdrDenaliPhy452Spec>;
#[doc = "Register `DDR_DENALI_PHY_452` writer"]
pub type W = crate::W<DdrDenaliPhy452Spec>;
#[doc = "Field `PHY_RDDM_SLAVE_DELAY_3` reader - Read DM/DBI slave delay setting for slice 3. May be used for data swap."]
pub type PhyRddmSlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDM_SLAVE_DELAY_3` writer - Read DM/DBI slave delay setting for slice 3. May be used for data swap."]
pub type PhyRddmSlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_DQ0_RISE_SLAVE_DELAY_3` reader - Rising edge read DQS slave delay setting for DQ0 for slice 3."]
pub type PhyRddqsDq0RiseSlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ0_RISE_SLAVE_DELAY_3` writer - Rising edge read DQS slave delay setting for DQ0 for slice 3."]
pub type PhyRddqsDq0RiseSlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Read DM/DBI slave delay setting for slice 3. May be used for data swap."]
    #[inline(always)]
    pub fn phy_rddm_slave_delay_3(&self) -> PhyRddmSlaveDelay3R {
        PhyRddmSlaveDelay3R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ0 for slice 3."]
    #[inline(always)]
    pub fn phy_rddqs_dq0_rise_slave_delay_3(&self) -> PhyRddqsDq0RiseSlaveDelay3R {
        PhyRddqsDq0RiseSlaveDelay3R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DM/DBI slave delay setting for slice 3. May be used for data swap."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddm_slave_delay_3(&mut self) -> PhyRddmSlaveDelay3W<DdrDenaliPhy452Spec> {
        PhyRddmSlaveDelay3W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ0 for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq0_rise_slave_delay_3(
        &mut self,
    ) -> PhyRddqsDq0RiseSlaveDelay3W<DdrDenaliPhy452Spec> {
        PhyRddqsDq0RiseSlaveDelay3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_452::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_452::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy452Spec;
impl crate::RegisterSpec for DdrDenaliPhy452Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_452::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy452Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_452::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy452Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_452 to value 0"]
impl crate::Resettable for DdrDenaliPhy452Spec {
    const RESET_VALUE: u32 = 0;
}
