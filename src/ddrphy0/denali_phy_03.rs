#[doc = "Register `DENALI_PHY_03` reader"]
pub type R = crate::R<DenaliPhy03Spec>;
#[doc = "Register `DENALI_PHY_03` writer"]
pub type W = crate::W<DenaliPhy03Spec>;
#[doc = "Field `PHY_SW_WRDQ0_SHIFT_0` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ0 for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq0Shift0R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ0_SHIFT_0` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ0 for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq0Shift0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_WRDQ1_SHIFT_0` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ1 for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq1Shift0R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ1_SHIFT_0` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ1 for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq1Shift0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_WRDQ2_SHIFT_0` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ2 for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq2Shift0R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ2_SHIFT_0` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ2 for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq2Shift0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_WRDQ3_SHIFT_0` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ3 for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq3Shift0R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ3_SHIFT_0` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ3 for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq3Shift0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ0 for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq0_shift_0(&self) -> PhySwWrdq0Shift0R {
        PhySwWrdq0Shift0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ1 for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq1_shift_0(&self) -> PhySwWrdq1Shift0R {
        PhySwWrdq1Shift0R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ2 for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq2_shift_0(&self) -> PhySwWrdq2Shift0R {
        PhySwWrdq2Shift0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ3 for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq3_shift_0(&self) -> PhySwWrdq3Shift0R {
        PhySwWrdq3Shift0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ0 for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq0_shift_0(&mut self) -> PhySwWrdq0Shift0W<DenaliPhy03Spec> {
        PhySwWrdq0Shift0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ1 for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq1_shift_0(&mut self) -> PhySwWrdq1Shift0W<DenaliPhy03Spec> {
        PhySwWrdq1Shift0W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ2 for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq2_shift_0(&mut self) -> PhySwWrdq2Shift0W<DenaliPhy03Spec> {
        PhySwWrdq2Shift0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ3 for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq3_shift_0(&mut self) -> PhySwWrdq3Shift0W<DenaliPhy03Spec> {
        PhySwWrdq3Shift0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_03::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_03::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy03Spec;
impl crate::RegisterSpec for DenaliPhy03Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_03::R`](R) reader structure"]
impl crate::Readable for DenaliPhy03Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_03::W`](W) writer structure"]
impl crate::Writable for DenaliPhy03Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_03 to value 0"]
impl crate::Resettable for DenaliPhy03Spec {
    const RESET_VALUE: u32 = 0;
}
