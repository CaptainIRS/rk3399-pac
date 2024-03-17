#[doc = "Register `DENALI_PHY_319` reader"]
pub type R = crate::R<DenaliPhy319Spec>;
#[doc = "Register `DENALI_PHY_319` writer"]
pub type W = crate::W<DenaliPhy319Spec>;
#[doc = "Field `PHY_CLK_WRDM_SLAVE_DELAY_2` reader - Write clock slave delay setting for DM for slice 2."]
pub type PhyClkWrdmSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDM_SLAVE_DELAY_2` writer - Write clock slave delay setting for DM for slice 2."]
pub type PhyClkWrdmSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CLK_WRDQS_SLAVE_DELAY_2` reader - Write clock slave delay setting for DQS for slice 2."]
pub type PhyClkWrdqsSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQS_SLAVE_DELAY_2` writer - Write clock slave delay setting for DQS for slice 2."]
pub type PhyClkWrdqsSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DM for slice 2."]
    #[inline(always)]
    pub fn phy_clk_wrdm_slave_delay_2(&self) -> PhyClkWrdmSlaveDelay2R {
        PhyClkWrdmSlaveDelay2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:25 - Write clock slave delay setting for DQS for slice 2."]
    #[inline(always)]
    pub fn phy_clk_wrdqs_slave_delay_2(&self) -> PhyClkWrdqsSlaveDelay2R {
        PhyClkWrdqsSlaveDelay2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DM for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdm_slave_delay_2(&mut self) -> PhyClkWrdmSlaveDelay2W<DenaliPhy319Spec> {
        PhyClkWrdmSlaveDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Write clock slave delay setting for DQS for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdqs_slave_delay_2(&mut self) -> PhyClkWrdqsSlaveDelay2W<DenaliPhy319Spec> {
        PhyClkWrdqsSlaveDelay2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_319::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_319::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy319Spec;
impl crate::RegisterSpec for DenaliPhy319Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_319::R`](R) reader structure"]
impl crate::Readable for DenaliPhy319Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_319::W`](W) writer structure"]
impl crate::Writable for DenaliPhy319Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_319 to value 0"]
impl crate::Resettable for DenaliPhy319Spec {
    const RESET_VALUE: u32 = 0;
}
