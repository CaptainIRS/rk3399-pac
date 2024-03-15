#[doc = "Register `EMMCCORE_PRESTS` reader"]
pub type R = crate::R<EmmccorePrestsSpec>;
#[doc = "Register `EMMCCORE_PRESTS` writer"]
pub type W = crate::W<EmmccorePrestsSpec>;
#[doc = "Field `CMDINHIBIT` reader - If this bit is 0, it indicates the CMD line is not in use and the HC can issue a SD command using the CMD line. This bit is set immediately after the Command register is written. This bit is cleared when the command response is received. Even if the Command Inhibit (DAT) is set to 1, Commands using only the CMD line can be issued if this bit is 0. Changing from 1 to 0 generates a Command complete interrupt in the Normal Interrupt Status register. If the HC cannot issue the command because of a command conflict error or because of Command Not Issued By Auto CMD12 Error, this bit shall remain 1 and the Command Complete is not set. Status issuing Auto CMD12 is not read from this bit. Auto CMD12 and Auto CMD23 consist of two responses. In this case, this bit is not cleared by the response of CMD12 or CMD23 but cleared by the response of a read/write command. Status issuing Auto CMD12 is not read from this bit. So if a command is issued during Auto CMD12 operation, Host Controller shall manage to issue two commands: CMD12 and a command set by Command register."]
pub type CmdinhibitR = crate::BitReader;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datinhibit {
    #[doc = "1: Can issue command which uses the DAT line This status bit is generated if either the DAT Line Active or the Read transfer Active is set to 1. If this bit is 0, it indicates the HC can issue the next SD command. Commands with busy signal belong to Command Inhibit (DAT) (ex. R1b, R5b type). Changing from 1 to 0 generates a Transfer Complete interrupt in the Normal interrupt status register. Note: The SD Host Driver can save registers in the range of 000- 00Dh for a suspend transaction after this bit has changed from 1 to 0."]
    B1 = 1,
    #[doc = "0: Can issue command which uses the DAT line This status bit is generated if either the DAT Line Active or the Read transfer Active is set to 1. If this bit is 0, it indicates the HC can issue the next SD command. Commands with busy signal belong to Command Inhibit (DAT) (ex. R1b, R5b type). Changing from 1 to 0 generates a Transfer Complete interrupt in the Normal interrupt status register. Note: The SD Host Driver can save registers in the range of 000- 00Dh for a suspend transaction after this bit has changed from 1 to 0."]
    B0 = 0,
}
impl From<Datinhibit> for bool {
    #[inline(always)]
    fn from(variant: Datinhibit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATINHIBIT` reader - "]
pub type DatinhibitR = crate::BitReader<Datinhibit>;
impl DatinhibitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datinhibit {
        match self.bits {
            true => Datinhibit::B1,
            false => Datinhibit::B0,
        }
    }
    #[doc = "Can issue command which uses the DAT line This status bit is generated if either the DAT Line Active or the Read transfer Active is set to 1. If this bit is 0, it indicates the HC can issue the next SD command. Commands with busy signal belong to Command Inhibit (DAT) (ex. R1b, R5b type). Changing from 1 to 0 generates a Transfer Complete interrupt in the Normal interrupt status register. Note: The SD Host Driver can save registers in the range of 000- 00Dh for a suspend transaction after this bit has changed from 1 to 0."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Datinhibit::B1
    }
    #[doc = "Can issue command which uses the DAT line This status bit is generated if either the DAT Line Active or the Read transfer Active is set to 1. If this bit is 0, it indicates the HC can issue the next SD command. Commands with busy signal belong to Command Inhibit (DAT) (ex. R1b, R5b type). Changing from 1 to 0 generates a Transfer Complete interrupt in the Normal interrupt status register. Note: The SD Host Driver can save registers in the range of 000- 00Dh for a suspend transaction after this bit has changed from 1 to 0."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Datinhibit::B0
    }
}
#[doc = "This bit indicates whether one of the DAT line on SD bus is in use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datlineactive {
    #[doc = "1: DAT line inactive"]
    B1 = 1,
    #[doc = "0: DAT line inactive"]
    B0 = 0,
}
impl From<Datlineactive> for bool {
    #[inline(always)]
    fn from(variant: Datlineactive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATLINEACTIVE` reader - This bit indicates whether one of the DAT line on SD bus is in use."]
pub type DatlineactiveR = crate::BitReader<Datlineactive>;
impl DatlineactiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datlineactive {
        match self.bits {
            true => Datlineactive::B1,
            false => Datlineactive::B0,
        }
    }
    #[doc = "DAT line inactive"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Datlineactive::B1
    }
    #[doc = "DAT line inactive"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Datlineactive::B0
    }
}
#[doc = "Re-Tuning Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Retuningreq {
    #[doc = "1: Fixed or well tuned sampling clock Host Controller may request Host Driver to execute re-tuning sequence by setting this bit when the data window is shifted by temperature drift and a tuned sampling point does not have a good margin to receive correct data. This bit is cleared when a command is issued with setting Execute Tuning in the Host Control 2 register. Changing of this bit from 0 to 1 generates Re-Tuning Event. Refer to Normal Interrupt registers for more detail. This bit isn't set to 1 if Sampling Clock Select in the Host Control 2 register is set to 0 (using fixed sampling clock)."]
    B1 = 1,
    #[doc = "0: Fixed or well tuned sampling clock Host Controller may request Host Driver to execute re-tuning sequence by setting this bit when the data window is shifted by temperature drift and a tuned sampling point does not have a good margin to receive correct data. This bit is cleared when a command is issued with setting Execute Tuning in the Host Control 2 register. Changing of this bit from 0 to 1 generates Re-Tuning Event. Refer to Normal Interrupt registers for more detail. This bit isn't set to 1 if Sampling Clock Select in the Host Control 2 register is set to 0 (using fixed sampling clock)."]
    B0 = 0,
}
impl From<Retuningreq> for bool {
    #[inline(always)]
    fn from(variant: Retuningreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RETUNINGREQ` reader - Re-Tuning Request"]
pub type RetuningreqR = crate::BitReader<Retuningreq>;
impl RetuningreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Retuningreq {
        match self.bits {
            true => Retuningreq::B1,
            false => Retuningreq::B0,
        }
    }
    #[doc = "Fixed or well tuned sampling clock Host Controller may request Host Driver to execute re-tuning sequence by setting this bit when the data window is shifted by temperature drift and a tuned sampling point does not have a good margin to receive correct data. This bit is cleared when a command is issued with setting Execute Tuning in the Host Control 2 register. Changing of this bit from 0 to 1 generates Re-Tuning Event. Refer to Normal Interrupt registers for more detail. This bit isn't set to 1 if Sampling Clock Select in the Host Control 2 register is set to 0 (using fixed sampling clock)."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Retuningreq::B1
    }
    #[doc = "Fixed or well tuned sampling clock Host Controller may request Host Driver to execute re-tuning sequence by setting this bit when the data window is shifted by temperature drift and a tuned sampling point does not have a good margin to receive correct data. This bit is cleared when a command is issued with setting Execute Tuning in the Host Control 2 register. Changing of this bit from 0 to 1 generates Re-Tuning Event. Refer to Normal Interrupt registers for more detail. This bit isn't set to 1 if Sampling Clock Select in the Host Control 2 register is set to 0 (using fixed sampling clock)."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Retuningreq::B0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Writetransactive {
    #[doc = "1: No valid data This status indicates a write transfer is active. If this bit is 0, it means no valid write data exists in the HC. This bit is set in either of the following cases: a. After the end bit of the write command. b. When writing a 1 to Continue Request in the Block Gap Control register to restart a write transfer. This bit is cleared in either of the following cases: a. After getting the CRC status of the last data block as specified by the transfer count (Single or Multiple). b. After getting a CRC status of any block where data transmission is about to be stopped by a Stop At Block Gap Request. During a write transaction, a Block Gap Event interrupt is generated when this bit is changed to 0, as a result of the Stop At Block Gap Request being set. This status is useful for the HD in determining when to issue commands during write busy."]
    B1 = 1,
    #[doc = "0: No valid data This status indicates a write transfer is active. If this bit is 0, it means no valid write data exists in the HC. This bit is set in either of the following cases: a. After the end bit of the write command. b. When writing a 1 to Continue Request in the Block Gap Control register to restart a write transfer. This bit is cleared in either of the following cases: a. After getting the CRC status of the last data block as specified by the transfer count (Single or Multiple). b. After getting a CRC status of any block where data transmission is about to be stopped by a Stop At Block Gap Request. During a write transaction, a Block Gap Event interrupt is generated when this bit is changed to 0, as a result of the Stop At Block Gap Request being set. This status is useful for the HD in determining when to issue commands during write busy."]
    B0 = 0,
}
impl From<Writetransactive> for bool {
    #[inline(always)]
    fn from(variant: Writetransactive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITETRANSACTIVE` reader - "]
pub type WritetransactiveR = crate::BitReader<Writetransactive>;
impl WritetransactiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Writetransactive {
        match self.bits {
            true => Writetransactive::B1,
            false => Writetransactive::B0,
        }
    }
    #[doc = "No valid data This status indicates a write transfer is active. If this bit is 0, it means no valid write data exists in the HC. This bit is set in either of the following cases: a. After the end bit of the write command. b. When writing a 1 to Continue Request in the Block Gap Control register to restart a write transfer. This bit is cleared in either of the following cases: a. After getting the CRC status of the last data block as specified by the transfer count (Single or Multiple). b. After getting a CRC status of any block where data transmission is about to be stopped by a Stop At Block Gap Request. During a write transaction, a Block Gap Event interrupt is generated when this bit is changed to 0, as a result of the Stop At Block Gap Request being set. This status is useful for the HD in determining when to issue commands during write busy."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Writetransactive::B1
    }
    #[doc = "No valid data This status indicates a write transfer is active. If this bit is 0, it means no valid write data exists in the HC. This bit is set in either of the following cases: a. After the end bit of the write command. b. When writing a 1 to Continue Request in the Block Gap Control register to restart a write transfer. This bit is cleared in either of the following cases: a. After getting the CRC status of the last data block as specified by the transfer count (Single or Multiple). b. After getting a CRC status of any block where data transmission is about to be stopped by a Stop At Block Gap Request. During a write transaction, a Block Gap Event interrupt is generated when this bit is changed to 0, as a result of the Stop At Block Gap Request being set. This status is useful for the HD in determining when to issue commands during write busy."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Writetransactive::B0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Readtransactive {
    #[doc = "1: No valid data This status is used for detecting completion of a read transfer. This bit is set to 1 for either of the following conditions: a. After the end bit of the read command b. When writing a 1 to continue Request in the Block Gap Control register to restart a read transfer This bit is cleared to 0 for either of the following conditions: a. When the last data block as specified by block length is transferred to the system. b. When all valid data blocks have been transferred to the system and no current block transfers are being sent as a result of the Stop At Block Gap Request set to 1. A transfer complete interrupt is generated when this bit changes to 0."]
    B1 = 1,
    #[doc = "0: No valid data This status is used for detecting completion of a read transfer. This bit is set to 1 for either of the following conditions: a. After the end bit of the read command b. When writing a 1 to continue Request in the Block Gap Control register to restart a read transfer This bit is cleared to 0 for either of the following conditions: a. When the last data block as specified by block length is transferred to the system. b. When all valid data blocks have been transferred to the system and no current block transfers are being sent as a result of the Stop At Block Gap Request set to 1. A transfer complete interrupt is generated when this bit changes to 0."]
    B0 = 0,
}
impl From<Readtransactive> for bool {
    #[inline(always)]
    fn from(variant: Readtransactive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READTRANSACTIVE` reader - "]
pub type ReadtransactiveR = crate::BitReader<Readtransactive>;
impl ReadtransactiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Readtransactive {
        match self.bits {
            true => Readtransactive::B1,
            false => Readtransactive::B0,
        }
    }
    #[doc = "No valid data This status is used for detecting completion of a read transfer. This bit is set to 1 for either of the following conditions: a. After the end bit of the read command b. When writing a 1 to continue Request in the Block Gap Control register to restart a read transfer This bit is cleared to 0 for either of the following conditions: a. When the last data block as specified by block length is transferred to the system. b. When all valid data blocks have been transferred to the system and no current block transfers are being sent as a result of the Stop At Block Gap Request set to 1. A transfer complete interrupt is generated when this bit changes to 0."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Readtransactive::B1
    }
    #[doc = "No valid data This status is used for detecting completion of a read transfer. This bit is set to 1 for either of the following conditions: a. After the end bit of the read command b. When writing a 1 to continue Request in the Block Gap Control register to restart a read transfer This bit is cleared to 0 for either of the following conditions: a. When the last data block as specified by block length is transferred to the system. b. When all valid data blocks have been transferred to the system and no current block transfers are being sent as a result of the Stop At Block Gap Request set to 1. A transfer complete interrupt is generated when this bit changes to 0."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Readtransactive::B0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufferwriteenable {
    #[doc = "0: Write Enable This status is used for non-DMA write transfers. This read only flag indicates if space is available for write data. If this bit is 1, data can be written to the buffer. A change of this bit from 1 to 0 occurs when all the block data is written to the buffer. A change of this bit from 0 to 1 occurs when top of block data can be written to the buffer and generates the Buffer Write Ready Interrupt."]
    B0 = 0,
    #[doc = "1: Write Enable This status is used for non-DMA write transfers. This read only flag indicates if space is available for write data. If this bit is 1, data can be written to the buffer. A change of this bit from 1 to 0 occurs when all the block data is written to the buffer. A change of this bit from 0 to 1 occurs when top of block data can be written to the buffer and generates the Buffer Write Ready Interrupt."]
    B1 = 1,
}
impl From<Bufferwriteenable> for bool {
    #[inline(always)]
    fn from(variant: Bufferwriteenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFFERWRITEENABLE` reader - "]
pub type BufferwriteenableR = crate::BitReader<Bufferwriteenable>;
impl BufferwriteenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufferwriteenable {
        match self.bits {
            false => Bufferwriteenable::B0,
            true => Bufferwriteenable::B1,
        }
    }
    #[doc = "Write Enable This status is used for non-DMA write transfers. This read only flag indicates if space is available for write data. If this bit is 1, data can be written to the buffer. A change of this bit from 1 to 0 occurs when all the block data is written to the buffer. A change of this bit from 0 to 1 occurs when top of block data can be written to the buffer and generates the Buffer Write Ready Interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bufferwriteenable::B0
    }
    #[doc = "Write Enable This status is used for non-DMA write transfers. This read only flag indicates if space is available for write data. If this bit is 1, data can be written to the buffer. A change of this bit from 1 to 0 occurs when all the block data is written to the buffer. A change of this bit from 0 to 1 occurs when top of block data can be written to the buffer and generates the Buffer Write Ready Interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bufferwriteenable::B1
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufferreadenable {
    #[doc = "0: Read Enable This status is used for non-DMA read transfers. This read only flag indicates that valid data exists in the host side buffer status. If this bit is 1, readable data exists in the buffer. A change of this bit from 1 to 0 occurs when all the block data is read from the buffer. A change of this bit from 0 to 1 occurs when all the block data is ready in the buffer and generates the Buffer Read Ready Interrupt."]
    B0 = 0,
    #[doc = "1: Read Enable This status is used for non-DMA read transfers. This read only flag indicates that valid data exists in the host side buffer status. If this bit is 1, readable data exists in the buffer. A change of this bit from 1 to 0 occurs when all the block data is read from the buffer. A change of this bit from 0 to 1 occurs when all the block data is ready in the buffer and generates the Buffer Read Ready Interrupt."]
    B1 = 1,
}
impl From<Bufferreadenable> for bool {
    #[inline(always)]
    fn from(variant: Bufferreadenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFFERREADENABLE` reader - "]
pub type BufferreadenableR = crate::BitReader<Bufferreadenable>;
impl BufferreadenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufferreadenable {
        match self.bits {
            false => Bufferreadenable::B0,
            true => Bufferreadenable::B1,
        }
    }
    #[doc = "Read Enable This status is used for non-DMA read transfers. This read only flag indicates that valid data exists in the host side buffer status. If this bit is 1, readable data exists in the buffer. A change of this bit from 1 to 0 occurs when all the block data is read from the buffer. A change of this bit from 0 to 1 occurs when all the block data is ready in the buffer and generates the Buffer Read Ready Interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bufferreadenable::B0
    }
    #[doc = "Read Enable This status is used for non-DMA read transfers. This read only flag indicates that valid data exists in the host side buffer status. If this bit is 1, readable data exists in the buffer. A change of this bit from 1 to 0 occurs when all the block data is read from the buffer. A change of this bit from 0 to 1 occurs when all the block data is ready in the buffer and generates the Buffer Read Ready Interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bufferreadenable::B1
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardinserted {
    #[doc = "0: Card Inserted This bit indicates whether a card has been inserted. Changing from 0 to 1 generates a Card Insertion interrupt in the Normal Interrupt Status register and changing from 1 to 0 generates a Card Removal Interrupt in the Normal Interrupt Status register. The Software Reset For All in the Software Reset register shall not affect this bit. If a Card is removed while its power is on and its clock is oscillating, the HC shall clear SD Bus Power in the Power Control register and SD Clock Enable in the Clock control register. In addition the HD should clear the HC by the Software Reset For All in Software register. The card detect is active regardless of the SD Bus Power."]
    B0 = 0,
    #[doc = "1: Card Inserted This bit indicates whether a card has been inserted. Changing from 0 to 1 generates a Card Insertion interrupt in the Normal Interrupt Status register and changing from 1 to 0 generates a Card Removal Interrupt in the Normal Interrupt Status register. The Software Reset For All in the Software Reset register shall not affect this bit. If a Card is removed while its power is on and its clock is oscillating, the HC shall clear SD Bus Power in the Power Control register and SD Clock Enable in the Clock control register. In addition the HD should clear the HC by the Software Reset For All in Software register. The card detect is active regardless of the SD Bus Power."]
    B1 = 1,
}
impl From<Cardinserted> for bool {
    #[inline(always)]
    fn from(variant: Cardinserted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDINSERTED` reader - "]
pub type CardinsertedR = crate::BitReader<Cardinserted>;
impl CardinsertedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardinserted {
        match self.bits {
            false => Cardinserted::B0,
            true => Cardinserted::B1,
        }
    }
    #[doc = "Card Inserted This bit indicates whether a card has been inserted. Changing from 0 to 1 generates a Card Insertion interrupt in the Normal Interrupt Status register and changing from 1 to 0 generates a Card Removal Interrupt in the Normal Interrupt Status register. The Software Reset For All in the Software Reset register shall not affect this bit. If a Card is removed while its power is on and its clock is oscillating, the HC shall clear SD Bus Power in the Power Control register and SD Clock Enable in the Clock control register. In addition the HD should clear the HC by the Software Reset For All in Software register. The card detect is active regardless of the SD Bus Power."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cardinserted::B0
    }
    #[doc = "Card Inserted This bit indicates whether a card has been inserted. Changing from 0 to 1 generates a Card Insertion interrupt in the Normal Interrupt Status register and changing from 1 to 0 generates a Card Removal Interrupt in the Normal Interrupt Status register. The Software Reset For All in the Software Reset register shall not affect this bit. If a Card is removed while its power is on and its clock is oscillating, the HC shall clear SD Bus Power in the Power Control register and SD Clock Enable in the Clock control register. In addition the HD should clear the HC by the Software Reset For All in Software register. The card detect is active regardless of the SD Bus Power."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cardinserted::B1
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardstatestable {
    #[doc = "0: No Card or Inserted This bit is used for testing. If it is 0, the Card Detect Pin Level is not stable. If this bit is set to 1, it means the Card Detect Pin Level is stable. The Software Reset For All in the Software Reset Register shall not affect this bit."]
    B0 = 0,
    #[doc = "1: No Card or Inserted This bit is used for testing. If it is 0, the Card Detect Pin Level is not stable. If this bit is set to 1, it means the Card Detect Pin Level is stable. The Software Reset For All in the Software Reset Register shall not affect this bit."]
    B1 = 1,
}
impl From<Cardstatestable> for bool {
    #[inline(always)]
    fn from(variant: Cardstatestable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDSTATESTABLE` reader - "]
pub type CardstatestableR = crate::BitReader<Cardstatestable>;
impl CardstatestableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardstatestable {
        match self.bits {
            false => Cardstatestable::B0,
            true => Cardstatestable::B1,
        }
    }
    #[doc = "No Card or Inserted This bit is used for testing. If it is 0, the Card Detect Pin Level is not stable. If this bit is set to 1, it means the Card Detect Pin Level is stable. The Software Reset For All in the Software Reset Register shall not affect this bit."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cardstatestable::B0
    }
    #[doc = "No Card or Inserted This bit is used for testing. If it is 0, the Card Detect Pin Level is not stable. If this bit is set to 1, it means the Card Detect Pin Level is stable. The Software Reset For All in the Software Reset Register shall not affect this bit."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cardstatestable::B1
    }
}
#[doc = "This bit reflects the inverse value of the SDCD# pin.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Carddetectpinlevel {
    #[doc = "0: Card present (SDCD# = 0)"]
    B0 = 0,
    #[doc = "1: Card present (SDCD# = 0)"]
    B1 = 1,
}
impl From<Carddetectpinlevel> for bool {
    #[inline(always)]
    fn from(variant: Carddetectpinlevel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDDETECTPINLEVEL` reader - This bit reflects the inverse value of the SDCD# pin."]
pub type CarddetectpinlevelR = crate::BitReader<Carddetectpinlevel>;
impl CarddetectpinlevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Carddetectpinlevel {
        match self.bits {
            false => Carddetectpinlevel::B0,
            true => Carddetectpinlevel::B1,
        }
    }
    #[doc = "Card present (SDCD# = 0)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Carddetectpinlevel::B0
    }
    #[doc = "Card present (SDCD# = 0)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Carddetectpinlevel::B1
    }
}
#[doc = "Write Protect Switch Pin Level. The Write Protect Switch is supported for memory and combo cards. This bit reflects the SDWP# pin.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrprtswpinlvl {
    #[doc = "0: Write enabled (SDWP# = 1)"]
    B0 = 0,
    #[doc = "1: Write enabled (SDWP# = 1)"]
    B1 = 1,
}
impl From<Wrprtswpinlvl> for bool {
    #[inline(always)]
    fn from(variant: Wrprtswpinlvl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPRTSWPINLVL` reader - Write Protect Switch Pin Level. The Write Protect Switch is supported for memory and combo cards. This bit reflects the SDWP# pin."]
pub type WrprtswpinlvlR = crate::BitReader<Wrprtswpinlvl>;
impl WrprtswpinlvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrprtswpinlvl {
        match self.bits {
            false => Wrprtswpinlvl::B0,
            true => Wrprtswpinlvl::B1,
        }
    }
    #[doc = "Write enabled (SDWP# = 1)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Wrprtswpinlvl::B0
    }
    #[doc = "Write enabled (SDWP# = 1)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Wrprtswpinlvl::B1
    }
}
#[doc = "Field `DAT30LINESIGNALLEVEL` reader - DAT\\[3:0\\]
Line Signal Level This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[0\\]. \\[23\\]: DAT\\[3\\]
\\[22\\]: DAT\\[2\\]
\\[21\\]: DAT\\[1\\]
\\[20\\]: DAT\\[0\\]"]
pub type Dat30linesignallevelR = crate::FieldReader;
#[doc = "Field `CMDLINESIGNALLEVEL` reader - This status is used to check CMD line level to recover from errors, and for debugging."]
pub type CmdlinesignallevelR = crate::BitReader;
#[doc = "Field `DAT74LINESIGNALLEVEL` reader - This status is used to check DAT line level to recover from errors, and for debugging. \\[28\\]: DAT\\[7\\]
\\[27\\]: DAT\\[6\\]
\\[26\\]: DAT\\[5\\]
\\[25\\]: DAT\\[4\\]"]
pub type Dat74linesignallevelR = crate::FieldReader;
#[doc = "Field `DAT74LINESIGNALLEVEL` writer - This status is used to check DAT line level to recover from errors, and for debugging. \\[28\\]: DAT\\[7\\]
\\[27\\]: DAT\\[6\\]
\\[26\\]: DAT\\[5\\]
\\[25\\]: DAT\\[4\\]"]
pub type Dat74linesignallevelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - If this bit is 0, it indicates the CMD line is not in use and the HC can issue a SD command using the CMD line. This bit is set immediately after the Command register is written. This bit is cleared when the command response is received. Even if the Command Inhibit (DAT) is set to 1, Commands using only the CMD line can be issued if this bit is 0. Changing from 1 to 0 generates a Command complete interrupt in the Normal Interrupt Status register. If the HC cannot issue the command because of a command conflict error or because of Command Not Issued By Auto CMD12 Error, this bit shall remain 1 and the Command Complete is not set. Status issuing Auto CMD12 is not read from this bit. Auto CMD12 and Auto CMD23 consist of two responses. In this case, this bit is not cleared by the response of CMD12 or CMD23 but cleared by the response of a read/write command. Status issuing Auto CMD12 is not read from this bit. So if a command is issued during Auto CMD12 operation, Host Controller shall manage to issue two commands: CMD12 and a command set by Command register."]
    #[inline(always)]
    pub fn cmdinhibit(&self) -> CmdinhibitR {
        CmdinhibitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn datinhibit(&self) -> DatinhibitR {
        DatinhibitR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit indicates whether one of the DAT line on SD bus is in use."]
    #[inline(always)]
    pub fn datlineactive(&self) -> DatlineactiveR {
        DatlineactiveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Re-Tuning Request"]
    #[inline(always)]
    pub fn retuningreq(&self) -> RetuningreqR {
        RetuningreqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn writetransactive(&self) -> WritetransactiveR {
        WritetransactiveR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn readtransactive(&self) -> ReadtransactiveR {
        ReadtransactiveR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bufferwriteenable(&self) -> BufferwriteenableR {
        BufferwriteenableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bufferreadenable(&self) -> BufferreadenableR {
        BufferreadenableR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cardinserted(&self) -> CardinsertedR {
        CardinsertedR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cardstatestable(&self) -> CardstatestableR {
        CardstatestableR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This bit reflects the inverse value of the SDCD# pin."]
    #[inline(always)]
    pub fn carddetectpinlevel(&self) -> CarddetectpinlevelR {
        CarddetectpinlevelR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write Protect Switch Pin Level. The Write Protect Switch is supported for memory and combo cards. This bit reflects the SDWP# pin."]
    #[inline(always)]
    pub fn wrprtswpinlvl(&self) -> WrprtswpinlvlR {
        WrprtswpinlvlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - DAT\\[3:0\\]
Line Signal Level This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[0\\]. \\[23\\]: DAT\\[3\\]
\\[22\\]: DAT\\[2\\]
\\[21\\]: DAT\\[1\\]
\\[20\\]: DAT\\[0\\]"]
    #[inline(always)]
    pub fn dat30linesignallevel(&self) -> Dat30linesignallevelR {
        Dat30linesignallevelR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - This status is used to check CMD line level to recover from errors, and for debugging."]
    #[inline(always)]
    pub fn cmdlinesignallevel(&self) -> CmdlinesignallevelR {
        CmdlinesignallevelR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - This status is used to check DAT line level to recover from errors, and for debugging. \\[28\\]: DAT\\[7\\]
\\[27\\]: DAT\\[6\\]
\\[26\\]: DAT\\[5\\]
\\[25\\]: DAT\\[4\\]"]
    #[inline(always)]
    pub fn dat74linesignallevel(&self) -> Dat74linesignallevelR {
        Dat74linesignallevelR::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:28 - This status is used to check DAT line level to recover from errors, and for debugging. \\[28\\]: DAT\\[7\\]
\\[27\\]: DAT\\[6\\]
\\[26\\]: DAT\\[5\\]
\\[25\\]: DAT\\[4\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dat74linesignallevel(&mut self) -> Dat74linesignallevelW<EmmccorePrestsSpec> {
        Dat74linesignallevelW::new(self, 25)
    }
}
#[doc = "Present state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_prests::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_prests::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccorePrestsSpec;
impl crate::RegisterSpec for EmmccorePrestsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_prests::R`](R) reader structure"]
impl crate::Readable for EmmccorePrestsSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_prests::W`](W) writer structure"]
impl crate::Writable for EmmccorePrestsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_PRESTS to value 0x1fff_0000"]
impl crate::Resettable for EmmccorePrestsSpec {
    const RESET_VALUE: u32 = 0x1fff_0000;
}
