#[doc = "Register `DENALI_PHY_187` reader"]
pub type R = crate::R<DenaliPhy187Spec>;
#[doc = "Register `DENALI_PHY_187` writer"]
pub type W = crate::W<DenaliPhy187Spec>;
#[doc = "Field `PHY_CLK_WRDQ0_SLAVE_DELAY_1` reader - Write clock slave delay setting for DQ0 for slice 1."]
pub type PhyClkWrdq0SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ0_SLAVE_DELAY_1` writer - Write clock slave delay setting for DQ0 for slice 1."]
pub type PhyClkWrdq0SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CLK_WRDQ1_SLAVE_DELAY_1` reader - Write clock slave delay setting for DQ1 for slice 1."]
pub type PhyClkWrdq1SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ1_SLAVE_DELAY_1` writer - Write clock slave delay setting for DQ1 for slice 1."]
pub type PhyClkWrdq1SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ0 for slice 1."]
    #[inline(always)]
    pub fn phy_clk_wrdq0_slave_delay_1(&self) -> PhyClkWrdq0SlaveDelay1R {
        PhyClkWrdq0SlaveDelay1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ1 for slice 1."]
    #[inline(always)]
    pub fn phy_clk_wrdq1_slave_delay_1(&self) -> PhyClkWrdq1SlaveDelay1R {
        PhyClkWrdq1SlaveDelay1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ0 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq0_slave_delay_1(&mut self) -> PhyClkWrdq0SlaveDelay1W<DenaliPhy187Spec> {
        PhyClkWrdq0SlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ1 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq1_slave_delay_1(&mut self) -> PhyClkWrdq1SlaveDelay1W<DenaliPhy187Spec> {
        PhyClkWrdq1SlaveDelay1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_187::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_187::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy187Spec;
impl crate::RegisterSpec for DenaliPhy187Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_187::R`](R) reader structure"]
impl crate::Readable for DenaliPhy187Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_187::W`](W) writer structure"]
impl crate::Writable for DenaliPhy187Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_187 to value 0"]
impl crate::Resettable for DenaliPhy187Spec {
    const RESET_VALUE: u32 = 0;
}
