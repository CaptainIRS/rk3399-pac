#[doc = "Register `EMMCCORE_CMD` reader"]
pub type R = crate::R<EmmccoreCmdSpec>;
#[doc = "Register `EMMCCORE_CMD` writer"]
pub type W = crate::W<EmmccoreCmdSpec>;
#[doc = "Response Type Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Resptypesel {
    #[doc = "0: No Response"]
    D0 = 0,
    #[doc = "1: Response length 136"]
    D1 = 1,
    #[doc = "2: Response length 48"]
    D2 = 2,
    #[doc = "3: Response length 48 check Busy after response"]
    D3 = 3,
}
impl From<Resptypesel> for u8 {
    #[inline(always)]
    fn from(variant: Resptypesel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Resptypesel {
    type Ux = u8;
}
#[doc = "Field `RESPTYPESEL` reader - Response Type Select"]
pub type ResptypeselR = crate::FieldReader<Resptypesel>;
impl ResptypeselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resptypesel {
        match self.bits {
            0 => Resptypesel::D0,
            1 => Resptypesel::D1,
            2 => Resptypesel::D2,
            3 => Resptypesel::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "No Response"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Resptypesel::D0
    }
    #[doc = "Response length 136"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Resptypesel::D1
    }
    #[doc = "Response length 48"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Resptypesel::D2
    }
    #[doc = "Response length 48 check Busy after response"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Resptypesel::D3
    }
}
#[doc = "Field `RESPTYPESEL` writer - Response Type Select"]
pub type ResptypeselW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Resptypesel>;
impl<'a, REG> ResptypeselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Response"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(Resptypesel::D0)
    }
    #[doc = "Response length 136"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(Resptypesel::D1)
    }
    #[doc = "Response length 48"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(Resptypesel::D2)
    }
    #[doc = "Response length 48 check Busy after response"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(Resptypesel::D3)
    }
}
#[doc = "If this bit is set to 1, the HC shall check the CRC field in the\n\nresponse. If an error is detected, it is reported as a Command\n\nCRC Error. If this bit is set to 0, the CRC field is not checked.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdcrcchkena {
    #[doc = "0: Disable"]
    B0 = 0,
    #[doc = "1: Enable"]
    B1 = 1,
}
impl From<Cmdcrcchkena> for bool {
    #[inline(always)]
    fn from(variant: Cmdcrcchkena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDCRCCHKENA` reader - If this bit is set to 1, the HC shall check the CRC field in the\n\nresponse. If an error is detected, it is reported as a Command\n\nCRC Error. If this bit is set to 0, the CRC field is not checked."]
pub type CmdcrcchkenaR = crate::BitReader<Cmdcrcchkena>;
impl CmdcrcchkenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdcrcchkena {
        match self.bits {
            false => Cmdcrcchkena::B0,
            true => Cmdcrcchkena::B1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cmdcrcchkena::B0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cmdcrcchkena::B1
    }
}
#[doc = "Field `CMDCRCCHKENA` writer - If this bit is set to 1, the HC shall check the CRC field in the\n\nresponse. If an error is detected, it is reported as a Command\n\nCRC Error. If this bit is set to 0, the CRC field is not checked."]
pub type CmdcrcchkenaW<'a, REG> = crate::BitWriter<'a, REG, Cmdcrcchkena>;
impl<'a, REG> CmdcrcchkenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcrcchkena::B0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcrcchkena::B1)
    }
}
#[doc = "If this bit is set to 1, the HC shall check the index field in the\n\nresponse to see if it has the same value as the command index.\n\nIf it is not, it is reported as a Command Index Error. If this bit is\n\nset to 0, the Index field is not checked.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdindexchkena {
    #[doc = "0: Disable"]
    B0 = 0,
    #[doc = "1: Enable"]
    B1 = 1,
}
impl From<Cmdindexchkena> for bool {
    #[inline(always)]
    fn from(variant: Cmdindexchkena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDINDEXCHKENA` reader - If this bit is set to 1, the HC shall check the index field in the\n\nresponse to see if it has the same value as the command index.\n\nIf it is not, it is reported as a Command Index Error. If this bit is\n\nset to 0, the Index field is not checked."]
pub type CmdindexchkenaR = crate::BitReader<Cmdindexchkena>;
impl CmdindexchkenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdindexchkena {
        match self.bits {
            false => Cmdindexchkena::B0,
            true => Cmdindexchkena::B1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cmdindexchkena::B0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cmdindexchkena::B1
    }
}
#[doc = "Field `CMDINDEXCHKENA` writer - If this bit is set to 1, the HC shall check the index field in the\n\nresponse to see if it has the same value as the command index.\n\nIf it is not, it is reported as a Command Index Error. If this bit is\n\nset to 0, the Index field is not checked."]
pub type CmdindexchkenaW<'a, REG> = crate::BitWriter<'a, REG, Cmdindexchkena>;
impl<'a, REG> CmdindexchkenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdindexchkena::B0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdindexchkena::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datapresentsel {
    #[doc = "0: No Data Present"]
    B0 = 0,
    #[doc = "1: Data Present This bit is set to 1 to indicate that data is present and shall be transferred using the DAT line. If is set to 0 for the following: a. Commands using only CMD line (ex. CMD52) b. Commands with no data transfer but using busy signal on DAT\\[0\\]
line (R1b or R5b ex. CMD38) c Resume Command"]
    B1 = 1,
}
impl From<Datapresentsel> for bool {
    #[inline(always)]
    fn from(variant: Datapresentsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAPRESENTSEL` reader - "]
pub type DatapresentselR = crate::BitReader<Datapresentsel>;
impl DatapresentselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datapresentsel {
        match self.bits {
            false => Datapresentsel::B0,
            true => Datapresentsel::B1,
        }
    }
    #[doc = "No Data Present"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Datapresentsel::B0
    }
    #[doc = "Data Present This bit is set to 1 to indicate that data is present and shall be transferred using the DAT line. If is set to 0 for the following: a. Commands using only CMD line (ex. CMD52) b. Commands with no data transfer but using busy signal on DAT\\[0\\]
line (R1b or R5b ex. CMD38) c Resume Command"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Datapresentsel::B1
    }
}
#[doc = "Field `DATAPRESENTSEL` writer - "]
pub type DatapresentselW<'a, REG> = crate::BitWriter<'a, REG, Datapresentsel>;
impl<'a, REG> DatapresentselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Data Present"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Datapresentsel::B0)
    }
    #[doc = "Data Present This bit is set to 1 to indicate that data is present and shall be transferred using the DAT line. If is set to 0 for the following: a. Commands using only CMD line (ex. CMD52) b. Commands with no data transfer but using busy signal on DAT\\[0\\]
line (R1b or R5b ex. CMD38) c Resume Command"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Datapresentsel::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdtype {
    #[doc = "0: Normal"]
    D0 = 0,
    #[doc = "1: Suspend"]
    D1 = 1,
    #[doc = "2: Resume"]
    D2 = 2,
    #[doc = "3: Abort There are three types of special commands. Suspend, Resume and Abort. These bits shall bet set to 00b for all other commands. Suspend Command If the Suspend command succeeds, the HC shall assume the SD Bus has been released and that it is possible to issue the next command which uses the DAT line. The HC shall de-assert Read Wait for read transactions and stop checking busy for write transactions. The Interrupt cycle shall start, in 4-bit mode. If the Suspend command fails, the HC shall maintain its current state. and the HD shall restart the transfer by setting Continue Request in the Block Gap Control Register. Resume Command The HD re-starts the data transfer by restoring the registers in the range of 000-00Dh. The HC shall check for busy before starting write transfers. Abort Command If this command is set when executing a read transfer, the HC shall stop reads to the buffer. If this command is set when executing a write transfer, the HC shall stop driving the DAT line. After issuing the Abort command, the HD should issue a software reset."]
    D3 = 3,
}
impl From<Cmdtype> for u8 {
    #[inline(always)]
    fn from(variant: Cmdtype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdtype {
    type Ux = u8;
}
#[doc = "Field `CMDTYPE` reader - "]
pub type CmdtypeR = crate::FieldReader<Cmdtype>;
impl CmdtypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdtype {
        match self.bits {
            0 => Cmdtype::D0,
            1 => Cmdtype::D1,
            2 => Cmdtype::D2,
            3 => Cmdtype::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Cmdtype::D0
    }
    #[doc = "Suspend"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Cmdtype::D1
    }
    #[doc = "Resume"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Cmdtype::D2
    }
    #[doc = "Abort There are three types of special commands. Suspend, Resume and Abort. These bits shall bet set to 00b for all other commands. Suspend Command If the Suspend command succeeds, the HC shall assume the SD Bus has been released and that it is possible to issue the next command which uses the DAT line. The HC shall de-assert Read Wait for read transactions and stop checking busy for write transactions. The Interrupt cycle shall start, in 4-bit mode. If the Suspend command fails, the HC shall maintain its current state. and the HD shall restart the transfer by setting Continue Request in the Block Gap Control Register. Resume Command The HD re-starts the data transfer by restoring the registers in the range of 000-00Dh. The HC shall check for busy before starting write transfers. Abort Command If this command is set when executing a read transfer, the HC shall stop reads to the buffer. If this command is set when executing a write transfer, the HC shall stop driving the DAT line. After issuing the Abort command, the HD should issue a software reset."]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Cmdtype::D3
    }
}
#[doc = "Field `CMDTYPE` writer - "]
pub type CmdtypeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Cmdtype>;
impl<'a, REG> CmdtypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtype::D0)
    }
    #[doc = "Suspend"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtype::D1)
    }
    #[doc = "Resume"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtype::D2)
    }
    #[doc = "Abort There are three types of special commands. Suspend, Resume and Abort. These bits shall bet set to 00b for all other commands. Suspend Command If the Suspend command succeeds, the HC shall assume the SD Bus has been released and that it is possible to issue the next command which uses the DAT line. The HC shall de-assert Read Wait for read transactions and stop checking busy for write transactions. The Interrupt cycle shall start, in 4-bit mode. If the Suspend command fails, the HC shall maintain its current state. and the HD shall restart the transfer by setting Continue Request in the Block Gap Control Register. Resume Command The HD re-starts the data transfer by restoring the registers in the range of 000-00Dh. The HC shall check for busy before starting write transfers. Abort Command If this command is set when executing a read transfer, the HC shall stop reads to the buffer. If this command is set when executing a write transfer, the HC shall stop driving the DAT line. After issuing the Abort command, the HD should issue a software reset."]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtype::D3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Response Type Select"]
    #[inline(always)]
    pub fn resptypesel(&self) -> ResptypeselR {
        ResptypeselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - If this bit is set to 1, the HC shall check the CRC field in the\n\nresponse. If an error is detected, it is reported as a Command\n\nCRC Error. If this bit is set to 0, the CRC field is not checked."]
    #[inline(always)]
    pub fn cmdcrcchkena(&self) -> CmdcrcchkenaR {
        CmdcrcchkenaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If this bit is set to 1, the HC shall check the index field in the\n\nresponse to see if it has the same value as the command index.\n\nIf it is not, it is reported as a Command Index Error. If this bit is\n\nset to 0, the Index field is not checked."]
    #[inline(always)]
    pub fn cmdindexchkena(&self) -> CmdindexchkenaR {
        CmdindexchkenaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn datapresentsel(&self) -> DatapresentselR {
        DatapresentselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn cmdtype(&self) -> CmdtypeR {
        CmdtypeR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Response Type Select"]
    #[inline(always)]
    #[must_use]
    pub fn resptypesel(&mut self) -> ResptypeselW<EmmccoreCmdSpec> {
        ResptypeselW::new(self, 0)
    }
    #[doc = "Bit 3 - If this bit is set to 1, the HC shall check the CRC field in the\n\nresponse. If an error is detected, it is reported as a Command\n\nCRC Error. If this bit is set to 0, the CRC field is not checked."]
    #[inline(always)]
    #[must_use]
    pub fn cmdcrcchkena(&mut self) -> CmdcrcchkenaW<EmmccoreCmdSpec> {
        CmdcrcchkenaW::new(self, 3)
    }
    #[doc = "Bit 4 - If this bit is set to 1, the HC shall check the index field in the\n\nresponse to see if it has the same value as the command index.\n\nIf it is not, it is reported as a Command Index Error. If this bit is\n\nset to 0, the Index field is not checked."]
    #[inline(always)]
    #[must_use]
    pub fn cmdindexchkena(&mut self) -> CmdindexchkenaW<EmmccoreCmdSpec> {
        CmdindexchkenaW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn datapresentsel(&mut self) -> DatapresentselW<EmmccoreCmdSpec> {
        DatapresentselW::new(self, 5)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtype(&mut self) -> CmdtypeW<EmmccoreCmdSpec> {
        CmdtypeW::new(self, 6)
    }
}
#[doc = "Command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreCmdSpec;
impl crate::RegisterSpec for EmmccoreCmdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`emmccore_cmd::R`](R) reader structure"]
impl crate::Readable for EmmccoreCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_cmd::W`](W) writer structure"]
impl crate::Writable for EmmccoreCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_CMD to value 0"]
impl crate::Resettable for EmmccoreCmdSpec {
    const RESET_VALUE: u16 = 0;
}
