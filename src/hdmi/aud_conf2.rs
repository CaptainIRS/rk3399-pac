#[doc = "Register `AUD_CONF2` reader"]
pub type R = crate::R<AudConf2Spec>;
#[doc = "Register `AUD_CONF2` writer"]
pub type W = crate::W<AudConf2Spec>;
#[doc = "Field `HBR` reader - I2S HBR Mode Enable. When enabled, the I2S audio\n\nstream is transmitted using HBR packets."]
pub type HbrR = crate::BitReader;
#[doc = "Field `HBR` writer - I2S HBR Mode Enable. When enabled, the I2S audio\n\nstream is transmitted using HBR packets."]
pub type HbrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NLPCM` reader - I2S NLPCM Mode Enable. When enabled, this bit\n\nassumes that PCUV data is included on the I2S audio\n\nstream according to the description located in the\n\n\"I2S Interface\" section of Chapter 2, \"Functional\n\nDescription.\""]
pub type NlpcmR = crate::BitReader;
#[doc = "Field `NLPCM` writer - I2S NLPCM Mode Enable. When enabled, this bit\n\nassumes that PCUV data is included on the I2S audio\n\nstream according to the description located in the\n\n\"I2S Interface\" section of Chapter 2, \"Functional\n\nDescription.\""]
pub type NlpcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSERT_PCUV` reader - When set (1'b1), this bit enables the insertion of the\n\nPCUV (Parity, Channel Status, User bit and Validity)\n\nbits on the incoming audio stream (support limited to\n\nLinear PCM audio). If disabled, the incoming audio\n\nstream must contain the PCUV bits, mapped\n\naccording to Databook."]
pub type InsertPcuvR = crate::BitReader;
#[doc = "Field `INSERT_PCUV` writer - When set (1'b1), this bit enables the insertion of the\n\nPCUV (Parity, Channel Status, User bit and Validity)\n\nbits on the incoming audio stream (support limited to\n\nLinear PCM audio). If disabled, the incoming audio\n\nstream must contain the PCUV bits, mapped\n\naccording to Databook."]
pub type InsertPcuvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2S HBR Mode Enable. When enabled, the I2S audio\n\nstream is transmitted using HBR packets."]
    #[inline(always)]
    pub fn hbr(&self) -> HbrR {
        HbrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2S NLPCM Mode Enable. When enabled, this bit\n\nassumes that PCUV data is included on the I2S audio\n\nstream according to the description located in the\n\n\"I2S Interface\" section of Chapter 2, \"Functional\n\nDescription.\""]
    #[inline(always)]
    pub fn nlpcm(&self) -> NlpcmR {
        NlpcmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set (1'b1), this bit enables the insertion of the\n\nPCUV (Parity, Channel Status, User bit and Validity)\n\nbits on the incoming audio stream (support limited to\n\nLinear PCM audio). If disabled, the incoming audio\n\nstream must contain the PCUV bits, mapped\n\naccording to Databook."]
    #[inline(always)]
    pub fn insert_pcuv(&self) -> InsertPcuvR {
        InsertPcuvR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2S HBR Mode Enable. When enabled, the I2S audio\n\nstream is transmitted using HBR packets."]
    #[inline(always)]
    #[must_use]
    pub fn hbr(&mut self) -> HbrW<AudConf2Spec> {
        HbrW::new(self, 0)
    }
    #[doc = "Bit 1 - I2S NLPCM Mode Enable. When enabled, this bit\n\nassumes that PCUV data is included on the I2S audio\n\nstream according to the description located in the\n\n\"I2S Interface\" section of Chapter 2, \"Functional\n\nDescription.\""]
    #[inline(always)]
    #[must_use]
    pub fn nlpcm(&mut self) -> NlpcmW<AudConf2Spec> {
        NlpcmW::new(self, 1)
    }
    #[doc = "Bit 2 - When set (1'b1), this bit enables the insertion of the\n\nPCUV (Parity, Channel Status, User bit and Validity)\n\nbits on the incoming audio stream (support limited to\n\nLinear PCM audio). If disabled, the incoming audio\n\nstream must contain the PCUV bits, mapped\n\naccording to Databook."]
    #[inline(always)]
    #[must_use]
    pub fn insert_pcuv(&mut self) -> InsertPcuvW<AudConf2Spec> {
        InsertPcuvW::new(self, 2)
    }
}
#[doc = "Audio I2S PCUV, NLPCM and HBR configuration Register 2\n\nThis register configures the I2S Audio Data mapping. By default, audio data mapping is the\n\nstandard I2S Linear PCM (L-PCM) mapping. You can choose to use the I2S interface to\n\ntransport HBR or Non- Linear PCM (NL-PCM) audio, by setting the relevant bit in this\n\nregister.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aud_conf2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aud_conf2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudConf2Spec;
impl crate::RegisterSpec for AudConf2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aud_conf2::R`](R) reader structure"]
impl crate::Readable for AudConf2Spec {}
#[doc = "`write(|w| ..)` method takes [`aud_conf2::W`](W) writer structure"]
impl crate::Writable for AudConf2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AUD_CONF2 to value 0x04"]
impl crate::Resettable for AudConf2Spec {
    const RESET_VALUE: u8 = 0x04;
}
