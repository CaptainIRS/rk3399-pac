#[doc = "Register `DENALI_PHY_544` reader"]
pub type R = crate::R<DenaliPhy544Spec>;
#[doc = "Register `DENALI_PHY_544` writer"]
pub type W = crate::W<DenaliPhy544Spec>;
#[doc = "Field `PHY_ADR_TSEL_SELECT_0` reader - Tsel select values for address slice 0."]
pub type PhyAdrTselSelect0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_TSEL_SELECT_0` writer - Tsel select values for address slice 0."]
pub type PhyAdrTselSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_ADR0_CLK_WR_SLAVE_DELAY_0` reader - Address slice slave delay setting for address slice 0."]
pub type PhyAdr0ClkWrSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR0_CLK_WR_SLAVE_DELAY_0` writer - Address slice slave delay setting for address slice 0."]
pub type PhyAdr0ClkWrSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:7 - Tsel select values for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_tsel_select_0(&self) -> PhyAdrTselSelect0R {
        PhyAdrTselSelect0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:18 - Address slice slave delay setting for address slice 0."]
    #[inline(always)]
    pub fn phy_adr0_clk_wr_slave_delay_0(&self) -> PhyAdr0ClkWrSlaveDelay0R {
        PhyAdr0ClkWrSlaveDelay0R::new(((self.bits >> 8) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Tsel select values for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_tsel_select_0(&mut self) -> PhyAdrTselSelect0W<DenaliPhy544Spec> {
        PhyAdrTselSelect0W::new(self, 0)
    }
    #[doc = "Bits 8:18 - Address slice slave delay setting for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr0_clk_wr_slave_delay_0(&mut self) -> PhyAdr0ClkWrSlaveDelay0W<DenaliPhy544Spec> {
        PhyAdr0ClkWrSlaveDelay0W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_544::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_544::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy544Spec;
impl crate::RegisterSpec for DenaliPhy544Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_544::R`](R) reader structure"]
impl crate::Readable for DenaliPhy544Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_544::W`](W) writer structure"]
impl crate::Writable for DenaliPhy544Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_544 to value 0"]
impl crate::Resettable for DenaliPhy544Spec {
    const RESET_VALUE: u32 = 0;
}
