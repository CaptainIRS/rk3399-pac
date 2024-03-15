#[doc = "Register `FC_GMD_HB` reader"]
pub type R = crate::R<FcGmdHbSpec>;
#[doc = "Register `FC_GMD_HB` writer"]
pub type W = crate::W<FcGmdHbSpec>;
#[doc = "Field `GMDAFFECTED_GAMUT_SEQ_NUM` reader - Affected gamut sequence number"]
pub type GmdaffectedGamutSeqNumR = crate::FieldReader;
#[doc = "Field `GMDAFFECTED_GAMUT_SEQ_NUM` writer - Affected gamut sequence number"]
pub type GmdaffectedGamutSeqNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GMDGBD_PROFILE` reader - GMD profile bits. Hdmi_tx only supports Profile 0 (P0) of the Gamut Boundary Description Metadata Profiles described in the HDMI 1.4 Specification (which defines four profiles, P0-P4)."]
pub type GmdgbdProfileR = crate::FieldReader;
#[doc = "Field `GMDGBD_PROFILE` writer - GMD profile bits. Hdmi_tx only supports Profile 0 (P0) of the Gamut Boundary Description Metadata Profiles described in the HDMI 1.4 Specification (which defines four profiles, P0-P4)."]
pub type GmdgbdProfileW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Affected gamut sequence number"]
    #[inline(always)]
    pub fn gmdaffected_gamut_seq_num(&self) -> GmdaffectedGamutSeqNumR {
        GmdaffectedGamutSeqNumR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - GMD profile bits. Hdmi_tx only supports Profile 0 (P0) of the Gamut Boundary Description Metadata Profiles described in the HDMI 1.4 Specification (which defines four profiles, P0-P4)."]
    #[inline(always)]
    pub fn gmdgbd_profile(&self) -> GmdgbdProfileR {
        GmdgbdProfileR::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - Affected gamut sequence number"]
    #[inline(always)]
    #[must_use]
    pub fn gmdaffected_gamut_seq_num(&mut self) -> GmdaffectedGamutSeqNumW<FcGmdHbSpec> {
        GmdaffectedGamutSeqNumW::new(self, 0)
    }
    #[doc = "Bits 4:6 - GMD profile bits. Hdmi_tx only supports Profile 0 (P0) of the Gamut Boundary Description Metadata Profiles described in the HDMI 1.4 Specification (which defines four profiles, P0-P4)."]
    #[inline(always)]
    #[must_use]
    pub fn gmdgbd_profile(&mut self) -> GmdgbdProfileW<FcGmdHbSpec> {
        GmdgbdProfileW::new(self, 4)
    }
}
#[doc = "Affected gamut sequence number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_gmd_hb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_gmd_hb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcGmdHbSpec;
impl crate::RegisterSpec for FcGmdHbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_gmd_hb::R`](R) reader structure"]
impl crate::Readable for FcGmdHbSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_gmd_hb::W`](W) writer structure"]
impl crate::Writable for FcGmdHbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_GMD_HB to value 0"]
impl crate::Resettable for FcGmdHbSpec {
    const RESET_VALUE: u8 = 0;
}
