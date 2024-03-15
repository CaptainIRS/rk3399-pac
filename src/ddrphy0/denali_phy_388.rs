#[doc = "Register `DENALI_PHY_388` reader"]
pub type R = crate::R<DenaliPhy388Spec>;
#[doc = "Register `DENALI_PHY_388` writer"]
pub type W = crate::W<DenaliPhy388Spec>;
#[doc = "Field `PHY_SW_WRDQ4_SHIFT_3` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ4 for slice 3. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq4Shift3R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ4_SHIFT_3` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ4 for slice 3. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq4Shift3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_WRDQ5_SHIFT_3` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ5 for slice 3. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq5Shift3R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ5_SHIFT_3` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ5 for slice 3. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq5Shift3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_WRDQ6_SHIFT_3` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ6 for slice 3. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq6Shift3R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ6_SHIFT_3` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ6 for slice 3. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq6Shift3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_WRDQ7_SHIFT_3` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQ7 for slice 3. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq7Shift3R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ7_SHIFT_3` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQ7 for slice 3. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdq7Shift3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ4 for slice 3. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq4_shift_3(&self) -> PhySwWrdq4Shift3R {
        PhySwWrdq4Shift3R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ5 for slice 3. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq5_shift_3(&self) -> PhySwWrdq5Shift3R {
        PhySwWrdq5Shift3R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ6 for slice 3. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq6_shift_3(&self) -> PhySwWrdq6Shift3R {
        PhySwWrdq6Shift3R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ7 for slice 3. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq7_shift_3(&self) -> PhySwWrdq7Shift3R {
        PhySwWrdq7Shift3R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ4 for slice 3. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq4_shift_3(&mut self) -> PhySwWrdq4Shift3W<DenaliPhy388Spec> {
        PhySwWrdq4Shift3W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ5 for slice 3. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq5_shift_3(&mut self) -> PhySwWrdq5Shift3W<DenaliPhy388Spec> {
        PhySwWrdq5Shift3W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ6 for slice 3. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq6_shift_3(&mut self) -> PhySwWrdq6Shift3W<DenaliPhy388Spec> {
        PhySwWrdq6Shift3W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Manual override of automatic half_cycle_shift/cycle_shift for write DQ7 for slice 3. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq7_shift_3(&mut self) -> PhySwWrdq7Shift3W<DenaliPhy388Spec> {
        PhySwWrdq7Shift3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_388::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_388::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy388Spec;
impl crate::RegisterSpec for DenaliPhy388Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_388::R`](R) reader structure"]
impl crate::Readable for DenaliPhy388Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_388::W`](W) writer structure"]
impl crate::Writable for DenaliPhy388Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_388 to value 0"]
impl crate::Resettable for DenaliPhy388Spec {
    const RESET_VALUE: u32 = 0;
}
