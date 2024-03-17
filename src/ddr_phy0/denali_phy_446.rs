#[doc = "Register `DENALI_PHY_446` reader"]
pub type R = crate::R<DenaliPhy446Spec>;
#[doc = "Register `DENALI_PHY_446` writer"]
pub type W = crate::W<DenaliPhy446Spec>;
#[doc = "Field `PHY_CLK_WRDQ6_SLAVE_DELAY_3` reader - Write clock slave delay setting for DQ6 for slice 3."]
pub type PhyClkWrdq6SlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ6_SLAVE_DELAY_3` writer - Write clock slave delay setting for DQ6 for slice 3."]
pub type PhyClkWrdq6SlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CLK_WRDQ7_SLAVE_DELAY_3` reader - Write clock slave delay setting for DQ7 for slice 3."]
pub type PhyClkWrdq7SlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ7_SLAVE_DELAY_3` writer - Write clock slave delay setting for DQ7 for slice 3."]
pub type PhyClkWrdq7SlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ6 for slice 3."]
    #[inline(always)]
    pub fn phy_clk_wrdq6_slave_delay_3(&self) -> PhyClkWrdq6SlaveDelay3R {
        PhyClkWrdq6SlaveDelay3R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ7 for slice 3."]
    #[inline(always)]
    pub fn phy_clk_wrdq7_slave_delay_3(&self) -> PhyClkWrdq7SlaveDelay3R {
        PhyClkWrdq7SlaveDelay3R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ6 for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq6_slave_delay_3(&mut self) -> PhyClkWrdq6SlaveDelay3W<DenaliPhy446Spec> {
        PhyClkWrdq6SlaveDelay3W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ7 for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq7_slave_delay_3(&mut self) -> PhyClkWrdq7SlaveDelay3W<DenaliPhy446Spec> {
        PhyClkWrdq7SlaveDelay3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_446::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_446::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy446Spec;
impl crate::RegisterSpec for DenaliPhy446Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_446::R`](R) reader structure"]
impl crate::Readable for DenaliPhy446Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_446::W`](W) writer structure"]
impl crate::Writable for DenaliPhy446Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_446 to value 0"]
impl crate::Resettable for DenaliPhy446Spec {
    const RESET_VALUE: u32 = 0;
}
