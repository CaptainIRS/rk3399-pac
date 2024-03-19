#[doc = "Register `DDR_DENALI_PHY_318` reader"]
pub type R = crate::R<DdrDenaliPhy318Spec>;
#[doc = "Register `DDR_DENALI_PHY_318` writer"]
pub type W = crate::W<DdrDenaliPhy318Spec>;
#[doc = "Field `PHY_CLK_WRDQ6_SLAVE_DELAY_2` reader - Write clock slave delay setting for DQ6 for slice 2."]
pub type PhyClkWrdq6SlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ6_SLAVE_DELAY_2` writer - Write clock slave delay setting for DQ6 for slice 2."]
pub type PhyClkWrdq6SlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CLK_WRDQ7_SLAVE_DELAY_2` reader - Write clock slave delay setting for DQ7 for slice 2."]
pub type PhyClkWrdq7SlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ7_SLAVE_DELAY_2` writer - Write clock slave delay setting for DQ7 for slice 2."]
pub type PhyClkWrdq7SlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ6 for slice 2."]
    #[inline(always)]
    pub fn phy_clk_wrdq6_slave_delay_2(&self) -> PhyClkWrdq6SlaveDelay2R {
        PhyClkWrdq6SlaveDelay2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ7 for slice 2."]
    #[inline(always)]
    pub fn phy_clk_wrdq7_slave_delay_2(&self) -> PhyClkWrdq7SlaveDelay2R {
        PhyClkWrdq7SlaveDelay2R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ6 for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq6_slave_delay_2(&mut self) -> PhyClkWrdq6SlaveDelay2W<DdrDenaliPhy318Spec> {
        PhyClkWrdq6SlaveDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ7 for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq7_slave_delay_2(&mut self) -> PhyClkWrdq7SlaveDelay2W<DdrDenaliPhy318Spec> {
        PhyClkWrdq7SlaveDelay2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_318::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_318::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy318Spec;
impl crate::RegisterSpec for DdrDenaliPhy318Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_318::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy318Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_318::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy318Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_318 to value 0"]
impl crate::Resettable for DdrDenaliPhy318Spec {
    const RESET_VALUE: u32 = 0;
}
