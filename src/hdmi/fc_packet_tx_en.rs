#[doc = "Register `FC_PACKET_TX_EN` reader"]
pub type R = crate::R<FcPacketTxEnSpec>;
#[doc = "Register `FC_PACKET_TX_EN` writer"]
pub type W = crate::W<FcPacketTxEnSpec>;
#[doc = "Field `ACR_TX_EN` reader - ACR packet transmission control 1b: Transmission\n\nenabled\n\n0b: Transmission disabled"]
pub type AcrTxEnR = crate::BitReader;
#[doc = "Field `ACR_TX_EN` writer - ACR packet transmission control 1b: Transmission\n\nenabled\n\n0b: Transmission disabled"]
pub type AcrTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCP_TX_EN` reader - GCP transmission control 1b: Transmission enabled\n\n0b: Transmission disabled"]
pub type GcpTxEnR = crate::BitReader;
#[doc = "Field `GCP_TX_EN` writer - GCP transmission control 1b: Transmission enabled\n\n0b: Transmission disabled"]
pub type GcpTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVI_TX_EN` reader - AVI packet transmission control 1b: Transmission\n\nenabled\n\n0b: Transmission disabled"]
pub type AviTxEnR = crate::BitReader;
#[doc = "Field `AVI_TX_EN` writer - AVI packet transmission control 1b: Transmission\n\nenabled\n\n0b: Transmission disabled"]
pub type AviTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUDI_TX_EN` reader - AUDI packet transmission control 1b: Transmission\n\nenabled\n\n0b: Transmission disabled"]
pub type AudiTxEnR = crate::BitReader;
#[doc = "Field `AUDI_TX_EN` writer - AUDI packet transmission control 1b: Transmission\n\nenabled\n\n0b: Transmission disabled"]
pub type AudiTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUT_TX_EN` reader - ACP, SPD, VSIF, ISRC1, and SRC2 packet\n\ntransmission control 1b: Transmission enabled\n\n0b: Transmission disabled"]
pub type AutTxEnR = crate::BitReader;
#[doc = "Field `AUT_TX_EN` writer - ACP, SPD, VSIF, ISRC1, and SRC2 packet\n\ntransmission control 1b: Transmission enabled\n\n0b: Transmission disabled"]
pub type AutTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMP_TX_EN` reader - AMP transmission control 1b: Transmission enabled\n\n0b: Transmission disabled"]
pub type AmpTxEnR = crate::BitReader;
#[doc = "Field `AMP_TX_EN` writer - AMP transmission control 1b: Transmission enabled\n\n0b: Transmission disabled"]
pub type AmpTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVBI_TX_EN` reader - NTSC VBI transmission control 1b: Transmission\n\nenabled\n\n0b: Transmission disabled"]
pub type NvbiTxEnR = crate::BitReader;
#[doc = "Field `NVBI_TX_EN` writer - NTSC VBI transmission control 1b: Transmission\n\nenabled\n\n0b: Transmission disabled"]
pub type NvbiTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRM_TX_EN` reader - DRM transmission control 1b: Transmission enabled\n\n0b: Transmission disabled"]
pub type DrmTxEnR = crate::BitReader;
#[doc = "Field `DRM_TX_EN` writer - DRM transmission control 1b: Transmission enabled\n\n0b: Transmission disabled"]
pub type DrmTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ACR packet transmission control 1b: Transmission\n\nenabled\n\n0b: Transmission disabled"]
    #[inline(always)]
    pub fn acr_tx_en(&self) -> AcrTxEnR {
        AcrTxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GCP transmission control 1b: Transmission enabled\n\n0b: Transmission disabled"]
    #[inline(always)]
    pub fn gcp_tx_en(&self) -> GcpTxEnR {
        GcpTxEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AVI packet transmission control 1b: Transmission\n\nenabled\n\n0b: Transmission disabled"]
    #[inline(always)]
    pub fn avi_tx_en(&self) -> AviTxEnR {
        AviTxEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AUDI packet transmission control 1b: Transmission\n\nenabled\n\n0b: Transmission disabled"]
    #[inline(always)]
    pub fn audi_tx_en(&self) -> AudiTxEnR {
        AudiTxEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ACP, SPD, VSIF, ISRC1, and SRC2 packet\n\ntransmission control 1b: Transmission enabled\n\n0b: Transmission disabled"]
    #[inline(always)]
    pub fn aut_tx_en(&self) -> AutTxEnR {
        AutTxEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AMP transmission control 1b: Transmission enabled\n\n0b: Transmission disabled"]
    #[inline(always)]
    pub fn amp_tx_en(&self) -> AmpTxEnR {
        AmpTxEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NTSC VBI transmission control 1b: Transmission\n\nenabled\n\n0b: Transmission disabled"]
    #[inline(always)]
    pub fn nvbi_tx_en(&self) -> NvbiTxEnR {
        NvbiTxEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DRM transmission control 1b: Transmission enabled\n\n0b: Transmission disabled"]
    #[inline(always)]
    pub fn drm_tx_en(&self) -> DrmTxEnR {
        DrmTxEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACR packet transmission control 1b: Transmission\n\nenabled\n\n0b: Transmission disabled"]
    #[inline(always)]
    #[must_use]
    pub fn acr_tx_en(&mut self) -> AcrTxEnW<FcPacketTxEnSpec> {
        AcrTxEnW::new(self, 0)
    }
    #[doc = "Bit 1 - GCP transmission control 1b: Transmission enabled\n\n0b: Transmission disabled"]
    #[inline(always)]
    #[must_use]
    pub fn gcp_tx_en(&mut self) -> GcpTxEnW<FcPacketTxEnSpec> {
        GcpTxEnW::new(self, 1)
    }
    #[doc = "Bit 2 - AVI packet transmission control 1b: Transmission\n\nenabled\n\n0b: Transmission disabled"]
    #[inline(always)]
    #[must_use]
    pub fn avi_tx_en(&mut self) -> AviTxEnW<FcPacketTxEnSpec> {
        AviTxEnW::new(self, 2)
    }
    #[doc = "Bit 3 - AUDI packet transmission control 1b: Transmission\n\nenabled\n\n0b: Transmission disabled"]
    #[inline(always)]
    #[must_use]
    pub fn audi_tx_en(&mut self) -> AudiTxEnW<FcPacketTxEnSpec> {
        AudiTxEnW::new(self, 3)
    }
    #[doc = "Bit 4 - ACP, SPD, VSIF, ISRC1, and SRC2 packet\n\ntransmission control 1b: Transmission enabled\n\n0b: Transmission disabled"]
    #[inline(always)]
    #[must_use]
    pub fn aut_tx_en(&mut self) -> AutTxEnW<FcPacketTxEnSpec> {
        AutTxEnW::new(self, 4)
    }
    #[doc = "Bit 5 - AMP transmission control 1b: Transmission enabled\n\n0b: Transmission disabled"]
    #[inline(always)]
    #[must_use]
    pub fn amp_tx_en(&mut self) -> AmpTxEnW<FcPacketTxEnSpec> {
        AmpTxEnW::new(self, 5)
    }
    #[doc = "Bit 6 - NTSC VBI transmission control 1b: Transmission\n\nenabled\n\n0b: Transmission disabled"]
    #[inline(always)]
    #[must_use]
    pub fn nvbi_tx_en(&mut self) -> NvbiTxEnW<FcPacketTxEnSpec> {
        NvbiTxEnW::new(self, 6)
    }
    #[doc = "Bit 7 - DRM transmission control 1b: Transmission enabled\n\n0b: Transmission disabled"]
    #[inline(always)]
    #[must_use]
    pub fn drm_tx_en(&mut self) -> DrmTxEnW<FcPacketTxEnSpec> {
        DrmTxEnW::new(self, 7)
    }
}
#[doc = "Frame Composer Packet Transmission Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_packet_tx_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_packet_tx_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcPacketTxEnSpec;
impl crate::RegisterSpec for FcPacketTxEnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_packet_tx_en::R`](R) reader structure"]
impl crate::Readable for FcPacketTxEnSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_packet_tx_en::W`](W) writer structure"]
impl crate::Writable for FcPacketTxEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_PACKET_TX_EN to value 0x1f"]
impl crate::Resettable for FcPacketTxEnSpec {
    const RESET_VALUE: u8 = 0x1f;
}
