#[doc = "Register `DENALI_PHY_674` reader"]
pub type R = crate::R<DenaliPhy674Spec>;
#[doc = "Register `DENALI_PHY_674` writer"]
pub type W = crate::W<DenaliPhy674Spec>;
#[doc = "Field `PHY_ADR3_CLK_WR_SLAVE_DELAY_1` reader - Address slice slave delay setting for address slice 1."]
pub type PhyAdr3ClkWrSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR3_CLK_WR_SLAVE_DELAY_1` writer - Address slice slave delay setting for address slice 1."]
pub type PhyAdr3ClkWrSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_ADR4_CLK_WR_SLAVE_DELAY_1` reader - Address slice slave delay setting for address slice 1."]
pub type PhyAdr4ClkWrSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR4_CLK_WR_SLAVE_DELAY_1` writer - Address slice slave delay setting for address slice 1."]
pub type PhyAdr4ClkWrSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Address slice slave delay setting for address slice 1."]
    #[inline(always)]
    pub fn phy_adr3_clk_wr_slave_delay_1(&self) -> PhyAdr3ClkWrSlaveDelay1R {
        PhyAdr3ClkWrSlaveDelay1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Address slice slave delay setting for address slice 1."]
    #[inline(always)]
    pub fn phy_adr4_clk_wr_slave_delay_1(&self) -> PhyAdr4ClkWrSlaveDelay1R {
        PhyAdr4ClkWrSlaveDelay1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Address slice slave delay setting for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr3_clk_wr_slave_delay_1(&mut self) -> PhyAdr3ClkWrSlaveDelay1W<DenaliPhy674Spec> {
        PhyAdr3ClkWrSlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Address slice slave delay setting for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr4_clk_wr_slave_delay_1(&mut self) -> PhyAdr4ClkWrSlaveDelay1W<DenaliPhy674Spec> {
        PhyAdr4ClkWrSlaveDelay1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_674::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_674::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy674Spec;
impl crate::RegisterSpec for DenaliPhy674Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_674::R`](R) reader structure"]
impl crate::Readable for DenaliPhy674Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_674::W`](W) writer structure"]
impl crate::Writable for DenaliPhy674Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_674 to value 0"]
impl crate::Resettable for DenaliPhy674Spec {
    const RESET_VALUE: u32 = 0;
}
