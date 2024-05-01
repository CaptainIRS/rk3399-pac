#[doc = "Register `INT_ST1` reader"]
pub type R = crate::R<IntSt1Spec>;
#[doc = "Field `TO_HS_TX` reader - to_hs_tx\n\nThis bit indicates that the high-speed transmission timeout counter\n\nreached the end and contention is detected."]
pub type ToHsTxR = crate::BitReader;
#[doc = "Field `TO_LP_RX` reader - to_lp_rx\n\nThis bit indicates that the low-power reception timeout counter\n\nreached the end and contention is detected."]
pub type ToLpRxR = crate::BitReader;
#[doc = "Field `ECC_SINGLE_ERR` reader - ecc_single_err\n\nThis bit indicates that the ECC single error is detected and corrected\n\nin a received packet."]
pub type EccSingleErrR = crate::BitReader;
#[doc = "Field `ECC_MULTI_ERR` reader - ecc_multi_err\n\nThis bit indicates that the ECC multiple error is detected in a\n\nreceived error."]
pub type EccMultiErrR = crate::BitReader;
#[doc = "Field `CRC_ERR` reader - crc_err\n\nThis bit indicates that the CRC error is detected in the received\n\npacket payload."]
pub type CrcErrR = crate::BitReader;
#[doc = "Field `PKT_SIZE_ERR` reader - pkt_size_err\n\nThis bit indicates that the packet size error is detected during the\n\npacket reception"]
pub type PktSizeErrR = crate::BitReader;
#[doc = "Field `EOPT_ERR` reader - eopt_err\n\nThis bit indicates that the EoTp packet is not received at the end of\n\nthe incoming peripheral transmission."]
pub type EoptErrR = crate::BitReader;
#[doc = "Field `DPI_PLD_WR_ERR` reader - dpi_pld_wr_err\n\nThis bit indicates that during a DPI pixel line storage, the payload\n\nFIFO becomes full and the data stored is corrupted."]
pub type DpiPldWrErrR = crate::BitReader;
#[doc = "Field `GEN_CMD_WR_ERR` reader - gen_cmd_wr_err\n\nThis bit indicates that the system tried to write a command through\n\nthe Generic interface and the FIFO is full. Therefore, the command\n\nis not written."]
pub type GenCmdWrErrR = crate::BitReader;
#[doc = "Field `GEN_PLD_WR_ERR` reader - gen_pld_wr_err\n\nThis bit indicates that the system tried to write a payload data\n\nthrough the Generic interface and the FIFO is full. Therefore, the\n\npayload is not written."]
pub type GenPldWrErrR = crate::BitReader;
#[doc = "Field `GEN_PLD_SEND_ERR` reader - gen_pld_send_err\n\nThis bit indicates that during a Generic interface packet build, the\n\npayload FIFO becomes empty and corrupt data is sent."]
pub type GenPldSendErrR = crate::BitReader;
#[doc = "Field `GEN_PLD_RD_ERR` reader - gen_pld_rd_err\n\nThis bit indicates that during a DCS read data, the payload FIFO\n\nbecomes empty and the data sent to the interface is corrupted."]
pub type GenPldRdErrR = crate::BitReader;
#[doc = "Field `GEN_PLD_RECEV_ERR` reader - gen_pld_recev_err\n\nThis bit indicates that during a generic interface packet read back,\n\nthe payload FIFO becomes full and the received data is corrupted."]
pub type GenPldRecevErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - to_hs_tx\n\nThis bit indicates that the high-speed transmission timeout counter\n\nreached the end and contention is detected."]
    #[inline(always)]
    pub fn to_hs_tx(&self) -> ToHsTxR {
        ToHsTxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - to_lp_rx\n\nThis bit indicates that the low-power reception timeout counter\n\nreached the end and contention is detected."]
    #[inline(always)]
    pub fn to_lp_rx(&self) -> ToLpRxR {
        ToLpRxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ecc_single_err\n\nThis bit indicates that the ECC single error is detected and corrected\n\nin a received packet."]
    #[inline(always)]
    pub fn ecc_single_err(&self) -> EccSingleErrR {
        EccSingleErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ecc_multi_err\n\nThis bit indicates that the ECC multiple error is detected in a\n\nreceived error."]
    #[inline(always)]
    pub fn ecc_multi_err(&self) -> EccMultiErrR {
        EccMultiErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - crc_err\n\nThis bit indicates that the CRC error is detected in the received\n\npacket payload."]
    #[inline(always)]
    pub fn crc_err(&self) -> CrcErrR {
        CrcErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - pkt_size_err\n\nThis bit indicates that the packet size error is detected during the\n\npacket reception"]
    #[inline(always)]
    pub fn pkt_size_err(&self) -> PktSizeErrR {
        PktSizeErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - eopt_err\n\nThis bit indicates that the EoTp packet is not received at the end of\n\nthe incoming peripheral transmission."]
    #[inline(always)]
    pub fn eopt_err(&self) -> EoptErrR {
        EoptErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - dpi_pld_wr_err\n\nThis bit indicates that during a DPI pixel line storage, the payload\n\nFIFO becomes full and the data stored is corrupted."]
    #[inline(always)]
    pub fn dpi_pld_wr_err(&self) -> DpiPldWrErrR {
        DpiPldWrErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - gen_cmd_wr_err\n\nThis bit indicates that the system tried to write a command through\n\nthe Generic interface and the FIFO is full. Therefore, the command\n\nis not written."]
    #[inline(always)]
    pub fn gen_cmd_wr_err(&self) -> GenCmdWrErrR {
        GenCmdWrErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - gen_pld_wr_err\n\nThis bit indicates that the system tried to write a payload data\n\nthrough the Generic interface and the FIFO is full. Therefore, the\n\npayload is not written."]
    #[inline(always)]
    pub fn gen_pld_wr_err(&self) -> GenPldWrErrR {
        GenPldWrErrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - gen_pld_send_err\n\nThis bit indicates that during a Generic interface packet build, the\n\npayload FIFO becomes empty and corrupt data is sent."]
    #[inline(always)]
    pub fn gen_pld_send_err(&self) -> GenPldSendErrR {
        GenPldSendErrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - gen_pld_rd_err\n\nThis bit indicates that during a DCS read data, the payload FIFO\n\nbecomes empty and the data sent to the interface is corrupted."]
    #[inline(always)]
    pub fn gen_pld_rd_err(&self) -> GenPldRdErrR {
        GenPldRdErrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - gen_pld_recev_err\n\nThis bit indicates that during a generic interface packet read back,\n\nthe payload FIFO becomes full and the received data is corrupted."]
    #[inline(always)]
    pub fn gen_pld_recev_err(&self) -> GenPldRecevErrR {
        GenPldRecevErrR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSt1Spec;
impl crate::RegisterSpec for IntSt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st1::R`](R) reader structure"]
impl crate::Readable for IntSt1Spec {}
#[doc = "`reset()` method sets INT_ST1 to value 0"]
impl crate::Resettable for IntSt1Spec {
    const RESET_VALUE: u32 = 0;
}
