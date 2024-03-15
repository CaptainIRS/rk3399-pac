#[doc = "Register `EMMCCORE_ERRINTSTSENA` reader"]
pub type R = crate::R<EmmccoreErrintstsenaSpec>;
#[doc = "Register `EMMCCORE_ERRINTSTSENA` writer"]
pub type W = crate::W<EmmccoreErrintstsenaSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdtimeouterr {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
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
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cmdtimeouterr::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cmdtimeouterr::B1
    }
}
#[doc = "Field `CMDTIMEOUTERR` writer - "]
pub type CmdtimeouterrW<'a, REG> = crate::BitWriter<'a, REG, Cmdtimeouterr>;
impl<'a, REG> CmdtimeouterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtimeouterr::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtimeouterr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdcrcerr {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
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
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cmdcrcerr::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cmdcrcerr::B1
    }
}
#[doc = "Field `CMDCRCERR` writer - "]
pub type CmdcrcerrW<'a, REG> = crate::BitWriter<'a, REG, Cmdcrcerr>;
impl<'a, REG> CmdcrcerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcrcerr::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdcrcerr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdendbiterr {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
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
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cmdendbiterr::B0
    }
    #[doc = "Enabled"]
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
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdendbiterr::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdendbiterr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdindexerr {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
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
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cmdindexerr::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cmdindexerr::B1
    }
}
#[doc = "Field `CMDINDEXERR` writer - "]
pub type CmdindexerrW<'a, REG> = crate::BitWriter<'a, REG, Cmdindexerr>;
impl<'a, REG> CmdindexerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdindexerr::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdindexerr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datatimeouterr {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
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
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Datatimeouterr::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Datatimeouterr::B1
    }
}
#[doc = "Field `DATATIMEOUTERR` writer - "]
pub type DatatimeouterrW<'a, REG> = crate::BitWriter<'a, REG, Datatimeouterr>;
impl<'a, REG> DatatimeouterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Datatimeouterr::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Datatimeouterr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datacrcerr {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
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
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Datacrcerr::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Datacrcerr::B1
    }
}
#[doc = "Field `DATACRCERR` writer - "]
pub type DatacrcerrW<'a, REG> = crate::BitWriter<'a, REG, Datacrcerr>;
impl<'a, REG> DatacrcerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Datacrcerr::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Datacrcerr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dataendbiterr {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
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
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dataendbiterr::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dataendbiterr::B1
    }
}
#[doc = "Field `DATAENDBITERR` writer - "]
pub type DataendbiterrW<'a, REG> = crate::BitWriter<'a, REG, Dataendbiterr>;
impl<'a, REG> DataendbiterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dataendbiterr::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dataendbiterr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Currentlimiterr {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
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
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Currentlimiterr::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Currentlimiterr::B1
    }
}
#[doc = "Field `CURRENTLIMITERR` writer - "]
pub type CurrentlimiterrW<'a, REG> = crate::BitWriter<'a, REG, Currentlimiterr>;
impl<'a, REG> CurrentlimiterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Currentlimiterr::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Currentlimiterr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autocmderr {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
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
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Autocmderr::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Autocmderr::B1
    }
}
#[doc = "Field `AUTOCMDERR` writer - "]
pub type AutocmderrW<'a, REG> = crate::BitWriter<'a, REG, Autocmderr>;
impl<'a, REG> AutocmderrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Autocmderr::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Autocmderr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admaerr {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
    B1 = 1,
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
            false => Admaerr::B0,
            true => Admaerr::B1,
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Admaerr::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Admaerr::B1
    }
}
#[doc = "Field `ADMAERR` writer - "]
pub type AdmaerrW<'a, REG> = crate::BitWriter<'a, REG, Admaerr>;
impl<'a, REG> AdmaerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Admaerr::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Admaerr::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Targetresperr {
    #[doc = "0: Enabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
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
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Targetresperr::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Targetresperr::B1
    }
}
#[doc = "Field `TARGETRESPERR` writer - "]
pub type TargetresperrW<'a, REG> = crate::BitWriter<'a, REG, Targetresperr>;
impl<'a, REG> TargetresperrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Targetresperr::B0)
    }
    #[doc = "Enabled"]
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
    pub fn cmdtimeouterr(&mut self) -> CmdtimeouterrW<EmmccoreErrintstsenaSpec> {
        CmdtimeouterrW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcrcerr(&mut self) -> CmdcrcerrW<EmmccoreErrintstsenaSpec> {
        CmdcrcerrW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cmdendbiterr(&mut self) -> CmdendbiterrW<EmmccoreErrintstsenaSpec> {
        CmdendbiterrW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cmdindexerr(&mut self) -> CmdindexerrW<EmmccoreErrintstsenaSpec> {
        CmdindexerrW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn datatimeouterr(&mut self) -> DatatimeouterrW<EmmccoreErrintstsenaSpec> {
        DatatimeouterrW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn datacrcerr(&mut self) -> DatacrcerrW<EmmccoreErrintstsenaSpec> {
        DatacrcerrW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn dataendbiterr(&mut self) -> DataendbiterrW<EmmccoreErrintstsenaSpec> {
        DataendbiterrW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn currentlimiterr(&mut self) -> CurrentlimiterrW<EmmccoreErrintstsenaSpec> {
        CurrentlimiterrW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn autocmderr(&mut self) -> AutocmderrW<EmmccoreErrintstsenaSpec> {
        AutocmderrW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn admaerr(&mut self) -> AdmaerrW<EmmccoreErrintstsenaSpec> {
        AdmaerrW::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn targetresperr(&mut self) -> TargetresperrW<EmmccoreErrintstsenaSpec> {
        TargetresperrW::new(self, 12)
    }
}
#[doc = "Error interrupt status enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_errintstsena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_errintstsena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreErrintstsenaSpec;
impl crate::RegisterSpec for EmmccoreErrintstsenaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`emmccore_errintstsena::R`](R) reader structure"]
impl crate::Readable for EmmccoreErrintstsenaSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_errintstsena::W`](W) writer structure"]
impl crate::Writable for EmmccoreErrintstsenaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_ERRINTSTSENA to value 0"]
impl crate::Resettable for EmmccoreErrintstsenaSpec {
    const RESET_VALUE: u16 = 0;
}
