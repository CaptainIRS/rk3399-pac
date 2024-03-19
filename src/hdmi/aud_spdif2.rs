#[doc = "Register `AUD_SPDIF2` reader"]
pub type R = crate::R<AudSpdif2Spec>;
#[doc = "Register `AUD_SPDIF2` writer"]
pub type W = crate::W<AudSpdif2Spec>;
#[doc = "Field `SPDIF_IN_EN` reader - Action\n\nSPDIF_in_en\\[0\\]
- ispdifdata\\[0\\]
enable SPDIF_in_en\\[1\\]\n\n- ispdifdata\\[1\\]
enable SPDIF_in_en\\[2\\]
- ispdifdata\\[2\\]\n\nenable SPDIF_in_en\\[3\\]
- ispdifdata\\[3\\]
enable"]
pub type SpdifInEnR = crate::FieldReader;
#[doc = "Field `SPDIF_IN_EN` writer - Action\n\nSPDIF_in_en\\[0\\]
- ispdifdata\\[0\\]
enable SPDIF_in_en\\[1\\]\n\n- ispdifdata\\[1\\]
enable SPDIF_in_en\\[2\\]
- ispdifdata\\[2\\]\n\nenable SPDIF_in_en\\[3\\]
- ispdifdata\\[3\\]
enable"]
pub type SpdifInEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Action\n\nSPDIF_in_en\\[0\\]
- ispdifdata\\[0\\]
enable SPDIF_in_en\\[1\\]\n\n- ispdifdata\\[1\\]
enable SPDIF_in_en\\[2\\]
- ispdifdata\\[2\\]\n\nenable SPDIF_in_en\\[3\\]
- ispdifdata\\[3\\]
enable"]
    #[inline(always)]
    pub fn spdif_in_en(&self) -> SpdifInEnR {
        SpdifInEnR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Action\n\nSPDIF_in_en\\[0\\]
- ispdifdata\\[0\\]
enable SPDIF_in_en\\[1\\]\n\n- ispdifdata\\[1\\]
enable SPDIF_in_en\\[2\\]
- ispdifdata\\[2\\]\n\nenable SPDIF_in_en\\[3\\]
- ispdifdata\\[3\\]
enable"]
    #[inline(always)]
    #[must_use]
    pub fn spdif_in_en(&mut self) -> SpdifInEnW<AudSpdif2Spec> {
        SpdifInEnW::new(self, 0)
    }
}
#[doc = "Audio SPDIF Enable Confiiguration Register 2\n\nThis register configures the SPDIF input enable that indicates which input SPDIF channels\n\nhave valid data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_spdif2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_spdif2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudSpdif2Spec;
impl crate::RegisterSpec for AudSpdif2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aud_spdif2::R`](R) reader structure"]
impl crate::Readable for AudSpdif2Spec {}
#[doc = "`write(|w| ..)` method takes [`aud_spdif2::W`](W) writer structure"]
impl crate::Writable for AudSpdif2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AUD_SPDIF2 to value 0x01"]
impl crate::Resettable for AudSpdif2Spec {
    const RESET_VALUE: u8 = 0x01;
}
