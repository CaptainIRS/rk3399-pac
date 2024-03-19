#[doc = "Register `FC_DRM_UP` writer"]
pub type W = crate::W<FcDrmUpSpec>;
#[doc = "Field `DRMPACKETUPDATE` writer - DRM packet update"]
pub type DrmpacketupdateW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DRM packet update"]
    #[inline(always)]
    #[must_use]
    pub fn drmpacketupdate(&mut self) -> DrmpacketupdateW<FcDrmUpSpec> {
        DrmpacketupdateW::new(self, 0)
    }
}
#[doc = "Frame Composer DRM Packet Update Register\n\nThis register performs an DRM packet content update according to the configured packet\n\nbody (FC_DRM_PB0 to FC_DRM_PB27) and packet header (FC_DRM_HB). This active high\n\nauto clear register reflects the body and header configurations on the DRM packets change\n\nto be performed.\n\nAttention packet update request must only be done after correct configuration of DRM\n\npacket body and header registers.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_drm_up::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDrmUpSpec;
impl crate::RegisterSpec for FcDrmUpSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`fc_drm_up::W`](W) writer structure"]
impl crate::Writable for FcDrmUpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_DRM_UP to value 0"]
impl crate::Resettable for FcDrmUpSpec {
    const RESET_VALUE: u8 = 0;
}
