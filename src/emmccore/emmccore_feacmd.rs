#[doc = "Register `EMMCCORE_FEACMD` writer"]
pub type W = crate::W<EmmccoreFeacmdSpec>;
#[doc = "Force Event for Auto CMD12 NOT Executed\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Notexe {
    #[doc = "1: Interrupt is generated"]
    B1 = 1,
    #[doc = "0: No interrupt"]
    B0 = 0,
}
impl From<Notexe> for bool {
    #[inline(always)]
    fn from(variant: Notexe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTEXE` writer - Force Event for Auto CMD12 NOT Executed"]
pub type NotexeW<'a, REG> = crate::BitWriter<'a, REG, Notexe>;
impl<'a, REG> NotexeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Notexe::B1)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Notexe::B0)
    }
}
#[doc = "Force Event for Auto CMD timeout Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timeouterr {
    #[doc = "1: Interrupt is generated"]
    B1 = 1,
    #[doc = "0: No interrupt"]
    B0 = 0,
}
impl From<Timeouterr> for bool {
    #[inline(always)]
    fn from(variant: Timeouterr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUTERR` writer - Force Event for Auto CMD timeout Error"]
pub type TimeouterrW<'a, REG> = crate::BitWriter<'a, REG, Timeouterr>;
impl<'a, REG> TimeouterrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Timeouterr::B1)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Timeouterr::B0)
    }
}
#[doc = "Force Event for Auto CMD CRC Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcerr {
    #[doc = "1: Interrupt is generated"]
    B1 = 1,
    #[doc = "0: No interrupt"]
    B0 = 0,
}
impl From<Crcerr> for bool {
    #[inline(always)]
    fn from(variant: Crcerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERR` writer - Force Event for Auto CMD CRC Error"]
pub type CrcerrW<'a, REG> = crate::BitWriter<'a, REG, Crcerr>;
impl<'a, REG> CrcerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Crcerr::B1)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Crcerr::B0)
    }
}
#[doc = "Force Event for Auto CMD End bit Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enderr {
    #[doc = "1: Interrupt is generated"]
    B1 = 1,
    #[doc = "0: No interrupt"]
    B0 = 0,
}
impl From<Enderr> for bool {
    #[inline(always)]
    fn from(variant: Enderr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDERR` writer - Force Event for Auto CMD End bit Error"]
pub type EnderrW<'a, REG> = crate::BitWriter<'a, REG, Enderr>;
impl<'a, REG> EnderrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Enderr::B1)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Enderr::B0)
    }
}
#[doc = "Force Event for Auto CMD Index Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Indexerr {
    #[doc = "1: Interrupt is generated"]
    B1 = 1,
    #[doc = "0: No interrupt"]
    B0 = 0,
}
impl From<Indexerr> for bool {
    #[inline(always)]
    fn from(variant: Indexerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INDEXERR` writer - Force Event for Auto CMD Index Error"]
pub type IndexerrW<'a, REG> = crate::BitWriter<'a, REG, Indexerr>;
impl<'a, REG> IndexerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Indexerr::B1)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Indexerr::B0)
    }
}
#[doc = "Force Event for command not issued by Auto CMD12 Error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmderr {
    #[doc = "1: Interrupt is generated"]
    B1 = 1,
    #[doc = "0: No interrupt"]
    B0 = 0,
}
impl From<Cmderr> for bool {
    #[inline(always)]
    fn from(variant: Cmderr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDERR` writer - Force Event for command not issued by Auto CMD12 Error"]
pub type CmderrW<'a, REG> = crate::BitWriter<'a, REG, Cmderr>;
impl<'a, REG> CmderrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmderr::B1)
    }
    #[doc = "No interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmderr::B0)
    }
}
impl W {
    #[doc = "Bit 0 - Force Event for Auto CMD12 NOT Executed"]
    #[inline(always)]
    #[must_use]
    pub fn notexe(&mut self) -> NotexeW<EmmccoreFeacmdSpec> {
        NotexeW::new(self, 0)
    }
    #[doc = "Bit 1 - Force Event for Auto CMD timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn timeouterr(&mut self) -> TimeouterrW<EmmccoreFeacmdSpec> {
        TimeouterrW::new(self, 1)
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn crcerr(&mut self) -> CrcerrW<EmmccoreFeacmdSpec> {
        CrcerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn enderr(&mut self) -> EnderrW<EmmccoreFeacmdSpec> {
        EnderrW::new(self, 3)
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error"]
    #[inline(always)]
    #[must_use]
    pub fn indexerr(&mut self) -> IndexerrW<EmmccoreFeacmdSpec> {
        IndexerrW::new(self, 4)
    }
    #[doc = "Bit 7 - Force Event for command not issued by Auto CMD12 Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmderr(&mut self) -> CmderrW<EmmccoreFeacmdSpec> {
        CmderrW::new(self, 7)
    }
}
#[doc = "Force event register for Auto CMD error status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_feacmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreFeacmdSpec;
impl crate::RegisterSpec for EmmccoreFeacmdSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`emmccore_feacmd::W`](W) writer structure"]
impl crate::Writable for EmmccoreFeacmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_FEACMD to value 0"]
impl crate::Resettable for EmmccoreFeacmdSpec {
    const RESET_VALUE: u16 = 0;
}
