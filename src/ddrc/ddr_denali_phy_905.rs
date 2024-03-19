#[doc = "Register `DDR_DENALI_PHY_905` reader"]
pub type R = crate::R<DdrDenaliPhy905Spec>;
#[doc = "Register `DDR_DENALI_PHY_905` writer"]
pub type W = crate::W<DdrDenaliPhy905Spec>;
#[doc = "Field `PHY_CALVL_CS_MAP` reader - Defines the slice numbers associated with each CS during CA training."]
pub type PhyCalvlCsMapR = crate::FieldReader;
#[doc = "Field `PHY_CALVL_CS_MAP` writer - Defines the slice numbers associated with each CS during CA training."]
pub type PhyCalvlCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_GRP_SLV_DLY_ENC_OBS_SELECT` reader - Select value to map an individual address/control group slice slave delay to the encoded value observation register."]
pub type PhyGrpSlvDlyEncObsSelectR = crate::FieldReader<u16>;
#[doc = "Field `PHY_GRP_SLV_DLY_ENC_OBS_SELECT` writer - Select value to map an individual address/control group slice slave delay to the encoded value observation register."]
pub type PhyGrpSlvDlyEncObsSelectW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_GRP_SHIFT_OBS_SELECT` reader - Select value to map an individual address/control group slice automatic cycle/half_cycle shift settings to the observation register."]
pub type PhyGrpShiftObsSelectR = crate::FieldReader;
#[doc = "Field `PHY_GRP_SHIFT_OBS_SELECT` writer - Select value to map an individual address/control group slice automatic cycle/half_cycle shift settings to the observation register."]
pub type PhyGrpShiftObsSelectW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - Defines the slice numbers associated with each CS during CA training."]
    #[inline(always)]
    pub fn phy_calvl_cs_map(&self) -> PhyCalvlCsMapR {
        PhyCalvlCsMapR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:17 - Select value to map an individual address/control group slice slave delay to the encoded value observation register."]
    #[inline(always)]
    pub fn phy_grp_slv_dly_enc_obs_select(&self) -> PhyGrpSlvDlyEncObsSelectR {
        PhyGrpSlvDlyEncObsSelectR::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:28 - Select value to map an individual address/control group slice automatic cycle/half_cycle shift settings to the observation register."]
    #[inline(always)]
    pub fn phy_grp_shift_obs_select(&self) -> PhyGrpShiftObsSelectR {
        PhyGrpShiftObsSelectR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Defines the slice numbers associated with each CS during CA training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_calvl_cs_map(&mut self) -> PhyCalvlCsMapW<DdrDenaliPhy905Spec> {
        PhyCalvlCsMapW::new(self, 0)
    }
    #[doc = "Bits 8:17 - Select value to map an individual address/control group slice slave delay to the encoded value observation register."]
    #[inline(always)]
    #[must_use]
    pub fn phy_grp_slv_dly_enc_obs_select(
        &mut self,
    ) -> PhyGrpSlvDlyEncObsSelectW<DdrDenaliPhy905Spec> {
        PhyGrpSlvDlyEncObsSelectW::new(self, 8)
    }
    #[doc = "Bits 24:28 - Select value to map an individual address/control group slice automatic cycle/half_cycle shift settings to the observation register."]
    #[inline(always)]
    #[must_use]
    pub fn phy_grp_shift_obs_select(&mut self) -> PhyGrpShiftObsSelectW<DdrDenaliPhy905Spec> {
        PhyGrpShiftObsSelectW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_905::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_905::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy905Spec;
impl crate::RegisterSpec for DdrDenaliPhy905Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_905::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy905Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_905::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy905Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_905 to value 0"]
impl crate::Resettable for DdrDenaliPhy905Spec {
    const RESET_VALUE: u32 = 0;
}
