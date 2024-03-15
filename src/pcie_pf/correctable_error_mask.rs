#[doc = "Register `CORRECTABLE_ERROR_MASK` reader"]
pub type R = crate::R<CorrectableErrorMaskSpec>;
#[doc = "Register `CORRECTABLE_ERROR_MASK` writer"]
pub type W = crate::W<CorrectableErrorMaskSpec>;
#[doc = "Field `REM` reader - Receiver Error Mask \\[REM\\]
This bit, when set, masks the generation of error messages in response to the Physical Layer errors STICKY."]
pub type RemR = crate::BitReader;
#[doc = "Field `REM` writer - Receiver Error Mask \\[REM\\]
This bit, when set, masks the generation of error messages in response to the Physical Layer errors STICKY."]
pub type RemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R15` reader - Reserved \\[R15\\]
Reserved"]
pub type R15R = crate::FieldReader;
#[doc = "Field `BTM` reader - Bad TLP Mask \\[BTM\\]
This bit, when set, masks the generation of error messages in response to a 'Bad TLP' received. STICKY."]
pub type BtmR = crate::BitReader;
#[doc = "Field `BTM` writer - Bad TLP Mask \\[BTM\\]
This bit, when set, masks the generation of error messages in response to a 'Bad TLP' received. STICKY."]
pub type BtmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDM` reader - Bad DLLP Mask \\[BDM\\]
This bit, when set, masks the generation of error messages in response to a 'Bad DLLP' received. STICKY."]
pub type BdmR = crate::BitReader;
#[doc = "Field `BDM` writer - Bad DLLP Mask \\[BDM\\]
This bit, when set, masks the generation of error messages in response to a 'Bad DLLP' received. STICKY."]
pub type BdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNRM` reader - Replay Number Rollover Mask \\[RNRM\\]
This bit, when set, masks the generation of error messages in response to a Replay Number Rollover event. STICKY."]
pub type RnrmR = crate::BitReader;
#[doc = "Field `RNRM` writer - Replay Number Rollover Mask \\[RNRM\\]
This bit, when set, masks the generation of error messages in response to a Replay Number Rollover event. STICKY."]
pub type RnrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R16` reader - Reserved \\[R16\\]
Reserved"]
pub type R16R = crate::FieldReader;
#[doc = "Field `RTTM` reader - Replay Timer Timeout Mask \\[RTTM\\]
This bit, when set, masks the generation of error messages in response to a Replay Timer timeout event. STICKY."]
pub type RttmR = crate::BitReader;
#[doc = "Field `RTTM` writer - Replay Timer Timeout Mask \\[RTTM\\]
This bit, when set, masks the generation of error messages in response to a Replay Timer timeout event. STICKY."]
pub type RttmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANFEM` reader - Advisory Non-Fatal Error Mask \\[ANFEM\\]
This bit, when set, masks the generation of error messages in response to an uncorrectable error occur, which is determined to belong to one of the special cases (as described in Section 6.2.3.2.4 of the PCI Express 2.0 Specifications). STICKY."]
pub type AnfemR = crate::BitReader;
#[doc = "Field `ANFEM` writer - Advisory Non-Fatal Error Mask \\[ANFEM\\]
This bit, when set, masks the generation of error messages in response to an uncorrectable error occur, which is determined to belong to one of the special cases (as described in Section 6.2.3.2.4 of the PCI Express 2.0 Specifications). STICKY."]
pub type AnfemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIEM` reader - Corrected Internal Error Mask \\[CIEM\\]
This bit, when set, masks the generation of error messages in response to a corrected internal error condition. STICKY."]
pub type CiemR = crate::BitReader;
#[doc = "Field `CIEM` writer - Corrected Internal Error Mask \\[CIEM\\]
This bit, when set, masks the generation of error messages in response to a corrected internal error condition. STICKY."]
pub type CiemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HLOM` reader - Header Log Overflow Mask \\[HLOM\\]
This bit, when set, masks the generation of error messages in response to a Header Log register overflow. STICKY."]
pub type HlomR = crate::BitReader;
#[doc = "Field `HLOM` writer - Header Log Overflow Mask \\[HLOM\\]
This bit, when set, masks the generation of error messages in response to a Header Log register overflow. STICKY."]
pub type HlomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R17` reader - Reserved \\[R17\\]
Reserved"]
pub type R17R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Receiver Error Mask \\[REM\\]
This bit, when set, masks the generation of error messages in response to the Physical Layer errors STICKY."]
    #[inline(always)]
    pub fn rem(&self) -> RemR {
        RemR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Reserved \\[R15\\]
Reserved"]
    #[inline(always)]
    pub fn r15(&self) -> R15R {
        R15R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Bad TLP Mask \\[BTM\\]
This bit, when set, masks the generation of error messages in response to a 'Bad TLP' received. STICKY."]
    #[inline(always)]
    pub fn btm(&self) -> BtmR {
        BtmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bad DLLP Mask \\[BDM\\]
This bit, when set, masks the generation of error messages in response to a 'Bad DLLP' received. STICKY."]
    #[inline(always)]
    pub fn bdm(&self) -> BdmR {
        BdmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Replay Number Rollover Mask \\[RNRM\\]
This bit, when set, masks the generation of error messages in response to a Replay Number Rollover event. STICKY."]
    #[inline(always)]
    pub fn rnrm(&self) -> RnrmR {
        RnrmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Reserved \\[R16\\]
Reserved"]
    #[inline(always)]
    pub fn r16(&self) -> R16R {
        R16R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Replay Timer Timeout Mask \\[RTTM\\]
This bit, when set, masks the generation of error messages in response to a Replay Timer timeout event. STICKY."]
    #[inline(always)]
    pub fn rttm(&self) -> RttmR {
        RttmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Advisory Non-Fatal Error Mask \\[ANFEM\\]
This bit, when set, masks the generation of error messages in response to an uncorrectable error occur, which is determined to belong to one of the special cases (as described in Section 6.2.3.2.4 of the PCI Express 2.0 Specifications). STICKY."]
    #[inline(always)]
    pub fn anfem(&self) -> AnfemR {
        AnfemR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Corrected Internal Error Mask \\[CIEM\\]
This bit, when set, masks the generation of error messages in response to a corrected internal error condition. STICKY."]
    #[inline(always)]
    pub fn ciem(&self) -> CiemR {
        CiemR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Header Log Overflow Mask \\[HLOM\\]
This bit, when set, masks the generation of error messages in response to a Header Log register overflow. STICKY."]
    #[inline(always)]
    pub fn hlom(&self) -> HlomR {
        HlomR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Reserved \\[R17\\]
Reserved"]
    #[inline(always)]
    pub fn r17(&self) -> R17R {
        R17R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Error Mask \\[REM\\]
This bit, when set, masks the generation of error messages in response to the Physical Layer errors STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn rem(&mut self) -> RemW<CorrectableErrorMaskSpec> {
        RemW::new(self, 0)
    }
    #[doc = "Bit 6 - Bad TLP Mask \\[BTM\\]
This bit, when set, masks the generation of error messages in response to a 'Bad TLP' received. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn btm(&mut self) -> BtmW<CorrectableErrorMaskSpec> {
        BtmW::new(self, 6)
    }
    #[doc = "Bit 7 - Bad DLLP Mask \\[BDM\\]
This bit, when set, masks the generation of error messages in response to a 'Bad DLLP' received. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn bdm(&mut self) -> BdmW<CorrectableErrorMaskSpec> {
        BdmW::new(self, 7)
    }
    #[doc = "Bit 8 - Replay Number Rollover Mask \\[RNRM\\]
This bit, when set, masks the generation of error messages in response to a Replay Number Rollover event. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn rnrm(&mut self) -> RnrmW<CorrectableErrorMaskSpec> {
        RnrmW::new(self, 8)
    }
    #[doc = "Bit 12 - Replay Timer Timeout Mask \\[RTTM\\]
This bit, when set, masks the generation of error messages in response to a Replay Timer timeout event. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn rttm(&mut self) -> RttmW<CorrectableErrorMaskSpec> {
        RttmW::new(self, 12)
    }
    #[doc = "Bit 13 - Advisory Non-Fatal Error Mask \\[ANFEM\\]
This bit, when set, masks the generation of error messages in response to an uncorrectable error occur, which is determined to belong to one of the special cases (as described in Section 6.2.3.2.4 of the PCI Express 2.0 Specifications). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn anfem(&mut self) -> AnfemW<CorrectableErrorMaskSpec> {
        AnfemW::new(self, 13)
    }
    #[doc = "Bit 14 - Corrected Internal Error Mask \\[CIEM\\]
This bit, when set, masks the generation of error messages in response to a corrected internal error condition. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ciem(&mut self) -> CiemW<CorrectableErrorMaskSpec> {
        CiemW::new(self, 14)
    }
    #[doc = "Bit 15 - Header Log Overflow Mask \\[HLOM\\]
This bit, when set, masks the generation of error messages in response to a Header Log register overflow. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn hlom(&mut self) -> HlomW<CorrectableErrorMaskSpec> {
        HlomW::new(self, 15)
    }
}
#[doc = "Correctable Error Mask Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`correctable_error_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`correctable_error_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CorrectableErrorMaskSpec;
impl crate::RegisterSpec for CorrectableErrorMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`correctable_error_mask::R`](R) reader structure"]
impl crate::Readable for CorrectableErrorMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`correctable_error_mask::W`](W) writer structure"]
impl crate::Writable for CorrectableErrorMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORRECTABLE_ERROR_MASK to value 0xe000"]
impl crate::Resettable for CorrectableErrorMaskSpec {
    const RESET_VALUE: u32 = 0xe000;
}
