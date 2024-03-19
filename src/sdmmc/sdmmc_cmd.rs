#[doc = "Register `SDMMC_CMD` reader"]
pub type R = crate::R<SdmmcCmdSpec>;
#[doc = "Register `SDMMC_CMD` writer"]
pub type W = crate::W<SdmmcCmdSpec>;
#[doc = "Field `CMD_INDEX` reader - Command index"]
pub type CmdIndexR = crate::FieldReader;
#[doc = "Field `CMD_INDEX` writer - Command index"]
pub type CmdIndexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResponseExpect {
    #[doc = "0: no response expected from card"]
    B0 = 0,
    #[doc = "1: response expected from card"]
    B1 = 1,
}
impl From<ResponseExpect> for bool {
    #[inline(always)]
    fn from(variant: ResponseExpect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESPONSE_EXPECT` reader - "]
pub type ResponseExpectR = crate::BitReader<ResponseExpect>;
impl ResponseExpectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ResponseExpect {
        match self.bits {
            false => ResponseExpect::B0,
            true => ResponseExpect::B1,
        }
    }
    #[doc = "no response expected from card"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ResponseExpect::B0
    }
    #[doc = "response expected from card"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ResponseExpect::B1
    }
}
#[doc = "Field `RESPONSE_EXPECT` writer - "]
pub type ResponseExpectW<'a, REG> = crate::BitWriter<'a, REG, ResponseExpect>;
impl<'a, REG> ResponseExpectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no response expected from card"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ResponseExpect::B0)
    }
    #[doc = "response expected from card"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ResponseExpect::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResponseLength {
    #[doc = "0: short response expected from card"]
    B0 = 0,
    #[doc = "1: long response expected from card"]
    B1 = 1,
}
impl From<ResponseLength> for bool {
    #[inline(always)]
    fn from(variant: ResponseLength) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESPONSE_LENGTH` reader - "]
pub type ResponseLengthR = crate::BitReader<ResponseLength>;
impl ResponseLengthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ResponseLength {
        match self.bits {
            false => ResponseLength::B0,
            true => ResponseLength::B1,
        }
    }
    #[doc = "short response expected from card"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ResponseLength::B0
    }
    #[doc = "long response expected from card"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ResponseLength::B1
    }
}
#[doc = "Field `RESPONSE_LENGTH` writer - "]
pub type ResponseLengthW<'a, REG> = crate::BitWriter<'a, REG, ResponseLength>;
impl<'a, REG> ResponseLengthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "short response expected from card"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ResponseLength::B0)
    }
    #[doc = "long response expected from card"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ResponseLength::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckResponseCrc {
    #[doc = "0: do not check response CRC"]
    B0 = 0,
    #[doc = "1: check response CRC Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller"]
    B1 = 1,
}
impl From<CheckResponseCrc> for bool {
    #[inline(always)]
    fn from(variant: CheckResponseCrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHECK_RESPONSE_CRC` reader - "]
pub type CheckResponseCrcR = crate::BitReader<CheckResponseCrc>;
impl CheckResponseCrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CheckResponseCrc {
        match self.bits {
            false => CheckResponseCrc::B0,
            true => CheckResponseCrc::B1,
        }
    }
    #[doc = "do not check response CRC"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CheckResponseCrc::B0
    }
    #[doc = "check response CRC Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CheckResponseCrc::B1
    }
}
#[doc = "Field `CHECK_RESPONSE_CRC` writer - "]
pub type CheckResponseCrcW<'a, REG> = crate::BitWriter<'a, REG, CheckResponseCrc>;
impl<'a, REG> CheckResponseCrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "do not check response CRC"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CheckResponseCrc::B0)
    }
    #[doc = "check response CRC Some of command responses do not return valid CRC bits. Software should disable CRC checks for those commands in order to disable CRC checking by controller"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CheckResponseCrc::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataExpected {
    #[doc = "0: no data transfer expected (read/write)"]
    B0 = 0,
    #[doc = "1: data transfer expected (read/write)"]
    B1 = 1,
}
impl From<DataExpected> for bool {
    #[inline(always)]
    fn from(variant: DataExpected) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_EXPECTED` reader - "]
pub type DataExpectedR = crate::BitReader<DataExpected>;
impl DataExpectedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataExpected {
        match self.bits {
            false => DataExpected::B0,
            true => DataExpected::B1,
        }
    }
    #[doc = "no data transfer expected (read/write)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DataExpected::B0
    }
    #[doc = "data transfer expected (read/write)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DataExpected::B1
    }
}
#[doc = "Field `DATA_EXPECTED` writer - "]
pub type DataExpectedW<'a, REG> = crate::BitWriter<'a, REG, DataExpected>;
impl<'a, REG> DataExpectedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no data transfer expected (read/write)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DataExpected::B0)
    }
    #[doc = "data transfer expected (read/write)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DataExpected::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wr {
    #[doc = "0: read from card"]
    B0 = 0,
    #[doc = "1: write to card Don't care if no data expected from card."]
    B1 = 1,
}
impl From<Wr> for bool {
    #[inline(always)]
    fn from(variant: Wr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WR` reader - "]
pub type WrR = crate::BitReader<Wr>;
impl WrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wr {
        match self.bits {
            false => Wr::B0,
            true => Wr::B1,
        }
    }
    #[doc = "read from card"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Wr::B0
    }
    #[doc = "write to card Don't care if no data expected from card."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Wr::B1
    }
}
#[doc = "Field `WR` writer - "]
pub type WrW<'a, REG> = crate::BitWriter<'a, REG, Wr>;
impl<'a, REG> WrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "read from card"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Wr::B0)
    }
    #[doc = "write to card Don't care if no data expected from card."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Wr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferMode {
    #[doc = "0: block data transfer command"]
    B0 = 0,
    #[doc = "1: stream data transfer command Don't care if no data expected."]
    B1 = 1,
}
impl From<TransferMode> for bool {
    #[inline(always)]
    fn from(variant: TransferMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRANSFER_MODE` reader - "]
pub type TransferModeR = crate::BitReader<TransferMode>;
impl TransferModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TransferMode {
        match self.bits {
            false => TransferMode::B0,
            true => TransferMode::B1,
        }
    }
    #[doc = "block data transfer command"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TransferMode::B0
    }
    #[doc = "stream data transfer command Don't care if no data expected."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TransferMode::B1
    }
}
#[doc = "Field `TRANSFER_MODE` writer - "]
pub type TransferModeW<'a, REG> = crate::BitWriter<'a, REG, TransferMode>;
impl<'a, REG> TransferModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "block data transfer command"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TransferMode::B0)
    }
    #[doc = "stream data transfer command Don't care if no data expected."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TransferMode::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SendAutoStop {
    #[doc = "0: no stop command sent at end of data transfer"]
    B0 = 0,
    #[doc = "1: send stop command at end of data transfer When set, SDMMC Controller sends stop command to SD_MMC cards at end of data transfer. a. when send_auto_stop bit should be set, since some data transfers do not need explicit stop commands b. open-ended transfers that software should explicitly send to stop command Additionally, when \"resume\" is sent to resume –suspended memory access of SD-Combo card –bit should be set correctly if suspended data transfer needs send_auto_stop. Don't care if no data expected from card."]
    B1 = 1,
}
impl From<SendAutoStop> for bool {
    #[inline(always)]
    fn from(variant: SendAutoStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_AUTO_STOP` reader - "]
pub type SendAutoStopR = crate::BitReader<SendAutoStop>;
impl SendAutoStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SendAutoStop {
        match self.bits {
            false => SendAutoStop::B0,
            true => SendAutoStop::B1,
        }
    }
    #[doc = "no stop command sent at end of data transfer"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SendAutoStop::B0
    }
    #[doc = "send stop command at end of data transfer When set, SDMMC Controller sends stop command to SD_MMC cards at end of data transfer. a. when send_auto_stop bit should be set, since some data transfers do not need explicit stop commands b. open-ended transfers that software should explicitly send to stop command Additionally, when \"resume\" is sent to resume –suspended memory access of SD-Combo card –bit should be set correctly if suspended data transfer needs send_auto_stop. Don't care if no data expected from card."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SendAutoStop::B1
    }
}
#[doc = "Field `SEND_AUTO_STOP` writer - "]
pub type SendAutoStopW<'a, REG> = crate::BitWriter<'a, REG, SendAutoStop>;
impl<'a, REG> SendAutoStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no stop command sent at end of data transfer"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SendAutoStop::B0)
    }
    #[doc = "send stop command at end of data transfer When set, SDMMC Controller sends stop command to SD_MMC cards at end of data transfer. a. when send_auto_stop bit should be set, since some data transfers do not need explicit stop commands b. open-ended transfers that software should explicitly send to stop command Additionally, when \"resume\" is sent to resume –suspended memory access of SD-Combo card –bit should be set correctly if suspended data transfer needs send_auto_stop. Don't care if no data expected from card."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SendAutoStop::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WaitPrvdataComplete {
    #[doc = "0: send command at once, even if previous data transfer has not completed"]
    B0 = 0,
    #[doc = "1: wait for previous data transfer completion before sending command The wait_prvdata_complete = 0 option typically used to query status of card during data transfer or to stop current data transfer; card_number should be same as in previous command."]
    B1 = 1,
}
impl From<WaitPrvdataComplete> for bool {
    #[inline(always)]
    fn from(variant: WaitPrvdataComplete) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAIT_PRVDATA_COMPLETE` reader - "]
pub type WaitPrvdataCompleteR = crate::BitReader<WaitPrvdataComplete>;
impl WaitPrvdataCompleteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WaitPrvdataComplete {
        match self.bits {
            false => WaitPrvdataComplete::B0,
            true => WaitPrvdataComplete::B1,
        }
    }
    #[doc = "send command at once, even if previous data transfer has not completed"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WaitPrvdataComplete::B0
    }
    #[doc = "wait for previous data transfer completion before sending command The wait_prvdata_complete = 0 option typically used to query status of card during data transfer or to stop current data transfer; card_number should be same as in previous command."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WaitPrvdataComplete::B1
    }
}
#[doc = "Field `WAIT_PRVDATA_COMPLETE` writer - "]
pub type WaitPrvdataCompleteW<'a, REG> = crate::BitWriter<'a, REG, WaitPrvdataComplete>;
impl<'a, REG> WaitPrvdataCompleteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "send command at once, even if previous data transfer has not completed"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WaitPrvdataComplete::B0)
    }
    #[doc = "wait for previous data transfer completion before sending command The wait_prvdata_complete = 0 option typically used to query status of card during data transfer or to stop current data transfer; card_number should be same as in previous command."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WaitPrvdataComplete::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopAbortCmd {
    #[doc = "0: neither stop nor abort command to stop current data transfer in progress. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0."]
    B0 = 0,
    #[doc = "1: stop or abort command intended to stop current data transfer in progress. When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state. This is also applicable for Boot mode transfers. To Abort boot mode, this bit should be set along with CMD\\[26\\]
= disable_boot."]
    B1 = 1,
}
impl From<StopAbortCmd> for bool {
    #[inline(always)]
    fn from(variant: StopAbortCmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_ABORT_CMD` reader - "]
pub type StopAbortCmdR = crate::BitReader<StopAbortCmd>;
impl StopAbortCmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StopAbortCmd {
        match self.bits {
            false => StopAbortCmd::B0,
            true => StopAbortCmd::B1,
        }
    }
    #[doc = "neither stop nor abort command to stop current data transfer in progress. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == StopAbortCmd::B0
    }
    #[doc = "stop or abort command intended to stop current data transfer in progress. When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state. This is also applicable for Boot mode transfers. To Abort boot mode, this bit should be set along with CMD\\[26\\]
= disable_boot."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == StopAbortCmd::B1
    }
}
#[doc = "Field `STOP_ABORT_CMD` writer - "]
pub type StopAbortCmdW<'a, REG> = crate::BitWriter<'a, REG, StopAbortCmd>;
impl<'a, REG> StopAbortCmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "neither stop nor abort command to stop current data transfer in progress. If abort is sent to function-number currently selected or not in data-transfer mode, then bit should be set to 0."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(StopAbortCmd::B0)
    }
    #[doc = "stop or abort command intended to stop current data transfer in progress. When open-ended or predefined data transfer is in progress, and host issues stop or abort command to stop data transfer, bit should be set so that command/data state-machines of CIU can return correctly to idle state. This is also applicable for Boot mode transfers. To Abort boot mode, this bit should be set along with CMD\\[26\\]
= disable_boot."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(StopAbortCmd::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SendInitialization {
    #[doc = "0: do not send initialization sequence (80 clocks of 1) before sending this command"]
    B0 = 0,
    #[doc = "1: send initialization sequence before sending this command After power on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card. This bit should not be set for either of the boot modes (alternate or mandatory)."]
    B1 = 1,
}
impl From<SendInitialization> for bool {
    #[inline(always)]
    fn from(variant: SendInitialization) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEND_INITIALIZATION` reader - "]
pub type SendInitializationR = crate::BitReader<SendInitialization>;
impl SendInitializationR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SendInitialization {
        match self.bits {
            false => SendInitialization::B0,
            true => SendInitialization::B1,
        }
    }
    #[doc = "do not send initialization sequence (80 clocks of 1) before sending this command"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SendInitialization::B0
    }
    #[doc = "send initialization sequence before sending this command After power on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card. This bit should not be set for either of the boot modes (alternate or mandatory)."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SendInitialization::B1
    }
}
#[doc = "Field `SEND_INITIALIZATION` writer - "]
pub type SendInitializationW<'a, REG> = crate::BitWriter<'a, REG, SendInitialization>;
impl<'a, REG> SendInitializationW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "do not send initialization sequence (80 clocks of 1) before sending this command"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SendInitialization::B0)
    }
    #[doc = "send initialization sequence before sending this command After power on, 80 clocks must be sent to card for initialization before sending any commands to card. Bit should be set while sending first command to card so that controller will initialize clocks before sending command to card. This bit should not be set for either of the boot modes (alternate or mandatory)."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SendInitialization::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpdateClockRegistersOnly {
    #[doc = "0: normal command sequence"]
    B0 = 0,
    #[doc = "1: do not send commands, just update clock register value into card clock domain Following register values transferred into card clock domain: CLKDIV, CLRSRC, CLKENA. Changes card clocks (change frequency, truncate off or on, and set low-frequency mode); provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, BYTCNT. CIU uses new register values for new command sequence to card. When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards."]
    B1 = 1,
}
impl From<UpdateClockRegistersOnly> for bool {
    #[inline(always)]
    fn from(variant: UpdateClockRegistersOnly) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDATE_CLOCK_REGISTERS_ONLY` reader - "]
pub type UpdateClockRegistersOnlyR = crate::BitReader<UpdateClockRegistersOnly>;
impl UpdateClockRegistersOnlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UpdateClockRegistersOnly {
        match self.bits {
            false => UpdateClockRegistersOnly::B0,
            true => UpdateClockRegistersOnly::B1,
        }
    }
    #[doc = "normal command sequence"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == UpdateClockRegistersOnly::B0
    }
    #[doc = "do not send commands, just update clock register value into card clock domain Following register values transferred into card clock domain: CLKDIV, CLRSRC, CLKENA. Changes card clocks (change frequency, truncate off or on, and set low-frequency mode); provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, BYTCNT. CIU uses new register values for new command sequence to card. When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == UpdateClockRegistersOnly::B1
    }
}
#[doc = "Field `UPDATE_CLOCK_REGISTERS_ONLY` writer - "]
pub type UpdateClockRegistersOnlyW<'a, REG> = crate::BitWriter<'a, REG, UpdateClockRegistersOnly>;
impl<'a, REG> UpdateClockRegistersOnlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal command sequence"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(UpdateClockRegistersOnly::B0)
    }
    #[doc = "do not send commands, just update clock register value into card clock domain Following register values transferred into card clock domain: CLKDIV, CLRSRC, CLKENA. Changes card clocks (change frequency, truncate off or on, and set low-frequency mode); provided in order to change clock frequency or stop clock without having to send command to cards. During normal command sequence, when update_clock_registers_only = 0, following control registers are transferred from BIU to CIU: CMD, CMDARG, TMOUT, CTYPE, BLKSIZ, BYTCNT. CIU uses new register values for new command sequence to card. When bit is set, there are no Command Done interrupts because no command is sent to SD_MMC_CEATA cards."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(UpdateClockRegistersOnly::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadCeataDevice {
    #[doc = "0: Host is not performing read access (RW_REG or RW_BLK) towards CE-ATA device"]
    B0 = 0,
    #[doc = "1: Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. Mobile Storage Host Controller should not indicate read data timeout while waiting for data from CE-ATA device."]
    B1 = 1,
}
impl From<ReadCeataDevice> for bool {
    #[inline(always)]
    fn from(variant: ReadCeataDevice) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ_CEATA_DEVICE` reader - "]
pub type ReadCeataDeviceR = crate::BitReader<ReadCeataDevice>;
impl ReadCeataDeviceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReadCeataDevice {
        match self.bits {
            false => ReadCeataDevice::B0,
            true => ReadCeataDevice::B1,
        }
    }
    #[doc = "Host is not performing read access (RW_REG or RW_BLK) towards CE-ATA device"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ReadCeataDevice::B0
    }
    #[doc = "Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. Mobile Storage Host Controller should not indicate read data timeout while waiting for data from CE-ATA device."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ReadCeataDevice::B1
    }
}
#[doc = "Field `READ_CEATA_DEVICE` writer - "]
pub type ReadCeataDeviceW<'a, REG> = crate::BitWriter<'a, REG, ReadCeataDevice>;
impl<'a, REG> ReadCeataDeviceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Host is not performing read access (RW_REG or RW_BLK) towards CE-ATA device"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ReadCeataDevice::B0)
    }
    #[doc = "Host is performing read access (RW_REG or RW_BLK) towards CE-ATA device Software should set this bit to indicate that CE-ATA device is being accessed for read transfer. This bit is used to disable read data timeout indication while performing CE-ATA read transfers. Maximum value of I/O transmission delay can be no less than 10 seconds. Mobile Storage Host Controller should not indicate read data timeout while waiting for data from CE-ATA device."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ReadCeataDevice::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcsExpected {
    #[doc = "0: Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device"]
    B0 = 0,
    #[doc = "1: Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE- ATA device. If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. Mobile Storage Host Controller sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked."]
    B1 = 1,
}
impl From<CcsExpected> for bool {
    #[inline(always)]
    fn from(variant: CcsExpected) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCS_EXPECTED` reader - "]
pub type CcsExpectedR = crate::BitReader<CcsExpected>;
impl CcsExpectedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CcsExpected {
        match self.bits {
            false => CcsExpected::B0,
            true => CcsExpected::B1,
        }
    }
    #[doc = "Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CcsExpected::B0
    }
    #[doc = "Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE- ATA device. If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. Mobile Storage Host Controller sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CcsExpected::B1
    }
}
#[doc = "Field `CCS_EXPECTED` writer - "]
pub type CcsExpectedW<'a, REG> = crate::BitWriter<'a, REG, CcsExpected>;
impl<'a, REG> CcsExpectedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts are not enabled in CE-ATA device (nIEN = 1 in ATA control register), or command does not expect CCS from device"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CcsExpected::B0)
    }
    #[doc = "Interrupts are enabled in CE-ATA device (nIEN = 0), and RW_BLK command expects command completion signal from CE- ATA device. If the command expects Command Completion Signal (CCS) from the CE-ATA device, the software should set this control bit. Mobile Storage Host Controller sets Data Transfer Over (DTO) bit in RINTSTS register and generates interrupt to host if Data Transfer Over interrupt is not masked."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CcsExpected::B1)
    }
}
#[doc = "Field `ENABLE_BOOT` reader - Enable Boot—this bit should be set only for mandatory boot\n\nmode.When Software sets this bit along with start_cmd, CIU\n\nstarts the boot sequence for the corresponding card by asserting\n\nthe CMD line low. Do NOT set disable_boot and enable_boot\n\ntogether."]
pub type EnableBootR = crate::BitReader;
#[doc = "Field `ENABLE_BOOT` writer - Enable Boot—this bit should be set only for mandatory boot\n\nmode.When Software sets this bit along with start_cmd, CIU\n\nstarts the boot sequence for the corresponding card by asserting\n\nthe CMD line low. Do NOT set disable_boot and enable_boot\n\ntogether."]
pub type EnableBootW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXPECT_BOOT_ACK` reader - Expect Boot Acknowledge. When Software sets this bit along with\n\nenable_boot, CIU expects a boot acknowledge start pattern of 0-\n\n1-0 from the selected card."]
pub type ExpectBootAckR = crate::BitReader;
#[doc = "Field `EXPECT_BOOT_ACK` writer - Expect Boot Acknowledge. When Software sets this bit along with\n\nenable_boot, CIU expects a boot acknowledge start pattern of 0-\n\n1-0 from the selected card."]
pub type ExpectBootAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_BOOT` reader - Disable Boot. When software sets this bit along with start_cmd,\n\nCIU terminates the boot operation. Do NOT set disable_boot and\n\nenable_boot together."]
pub type DisableBootR = crate::BitReader;
#[doc = "Field `DISABLE_BOOT` writer - Disable Boot. When software sets this bit along with start_cmd,\n\nCIU terminates the boot operation. Do NOT set disable_boot and\n\nenable_boot together."]
pub type DisableBootW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Boot Mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootMode {
    #[doc = "0: mandatory Boot operation"]
    B0 = 0,
    #[doc = "1: alternate Boot operation"]
    B1 = 1,
}
impl From<BootMode> for bool {
    #[inline(always)]
    fn from(variant: BootMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOT_MODE` reader - Boot Mode."]
pub type BootModeR = crate::BitReader<BootMode>;
impl BootModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BootMode {
        match self.bits {
            false => BootMode::B0,
            true => BootMode::B1,
        }
    }
    #[doc = "mandatory Boot operation"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BootMode::B0
    }
    #[doc = "alternate Boot operation"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BootMode::B1
    }
}
#[doc = "Field `BOOT_MODE` writer - Boot Mode."]
pub type BootModeW<'a, REG> = crate::BitWriter<'a, REG, BootMode>;
impl<'a, REG> BootModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "mandatory Boot operation"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BootMode::B0)
    }
    #[doc = "alternate Boot operation"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BootMode::B1)
    }
}
#[doc = "Voltage switch bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VoltSwitch {
    #[doc = "0: no voltage switching"]
    B0 = 0,
    #[doc = "1: voltage switching enabled; must be set for CMD11 only"]
    B1 = 1,
}
impl From<VoltSwitch> for bool {
    #[inline(always)]
    fn from(variant: VoltSwitch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOLT_SWITCH` reader - Voltage switch bit."]
pub type VoltSwitchR = crate::BitReader<VoltSwitch>;
impl VoltSwitchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VoltSwitch {
        match self.bits {
            false => VoltSwitch::B0,
            true => VoltSwitch::B1,
        }
    }
    #[doc = "no voltage switching"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VoltSwitch::B0
    }
    #[doc = "voltage switching enabled; must be set for CMD11 only"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VoltSwitch::B1
    }
}
#[doc = "Field `VOLT_SWITCH` writer - Voltage switch bit."]
pub type VoltSwitchW<'a, REG> = crate::BitWriter<'a, REG, VoltSwitch>;
impl<'a, REG> VoltSwitchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no voltage switching"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VoltSwitch::B0)
    }
    #[doc = "voltage switching enabled; must be set for CMD11 only"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VoltSwitch::B1)
    }
}
#[doc = "Use Hold Register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UseHoldReg {
    #[doc = "0: CMD and DATA sent to card bypassing HOLD Register"]
    B0 = 0,
    #[doc = "1: CMD and DATA sent to card through the HOLD Register Note: a. Set to 1'b1 for SDR12 and SDR25 (with non-zero phase-shifted cclk_in_drv); zero phase shift is not allowed in these modes. b. Set to 1'b0 for SDR50, SDR104, and DDR50 (with zero phase- shifted cclk_in_drv). c. Set to 1'b1 for SDR50, SDR104, and DDR50 (with non-zero phase-shifted cclk_in_drv)."]
    B1 = 1,
}
impl From<UseHoldReg> for bool {
    #[inline(always)]
    fn from(variant: UseHoldReg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USE_HOLD_REG` reader - Use Hold Register"]
pub type UseHoldRegR = crate::BitReader<UseHoldReg>;
impl UseHoldRegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UseHoldReg {
        match self.bits {
            false => UseHoldReg::B0,
            true => UseHoldReg::B1,
        }
    }
    #[doc = "CMD and DATA sent to card bypassing HOLD Register"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == UseHoldReg::B0
    }
    #[doc = "CMD and DATA sent to card through the HOLD Register Note: a. Set to 1'b1 for SDR12 and SDR25 (with non-zero phase-shifted cclk_in_drv); zero phase shift is not allowed in these modes. b. Set to 1'b0 for SDR50, SDR104, and DDR50 (with zero phase- shifted cclk_in_drv). c. Set to 1'b1 for SDR50, SDR104, and DDR50 (with non-zero phase-shifted cclk_in_drv)."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == UseHoldReg::B1
    }
}
#[doc = "Field `USE_HOLD_REG` writer - Use Hold Register"]
pub type UseHoldRegW<'a, REG> = crate::BitWriter<'a, REG, UseHoldReg>;
impl<'a, REG> UseHoldRegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMD and DATA sent to card bypassing HOLD Register"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(UseHoldReg::B0)
    }
    #[doc = "CMD and DATA sent to card through the HOLD Register Note: a. Set to 1'b1 for SDR12 and SDR25 (with non-zero phase-shifted cclk_in_drv); zero phase shift is not allowed in these modes. b. Set to 1'b0 for SDR50, SDR104, and DDR50 (with zero phase- shifted cclk_in_drv). c. Set to 1'b1 for SDR50, SDR104, and DDR50 (with non-zero phase-shifted cclk_in_drv)."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(UseHoldReg::B1)
    }
}
#[doc = "Field `START_CMD` reader - Start command. Once command is taken by CIU, bit is cleared.\n\nWhen bit is set, host should not attempt to write to any command\n\nregisters. If write is attempted, hardware lock error is set in raw\n\ninterrupt register.\n\nOnce command is sent and response is received from SD_MMC\n\ncards, Command Done bit is set in raw interrupt register."]
pub type StartCmdR = crate::BitReader;
#[doc = "Field `START_CMD` writer - Start command. Once command is taken by CIU, bit is cleared.\n\nWhen bit is set, host should not attempt to write to any command\n\nregisters. If write is attempted, hardware lock error is set in raw\n\ninterrupt register.\n\nOnce command is sent and response is received from SD_MMC\n\ncards, Command Done bit is set in raw interrupt register."]
pub type StartCmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    pub fn cmd_index(&self) -> CmdIndexR {
        CmdIndexR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn response_expect(&self) -> ResponseExpectR {
        ResponseExpectR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn response_length(&self) -> ResponseLengthR {
        ResponseLengthR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn check_response_crc(&self) -> CheckResponseCrcR {
        CheckResponseCrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn data_expected(&self) -> DataExpectedR {
        DataExpectedR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn wr(&self) -> WrR {
        WrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn transfer_mode(&self) -> TransferModeR {
        TransferModeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn send_auto_stop(&self) -> SendAutoStopR {
        SendAutoStopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wait_prvdata_complete(&self) -> WaitPrvdataCompleteR {
        WaitPrvdataCompleteR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn stop_abort_cmd(&self) -> StopAbortCmdR {
        StopAbortCmdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn send_initialization(&self) -> SendInitializationR {
        SendInitializationR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn update_clock_registers_only(&self) -> UpdateClockRegistersOnlyR {
        UpdateClockRegistersOnlyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn read_ceata_device(&self) -> ReadCeataDeviceR {
        ReadCeataDeviceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ccs_expected(&self) -> CcsExpectedR {
        CcsExpectedR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Boot—this bit should be set only for mandatory boot\n\nmode.When Software sets this bit along with start_cmd, CIU\n\nstarts the boot sequence for the corresponding card by asserting\n\nthe CMD line low. Do NOT set disable_boot and enable_boot\n\ntogether."]
    #[inline(always)]
    pub fn enable_boot(&self) -> EnableBootR {
        EnableBootR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Expect Boot Acknowledge. When Software sets this bit along with\n\nenable_boot, CIU expects a boot acknowledge start pattern of 0-\n\n1-0 from the selected card."]
    #[inline(always)]
    pub fn expect_boot_ack(&self) -> ExpectBootAckR {
        ExpectBootAckR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Disable Boot. When software sets this bit along with start_cmd,\n\nCIU terminates the boot operation. Do NOT set disable_boot and\n\nenable_boot together."]
    #[inline(always)]
    pub fn disable_boot(&self) -> DisableBootR {
        DisableBootR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Boot Mode."]
    #[inline(always)]
    pub fn boot_mode(&self) -> BootModeR {
        BootModeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Voltage switch bit."]
    #[inline(always)]
    pub fn volt_switch(&self) -> VoltSwitchR {
        VoltSwitchR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Use Hold Register"]
    #[inline(always)]
    pub fn use_hold_reg(&self) -> UseHoldRegR {
        UseHoldRegR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Start command. Once command is taken by CIU, bit is cleared.\n\nWhen bit is set, host should not attempt to write to any command\n\nregisters. If write is attempted, hardware lock error is set in raw\n\ninterrupt register.\n\nOnce command is sent and response is received from SD_MMC\n\ncards, Command Done bit is set in raw interrupt register."]
    #[inline(always)]
    pub fn start_cmd(&self) -> StartCmdR {
        StartCmdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_index(&mut self) -> CmdIndexW<SdmmcCmdSpec> {
        CmdIndexW::new(self, 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn response_expect(&mut self) -> ResponseExpectW<SdmmcCmdSpec> {
        ResponseExpectW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn response_length(&mut self) -> ResponseLengthW<SdmmcCmdSpec> {
        ResponseLengthW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn check_response_crc(&mut self) -> CheckResponseCrcW<SdmmcCmdSpec> {
        CheckResponseCrcW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn data_expected(&mut self) -> DataExpectedW<SdmmcCmdSpec> {
        DataExpectedW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn wr(&mut self) -> WrW<SdmmcCmdSpec> {
        WrW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn transfer_mode(&mut self) -> TransferModeW<SdmmcCmdSpec> {
        TransferModeW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn send_auto_stop(&mut self) -> SendAutoStopW<SdmmcCmdSpec> {
        SendAutoStopW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn wait_prvdata_complete(&mut self) -> WaitPrvdataCompleteW<SdmmcCmdSpec> {
        WaitPrvdataCompleteW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn stop_abort_cmd(&mut self) -> StopAbortCmdW<SdmmcCmdSpec> {
        StopAbortCmdW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn send_initialization(&mut self) -> SendInitializationW<SdmmcCmdSpec> {
        SendInitializationW::new(self, 15)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn update_clock_registers_only(&mut self) -> UpdateClockRegistersOnlyW<SdmmcCmdSpec> {
        UpdateClockRegistersOnlyW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn read_ceata_device(&mut self) -> ReadCeataDeviceW<SdmmcCmdSpec> {
        ReadCeataDeviceW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn ccs_expected(&mut self) -> CcsExpectedW<SdmmcCmdSpec> {
        CcsExpectedW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Boot—this bit should be set only for mandatory boot\n\nmode.When Software sets this bit along with start_cmd, CIU\n\nstarts the boot sequence for the corresponding card by asserting\n\nthe CMD line low. Do NOT set disable_boot and enable_boot\n\ntogether."]
    #[inline(always)]
    #[must_use]
    pub fn enable_boot(&mut self) -> EnableBootW<SdmmcCmdSpec> {
        EnableBootW::new(self, 24)
    }
    #[doc = "Bit 25 - Expect Boot Acknowledge. When Software sets this bit along with\n\nenable_boot, CIU expects a boot acknowledge start pattern of 0-\n\n1-0 from the selected card."]
    #[inline(always)]
    #[must_use]
    pub fn expect_boot_ack(&mut self) -> ExpectBootAckW<SdmmcCmdSpec> {
        ExpectBootAckW::new(self, 25)
    }
    #[doc = "Bit 26 - Disable Boot. When software sets this bit along with start_cmd,\n\nCIU terminates the boot operation. Do NOT set disable_boot and\n\nenable_boot together."]
    #[inline(always)]
    #[must_use]
    pub fn disable_boot(&mut self) -> DisableBootW<SdmmcCmdSpec> {
        DisableBootW::new(self, 26)
    }
    #[doc = "Bit 27 - Boot Mode."]
    #[inline(always)]
    #[must_use]
    pub fn boot_mode(&mut self) -> BootModeW<SdmmcCmdSpec> {
        BootModeW::new(self, 27)
    }
    #[doc = "Bit 28 - Voltage switch bit."]
    #[inline(always)]
    #[must_use]
    pub fn volt_switch(&mut self) -> VoltSwitchW<SdmmcCmdSpec> {
        VoltSwitchW::new(self, 28)
    }
    #[doc = "Bit 29 - Use Hold Register"]
    #[inline(always)]
    #[must_use]
    pub fn use_hold_reg(&mut self) -> UseHoldRegW<SdmmcCmdSpec> {
        UseHoldRegW::new(self, 29)
    }
    #[doc = "Bit 31 - Start command. Once command is taken by CIU, bit is cleared.\n\nWhen bit is set, host should not attempt to write to any command\n\nregisters. If write is attempted, hardware lock error is set in raw\n\ninterrupt register.\n\nOnce command is sent and response is received from SD_MMC\n\ncards, Command Done bit is set in raw interrupt register."]
    #[inline(always)]
    #[must_use]
    pub fn start_cmd(&mut self) -> StartCmdW<SdmmcCmdSpec> {
        StartCmdW::new(self, 31)
    }
}
#[doc = "Command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcCmdSpec;
impl crate::RegisterSpec for SdmmcCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_cmd::R`](R) reader structure"]
impl crate::Readable for SdmmcCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_cmd::W`](W) writer structure"]
impl crate::Writable for SdmmcCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_CMD to value 0"]
impl crate::Resettable for SdmmcCmdSpec {
    const RESET_VALUE: u32 = 0;
}
