#[doc = "Register `DDR_DENALI_PHY_917` reader"]
pub type R = crate::R<DdrDenaliPhy917Spec>;
#[doc = "Register `DDR_DENALI_PHY_917` writer"]
pub type W = crate::W<DdrDenaliPhy917Spec>;
#[doc = "Field `PHY_GRP_SLAVE_DELAY_1` reader - Address/control group slice 1 slave delay setting."]
pub type PhyGrpSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GRP_SLAVE_DELAY_1` writer - Address/control group slice 1 slave delay setting."]
pub type PhyGrpSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_GRP_SLAVE_DELAY_2` reader - Address/control group slice 2 slave delay setting."]
pub type PhyGrpSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GRP_SLAVE_DELAY_2` writer - Address/control group slice 2 slave delay setting."]
pub type PhyGrpSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Address/control group slice 1 slave delay setting."]
    #[inline(always)]
    pub fn phy_grp_slave_delay_1(&self) -> PhyGrpSlaveDelay1R {
        PhyGrpSlaveDelay1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Address/control group slice 2 slave delay setting."]
    #[inline(always)]
    pub fn phy_grp_slave_delay_2(&self) -> PhyGrpSlaveDelay2R {
        PhyGrpSlaveDelay2R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Address/control group slice 1 slave delay setting."]
    #[inline(always)]
    #[must_use]
    pub fn phy_grp_slave_delay_1(&mut self) -> PhyGrpSlaveDelay1W<DdrDenaliPhy917Spec> {
        PhyGrpSlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Address/control group slice 2 slave delay setting."]
    #[inline(always)]
    #[must_use]
    pub fn phy_grp_slave_delay_2(&mut self) -> PhyGrpSlaveDelay2W<DdrDenaliPhy917Spec> {
        PhyGrpSlaveDelay2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_917::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_917::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy917Spec;
impl crate::RegisterSpec for DdrDenaliPhy917Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_917::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy917Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_917::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy917Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_917 to value 0"]
impl crate::Resettable for DdrDenaliPhy917Spec {
    const RESET_VALUE: u32 = 0;
}
