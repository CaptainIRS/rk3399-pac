#[doc = "Register `DENALI_PHY_519` reader"]
pub type R = crate::R<DenaliPhy519Spec>;
#[doc = "Register `DENALI_PHY_519` writer"]
pub type W = crate::W<DenaliPhy519Spec>;
#[doc = "Field `PHY_ADR_TYPE_0` reader - DRAM type for address slice 0."]
pub type PhyAdrType0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_TYPE_0` writer - DRAM type for address slice 0."]
pub type PhyAdrType0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_ADR_WRADDR_SHIFT_OBS_0` reader - Observation register for automatic half cycle and cycle shift values for address slice 0."]
pub type PhyAdrWraddrShiftObs0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_IE_MODE_0` reader - Input enable control for address slice 0."]
pub type PhyAdrIeMode0R = crate::BitReader;
#[doc = "Field `PHY_ADR_IE_MODE_0` writer - Input enable control for address slice 0."]
pub type PhyAdrIeMode0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - DRAM type for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_type_0(&self) -> PhyAdrType0R {
        PhyAdrType0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:10 - Observation register for automatic half cycle and cycle shift values for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_wraddr_shift_obs_0(&self) -> PhyAdrWraddrShiftObs0R {
        PhyAdrWraddrShiftObs0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - Input enable control for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_ie_mode_0(&self) -> PhyAdrIeMode0R {
        PhyAdrIeMode0R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DRAM type for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_type_0(&mut self) -> PhyAdrType0W<DenaliPhy519Spec> {
        PhyAdrType0W::new(self, 0)
    }
    #[doc = "Bit 16 - Input enable control for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_ie_mode_0(&mut self) -> PhyAdrIeMode0W<DenaliPhy519Spec> {
        PhyAdrIeMode0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_519::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_519::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy519Spec;
impl crate::RegisterSpec for DenaliPhy519Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_519::R`](R) reader structure"]
impl crate::Readable for DenaliPhy519Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_519::W`](W) writer structure"]
impl crate::Writable for DenaliPhy519Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_519 to value 0"]
impl crate::Resettable for DenaliPhy519Spec {
    const RESET_VALUE: u32 = 0;
}
