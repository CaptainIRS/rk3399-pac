#[doc = "Register `PCKHDL_CFG` reader"]
pub type R = crate::R<PckhdlCfgSpec>;
#[doc = "Register `PCKHDL_CFG` writer"]
pub type W = crate::W<PckhdlCfgSpec>;
#[doc = "Field `EOTP_TX_EN` reader - eotp_tx_en\n\nWhen set to 1, this bit enables the EoTp transmission. (When\n\ntransfer video to UNIPRO, this bit must be set to 0)."]
pub type EotpTxEnR = crate::BitReader;
#[doc = "Field `EOTP_TX_EN` writer - eotp_tx_en\n\nWhen set to 1, this bit enables the EoTp transmission. (When\n\ntransfer video to UNIPRO, this bit must be set to 0)."]
pub type EotpTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOTP_RX_EN` reader - eotp_rx_en\n\nWhen set to 1, this bit enables the EoTp reception."]
pub type EotpRxEnR = crate::BitReader;
#[doc = "Field `EOTP_RX_EN` writer - eotp_rx_en\n\nWhen set to 1, this bit enables the EoTp reception."]
pub type EotpRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTA_EN` reader - Field0000 Abstract\n\nWhen set to 1, this bit enables the Bus Turn-Around (BTA) request."]
pub type BtaEnR = crate::BitReader;
#[doc = "Field `BTA_EN` writer - Field0000 Abstract\n\nWhen set to 1, this bit enables the Bus Turn-Around (BTA) request."]
pub type BtaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_RX_EN` reader - ecc_rx_en\n\nWhen set to 1, this bit enables the ECC reception, error correction,\n\nand\n\nreporting."]
pub type EccRxEnR = crate::BitReader;
#[doc = "Field `ECC_RX_EN` writer - ecc_rx_en\n\nWhen set to 1, this bit enables the ECC reception, error correction,\n\nand\n\nreporting."]
pub type EccRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_RX_EN` reader - crc_rx_en\n\nWhen set to 1, this bit enables the CRC reception and error\n\nreporting"]
pub type CrcRxEnR = crate::BitReader;
#[doc = "Field `CRC_RX_EN` writer - crc_rx_en\n\nWhen set to 1, this bit enables the CRC reception and error\n\nreporting"]
pub type CrcRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - eotp_tx_en\n\nWhen set to 1, this bit enables the EoTp transmission. (When\n\ntransfer video to UNIPRO, this bit must be set to 0)."]
    #[inline(always)]
    pub fn eotp_tx_en(&self) -> EotpTxEnR {
        EotpTxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - eotp_rx_en\n\nWhen set to 1, this bit enables the EoTp reception."]
    #[inline(always)]
    pub fn eotp_rx_en(&self) -> EotpRxEnR {
        EotpRxEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Field0000 Abstract\n\nWhen set to 1, this bit enables the Bus Turn-Around (BTA) request."]
    #[inline(always)]
    pub fn bta_en(&self) -> BtaEnR {
        BtaEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ecc_rx_en\n\nWhen set to 1, this bit enables the ECC reception, error correction,\n\nand\n\nreporting."]
    #[inline(always)]
    pub fn ecc_rx_en(&self) -> EccRxEnR {
        EccRxEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - crc_rx_en\n\nWhen set to 1, this bit enables the CRC reception and error\n\nreporting"]
    #[inline(always)]
    pub fn crc_rx_en(&self) -> CrcRxEnR {
        CrcRxEnR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - eotp_tx_en\n\nWhen set to 1, this bit enables the EoTp transmission. (When\n\ntransfer video to UNIPRO, this bit must be set to 0)."]
    #[inline(always)]
    #[must_use]
    pub fn eotp_tx_en(&mut self) -> EotpTxEnW<PckhdlCfgSpec> {
        EotpTxEnW::new(self, 0)
    }
    #[doc = "Bit 1 - eotp_rx_en\n\nWhen set to 1, this bit enables the EoTp reception."]
    #[inline(always)]
    #[must_use]
    pub fn eotp_rx_en(&mut self) -> EotpRxEnW<PckhdlCfgSpec> {
        EotpRxEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Field0000 Abstract\n\nWhen set to 1, this bit enables the Bus Turn-Around (BTA) request."]
    #[inline(always)]
    #[must_use]
    pub fn bta_en(&mut self) -> BtaEnW<PckhdlCfgSpec> {
        BtaEnW::new(self, 2)
    }
    #[doc = "Bit 3 - ecc_rx_en\n\nWhen set to 1, this bit enables the ECC reception, error correction,\n\nand\n\nreporting."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_rx_en(&mut self) -> EccRxEnW<PckhdlCfgSpec> {
        EccRxEnW::new(self, 3)
    }
    #[doc = "Bit 4 - crc_rx_en\n\nWhen set to 1, this bit enables the CRC reception and error\n\nreporting"]
    #[inline(always)]
    #[must_use]
    pub fn crc_rx_en(&mut self) -> CrcRxEnW<PckhdlCfgSpec> {
        CrcRxEnW::new(self, 4)
    }
}
#[doc = "Packet Handler Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pckhdl_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pckhdl_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PckhdlCfgSpec;
impl crate::RegisterSpec for PckhdlCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pckhdl_cfg::R`](R) reader structure"]
impl crate::Readable for PckhdlCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pckhdl_cfg::W`](W) writer structure"]
impl crate::Writable for PckhdlCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCKHDL_CFG to value 0"]
impl crate::Resettable for PckhdlCfgSpec {
    const RESET_VALUE: u32 = 0;
}
