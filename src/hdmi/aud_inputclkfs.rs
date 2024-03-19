#[doc = "Register `AUD_INPUTCLKFS` reader"]
pub type R = crate::R<AudInputclkfsSpec>;
#[doc = "Register `AUD_INPUTCLKFS` writer"]
pub type W = crate::W<AudInputclkfsSpec>;
#[doc = "Field `IFSFACTOR` reader - Fs factor configuration:\n\nifsfactor\\[2:0\\]
| Audio Clock | Action\n\n0 | 128xFs | If you select the Bypass SPDIF\n\nDRU unit in coreConsultant, the input audio clock\n\n(either I2S or SPDIF according to configuration) is\n\nused at the audio packetizer to calculate the CTS\n\nvalue and ACR packet insertion rate.\n\n| 256xFs | The input audio clock (I2S only) is\n\ndivided by 2 and then used at audio packetizer to\n\ncalculate the CTS value and ACR packet insertion\n\nrate.\n\n| 512xFs | The input audio clock (either I2S or\n\nSPDIF according to configuration) used divided by 4\n\nand then used at the audio packetizer to calculate the\n\nCTS value and ACR packet insertion rate.\n\nNote: When the SPDIF interface is receiving an HBR\n\naudio stream (\"Support for HBR over SDPIF\"\n\nparameter must be enabled), it is required that the\n\nselected IFSFACTOR to be set at 512xFs in order to\n\ncomply with the HDMI ACR requirements for HBR\n\naudio streams.\n\n| Reserved\n\n| 64xFs | The input audio clock (I2S only) is\n\nmultiplied by 2 and then used at the audio packetizer\n\nto calculate the CTS value and ACR packet insertion\n\nrate.\n\nothers | 128xFs | If you select the Bypass SPDIF\n\nDRU unit in coreConsultant, the input audio clock\n\n(either I2S or SPDIF according to configuration) is\n\nused at the audio packetizer to calculate the CTS\n\nvalue and ACR packet insertion rate.\n\nThe SPDIF interface, for non HBR audio, requires that\n\nthe configured oversampling value to be 128xFs when\n\nHTX_SPDIFBYPDRU is enabled and 512xFs if not.\n\nWhen the SPDIF interface is receiving HBR audio\n\n(HBR_ON_SPDIF must be enabled), in order to\n\ncomply with the HDMI ACR requirements for HBR\n\naudio streams."]
pub type IfsfactorR = crate::FieldReader;
#[doc = "Field `IFSFACTOR` writer - Fs factor configuration:\n\nifsfactor\\[2:0\\]
| Audio Clock | Action\n\n0 | 128xFs | If you select the Bypass SPDIF\n\nDRU unit in coreConsultant, the input audio clock\n\n(either I2S or SPDIF according to configuration) is\n\nused at the audio packetizer to calculate the CTS\n\nvalue and ACR packet insertion rate.\n\n| 256xFs | The input audio clock (I2S only) is\n\ndivided by 2 and then used at audio packetizer to\n\ncalculate the CTS value and ACR packet insertion\n\nrate.\n\n| 512xFs | The input audio clock (either I2S or\n\nSPDIF according to configuration) used divided by 4\n\nand then used at the audio packetizer to calculate the\n\nCTS value and ACR packet insertion rate.\n\nNote: When the SPDIF interface is receiving an HBR\n\naudio stream (\"Support for HBR over SDPIF\"\n\nparameter must be enabled), it is required that the\n\nselected IFSFACTOR to be set at 512xFs in order to\n\ncomply with the HDMI ACR requirements for HBR\n\naudio streams.\n\n| Reserved\n\n| 64xFs | The input audio clock (I2S only) is\n\nmultiplied by 2 and then used at the audio packetizer\n\nto calculate the CTS value and ACR packet insertion\n\nrate.\n\nothers | 128xFs | If you select the Bypass SPDIF\n\nDRU unit in coreConsultant, the input audio clock\n\n(either I2S or SPDIF according to configuration) is\n\nused at the audio packetizer to calculate the CTS\n\nvalue and ACR packet insertion rate.\n\nThe SPDIF interface, for non HBR audio, requires that\n\nthe configured oversampling value to be 128xFs when\n\nHTX_SPDIFBYPDRU is enabled and 512xFs if not.\n\nWhen the SPDIF interface is receiving HBR audio\n\n(HBR_ON_SPDIF must be enabled), in order to\n\ncomply with the HDMI ACR requirements for HBR\n\naudio streams."]
pub type IfsfactorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Fs factor configuration:\n\nifsfactor\\[2:0\\]
| Audio Clock | Action\n\n0 | 128xFs | If you select the Bypass SPDIF\n\nDRU unit in coreConsultant, the input audio clock\n\n(either I2S or SPDIF according to configuration) is\n\nused at the audio packetizer to calculate the CTS\n\nvalue and ACR packet insertion rate.\n\n| 256xFs | The input audio clock (I2S only) is\n\ndivided by 2 and then used at audio packetizer to\n\ncalculate the CTS value and ACR packet insertion\n\nrate.\n\n| 512xFs | The input audio clock (either I2S or\n\nSPDIF according to configuration) used divided by 4\n\nand then used at the audio packetizer to calculate the\n\nCTS value and ACR packet insertion rate.\n\nNote: When the SPDIF interface is receiving an HBR\n\naudio stream (\"Support for HBR over SDPIF\"\n\nparameter must be enabled), it is required that the\n\nselected IFSFACTOR to be set at 512xFs in order to\n\ncomply with the HDMI ACR requirements for HBR\n\naudio streams.\n\n| Reserved\n\n| 64xFs | The input audio clock (I2S only) is\n\nmultiplied by 2 and then used at the audio packetizer\n\nto calculate the CTS value and ACR packet insertion\n\nrate.\n\nothers | 128xFs | If you select the Bypass SPDIF\n\nDRU unit in coreConsultant, the input audio clock\n\n(either I2S or SPDIF according to configuration) is\n\nused at the audio packetizer to calculate the CTS\n\nvalue and ACR packet insertion rate.\n\nThe SPDIF interface, for non HBR audio, requires that\n\nthe configured oversampling value to be 128xFs when\n\nHTX_SPDIFBYPDRU is enabled and 512xFs if not.\n\nWhen the SPDIF interface is receiving HBR audio\n\n(HBR_ON_SPDIF must be enabled), in order to\n\ncomply with the HDMI ACR requirements for HBR\n\naudio streams."]
    #[inline(always)]
    pub fn ifsfactor(&self) -> IfsfactorR {
        IfsfactorR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Fs factor configuration:\n\nifsfactor\\[2:0\\]
| Audio Clock | Action\n\n0 | 128xFs | If you select the Bypass SPDIF\n\nDRU unit in coreConsultant, the input audio clock\n\n(either I2S or SPDIF according to configuration) is\n\nused at the audio packetizer to calculate the CTS\n\nvalue and ACR packet insertion rate.\n\n| 256xFs | The input audio clock (I2S only) is\n\ndivided by 2 and then used at audio packetizer to\n\ncalculate the CTS value and ACR packet insertion\n\nrate.\n\n| 512xFs | The input audio clock (either I2S or\n\nSPDIF according to configuration) used divided by 4\n\nand then used at the audio packetizer to calculate the\n\nCTS value and ACR packet insertion rate.\n\nNote: When the SPDIF interface is receiving an HBR\n\naudio stream (\"Support for HBR over SDPIF\"\n\nparameter must be enabled), it is required that the\n\nselected IFSFACTOR to be set at 512xFs in order to\n\ncomply with the HDMI ACR requirements for HBR\n\naudio streams.\n\n| Reserved\n\n| 64xFs | The input audio clock (I2S only) is\n\nmultiplied by 2 and then used at the audio packetizer\n\nto calculate the CTS value and ACR packet insertion\n\nrate.\n\nothers | 128xFs | If you select the Bypass SPDIF\n\nDRU unit in coreConsultant, the input audio clock\n\n(either I2S or SPDIF according to configuration) is\n\nused at the audio packetizer to calculate the CTS\n\nvalue and ACR packet insertion rate.\n\nThe SPDIF interface, for non HBR audio, requires that\n\nthe configured oversampling value to be 128xFs when\n\nHTX_SPDIFBYPDRU is enabled and 512xFs if not.\n\nWhen the SPDIF interface is receiving HBR audio\n\n(HBR_ON_SPDIF must be enabled), in order to\n\ncomply with the HDMI ACR requirements for HBR\n\naudio streams."]
    #[inline(always)]
    #[must_use]
    pub fn ifsfactor(&mut self) -> IfsfactorW<AudInputclkfsSpec> {
        IfsfactorW::new(self, 0)
    }
}
#[doc = "Audio Input Clock FS Factor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_inputclkfs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_inputclkfs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudInputclkfsSpec;
impl crate::RegisterSpec for AudInputclkfsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aud_inputclkfs::R`](R) reader structure"]
impl crate::Readable for AudInputclkfsSpec {}
#[doc = "`write(|w| ..)` method takes [`aud_inputclkfs::W`](W) writer structure"]
impl crate::Writable for AudInputclkfsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AUD_INPUTCLKFS to value 0"]
impl crate::Resettable for AudInputclkfsSpec {
    const RESET_VALUE: u8 = 0;
}
