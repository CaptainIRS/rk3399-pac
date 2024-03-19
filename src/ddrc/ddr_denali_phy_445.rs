#[doc = "Register `DDR_DENALI_PHY_445` reader"]
pub type R = crate::R<DdrDenaliPhy445Spec>;
#[doc = "Register `DDR_DENALI_PHY_445` writer"]
pub type W = crate::W<DdrDenaliPhy445Spec>;
#[doc = "Field `PHY_CLK_WRDQ4_SLAVE_DELAY_3` reader - Write clock slave delay setting for DQ4 for slice 3."]
pub type PhyClkWrdq4SlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ4_SLAVE_DELAY_3` writer - Write clock slave delay setting for DQ4 for slice 3."]
pub type PhyClkWrdq4SlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CLK_WRDQ5_SLAVE_DELAY_3` reader - Write clock slave delay setting for DQ5 for slice 3."]
pub type PhyClkWrdq5SlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ5_SLAVE_DELAY_3` writer - Write clock slave delay setting for DQ5 for slice 3."]
pub type PhyClkWrdq5SlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ4 for slice 3."]
    #[inline(always)]
    pub fn phy_clk_wrdq4_slave_delay_3(&self) -> PhyClkWrdq4SlaveDelay3R {
        PhyClkWrdq4SlaveDelay3R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ5 for slice 3."]
    #[inline(always)]
    pub fn phy_clk_wrdq5_slave_delay_3(&self) -> PhyClkWrdq5SlaveDelay3R {
        PhyClkWrdq5SlaveDelay3R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ4 for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq4_slave_delay_3(&mut self) -> PhyClkWrdq4SlaveDelay3W<DdrDenaliPhy445Spec> {
        PhyClkWrdq4SlaveDelay3W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ5 for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq5_slave_delay_3(&mut self) -> PhyClkWrdq5SlaveDelay3W<DdrDenaliPhy445Spec> {
        PhyClkWrdq5SlaveDelay3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_445::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_445::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy445Spec;
impl crate::RegisterSpec for DdrDenaliPhy445Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_445::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy445Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_445::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy445Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_445 to value 0"]
impl crate::Resettable for DdrDenaliPhy445Spec {
    const RESET_VALUE: u32 = 0;
}
