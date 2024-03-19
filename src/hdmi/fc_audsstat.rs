#[doc = "Register `FC_AUDSSTAT` reader"]
pub type R = crate::R<FcAudsstatSpec>;
#[doc = "Field `PACKET_SAMPPRS` reader - Shows the data sample present indication of the\n\nlast Audio sample packet sent by the HDMI Tx\n\nController. This register information is at TMDS\n\nclock rate."]
pub type PacketSampprsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Shows the data sample present indication of the\n\nlast Audio sample packet sent by the HDMI Tx\n\nController. This register information is at TMDS\n\nclock rate."]
    #[inline(always)]
    pub fn packet_sampprs(&self) -> PacketSampprsR {
        PacketSampprsR::new(self.bits & 0x0f)
    }
}
#[doc = "Frame Composer Audio Sample Flat and Layout Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audsstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAudsstatSpec;
impl crate::RegisterSpec for FcAudsstatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_audsstat::R`](R) reader structure"]
impl crate::Readable for FcAudsstatSpec {}
#[doc = "`reset()` method sets FC_AUDSSTAT to value 0"]
impl crate::Resettable for FcAudsstatSpec {
    const RESET_VALUE: u8 = 0;
}
