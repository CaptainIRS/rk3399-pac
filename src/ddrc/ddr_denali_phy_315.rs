#[doc = "Register `DDR_DENALI_PHY_315` reader"]
pub type R = crate::R<DdrDenaliPhy315Spec>;
#[doc = "Register `DDR_DENALI_PHY_315` writer"]
pub type W = crate::W<DdrDenaliPhy315Spec>;
#[doc = "Field `PHY_CLK_WRDQ0_SLAVE_DELAY_2` reader - Write clock slave delay setting for DQ0 for slice 2."]
pub type PhyClkWrdq0SlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ0_SLAVE_DELAY_2` writer - Write clock slave delay setting for DQ0 for slice 2."]
pub type PhyClkWrdq0SlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CLK_WRDQ1_SLAVE_DELAY_2` reader - Write clock slave delay setting for DQ1 for slice 2."]
pub type PhyClkWrdq1SlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ1_SLAVE_DELAY_2` writer - Write clock slave delay setting for DQ1 for slice 2."]
pub type PhyClkWrdq1SlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ0 for slice 2."]
    #[inline(always)]
    pub fn phy_clk_wrdq0_slave_delay_2(&self) -> PhyClkWrdq0SlaveDelay2R {
        PhyClkWrdq0SlaveDelay2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ1 for slice 2."]
    #[inline(always)]
    pub fn phy_clk_wrdq1_slave_delay_2(&self) -> PhyClkWrdq1SlaveDelay2R {
        PhyClkWrdq1SlaveDelay2R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ0 for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq0_slave_delay_2(&mut self) -> PhyClkWrdq0SlaveDelay2W<DdrDenaliPhy315Spec> {
        PhyClkWrdq0SlaveDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ1 for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq1_slave_delay_2(&mut self) -> PhyClkWrdq1SlaveDelay2W<DdrDenaliPhy315Spec> {
        PhyClkWrdq1SlaveDelay2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_315::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_315::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy315Spec;
impl crate::RegisterSpec for DdrDenaliPhy315Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_315::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy315Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_315::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy315Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_315 to value 0"]
impl crate::Resettable for DdrDenaliPhy315Spec {
    const RESET_VALUE: u32 = 0;
}
