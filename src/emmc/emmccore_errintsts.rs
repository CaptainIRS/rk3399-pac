#[doc = "Register `EMMCCORE_ERRINTSTS` reader"]
pub type R = crate::R<EmmccoreErrintstsSpec>;
#[doc = "Register `EMMCCORE_ERRINTSTS` writer"]
pub type W = crate::W<EmmccoreErrintstsSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdtimeouterr {
    #[doc = "0: Timeout Occurs only if the no response is returned within 64 SDCLK cycles from the end bit of the command. If the HC detects a CMD line conflict, in which case Command CRC Error shall also be set. This bit shall be set without waiting for 64 SDCLK cycles because the command will be aborted by the HC."]
    B0 = 0,
    #[doc = "1: Timeout Occurs only if the no response is returned within 64 SDCLK cycles from the end bit of the command. If the HC detects a CMD line conflict, in which case Command CRC Error shall also be set. This bit shall be set without waiting for 64 SDCLK cycles because the command will be aborted by the HC."]
    B1 = 1,
}
impl From<Cmdtimeouterr> for bool {
    #[inline(always)]
    fn from(variant: Cmdtimeouterr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDTIMEOUTERR` reader - "]
pub type CmdtimeouterrR = crate::BitReader<Cmdtimeouterr>;
impl CmdtimeouterrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdtimeouterr {
        match self.bits {
            false => Cmdtimeouterr::B0,
            true => Cmdtimeouterr::B1,
        }
    }
    #[doc = "Timeout Occurs only if the no response is returned within 64 SDCLK cycles from the end bit of the command. If the HC detects a CMD line conflict, in which case Command CRC Error shall also be set. This bit shall be set without waiting for 64 SDCLK cycles because the command will be aborted by the HC."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cmdtimeouterr::B0
    }
    #[doc = "Timeout Occurs only if the no response is returned within 64 SDCLK cycles from the end bit of the command. If the HC detects a CMD line conflict, in which case Command CRC Error shall also be set. This bit shall be set without waiting for 64 SDCLK cycles because the command will be aborted by the HC."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cmdtimeouterr::B1
    }
}
#[doc = "Field `CMDTIMEOUTERR` writer - "]
pub type CmdtimeouterrW<'a, REG> = crate::BitWriter1C<'a, REG, Cmdtimeouterr>;
impl<'a, REG> CmdtimeouterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timeout Occurs only if the no response is returned within 64 SDCLK cycles from the end bit of the command. If the HC detects a CMD line conflict, in which case Command CRC Error shall also be set. This bit shall be set without waiting for 64 SDCLK cycles because the command will be aborted by the HC."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtimeouterr::B0)
    }
    #[doc = "Timeout Occurs only if the no response is returned within 64 SDCLK cycles from the end bit of the command. If the HC detects a CMD line conflict, in which case Command CRC Error shall also be set. This bit shall be set without waiting for 64 SDCLK cycles because the command will be aborted by the HC."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtimeouterr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdcrcerr {
    #[doc = "0: CRC Error Generated Command CRC Error is generated in two cases. a. If a response is returned and the Command Time-out Error is set to 0, this bit is set to 1 when detecting a CRT error in the command response b. The HC detects a CMD line conflict by monitoring the CMD line when a command is issued. If the HC drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SDCLK edge, then the HC shall abort the command (Stop driving CMD line) and set this bit to 1. The Command Timeout Error shall also be set to 1 to distinguish CMD line conflict."]
    B0 = 0,
    #[doc = "1: CRC Error Generated Command CRC Error is generated in two cases. a. If a response is returned and the Command Time-out Error is set to 0, this bit is set to 1 when detecting a CRT error in the command response b. The HC detects a CMD line conflict by monitoring the CMD line when a command is issued. If the HC drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SDCLK edge, then the HC shall abort the command (Stop driving CMD line) and set this bit to 1. The Command Timeout Error shall also be set to 1 to distinguish CMD line conflict."]
    B1 = 1,
}
impl From<Cmdcrcerr> for bool {
    #[inline(always)]
    fn from(variant: Cmdcrcerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDCRCERR` reader - "]
pub type CmdcrcerrR = crate::BitReader<Cmdcrcerr>;
impl CmdcrcerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdcrcerr {
        match self.bits {
            false => Cmdcrcerr::B0,
            true => Cmdcrcerr::B1,
        }
    }
    #[doc = "CRC Error Generated Command CRC Error is generated in two cases. a. If a response is returned and the Command Time-out Error is set to 0, this bit is set to 1 when detecting a CRT error in the command response b. The HC detects a CMD line conflict by monitoring the CMD line when a command is issued. If the HC drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SDCLK edge, then the HC shall abort the command (Stop driving CMD line) and set this bit to 1. The Command Timeout Error shall also be set to 1 to distinguish CMD line conflict."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cmdcrcerr::B0
    }
    #[doc = "CRC Error Generated Command CRC Error is generated in two cases. a. If a response is returned and the Command Time-out Error is set to 0, this bit is set to 1 when detecting a CRT error in the command response b. The HC detects a CMD line conflict by monitoring the CMD line when a command is issued. If the HC drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SDCLK edge, then the HC shall abort the command (Stop driving CMD line) and set this bit to 1. The Command Timeout Error shall also be set to 1 to distinguish CMD line conflict."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cmdcrcerr::B1
    }
}
#[doc = "Field `CMDCRCERR` writer - "]
pub type CmdcrcerrW<'a, REG> = crate::BitWriter1C<'a, REG, Cmdcrcerr>;
impl<'a, REG> CmdcrcerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC Error Generated Command CRC Error is generated in two cases. a. If a response is returned and the Command Time-out Error is set to 0, this bit is set to 1 when detecting a CRT error in the command response b. The HC detects a CMD line conflict by monitoring the CMD line when a command is issued. If the HC drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SDCLK edge, then the HC shall abort the command (Stop driving CMD line) and set this bit to 1. The Command Timeout Error shall also be set to 1 to distinguish CMD line conflict."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcrcerr::B0)
    }
    #[doc = "CRC Error Generated Command CRC Error is generated in two cases. a. If a response is returned and the Command Time-out Error is set to 0, this bit is set to 1 when detecting a CRT error in the command response b. The HC detects a CMD line conflict by monitoring the CMD line when a command is issued. If the HC drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SDCLK edge, then the HC shall abort the command (Stop driving CMD line) and set this bit to 1. The Command Timeout Error shall also be set to 1 to distinguish CMD line conflict."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcrcerr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdendbiterr {
    #[doc = "0: End Bit Error Generated Occurs when detecting that the end bit of a command response is 0."]
    B0 = 0,
    #[doc = "1: End Bit Error Generated Occurs when detecting that the end bit of a command response is 0."]
    B1 = 1,
}
impl From<Cmdendbiterr> for bool {
    #[inline(always)]
    fn from(variant: Cmdendbiterr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDENDBITERR` reader - "]
pub type CmdendbiterrR = crate::BitReader<Cmdendbiterr>;
impl CmdendbiterrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdendbiterr {
        match self.bits {
            false => Cmdendbiterr::B0,
            true => Cmdendbiterr::B1,
        }
    }
    #[doc = "End Bit Error Generated Occurs when detecting that the end bit of a command response is 0."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cmdendbiterr::B0
    }
    #[doc = "End Bit Error Generated Occurs when detecting that the end bit of a command response is 0."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cmdendbiterr::B1
    }
}
#[doc = "Field `CMDENDBITERR` writer - "]
pub type CmdendbiterrW<'a, REG> = crate::BitWriter<'a, REG, Cmdendbiterr>;
impl<'a, REG> CmdendbiterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End Bit Error Generated Occurs when detecting that the end bit of a command response is 0."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdendbiterr::B0)
    }
    #[doc = "End Bit Error Generated Occurs when detecting that the end bit of a command response is 0."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdendbiterr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdindexerr {
    #[doc = "0: Error Occurs if a Command Index error occurs in the Command Response."]
    B0 = 0,
    #[doc = "1: Error Occurs if a Command Index error occurs in the Command Response."]
    B1 = 1,
}
impl From<Cmdindexerr> for bool {
    #[inline(always)]
    fn from(variant: Cmdindexerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDINDEXERR` reader - "]
pub type CmdindexerrR = crate::BitReader<Cmdindexerr>;
impl CmdindexerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdindexerr {
        match self.bits {
            false => Cmdindexerr::B0,
            true => Cmdindexerr::B1,
        }
    }
    #[doc = "Error Occurs if a Command Index error occurs in the Command Response."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cmdindexerr::B0
    }
    #[doc = "Error Occurs if a Command Index error occurs in the Command Response."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cmdindexerr::B1
    }
}
#[doc = "Field `CMDINDEXERR` writer - "]
pub type CmdindexerrW<'a, REG> = crate::BitWriter1C<'a, REG, Cmdindexerr>;
impl<'a, REG> CmdindexerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error Occurs if a Command Index error occurs in the Command Response."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdindexerr::B0)
    }
    #[doc = "Error Occurs if a Command Index error occurs in the Command Response."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdindexerr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datatimeouterr {
    #[doc = "0: Timeout Occurs when detecting one of following timeout conditions. a. Busy Timeout for R1b, R5b type. b. Busy Timeout after Write CRC status c. Write CRC status Timeout d. Read Data Timeout"]
    B0 = 0,
    #[doc = "1: Timeout Occurs when detecting one of following timeout conditions. a. Busy Timeout for R1b, R5b type. b. Busy Timeout after Write CRC status c. Write CRC status Timeout d. Read Data Timeout"]
    B1 = 1,
}
impl From<Datatimeouterr> for bool {
    #[inline(always)]
    fn from(variant: Datatimeouterr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATATIMEOUTERR` reader - "]
pub type DatatimeouterrR = crate::BitReader<Datatimeouterr>;
impl DatatimeouterrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datatimeouterr {
        match self.bits {
            false => Datatimeouterr::B0,
            true => Datatimeouterr::B1,
        }
    }
    #[doc = "Timeout Occurs when detecting one of following timeout conditions. a. Busy Timeout for R1b, R5b type. b. Busy Timeout after Write CRC status c. Write CRC status Timeout d. Read Data Timeout"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Datatimeouterr::B0
    }
    #[doc = "Timeout Occurs when detecting one of following timeout conditions. a. Busy Timeout for R1b, R5b type. b. Busy Timeout after Write CRC status c. Write CRC status Timeout d. Read Data Timeout"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Datatimeouterr::B1
    }
}
#[doc = "Field `DATATIMEOUTERR` writer - "]
pub type DatatimeouterrW<'a, REG> = crate::BitWriter1C<'a, REG, Datatimeouterr>;
impl<'a, REG> DatatimeouterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timeout Occurs when detecting one of following timeout conditions. a. Busy Timeout for R1b, R5b type. b. Busy Timeout after Write CRC status c. Write CRC status Timeout d. Read Data Timeout"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Datatimeouterr::B0)
    }
    #[doc = "Timeout Occurs when detecting one of following timeout conditions. a. Busy Timeout for R1b, R5b type. b. Busy Timeout after Write CRC status c. Write CRC status Timeout d. Read Data Timeout"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Datatimeouterr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datacrcerr {
    #[doc = "0: Error Occurs when detecting CRC error when transferring read data which uses the DAT line or when detecting the Write CRC Status having a value of other than \"010\"."]
    B0 = 0,
    #[doc = "1: Error Occurs when detecting CRC error when transferring read data which uses the DAT line or when detecting the Write CRC Status having a value of other than \"010\"."]
    B1 = 1,
}
impl From<Datacrcerr> for bool {
    #[inline(always)]
    fn from(variant: Datacrcerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATACRCERR` reader - "]
pub type DatacrcerrR = crate::BitReader<Datacrcerr>;
impl DatacrcerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datacrcerr {
        match self.bits {
            false => Datacrcerr::B0,
            true => Datacrcerr::B1,
        }
    }
    #[doc = "Error Occurs when detecting CRC error when transferring read data which uses the DAT line or when detecting the Write CRC Status having a value of other than \"010\"."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Datacrcerr::B0
    }
    #[doc = "Error Occurs when detecting CRC error when transferring read data which uses the DAT line or when detecting the Write CRC Status having a value of other than \"010\"."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Datacrcerr::B1
    }
}
#[doc = "Field `DATACRCERR` writer - "]
pub type DatacrcerrW<'a, REG> = crate::BitWriter1C<'a, REG, Datacrcerr>;
impl<'a, REG> DatacrcerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error Occurs when detecting CRC error when transferring read data which uses the DAT line or when detecting the Write CRC Status having a value of other than \"010\"."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Datacrcerr::B0)
    }
    #[doc = "Error Occurs when detecting CRC error when transferring read data which uses the DAT line or when detecting the Write CRC Status having a value of other than \"010\"."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Datacrcerr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dataendbiterr {
    #[doc = "0: Error Occurs when detecting 0 at the end bit position of read data which uses the DAT line or the end bit position of the CRC status."]
    B0 = 0,
    #[doc = "1: Error Occurs when detecting 0 at the end bit position of read data which uses the DAT line or the end bit position of the CRC status."]
    B1 = 1,
}
impl From<Dataendbiterr> for bool {
    #[inline(always)]
    fn from(variant: Dataendbiterr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAENDBITERR` reader - "]
pub type DataendbiterrR = crate::BitReader<Dataendbiterr>;
impl DataendbiterrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dataendbiterr {
        match self.bits {
            false => Dataendbiterr::B0,
            true => Dataendbiterr::B1,
        }
    }
    #[doc = "Error Occurs when detecting 0 at the end bit position of read data which uses the DAT line or the end bit position of the CRC status."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dataendbiterr::B0
    }
    #[doc = "Error Occurs when detecting 0 at the end bit position of read data which uses the DAT line or the end bit position of the CRC status."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dataendbiterr::B1
    }
}
#[doc = "Field `DATAENDBITERR` writer - "]
pub type DataendbiterrW<'a, REG> = crate::BitWriter1C<'a, REG, Dataendbiterr>;
impl<'a, REG> DataendbiterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error Occurs when detecting 0 at the end bit position of read data which uses the DAT line or the end bit position of the CRC status."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dataendbiterr::B0)
    }
    #[doc = "Error Occurs when detecting 0 at the end bit position of read data which uses the DAT line or the end bit position of the CRC status."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dataendbiterr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Currentlimiterr {
    #[doc = "0: Power Fail By setting the SD Bus Power bit in the Power Control Register, the HC is requested to supply power for the SD Bus. If the HC supports the Current Limit Function, it can be protected from an Illegal card by stopping power supply to the card in which case this bit indicates a failure status. Reading 1 means the HC is not supplying power to SD card due to some failure. Reading 0 means that the HC is supplying power and no error has occurred. This bit shall always set to be 0, if the HC does not support this function. Note: The current_Limit_Error is to be implemented if customer application requires it.. By default it is not implementedas there is no specific requirement from Customers."]
    B0 = 0,
    #[doc = "1: Power Fail By setting the SD Bus Power bit in the Power Control Register, the HC is requested to supply power for the SD Bus. If the HC supports the Current Limit Function, it can be protected from an Illegal card by stopping power supply to the card in which case this bit indicates a failure status. Reading 1 means the HC is not supplying power to SD card due to some failure. Reading 0 means that the HC is supplying power and no error has occurred. This bit shall always set to be 0, if the HC does not support this function. Note: The current_Limit_Error is to be implemented if customer application requires it.. By default it is not implementedas there is no specific requirement from Customers."]
    B1 = 1,
}
impl From<Currentlimiterr> for bool {
    #[inline(always)]
    fn from(variant: Currentlimiterr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CURRENTLIMITERR` reader - "]
pub type CurrentlimiterrR = crate::BitReader<Currentlimiterr>;
impl CurrentlimiterrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Currentlimiterr {
        match self.bits {
            false => Currentlimiterr::B0,
            true => Currentlimiterr::B1,
        }
    }
    #[doc = "Power Fail By setting the SD Bus Power bit in the Power Control Register, the HC is requested to supply power for the SD Bus. If the HC supports the Current Limit Function, it can be protected from an Illegal card by stopping power supply to the card in which case this bit indicates a failure status. Reading 1 means the HC is not supplying power to SD card due to some failure. Reading 0 means that the HC is supplying power and no error has occurred. This bit shall always set to be 0, if the HC does not support this function. Note: The current_Limit_Error is to be implemented if customer application requires it.. By default it is not implementedas there is no specific requirement from Customers."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Currentlimiterr::B0
    }
    #[doc = "Power Fail By setting the SD Bus Power bit in the Power Control Register, the HC is requested to supply power for the SD Bus. If the HC supports the Current Limit Function, it can be protected from an Illegal card by stopping power supply to the card in which case this bit indicates a failure status. Reading 1 means the HC is not supplying power to SD card due to some failure. Reading 0 means that the HC is supplying power and no error has occurred. This bit shall always set to be 0, if the HC does not support this function. Note: The current_Limit_Error is to be implemented if customer application requires it.. By default it is not implementedas there is no specific requirement from Customers."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Currentlimiterr::B1
    }
}
#[doc = "Field `CURRENTLIMITERR` writer - "]
pub type CurrentlimiterrW<'a, REG> = crate::BitWriter1C<'a, REG, Currentlimiterr>;
impl<'a, REG> CurrentlimiterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power Fail By setting the SD Bus Power bit in the Power Control Register, the HC is requested to supply power for the SD Bus. If the HC supports the Current Limit Function, it can be protected from an Illegal card by stopping power supply to the card in which case this bit indicates a failure status. Reading 1 means the HC is not supplying power to SD card due to some failure. Reading 0 means that the HC is supplying power and no error has occurred. This bit shall always set to be 0, if the HC does not support this function. Note: The current_Limit_Error is to be implemented if customer application requires it.. By default it is not implementedas there is no specific requirement from Customers."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Currentlimiterr::B0)
    }
    #[doc = "Power Fail By setting the SD Bus Power bit in the Power Control Register, the HC is requested to supply power for the SD Bus. If the HC supports the Current Limit Function, it can be protected from an Illegal card by stopping power supply to the card in which case this bit indicates a failure status. Reading 1 means the HC is not supplying power to SD card due to some failure. Reading 0 means that the HC is supplying power and no error has occurred. This bit shall always set to be 0, if the HC does not support this function. Note: The current_Limit_Error is to be implemented if customer application requires it.. By default it is not implementedas there is no specific requirement from Customers."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Currentlimiterr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autocmderr {
    #[doc = "0: Error Auto CMD12 and Auto CMD23 use this error status. This bit is set when detecting that one of the bits D00-D04 in Auto CMD Error Status register has changed from 0 to 1. In case of Auto CMD12, this bit is set to 1, not only when the errors in Auto CMD12 occur but also when Auto CMD12 is not executed due to the previous command error."]
    B0 = 0,
    #[doc = "1: Error Auto CMD12 and Auto CMD23 use this error status. This bit is set when detecting that one of the bits D00-D04 in Auto CMD Error Status register has changed from 0 to 1. In case of Auto CMD12, this bit is set to 1, not only when the errors in Auto CMD12 occur but also when Auto CMD12 is not executed due to the previous command error."]
    B1 = 1,
}
impl From<Autocmderr> for bool {
    #[inline(always)]
    fn from(variant: Autocmderr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOCMDERR` reader - "]
pub type AutocmderrR = crate::BitReader<Autocmderr>;
impl AutocmderrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autocmderr {
        match self.bits {
            false => Autocmderr::B0,
            true => Autocmderr::B1,
        }
    }
    #[doc = "Error Auto CMD12 and Auto CMD23 use this error status. This bit is set when detecting that one of the bits D00-D04 in Auto CMD Error Status register has changed from 0 to 1. In case of Auto CMD12, this bit is set to 1, not only when the errors in Auto CMD12 occur but also when Auto CMD12 is not executed due to the previous command error."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Autocmderr::B0
    }
    #[doc = "Error Auto CMD12 and Auto CMD23 use this error status. This bit is set when detecting that one of the bits D00-D04 in Auto CMD Error Status register has changed from 0 to 1. In case of Auto CMD12, this bit is set to 1, not only when the errors in Auto CMD12 occur but also when Auto CMD12 is not executed due to the previous command error."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Autocmderr::B1
    }
}
#[doc = "Field `AUTOCMDERR` writer - "]
pub type AutocmderrW<'a, REG> = crate::BitWriter1C<'a, REG, Autocmderr>;
impl<'a, REG> AutocmderrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error Auto CMD12 and Auto CMD23 use this error status. This bit is set when detecting that one of the bits D00-D04 in Auto CMD Error Status register has changed from 0 to 1. In case of Auto CMD12, this bit is set to 1, not only when the errors in Auto CMD12 occur but also when Auto CMD12 is not executed due to the previous command error."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Autocmderr::B0)
    }
    #[doc = "Error Auto CMD12 and Auto CMD23 use this error status. This bit is set when detecting that one of the bits D00-D04 in Auto CMD Error Status register has changed from 0 to 1. In case of Auto CMD12, this bit is set to 1, not only when the errors in Auto CMD12 occur but also when Auto CMD12 is not executed due to the previous command error."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Autocmderr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admaerr {
    #[doc = "1: No error This bit is set when the Host Controller detects errors during ADMA based data transfer. The state of the ADMA at an error occurrence is saved in the ADMA Error Status Register."]
    B1 = 1,
    #[doc = "0: No error This bit is set when the Host Controller detects errors during ADMA based data transfer. The state of the ADMA at an error occurrence is saved in the ADMA Error Status Register."]
    B0 = 0,
}
impl From<Admaerr> for bool {
    #[inline(always)]
    fn from(variant: Admaerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMAERR` reader - "]
pub type AdmaerrR = crate::BitReader<Admaerr>;
impl AdmaerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Admaerr {
        match self.bits {
            true => Admaerr::B1,
            false => Admaerr::B0,
        }
    }
    #[doc = "No error This bit is set when the Host Controller detects errors during ADMA based data transfer. The state of the ADMA at an error occurrence is saved in the ADMA Error Status Register."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Admaerr::B1
    }
    #[doc = "No error This bit is set when the Host Controller detects errors during ADMA based data transfer. The state of the ADMA at an error occurrence is saved in the ADMA Error Status Register."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Admaerr::B0
    }
}
#[doc = "Field `ADMAERR` writer - "]
pub type AdmaerrW<'a, REG> = crate::BitWriter1C<'a, REG, Admaerr>;
impl<'a, REG> AdmaerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error This bit is set when the Host Controller detects errors during ADMA based data transfer. The state of the ADMA at an error occurrence is saved in the ADMA Error Status Register."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Admaerr::B1)
    }
    #[doc = "No error This bit is set when the Host Controller detects errors during ADMA based data transfer. The state of the ADMA at an error occurrence is saved in the ADMA Error Status Register."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Admaerr::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Targetresperr {
    #[doc = "0: error Occurs when detecting ERROR in m_hresp(dma transaction)"]
    B0 = 0,
    #[doc = "1: error Occurs when detecting ERROR in m_hresp(dma transaction)"]
    B1 = 1,
}
impl From<Targetresperr> for bool {
    #[inline(always)]
    fn from(variant: Targetresperr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TARGETRESPERR` reader - "]
pub type TargetresperrR = crate::BitReader<Targetresperr>;
impl TargetresperrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Targetresperr {
        match self.bits {
            false => Targetresperr::B0,
            true => Targetresperr::B1,
        }
    }
    #[doc = "error Occurs when detecting ERROR in m_hresp(dma transaction)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Targetresperr::B0
    }
    #[doc = "error Occurs when detecting ERROR in m_hresp(dma transaction)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Targetresperr::B1
    }
}
#[doc = "Field `TARGETRESPERR` writer - "]
pub type TargetresperrW<'a, REG> = crate::BitWriter1C<'a, REG, Targetresperr>;
impl<'a, REG> TargetresperrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "error Occurs when detecting ERROR in m_hresp(dma transaction)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Targetresperr::B0)
    }
    #[doc = "error Occurs when detecting ERROR in m_hresp(dma transaction)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Targetresperr::B1)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmdtimeouterr(&self) -> CmdtimeouterrR {
        CmdtimeouterrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cmdcrcerr(&self) -> CmdcrcerrR {
        CmdcrcerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cmdendbiterr(&self) -> CmdendbiterrR {
        CmdendbiterrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cmdindexerr(&self) -> CmdindexerrR {
        CmdindexerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn datatimeouterr(&self) -> DatatimeouterrR {
        DatatimeouterrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn datacrcerr(&self) -> DatacrcerrR {
        DatacrcerrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dataendbiterr(&self) -> DataendbiterrR {
        DataendbiterrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn currentlimiterr(&self) -> CurrentlimiterrR {
        CurrentlimiterrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn autocmderr(&self) -> AutocmderrR {
        AutocmderrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn admaerr(&self) -> AdmaerrR {
        AdmaerrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn targetresperr(&self) -> TargetresperrR {
        TargetresperrR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtimeouterr(&mut self) -> CmdtimeouterrW<EmmccoreErrintstsSpec> {
        CmdtimeouterrW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcrcerr(&mut self) -> CmdcrcerrW<EmmccoreErrintstsSpec> {
        CmdcrcerrW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cmdendbiterr(&mut self) -> CmdendbiterrW<EmmccoreErrintstsSpec> {
        CmdendbiterrW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cmdindexerr(&mut self) -> CmdindexerrW<EmmccoreErrintstsSpec> {
        CmdindexerrW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn datatimeouterr(&mut self) -> DatatimeouterrW<EmmccoreErrintstsSpec> {
        DatatimeouterrW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn datacrcerr(&mut self) -> DatacrcerrW<EmmccoreErrintstsSpec> {
        DatacrcerrW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn dataendbiterr(&mut self) -> DataendbiterrW<EmmccoreErrintstsSpec> {
        DataendbiterrW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn currentlimiterr(&mut self) -> CurrentlimiterrW<EmmccoreErrintstsSpec> {
        CurrentlimiterrW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn autocmderr(&mut self) -> AutocmderrW<EmmccoreErrintstsSpec> {
        AutocmderrW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn admaerr(&mut self) -> AdmaerrW<EmmccoreErrintstsSpec> {
        AdmaerrW::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn targetresperr(&mut self) -> TargetresperrW<EmmccoreErrintstsSpec> {
        TargetresperrW::new(self, 12)
    }
}
#[doc = "Error interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_errintsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_errintsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreErrintstsSpec;
impl crate::RegisterSpec for EmmccoreErrintstsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`emmccore_errintsts::R`](R) reader structure"]
impl crate::Readable for EmmccoreErrintstsSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_errintsts::W`](W) writer structure"]
impl crate::Writable for EmmccoreErrintstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0x13fb;
}
#[doc = "`reset()` method sets EMMCCORE_ERRINTSTS to value 0"]
impl crate::Resettable for EmmccoreErrintstsSpec {
    const RESET_VALUE: u16 = 0;
}
