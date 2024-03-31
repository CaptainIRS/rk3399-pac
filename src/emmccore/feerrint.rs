#[doc = "Register `FEERRINT` reader"]
pub type R = crate::R<FeerrintSpec>;
#[doc = "Register `FEERRINT` writer"]
pub type W = crate::W<FeerrintSpec>;
#[doc = "Force Event for Command Timeout Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdtimeouterr {
    #[doc = "1: Interrupt is generated"]
    B1 = 1,
    #[doc = "0: No interrupt"]
    B0 = 0,
}
impl From<Cmdtimeouterr> for bool {
    #[inline(always)]
    fn from(variant: Cmdtimeouterr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDTIMEOUTERR` writer - Force Event for Command Timeout Error"]
pub type CmdtimeouterrW<'a, REG> = crate::BitWriter<'a, REG, Cmdtimeouterr>;
impl<'a, REG> CmdtimeouterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtimeouterr::B1)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtimeouterr::B0)
    }
}
#[doc = "Force Event for Command CRC Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdcrcerr {
    #[doc = "1: Interrupt is generated"]
    B1 = 1,
    #[doc = "0: No interrupt"]
    B0 = 0,
}
impl From<Cmdcrcerr> for bool {
    #[inline(always)]
    fn from(variant: Cmdcrcerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDCRCERR` writer - Force Event for Command CRC Error"]
pub type CmdcrcerrW<'a, REG> = crate::BitWriter<'a, REG, Cmdcrcerr>;
impl<'a, REG> CmdcrcerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcrcerr::B1)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcrcerr::B0)
    }
}
#[doc = "Force Event for Command End Bit Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdendbiterr {
    #[doc = "1: Interrupt is generated"]
    B1 = 1,
    #[doc = "0: No interrupt"]
    B0 = 0,
}
impl From<Cmdendbiterr> for bool {
    #[inline(always)]
    fn from(variant: Cmdendbiterr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDENDBITERR` writer - Force Event for Command End Bit Error"]
pub type CmdendbiterrW<'a, REG> = crate::BitWriter<'a, REG, Cmdendbiterr>;
impl<'a, REG> CmdendbiterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdendbiterr::B1)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdendbiterr::B0)
    }
}
#[doc = "Force Event for Command Index Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdindexerr {
    #[doc = "1: Interrupt is generated"]
    B1 = 1,
    #[doc = "0: No interrupt"]
    B0 = 0,
}
impl From<Cmdindexerr> for bool {
    #[inline(always)]
    fn from(variant: Cmdindexerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDINDEXERR` writer - Force Event for Command Index Error"]
pub type CmdindexerrW<'a, REG> = crate::BitWriter<'a, REG, Cmdindexerr>;
impl<'a, REG> CmdindexerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdindexerr::B1)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdindexerr::B0)
    }
}
#[doc = "Force Event for Data Timeout Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dattimeouterr {
    #[doc = "1: Interrupt is generated"]
    B1 = 1,
    #[doc = "0: No interrupt"]
    B0 = 0,
}
impl From<Dattimeouterr> for bool {
    #[inline(always)]
    fn from(variant: Dattimeouterr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATTIMEOUTERR` writer - Force Event for Data Timeout Error"]
pub type DattimeouterrW<'a, REG> = crate::BitWriter<'a, REG, Dattimeouterr>;
impl<'a, REG> DattimeouterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dattimeouterr::B1)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dattimeouterr::B0)
    }
}
#[doc = "Force Event for Data CRC Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datcrcerr {
    #[doc = "1: Interrupt is generated"]
    B1 = 1,
    #[doc = "0: No interrupt"]
    B0 = 0,
}
impl From<Datcrcerr> for bool {
    #[inline(always)]
    fn from(variant: Datcrcerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATCRCERR` writer - Force Event for Data CRC Error"]
pub type DatcrcerrW<'a, REG> = crate::BitWriter<'a, REG, Datcrcerr>;
impl<'a, REG> DatcrcerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Datcrcerr::B1)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Datcrcerr::B0)
    }
}
#[doc = "Force Event for Data End Bit Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datendbiterr {
    #[doc = "1: Interrupt is generated"]
    B1 = 1,
    #[doc = "0: No interrupt"]
    B0 = 0,
}
impl From<Datendbiterr> for bool {
    #[inline(always)]
    fn from(variant: Datendbiterr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATENDBITERR` writer - Force Event for Data End Bit Error"]
pub type DatendbiterrW<'a, REG> = crate::BitWriter<'a, REG, Datendbiterr>;
impl<'a, REG> DatendbiterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Datendbiterr::B1)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Datendbiterr::B0)
    }
}
#[doc = "Force Event for Current Limit Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Currenterr {
    #[doc = "1: Interrupt is generated"]
    B1 = 1,
    #[doc = "0: No interrupt"]
    B0 = 0,
}
impl From<Currenterr> for bool {
    #[inline(always)]
    fn from(variant: Currenterr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CURRENTERR` writer - Force Event for Current Limit Error"]
pub type CurrenterrW<'a, REG> = crate::BitWriter<'a, REG, Currenterr>;
impl<'a, REG> CurrenterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Currenterr::B1)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Currenterr::B0)
    }
}
#[doc = "Force Event for Auto CMD Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmderr {
    #[doc = "1: Interrupt is generated"]
    B1 = 1,
    #[doc = "0: No interrupt"]
    B0 = 0,
}
impl From<Acmderr> for bool {
    #[inline(always)]
    fn from(variant: Acmderr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDERR` writer - Force Event for Auto CMD Error"]
pub type AcmderrW<'a, REG> = crate::BitWriter<'a, REG, Acmderr>;
impl<'a, REG> AcmderrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Acmderr::B1)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Acmderr::B0)
    }
}
#[doc = "Force Event for ADMA Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admaerr {
    #[doc = "1: Interrupt is generated"]
    B1 = 1,
    #[doc = "0: No interrupt"]
    B0 = 0,
}
impl From<Admaerr> for bool {
    #[inline(always)]
    fn from(variant: Admaerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMAERR` writer - Force Event for ADMA Error"]
pub type AdmaerrW<'a, REG> = crate::BitWriter<'a, REG, Admaerr>;
impl<'a, REG> AdmaerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Admaerr::B1)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Admaerr::B0)
    }
}
#[doc = "Field `VENDORERR` reader - Force Event for Vendor Specific Error Status"]
pub type VendorerrR = crate::FieldReader;
impl R {
    #[doc = "Bits 12:15 - Force Event for Vendor Specific Error Status"]
    #[inline(always)]
    pub fn vendorerr(&self) -> VendorerrR {
        VendorerrR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Force Event for Command Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtimeouterr(&mut self) -> CmdtimeouterrW<FeerrintSpec> {
        CmdtimeouterrW::new(self, 0)
    }
    #[doc = "Bit 1 - Force Event for Command CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcrcerr(&mut self) -> CmdcrcerrW<FeerrintSpec> {
        CmdcrcerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Force Event for Command End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmdendbiterr(&mut self) -> CmdendbiterrW<FeerrintSpec> {
        CmdendbiterrW::new(self, 2)
    }
    #[doc = "Bit 3 - Force Event for Command Index Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmdindexerr(&mut self) -> CmdindexerrW<FeerrintSpec> {
        CmdindexerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Force Event for Data Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn dattimeouterr(&mut self) -> DattimeouterrW<FeerrintSpec> {
        DattimeouterrW::new(self, 4)
    }
    #[doc = "Bit 5 - Force Event for Data CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn datcrcerr(&mut self) -> DatcrcerrW<FeerrintSpec> {
        DatcrcerrW::new(self, 5)
    }
    #[doc = "Bit 6 - Force Event for Data End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn datendbiterr(&mut self) -> DatendbiterrW<FeerrintSpec> {
        DatendbiterrW::new(self, 6)
    }
    #[doc = "Bit 7 - Force Event for Current Limit Error"]
    #[inline(always)]
    #[must_use]
    pub fn currenterr(&mut self) -> CurrenterrW<FeerrintSpec> {
        CurrenterrW::new(self, 7)
    }
    #[doc = "Bit 8 - Force Event for Auto CMD Error"]
    #[inline(always)]
    #[must_use]
    pub fn acmderr(&mut self) -> AcmderrW<FeerrintSpec> {
        AcmderrW::new(self, 8)
    }
    #[doc = "Bit 9 - Force Event for ADMA Error"]
    #[inline(always)]
    #[must_use]
    pub fn admaerr(&mut self) -> AdmaerrW<FeerrintSpec> {
        AdmaerrW::new(self, 9)
    }
}
#[doc = "Force event register for error interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`feerrint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`feerrint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeerrintSpec;
impl crate::RegisterSpec for FeerrintSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`feerrint::R`](R) reader structure"]
impl crate::Readable for FeerrintSpec {}
#[doc = "`write(|w| ..)` method takes [`feerrint::W`](W) writer structure"]
impl crate::Writable for FeerrintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FEERRINT to value 0"]
impl crate::Resettable for FeerrintSpec {
    const RESET_VALUE: u16 = 0;
}
