#[doc = "Register `DDR_DENALI_PHY_916` reader"]
pub type R = crate::R<DdrDenaliPhy916Spec>;
#[doc = "Register `DDR_DENALI_PHY_916` writer"]
pub type W = crate::W<DdrDenaliPhy916Spec>;
#[doc = "Field `PHY_CSLVL_DLY_STEP` reader - Sets the delay step size plus 1 during CS training."]
pub type PhyCslvlDlyStepR = crate::FieldReader;
#[doc = "Field `PHY_CSLVL_DLY_STEP` writer - Sets the delay step size plus 1 during CS training."]
pub type PhyCslvlDlyStepW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_GRP_SLAVE_DELAY_0` reader - Address/control group slice 0 slave delay setting."]
pub type PhyGrpSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GRP_SLAVE_DELAY_0` writer - Address/control group slice 0 slave delay setting."]
pub type PhyGrpSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:3 - Sets the delay step size plus 1 during CS training."]
    #[inline(always)]
    pub fn phy_cslvl_dly_step(&self) -> PhyCslvlDlyStepR {
        PhyCslvlDlyStepR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:18 - Address/control group slice 0 slave delay setting."]
    #[inline(always)]
    pub fn phy_grp_slave_delay_0(&self) -> PhyGrpSlaveDelay0R {
        PhyGrpSlaveDelay0R::new(((self.bits >> 8) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sets the delay step size plus 1 during CS training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_dly_step(&mut self) -> PhyCslvlDlyStepW<DdrDenaliPhy916Spec> {
        PhyCslvlDlyStepW::new(self, 0)
    }
    #[doc = "Bits 8:18 - Address/control group slice 0 slave delay setting."]
    #[inline(always)]
    #[must_use]
    pub fn phy_grp_slave_delay_0(&mut self) -> PhyGrpSlaveDelay0W<DdrDenaliPhy916Spec> {
        PhyGrpSlaveDelay0W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_916::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_916::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy916Spec;
impl crate::RegisterSpec for DdrDenaliPhy916Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_916::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy916Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_916::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy916Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_916 to value 0"]
impl crate::Resettable for DdrDenaliPhy916Spec {
    const RESET_VALUE: u32 = 0;
}
