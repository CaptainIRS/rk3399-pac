#[doc = "Register `FC_GMD_UP` writer"]
pub type W = crate::W<FcGmdUpSpec>;
#[doc = "Field `GMDUPDATEPACKET` writer - Gamut Metadata packet update"]
pub type GmdupdatepacketW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Gamut Metadata packet update"]
    #[inline(always)]
    #[must_use]
    pub fn gmdupdatepacket(&mut self) -> GmdupdatepacketW<FcGmdUpSpec> {
        GmdupdatepacketW::new(self, 0)
    }
}
#[doc = "Frame Composer GMD Packet Update Register\n\nThis register performs an GMD packet content update according to the configured packet\n\nbody (FC_GMD_PB0 to FC_GMD_PB27) and packet header (FC_GMD_HB). This active high\n\nauto clear register reflects the body and header configurations on the GMD packets sent\n\narbitrating the current_gamut_seq_num, gmd_packet_sequence and next_gmd_field bits\n\non packet to correctly indicate to source the Gamut change to be performed. After enable\n\nGMD packets the first update request is also responsible for deactivating the\n\nno_current_gmd indication bit.\n\nAttention packet update request must only be done after correct configuration of GMD\n\npacket body and header registers. Correct affected_gamut_seq_num and gmd_profile\n\nconfiguration is user responsibility and must convey with HDMI 1.4b standard gamut rules.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_gmd_up::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcGmdUpSpec;
impl crate::RegisterSpec for FcGmdUpSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`fc_gmd_up::W`](W) writer structure"]
impl crate::Writable for FcGmdUpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_GMD_UP to value 0"]
impl crate::Resettable for FcGmdUpSpec {
    const RESET_VALUE: u8 = 0;
}
