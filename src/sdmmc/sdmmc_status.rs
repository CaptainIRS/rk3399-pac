#[doc = "Register `SDMMC_STATUS` reader"]
pub type R = crate::R<SdmmcStatusSpec>;
#[doc = "Field `FIFO_RX_WATERMARK` reader - FIFO reached Receive watermark level; not qualified with data transfer"]
pub type FifoRxWatermarkR = crate::BitReader;
#[doc = "Field `FIFO_TX_WATERMARK` reader - FIFO reached Transmit watermark level; not qualified with data transfer"]
pub type FifoTxWatermarkR = crate::BitReader;
#[doc = "Field `FIFO_EMPTY` reader - FIFO is empty status"]
pub type FifoEmptyR = crate::BitReader;
#[doc = "Field `FIFO_FULL` reader - FIFO is full status"]
pub type FifoFullR = crate::BitReader;
#[doc = "Field `COMMAND_FSM_STATES` reader - Command FSM states: 0: idle 1: send init sequence 2: Tx cmd start bit 3: Tx cmd tx bit 4: Tx cmd index + arg 5: Tx cmd crc7 6: Tx cmd end bit 7: Rx resp start bit 8: Rx resp IRQ response 9: Rx resp tx bit 10: Rx resp cmd idx 11: Rx resp data 12: Rx resp crc7 13: Rx resp end bit 14: Cmd path wait NCC 15: Wait; CMD-to-response turnaround The command FSM state is represented using 19 bits. The STATUS Register\\[7:4\\]
has 4 bits to represent the command FSM states. Using these 4 bits, only 16 states can be represented. Thus three states cannot be represented in the STATUS\\[7:4\\]
register. The three states that are not represented in the STATUS Register\\[7:4\\]
are: a. Bit 16 –Wait for CCS b. Bit 17 –Send CCSD c. Bit 18 –Boot Mode Due to this, while command FSM is in \"Wait for CCS state\" or \"Send CCSD\" or \"Boot Mode\", the Status register indicates status as 0 for the bit field \\[7:4\\]."]
pub type CommandFsmStatesR = crate::FieldReader;
#[doc = "Field `DATA_3_STATUS` reader - Raw selected card_data\\[3\\]; checks whether card is present 0: card not present 1: card present default value is 1 or 0 depending on cdata_in"]
pub type Data3StatusR = crate::BitReader;
#[doc = "Field `DATA_BUSY` reader - Inverted version of raw selected card_data\\[0\\]
0: card data not busy 1: card data busy default value is 1 or 0 depending on cdata_in"]
pub type DataBusyR = crate::BitReader;
#[doc = "Field `DATA_STATE_MC_BUSY` reader - Data transmit or receive state-machine is busy"]
pub type DataStateMcBusyR = crate::BitReader;
#[doc = "Field `RESPONSE_INDEX` reader - Index of previous response, including any auto-stop sent by core"]
pub type ResponseIndexR = crate::FieldReader;
#[doc = "Field `FIFO_COUNT` reader - Number of filled locations in FIFO"]
pub type FifoCountR = crate::FieldReader<u16>;
#[doc = "Field `DMA_ACK` reader - DMA acknowledge signal state"]
pub type DmaAckR = crate::BitReader;
#[doc = "Field `DMA_REQ` reader - DMA request signal state"]
pub type DmaReqR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FIFO reached Receive watermark level; not qualified with data transfer"]
    #[inline(always)]
    pub fn fifo_rx_watermark(&self) -> FifoRxWatermarkR {
        FifoRxWatermarkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO reached Transmit watermark level; not qualified with data transfer"]
    #[inline(always)]
    pub fn fifo_tx_watermark(&self) -> FifoTxWatermarkR {
        FifoTxWatermarkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO is empty status"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FifoEmptyR {
        FifoEmptyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO is full status"]
    #[inline(always)]
    pub fn fifo_full(&self) -> FifoFullR {
        FifoFullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Command FSM states: 0: idle 1: send init sequence 2: Tx cmd start bit 3: Tx cmd tx bit 4: Tx cmd index + arg 5: Tx cmd crc7 6: Tx cmd end bit 7: Rx resp start bit 8: Rx resp IRQ response 9: Rx resp tx bit 10: Rx resp cmd idx 11: Rx resp data 12: Rx resp crc7 13: Rx resp end bit 14: Cmd path wait NCC 15: Wait; CMD-to-response turnaround The command FSM state is represented using 19 bits. The STATUS Register\\[7:4\\]
has 4 bits to represent the command FSM states. Using these 4 bits, only 16 states can be represented. Thus three states cannot be represented in the STATUS\\[7:4\\]
register. The three states that are not represented in the STATUS Register\\[7:4\\]
are: a. Bit 16 –Wait for CCS b. Bit 17 –Send CCSD c. Bit 18 –Boot Mode Due to this, while command FSM is in \"Wait for CCS state\" or \"Send CCSD\" or \"Boot Mode\", the Status register indicates status as 0 for the bit field \\[7:4\\]."]
    #[inline(always)]
    pub fn command_fsm_states(&self) -> CommandFsmStatesR {
        CommandFsmStatesR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Raw selected card_data\\[3\\]; checks whether card is present 0: card not present 1: card present default value is 1 or 0 depending on cdata_in"]
    #[inline(always)]
    pub fn data_3_status(&self) -> Data3StatusR {
        Data3StatusR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Inverted version of raw selected card_data\\[0\\]
0: card data not busy 1: card data busy default value is 1 or 0 depending on cdata_in"]
    #[inline(always)]
    pub fn data_busy(&self) -> DataBusyR {
        DataBusyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data transmit or receive state-machine is busy"]
    #[inline(always)]
    pub fn data_state_mc_busy(&self) -> DataStateMcBusyR {
        DataStateMcBusyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:16 - Index of previous response, including any auto-stop sent by core"]
    #[inline(always)]
    pub fn response_index(&self) -> ResponseIndexR {
        ResponseIndexR::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 17:29 - Number of filled locations in FIFO"]
    #[inline(always)]
    pub fn fifo_count(&self) -> FifoCountR {
        FifoCountR::new(((self.bits >> 17) & 0x1fff) as u16)
    }
    #[doc = "Bit 30 - DMA acknowledge signal state"]
    #[inline(always)]
    pub fn dma_ack(&self) -> DmaAckR {
        DmaAckR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA request signal state"]
    #[inline(always)]
    pub fn dma_req(&self) -> DmaReqR {
        DmaReqR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcStatusSpec;
impl crate::RegisterSpec for SdmmcStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_status::R`](R) reader structure"]
impl crate::Readable for SdmmcStatusSpec {}
#[doc = "`reset()` method sets SDMMC_STATUS to value 0x0406"]
impl crate::Resettable for SdmmcStatusSpec {
    const RESET_VALUE: u32 = 0x0406;
}
