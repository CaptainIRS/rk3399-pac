#[doc = "Register `DDR_DENALI_PHY_673` reader"]
pub type R = crate::R<DdrDenaliPhy673Spec>;
#[doc = "Register `DDR_DENALI_PHY_673` writer"]
pub type W = crate::W<DdrDenaliPhy673Spec>;
#[doc = "Field `PHY_ADR1_CLK_WR_SLAVE_DELAY_1` reader - Address slice slave delay setting for address slice 1."]
pub type PhyAdr1ClkWrSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR1_CLK_WR_SLAVE_DELAY_1` writer - Address slice slave delay setting for address slice 1."]
pub type PhyAdr1ClkWrSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_ADR2_CLK_WR_SLAVE_DELAY_1` reader - Address slice slave delay setting for address slice 1."]
pub type PhyAdr2ClkWrSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR2_CLK_WR_SLAVE_DELAY_1` writer - Address slice slave delay setting for address slice 1."]
pub type PhyAdr2ClkWrSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Address slice slave delay setting for address slice 1."]
    #[inline(always)]
    pub fn phy_adr1_clk_wr_slave_delay_1(&self) -> PhyAdr1ClkWrSlaveDelay1R {
        PhyAdr1ClkWrSlaveDelay1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Address slice slave delay setting for address slice 1."]
    #[inline(always)]
    pub fn phy_adr2_clk_wr_slave_delay_1(&self) -> PhyAdr2ClkWrSlaveDelay1R {
        PhyAdr2ClkWrSlaveDelay1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Address slice slave delay setting for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr1_clk_wr_slave_delay_1(
        &mut self,
    ) -> PhyAdr1ClkWrSlaveDelay1W<DdrDenaliPhy673Spec> {
        PhyAdr1ClkWrSlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Address slice slave delay setting for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr2_clk_wr_slave_delay_1(
        &mut self,
    ) -> PhyAdr2ClkWrSlaveDelay1W<DdrDenaliPhy673Spec> {
        PhyAdr2ClkWrSlaveDelay1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_673::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_673::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy673Spec;
impl crate::RegisterSpec for DdrDenaliPhy673Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_673::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy673Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_673::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy673Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_673 to value 0"]
impl crate::Resettable for DdrDenaliPhy673Spec {
    const RESET_VALUE: u32 = 0;
}
