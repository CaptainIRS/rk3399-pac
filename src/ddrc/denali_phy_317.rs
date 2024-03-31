#[doc = "Register `DENALI_PHY_317` reader"]
pub type R = crate::R<DenaliPhy317Spec>;
#[doc = "Register `DENALI_PHY_317` writer"]
pub type W = crate::W<DenaliPhy317Spec>;
#[doc = "Field `PHY_CLK_WRDQ4_SLAVE_DELAY_2` reader - Write clock slave delay setting for DQ4 for slice 2."]
pub type PhyClkWrdq4SlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ4_SLAVE_DELAY_2` writer - Write clock slave delay setting for DQ4 for slice 2."]
pub type PhyClkWrdq4SlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CLK_WRDQ5_SLAVE_DELAY_2` reader - Write clock slave delay setting for DQ5 for slice 2."]
pub type PhyClkWrdq5SlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ5_SLAVE_DELAY_2` writer - Write clock slave delay setting for DQ5 for slice 2."]
pub type PhyClkWrdq5SlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ4 for slice 2."]
    #[inline(always)]
    pub fn phy_clk_wrdq4_slave_delay_2(&self) -> PhyClkWrdq4SlaveDelay2R {
        PhyClkWrdq4SlaveDelay2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ5 for slice 2."]
    #[inline(always)]
    pub fn phy_clk_wrdq5_slave_delay_2(&self) -> PhyClkWrdq5SlaveDelay2R {
        PhyClkWrdq5SlaveDelay2R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ4 for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq4_slave_delay_2(&mut self) -> PhyClkWrdq4SlaveDelay2W<DenaliPhy317Spec> {
        PhyClkWrdq4SlaveDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ5 for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq5_slave_delay_2(&mut self) -> PhyClkWrdq5SlaveDelay2W<DenaliPhy317Spec> {
        PhyClkWrdq5SlaveDelay2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_317::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_317::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy317Spec;
impl crate::RegisterSpec for DenaliPhy317Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_317::R`](R) reader structure"]
impl crate::Readable for DenaliPhy317Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_317::W`](W) writer structure"]
impl crate::Writable for DenaliPhy317Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_317 to value 0"]
impl crate::Resettable for DenaliPhy317Spec {
    const RESET_VALUE: u32 = 0;
}
