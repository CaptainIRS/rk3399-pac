#[doc = "Register `DDR_DENALI_PHY_775` reader"]
pub type R = crate::R<DdrDenaliPhy775Spec>;
#[doc = "Register `DDR_DENALI_PHY_775` writer"]
pub type W = crate::W<DdrDenaliPhy775Spec>;
#[doc = "Field `PHY_ADR_TYPE_2` reader - DRAM type for address slice 2."]
pub type PhyAdrType2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_TYPE_2` writer - DRAM type for address slice 2."]
pub type PhyAdrType2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_ADR_WRADDR_SHIFT_OBS_2` reader - Observation register for automatic half cycle and cycle shift values for address slice 2. READ-ONLY"]
pub type PhyAdrWraddrShiftObs2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_IE_MODE_2` reader - Input enable control for address slice 2."]
pub type PhyAdrIeMode2R = crate::BitReader;
#[doc = "Field `PHY_ADR_IE_MODE_2` writer - Input enable control for address slice 2."]
pub type PhyAdrIeMode2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - DRAM type for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_type_2(&self) -> PhyAdrType2R {
        PhyAdrType2R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:10 - Observation register for automatic half cycle and cycle shift values for address slice 2. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_wraddr_shift_obs_2(&self) -> PhyAdrWraddrShiftObs2R {
        PhyAdrWraddrShiftObs2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - Input enable control for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_ie_mode_2(&self) -> PhyAdrIeMode2R {
        PhyAdrIeMode2R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DRAM type for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_type_2(&mut self) -> PhyAdrType2W<DdrDenaliPhy775Spec> {
        PhyAdrType2W::new(self, 0)
    }
    #[doc = "Bit 16 - Input enable control for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_ie_mode_2(&mut self) -> PhyAdrIeMode2W<DdrDenaliPhy775Spec> {
        PhyAdrIeMode2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_775::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_775::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy775Spec;
impl crate::RegisterSpec for DdrDenaliPhy775Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_775::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy775Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_775::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy775Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_775 to value 0"]
impl crate::Resettable for DdrDenaliPhy775Spec {
    const RESET_VALUE: u32 = 0;
}
