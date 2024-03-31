#[doc = "Register `DENALI_PHY_512` reader"]
pub type R = crate::R<DenaliPhy512Spec>;
#[doc = "Register `DENALI_PHY_512` writer"]
pub type W = crate::W<DenaliPhy512Spec>;
#[doc = "Field `PHY_ADR0_SW_WRADDR_SHIFT_0` reader - Manual override of automatic half_cycle_shift/cycle_shift for address slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr0SwWraddrShift0R = crate::FieldReader;
#[doc = "Field `PHY_ADR0_SW_WRADDR_SHIFT_0` writer - Manual override of automatic half_cycle_shift/cycle_shift for address slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr0SwWraddrShift0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_ADR1_SW_WRADDR_SHIFT_0` reader - Manual override of automatic half_cycle_shift/cycle_shift for address slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr1SwWraddrShift0R = crate::FieldReader;
#[doc = "Field `PHY_ADR1_SW_WRADDR_SHIFT_0` writer - Manual override of automatic half_cycle_shift/cycle_shift for address slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr1SwWraddrShift0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_ADR2_SW_WRADDR_SHIFT_0` reader - Manual override of automatic half_cycle_shift/cycle_shift for address slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr2SwWraddrShift0R = crate::FieldReader;
#[doc = "Field `PHY_ADR2_SW_WRADDR_SHIFT_0` writer - Manual override of automatic half_cycle_shift/cycle_shift for address slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr2SwWraddrShift0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_ADR3_SW_WRADDR_SHIFT_0` reader - Manual override of automatic half_cycle_shift/cycle_shift for address slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr3SwWraddrShift0R = crate::FieldReader;
#[doc = "Field `PHY_ADR3_SW_WRADDR_SHIFT_0` writer - Manual override of automatic half_cycle_shift/cycle_shift for address slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
pub type PhyAdr3SwWraddrShift0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Manual override of automatic half_cycle_shift/cycle_shift for address slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    pub fn phy_adr0_sw_wraddr_shift_0(&self) -> PhyAdr0SwWraddrShift0R {
        PhyAdr0SwWraddrShift0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Manual override of automatic half_cycle_shift/cycle_shift for address slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    pub fn phy_adr1_sw_wraddr_shift_0(&self) -> PhyAdr1SwWraddrShift0R {
        PhyAdr1SwWraddrShift0R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Manual override of automatic half_cycle_shift/cycle_shift for address slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    pub fn phy_adr2_sw_wraddr_shift_0(&self) -> PhyAdr2SwWraddrShift0R {
        PhyAdr2SwWraddrShift0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Manual override of automatic half_cycle_shift/cycle_shift for address slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    pub fn phy_adr3_sw_wraddr_shift_0(&self) -> PhyAdr3SwWraddrShift0R {
        PhyAdr3SwWraddrShift0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Manual override of automatic half_cycle_shift/cycle_shift for address slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr0_sw_wraddr_shift_0(&mut self) -> PhyAdr0SwWraddrShift0W<DenaliPhy512Spec> {
        PhyAdr0SwWraddrShift0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Manual override of automatic half_cycle_shift/cycle_shift for address slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr1_sw_wraddr_shift_0(&mut self) -> PhyAdr1SwWraddrShift0W<DenaliPhy512Spec> {
        PhyAdr1SwWraddrShift0W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Manual override of automatic half_cycle_shift/cycle_shift for address slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr2_sw_wraddr_shift_0(&mut self) -> PhyAdr2SwWraddrShift0W<DenaliPhy512Spec> {
        PhyAdr2SwWraddrShift0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Manual override of automatic half_cycle_shift/cycle_shift for address slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) is the cycle_shift value. For bits (4:3), clear to 0x0 for no offset, program to 0x1 for -1 cycle, program to 0x2 for +1 cycle, or program to 0x3 for -2 cycles."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr3_sw_wraddr_shift_0(&mut self) -> PhyAdr3SwWraddrShift0W<DenaliPhy512Spec> {
        PhyAdr3SwWraddrShift0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_512::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_512::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy512Spec;
impl crate::RegisterSpec for DenaliPhy512Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_512::R`](R) reader structure"]
impl crate::Readable for DenaliPhy512Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_512::W`](W) writer structure"]
impl crate::Writable for DenaliPhy512Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_512 to value 0"]
impl crate::Resettable for DenaliPhy512Spec {
    const RESET_VALUE: u32 = 0;
}
