#[doc = "Register `DENALI_PHY_324` reader"]
pub type R = crate::R<DenaliPhy324Spec>;
#[doc = "Register `DENALI_PHY_324` writer"]
pub type W = crate::W<DenaliPhy324Spec>;
#[doc = "Field `PHY_RDDM_SLAVE_DELAY_2` reader - Read DM/DBI slave delay setting for slice 2. May be used for data swap."]
pub type PhyRddmSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDM_SLAVE_DELAY_2` writer - Read DM/DBI slave delay setting for slice 2. May be used for data swap."]
pub type PhyRddmSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQS_DQ0_RISE_SLAVE_DELAY_2` reader - Rising edge read DQS slave delay setting for DQ0 for slice 2."]
pub type PhyRddqsDq0RiseSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQS_DQ0_RISE_SLAVE_DELAY_2` writer - Rising edge read DQS slave delay setting for DQ0 for slice 2."]
pub type PhyRddqsDq0RiseSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Read DM/DBI slave delay setting for slice 2. May be used for data swap."]
    #[inline(always)]
    pub fn phy_rddm_slave_delay_2(&self) -> PhyRddmSlaveDelay2R {
        PhyRddmSlaveDelay2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ0 for slice 2."]
    #[inline(always)]
    pub fn phy_rddqs_dq0_rise_slave_delay_2(&self) -> PhyRddqsDq0RiseSlaveDelay2R {
        PhyRddqsDq0RiseSlaveDelay2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DM/DBI slave delay setting for slice 2. May be used for data swap."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddm_slave_delay_2(&mut self) -> PhyRddmSlaveDelay2W<DenaliPhy324Spec> {
        PhyRddmSlaveDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Rising edge read DQS slave delay setting for DQ0 for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddqs_dq0_rise_slave_delay_2(
        &mut self,
    ) -> PhyRddqsDq0RiseSlaveDelay2W<DenaliPhy324Spec> {
        PhyRddqsDq0RiseSlaveDelay2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_324::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_324::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy324Spec;
impl crate::RegisterSpec for DenaliPhy324Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_324::R`](R) reader structure"]
impl crate::Readable for DenaliPhy324Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_324::W`](W) writer structure"]
impl crate::Writable for DenaliPhy324Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_324 to value 0"]
impl crate::Resettable for DenaliPhy324Spec {
    const RESET_VALUE: u32 = 0;
}
