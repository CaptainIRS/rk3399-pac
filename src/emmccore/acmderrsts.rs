#[doc = "Register `ACMDERRSTS` reader"]
pub type R = crate::R<AcmderrstsSpec>;
#[doc = "Auto CMD12 not Executed\n\nIf memory multiple block data transfer is not started due to\n\ncommand error, this bit is not set because it is not necessary to\n\nissue Auto CMD12. Setting this bit to 1 means the HC cannot\n\nissue Auto CMD12 to stop memory multiple block transfer due to\n\nsome\n\nerror. If this bit isset to 1, other error status bits (D04 - D01) are\n\nmeaningless.\n\nThis bit is set to 0 when Auto CMD Error is generated by Auto\n\nCMD23.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmd12notexe {
    #[doc = "0: Executed"]
    B0 = 0,
    #[doc = "1: Not Executed"]
    B1 = 1,
}
impl From<Acmd12notexe> for bool {
    #[inline(always)]
    fn from(variant: Acmd12notexe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMD12NOTEXE` reader - Auto CMD12 not Executed\n\nIf memory multiple block data transfer is not started due to\n\ncommand error, this bit is not set because it is not necessary to\n\nissue Auto CMD12. Setting this bit to 1 means the HC cannot\n\nissue Auto CMD12 to stop memory multiple block transfer due to\n\nsome\n\nerror. If this bit isset to 1, other error status bits (D04 - D01) are\n\nmeaningless.\n\nThis bit is set to 0 when Auto CMD Error is generated by Auto\n\nCMD23."]
pub type Acmd12notexeR = crate::BitReader<Acmd12notexe>;
impl Acmd12notexeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmd12notexe {
        match self.bits {
            false => Acmd12notexe::B0,
            true => Acmd12notexe::B1,
        }
    }
    #[doc = "Executed"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Acmd12notexe::B0
    }
    #[doc = "Not Executed"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Acmd12notexe::B1
    }
}
#[doc = "Auto CMD Timeout Error\n\nOccurs if the no response is returned within 64 SDCLK cycles\n\nfrom the end bit of the command.\n\nIf this bit is set to 1, the other error status bits (D04 - D02) are\n\nmeaningless.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmdtimeouterr {
    #[doc = "0: No Error"]
    B0 = 0,
    #[doc = "1: Timeout"]
    B1 = 1,
}
impl From<Acmdtimeouterr> for bool {
    #[inline(always)]
    fn from(variant: Acmdtimeouterr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDTIMEOUTERR` reader - Auto CMD Timeout Error\n\nOccurs if the no response is returned within 64 SDCLK cycles\n\nfrom the end bit of the command.\n\nIf this bit is set to 1, the other error status bits (D04 - D02) are\n\nmeaningless."]
pub type AcmdtimeouterrR = crate::BitReader<Acmdtimeouterr>;
impl AcmdtimeouterrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmdtimeouterr {
        match self.bits {
            false => Acmdtimeouterr::B0,
            true => Acmdtimeouterr::B1,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Acmdtimeouterr::B0
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Acmdtimeouterr::B1
    }
}
#[doc = "Auto CMD CRC Error\n\nOccurs when detecting a CRC error in the command response.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmdcrcerr {
    #[doc = "0: No Error"]
    B0 = 0,
    #[doc = "1: CRC Error Generated"]
    B1 = 1,
}
impl From<Acmdcrcerr> for bool {
    #[inline(always)]
    fn from(variant: Acmdcrcerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDCRCERR` reader - Auto CMD CRC Error\n\nOccurs when detecting a CRC error in the command response."]
pub type AcmdcrcerrR = crate::BitReader<Acmdcrcerr>;
impl AcmdcrcerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmdcrcerr {
        match self.bits {
            false => Acmdcrcerr::B0,
            true => Acmdcrcerr::B1,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Acmdcrcerr::B0
    }
    #[doc = "CRC Error Generated"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Acmdcrcerr::B1
    }
}
#[doc = "Auto CMD End Bit Error.\n\nOccurs when detecting that the end bit of command response is\n\n0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmdendbiterr {
    #[doc = "0: No Error"]
    B0 = 0,
    #[doc = "1: End Bit Error Generated"]
    B1 = 1,
}
impl From<Acmdendbiterr> for bool {
    #[inline(always)]
    fn from(variant: Acmdendbiterr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDENDBITERR` reader - Auto CMD End Bit Error.\n\nOccurs when detecting that the end bit of command response is\n\n0."]
pub type AcmdendbiterrR = crate::BitReader<Acmdendbiterr>;
impl AcmdendbiterrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmdendbiterr {
        match self.bits {
            false => Acmdendbiterr::B0,
            true => Acmdendbiterr::B1,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Acmdendbiterr::B0
    }
    #[doc = "End Bit Error Generated"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Acmdendbiterr::B1
    }
}
#[doc = "Auto CMD Index Error.\n\nOccurs if the Command Index error occurs in response to a\n\ncommand.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmdindexerr {
    #[doc = "0: No Error"]
    B0 = 0,
    #[doc = "1: Error"]
    B1 = 1,
}
impl From<Acmdindexerr> for bool {
    #[inline(always)]
    fn from(variant: Acmdindexerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMDINDEXERR` reader - Auto CMD Index Error.\n\nOccurs if the Command Index error occurs in response to a\n\ncommand."]
pub type AcmdindexerrR = crate::BitReader<Acmdindexerr>;
impl AcmdindexerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmdindexerr {
        match self.bits {
            false => Acmdindexerr::B0,
            true => Acmdindexerr::B1,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Acmdindexerr::B0
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Acmdindexerr::B1
    }
}
#[doc = "Command Not Issued By Auto CMD12 Error.\n\nSetting this bit to 1 means CMD_wo_DAT is not executed due to\n\nan Auto CMD12 error (D04 - D01) in this register.\n\nThis bit is set to 0 when Auto CMD Error is generated by Auto\n\nCMD23.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmdnotissbyacmd12err {
    #[doc = "0: No Error"]
    B0 = 0,
    #[doc = "1: Not Issued"]
    B1 = 1,
}
impl From<Cmdnotissbyacmd12err> for bool {
    #[inline(always)]
    fn from(variant: Cmdnotissbyacmd12err) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMDNOTISSBYACMD12ERR` reader - Command Not Issued By Auto CMD12 Error.\n\nSetting this bit to 1 means CMD_wo_DAT is not executed due to\n\nan Auto CMD12 error (D04 - D01) in this register.\n\nThis bit is set to 0 when Auto CMD Error is generated by Auto\n\nCMD23."]
pub type Cmdnotissbyacmd12errR = crate::BitReader<Cmdnotissbyacmd12err>;
impl Cmdnotissbyacmd12errR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmdnotissbyacmd12err {
        match self.bits {
            false => Cmdnotissbyacmd12err::B0,
            true => Cmdnotissbyacmd12err::B1,
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cmdnotissbyacmd12err::B0
    }
    #[doc = "Not Issued"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cmdnotissbyacmd12err::B1
    }
}
impl R {
    #[doc = "Bit 0 - Auto CMD12 not Executed\n\nIf memory multiple block data transfer is not started due to\n\ncommand error, this bit is not set because it is not necessary to\n\nissue Auto CMD12. Setting this bit to 1 means the HC cannot\n\nissue Auto CMD12 to stop memory multiple block transfer due to\n\nsome\n\nerror. If this bit isset to 1, other error status bits (D04 - D01) are\n\nmeaningless.\n\nThis bit is set to 0 when Auto CMD Error is generated by Auto\n\nCMD23."]
    #[inline(always)]
    pub fn acmd12notexe(&self) -> Acmd12notexeR {
        Acmd12notexeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto CMD Timeout Error\n\nOccurs if the no response is returned within 64 SDCLK cycles\n\nfrom the end bit of the command.\n\nIf this bit is set to 1, the other error status bits (D04 - D02) are\n\nmeaningless."]
    #[inline(always)]
    pub fn acmdtimeouterr(&self) -> AcmdtimeouterrR {
        AcmdtimeouterrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto CMD CRC Error\n\nOccurs when detecting a CRC error in the command response."]
    #[inline(always)]
    pub fn acmdcrcerr(&self) -> AcmdcrcerrR {
        AcmdcrcerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auto CMD End Bit Error.\n\nOccurs when detecting that the end bit of command response is\n\n0."]
    #[inline(always)]
    pub fn acmdendbiterr(&self) -> AcmdendbiterrR {
        AcmdendbiterrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Auto CMD Index Error.\n\nOccurs if the Command Index error occurs in response to a\n\ncommand."]
    #[inline(always)]
    pub fn acmdindexerr(&self) -> AcmdindexerrR {
        AcmdindexerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error.\n\nSetting this bit to 1 means CMD_wo_DAT is not executed due to\n\nan Auto CMD12 error (D04 - D01) in this register.\n\nThis bit is set to 0 when Auto CMD Error is generated by Auto\n\nCMD23."]
    #[inline(always)]
    pub fn cmdnotissbyacmd12err(&self) -> Cmdnotissbyacmd12errR {
        Cmdnotissbyacmd12errR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Auto CMD error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acmderrsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcmderrstsSpec;
impl crate::RegisterSpec for AcmderrstsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`acmderrsts::R`](R) reader structure"]
impl crate::Readable for AcmderrstsSpec {}
#[doc = "`reset()` method sets ACMDERRSTS to value 0"]
impl crate::Resettable for AcmderrstsSpec {
    const RESET_VALUE: u16 = 0;
}