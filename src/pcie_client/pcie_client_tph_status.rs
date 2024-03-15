#[doc = "Register `PCIE_CLIENT_TPH_STATUS` reader"]
pub type R = crate::R<PcieClientTphStatusSpec>;
#[doc = "Field `TPH_ST_MODE` reader - Physical function TPH steering tag mode Bits \\[2:0\\]
of this output reflect the setting of the ST Mode Select bits in the TPH Requester Control Register of Physical Function 0. These bits are active only in the Endpoint mode. They indicate the allowed modes for generation of TPH Hints by the corresponding Physical Function."]
pub type TphStModeR = crate::FieldReader;
#[doc = "Field `TPH_REQR_EN` reader - Physical function TPH requester enable Bit 0 of this output is drives the TPH Requester Enable bit \\[8\\]
of the TPH Requester Control Register in the TPH Requester Capability Structure of the Physical Function 0. These bits are active only in the Endpoint mode. They indicate whether the software has enabled the device to generate requests with TPH Hints from the associated Physical Function."]
pub type TphReqrEnR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Physical function TPH steering tag mode Bits \\[2:0\\]
of this output reflect the setting of the ST Mode Select bits in the TPH Requester Control Register of Physical Function 0. These bits are active only in the Endpoint mode. They indicate the allowed modes for generation of TPH Hints by the corresponding Physical Function."]
    #[inline(always)]
    pub fn tph_st_mode(&self) -> TphStModeR {
        TphStModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Physical function TPH requester enable Bit 0 of this output is drives the TPH Requester Enable bit \\[8\\]
of the TPH Requester Control Register in the TPH Requester Capability Structure of the Physical Function 0. These bits are active only in the Endpoint mode. They indicate whether the software has enabled the device to generate requests with TPH Hints from the associated Physical Function."]
    #[inline(always)]
    pub fn tph_reqr_en(&self) -> TphReqrEnR {
        TphReqrEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Physical TPH status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_tph_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientTphStatusSpec;
impl crate::RegisterSpec for PcieClientTphStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_tph_status::R`](R) reader structure"]
impl crate::Readable for PcieClientTphStatusSpec {}
#[doc = "`reset()` method sets PCIE_CLIENT_TPH_STATUS to value 0"]
impl crate::Resettable for PcieClientTphStatusSpec {
    const RESET_VALUE: u32 = 0;
}
