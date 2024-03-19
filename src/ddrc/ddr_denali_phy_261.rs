#[doc = "Register `DDR_DENALI_PHY_261` reader"]
pub type R = crate::R<DdrDenaliPhy261Spec>;
#[doc = "Register `DDR_DENALI_PHY_261` writer"]
pub type W = crate::W<DdrDenaliPhy261Spec>;
#[doc = "Field `PHY_SW_WRDM_SHIFT_2` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DM for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdmShift2R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDM_SHIFT_2` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DM for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
pub type PhySwWrdmShift2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_WRDQS_SHIFT_2` reader - Manual override of automatic half_cycle_shift/cycle_shift for write DQS for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
pub type PhySwWrdqsShift2R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQS_SHIFT_2` writer - Manual override of automatic half_cycle_shift/cycle_shift for write DQS for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
pub type PhySwWrdqsShift2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_DQ_TSEL_ENABLE_2` reader - Operation type tsel enables for DQ/ DM signals for slice 2. Bit (0) enables tsel_en during read cycles. Bit (1) enables tsel_en during write cycles. Bit (2) enables tsel_en during idle cycles. Set each bit to 1 to enable."]
pub type PhyDqTselEnable2R = crate::FieldReader;
#[doc = "Field `PHY_DQ_TSEL_ENABLE_2` writer - Operation type tsel enables for DQ/ DM signals for slice 2. Bit (0) enables tsel_en during read cycles. Bit (1) enables tsel_en during write cycles. Bit (2) enables tsel_en during idle cycles. Set each bit to 1 to enable."]
pub type PhyDqTselEnable2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - Manual override of automatic half_cycle_shift/cycle_shift for write DM for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdm_shift_2(&self) -> PhySwWrdmShift2R {
        PhySwWrdmShift2R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Manual override of automatic half_cycle_shift/cycle_shift for write DQS for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdqs_shift_2(&self) -> PhySwWrdqsShift2R {
        PhySwWrdqsShift2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Operation type tsel enables for DQ/ DM signals for slice 2. Bit (0) enables tsel_en during read cycles. Bit (1) enables tsel_en during write cycles. Bit (2) enables tsel_en during idle cycles. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn phy_dq_tsel_enable_2(&self) -> PhyDqTselEnable2R {
        PhyDqTselEnable2R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Manual override of automatic half_cycle_shift/cycle_shift for write DM for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bits (4:3) are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdm_shift_2(&mut self) -> PhySwWrdmShift2W<DdrDenaliPhy261Spec> {
        PhySwWrdmShift2W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Manual override of automatic half_cycle_shift/cycle_shift for write DQS for slice 2. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdqs_shift_2(&mut self) -> PhySwWrdqsShift2W<DdrDenaliPhy261Spec> {
        PhySwWrdqsShift2W::new(self, 8)
    }
    #[doc = "Bits 16:18 - Operation type tsel enables for DQ/ DM signals for slice 2. Bit (0) enables tsel_en during read cycles. Bit (1) enables tsel_en during write cycles. Bit (2) enables tsel_en during idle cycles. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_tsel_enable_2(&mut self) -> PhyDqTselEnable2W<DdrDenaliPhy261Spec> {
        PhyDqTselEnable2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_261::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_261::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy261Spec;
impl crate::RegisterSpec for DdrDenaliPhy261Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_261::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy261Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_261::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy261Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_261 to value 0"]
impl crate::Resettable for DdrDenaliPhy261Spec {
    const RESET_VALUE: u32 = 0;
}
