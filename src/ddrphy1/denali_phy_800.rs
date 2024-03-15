#[doc = "Register `DENALI_PHY_800` reader"]
pub type R = crate::R<DenaliPhy800Spec>;
#[doc = "Register `DENALI_PHY_800` writer"]
pub type W = crate::W<DenaliPhy800Spec>;
#[doc = "Field `PHY_ADR_TSEL_SELECT_2` reader - Tsel select values for address slice 2."]
pub type PhyAdrTselSelect2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_TSEL_SELECT_2` writer - Tsel select values for address slice 2."]
pub type PhyAdrTselSelect2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_ADR0_CLK_WR_SLAVE_DELAY_2` reader - Address slice slave delay setting for address slice 2."]
pub type PhyAdr0ClkWrSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR0_CLK_WR_SLAVE_DELAY_2` writer - Address slice slave delay setting for address slice 2."]
pub type PhyAdr0ClkWrSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:7 - Tsel select values for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_tsel_select_2(&self) -> PhyAdrTselSelect2R {
        PhyAdrTselSelect2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:18 - Address slice slave delay setting for address slice 2."]
    #[inline(always)]
    pub fn phy_adr0_clk_wr_slave_delay_2(&self) -> PhyAdr0ClkWrSlaveDelay2R {
        PhyAdr0ClkWrSlaveDelay2R::new(((self.bits >> 8) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Tsel select values for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_tsel_select_2(&mut self) -> PhyAdrTselSelect2W<DenaliPhy800Spec> {
        PhyAdrTselSelect2W::new(self, 0)
    }
    #[doc = "Bits 8:18 - Address slice slave delay setting for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr0_clk_wr_slave_delay_2(&mut self) -> PhyAdr0ClkWrSlaveDelay2W<DenaliPhy800Spec> {
        PhyAdr0ClkWrSlaveDelay2W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_800::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_800::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy800Spec;
impl crate::RegisterSpec for DenaliPhy800Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_800::R`](R) reader structure"]
impl crate::Readable for DenaliPhy800Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_800::W`](W) writer structure"]
impl crate::Writable for DenaliPhy800Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_800 to value 0"]
impl crate::Resettable for DenaliPhy800Spec {
    const RESET_VALUE: u32 = 0;
}
