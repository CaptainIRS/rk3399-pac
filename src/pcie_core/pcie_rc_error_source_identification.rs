#[doc = "Register `PCIE_RC_ERROR_SOURCE_IDENTIFICATION` reader"]
pub type R = crate::R<PcieRcErrorSourceIdentificationSpec>;
#[doc = "Field `ECSI` reader - Correctable Error Message Source ID \\[ECSI\\]\n\nThis field captures and stores the\n\nRequester ID from an ERR_COR\n\nmessage received by the RC, if the\n\nERR_COR bit was not set at the\n\ntime the message was received.\n\nSTICKY"]
pub type EcsiR = crate::FieldReader<u16>;
#[doc = "Field `EFNSI` reader - Fatal/Non- Fatal Error Message Source ID \\[EFNSI\\]\n\nThis field captures and stores the\n\nRequester ID from an ERR_FATAL or\n\nERROR_NONFATAL message\n\nreceived by the RC, if the\n\nERR_FATAL or NONFATAL Received\n\nbit was not set at the time the\n\nmessage was received. STICKY"]
pub type EfnsiR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Correctable Error Message Source ID \\[ECSI\\]\n\nThis field captures and stores the\n\nRequester ID from an ERR_COR\n\nmessage received by the RC, if the\n\nERR_COR bit was not set at the\n\ntime the message was received.\n\nSTICKY"]
    #[inline(always)]
    pub fn ecsi(&self) -> EcsiR {
        EcsiR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Fatal/Non- Fatal Error Message Source ID \\[EFNSI\\]\n\nThis field captures and stores the\n\nRequester ID from an ERR_FATAL or\n\nERROR_NONFATAL message\n\nreceived by the RC, if the\n\nERR_FATAL or NONFATAL Received\n\nbit was not set at the time the\n\nmessage was received. STICKY"]
    #[inline(always)]
    pub fn efnsi(&self) -> EfnsiR {
        EfnsiR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Error Source Identification Register\n\nThis field captures and stores the\n\nRequester ID from an ERR_FATAL or\n\nERROR_NONFATAL message\n\nreceived by the RC, if the\n\nERR_FATAL or NONFATAL Received\n\nbit was not set at the time the\n\nmessage was received. STICKY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_error_source_identification::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
