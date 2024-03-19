#[doc = "Register `DDR_DENALI_PHY_443` reader"]
pub type R = crate::R<DdrDenaliPhy443Spec>;
#[doc = "Register `DDR_DENALI_PHY_443` writer"]
pub type W = crate::W<DdrDenaliPhy443Spec>;
#[doc = "Field `PHY_CLK_WRDQ0_SLAVE_DELAY_3` reader - Write clock slave delay setting for DQ0 for slice 3."]
pub type PhyClkWrdq0SlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ0_SLAVE_DELAY_3` writer - Write clock slave delay setting for DQ0 for slice 3."]
pub type PhyClkWrdq0SlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CLK_WRDQ1_SLAVE_DELAY_3` reader - Write clock slave delay setting for DQ1 for slice 3."]
pub type PhyClkWrdq1SlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ1_SLAVE_DELAY_3` writer - Write clock slave delay setting for DQ1 for slice 3."]
pub type PhyClkWrdq1SlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ0 for slice 3."]
    #[inline(always)]
    pub fn phy_clk_wrdq0_slave_delay_3(&self) -> PhyClkWrdq0SlaveDelay3R {
        PhyClkWrdq0SlaveDelay3R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ1 for slice 3."]
    #[inline(always)]
    pub fn phy_clk_wrdq1_slave_delay_3(&self) -> PhyClkWrdq1SlaveDelay3R {
        PhyClkWrdq1SlaveDelay3R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ0 for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq0_slave_delay_3(&mut self) -> PhyClkWrdq0SlaveDelay3W<DdrDenaliPhy443Spec> {
        PhyClkWrdq0SlaveDelay3W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ1 for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq1_slave_delay_3(&mut self) -> PhyClkWrdq1SlaveDelay3W<DdrDenaliPhy443Spec> {
        PhyClkWrdq1SlaveDelay3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_443::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_443::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy443Spec;
impl crate::RegisterSpec for DdrDenaliPhy443Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_443::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy443Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_443::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy443Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_443 to value 0"]
impl crate::Resettable for DdrDenaliPhy443Spec {
    const RESET_VALUE: u32 = 0;
}
