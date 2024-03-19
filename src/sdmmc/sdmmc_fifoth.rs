#[doc = "Register `SDMMC_FIFOTH` reader"]
pub type R = crate::R<SdmmcFifothSpec>;
#[doc = "Register `SDMMC_FIFOTH` writer"]
pub type W = crate::W<SdmmcFifothSpec>;
#[doc = "Field `TX_WMARK` reader - FIFO threshold watermark level when transmitting data to card.\n\nWhen FIFO data count is less than or equal to this number,\n\nDMA/FIFO request is raised. If Interrupt is enabled, then interrupt\n\noccurs. During end of packet, request or interrupt is generated,\n\nregardless of threshold programming.\n\nIn non-DMA mode, when transmit FIFO threshold (TXDR)\n\ninterrupt is enabled, then interrupt is generated instead of DMA\n\nrequest. During end of packet, on last interrupt, host is\n\nresponsible for filling FIFO with only required remaining bytes\n\n(not before FIFO is full or after CIU completes data transfers,\n\nbecause FIFO may not be empty).\n\nIn DMA mode, at end of packet, if last transfer is less than burst\n\nsize, DMA controller does single cycles until required bytes are\n\ntransferred.\n\n12 bits -1 bit less than FIFO-count of status register, which is 13\n\nbits.\n\nLimitation: TX_WMark >= 1;\n\nRecommended: FIFO_DEPTH/2; (means less than or equal to\n\nFIFO_DEPTH/2)"]
pub type TxWmarkR = crate::FieldReader<u16>;
#[doc = "Field `TX_WMARK` writer - FIFO threshold watermark level when transmitting data to card.\n\nWhen FIFO data count is less than or equal to this number,\n\nDMA/FIFO request is raised. If Interrupt is enabled, then interrupt\n\noccurs. During end of packet, request or interrupt is generated,\n\nregardless of threshold programming.\n\nIn non-DMA mode, when transmit FIFO threshold (TXDR)\n\ninterrupt is enabled, then interrupt is generated instead of DMA\n\nrequest. During end of packet, on last interrupt, host is\n\nresponsible for filling FIFO with only required remaining bytes\n\n(not before FIFO is full or after CIU completes data transfers,\n\nbecause FIFO may not be empty).\n\nIn DMA mode, at end of packet, if last transfer is less than burst\n\nsize, DMA controller does single cycles until required bytes are\n\ntransferred.\n\n12 bits -1 bit less than FIFO-count of status register, which is 13\n\nbits.\n\nLimitation: TX_WMark >= 1;\n\nRecommended: FIFO_DEPTH/2; (means less than or equal to\n\nFIFO_DEPTH/2)"]
pub type TxWmarkW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RX_WMARK` reader - FIFO threshold watermark level when receiving data to card.\n\nWhen FIFO data count reaches greater than this number,\n\nDMA/FIFO request is raised. During end of packet, request is\n\ngenerated regardless of threshold programming in order to\n\ncomplete any remaining data.\n\nIn non-DMA mode, when receiver FIFO threshold (RXDR)\n\ninterrupt is enabled, then interrupt is generated instead of DMA\n\nrequest. During end of packet, interrupt is not generated if\n\nthreshold programming is larger than any remaining data. It is\n\nresponsibility of host to read remaining bytes on seeing Data\n\nTransfer Done interrupt.\n\nIn DMA mode, at end of packet, even if remaining bytes are less\n\nthan threshold, DMA request does single transfers to flush out\n\nany remaining bytes before Data Transfer Done interrupt is set.\n\n12 bits-1 bit less than FIFO-count of status register, which is 13\n\nbits.\n\nLimitation: RX_WMark &lt;= FIFO_DEPTH-2\n\nRecommended: (FIFO_DEPTH/2) - 1; (means greater than\n\n(FIFO_DEPTH/2) - 1)\n\nNOTE: In DMA mode during CCS time-out, the DMA does not generate the\n\nrequest at the end of packet, even if remaining bytes are less than\n\nthreshold. In this case, there will be some data left in the FIFO. It is the\n\nresponsibility of the application to reset the FIFO after the CCS timeout."]
pub type RxWmarkR = crate::FieldReader<u16>;
#[doc = "Field `RX_WMARK` writer - FIFO threshold watermark level when receiving data to card.\n\nWhen FIFO data count reaches greater than this number,\n\nDMA/FIFO request is raised. During end of packet, request is\n\ngenerated regardless of threshold programming in order to\n\ncomplete any remaining data.\n\nIn non-DMA mode, when receiver FIFO threshold (RXDR)\n\ninterrupt is enabled, then interrupt is generated instead of DMA\n\nrequest. During end of packet, interrupt is not generated if\n\nthreshold programming is larger than any remaining data. It is\n\nresponsibility of host to read remaining bytes on seeing Data\n\nTransfer Done interrupt.\n\nIn DMA mode, at end of packet, even if remaining bytes are less\n\nthan threshold, DMA request does single transfers to flush out\n\nany remaining bytes before Data Transfer Done interrupt is set.\n\n12 bits-1 bit less than FIFO-count of status register, which is 13\n\nbits.\n\nLimitation: RX_WMark &lt;= FIFO_DEPTH-2\n\nRecommended: (FIFO_DEPTH/2) - 1; (means greater than\n\n(FIFO_DEPTH/2) - 1)\n\nNOTE: In DMA mode during CCS time-out, the DMA does not generate the\n\nrequest at the end of packet, even if remaining bytes are less than\n\nthreshold. In this case, there will be some data left in the FIFO. It is the\n\nresponsibility of the application to reset the FIFO after the CCS timeout."]
pub type RxWmarkW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Burst size of multiple transaction; should be programmed same\n\nas DMA controller multiple-transaction-size SRC/DEST_MSIZE.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmaMutipleTransactionSize {
    #[doc = "0: 1 transfers"]
    D0 = 0,
    #[doc = "1: 4"]
    D1 = 1,
    #[doc = "2: 8"]
    D2 = 2,
    #[doc = "3: 16"]
    D3 = 3,
    #[doc = "4: 32"]
    D4 = 4,
    #[doc = "5: 64"]
    D5 = 5,
    #[doc = "6: 128"]
    D6 = 6,
    #[doc = "7: 256 The unit for transfer is the H_DATA_WIDTH parameter. A single transfer (dw_dma_single assertion in case of Non DW DMA interface) would be signalled based on this value. Value should be sub-multiple of (RX_WMark + 1)* (F_DATA_WIDTH/H_DATA_WIDTH) and (FIFO_DEPTH - TX_WMark)* (F_DATA_WIDTH/ H_DATA_WIDTH) For example, if FIFO_DEPTH = 16, FDATA_WIDTH == H_DATA_WIDTH Allowed combinations for MSize and TX_WMark are: MSize = 1, TX_WMARK = 1-15 MSize = 4, TX_WMark = 8 MSize = 4, TX_WMark = 4 MSize = 4, TX_WMark = 12 MSize = 8, TX_WMark = 8 MSize = 8, TX_WMark = 4 Allowed combinations for MSize and RX_WMark are: MSize = 1, RX_WMARK = 0-14 MSize = 4, RX_WMark = 3 MSize = 4, RX_WMark = 7 MSize = 4, RX_WMark = 11 MSize = 8, RX_WMark = 7 Recommended: MSize = 8, TX_WMark = 8, RX_WMark = 7"]
    D7 = 7,
}
impl From<DmaMutipleTransactionSize> for u8 {
    #[inline(always)]
    fn from(variant: DmaMutipleTransactionSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DmaMutipleTransactionSize {
    type Ux = u8;
}
#[doc = "Field `DMA_MUTIPLE_TRANSACTION_SIZE` reader - Burst size of multiple transaction; should be programmed same\n\nas DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
pub type DmaMutipleTransactionSizeR = crate::FieldReader<DmaMutipleTransactionSize>;
impl DmaMutipleTransactionSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaMutipleTransactionSize {
        match self.bits {
            0 => DmaMutipleTransactionSize::D0,
            1 => DmaMutipleTransactionSize::D1,
            2 => DmaMutipleTransactionSize::D2,
            3 => DmaMutipleTransactionSize::D3,
            4 => DmaMutipleTransactionSize::D4,
            5 => DmaMutipleTransactionSize::D5,
            6 => DmaMutipleTransactionSize::D6,
            7 => DmaMutipleTransactionSize::D7,
            _ => unreachable!(),
        }
    }
    #[doc = "1 transfers"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == DmaMutipleTransactionSize::D0
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == DmaMutipleTransactionSize::D1
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == DmaMutipleTransactionSize::D2
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == DmaMutipleTransactionSize::D3
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == DmaMutipleTransactionSize::D4
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == DmaMutipleTransactionSize::D5
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == DmaMutipleTransactionSize::D6
    }
    #[doc = "256 The unit for transfer is the H_DATA_WIDTH parameter. A single transfer (dw_dma_single assertion in case of Non DW DMA interface) would be signalled based on this value. Value should be sub-multiple of (RX_WMark + 1)* (F_DATA_WIDTH/H_DATA_WIDTH) and (FIFO_DEPTH - TX_WMark)* (F_DATA_WIDTH/ H_DATA_WIDTH) For example, if FIFO_DEPTH = 16, FDATA_WIDTH == H_DATA_WIDTH Allowed combinations for MSize and TX_WMark are: MSize = 1, TX_WMARK = 1-15 MSize = 4, TX_WMark = 8 MSize = 4, TX_WMark = 4 MSize = 4, TX_WMark = 12 MSize = 8, TX_WMark = 8 MSize = 8, TX_WMark = 4 Allowed combinations for MSize and RX_WMark are: MSize = 1, RX_WMARK = 0-14 MSize = 4, RX_WMark = 3 MSize = 4, RX_WMark = 7 MSize = 4, RX_WMark = 11 MSize = 8, RX_WMark = 7 Recommended: MSize = 8, TX_WMark = 8, RX_WMark = 7"]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == DmaMutipleTransactionSize::D7
    }
}
#[doc = "Field `DMA_MUTIPLE_TRANSACTION_SIZE` writer - Burst size of multiple transaction; should be programmed same\n\nas DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
pub type DmaMutipleTransactionSizeW<'a, REG> =
    crate::FieldWriterSafe<'a, REG, 3, DmaMutipleTransactionSize>;
impl<'a, REG> DmaMutipleTransactionSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 transfers"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(DmaMutipleTransactionSize::D0)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(DmaMutipleTransactionSize::D1)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(DmaMutipleTransactionSize::D2)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(DmaMutipleTransactionSize::D3)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(DmaMutipleTransactionSize::D4)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn d5(self) -> &'a mut crate::W<REG> {
        self.variant(DmaMutipleTransactionSize::D5)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn d6(self) -> &'a mut crate::W<REG> {
        self.variant(DmaMutipleTransactionSize::D6)
    }
    #[doc = "256 The unit for transfer is the H_DATA_WIDTH parameter. A single transfer (dw_dma_single assertion in case of Non DW DMA interface) would be signalled based on this value. Value should be sub-multiple of (RX_WMark + 1)* (F_DATA_WIDTH/H_DATA_WIDTH) and (FIFO_DEPTH - TX_WMark)* (F_DATA_WIDTH/ H_DATA_WIDTH) For example, if FIFO_DEPTH = 16, FDATA_WIDTH == H_DATA_WIDTH Allowed combinations for MSize and TX_WMark are: MSize = 1, TX_WMARK = 1-15 MSize = 4, TX_WMark = 8 MSize = 4, TX_WMark = 4 MSize = 4, TX_WMark = 12 MSize = 8, TX_WMark = 8 MSize = 8, TX_WMark = 4 Allowed combinations for MSize and RX_WMark are: MSize = 1, RX_WMARK = 0-14 MSize = 4, RX_WMark = 3 MSize = 4, RX_WMark = 7 MSize = 4, RX_WMark = 11 MSize = 8, RX_WMark = 7 Recommended: MSize = 8, TX_WMark = 8, RX_WMark = 7"]
    #[inline(always)]
    pub fn d7(self) -> &'a mut crate::W<REG> {
        self.variant(DmaMutipleTransactionSize::D7)
    }
}
impl R {
    #[doc = "Bits 0:11 - FIFO threshold watermark level when transmitting data to card.\n\nWhen FIFO data count is less than or equal to this number,\n\nDMA/FIFO request is raised. If Interrupt is enabled, then interrupt\n\noccurs. During end of packet, request or interrupt is generated,\n\nregardless of threshold programming.\n\nIn non-DMA mode, when transmit FIFO threshold (TXDR)\n\ninterrupt is enabled, then interrupt is generated instead of DMA\n\nrequest. During end of packet, on last interrupt, host is\n\nresponsible for filling FIFO with only required remaining bytes\n\n(not before FIFO is full or after CIU completes data transfers,\n\nbecause FIFO may not be empty).\n\nIn DMA mode, at end of packet, if last transfer is less than burst\n\nsize, DMA controller does single cycles until required bytes are\n\ntransferred.\n\n12 bits -1 bit less than FIFO-count of status register, which is 13\n\nbits.\n\nLimitation: TX_WMark >= 1;\n\nRecommended: FIFO_DEPTH/2; (means less than or equal to\n\nFIFO_DEPTH/2)"]
    #[inline(always)]
    pub fn tx_wmark(&self) -> TxWmarkR {
        TxWmarkR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - FIFO threshold watermark level when receiving data to card.\n\nWhen FIFO data count reaches greater than this number,\n\nDMA/FIFO request is raised. During end of packet, request is\n\ngenerated regardless of threshold programming in order to\n\ncomplete any remaining data.\n\nIn non-DMA mode, when receiver FIFO threshold (RXDR)\n\ninterrupt is enabled, then interrupt is generated instead of DMA\n\nrequest. During end of packet, interrupt is not generated if\n\nthreshold programming is larger than any remaining data. It is\n\nresponsibility of host to read remaining bytes on seeing Data\n\nTransfer Done interrupt.\n\nIn DMA mode, at end of packet, even if remaining bytes are less\n\nthan threshold, DMA request does single transfers to flush out\n\nany remaining bytes before Data Transfer Done interrupt is set.\n\n12 bits-1 bit less than FIFO-count of status register, which is 13\n\nbits.\n\nLimitation: RX_WMark &lt;= FIFO_DEPTH-2\n\nRecommended: (FIFO_DEPTH/2) - 1; (means greater than\n\n(FIFO_DEPTH/2) - 1)\n\nNOTE: In DMA mode during CCS time-out, the DMA does not generate the\n\nrequest at the end of packet, even if remaining bytes are less than\n\nthreshold. In this case, there will be some data left in the FIFO. It is the\n\nresponsibility of the application to reset the FIFO after the CCS timeout."]
    #[inline(always)]
    pub fn rx_wmark(&self) -> RxWmarkR {
        RxWmarkR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:30 - Burst size of multiple transaction; should be programmed same\n\nas DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
    #[inline(always)]
    pub fn dma_mutiple_transaction_size(&self) -> DmaMutipleTransactionSizeR {
        DmaMutipleTransactionSizeR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - FIFO threshold watermark level when transmitting data to card.\n\nWhen FIFO data count is less than or equal to this number,\n\nDMA/FIFO request is raised. If Interrupt is enabled, then interrupt\n\noccurs. During end of packet, request or interrupt is generated,\n\nregardless of threshold programming.\n\nIn non-DMA mode, when transmit FIFO threshold (TXDR)\n\ninterrupt is enabled, then interrupt is generated instead of DMA\n\nrequest. During end of packet, on last interrupt, host is\n\nresponsible for filling FIFO with only required remaining bytes\n\n(not before FIFO is full or after CIU completes data transfers,\n\nbecause FIFO may not be empty).\n\nIn DMA mode, at end of packet, if last transfer is less than burst\n\nsize, DMA controller does single cycles until required bytes are\n\ntransferred.\n\n12 bits -1 bit less than FIFO-count of status register, which is 13\n\nbits.\n\nLimitation: TX_WMark >= 1;\n\nRecommended: FIFO_DEPTH/2; (means less than or equal to\n\nFIFO_DEPTH/2)"]
    #[inline(always)]
    #[must_use]
    pub fn tx_wmark(&mut self) -> TxWmarkW<SdmmcFifothSpec> {
        TxWmarkW::new(self, 0)
    }
    #[doc = "Bits 16:27 - FIFO threshold watermark level when receiving data to card.\n\nWhen FIFO data count reaches greater than this number,\n\nDMA/FIFO request is raised. During end of packet, request is\n\ngenerated regardless of threshold programming in order to\n\ncomplete any remaining data.\n\nIn non-DMA mode, when receiver FIFO threshold (RXDR)\n\ninterrupt is enabled, then interrupt is generated instead of DMA\n\nrequest. During end of packet, interrupt is not generated if\n\nthreshold programming is larger than any remaining data. It is\n\nresponsibility of host to read remaining bytes on seeing Data\n\nTransfer Done interrupt.\n\nIn DMA mode, at end of packet, even if remaining bytes are less\n\nthan threshold, DMA request does single transfers to flush out\n\nany remaining bytes before Data Transfer Done interrupt is set.\n\n12 bits-1 bit less than FIFO-count of status register, which is 13\n\nbits.\n\nLimitation: RX_WMark &lt;= FIFO_DEPTH-2\n\nRecommended: (FIFO_DEPTH/2) - 1; (means greater than\n\n(FIFO_DEPTH/2) - 1)\n\nNOTE: In DMA mode during CCS time-out, the DMA does not generate the\n\nrequest at the end of packet, even if remaining bytes are less than\n\nthreshold. In this case, there will be some data left in the FIFO. It is the\n\nresponsibility of the application to reset the FIFO after the CCS timeout."]
    #[inline(always)]
    #[must_use]
    pub fn rx_wmark(&mut self) -> RxWmarkW<SdmmcFifothSpec> {
        RxWmarkW::new(self, 16)
    }
    #[doc = "Bits 28:30 - Burst size of multiple transaction; should be programmed same\n\nas DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
    #[inline(always)]
    #[must_use]
    pub fn dma_mutiple_transaction_size(&mut self) -> DmaMutipleTransactionSizeW<SdmmcFifothSpec> {
        DmaMutipleTransactionSizeW::new(self, 28)
    }
}
#[doc = "FIFO threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifoth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifoth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcFifothSpec;
impl crate::RegisterSpec for SdmmcFifothSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_fifoth::R`](R) reader structure"]
impl crate::Readable for SdmmcFifothSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_fifoth::W`](W) writer structure"]
impl crate::Writable for SdmmcFifothSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_FIFOTH to value 0"]
impl crate::Resettable for SdmmcFifothSpec {
    const RESET_VALUE: u32 = 0;
}
