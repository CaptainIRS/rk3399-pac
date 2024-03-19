#[doc = "Register `DDR_DENALI_PHY_260` reader"]
pub type R = crate::R<DdrDenaliPhy260Spec>;
#[doc = "Register `DDR_DENALI_PHY_260` writer"]
pub type W = crate::W<DdrDenaliPhy260Spec>;
#[doc = "Field `PHY_SW_WRDQ4_SHIFT_2` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ4 for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq4Shift2R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ4_SHIFT_2` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ4 for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq4Shift2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_WRDQ5_SHIFT_2` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ5 for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq5Shift2R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ5_SHIFT_2` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ5 for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq5Shift2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_WRDQ6_SHIFT_2` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ6 for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq6Shift2R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ6_SHIFT_2` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ6 for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq6Shift2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_WRDQ7_SHIFT_2` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ7 for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq7Shift2R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ7_SHIFT_2` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ7 for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq7Shift2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ4 for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq4_shift_2(&self) -> PhySwWrdq4Shift2R {
        PhySwWrdq4Shift2R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ5 for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq5_shift_2(&self) -> PhySwWrdq5Shift2R {
        PhySwWrdq5Shift2R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ6 for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq6_shift_2(&self) -> PhySwWrdq6Shift2R {
        PhySwWrdq6Shift2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ7 for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq7_shift_2(&self) -> PhySwWrdq7Shift2R {
        PhySwWrdq7Shift2R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ4 for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq4_shift_2(&mut self) -> PhySwWrdq4Shift2W<DdrDenaliPhy260Spec> {
        PhySwWrdq4Shift2W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ5 for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq5_shift_2(&mut self) -> PhySwWrdq5Shift2W<DdrDenaliPhy260Spec> {
        PhySwWrdq5Shift2W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ6 for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq6_shift_2(&mut self) -> PhySwWrdq6Shift2W<DdrDenaliPhy260Spec> {
        PhySwWrdq6Shift2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ7 for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq7_shift_2(&mut self) -> PhySwWrdq7Shift2W<DdrDenaliPhy260Spec> {
        PhySwWrdq7Shift2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_260::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_260::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy260Spec;
impl crate::RegisterSpec for DdrDenaliPhy260Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_260::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy260Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_260::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy260Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_260 to value 0"]
impl crate::Resettable for DdrDenaliPhy260Spec {
    const RESET_VALUE: u32 = 0;
}
