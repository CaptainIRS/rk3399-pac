#[doc = "Register `INT_FORCE1` writer"]
pub type W = crate::W<IntForce1Spec>;
#[doc = "Field `TO_HS_TX` writer - to_hs_tx\n\nThis bit indicates that the high-speed transmission timeout counter\n\nreached the end and contention is detected."]
pub type ToHsTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TO_LP_RX` writer - to_lp_rx\n\nThis bit indicates that the low-power reception timeout counter\n\nreached the end and contention is detected."]
pub type ToLpRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_SINGLE_ERR` writer - ecc_single_err\n\nThis bit indicates that the ECC single error is detected and corrected\n\nin a received packet."]
pub type EccSingleErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_MULTI_ERR` writer - ecc_multi_err\n\nThis bit indicates that the ECC multiple error is detected in a\n\nreceived error."]
pub type EccMultiErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_ERR` writer - crc_err\n\nThis bit indicates that the CRC error is detected in the received\n\npacket payload."]
pub type CrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKT_SIZE_ERR` writer - pkt_size_err\n\nThis bit indicates that the packet size error is detected during the\n\npacket reception"]
pub type PktSizeErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPT_ERR` writer - eopt_err\n\nThis bit indicates that the EoTp packet is not received at the end of\n\nthe incoming peripheral transmission."]
pub type EoptErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPI_PLD_WR_ERR` writer - dpi_pld_wr_err\n\nThis bit indicates that during a DPI pixel line storage, the payload\n\nFIFO becomes full and the data stored is corrupted."]
pub type DpiPldWrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_CMD_WR_ERR` writer - gen_cmd_wr_err\n\nThis bit indicates that the system tried to write a command through\n\nthe Generic interface and the FIFO is full. Therefore, the command\n\nis not written."]
pub type GenCmdWrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_PLD_WR_ERR` writer - gen_pld_wr_err\n\nThis bit indicates that the system tried to write a payload data\n\nthrough the Generic interface and the FIFO is full. Therefore, the\n\npayload is not written."]
pub type GenPldWrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_PLD_SEND_ERR` writer - gen_pld_send_err\n\nThis bit indicates that during a Generic interface packet build, the\n\npayload FIFO becomes empty and corrupt data is sent."]
pub type GenPldSendErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_PLD_RD_ERR` writer - gen_pld_rd_err\n\nThis bit indicates that during a DCS read data, the payload FIFO\n\nbecomes empty and the data sent to the interface is corrupted."]
pub type GenPldRdErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_PLD_RECEV_ERR` writer - gen_pld_recev_err\n\nThis bit indicates that during a generic interface packet read back,\n\nthe payload FIFO becomes full and the received data is corrupted."]
pub type GenPldRecevErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - to_hs_tx\n\nThis bit indicates that the high-speed transmission timeout counter\n\nreached the end and contention is detected."]
    #[inline(always)]
    #[must_use]
    pub fn to_hs_tx(&mut self) -> ToHsTxW<IntForce1Spec> {
        ToHsTxW::new(self, 0)
    }
    #[doc = "Bit 1 - to_lp_rx\n\nThis bit indicates that the low-power reception timeout counter\n\nreached the end and contention is detected."]
    #[inline(always)]
    #[must_use]
    pub fn to_lp_rx(&mut self) -> ToLpRxW<IntForce1Spec> {
        ToLpRxW::new(self, 1)
    }
    #[doc = "Bit 2 - ecc_single_err\n\nThis bit indicates that the ECC single error is detected and corrected\n\nin a received packet."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_single_err(&mut self) -> EccSingleErrW<IntForce1Spec> {
        EccSingleErrW::new(self, 2)
    }
    #[doc = "Bit 3 - ecc_multi_err\n\nThis bit indicates that the ECC multiple error is detected in a\n\nreceived error."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_multi_err(&mut self) -> EccMultiErrW<IntForce1Spec> {
        EccMultiErrW::new(self, 3)
    }
    #[doc = "Bit 4 - crc_err\n\nThis bit indicates that the CRC error is detected in the received\n\npacket payload."]
    #[inline(always)]
    #[must_use]
    pub fn crc_err(&mut self) -> CrcErrW<IntForce1Spec> {
        CrcErrW::new(self, 4)
    }
    #[doc = "Bit 5 - pkt_size_err\n\nThis bit indicates that the packet size error is detected during the\n\npacket reception"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_size_err(&mut self) -> PktSizeErrW<IntForce1Spec> {
        PktSizeErrW::new(self, 5)
    }
    #[doc = "Bit 6 - eopt_err\n\nThis bit indicates that the EoTp packet is not received at the end of\n\nthe incoming peripheral transmission."]
    #[inline(always)]
    #[must_use]
    pub fn eopt_err(&mut self) -> EoptErrW<IntForce1Spec> {
        EoptErrW::new(self, 6)
    }
    #[doc = "Bit 7 - dpi_pld_wr_err\n\nThis bit indicates that during a DPI pixel line storage, the payload\n\nFIFO becomes full and the data stored is corrupted."]
    #[inline(always)]
    #[must_use]
    pub fn dpi_pld_wr_err(&mut self) -> DpiPldWrErrW<IntForce1Spec> {
        DpiPldWrErrW::new(self, 7)
    }
    #[doc = "Bit 8 - gen_cmd_wr_err\n\nThis bit indicates that the system tried to write a command through\n\nthe Generic interface and the FIFO is full. Therefore, the command\n\nis not written."]
    #[inline(always)]
    #[must_use]
    pub fn gen_cmd_wr_err(&mut self) -> GenCmdWrErrW<IntForce1Spec> {
        GenCmdWrErrW::new(self, 8)
    }
    #[doc = "Bit 9 - gen_pld_wr_err\n\nThis bit indicates that the system tried to write a payload data\n\nthrough the Generic interface and the FIFO is full. Therefore, the\n\npayload is not written."]
    #[inline(always)]
    #[must_use]
    pub fn gen_pld_wr_err(&mut self) -> GenPldWrErrW<IntForce1Spec> {
        GenPldWrErrW::new(self, 9)
    }
    #[doc = "Bit 10 - gen_pld_send_err\n\nThis bit indicates that during a Generic interface packet build, the\n\npayload FIFO becomes empty and corrupt data is sent."]
    #[inline(always)]
    #[must_use]
    pub fn gen_pld_send_err(&mut self) -> GenPldSendErrW<IntForce1Spec> {
        GenPldSendErrW::new(self, 10)
    }
    #[doc = "Bit 11 - gen_pld_rd_err\n\nThis bit indicates that during a DCS read data, the payload FIFO\n\nbecomes empty and the data sent to the interface is corrupted."]
    #[inline(always)]
    #[must_use]
    pub fn gen_pld_rd_err(&mut self) -> GenPldRdErrW<IntForce1Spec> {
        GenPldRdErrW::new(self, 11)
    }
    #[doc = "Bit 12 - gen_pld_recev_err\n\nThis bit indicates that during a generic interface packet read back,\n\nthe payload FIFO becomes full and the received data is corrupted."]
    #[inline(always)]
    #[must_use]
    pub fn gen_pld_recev_err(&mut self) -> GenPldRecevErrW<IntForce1Spec> {
        GenPldRecevErrW::new(self, 12)
    }
}
#[doc = "Force Interrupt Configuration Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_force1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntForce1Spec;
impl crate::RegisterSpec for IntForce1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_force1::W`](W) writer structure"]
impl crate::Writable for IntForce1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_FORCE1 to value 0"]
impl crate::Resettable for IntForce1Spec {
    const RESET_VALUE: u32 = 0;
}
