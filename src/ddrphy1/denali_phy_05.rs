#[doc = "Register `DENALI_PHY_05` reader"]
pub type R = crate::R<DenaliPhy05Spec>;
#[doc = "Register `DENALI_PHY_05` writer"]
pub type W = crate::W<DenaliPhy05Spec>;
#[doc = "Field `PHY_SW_WRDM_SHIFT_0` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DM for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdmShift0R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDM_SHIFT_0` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DM for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdmShift0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_WRDQS_SHIFT_0` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQS for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
pub type PhySwWrdqsShift0R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQS_SHIFT_0` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQS for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
pub type PhySwWrdqsShift0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_DQ_TSEL_ENABLE_0` reader - Operation type tsel enables for DQ/ DM signals for slice 0. Bit (0) enables tsel_en during read cycles. Bit (1) enables tsel_en during write cycles. Bit (2) enables tsel_en during idle cycles. Set each bit to 1 to enable."]
pub type PhyDqTselEnable0R = crate::FieldReader;
#[doc = "Field `PHY_DQ_TSEL_ENABLE_0` writer - Operation type tsel enables for DQ/ DM signals for slice 0. Bit (0) enables tsel_en during read cycles. Bit (1) enables tsel_en during write cycles. Bit (2) enables tsel_en during idle cycles. Set each bit to 1 to enable."]
pub type PhyDqTselEnable0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - Manual override of automatic half_cycle_shift/cycle_shift for write DM for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdm_shift_0(&self) -> PhySwWrdmShift0R {
        PhySwWrdmShift0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Manual override of automatic half_cycle_shift/cycle_shift for write DQS for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdqs_shift_0(&self) -> PhySwWrdqsShift0R {
        PhySwWrdqsShift0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Operation type tsel enables for DQ/ DM signals for slice 0. Bit (0) enables tsel_en during read cycles. Bit (1) enables tsel_en during write cycles. Bit (2) enables tsel_en during idle cycles. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn phy_dq_tsel_enable_0(&self) -> PhyDqTselEnable0R {
        PhyDqTselEnable0R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Manual override of automatic half_cycle_shift/cycle_shift for write DM for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdm_shift_0(&mut self) -> PhySwWrdmShift0W<DenaliPhy05Spec> {
        PhySwWrdmShift0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Manual override of automatic half_cycle_shift/cycle_shift for write DQS for slice 0. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdqs_shift_0(&mut self) -> PhySwWrdqsShift0W<DenaliPhy05Spec> {
        PhySwWrdqsShift0W::new(self, 8)
    }
    #[doc = "Bits 16:18 - Operation type tsel enables for DQ/ DM signals for slice 0. Bit (0) enables tsel_en during read cycles. Bit (1) enables tsel_en during write cycles. Bit (2) enables tsel_en during idle cycles. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_tsel_enable_0(&mut self) -> PhyDqTselEnable0W<DenaliPhy05Spec> {
        PhyDqTselEnable0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_05::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_05::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy05Spec;
impl crate::RegisterSpec for DenaliPhy05Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_05::R`](R) reader structure"]
impl crate::Readable for DenaliPhy05Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_05::W`](W) writer structure"]
impl crate::Writable for DenaliPhy05Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_05 to value 0"]
impl crate::Resettable for DenaliPhy05Spec {
    const RESET_VALUE: u32 = 0;
}
