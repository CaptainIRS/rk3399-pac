#[doc = "Register `PCIE_PF_CORRECTABLE_ERROR_MASK` reader"]
pub type R = crate::R<PciePfCorrectableErrorMaskSpec>;
#[doc = "Register `PCIE_PF_CORRECTABLE_ERROR_MASK` writer"]
pub type W = crate::W<PciePfCorrectableErrorMaskSpec>;
#[doc = "Field `REM` reader - Receiver Error Mask \\[REM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to the Physical Layer errors\n\nSTICKY."]
pub type RemR = crate::BitReader;
#[doc = "Field `REM` writer - Receiver Error Mask \\[REM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to the Physical Layer errors\n\nSTICKY."]
pub type RemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R15` reader - Reserved \\[R15\\]\n\nReserved"]
pub type R15R = crate::FieldReader;
#[doc = "Field `BTM` reader - Bad TLP Mask \\[BTM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a 'Bad TLP' received.\n\nSTICKY."]
pub type BtmR = crate::BitReader;
#[doc = "Field `BTM` writer - Bad TLP Mask \\[BTM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a 'Bad TLP' received.\n\nSTICKY."]
pub type BtmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDM` reader - Bad DLLP Mask \\[BDM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a 'Bad DLLP' received.\n\nSTICKY."]
pub type BdmR = crate::BitReader;
#[doc = "Field `BDM` writer - Bad DLLP Mask \\[BDM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a 'Bad DLLP' received.\n\nSTICKY."]
pub type BdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNRM` reader - Replay Number Rollover Mask \\[RNRM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a Replay Number\n\nRollover event. STICKY."]
pub type RnrmR = crate::BitReader;
#[doc = "Field `RNRM` writer - Replay Number Rollover Mask \\[RNRM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a Replay Number\n\nRollover event. STICKY."]
pub type RnrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R16` reader - Reserved \\[R16\\]\n\nReserved"]
pub type R16R = crate::FieldReader;
#[doc = "Field `RTTM` reader - Replay Timer Timeout Mask \\[RTTM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a Replay Timer timeout\n\nevent. STICKY."]
pub type RttmR = crate::BitReader;
#[doc = "Field `RTTM` writer - Replay Timer Timeout Mask \\[RTTM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a Replay Timer timeout\n\nevent. STICKY."]
pub type RttmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANFEM` reader - Advisory Non-Fatal Error Mask \\[ANFEM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to an uncorrectable error\n\noccur, which is determined to belong\n\nto one of the special cases (as\n\ndescribed in Section 6.2.3.2.4 of the\n\nPCI Express 2.0 Specifications).\n\nSTICKY."]
pub type AnfemR = crate::BitReader;
#[doc = "Field `ANFEM` writer - Advisory Non-Fatal Error Mask \\[ANFEM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to an uncorrectable error\n\noccur, which is determined to belong\n\nto one of the special cases (as\n\ndescribed in Section 6.2.3.2.4 of the\n\nPCI Express 2.0 Specifications).\n\nSTICKY."]
pub type AnfemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIEM` reader - Corrected Internal Error Mask \\[CIEM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a corrected internal\n\nerror condition. STICKY."]
pub type CiemR = crate::BitReader;
#[doc = "Field `CIEM` writer - Corrected Internal Error Mask \\[CIEM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a corrected internal\n\nerror condition. STICKY."]
pub type CiemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HLOM` reader - Header Log Overflow Mask \\[HLOM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a Header Log register\n\noverflow. STICKY."]
pub type HlomR = crate::BitReader;
#[doc = "Field `HLOM` writer - Header Log Overflow Mask \\[HLOM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a Header Log register\n\noverflow. STICKY."]
pub type HlomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R17` reader - Reserved \\[R17\\]\n\nReserved"]
pub type R17R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Receiver Error Mask \\[REM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to the Physical Layer errors\n\nSTICKY."]
    #[inline(always)]
    pub fn rem(&self) -> RemR {
        RemR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Reserved \\[R15\\]\n\nReserved"]
    #[inline(always)]
    pub fn r15(&self) -> R15R {
        R15R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Bad TLP Mask \\[BTM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a 'Bad TLP' received.\n\nSTICKY."]
    #[inline(always)]
    pub fn btm(&self) -> BtmR {
        BtmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bad DLLP Mask \\[BDM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a 'Bad DLLP' received.\n\nSTICKY."]
    #[inline(always)]
    pub fn bdm(&self) -> BdmR {
        BdmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Replay Number Rollover Mask \\[RNRM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a Replay Number\n\nRollover event. STICKY."]
    #[inline(always)]
    pub fn rnrm(&self) -> RnrmR {
        RnrmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Reserved \\[R16\\]\n\nReserved"]
    #[inline(always)]
    pub fn r16(&self) -> R16R {
        R16R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Replay Timer Timeout Mask \\[RTTM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a Replay Timer timeout\n\nevent. STICKY."]
    #[inline(always)]
    pub fn rttm(&self) -> RttmR {
        RttmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Advisory Non-Fatal Error Mask \\[ANFEM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to an uncorrectable error\n\noccur, which is determined to belong\n\nto one of the special cases (as\n\ndescribed in Section 6.2.3.2.4 of the\n\nPCI Express 2.0 Specifications).\n\nSTICKY."]
    #[inline(always)]
    pub fn anfem(&self) -> AnfemR {
        AnfemR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Corrected Internal Error Mask \\[CIEM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a corrected internal\n\nerror condition. STICKY."]
    #[inline(always)]
    pub fn ciem(&self) -> CiemR {
        CiemR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Header Log Overflow Mask \\[HLOM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a Header Log register\n\noverflow. STICKY."]
    #[inline(always)]
    pub fn hlom(&self) -> HlomR {
        HlomR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Reserved \\[R17\\]\n\nReserved"]
    #[inline(always)]
    pub fn r17(&self) -> R17R {
        R17R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Error Mask \\[REM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to the Physical Layer errors\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn rem(&mut self) -> RemW<PciePfCorrectableErrorMaskSpec> {
        RemW::new(self, 0)
    }
    #[doc = "Bit 6 - Bad TLP Mask \\[BTM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a 'Bad TLP' received.\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn btm(&mut self) -> BtmW<PciePfCorrectableErrorMaskSpec> {
        BtmW::new(self, 6)
    }
    #[doc = "Bit 7 - Bad DLLP Mask \\[BDM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a 'Bad DLLP' received.\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn bdm(&mut self) -> BdmW<PciePfCorrectableErrorMaskSpec> {
        BdmW::new(self, 7)
    }
    #[doc = "Bit 8 - Replay Number Rollover Mask \\[RNRM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a Replay Number\n\nRollover event. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn rnrm(&mut self) -> RnrmW<PciePfCorrectableErrorMaskSpec> {
        RnrmW::new(self, 8)
    }
    #[doc = "Bit 12 - Replay Timer Timeout Mask \\[RTTM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a Replay Timer timeout\n\nevent. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn rttm(&mut self) -> RttmW<PciePfCorrectableErrorMaskSpec> {
        RttmW::new(self, 12)
    }
    #[doc = "Bit 13 - Advisory Non-Fatal Error Mask \\[ANFEM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to an uncorrectable error\n\noccur, which is determined to belong\n\nto one of the special cases (as\n\ndescribed in Section 6.2.3.2.4 of the\n\nPCI Express 2.0 Specifications).\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn anfem(&mut self) -> AnfemW<PciePfCorrectableErrorMaskSpec> {
        AnfemW::new(self, 13)
    }
    #[doc = "Bit 14 - Corrected Internal Error Mask \\[CIEM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a corrected internal\n\nerror condition. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ciem(&mut self) -> CiemW<PciePfCorrectableErrorMaskSpec> {
        CiemW::new(self, 14)
    }
    #[doc = "Bit 15 - Header Log Overflow Mask \\[HLOM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a Header Log register\n\noverflow. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn hlom(&mut self) -> HlomW<PciePfCorrectableErrorMaskSpec> {
        HlomW::new(self, 15)
    }
}
#[doc = "Correctable Error Mask Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_correctable_error_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_correctable_error_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfCorrectableErrorMaskSpec;
impl crate::RegisterSpec for PciePfCorrectableErrorMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_correctable_error_mask::R`](R) reader structure"]
impl crate::Readable for PciePfCorrectableErrorMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_correctable_error_mask::W`](W) writer structure"]
impl crate::Writable for PciePfCorrectableErrorMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_CORRECTABLE_ERROR_MASK to value 0xe000"]
impl crate::Resettable for PciePfCorrectableErrorMaskSpec {
    const RESET_VALUE: u32 = 0xe000;
}
