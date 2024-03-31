#[doc = "Register `NORINTSTS` reader"]
pub type R = crate::R<NorintstsSpec>;
#[doc = "Register `NORINTSTS` writer"]
pub type W = crate::W<NorintstsSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Commandcomplete {
    #[doc = "0: No Command Complete"]
    B0 = 0,
    #[doc = "1: Command Complete This bit is set when we get the end bit of the command response (Except Auto CMD12 and Auto CMD23) Note: Command Time-out Error has higher priority than Command Complete. If both are set to 1, it can be considered that the response was not received correctly."]
    B1 = 1,
}
impl From<Commandcomplete> for bool {
    #[inline(always)]
    fn from(variant: Commandcomplete) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMMANDCOMPLETE` reader - "]
pub type CommandcompleteR = crate::BitReader<Commandcomplete>;
impl CommandcompleteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Commandcomplete {
        match self.bits {
            false => Commandcomplete::B0,
            true => Commandcomplete::B1,
        }
    }
    #[doc = "No Command Complete"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Commandcomplete::B0
    }
    #[doc = "Command Complete This bit is set when we get the end bit of the command response (Except Auto CMD12 and Auto CMD23) Note: Command Time-out Error has higher priority than Command Complete. If both are set to 1, it can be considered that the response was not received correctly."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Commandcomplete::B1
    }
}
#[doc = "Field `COMMANDCOMPLETE` writer - "]
pub type CommandcompleteW<'a, REG> = crate::BitWriter1C<'a, REG, Commandcomplete>;
impl<'a, REG> CommandcompleteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Command Complete"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Commandcomplete::B0)
    }
    #[doc = "Command Complete This bit is set when we get the end bit of the command response (Except Auto CMD12 and Auto CMD23) Note: Command Time-out Error has higher priority than Command Complete. If both are set to 1, it can be considered that the response was not received correctly."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Commandcomplete::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transfercomplete {
    #[doc = "0: No Data Transfer Complete"]
    B0 = 0,
    #[doc = "1: Data Transfer Complete This bit is set when a read / write transaction is completed. a. Read Transaction: This bit is set at the falling edge of Read Transfer Active Status. There are two cases in which the Interrupt is generated. The first is when a data transfer is completed as specified by data length (After the last data has been read to the Host System). The second is when data has stopped at the block gap and completed the data transfer by setting the Stop At Block Gap Request in the Block Gap Control Register (After valid data has been read to the Host System). b. Write Transaction: This bit is set at the falling edge of the DAT Line Active Status. There are two cases in which the Interrupt is generated. The first is when the last data is written to the card as specified by data length and Busy signal is released. The second is when data transfers are stopped at the block gap by setting Stop At Block Gap Request in the Block Gap Control Register and data transfers completed. (Aftervalid data is written to the SD card and the busy signal is released). c. In case of command with busy This bit is set when busy is deasserted. Note: a. Transfer Complete has higher priority than Data Time-out Error. If both bits are set to 1, the data transfer can be considered complete b. While performing tuning procedure (Execute Tuning is set to 1), Transfer Complete is not set to 1"]
    B1 = 1,
}
impl From<Transfercomplete> for bool {
    #[inline(always)]
    fn from(variant: Transfercomplete) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRANSFERCOMPLETE` reader - "]
pub type TransfercompleteR = crate::BitReader<Transfercomplete>;
impl TransfercompleteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Transfercomplete {
        match self.bits {
            false => Transfercomplete::B0,
            true => Transfercomplete::B1,
        }
    }
    #[doc = "No Data Transfer Complete"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Transfercomplete::B0
    }
    #[doc = "Data Transfer Complete This bit is set when a read / write transaction is completed. a. Read Transaction: This bit is set at the falling edge of Read Transfer Active Status. There are two cases in which the Interrupt is generated. The first is when a data transfer is completed as specified by data length (After the last data has been read to the Host System). The second is when data has stopped at the block gap and completed the data transfer by setting the Stop At Block Gap Request in the Block Gap Control Register (After valid data has been read to the Host System). b. Write Transaction: This bit is set at the falling edge of the DAT Line Active Status. There are two cases in which the Interrupt is generated. The first is when the last data is written to the card as specified by data length and Busy signal is released. The second is when data transfers are stopped at the block gap by setting Stop At Block Gap Request in the Block Gap Control Register and data transfers completed. (Aftervalid data is written to the SD card and the busy signal is released). c. In case of command with busy This bit is set when busy is deasserted. Note: a. Transfer Complete has higher priority than Data Time-out Error. If both bits are set to 1, the data transfer can be considered complete b. While performing tuning procedure (Execute Tuning is set to 1), Transfer Complete is not set to 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Transfercomplete::B1
    }
}
#[doc = "Field `TRANSFERCOMPLETE` writer - "]
pub type TransfercompleteW<'a, REG> = crate::BitWriter1C<'a, REG, Transfercomplete>;
impl<'a, REG> TransfercompleteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Data Transfer Complete"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Transfercomplete::B0)
    }
    #[doc = "Data Transfer Complete This bit is set when a read / write transaction is completed. a. Read Transaction: This bit is set at the falling edge of Read Transfer Active Status. There are two cases in which the Interrupt is generated. The first is when a data transfer is completed as specified by data length (After the last data has been read to the Host System). The second is when data has stopped at the block gap and completed the data transfer by setting the Stop At Block Gap Request in the Block Gap Control Register (After valid data has been read to the Host System). b. Write Transaction: This bit is set at the falling edge of the DAT Line Active Status. There are two cases in which the Interrupt is generated. The first is when the last data is written to the card as specified by data length and Busy signal is released. The second is when data transfers are stopped at the block gap by setting Stop At Block Gap Request in the Block Gap Control Register and data transfers completed. (Aftervalid data is written to the SD card and the busy signal is released). c. In case of command with busy This bit is set when busy is deasserted. Note: a. Transfer Complete has higher priority than Data Time-out Error. If both bits are set to 1, the data transfer can be considered complete b. While performing tuning procedure (Execute Tuning is set to 1), Transfer Complete is not set to 1"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Transfercomplete::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blockgapevent {
    #[doc = "0: No Block Gap Event"]
    B0 = 0,
    #[doc = "1: Transaction stopped at Block Gap If the Stop At Block Gap Request in the Block Gap Control Register is set, this bit is set. a. Read Transaction: This bit is set at the falling edge of the DAT Line Active Status (When the transaction is stopped at SD Bus timing. The Read Wait must be supported inorder to use this function). b. Write Transaction: This bit is set at the falling edge of Write Transfer Active Status (After getting CRC status at SD Bus timing)."]
    B1 = 1,
}
impl From<Blockgapevent> for bool {
    #[inline(always)]
    fn from(variant: Blockgapevent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCKGAPEVENT` reader - "]
pub type BlockgapeventR = crate::BitReader<Blockgapevent>;
impl BlockgapeventR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blockgapevent {
        match self.bits {
            false => Blockgapevent::B0,
            true => Blockgapevent::B1,
        }
    }
    #[doc = "No Block Gap Event"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Blockgapevent::B0
    }
    #[doc = "Transaction stopped at Block Gap If the Stop At Block Gap Request in the Block Gap Control Register is set, this bit is set. a. Read Transaction: This bit is set at the falling edge of the DAT Line Active Status (When the transaction is stopped at SD Bus timing. The Read Wait must be supported inorder to use this function). b. Write Transaction: This bit is set at the falling edge of Write Transfer Active Status (After getting CRC status at SD Bus timing)."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Blockgapevent::B1
    }
}
#[doc = "Field `BLOCKGAPEVENT` writer - "]
pub type BlockgapeventW<'a, REG> = crate::BitWriter1C<'a, REG, Blockgapevent>;
impl<'a, REG> BlockgapeventW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Block Gap Event"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Blockgapevent::B0)
    }
    #[doc = "Transaction stopped at Block Gap If the Stop At Block Gap Request in the Block Gap Control Register is set, this bit is set. a. Read Transaction: This bit is set at the falling edge of the DAT Line Active Status (When the transaction is stopped at SD Bus timing. The Read Wait must be supported inorder to use this function). b. Write Transaction: This bit is set at the falling edge of Write Transfer Active Status (After getting CRC status at SD Bus timing)."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Blockgapevent::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmainterrupt {
    #[doc = "0: No DMA Interrupt"]
    B0 = 0,
    #[doc = "1: DMA Interrupt is Generated This status is set if the HC detects the Host DMA Buffer Boundary in the Block Size regiser."]
    B1 = 1,
}
impl From<Dmainterrupt> for bool {
    #[inline(always)]
    fn from(variant: Dmainterrupt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAINTERRUPT` reader - "]
pub type DmainterruptR = crate::BitReader<Dmainterrupt>;
impl DmainterruptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmainterrupt {
        match self.bits {
            false => Dmainterrupt::B0,
            true => Dmainterrupt::B1,
        }
    }
    #[doc = "No DMA Interrupt"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dmainterrupt::B0
    }
    #[doc = "DMA Interrupt is Generated This status is set if the HC detects the Host DMA Buffer Boundary in the Block Size regiser."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dmainterrupt::B1
    }
}
#[doc = "Field `DMAINTERRUPT` writer - "]
pub type DmainterruptW<'a, REG> = crate::BitWriter1C<'a, REG, Dmainterrupt>;
impl<'a, REG> DmainterruptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DMA Interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmainterrupt::B0)
    }
    #[doc = "DMA Interrupt is Generated This status is set if the HC detects the Host DMA Buffer Boundary in the Block Size regiser."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmainterrupt::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufferwriteready {
    #[doc = "0: Not Ready to Write Buffer"]
    B0 = 0,
    #[doc = "1: Ready to Write Buffer This status is set if the Buffer Write Enable changes from 0 to 1."]
    B1 = 1,
}
impl From<Bufferwriteready> for bool {
    #[inline(always)]
    fn from(variant: Bufferwriteready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFFERWRITEREADY` reader - "]
pub type BufferwritereadyR = crate::BitReader<Bufferwriteready>;
impl BufferwritereadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufferwriteready {
        match self.bits {
            false => Bufferwriteready::B0,
            true => Bufferwriteready::B1,
        }
    }
    #[doc = "Not Ready to Write Buffer"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bufferwriteready::B0
    }
    #[doc = "Ready to Write Buffer This status is set if the Buffer Write Enable changes from 0 to 1."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bufferwriteready::B1
    }
}
#[doc = "Field `BUFFERWRITEREADY` writer - "]
pub type BufferwritereadyW<'a, REG> = crate::BitWriter1C<'a, REG, Bufferwriteready>;
impl<'a, REG> BufferwritereadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Ready to Write Buffer"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferwriteready::B0)
    }
    #[doc = "Ready to Write Buffer This status is set if the Buffer Write Enable changes from 0 to 1."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferwriteready::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufferreadready {
    #[doc = "0: Not Ready to read Buffer"]
    B0 = 0,
    #[doc = "1: Ready to read Buffer This status is set if the Buffer Read Enable changes from 0 to 1. Buffer Read Ready is set to 1 for every CMD19 execution in tuning procedure."]
    B1 = 1,
}
impl From<Bufferreadready> for bool {
    #[inline(always)]
    fn from(variant: Bufferreadready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFFERREADREADY` reader - "]
pub type BufferreadreadyR = crate::BitReader<Bufferreadready>;
impl BufferreadreadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufferreadready {
        match self.bits {
            false => Bufferreadready::B0,
            true => Bufferreadready::B1,
        }
    }
    #[doc = "Not Ready to read Buffer"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bufferreadready::B0
    }
    #[doc = "Ready to read Buffer This status is set if the Buffer Read Enable changes from 0 to 1. Buffer Read Ready is set to 1 for every CMD19 execution in tuning procedure."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bufferreadready::B1
    }
}
#[doc = "Field `BUFFERREADREADY` writer - "]
pub type BufferreadreadyW<'a, REG> = crate::BitWriter1C<'a, REG, Bufferreadready>;
impl<'a, REG> BufferreadreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Ready to read Buffer"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferreadready::B0)
    }
    #[doc = "Ready to read Buffer This status is set if the Buffer Read Enable changes from 0 to 1. Buffer Read Ready is set to 1 for every CMD19 execution in tuning procedure."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferreadready::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardinsertion {
    #[doc = "0: Card State Stable or Debouncing"]
    B0 = 0,
    #[doc = "1: Card Inserted This status is set if the Card Inserted in the Present State register changes from 0 to 1. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State registershould be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated."]
    B1 = 1,
}
impl From<Cardinsertion> for bool {
    #[inline(always)]
    fn from(variant: Cardinsertion) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDINSERTION` reader - "]
pub type CardinsertionR = crate::BitReader<Cardinsertion>;
impl CardinsertionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardinsertion {
        match self.bits {
            false => Cardinsertion::B0,
            true => Cardinsertion::B1,
        }
    }
    #[doc = "Card State Stable or Debouncing"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cardinsertion::B0
    }
    #[doc = "Card Inserted This status is set if the Card Inserted in the Present State register changes from 0 to 1. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State registershould be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cardinsertion::B1
    }
}
#[doc = "Field `CARDINSERTION` writer - "]
pub type CardinsertionW<'a, REG> = crate::BitWriter1C<'a, REG, Cardinsertion>;
impl<'a, REG> CardinsertionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card State Stable or Debouncing"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cardinsertion::B0)
    }
    #[doc = "Card Inserted This status is set if the Card Inserted in the Present State register changes from 0 to 1. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State registershould be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cardinsertion::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardremoval {
    #[doc = "0: Card State Stable or Debouncing"]
    B0 = 0,
    #[doc = "1: Card Removed This status is set if the Card Inserted in the Present State register changes from 1 to 0. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State registershould be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated."]
    B1 = 1,
}
impl From<Cardremoval> for bool {
    #[inline(always)]
    fn from(variant: Cardremoval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDREMOVAL` reader - "]
pub type CardremovalR = crate::BitReader<Cardremoval>;
impl CardremovalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardremoval {
        match self.bits {
            false => Cardremoval::B0,
            true => Cardremoval::B1,
        }
    }
    #[doc = "Card State Stable or Debouncing"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cardremoval::B0
    }
    #[doc = "Card Removed This status is set if the Card Inserted in the Present State register changes from 1 to 0. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State registershould be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cardremoval::B1
    }
}
#[doc = "Field `CARDREMOVAL` writer - "]
pub type CardremovalW<'a, REG> = crate::BitWriter1C<'a, REG, Cardremoval>;
impl<'a, REG> CardremovalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card State Stable or Debouncing"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cardremoval::B0)
    }
    #[doc = "Card Removed This status is set if the Card Inserted in the Present State register changes from 1 to 0. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State registershould be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cardremoval::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cardinterrupt {
    #[doc = "0: No Card Interrupt"]
    B0 = 0,
    #[doc = "1: Generate Card Interrupt Writing this bit to 1 does not clear this bit. It is cleared by resetting the SD card interrupt factor. In 1-bit mode, the HC shall detect the Card Interrupt without SD Clock to support wakeup. In 4-bit mode, the card interrupt signal is sampled during the interrupt cycle, so there are some sample delays between the interrupt signal from the card and the interrupt to the Host system. when this status has been set and the HD needs to start this interrupt service, Card Interrupt Status Enable in the Normal Interrupt Status register shall be set to 0 in order to clear the card interrupt statuses latched in the HC and stop driving the Host System. After completion of the card interrupt service (the reset factor in the SD card and the interrupt signal may not be asserted), set Card Interrupt Status Enable to 1 and start sampling the interrupt signal again. Interrupt detected by DAT\\[1\\]
is supported when there is a card per slot."]
    B1 = 1,
}
impl From<Cardinterrupt> for bool {
    #[inline(always)]
    fn from(variant: Cardinterrupt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDINTERRUPT` reader - "]
pub type CardinterruptR = crate::BitReader<Cardinterrupt>;
impl CardinterruptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cardinterrupt {
        match self.bits {
            false => Cardinterrupt::B0,
            true => Cardinterrupt::B1,
        }
    }
    #[doc = "No Card Interrupt"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cardinterrupt::B0
    }
    #[doc = "Generate Card Interrupt Writing this bit to 1 does not clear this bit. It is cleared by resetting the SD card interrupt factor. In 1-bit mode, the HC shall detect the Card Interrupt without SD Clock to support wakeup. In 4-bit mode, the card interrupt signal is sampled during the interrupt cycle, so there are some sample delays between the interrupt signal from the card and the interrupt to the Host system. when this status has been set and the HD needs to start this interrupt service, Card Interrupt Status Enable in the Normal Interrupt Status register shall be set to 0 in order to clear the card interrupt statuses latched in the HC and stop driving the Host System. After completion of the card interrupt service (the reset factor in the SD card and the interrupt signal may not be asserted), set Card Interrupt Status Enable to 1 and start sampling the interrupt signal again. Interrupt detected by DAT\\[1\\]
is supported when there is a card per slot."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cardinterrupt::B1
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Retuningevent {
    #[doc = "1: Re-Tuning should be performed"]
    B1 = 1,
    #[doc = "0: Re-Tuning is not required This status is set if Re-Tuning Request in the Present State register changes from 0 to 1. Host Controller requestsHost Driver to perform re-tuning for next data transfer. Current data transfer (not large block count) can be completed without re-tuning."]
    B0 = 0,
}
impl From<Retuningevent> for bool {
    #[inline(always)]
    fn from(variant: Retuningevent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RETUNINGEVENT` reader - "]
pub type RetuningeventR = crate::BitReader<Retuningevent>;
impl RetuningeventR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Retuningevent {
        match self.bits {
            true => Retuningevent::B1,
            false => Retuningevent::B0,
        }
    }
    #[doc = "Re-Tuning should be performed"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Retuningevent::B1
    }
    #[doc = "Re-Tuning is not required This status is set if Re-Tuning Request in the Present State register changes from 0 to 1. Host Controller requestsHost Driver to perform re-tuning for next data transfer. Current data transfer (not large block count) can be completed without re-tuning."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Retuningevent::B0
    }
}
#[doc = "This status is set if the boot acknowledge is received from device.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bootackrcv {
    #[doc = "0: Boot ack is not received"]
    B0 = 0,
    #[doc = "1: Boot ack is received"]
    B1 = 1,
}
impl From<Bootackrcv> for bool {
    #[inline(always)]
    fn from(variant: Bootackrcv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOTACKRCV` reader - This status is set if the boot acknowledge is received from device."]
pub type BootackrcvR = crate::BitReader<Bootackrcv>;
impl BootackrcvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bootackrcv {
        match self.bits {
            false => Bootackrcv::B0,
            true => Bootackrcv::B1,
        }
    }
    #[doc = "Boot ack is not received"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bootackrcv::B0
    }
    #[doc = "Boot ack is received"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bootackrcv::B1
    }
}
#[doc = "Field `BOOTACKRCV` writer - This status is set if the boot acknowledge is received from device."]
pub type BootackrcvW<'a, REG> = crate::BitWriter1C<'a, REG, Bootackrcv>;
impl<'a, REG> BootackrcvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Boot ack is not received"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bootackrcv::B0)
    }
    #[doc = "Boot ack is received"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bootackrcv::B1)
    }
}
#[doc = "This status is set if the boot operation get terminated\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bootterminateinterrupt {
    #[doc = "0: Boot operation is not terminated"]
    B0 = 0,
    #[doc = "1: Boot operation is terminated"]
    B1 = 1,
}
impl From<Bootterminateinterrupt> for bool {
    #[inline(always)]
    fn from(variant: Bootterminateinterrupt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOTTERMINATEINTERRUPT` reader - This status is set if the boot operation get terminated"]
pub type BootterminateinterruptR = crate::BitReader<Bootterminateinterrupt>;
impl BootterminateinterruptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bootterminateinterrupt {
        match self.bits {
            false => Bootterminateinterrupt::B0,
            true => Bootterminateinterrupt::B1,
        }
    }
    #[doc = "Boot operation is not terminated"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bootterminateinterrupt::B0
    }
    #[doc = "Boot operation is terminated"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bootterminateinterrupt::B1
    }
}
#[doc = "Field `BOOTTERMINATEINTERRUPT` writer - This status is set if the boot operation get terminated"]
pub type BootterminateinterruptW<'a, REG> = crate::BitWriter1C<'a, REG, Bootterminateinterrupt>;
impl<'a, REG> BootterminateinterruptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Boot operation is not terminated"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bootterminateinterrupt::B0)
    }
    #[doc = "Boot operation is terminated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bootterminateinterrupt::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errorinterrupt {
    #[doc = "0: No Error"]
    B0 = 0,
    #[doc = "1: Error If any of the bits inthe Error Interrupt Status Register are set, then this bit is set. Therefore the HD can test for an error by checking this bit first."]
    B1 = 1,
}
impl From<Errorinterrupt> for bool {
    #[inline(always)]
    fn from(variant: Errorinterrupt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRORINTERRUPT` reader - "]
pub type ErrorinterruptR = crate::BitReader<Errorinterrupt>;
impl ErrorinterruptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errorinterrupt {
        match self.bits {
            false => Errorinterrupt::B0,
            true => Errorinterrupt::B1,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Errorinterrupt::B0
    }
    #[doc = "Error If any of the bits inthe Error Interrupt Status Register are set, then this bit is set. Therefore the HD can test for an error by checking this bit first."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Errorinterrupt::B1
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn commandcomplete(&self) -> CommandcompleteR {
        CommandcompleteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn transfercomplete(&self) -> TransfercompleteR {
        TransfercompleteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn blockgapevent(&self) -> BlockgapeventR {
        BlockgapeventR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dmainterrupt(&self) -> DmainterruptR {
        DmainterruptR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bufferwriteready(&self) -> BufferwritereadyR {
        BufferwritereadyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bufferreadready(&self) -> BufferreadreadyR {
        BufferreadreadyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cardinsertion(&self) -> CardinsertionR {
        CardinsertionR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cardremoval(&self) -> CardremovalR {
        CardremovalR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cardinterrupt(&self) -> CardinterruptR {
        CardinterruptR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn retuningevent(&self) -> RetuningeventR {
        RetuningeventR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This status is set if the boot acknowledge is received from device."]
    #[inline(always)]
    pub fn bootackrcv(&self) -> BootackrcvR {
        BootackrcvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This status is set if the boot operation get terminated"]
    #[inline(always)]
    pub fn bootterminateinterrupt(&self) -> BootterminateinterruptR {
        BootterminateinterruptR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn errorinterrupt(&self) -> ErrorinterruptR {
        ErrorinterruptR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn commandcomplete(&mut self) -> CommandcompleteW<NorintstsSpec> {
        CommandcompleteW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn transfercomplete(&mut self) -> TransfercompleteW<NorintstsSpec> {
        TransfercompleteW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn blockgapevent(&mut self) -> BlockgapeventW<NorintstsSpec> {
        BlockgapeventW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dmainterrupt(&mut self) -> DmainterruptW<NorintstsSpec> {
        DmainterruptW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn bufferwriteready(&mut self) -> BufferwritereadyW<NorintstsSpec> {
        BufferwritereadyW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn bufferreadready(&mut self) -> BufferreadreadyW<NorintstsSpec> {
        BufferreadreadyW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cardinsertion(&mut self) -> CardinsertionW<NorintstsSpec> {
        CardinsertionW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cardremoval(&mut self) -> CardremovalW<NorintstsSpec> {
        CardremovalW::new(self, 7)
    }
    #[doc = "Bit 13 - This status is set if the boot acknowledge is received from device."]
    #[inline(always)]
    #[must_use]
    pub fn bootackrcv(&mut self) -> BootackrcvW<NorintstsSpec> {
        BootackrcvW::new(self, 13)
    }
    #[doc = "Bit 14 - This status is set if the boot operation get terminated"]
    #[inline(always)]
    #[must_use]
    pub fn bootterminateinterrupt(&mut self) -> BootterminateinterruptW<NorintstsSpec> {
        BootterminateinterruptW::new(self, 14)
    }
}
#[doc = "Normal interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`norintsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`norintsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NorintstsSpec;
impl crate::RegisterSpec for NorintstsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`norintsts::R`](R) reader structure"]
impl crate::Readable for NorintstsSpec {}
#[doc = "`write(|w| ..)` method takes [`norintsts::W`](W) writer structure"]
impl crate::Writable for NorintstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0x60ff;
}
#[doc = "`reset()` method sets NORINTSTS to value 0"]
impl crate::Resettable for NorintstsSpec {
    const RESET_VALUE: u16 = 0;
}
