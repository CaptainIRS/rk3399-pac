#[doc = "Register `DDR_DENALI_PHY_60` reader"]
pub type R = crate::R<DdrDenaliPhy60Spec>;
#[doc = "Register `DDR_DENALI_PHY_60` writer"]
pub type W = crate::W<DdrDenaliPhy60Spec>;
#[doc = "Field `PHY_CLK_WRDQ2_SLAVE_DELAY_0` reader - Write clock slave delay setting for DQ2 for slice 0."]
pub type PhyClkWrdq2SlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ2_SLAVE_DELAY_0` writer - Write clock slave delay setting for DQ2 for slice 0."]
pub type PhyClkWrdq2SlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CLK_WRDQ3_SLAVE_DELAY_0` reader - Write clock slave delay setting for DQ3 for slice 0."]
pub type PhyClkWrdq3SlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ3_SLAVE_DELAY_0` writer - Write clock slave delay setting for DQ3 for slice 0."]
pub type PhyClkWrdq3SlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ2 for slice 0."]
    #[inline(always)]
    pub fn phy_clk_wrdq2_slave_delay_0(&self) -> PhyClkWrdq2SlaveDelay0R {
        PhyClkWrdq2SlaveDelay0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ3 for slice 0."]
    #[inline(always)]
    pub fn phy_clk_wrdq3_slave_delay_0(&self) -> PhyClkWrdq3SlaveDelay0R {
        PhyClkWrdq3SlaveDelay0R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ2 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq2_slave_delay_0(&mut self) -> PhyClkWrdq2SlaveDelay0W<DdrDenaliPhy60Spec> {
        PhyClkWrdq2SlaveDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ3 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq3_slave_delay_0(&mut self) -> PhyClkWrdq3SlaveDelay0W<DdrDenaliPhy60Spec> {
        PhyClkWrdq3SlaveDelay0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_60::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_60::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy60Spec;
impl crate::RegisterSpec for DdrDenaliPhy60Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_60::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy60Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_60::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy60Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_60 to value 0"]
impl crate::Resettable for DdrDenaliPhy60Spec {
    const RESET_VALUE: u32 = 0;
}
