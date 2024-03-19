#[doc = "Register `SDMMC_STATUS` reader"]
pub type R = crate::R<SdmmcStatusSpec>;
#[doc = "Field `FIFO_RX_WATERMARK` reader - FIFO reached Receive watermark level; not qualified with data\n\ntransfer"]
pub type FifoRxWatermarkR = crate::BitReader;
#[doc = "Field `FIFO_TX_WATERMARK` reader - FIFO reached Transmit watermark level; not qualified with data\n\ntransfer"]
pub type FifoTxWatermarkR = crate::BitReader;
#[doc = "Field `FIFO_EMPTY` reader - FIFO is empty status"]
pub type FifoEmptyR = crate::BitReader;
#[doc = "Field `FIFO_FULL` reader - FIFO is full status"]
pub type FifoFullR = crate::BitReader;
#[doc = "Command FSM states:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CommandFsmStates {
    #[doc = "0: idle"]
    D0 = 0,
    #[doc = "1: send init sequence"]
    D1 = 1,
    #[doc = "2: Tx cmd start bit"]
    D2 = 2,
    #[doc = "3: Tx cmd tx bit"]
    D3 = 3,
    #[doc = "4: Tx cmd index + arg"]
    D4 = 4,
    #[doc = "5: Tx cmd crc7"]
    D5 = 5,
    #[doc = "6: Tx cmd end bit"]
    D6 = 6,
    #[doc = "7: Rx resp start bit"]
    D7 = 7,
    #[doc = "8: Rx resp IRQ response"]
    D8 = 8,
    #[doc = "9: Rx resp tx bit"]
    D9 = 9,
    #[doc = "10: Rx resp cmd idx"]
    D10 = 10,
    #[doc = "11: Rx resp data"]
    D11 = 11,
    #[doc = "12: Rx resp crc7"]
    D12 = 12,
    #[doc = "13: Rx resp end bit"]
    D13 = 13,
    #[doc = "14: Cmd path wait NCC"]
    D14 = 14,
    #[doc = "15: Wait; CMD-to-response turnaround The command FSM state is represented using 19 bits. The STATUS Register\\[7:4\\]
has 4 bits to represent the command FSM states. Using these 4 bits, only 16 states can be represented. Thus three states cannot be represented in the STATUS\\[7:4\\]
register. The three states that are not represented in the STATUS Register\\[7:4\\]
are: a. Bit 16 –Wait for CCS b. Bit 17 –Send CCSD c. Bit 18 –Boot Mode Due to this, while command FSM is in \"Wait for CCS state\" or \"Send CCSD\" or \"Boot Mode\", the Status register indicates status as 0 for the bit field \\[7:4\\]."]
    D15 = 15,
}
impl From<CommandFsmStates> for u8 {
    #[inline(always)]
    fn from(variant: CommandFsmStates) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CommandFsmStates {
    type Ux = u8;
}
#[doc = "Field `COMMAND_FSM_STATES` reader - Command FSM states:"]
pub type CommandFsmStatesR = crate::FieldReader<CommandFsmStates>;
impl CommandFsmStatesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CommandFsmStates {
        match self.bits {
            0 => CommandFsmStates::D0,
            1 => CommandFsmStates::D1,
            2 => CommandFsmStates::D2,
            3 => CommandFsmStates::D3,
            4 => CommandFsmStates::D4,
            5 => CommandFsmStates::D5,
            6 => CommandFsmStates::D6,
            7 => CommandFsmStates::D7,
            8 => CommandFsmStates::D8,
            9 => CommandFsmStates::D9,
            10 => CommandFsmStates::D10,
            11 => CommandFsmStates::D11,
            12 => CommandFsmStates::D12,
            13 => CommandFsmStates::D13,
            14 => CommandFsmStates::D14,
            15 => CommandFsmStates::D15,
            _ => unreachable!(),
        }
    }
    #[doc = "idle"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == CommandFsmStates::D0
    }
    #[doc = "send init sequence"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == CommandFsmStates::D1
    }
    #[doc = "Tx cmd start bit"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == CommandFsmStates::D2
    }
    #[doc = "Tx cmd tx bit"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == CommandFsmStates::D3
    }
    #[doc = "Tx cmd index + arg"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == CommandFsmStates::D4
    }
    #[doc = "Tx cmd crc7"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == CommandFsmStates::D5
    }
    #[doc = "Tx cmd end bit"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == CommandFsmStates::D6
    }
    #[doc = "Rx resp start bit"]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == CommandFsmStates::D7
    }
    #[doc = "Rx resp IRQ response"]
    #[inline(always)]
    pub fn is_d8(&self) -> bool {
        *self == CommandFsmStates::D8
    }
    #[doc = "Rx resp tx bit"]
    #[inline(always)]
    pub fn is_d9(&self) -> bool {
        *self == CommandFsmStates::D9
    }
    #[doc = "Rx resp cmd idx"]
    #[inline(always)]
    pub fn is_d10(&self) -> bool {
        *self == CommandFsmStates::D10
    }
    #[doc = "Rx resp data"]
    #[inline(always)]
    pub fn is_d11(&self) -> bool {
        *self == CommandFsmStates::D11
    }
    #[doc = "Rx resp crc7"]
    #[inline(always)]
    pub fn is_d12(&self) -> bool {
        *self == CommandFsmStates::D12
    }
    #[doc = "Rx resp end bit"]
    #[inline(always)]
    pub fn is_d13(&self) -> bool {
        *self == CommandFsmStates::D13
    }
    #[doc = "Cmd path wait NCC"]
    #[inline(always)]
    pub fn is_d14(&self) -> bool {
        *self == CommandFsmStates::D14
    }
    #[doc = "Wait; CMD-to-response turnaround The command FSM state is represented using 19 bits. The STATUS Register\\[7:4\\]
has 4 bits to represent the command FSM states. Using these 4 bits, only 16 states can be represented. Thus three states cannot be represented in the STATUS\\[7:4\\]
register. The three states that are not represented in the STATUS Register\\[7:4\\]
are: a. Bit 16 –Wait for CCS b. Bit 17 –Send CCSD c. Bit 18 –Boot Mode Due to this, while command FSM is in \"Wait for CCS state\" or \"Send CCSD\" or \"Boot Mode\", the Status register indicates status as 0 for the bit field \\[7:4\\]."]
    #[inline(always)]
    pub fn is_d15(&self) -> bool {
        *self == CommandFsmStates::D15
    }
}
#[doc = "Raw selected card_data\\[3\\]; checks whether card is present\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Data3Status {
    #[doc = "0: card not present"]
    B0 = 0,
    #[doc = "1: card present default value is 1 or 0 depending on cdata_in"]
    B1 = 1,
}
impl From<Data3Status> for bool {
    #[inline(always)]
    fn from(variant: Data3Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_3_STATUS` reader - Raw selected card_data\\[3\\]; checks whether card is present"]
pub type Data3StatusR = crate::BitReader<Data3Status>;
impl Data3StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Data3Status {
        match self.bits {
            false => Data3Status::B0,
            true => Data3Status::B1,
        }
    }
    #[doc = "card not present"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Data3Status::B0
    }
    #[doc = "card present default value is 1 or 0 depending on cdata_in"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Data3Status::B1
    }
}
#[doc = "Inverted version of raw selected card_data\\[0\\]\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataBusy {
    #[doc = "0: card data not busy"]
    B0 = 0,
    #[doc = "1: card data busy default value is 1 or 0 depending on cdata_in"]
    B1 = 1,
}
impl From<DataBusy> for bool {
    #[inline(always)]
    fn from(variant: DataBusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_BUSY` reader - Inverted version of raw selected card_data\\[0\\]"]
pub type DataBusyR = crate::BitReader<DataBusy>;
impl DataBusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataBusy {
        match self.bits {
            false => DataBusy::B0,
            true => DataBusy::B1,
        }
    }
    #[doc = "card data not busy"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DataBusy::B0
    }
    #[doc = "card data busy default value is 1 or 0 depending on cdata_in"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DataBusy::B1
    }
}
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
    #[doc = "Bit 0 - FIFO reached Receive watermark level; not qualified with data\n\ntransfer"]
    #[inline(always)]
    pub fn fifo_rx_watermark(&self) -> FifoRxWatermarkR {
        FifoRxWatermarkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO reached Transmit watermark level; not qualified with data\n\ntransfer"]
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
    #[doc = "Bits 4:7 - Command FSM states:"]
    #[inline(always)]
    pub fn command_fsm_states(&self) -> CommandFsmStatesR {
        CommandFsmStatesR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Raw selected card_data\\[3\\]; checks whether card is present"]
    #[inline(always)]
    pub fn data_3_status(&self) -> Data3StatusR {
        Data3StatusR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Inverted version of raw selected card_data\\[0\\]"]
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
