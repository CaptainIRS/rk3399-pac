#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControllerReset {
    #[doc = "0: no change"]
    B0 = 0,
    #[doc = "1: reset SDMMC controller To reset controller, firmware should set bit to 1. This bit is auto- cleared after two AHB and two cclk_in clock cycles. This resets: a. BIU/CIU interface b. CIU and state machines c. abort_read_data, send_irq_response, and read_wait bits of Control register d. start_cmd bit of Command register Does not affect any registers or DMA interface, or FIFO or host interrupts"]
    B1 = 1,
}
impl From<ControllerReset> for bool {
    #[inline(always)]
    fn from(variant: ControllerReset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONTROLLER_RESET` reader - "]
pub type ControllerResetR = crate::BitReader<ControllerReset>;
impl ControllerResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ControllerReset {
        match self.bits {
            false => ControllerReset::B0,
            true => ControllerReset::B1,
        }
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ControllerReset::B0
    }
    #[doc = "reset SDMMC controller To reset controller, firmware should set bit to 1. This bit is auto- cleared after two AHB and two cclk_in clock cycles. This resets: a. BIU/CIU interface b. CIU and state machines c. abort_read_data, send_irq_response, and read_wait bits of Control register d. start_cmd bit of Command register Does not affect any registers or DMA interface, or FIFO or host interrupts"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ControllerReset::B1
    }
}
#[doc = "Field `CONTROLLER_RESET` writer - "]
pub type ControllerResetW<'a, REG> = crate::BitWriter1C<'a, REG, ControllerReset>;
impl<'a, REG> ControllerResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no change"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ControllerReset::B0)
    }
    #[doc = "reset SDMMC controller To reset controller, firmware should set bit to 1. This bit is auto- cleared after two AHB and two cclk_in clock cycles. This resets: a. BIU/CIU interface b. CIU and state machines c. abort_read_data, send_irq_response, and read_wait bits of Control register d. start_cmd bit of Command register Does not affect any registers or DMA interface, or FIFO or host interrupts"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ControllerReset::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FifoReset {
    #[doc = "0: no change"]
    B0 = 0,
    #[doc = "1: reset to data FIFO To reset FIFO pointers To reset FIFO, firmware should set bit to 1. This bit is auto- cleared after completion of reset operation"]
    B1 = 1,
}
impl From<FifoReset> for bool {
    #[inline(always)]
    fn from(variant: FifoReset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_RESET` reader - "]
pub type FifoResetR = crate::BitReader<FifoReset>;
impl FifoResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FifoReset {
        match self.bits {
            false => FifoReset::B0,
            true => FifoReset::B1,
        }
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FifoReset::B0
    }
    #[doc = "reset to data FIFO To reset FIFO pointers To reset FIFO, firmware should set bit to 1. This bit is auto- cleared after completion of reset operation"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FifoReset::B1
    }
}
#[doc = "Field `FIFO_RESET` writer - "]
pub type FifoResetW<'a, REG> = crate::BitWriter1C<'a, REG, FifoReset>;
impl<'a, REG> FifoResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no change"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FifoReset::B0)
    }
    #[doc = "reset to data FIFO To reset FIFO pointers To reset FIFO, firmware should set bit to 1. This bit is auto- cleared after completion of reset operation"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FifoReset::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaReset {
    #[doc = "0: no change"]
    B0 = 0,
    #[doc = "1: reset internal DMA interface control logic To reset DMA interface, firmware should set bit to 1. This bit is auto-cleared after two AHB clocks."]
    B1 = 1,
}
impl From<DmaReset> for bool {
    #[inline(always)]
    fn from(variant: DmaReset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RESET` reader - "]
pub type DmaResetR = crate::BitReader<DmaReset>;
impl DmaResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaReset {
        match self.bits {
            false => DmaReset::B0,
            true => DmaReset::B1,
        }
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DmaReset::B0
    }
    #[doc = "reset internal DMA interface control logic To reset DMA interface, firmware should set bit to 1. This bit is auto-cleared after two AHB clocks."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DmaReset::B1
    }
}
#[doc = "Field `DMA_RESET` writer - "]
pub type DmaResetW<'a, REG> = crate::BitWriter1C<'a, REG, DmaReset>;
impl<'a, REG> DmaResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no change"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DmaReset::B0)
    }
    #[doc = "reset internal DMA interface control logic To reset DMA interface, firmware should set bit to 1. This bit is auto-cleared after two AHB clocks."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DmaReset::B1)
    }
}
#[doc = "Global interrupt enable/disable bit:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEnable {
    #[doc = "0: disable interrupts"]
    B0 = 0,
    #[doc = "1: enable interrupts The int port is 1 only when this bit is 1 and one or more unmasked interrupts are set."]
    B1 = 1,
}
impl From<IntEnable> for bool {
    #[inline(always)]
    fn from(variant: IntEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_ENABLE` reader - Global interrupt enable/disable bit:"]
pub type IntEnableR = crate::BitReader<IntEnable>;
impl IntEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEnable {
        match self.bits {
            false => IntEnable::B0,
            true => IntEnable::B1,
        }
    }
    #[doc = "disable interrupts"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntEnable::B0
    }
    #[doc = "enable interrupts The int port is 1 only when this bit is 1 and one or more unmasked interrupts are set."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntEnable::B1
    }
}
#[doc = "Field `INT_ENABLE` writer - Global interrupt enable/disable bit:"]
pub type IntEnableW<'a, REG> = crate::BitWriter<'a, REG, IntEnable>;
impl<'a, REG> IntEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable interrupts"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnable::B0)
    }
    #[doc = "enable interrupts The int port is 1 only when this bit is 1 and one or more unmasked interrupts are set."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntEnable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaEnable {
    #[doc = "0: disable DMA transfer mode"]
    B0 = 0,
    #[doc = "1: enable DMA transfer mode Even when DMA mode is enabled, host can still push/pop data into or from FIFO; this should not happen during the normal operation. If there is simultaneous FIFO access from host/DMA, the data coherency is lost. Also, there is no arbitration inside SDMMC Controller to prioritize simultaneous host/DMA access."]
    B1 = 1,
}
impl From<DmaEnable> for bool {
    #[inline(always)]
    fn from(variant: DmaEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_ENABLE` reader - "]
pub type DmaEnableR = crate::BitReader<DmaEnable>;
impl DmaEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaEnable {
        match self.bits {
            false => DmaEnable::B0,
            true => DmaEnable::B1,
        }
    }
    #[doc = "disable DMA transfer mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DmaEnable::B0
    }
    #[doc = "enable DMA transfer mode Even when DMA mode is enabled, host can still push/pop data into or from FIFO; this should not happen during the normal operation. If there is simultaneous FIFO access from host/DMA, the data coherency is lost. Also, there is no arbitration inside SDMMC Controller to prioritize simultaneous host/DMA access."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DmaEnable::B1
    }
}
#[doc = "Field `DMA_ENABLE` writer - "]
pub type DmaEnableW<'a, REG> = crate::BitWriter<'a, REG, DmaEnable>;
impl<'a, REG> DmaEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable DMA transfer mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DmaEnable::B0)
    }
    #[doc = "enable DMA transfer mode Even when DMA mode is enabled, host can still push/pop data into or from FIFO; this should not happen during the normal operation. If there is simultaneous FIFO access from host/DMA, the data coherency is lost. Also, there is no arbitration inside SDMMC Controller to prioritize simultaneous host/DMA access."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DmaEnable::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadWait {
    #[doc = "0: clear read wait"]
    B0 = 0,
    #[doc = "1: assert read wait For sending read-wait to SDIO cards"]
    B1 = 1,
}
impl From<ReadWait> for bool {
    #[inline(always)]
    fn from(variant: ReadWait) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ_WAIT` reader - "]
pub type ReadWaitR = crate::BitReader<ReadWait>;
impl ReadWaitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReadWait {
        match self.bits {
            false => ReadWait::B0,
            true => ReadWait::B1,
        }
    }
    #[doc = "clear read wait"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ReadWait::B0
    }
    #[doc = "assert read wait For sending read-wait to SDIO cards"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ReadWait::B1
    }
}
#[doc = "Field `READ_WAIT` writer - "]
pub type ReadWaitW<'a, REG> = crate::BitWriter<'a, REG, ReadWait>;
impl<'a, REG> ReadWaitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clear read wait"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ReadWait::B0)
    }
    #[doc = "assert read wait For sending read-wait to SDIO cards"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ReadWait::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SendIrqResponse {
    #[doc = "0: no change"]
    B0 = 0,
    #[doc = "1: send auto IRQ response Bit automatically clears once response is sent. To wait for MMC card interrupts, host issues CMD40, and SDMMC Controller waits for interrupt response from MMC card(s). In meantime, if host wants SDMMC Controller to exit waiting for interrupt state, it can set this bit, at which time SDMMC Controller command state-machine sends CMD40 response on bus and returns to idle state."]
    B1 = 1,
}
impl From<SendIrqResponse> for bool {
    #[inline(always)]
    fn from(variant: SendIrqResponse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_IRQ_RESPONSE` reader - "]
pub type SendIrqResponseR = crate::BitReader<SendIrqResponse>;
impl SendIrqResponseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SendIrqResponse {
        match self.bits {
            false => SendIrqResponse::B0,
            true => SendIrqResponse::B1,
        }
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SendIrqResponse::B0
    }
    #[doc = "send auto IRQ response Bit automatically clears once response is sent. To wait for MMC card interrupts, host issues CMD40, and SDMMC Controller waits for interrupt response from MMC card(s). In meantime, if host wants SDMMC Controller to exit waiting for interrupt state, it can set this bit, at which time SDMMC Controller command state-machine sends CMD40 response on bus and returns to idle state."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SendIrqResponse::B1
    }
}
#[doc = "Field `SEND_IRQ_RESPONSE` writer - "]
pub type SendIrqResponseW<'a, REG> = crate::BitWriter<'a, REG, SendIrqResponse>;
impl<'a, REG> SendIrqResponseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no change"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SendIrqResponse::B0)
    }
    #[doc = "send auto IRQ response Bit automatically clears once response is sent. To wait for MMC card interrupts, host issues CMD40, and SDMMC Controller waits for interrupt response from MMC card(s). In meantime, if host wants SDMMC Controller to exit waiting for interrupt state, it can set this bit, at which time SDMMC Controller command state-machine sends CMD40 response on bus and returns to idle state."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SendIrqResponse::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbortReadData {
    #[doc = "0: no change"]
    B0 = 0,
    #[doc = "1: after suspend command is issued during read-transfer, software polls card to find when suspend happened. Once suspend occurs, software sets bit to reset data state-machine, which is waiting for next block of data. Bit automatically clears once data state machine resets to idle. Used in SDIO card suspend sequence."]
    B1 = 1,
}
impl From<AbortReadData> for bool {
    #[inline(always)]
    fn from(variant: AbortReadData) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT_READ_DATA` reader - "]
pub type AbortReadDataR = crate::BitReader<AbortReadData>;
impl AbortReadDataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AbortReadData {
        match self.bits {
            false => AbortReadData::B0,
            true => AbortReadData::B1,
        }
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AbortReadData::B0
    }
    #[doc = "after suspend command is issued during read-transfer, software polls card to find when suspend happened. Once suspend occurs, software sets bit to reset data state-machine, which is waiting for next block of data. Bit automatically clears once data state machine resets to idle. Used in SDIO card suspend sequence."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AbortReadData::B1
    }
}
#[doc = "Field `ABORT_READ_DATA` writer - "]
pub type AbortReadDataW<'a, REG> = crate::BitWriter<'a, REG, AbortReadData>;
impl<'a, REG> AbortReadDataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no change"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AbortReadData::B0)
    }
    #[doc = "after suspend command is issued during read-transfer, software polls card to find when suspend happened. Once suspend occurs, software sets bit to reset data state-machine, which is waiting for next block of data. Bit automatically clears once data state machine resets to idle. Used in SDIO card suspend sequence."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AbortReadData::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SendCcsd {
    #[doc = "0: Clear bit if Mobile Storage Host Controller does not reset the bit."]
    B0 = 0,
    #[doc = "1: Send Command Completion Signal Disable (CCSD) to CE-ATA device When set, Mobile Storage Host Controller sends CCSD to CE-ATA device. Software sets this bit only if current command is expecting CCS (that is, RW_BLK) and interrupts are enabled in CE-ATA device. Once the CCSD pattern is sent to device, Mobile Storage Host Controller automatically clears send_ccsd bit. It also sets Command Done (CD) bit in RINTSTS register and generates interrupt to host if Command Done interrupt is not masked. NOTE: Once send_ccsd bit is set, it takes two card clock cycles to drive the CCSD on the CMD line. Due to this, during the boundary conditions it may happen that CCSD is sent to the CE-ATA device, even if the device signalled CCS"]
    B1 = 1,
}
impl From<SendCcsd> for bool {
    #[inline(always)]
    fn from(variant: SendCcsd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_CCSD` reader - "]
pub type SendCcsdR = crate::BitReader<SendCcsd>;
impl SendCcsdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SendCcsd {
        match self.bits {
            false => SendCcsd::B0,
            true => SendCcsd::B1,
        }
    }
    #[doc = "Clear bit if Mobile Storage Host Controller does not reset the bit."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SendCcsd::B0
    }
    #[doc = "Send Command Completion Signal Disable (CCSD) to CE-ATA device When set, Mobile Storage Host Controller sends CCSD to CE-ATA device. Software sets this bit only if current command is expecting CCS (that is, RW_BLK) and interrupts are enabled in CE-ATA device. Once the CCSD pattern is sent to device, Mobile Storage Host Controller automatically clears send_ccsd bit. It also sets Command Done (CD) bit in RINTSTS register and generates interrupt to host if Command Done interrupt is not masked. NOTE: Once send_ccsd bit is set, it takes two card clock cycles to drive the CCSD on the CMD line. Due to this, during the boundary conditions it may happen that CCSD is sent to the CE-ATA device, even if the device signalled CCS"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SendCcsd::B1
    }
}
#[doc = "Field `SEND_CCSD` writer - "]
pub type SendCcsdW<'a, REG> = crate::BitWriter<'a, REG, SendCcsd>;
impl<'a, REG> SendCcsdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear bit if Mobile Storage Host Controller does not reset the bit."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SendCcsd::B0)
    }
    #[doc = "Send Command Completion Signal Disable (CCSD) to CE-ATA device When set, Mobile Storage Host Controller sends CCSD to CE-ATA device. Software sets this bit only if current command is expecting CCS (that is, RW_BLK) and interrupts are enabled in CE-ATA device. Once the CCSD pattern is sent to device, Mobile Storage Host Controller automatically clears send_ccsd bit. It also sets Command Done (CD) bit in RINTSTS register and generates interrupt to host if Command Done interrupt is not masked. NOTE: Once send_ccsd bit is set, it takes two card clock cycles to drive the CCSD on the CMD line. Due to this, during the boundary conditions it may happen that CCSD is sent to the CE-ATA device, even if the device signalled CCS"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SendCcsd::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SendAutoStopCcsd {
    #[doc = "0: Clear bit if Mobile Storage Host Controller does not reset the bit."]
    B0 = 0,
    #[doc = "1: Send internally generated STOP after sending CCSD to CE-ATA device. NOTE: Always set send_auto_stop_ccsd and send_ccsd bits together send_auto_stop_ccsd should not be set independent of send_ccsd. When set, Mobile Storage Host Controller automatically sends internally- generated STOP command (CMD12) to CE-ATA device. After sending internally-generated STOP command, Auto Command Done (ACD) in RINTSTS is set and generates interrupt to host if Auto Command Done interrupt is not masked. After sending the CCSD, Mobile Storage Host Controller automatically clears send_auto_stop_ccsd bit."]
    B1 = 1,
}
impl From<SendAutoStopCcsd> for bool {
    #[inline(always)]
    fn from(variant: SendAutoStopCcsd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_AUTO_STOP_CCSD` reader - "]
pub type SendAutoStopCcsdR = crate::BitReader<SendAutoStopCcsd>;
impl SendAutoStopCcsdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SendAutoStopCcsd {
        match self.bits {
            false => SendAutoStopCcsd::B0,
            true => SendAutoStopCcsd::B1,
        }
    }
    #[doc = "Clear bit if Mobile Storage Host Controller does not reset the bit."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SendAutoStopCcsd::B0
    }
    #[doc = "Send internally generated STOP after sending CCSD to CE-ATA device. NOTE: Always set send_auto_stop_ccsd and send_ccsd bits together send_auto_stop_ccsd should not be set independent of send_ccsd. When set, Mobile Storage Host Controller automatically sends internally- generated STOP command (CMD12) to CE-ATA device. After sending internally-generated STOP command, Auto Command Done (ACD) in RINTSTS is set and generates interrupt to host if Auto Command Done interrupt is not masked. After sending the CCSD, Mobile Storage Host Controller automatically clears send_auto_stop_ccsd bit."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SendAutoStopCcsd::B1
    }
}
#[doc = "Field `SEND_AUTO_STOP_CCSD` writer - "]
pub type SendAutoStopCcsdW<'a, REG> = crate::BitWriter<'a, REG, SendAutoStopCcsd>;
impl<'a, REG> SendAutoStopCcsdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear bit if Mobile Storage Host Controller does not reset the bit."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SendAutoStopCcsd::B0)
    }
    #[doc = "Send internally generated STOP after sending CCSD to CE-ATA device. NOTE: Always set send_auto_stop_ccsd and send_ccsd bits together send_auto_stop_ccsd should not be set independent of send_ccsd. When set, Mobile Storage Host Controller automatically sends internally- generated STOP command (CMD12) to CE-ATA device. After sending internally-generated STOP command, Auto Command Done (ACD) in RINTSTS is set and generates interrupt to host if Auto Command Done interrupt is not masked. After sending the CCSD, Mobile Storage Host Controller automatically clears send_auto_stop_ccsd bit."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SendAutoStopCcsd::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CeataDeviceInterruptStatus {
    #[doc = "0: Interrupts not enabled in CE-ATA device (nIEN = 1 in ATA control register)"]
    B0 = 0,
    #[doc = "1: Interrupts are enabled in CE-ATA device (nIEN = 0 in ATA control register) Software should appropriately write to this bit after power-on reset or any other reset to CE-ATA device. After reset, usually CE-ATA device interrupt is disabled (nIEN = 1). If the host enables CE-ATA device interrupt, then software should set this bit."]
    B1 = 1,
}
impl From<CeataDeviceInterruptStatus> for bool {
    #[inline(always)]
    fn from(variant: CeataDeviceInterruptStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEATA_DEVICE_INTERRUPT_STATUS` reader - "]
pub type CeataDeviceInterruptStatusR = crate::BitReader<CeataDeviceInterruptStatus>;
impl CeataDeviceInterruptStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CeataDeviceInterruptStatus {
        match self.bits {
            false => CeataDeviceInterruptStatus::B0,
            true => CeataDeviceInterruptStatus::B1,
        }
    }
    #[doc = "Interrupts not enabled in CE-ATA device (nIEN = 1 in ATA control register)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CeataDeviceInterruptStatus::B0
    }
    #[doc = "Interrupts are enabled in CE-ATA device (nIEN = 0 in ATA control register) Software should appropriately write to this bit after power-on reset or any other reset to CE-ATA device. After reset, usually CE-ATA device interrupt is disabled (nIEN = 1). If the host enables CE-ATA device interrupt, then software should set this bit."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CeataDeviceInterruptStatus::B1
    }
}
#[doc = "Field `CEATA_DEVICE_INTERRUPT_STATUS` writer - "]
pub type CeataDeviceInterruptStatusW<'a, REG> =
    crate::BitWriter<'a, REG, CeataDeviceInterruptStatus>;
impl<'a, REG> CeataDeviceInterruptStatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts not enabled in CE-ATA device (nIEN = 1 in ATA control register)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CeataDeviceInterruptStatus::B0)
    }
    #[doc = "Interrupts are enabled in CE-ATA device (nIEN = 0 in ATA control register) Software should appropriately write to this bit after power-on reset or any other reset to CE-ATA device. After reset, usually CE-ATA device interrupt is disabled (nIEN = 1). If the host enables CE-ATA device interrupt, then software should set this bit."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CeataDeviceInterruptStatus::B1)
    }
}
#[doc = "Present only for the Internal DMAC configuration; else, it is\n\nreserved.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UseInternalDmac {
    #[doc = "0: The host performs data transfers through the slave interface"]
    B0 = 0,
    #[doc = "1: Internal DMAC used for data transfe"]
    B1 = 1,
}
impl From<UseInternalDmac> for bool {
    #[inline(always)]
    fn from(variant: UseInternalDmac) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USE_INTERNAL_DMAC` reader - Present only for the Internal DMAC configuration; else, it is\n\nreserved."]
pub type UseInternalDmacR = crate::BitReader<UseInternalDmac>;
impl UseInternalDmacR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UseInternalDmac {
        match self.bits {
            false => UseInternalDmac::B0,
            true => UseInternalDmac::B1,
        }
    }
    #[doc = "The host performs data transfers through the slave interface"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == UseInternalDmac::B0
    }
    #[doc = "Internal DMAC used for data transfe"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == UseInternalDmac::B1
    }
}
#[doc = "Field `USE_INTERNAL_DMAC` writer - Present only for the Internal DMAC configuration; else, it is\n\nreserved."]
pub type UseInternalDmacW<'a, REG> = crate::BitWriter<'a, REG, UseInternalDmac>;
impl<'a, REG> UseInternalDmacW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The host performs data transfers through the slave interface"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(UseInternalDmac::B0)
    }
    #[doc = "Internal DMAC used for data transfe"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(UseInternalDmac::B1)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn controller_reset(&self) -> ControllerResetR {
        ControllerResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fifo_reset(&self) -> FifoResetR {
        FifoResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma_reset(&self) -> DmaResetR {
        DmaResetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Global interrupt enable/disable bit:"]
    #[inline(always)]
    pub fn int_enable(&self) -> IntEnableR {
        IntEnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dma_enable(&self) -> DmaEnableR {
        DmaEnableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn read_wait(&self) -> ReadWaitR {
        ReadWaitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn send_irq_response(&self) -> SendIrqResponseR {
        SendIrqResponseR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn abort_read_data(&self) -> AbortReadDataR {
        AbortReadDataR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn send_ccsd(&self) -> SendCcsdR {
        SendCcsdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn send_auto_stop_ccsd(&self) -> SendAutoStopCcsdR {
        SendAutoStopCcsdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ceata_device_interrupt_status(&self) -> CeataDeviceInterruptStatusR {
        CeataDeviceInterruptStatusR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 25 - Present only for the Internal DMAC configuration; else, it is\n\nreserved."]
    #[inline(always)]
    pub fn use_internal_dmac(&self) -> UseInternalDmacR {
        UseInternalDmacR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn controller_reset(&mut self) -> ControllerResetW<CtrlSpec> {
        ControllerResetW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_reset(&mut self) -> FifoResetW<CtrlSpec> {
        FifoResetW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dma_reset(&mut self) -> DmaResetW<CtrlSpec> {
        DmaResetW::new(self, 2)
    }
    #[doc = "Bit 4 - Global interrupt enable/disable bit:"]
    #[inline(always)]
    #[must_use]
    pub fn int_enable(&mut self) -> IntEnableW<CtrlSpec> {
        IntEnableW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn dma_enable(&mut self) -> DmaEnableW<CtrlSpec> {
        DmaEnableW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn read_wait(&mut self) -> ReadWaitW<CtrlSpec> {
        ReadWaitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn send_irq_response(&mut self) -> SendIrqResponseW<CtrlSpec> {
        SendIrqResponseW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn abort_read_data(&mut self) -> AbortReadDataW<CtrlSpec> {
        AbortReadDataW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn send_ccsd(&mut self) -> SendCcsdW<CtrlSpec> {
        SendCcsdW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn send_auto_stop_ccsd(&mut self) -> SendAutoStopCcsdW<CtrlSpec> {
        SendAutoStopCcsdW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ceata_device_interrupt_status(&mut self) -> CeataDeviceInterruptStatusW<CtrlSpec> {
        CeataDeviceInterruptStatusW::new(self, 11)
    }
    #[doc = "Bit 25 - Present only for the Internal DMAC configuration; else, it is\n\nreserved."]
    #[inline(always)]
    #[must_use]
    pub fn use_internal_dmac(&mut self) -> UseInternalDmacW<CtrlSpec> {
        UseInternalDmacW::new(self, 25)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07;
}
#[doc = "`reset()` method sets CTRL to value 0x0100_0000"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
