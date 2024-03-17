#[doc = "Register `DENALI_CTL_73` reader"]
pub type R = crate::R<DenaliCtl73Spec>;
#[doc = "Field `ZQ_CALSTART_STATUS` reader - Holds the status associated with the ZQ calibration interrupt. Bit (0) indicates that the ZQ cal start timer was exceeded and bit (1) indicates a ZQ command was received when memory was in the self-refresh state and the command was terminated without execution."]
pub type ZqCalstartStatusR = crate::FieldReader;
#[doc = "Field `ZQ_CALLATCH_STATUS` reader - Holds the status associated with the ZQ calibration interrupt. Bit (0) indicates that the ZQ cal latch timer was exceeded and bit (1) indicates a ZQ command was received when memory was in the self-refresh state and the command was terminated without execution."]
pub type ZqCallatchStatusR = crate::FieldReader;
#[doc = "Field `ZQ_CALINIT_CS_CL_STATUS` reader - Holds the status associated with the ZQ calibration interrupt. Bit (0) indicates that the ZQCS timer was exceeded and bit (1) indicates a ZQ command was received when memory was in the self-refresh state and the command was terminated without execution."]
pub type ZqCalinitCsClStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Holds the status associated with the ZQ calibration interrupt. Bit (0) indicates that the ZQ cal start timer was exceeded and bit (1) indicates a ZQ command was received when memory was in the self-refresh state and the command was terminated without execution."]
    #[inline(always)]
    pub fn zq_calstart_status(&self) -> ZqCalstartStatusR {
        ZqCalstartStatusR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Holds the status associated with the ZQ calibration interrupt. Bit (0) indicates that the ZQ cal latch timer was exceeded and bit (1) indicates a ZQ command was received when memory was in the self-refresh state and the command was terminated without execution."]
    #[inline(always)]
    pub fn zq_callatch_status(&self) -> ZqCallatchStatusR {
        ZqCallatchStatusR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Holds the status associated with the ZQ calibration interrupt. Bit (0) indicates that the ZQCS timer was exceeded and bit (1) indicates a ZQ command was received when memory was in the self-refresh state and the command was terminated without execution."]
    #[inline(always)]
    pub fn zq_calinit_cs_cl_status(&self) -> ZqCalinitCsClStatusR {
        ZqCalinitCsClStatusR::new(((self.bits >> 16) & 3) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_73::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl73Spec;
impl crate::RegisterSpec for DenaliCtl73Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_73::R`](R) reader structure"]
impl crate::Readable for DenaliCtl73Spec {}
#[doc = "`reset()` method sets DENALI_CTL_73 to value 0"]
impl crate::Resettable for DenaliCtl73Spec {
    const RESET_VALUE: u32 = 0;
}
