#[doc = "Register `FC_AUDSCONF` reader"]
pub type R = crate::R<FcAudsconfSpec>;
#[doc = "Register `FC_AUDSCONF` writer"]
pub type W = crate::W<FcAudsconfSpec>;
#[doc = "Field `AUD_PACKET_LAYOUT` reader - Set the audio packet layout to be sent in the\n\npacket: 1b: layout 1\n\n0b: layout 0\n\nIf HDMI_TX_20 is defined and register field\n\nfc_multistream_ctrl.fc_mas_packet_en is active,\n\nthis bit has no effect."]
pub type AudPacketLayoutR = crate::BitReader;
#[doc = "Field `AUD_PACKET_LAYOUT` writer - Set the audio packet layout to be sent in the\n\npacket: 1b: layout 1\n\n0b: layout 0\n\nIf HDMI_TX_20 is defined and register field\n\nfc_multistream_ctrl.fc_mas_packet_en is active,\n\nthis bit has no effect."]
pub type AudPacketLayoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUD_PACKET_SAMPFLT` reader - Set the audio packet sample flat value to be sent\n\non the packet."]
pub type AudPacketSampfltR = crate::FieldReader;
#[doc = "Field `AUD_PACKET_SAMPFLT` writer - Set the audio packet sample flat value to be sent\n\non the packet."]
pub type AudPacketSampfltW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Set the audio packet layout to be sent in the\n\npacket: 1b: layout 1\n\n0b: layout 0\n\nIf HDMI_TX_20 is defined and register field\n\nfc_multistream_ctrl.fc_mas_packet_en is active,\n\nthis bit has no effect."]
    #[inline(always)]
    pub fn aud_packet_layout(&self) -> AudPacketLayoutR {
        AudPacketLayoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Set the audio packet sample flat value to be sent\n\non the packet."]
    #[inline(always)]
    pub fn aud_packet_sampflt(&self) -> AudPacketSampfltR {
        AudPacketSampfltR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bit 0 - Set the audio packet layout to be sent in the\n\npacket: 1b: layout 1\n\n0b: layout 0\n\nIf HDMI_TX_20 is defined and register field\n\nfc_multistream_ctrl.fc_mas_packet_en is active,\n\nthis bit has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn aud_packet_layout(&mut self) -> AudPacketLayoutW<FcAudsconfSpec> {
        AudPacketLayoutW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Set the audio packet sample flat value to be sent\n\non the packet."]
    #[inline(always)]
    #[must_use]
    pub fn aud_packet_sampflt(&mut self) -> AudPacketSampfltW<FcAudsconfSpec> {
        AudPacketSampfltW::new(self, 4)
    }
}
#[doc = "Frame Composer Audio Sample Flat and Layout Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audsconf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audsconf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAudsconfSpec;
impl crate::RegisterSpec for FcAudsconfSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_audsconf::R`](R) reader structure"]
impl crate::Readable for FcAudsconfSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_audsconf::W`](W) writer structure"]
impl crate::Writable for FcAudsconfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AUDSCONF to value 0"]
impl crate::Resettable for FcAudsconfSpec {
    const RESET_VALUE: u8 = 0;
}
