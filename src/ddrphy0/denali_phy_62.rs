#[doc = "Register `DENALI_PHY_62` reader"]
pub type R = crate::R<DenaliPhy62Spec>;
#[doc = "Register `DENALI_PHY_62` writer"]
pub type W = crate::W<DenaliPhy62Spec>;
#[doc = "Field `PHY_CLK_WRDQ6_SLAVE_DELAY_0` reader - Write clock slave delay setting for DQ6 for slice 0."]
pub type PhyClkWrdq6SlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ6_SLAVE_DELAY_0` writer - Write clock slave delay setting for DQ6 for slice 0."]
pub type PhyClkWrdq6SlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CLK_WRDQ7_SLAVE_DELAY_0` reader - Write clock slave delay setting for DQ7 for slice 0."]
pub type PhyClkWrdq7SlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQ7_SLAVE_DELAY_0` writer - Write clock slave delay setting for DQ7 for slice 0."]
pub type PhyClkWrdq7SlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ6 for slice 0."]
    #[inline(always)]
    pub fn phy_clk_wrdq6_slave_delay_0(&self) -> PhyClkWrdq6SlaveDelay0R {
        PhyClkWrdq6SlaveDelay0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ7 for slice 0."]
    #[inline(always)]
    pub fn phy_clk_wrdq7_slave_delay_0(&self) -> PhyClkWrdq7SlaveDelay0R {
        PhyClkWrdq7SlaveDelay0R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DQ6 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq6_slave_delay_0(&mut self) -> PhyClkWrdq6SlaveDelay0W<DenaliPhy62Spec> {
        PhyClkWrdq6SlaveDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Write clock slave delay setting for DQ7 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdq7_slave_delay_0(&mut self) -> PhyClkWrdq7SlaveDelay0W<DenaliPhy62Spec> {
        PhyClkWrdq7SlaveDelay0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_62::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_62::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy62Spec;
impl crate::RegisterSpec for DenaliPhy62Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_62::R`](R) reader structure"]
impl crate::Readable for DenaliPhy62Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_62::W`](W) writer structure"]
impl crate::Writable for DenaliPhy62Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_62 to value 0"]
impl crate::Resettable for DenaliPhy62Spec {
    const RESET_VALUE: u32 = 0;
}
