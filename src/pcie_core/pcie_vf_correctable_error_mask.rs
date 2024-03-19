#[doc = "Register `PCIE_VF_CORRECTABLE_ERROR_MASK` reader"]
pub type R = crate::R<PcieVfCorrectableErrorMaskSpec>;
#[doc = "Field `REM` reader - Receiver Error Mask \\[REM\\]\n\nThis bit is not implemented for\n\nVirtual Functions. Hardwired to 0."]
pub type RemR = crate::BitReader;
#[doc = "Field `R15` reader - Reserved \\[R15\\]\n\nReserved"]
pub type R15R = crate::FieldReader;
#[doc = "Field `BTM` reader - Bad TLP Mask \\[BTM\\]\n\nThis bit is not implemented for\n\nVirtual Functions. Hardwired to 0."]
pub type BtmR = crate::BitReader;
#[doc = "Field `BDM` reader - Bad DLLP Mask \\[BDM\\]\n\nThis bit is not implemented for\n\nVirtual Functions. Hardwired to 0."]
pub type BdmR = crate::BitReader;
#[doc = "Field `RNRM` reader - Replay Number Rollover Mask \\[RNRM\\]\n\nThis bit is not implemented for\n\nVirtual Functions. Hardwired to 0."]
pub type RnrmR = crate::BitReader;
#[doc = "Field `R16` reader - Reserved \\[R16\\]\n\nReserved"]
pub type R16R = crate::FieldReader;
#[doc = "Field `RTTM` reader - Replay Timer Timeout Mask \\[RTTM\\]\n\nThis bit is not implemented for\n\nVirtual Functions. Hardwired to 0."]
pub type RttmR = crate::BitReader;
#[doc = "Field `ANFEM` reader - Advisory Non-Fatal Error Mask \\[ANFEM\\]\n\nThis bit is not implemented for\n\nVirtual Functions. Hardwired to 0."]
pub type AnfemR = crate::BitReader;
#[doc = "Field `CIEM` reader - Corrected Internal Error Mask \\[CIEM\\]\n\nThis bit is not implemented for\n\nVirtual Functions. Hardwired to 0."]
pub type CiemR = crate::BitReader;
#[doc = "Field `HLOM` reader - Header Log Overflow Mask \\[HLOM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a Header Log register\n\noverflow. STICKY. Header logs are\n\nshared across Vfs hence this field is\n\nreserved. This field is reserved since\n\nHeader log sharing is selected for\n\nthis configuration."]
pub type HlomR = crate::BitReader;
#[doc = "Field `R17` reader - Reserved \\[R17\\]\n\n(no description)"]
pub type R17R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Receiver Error Mask \\[REM\\]\n\nThis bit is not implemented for\n\nVirtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn rem(&self) -> RemR {
        RemR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Reserved \\[R15\\]\n\nReserved"]
    #[inline(always)]
    pub fn r15(&self) -> R15R {
        R15R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Bad TLP Mask \\[BTM\\]\n\nThis bit is not implemented for\n\nVirtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn btm(&self) -> BtmR {
        BtmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bad DLLP Mask \\[BDM\\]\n\nThis bit is not implemented for\n\nVirtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn bdm(&self) -> BdmR {
        BdmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Replay Number Rollover Mask \\[RNRM\\]\n\nThis bit is not implemented for\n\nVirtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn rnrm(&self) -> RnrmR {
        RnrmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Reserved \\[R16\\]\n\nReserved"]
    #[inline(always)]
    pub fn r16(&self) -> R16R {
        R16R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Replay Timer Timeout Mask \\[RTTM\\]\n\nThis bit is not implemented for\n\nVirtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn rttm(&self) -> RttmR {
        RttmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Advisory Non-Fatal Error Mask \\[ANFEM\\]\n\nThis bit is not implemented for\n\nVirtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn anfem(&self) -> AnfemR {
        AnfemR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Corrected Internal Error Mask \\[CIEM\\]\n\nThis bit is not implemented for\n\nVirtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn ciem(&self) -> CiemR {
        CiemR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Header Log Overflow Mask \\[HLOM\\]\n\nThis bit, when set, masks the\n\ngeneration of error messages in\n\nresponse to a Header Log register\n\noverflow. STICKY. Header logs are\n\nshared across Vfs hence this field is\n\nreserved. This field is reserved since\n\nHeader log sharing is selected for\n\nthis configuration."]
    #[inline(always)]
    pub fn hlom(&self) -> HlomR {
        HlomR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Reserved \\[R17\\]\n\n(no description)"]
    #[inline(always)]
    pub fn r17(&self) -> R17R {
        R17R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Correctable Error Mask Register\n\n(no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_correctable_error_mask::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfCorrectableErrorMaskSpec;
impl crate::RegisterSpec for PcieVfCorrectableErrorMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_correctable_error_mask::R`](R) reader structure"]
impl crate::Readable for PcieVfCorrectableErrorMaskSpec {}
#[doc = "`reset()` method sets PCIE_VF_CORRECTABLE_ERROR_MASK to value 0"]
impl crate::Resettable for PcieVfCorrectableErrorMaskSpec {
    const RESET_VALUE: u32 = 0;
}
