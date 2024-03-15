#[doc = "Register `DENALI_PHY_188` reader"]
pub type R = crate::R<DenaliPhy188Spec>;
#[doc = "Register `DENALI_PHY_188` writer"]
pub type W = crate::W<DenaliPhy188Spec>;
#[doc = "Field `PHY_CLK_WRDQ2_SLAVE_DELAY_1` reader - Write clock slave delay setting for DQ2 for slice 1."]
pub type PhyClkWrdq2SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ2_SLAVE_DELAY_1` writer - Write clock slave delay setting for DQ2 for slice 1."]
pub type PhyClkWrdq2SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CLK_WRDQ3_SLAVE_DELAY_1` reader - Write clock slave delay setting for DQ3 for slice 1."]
pub type PhyClkWrdq3SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ3_SLAVE_DELAY_1` writer - Write clock slave delay setting for DQ3 for slice 1."]
pub type PhyClkWrdq3SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ2 for slice 1."]
    #[inline(always)]
    pub fn phy_clk_wrdq2_slave_delay_1(&self) -> PhyClkWrdq2SlaveDelay1R {
        PhyClkWrdq2SlaveDelay1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ3 for slice 1."]
    #[inline(always)]
    pub fn phy_clk_wrdq3_slave_delay_1(&self) -> PhyClkWrdq3SlaveDelay1R {
        PhyClkWrdq3SlaveDelay1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ2 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq2_slave_delay_1(&mut self) -> PhyClkWrdq2SlaveDelay1W<DenaliPhy188Spec> {
        PhyClkWrdq2SlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ3 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq3_slave_delay_1(&mut self) -> PhyClkWrdq3SlaveDelay1W<DenaliPhy188Spec> {
        PhyClkWrdq3SlaveDelay1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_188::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_188::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy188Spec;
impl crate::RegisterSpec for DenaliPhy188Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_188::R`](R) reader structure"]
impl crate::Readable for DenaliPhy188Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_188::W`](W) writer structure"]
impl crate::Writable for DenaliPhy188Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_188 to value 0"]
impl crate::Resettable for DenaliPhy188Spec {
    const RESET_VALUE: u32 = 0;
}
