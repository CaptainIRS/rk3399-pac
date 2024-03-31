#[doc = "Register `DENALI_PHY_897` reader"]
pub type R = crate::R<DenaliPhy897Spec>;
#[doc = "Register `DENALI_PHY_897` writer"]
pub type W = crate::W<DenaliPhy897Spec>;
#[doc = "Field `PHY_SW_GRP_SHIFT_2` reader - Address/control group slice 2 manual override of automatic half_cycle_shift/cycle_shift. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
pub type PhySwGrpShift2R = crate::FieldReader;
#[doc = "Field `PHY_SW_GRP_SHIFT_2` writer - Address/control group slice 2 manual override of automatic half_cycle_shift/cycle_shift. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
pub type PhySwGrpShift2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_GRP_SHIFT_3` reader - Address/control group slice 3 manual override of automatic half_cycle_shift/cycle_shift. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
pub type PhySwGrpShift3R = crate::FieldReader;
#[doc = "Field `PHY_SW_GRP_SHIFT_3` writer - Address/control group slice 3 manual override of automatic half_cycle_shift/cycle_shift. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
pub type PhySwGrpShift3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_GRP_BYPASS_SLAVE_DELAY` reader - Address/control group slice bypass mode slave delay setting."]
pub type PhyGrpBypassSlaveDelayR = crate::FieldReader<u16>;
#[doc = "Field `PHY_GRP_BYPASS_SLAVE_DELAY` writer - Address/control group slice bypass mode slave delay setting."]
pub type PhyGrpBypassSlaveDelayW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:4 - Address/control group slice 2 manual override of automatic half_cycle_shift/cycle_shift. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_grp_shift_2(&self) -> PhySwGrpShift2R {
        PhySwGrpShift2R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Address/control group slice 3 manual override of automatic half_cycle_shift/cycle_shift. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_grp_shift_3(&self) -> PhySwGrpShift3R {
        PhySwGrpShift3R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - Address/control group slice bypass mode slave delay setting."]
    #[inline(always)]
    pub fn phy_grp_bypass_slave_delay(&self) -> PhyGrpBypassSlaveDelayR {
        PhyGrpBypassSlaveDelayR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - Address/control group slice 2 manual override of automatic half_cycle_shift/cycle_shift. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_grp_shift_2(&mut self) -> PhySwGrpShift2W<DenaliPhy897Spec> {
        PhySwGrpShift2W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Address/control group slice 3 manual override of automatic half_cycle_shift/cycle_shift. Bit (0) enables override of half_cycle_shift. Bit (1) is the half_cycle_shift value. Bit (2) enables override of cycle shift. Bit (3) is the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_grp_shift_3(&mut self) -> PhySwGrpShift3W<DenaliPhy897Spec> {
        PhySwGrpShift3W::new(self, 8)
    }
    #[doc = "Bits 16:26 - Address/control group slice bypass mode slave delay setting."]
    #[inline(always)]
    #[must_use]
    pub fn phy_grp_bypass_slave_delay(&mut self) -> PhyGrpBypassSlaveDelayW<DenaliPhy897Spec> {
        PhyGrpBypassSlaveDelayW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_897::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_897::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy897Spec;
impl crate::RegisterSpec for DenaliPhy897Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_897::R`](R) reader structure"]
impl crate::Readable for DenaliPhy897Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_897::W`](W) writer structure"]
impl crate::Writable for DenaliPhy897Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_897 to value 0"]
impl crate::Resettable for DenaliPhy897Spec {
    const RESET_VALUE: u32 = 0;
}
