#[doc = "Register `CORRECTABLE_ERROR_MASK` reader"]
pub type R = crate::R<CorrectableErrorMaskSpec>;
#[doc = "Field `REM` reader - Receiver Error Mask \\[REM\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type RemR = crate::BitReader;
#[doc = "Field `R15` reader - Reserved \\[R15\\]
Reserved"]
pub type R15R = crate::FieldReader;
#[doc = "Field `BTM` reader - Bad TLP Mask \\[BTM\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type BtmR = crate::BitReader;
#[doc = "Field `BDM` reader - Bad DLLP Mask \\[BDM\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type BdmR = crate::BitReader;
#[doc = "Field `RNRM` reader - Replay Number Rollover Mask \\[RNRM\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type RnrmR = crate::BitReader;
#[doc = "Field `R16` reader - Reserved \\[R16\\]
Reserved"]
pub type R16R = crate::FieldReader;
#[doc = "Field `RTTM` reader - Replay Timer Timeout Mask \\[RTTM\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type RttmR = crate::BitReader;
#[doc = "Field `ANFEM` reader - Advisory Non-Fatal Error Mask \\[ANFEM\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type AnfemR = crate::BitReader;
#[doc = "Field `CIEM` reader - Corrected Internal Error Mask \\[CIEM\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type CiemR = crate::BitReader;
#[doc = "Field `HLOM` reader - Header Log Overflow Mask \\[HLOM\\]
This bit, when set, masks the generation of error messages in response to a Header Log register overflow. STICKY. Header logs are shared across Vfs hence this field is reserved. This field is reserved since Header log sharing is selected for this configuration."]
pub type HlomR = crate::BitReader;
#[doc = "Field `R17` reader - Reserved \\[R17\\]
(no description)"]
pub type R17R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Receiver Error Mask \\[REM\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
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
This bit is not implemented for Virtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn btm(&self) -> BtmR {
        BtmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bad DLLP Mask \\[BDM\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn bdm(&self) -> BdmR {
        BdmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Replay Number Rollover Mask \\[RNRM\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
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
This bit is not implemented for Virtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn rttm(&self) -> RttmR {
        RttmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Advisory Non-Fatal Error Mask \\[ANFEM\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn anfem(&self) -> AnfemR {
        AnfemR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Corrected Internal Error Mask \\[CIEM\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn ciem(&self) -> CiemR {
        CiemR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Header Log Overflow Mask \\[HLOM\\]
This bit, when set, masks the generation of error messages in response to a Header Log register overflow. STICKY. Header logs are shared across Vfs hence this field is reserved. This field is reserved since Header log sharing is selected for this configuration."]
    #[inline(always)]
    pub fn hlom(&self) -> HlomR {
        HlomR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Reserved \\[R17\\]
(no description)"]
    #[inline(always)]
    pub fn r17(&self) -> R17R {
        R17R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Correctable Error Mask Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`correctable_error_mask::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CorrectableErrorMaskSpec;
impl crate::RegisterSpec for CorrectableErrorMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`correctable_error_mask::R`](R) reader structure"]
impl crate::Readable for CorrectableErrorMaskSpec {}
#[doc = "`reset()` method sets CORRECTABLE_ERROR_MASK to value 0"]
impl crate::Resettable for CorrectableErrorMaskSpec {
    const RESET_VALUE: u32 = 0;
}
