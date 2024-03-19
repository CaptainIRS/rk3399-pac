#[doc = "Register `DDR_DENALI_PHY_447` reader"]
pub type R = crate::R<DdrDenaliPhy447Spec>;
#[doc = "Register `DDR_DENALI_PHY_447` writer"]
pub type W = crate::W<DdrDenaliPhy447Spec>;
#[doc = "Field `PHY_CLK_WRDM_SLAVE_DELAY_3` reader - Write clock slave delay setting for DM for slice 3."]
pub type PhyClkWrdmSlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDM_SLAVE_DELAY_3` writer - Write clock slave delay setting for DM for slice 3."]
pub type PhyClkWrdmSlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CLK_WRDQS_SLAVE_DELAY_3` reader - Write clock slave delay setting for DQS for slice 3."]
pub type PhyClkWrdqsSlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WRDQS_SLAVE_DELAY_3` writer - Write clock slave delay setting for DQS for slice 3."]
pub type PhyClkWrdqsSlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DM for slice 3."]
    #[inline(always)]
    pub fn phy_clk_wrdm_slave_delay_3(&self) -> PhyClkWrdmSlaveDelay3R {
        PhyClkWrdmSlaveDelay3R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:25 - Write clock slave delay setting for DQS for slice 3."]
    #[inline(always)]
    pub fn phy_clk_wrdqs_slave_delay_3(&self) -> PhyClkWrdqsSlaveDelay3R {
        PhyClkWrdqsSlaveDelay3R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Write clock slave delay setting for DM for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdm_slave_delay_3(&mut self) -> PhyClkWrdmSlaveDelay3W<DdrDenaliPhy447Spec> {
        PhyClkWrdmSlaveDelay3W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Write clock slave delay setting for DQS for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wrdqs_slave_delay_3(&mut self) -> PhyClkWrdqsSlaveDelay3W<DdrDenaliPhy447Spec> {
        PhyClkWrdqsSlaveDelay3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_447::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_447::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy447Spec;
impl crate::RegisterSpec for DdrDenaliPhy447Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_447::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy447Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_447::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy447Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_447 to value 0"]
impl crate::Resettable for DdrDenaliPhy447Spec {
    const RESET_VALUE: u32 = 0;
}
