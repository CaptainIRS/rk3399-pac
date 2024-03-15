#[doc = "Register `DENALI_PHY_190` reader"]
pub type R = crate::R<DenaliPhy190Spec>;
#[doc = "Register `DENALI_PHY_190` writer"]
pub type W = crate::W<DenaliPhy190Spec>;
#[doc = "Field `PHY_CLK_WRDQ6_SLAVE_DELAY_1` reader - Write clock slave delay setting for DQ6 for slice 1."]
pub type PhyClkWrdq6SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ6_SLAVE_DELAY_1` writer - Write clock slave delay setting for DQ6 for slice 1."]
pub type PhyClkWrdq6SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CLK_WRDQ7_SLAVE_DELAY_1` reader - Write clock slave delay setting for DQ7 for slice 1."]
pub type PhyClkWrdq7SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ7_SLAVE_DELAY_1` writer - Write clock slave delay setting for DQ7 for slice 1."]
pub type PhyClkWrdq7SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ6 for slice 1."]
    #[inline(always)]
    pub fn phy_clk_wrdq6_slave_delay_1(&self) -> PhyClkWrdq6SlaveDelay1R {
        PhyClkWrdq6SlaveDelay1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ7 for slice 1."]
    #[inline(always)]
    pub fn phy_clk_wrdq7_slave_delay_1(&self) -> PhyClkWrdq7SlaveDelay1R {
        PhyClkWrdq7SlaveDelay1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ6 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq6_slave_delay_1(&mut self) -> PhyClkWrdq6SlaveDelay1W<DenaliPhy190Spec> {
        PhyClkWrdq6SlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ7 for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq7_slave_delay_1(&mut self) -> PhyClkWrdq7SlaveDelay1W<DenaliPhy190Spec> {
        PhyClkWrdq7SlaveDelay1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_190::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_190::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy190Spec;
impl crate::RegisterSpec for DenaliPhy190Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_190::R`](R) reader structure"]
impl crate::Readable for DenaliPhy190Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_190::W`](W) writer structure"]
impl crate::Writable for DenaliPhy190Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_190 to value 0"]
impl crate::Resettable for DenaliPhy190Spec {
    const RESET_VALUE: u32 = 0;
}
