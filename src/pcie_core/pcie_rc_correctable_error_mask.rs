#[doc = "Register `PCIE_RC_CORRECTABLE_ERROR_MASK` reader"]
pub type R = crate::R<PcieRcCorrectableErrorMaskSpec>;
#[doc = "Register `PCIE_RC_CORRECTABLE_ERROR_MASK` writer"]
pub type W = crate::W<PcieRcCorrectableErrorMaskSpec>;
#[doc = "Field `REM` reader - Receiver Error Mask \\[REM\\]
This bit, when set, masks the reporting of Physical Layer errors. STICKY."]
pub type RemR = crate::BitReader;
#[doc = "Field `REM` writer - Receiver Error Mask \\[REM\\]
This bit, when set, masks the reporting of Physical Layer errors. STICKY."]
pub type RemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R40` reader - Reserved \\[R40\\]
Reserved"]
pub type R40R = crate::FieldReader;
#[doc = "Field `BTM` reader - Bad TP Mask \\[BTM\\]
This bit, when set, masks the reporting of an error in response to a 'Bad TLP' received. STICKY."]
pub type BtmR = crate::BitReader;
#[doc = "Field `BTM` writer - Bad TP Mask \\[BTM\\]
This bit, when set, masks the reporting of an error in response to a 'Bad TLP' received. STICKY."]
pub type BtmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDM` reader - Bad DLLP Mask \\[BDM\\]
This bit, when set, masks the reporting of an error in response to a 'Bad DLLP' received. STICKY."]
pub type BdmR = crate::BitReader;
#[doc = "Field `BDM` writer - Bad DLLP Mask \\[BDM\\]
This bit, when set, masks the reporting of an error in response to a 'Bad DLLP' received. STICKY."]
pub type BdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNRM` reader - Replay Number Rollover Mask \\[RNRM\\]
This bit, when set, masks the reporting of an error in response to a Replay Number Rollover event. STICKY."]
pub type RnrmR = crate::BitReader;
#[doc = "Field `RNRM` writer - Replay Number Rollover Mask \\[RNRM\\]
This bit, when set, masks the reporting of an error in response to a Replay Number Rollover event. STICKY."]
pub type RnrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R41` reader - Reserved \\[R41\\]
Reserved"]
pub type R41R = crate::FieldReader;
#[doc = "Field `RTTM` reader - Replay Timer Timeout Mask \\[RTTM\\]
This bit, when set, masks the reporting of an error in response to a Replay Timer timeout event. STICKY."]
pub type RttmR = crate::BitReader;
#[doc = "Field `RTTM` writer - Replay Timer Timeout Mask \\[RTTM\\]
This bit, when set, masks the reporting of an error in response to a Replay Timer timeout event. STICKY."]
pub type RttmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANEM` reader - Advisory Non- Fatal Error Mask \\[ANEM\\]
This bit, when set, masks the reporting of an error in response to an uncorrectable error occurrence, which is determined to belong to one of the special cases in the PCI Express Base Specification 2.0. STICKY."]
pub type AnemR = crate::BitReader;
#[doc = "Field `ANEM` writer - Advisory Non- Fatal Error Mask \\[ANEM\\]
This bit, when set, masks the reporting of an error in response to an uncorrectable error occurrence, which is determined to belong to one of the special cases in the PCI Express Base Specification 2.0. STICKY."]
pub type AnemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIEM` reader - Corrected Internal Error Mask \\[CIEM\\]
This bit, when set, masks the reporting of an error in response to a corrected internal error condition. STICKY."]
pub type CiemR = crate::BitReader;
#[doc = "Field `CIEM` writer - Corrected Internal Error Mask \\[CIEM\\]
This bit, when set, masks the reporting of an error in response to a corrected internal error condition. STICKY."]
pub type CiemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HLOM` reader - Header Log Overflow Mask \\[HLOM\\]
This bit, when set, masks the reporting of an error in response to a Header Log register overflow. STICKY."]
pub type HlomR = crate::BitReader;
#[doc = "Field `HLOM` writer - Header Log Overflow Mask \\[HLOM\\]
This bit, when set, masks the reporting of an error in response to a Header Log register overflow. STICKY."]
pub type HlomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R42` reader - Reserved \\[R42\\]
Reserved"]
pub type R42R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Receiver Error Mask \\[REM\\]
This bit, when set, masks the reporting of Physical Layer errors. STICKY."]
    #[inline(always)]
    pub fn rem(&self) -> RemR {
        RemR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Reserved \\[R40\\]
Reserved"]
    #[inline(always)]
    pub fn r40(&self) -> R40R {
        R40R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Bad TP Mask \\[BTM\\]
This bit, when set, masks the reporting of an error in response to a 'Bad TLP' received. STICKY."]
    #[inline(always)]
    pub fn btm(&self) -> BtmR {
        BtmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bad DLLP Mask \\[BDM\\]
This bit, when set, masks the reporting of an error in response to a 'Bad DLLP' received. STICKY."]
    #[inline(always)]
    pub fn bdm(&self) -> BdmR {
        BdmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Replay Number Rollover Mask \\[RNRM\\]
This bit, when set, masks the reporting of an error in response to a Replay Number Rollover event. STICKY."]
    #[inline(always)]
    pub fn rnrm(&self) -> RnrmR {
        RnrmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Reserved \\[R41\\]
Reserved"]
    #[inline(always)]
    pub fn r41(&self) -> R41R {
        R41R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Replay Timer Timeout Mask \\[RTTM\\]
This bit, when set, masks the reporting of an error in response to a Replay Timer timeout event. STICKY."]
    #[inline(always)]
    pub fn rttm(&self) -> RttmR {
        RttmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Advisory Non- Fatal Error Mask \\[ANEM\\]
This bit, when set, masks the reporting of an error in response to an uncorrectable error occurrence, which is determined to belong to one of the special cases in the PCI Express Base Specification 2.0. STICKY."]
    #[inline(always)]
    pub fn anem(&self) -> AnemR {
        AnemR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Corrected Internal Error Mask \\[CIEM\\]
This bit, when set, masks the reporting of an error in response to a corrected internal error condition. STICKY."]
    #[inline(always)]
    pub fn ciem(&self) -> CiemR {
        CiemR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Header Log Overflow Mask \\[HLOM\\]
This bit, when set, masks the reporting of an error in response to a Header Log register overflow. STICKY."]
    #[inline(always)]
    pub fn hlom(&self) -> HlomR {
        HlomR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Reserved \\[R42\\]
Reserved"]
    #[inline(always)]
    pub fn r42(&self) -> R42R {
        R42R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Error Mask \\[REM\\]
This bit, when set, masks the reporting of Physical Layer errors. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn rem(&mut self) -> RemW<PcieRcCorrectableErrorMaskSpec> {
        RemW::new(self, 0)
    }
    #[doc = "Bit 6 - Bad TP Mask \\[BTM\\]
This bit, when set, masks the reporting of an error in response to a 'Bad TLP' received. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn btm(&mut self) -> BtmW<PcieRcCorrectableErrorMaskSpec> {
        BtmW::new(self, 6)
    }
    #[doc = "Bit 7 - Bad DLLP Mask \\[BDM\\]
This bit, when set, masks the reporting of an error in response to a 'Bad DLLP' received. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn bdm(&mut self) -> BdmW<PcieRcCorrectableErrorMaskSpec> {
        BdmW::new(self, 7)
    }
    #[doc = "Bit 8 - Replay Number Rollover Mask \\[RNRM\\]
This bit, when set, masks the reporting of an error in response to a Replay Number Rollover event. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn rnrm(&mut self) -> RnrmW<PcieRcCorrectableErrorMaskSpec> {
        RnrmW::new(self, 8)
    }
    #[doc = "Bit 12 - Replay Timer Timeout Mask \\[RTTM\\]
This bit, when set, masks the reporting of an error in response to a Replay Timer timeout event. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn rttm(&mut self) -> RttmW<PcieRcCorrectableErrorMaskSpec> {
        RttmW::new(self, 12)
    }
    #[doc = "Bit 13 - Advisory Non- Fatal Error Mask \\[ANEM\\]
This bit, when set, masks the reporting of an error in response to an uncorrectable error occurrence, which is determined to belong to one of the special cases in the PCI Express Base Specification 2.0. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn anem(&mut self) -> AnemW<PcieRcCorrectableErrorMaskSpec> {
        AnemW::new(self, 13)
    }
    #[doc = "Bit 14 - Corrected Internal Error Mask \\[CIEM\\]
This bit, when set, masks the reporting of an error in response to a corrected internal error condition. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ciem(&mut self) -> CiemW<PcieRcCorrectableErrorMaskSpec> {
        CiemW::new(self, 14)
    }
    #[doc = "Bit 15 - Header Log Overflow Mask \\[HLOM\\]
This bit, when set, masks the reporting of an error in response to a Header Log register overflow. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn hlom(&mut self) -> HlomW<PcieRcCorrectableErrorMaskSpec> {
        HlomW::new(self, 15)
    }
}
#[doc = "Correctable Error Mask Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_correctable_error_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_correctable_error_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcCorrectableErrorMaskSpec;
impl crate::RegisterSpec for PcieRcCorrectableErrorMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_correctable_error_mask::R`](R) reader structure"]
impl crate::Readable for PcieRcCorrectableErrorMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_correctable_error_mask::W`](W) writer structure"]
impl crate::Writable for PcieRcCorrectableErrorMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_RC_CORRECTABLE_ERROR_MASK to value 0xe000"]
impl crate::Resettable for PcieRcCorrectableErrorMaskSpec {
    const RESET_VALUE: u32 = 0xe000;
}
