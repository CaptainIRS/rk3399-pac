#[doc = "Register `GP_CONF2` reader"]
pub type R = crate::R<GpConf2Spec>;
#[doc = "Register `GP_CONF2` writer"]
pub type W = crate::W<GpConf2Spec>;
#[doc = "Field `HBR` reader - HBR packets enable. The Hdmi_tx sends the HBR\n\npackets. This bit is enabled when the audio frequency\n\nis higher than 192 kHz. If this bit is enabled, the\n\nnumber of channels configured in GP_CONF1 must be\n\nset to 8'hFF."]
pub type HbrR = crate::BitReader;
#[doc = "Field `HBR` writer - HBR packets enable. The Hdmi_tx sends the HBR\n\npackets. This bit is enabled when the audio frequency\n\nis higher than 192 kHz. If this bit is enabled, the\n\nnumber of channels configured in GP_CONF1 must be\n\nset to 8'hFF."]
pub type HbrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSERT_PCUV` reader - When set (1'b1), this bit enables the insertion of the\n\nPCUV (Parity, Channel Status, User bit and Validity)\n\nbits on the incoming audio stream (support limited to\n\nLinear PCM audio). If disabled, the incoming audio\n\nstream must contain the PCUV bits, mapped according\n\nto 2.6.4.2 Data Mapping Examples."]
pub type InsertPcuvR = crate::BitReader;
#[doc = "Field `INSERT_PCUV` writer - When set (1'b1), this bit enables the insertion of the\n\nPCUV (Parity, Channel Status, User bit and Validity)\n\nbits on the incoming audio stream (support limited to\n\nLinear PCM audio). If disabled, the incoming audio\n\nstream must contain the PCUV bits, mapped according\n\nto 2.6.4.2 Data Mapping Examples."]
pub type InsertPcuvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HBR packets enable. The Hdmi_tx sends the HBR\n\npackets. This bit is enabled when the audio frequency\n\nis higher than 192 kHz. If this bit is enabled, the\n\nnumber of channels configured in GP_CONF1 must be\n\nset to 8'hFF."]
    #[inline(always)]
    pub fn hbr(&self) -> HbrR {
        HbrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set (1'b1), this bit enables the insertion of the\n\nPCUV (Parity, Channel Status, User bit and Validity)\n\nbits on the incoming audio stream (support limited to\n\nLinear PCM audio). If disabled, the incoming audio\n\nstream must contain the PCUV bits, mapped according\n\nto 2.6.4.2 Data Mapping Examples."]
    #[inline(always)]
    pub fn insert_pcuv(&self) -> InsertPcuvR {
        InsertPcuvR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HBR packets enable. The Hdmi_tx sends the HBR\n\npackets. This bit is enabled when the audio frequency\n\nis higher than 192 kHz. If this bit is enabled, the\n\nnumber of channels configured in GP_CONF1 must be\n\nset to 8'hFF."]
    #[inline(always)]
    #[must_use]
    pub fn hbr(&mut self) -> HbrW<GpConf2Spec> {
        HbrW::new(self, 0)
    }
    #[doc = "Bit 1 - When set (1'b1), this bit enables the insertion of the\n\nPCUV (Parity, Channel Status, User bit and Validity)\n\nbits on the incoming audio stream (support limited to\n\nLinear PCM audio). If disabled, the incoming audio\n\nstream must contain the PCUV bits, mapped according\n\nto 2.6.4.2 Data Mapping Examples."]
    #[inline(always)]
    #[must_use]
    pub fn insert_pcuv(&mut self) -> InsertPcuvW<GpConf2Spec> {
        InsertPcuvW::new(self, 1)
    }
}
#[doc = "Audio GPA HBR Enable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_conf2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_conf2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpConf2Spec;
impl crate::RegisterSpec for GpConf2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gp_conf2::R`](R) reader structure"]
impl crate::Readable for GpConf2Spec {}
#[doc = "`write(|w| ..)` method takes [`gp_conf2::W`](W) writer structure"]
impl crate::Writable for GpConf2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets GP_CONF2 to value 0"]
impl crate::Resettable for GpConf2Spec {
    const RESET_VALUE: u8 = 0;
}
