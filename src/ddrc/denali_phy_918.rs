#[doc = "Register `DENALI_PHY_918` reader"]
pub type R = crate::R<DenaliPhy918Spec>;
#[doc = "Register `DENALI_PHY_918` writer"]
pub type W = crate::W<DenaliPhy918Spec>;
#[doc = "Field `PHY_GRP_SLAVE_DELAY_3` reader - Address/control group slice 3 slave delay setting."]
pub type PhyGrpSlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GRP_SLAVE_DELAY_3` writer - Address/control group slice 3 slave delay setting."]
pub type PhyGrpSlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Address/control group slice 3 slave delay setting."]
    #[inline(always)]
    pub fn phy_grp_slave_delay_3(&self) -> PhyGrpSlaveDelay3R {
        PhyGrpSlaveDelay3R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Address/control group slice 3 slave delay setting."]
    #[inline(always)]
    #[must_use]
    pub fn phy_grp_slave_delay_3(&mut self) -> PhyGrpSlaveDelay3W<DenaliPhy918Spec> {
        PhyGrpSlaveDelay3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_918::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_918::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy918Spec;
impl crate::RegisterSpec for DenaliPhy918Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_918::R`](R) reader structure"]
impl crate::Readable for DenaliPhy918Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_918::W`](W) writer structure"]
impl crate::Writable for DenaliPhy918Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_918 to value 0"]
impl crate::Resettable for DenaliPhy918Spec {
    const RESET_VALUE: u32 = 0;
}
