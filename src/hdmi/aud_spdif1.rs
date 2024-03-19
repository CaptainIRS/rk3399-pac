#[doc = "Register `AUD_SPDIF1` reader"]
pub type R = crate::R<AudSpdif1Spec>;
#[doc = "Register `AUD_SPDIF1` writer"]
pub type W = crate::W<AudSpdif1Spec>;
#[doc = "Field `SPDIF_HBR_MODE` reader - When set to 1'b1, this bit field indicates that the input\n\nstream has a High Bit Rate (HBR) to be transmitted in\n\nHDMI HBR packets. When clear (1b'0), the audio is\n\ntransmitted in HDMI AUDS packets.Note:&lt; Otherwise,\n\nthis field is a \"spare\" bit with no associated\n\nfunctionality."]
pub type SpdifHbrModeR = crate::BitReader;
#[doc = "Field `SPDIF_HBR_MODE` writer - When set to 1'b1, this bit field indicates that the input\n\nstream has a High Bit Rate (HBR) to be transmitted in\n\nHDMI HBR packets. When clear (1b'0), the audio is\n\ntransmitted in HDMI AUDS packets.Note:&lt; Otherwise,\n\nthis field is a \"spare\" bit with no associated\n\nfunctionality."]
pub type SpdifHbrModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETNLPCM` reader - Select Non-Linear (1b) / Linear (0b) PCM mode"]
pub type SetnlpcmR = crate::BitReader;
#[doc = "Field `SETNLPCM` writer - Select Non-Linear (1b) / Linear (0b) PCM mode"]
pub type SetnlpcmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - When set to 1'b1, this bit field indicates that the input\n\nstream has a High Bit Rate (HBR) to be transmitted in\n\nHDMI HBR packets. When clear (1b'0), the audio is\n\ntransmitted in HDMI AUDS packets.Note:&lt; Otherwise,\n\nthis field is a \"spare\" bit with no associated\n\nfunctionality."]
    #[inline(always)]
    pub fn spdif_hbr_mode(&self) -> SpdifHbrModeR {
        SpdifHbrModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Select Non-Linear (1b) / Linear (0b) PCM mode"]
    #[inline(always)]
    pub fn setnlpcm(&self) -> SetnlpcmR {
        SetnlpcmR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - When set to 1'b1, this bit field indicates that the input\n\nstream has a High Bit Rate (HBR) to be transmitted in\n\nHDMI HBR packets. When clear (1b'0), the audio is\n\ntransmitted in HDMI AUDS packets.Note:&lt; Otherwise,\n\nthis field is a \"spare\" bit with no associated\n\nfunctionality."]
    #[inline(always)]
    #[must_use]
    pub fn spdif_hbr_mode(&mut self) -> SpdifHbrModeW<AudSpdif1Spec> {
        SpdifHbrModeW::new(self, 6)
    }
    #[doc = "Bit 7 - Select Non-Linear (1b) / Linear (0b) PCM mode"]
    #[inline(always)]
    #[must_use]
    pub fn setnlpcm(&mut self) -> SetnlpcmW<AudSpdif1Spec> {
        SetnlpcmW::new(self, 7)
    }
}
#[doc = "Audio SPDIF NLPCM and Width Configuration Register 1 This register\n\nconfigures the SPDIF data width.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_spdif1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_spdif1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudSpdif1Spec;
impl crate::RegisterSpec for AudSpdif1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aud_spdif1::R`](R) reader structure"]
impl crate::Readable for AudSpdif1Spec {}
#[doc = "`write(|w| ..)` method takes [`aud_spdif1::W`](W) writer structure"]
impl crate::Writable for AudSpdif1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AUD_SPDIF1 to value 0"]
impl crate::Resettable for AudSpdif1Spec {
    const RESET_VALUE: u8 = 0;
}
