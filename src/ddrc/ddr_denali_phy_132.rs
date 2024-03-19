#[doc = "Register `DDR_DENALI_PHY_132` reader"]
pub type R = crate::R<DdrDenaliPhy132Spec>;
#[doc = "Register `DDR_DENALI_PHY_132` writer"]
pub type W = crate::W<DdrDenaliPhy132Spec>;
#[doc = "Field `PHY_SW_WRDQ4_SHIFT_1` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ4 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq4Shift1R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ4_SHIFT_1` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ4 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq4Shift1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_WRDQ5_SHIFT_1` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ5 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq5Shift1R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ5_SHIFT_1` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ5 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq5Shift1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_WRDQ6_SHIFT_1` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ6 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq6Shift1R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ6_SHIFT_1` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ6 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq6Shift1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_WRDQ7_SHIFT_1` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ7 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq7Shift1R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ7_SHIFT_1` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ7 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq7Shift1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ4 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq4_shift_1(&self) -> PhySwWrdq4Shift1R {
        PhySwWrdq4Shift1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ5 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq5_shift_1(&self) -> PhySwWrdq5Shift1R {
        PhySwWrdq5Shift1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ6 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq6_shift_1(&self) -> PhySwWrdq6Shift1R {
        PhySwWrdq6Shift1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ7 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq7_shift_1(&self) -> PhySwWrdq7Shift1R {
        PhySwWrdq7Shift1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ4 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq4_shift_1(&mut self) -> PhySwWrdq4Shift1W<DdrDenaliPhy132Spec> {
        PhySwWrdq4Shift1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ5 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq5_shift_1(&mut self) -> PhySwWrdq5Shift1W<DdrDenaliPhy132Spec> {
        PhySwWrdq5Shift1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ6 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq6_shift_1(&mut self) -> PhySwWrdq6Shift1W<DdrDenaliPhy132Spec> {
        PhySwWrdq6Shift1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ7 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq7_shift_1(&mut self) -> PhySwWrdq7Shift1W<DdrDenaliPhy132Spec> {
        PhySwWrdq7Shift1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_132::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_132::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy132Spec;
impl crate::RegisterSpec for DdrDenaliPhy132Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_132::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy132Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_132::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy132Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_132 to value 0"]
impl crate::Resettable for DdrDenaliPhy132Spec {
    const RESET_VALUE: u32 = 0;
}
