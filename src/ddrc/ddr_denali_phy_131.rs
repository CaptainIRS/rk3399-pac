#[doc = "Register `DDR_DENALI_PHY_131` reader"]
pub type R = crate::R<DdrDenaliPhy131Spec>;
#[doc = "Register `DDR_DENALI_PHY_131` writer"]
pub type W = crate::W<DdrDenaliPhy131Spec>;
#[doc = "Field `PHY_SW_WRDQ0_SHIFT_1` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ0 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq0Shift1R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ0_SHIFT_1` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ0 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq0Shift1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_WRDQ1_SHIFT_1` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ1 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq1Shift1R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ1_SHIFT_1` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ1 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq1Shift1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_WRDQ2_SHIFT_1` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ2 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq2Shift1R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ2_SHIFT_1` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ2 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq2Shift1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_WRDQ3_SHIFT_1` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ3 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq3Shift1R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ3_SHIFT_1` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ3 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq3Shift1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ0 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq0_shift_1(&self) -> PhySwWrdq0Shift1R {
        PhySwWrdq0Shift1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ1 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq1_shift_1(&self) -> PhySwWrdq1Shift1R {
        PhySwWrdq1Shift1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ2 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq2_shift_1(&self) -> PhySwWrdq2Shift1R {
        PhySwWrdq2Shift1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ3 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq3_shift_1(&self) -> PhySwWrdq3Shift1R {
        PhySwWrdq3Shift1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ0 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq0_shift_1(&mut self) -> PhySwWrdq0Shift1W<DdrDenaliPhy131Spec> {
        PhySwWrdq0Shift1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ1 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq1_shift_1(&mut self) -> PhySwWrdq1Shift1W<DdrDenaliPhy131Spec> {
        PhySwWrdq1Shift1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ2 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq2_shift_1(&mut self) -> PhySwWrdq2Shift1W<DdrDenaliPhy131Spec> {
        PhySwWrdq2Shift1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ3 for slice 1. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq3_shift_1(&mut self) -> PhySwWrdq3Shift1W<DdrDenaliPhy131Spec> {
        PhySwWrdq3Shift1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_131::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_131::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy131Spec;
impl crate::RegisterSpec for DdrDenaliPhy131Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_131::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy131Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_131::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy131Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_131 to value 0"]
impl crate::Resettable for DdrDenaliPhy131Spec {
    const RESET_VALUE: u32 = 0;
}
