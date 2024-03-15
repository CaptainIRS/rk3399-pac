#[doc = "Register `FC_MULTISTREAM_CTRL` reader"]
pub type R = crate::R<FcMultistreamCtrlSpec>;
#[doc = "Register `FC_MULTISTREAM_CTRL` writer"]
pub type W = crate::W<FcMultistreamCtrlSpec>;
#[doc = "Field `FC_MAS_PACKET_EN` reader - This field, when set (1'b1), activates the HDMI 2.0 Multi- Stream support. The audio stream present at the input of the Hdmi_tx controller is transported using Multi-Stream Audio Sample Packets."]
pub type FcMasPacketEnR = crate::BitReader;
#[doc = "Field `FC_MAS_PACKET_EN` writer - This field, when set (1'b1), activates the HDMI 2.0 Multi- Stream support. The audio stream present at the input of the Hdmi_tx controller is transported using Multi-Stream Audio Sample Packets."]
pub type FcMasPacketEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This field, when set (1'b1), activates the HDMI 2.0 Multi- Stream support. The audio stream present at the input of the Hdmi_tx controller is transported using Multi-Stream Audio Sample Packets."]
    #[inline(always)]
    pub fn fc_mas_packet_en(&self) -> FcMasPacketEnR {
        FcMasPacketEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This field, when set (1'b1), activates the HDMI 2.0 Multi- Stream support. The audio stream present at the input of the Hdmi_tx controller is transported using Multi-Stream Audio Sample Packets."]
    #[inline(always)]
    #[must_use]
    pub fn fc_mas_packet_en(&mut self) -> FcMasPacketEnW<FcMultistreamCtrlSpec> {
        FcMasPacketEnW::new(self, 0)
    }
}
#[doc = "This field, when set (1'b1), activates the HDMI 2.0 Multi- Stream support. The audio stream present at the input of the Hdmi_tx controller is transported using Multi-Stream Audio Sample Packets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_multistream_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_multistream_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcMultistreamCtrlSpec;
impl crate::RegisterSpec for FcMultistreamCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_multistream_ctrl::R`](R) reader structure"]
impl crate::Readable for FcMultistreamCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_multistream_ctrl::W`](W) writer structure"]
impl crate::Writable for FcMultistreamCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_MULTISTREAM_CTRL to value 0"]
impl crate::Resettable for FcMultistreamCtrlSpec {
    const RESET_VALUE: u8 = 0;
}
