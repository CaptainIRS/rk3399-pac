#[doc = "Register `PCIE_RC_ERROR_SOURCE_IDENTIFICATION` reader"]
pub type R = crate::R<PcieRcErrorSourceIdentificationSpec>;
#[doc = "Field `ECSI` reader - Correctable Error Message Source ID \\[ECSI\\]
This field captures and stores the Requester ID from an ERR_COR message received by the RC, if the ERR_COR bit was not set at the time the message was received. STICKY"]
pub type EcsiR = crate::FieldReader<u16>;
#[doc = "Field `EFNSI` reader - Fatal/Non- Fatal Error Message Source ID \\[EFNSI\\]
This field captures and stores the Requester ID from an ERR_FATAL or ERROR_NONFATAL message received by the RC, if the ERR_FATAL or NONFATAL Received bit was not set at the time the message was received. STICKY"]
pub type EfnsiR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Correctable Error Message Source ID \\[ECSI\\]
This field captures and stores the Requester ID from an ERR_COR message received by the RC, if the ERR_COR bit was not set at the time the message was received. STICKY"]
    #[inline(always)]
    pub fn ecsi(&self) -> EcsiR {
        EcsiR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Fatal/Non- Fatal Error Message Source ID \\[EFNSI\\]
This field captures and stores the Requester ID from an ERR_FATAL or ERROR_NONFATAL message received by the RC, if the ERR_FATAL or NONFATAL Received bit was not set at the time the message was received. STICKY"]
    #[inline(always)]
    pub fn efnsi(&self) -> EfnsiR {
        EfnsiR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Error Source Identification Register This field captures and stores the Requester ID from an ERR_FATAL or ERROR_NONFATAL message received by the RC, if the ERR_FATAL or NONFATAL Received bit was not set at the time the message was received. STICKY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_error_source_identification::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcErrorSourceIdentificationSpec;
impl crate::RegisterSpec for PcieRcErrorSourceIdentificationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_error_source_identification::R`](R) reader structure"]
impl crate::Readable for PcieRcErrorSourceIdentificationSpec {}
#[doc = "`reset()` method sets PCIE_RC_ERROR_SOURCE_IDENTIFICATION to value 0"]
impl crate::Resettable for PcieRcErrorSourceIdentificationSpec {
    const RESET_VALUE: u32 = 0;
}
