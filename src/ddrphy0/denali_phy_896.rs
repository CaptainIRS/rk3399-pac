#[doc = "Register `DENALI_PHY_896` reader"]
pub type R = crate::R<DenaliPhy896Spec>;
#[doc = "Register `DENALI_PHY_896` writer"]
pub type W = crate::W<DenaliPhy896Spec>;
#[doc = "Field `PHY_FREQ_SEL_MULTICAST_EN` reader - When set, a register write will update parameters for all frequency sets simultaneously. Set to 1 to enable."]
pub type PhyFreqSelMulticastEnR = crate::BitReader;
#[doc = "Field `PHY_FREQ_SEL_MULTICAST_EN` writer - When set, a register write will update parameters for all frequency sets simultaneously. Set to 1 to enable."]
pub type PhyFreqSelMulticastEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_FREQ_SEL_INDEX` reader - Selects which frequency set to update."]
pub type PhyFreqSelIndexR = crate::FieldReader;
#[doc = "Field `PHY_FREQ_SEL_INDEX` writer - Selects which frequency set to update."]
pub type PhyFreqSelIndexW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_SW_GRP_SHIFT_0` reader - Address/control group slice 0 manual override of automatic half_cycle_shift/cycle_shift. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
pub type PhySwGrpShift0R = crate::FieldReader;
#[doc = "Field `PHY_SW_GRP_SHIFT_0` writer - Address/control group slice 0 manual override of automatic half_cycle_shift/cycle_shift. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
pub type PhySwGrpShift0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_GRP_SHIFT_1` reader - Address/control group slice 1 manual override of automatic half_cycle_shift/cycle_shift. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
pub type PhySwGrpShift1R = crate::FieldReader;
#[doc = "Field `PHY_SW_GRP_SHIFT_1` writer - Address/control group slice 1 manual override of automatic half_cycle_shift/cycle_shift. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
pub type PhySwGrpShift1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - When set, a register write will update parameters for all frequency sets simultaneously. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_freq_sel_multicast_en(&self) -> PhyFreqSelMulticastEnR {
        PhyFreqSelMulticastEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Selects which frequency set to update."]
    #[inline(always)]
    pub fn phy_freq_sel_index(&self) -> PhyFreqSelIndexR {
        PhyFreqSelIndexR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:20 - Address/control group slice 0 manual override of automatic half_cycle_shift/cycle_shift. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_grp_shift_0(&self) -> PhySwGrpShift0R {
        PhySwGrpShift0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Address/control group slice 1 manual override of automatic half_cycle_shift/cycle_shift. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_grp_shift_1(&self) -> PhySwGrpShift1R {
        PhySwGrpShift1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - When set, a register write will update parameters for all frequency sets simultaneously. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_freq_sel_multicast_en(&mut self) -> PhyFreqSelMulticastEnW<DenaliPhy896Spec> {
        PhyFreqSelMulticastEnW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Selects which frequency set to update."]
    #[inline(always)]
    #[must_use]
    pub fn phy_freq_sel_index(&mut self) -> PhyFreqSelIndexW<DenaliPhy896Spec> {
        PhyFreqSelIndexW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Address/control group slice 0 manual override of automatic half_cycle_shift/cycle_shift. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_grp_shift_0(&mut self) -> PhySwGrpShift0W<DenaliPhy896Spec> {
        PhySwGrpShift0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Address/control group slice 1 manual override of automatic half_cycle_shift/cycle_shift. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_grp_shift_1(&mut self) -> PhySwGrpShift1W<DenaliPhy896Spec> {
        PhySwGrpShift1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_896::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_896::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy896Spec;
impl crate::RegisterSpec for DenaliPhy896Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_896::R`](R) reader structure"]
impl crate::Readable for DenaliPhy896Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_896::W`](W) writer structure"]
impl crate::Writable for DenaliPhy896Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_896 to value 0x01"]
impl crate::Resettable for DenaliPhy896Spec {
    const RESET_VALUE: u32 = 0x01;
}
