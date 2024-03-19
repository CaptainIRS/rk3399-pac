#[doc = "Register `DDR_DENALI_PHY_647` reader"]
pub type R = crate::R<DdrDenaliPhy647Spec>;
#[doc = "Register `DDR_DENALI_PHY_647` writer"]
pub type W = crate::W<DdrDenaliPhy647Spec>;
#[doc = "Field `PHY_ADR_TYPE_1` reader - DRAM type for address slice 1."]
pub type PhyAdrType1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_TYPE_1` writer - DRAM type for address slice 1."]
pub type PhyAdrType1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_ADR_WRADDR_SHIFT_OBS_1` reader - Observation register for automatic half cycle and cycle shift values for address slice 1. READ-ONLY"]
pub type PhyAdrWraddrShiftObs1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_IE_MODE_1` reader - Input enable control for address slice 1."]
pub type PhyAdrIeMode1R = crate::BitReader;
#[doc = "Field `PHY_ADR_IE_MODE_1` writer - Input enable control for address slice 1."]
pub type PhyAdrIeMode1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - DRAM type for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_type_1(&self) -> PhyAdrType1R {
        PhyAdrType1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:10 - Observation register for automatic half cycle and cycle shift values for address slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_wraddr_shift_obs_1(&self) -> PhyAdrWraddrShiftObs1R {
        PhyAdrWraddrShiftObs1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - Input enable control for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_ie_mode_1(&self) -> PhyAdrIeMode1R {
        PhyAdrIeMode1R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DRAM type for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_type_1(&mut self) -> PhyAdrType1W<DdrDenaliPhy647Spec> {
        PhyAdrType1W::new(self, 0)
    }
    #[doc = "Bit 16 - Input enable control for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_ie_mode_1(&mut self) -> PhyAdrIeMode1W<DdrDenaliPhy647Spec> {
        PhyAdrIeMode1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_647::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_647::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy647Spec;
impl crate::RegisterSpec for DdrDenaliPhy647Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_647::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy647Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_647::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy647Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_647 to value 0"]
impl crate::Resettable for DdrDenaliPhy647Spec {
    const RESET_VALUE: u32 = 0;
}
