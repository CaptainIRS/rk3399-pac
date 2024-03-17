#[doc = "Register `DENALI_PHY_672` reader"]
pub type R = crate::R<DenaliPhy672Spec>;
#[doc = "Register `DENALI_PHY_672` writer"]
pub type W = crate::W<DenaliPhy672Spec>;
#[doc = "Field `PHY_ADR_TSEL_SELECT_1` reader - Tsel select values for address slice 1."]
pub type PhyAdrTselSelect1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_TSEL_SELECT_1` writer - Tsel select values for address slice 1."]
pub type PhyAdrTselSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_ADR0_CLK_WR_SLAVE_DELAY_1` reader - Address slice slave delay setting for address slice 1."]
pub type PhyAdr0ClkWrSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR0_CLK_WR_SLAVE_DELAY_1` writer - Address slice slave delay setting for address slice 1."]
pub type PhyAdr0ClkWrSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:7 - Tsel select values for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_tsel_select_1(&self) -> PhyAdrTselSelect1R {
        PhyAdrTselSelect1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:18 - Address slice slave delay setting for address slice 1."]
    #[inline(always)]
    pub fn phy_adr0_clk_wr_slave_delay_1(&self) -> PhyAdr0ClkWrSlaveDelay1R {
        PhyAdr0ClkWrSlaveDelay1R::new(((self.bits >> 8) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Tsel select values for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_tsel_select_1(&mut self) -> PhyAdrTselSelect1W<DenaliPhy672Spec> {
        PhyAdrTselSelect1W::new(self, 0)
    }
    #[doc = "Bits 8:18 - Address slice slave delay setting for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr0_clk_wr_slave_delay_1(&mut self) -> PhyAdr0ClkWrSlaveDelay1W<DenaliPhy672Spec> {
        PhyAdr0ClkWrSlaveDelay1W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_672::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_672::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy672Spec;
impl crate::RegisterSpec for DenaliPhy672Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_672::R`](R) reader structure"]
impl crate::Readable for DenaliPhy672Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_672::W`](W) writer structure"]
impl crate::Writable for DenaliPhy672Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_672 to value 0"]
impl crate::Resettable for DenaliPhy672Spec {
    const RESET_VALUE: u32 = 0;
}
