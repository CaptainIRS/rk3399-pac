#[doc = "Register `DENALI_PHY_769` reader"]
pub type R = crate::R<DenaliPhy769Spec>;
#[doc = "Register `DENALI_PHY_769` writer"]
pub type W = crate::W<DenaliPhy769Spec>;
#[doc = "Field `PHY_ADR4_SW_WRADDR_SHIFT_2` reader - Manual override of automatic half_cycle_shift/cycle_shift for address slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr4SwWraddrShift2R = crate::FieldReader;
#[doc = "Field `PHY_ADR4_SW_WRADDR_SHIFT_2` writer - Manual override of automatic half_cycle_shift/cycle_shift for address slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr4SwWraddrShift2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_ADR5_SW_WRADDR_SHIFT_2` reader - Manual override of automatic half_cycle_shift/cycle_shift for address slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr5SwWraddrShift2R = crate::FieldReader;
#[doc = "Field `PHY_ADR5_SW_WRADDR_SHIFT_2` writer - Manual override of automatic half_cycle_shift/cycle_shift for address slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr5SwWraddrShift2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_ADR_CLK_WR_BYPASS_SLAVE_DELAY_2` reader - Write address clock bypass mode slave delay setting for address slice 2."]
pub type PhyAdrClkWrBypassSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_CLK_WR_BYPASS_SLAVE_DELAY_2` writer - Write address clock bypass mode slave delay setting for address slice 2."]
pub type PhyAdrClkWrBypassSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:4 - Manual override of automatic half_cycle_shift/cycle_shift for address slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    pub fn phy_adr4_sw_wraddr_shift_2(&self) -> PhyAdr4SwWraddrShift2R {
        PhyAdr4SwWraddrShift2R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Manual override of automatic half_cycle_shift/cycle_shift for address slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    pub fn phy_adr5_sw_wraddr_shift_2(&self) -> PhyAdr5SwWraddrShift2R {
        PhyAdr5SwWraddrShift2R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - Write address clock bypass mode slave delay setting for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_clk_wr_bypass_slave_delay_2(&self) -> PhyAdrClkWrBypassSlaveDelay2R {
        PhyAdrClkWrBypassSlaveDelay2R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - Manual override of automatic half_cycle_shift/cycle_shift for address slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr4_sw_wraddr_shift_2(&mut self) -> PhyAdr4SwWraddrShift2W<DenaliPhy769Spec> {
        PhyAdr4SwWraddrShift2W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Manual override of automatic half_cycle_shift/cycle_shift for address slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr5_sw_wraddr_shift_2(&mut self) -> PhyAdr5SwWraddrShift2W<DenaliPhy769Spec> {
        PhyAdr5SwWraddrShift2W::new(self, 8)
    }
    #[doc = "Bits 16:26 - Write address clock bypass mode slave delay setting for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_clk_wr_bypass_slave_delay_2(
        &mut self,
    ) -> PhyAdrClkWrBypassSlaveDelay2W<DenaliPhy769Spec> {
        PhyAdrClkWrBypassSlaveDelay2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_769::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_769::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy769Spec;
impl crate::RegisterSpec for DenaliPhy769Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_769::R`](R) reader structure"]
impl crate::Readable for DenaliPhy769Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_769::W`](W) writer structure"]
impl crate::Writable for DenaliPhy769Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_769 to value 0"]
impl crate::Resettable for DenaliPhy769Spec {
    const RESET_VALUE: u32 = 0;
}
