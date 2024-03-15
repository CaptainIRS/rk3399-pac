#[doc = "Register `PCIE_CLIENT_VF_TPH_STATUS` reader"]
pub type R = crate::R<PcieClientVfTphStatusSpec>;
#[doc = "Field `VF_TPH_ST_MODE` reader - Virtual function TPH steering tag mode Bits \\[2:0\\]
of this output reflect the setting of the ST Mode Select bits in the TPH Requester Control Register of Virtual Function 0. Bits \\[5:3\\]
reflect the setting of the same register field of VF 1, and so on. These bits are active only in the Endpoint mode. They indicate the allowed modes for generation of TPH Hints by the corresponding VF."]
pub type VfTphStModeR = crate::FieldReader<u32>;
#[doc = "Field `VF_TPH_REQR_EN` reader - Virtual function TPH requester enable Each of the 16 bits of this output is driven the TPH Requester Enable bit \\[8\\]
of the TPH Requester Control Register in the TPH Requester Capability Structure of the corresponding Virtual Function. These bits are active only in the Endpoint mode when SR-IOV is enabled. They indicate whether the software has enabled the device to generate requests with TPH Hints from the associated Virtual Function."]
pub type VfTphReqrEnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - Virtual function TPH steering tag mode Bits \\[2:0\\]
of this output reflect the setting of the ST Mode Select bits in the TPH Requester Control Register of Virtual Function 0. Bits \\[5:3\\]
reflect the setting of the same register field of VF 1, and so on. These bits are active only in the Endpoint mode. They indicate the allowed modes for generation of TPH Hints by the corresponding VF."]
    #[inline(always)]
    pub fn vf_tph_st_mode(&self) -> VfTphStModeR {
        VfTphStModeR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - Virtual function TPH requester enable Each of the 16 bits of this output is driven the TPH Requester Enable bit \\[8\\]
of the TPH Requester Control Register in the TPH Requester Capability Structure of the corresponding Virtual Function. These bits are active only in the Endpoint mode when SR-IOV is enabled. They indicate whether the software has enabled the device to generate requests with TPH Hints from the associated Virtual Function."]
    #[inline(always)]
    pub fn vf_tph_reqr_en(&self) -> VfTphReqrEnR {
        VfTphReqrEnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Virtual function TPH status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_vf_tph_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientVfTphStatusSpec;
impl crate::RegisterSpec for PcieClientVfTphStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_vf_tph_status::R`](R) reader structure"]
impl crate::Readable for PcieClientVfTphStatusSpec {}
#[doc = "`reset()` method sets PCIE_CLIENT_VF_TPH_STATUS to value 0"]
impl crate::Resettable for PcieClientVfTphStatusSpec {
    const RESET_VALUE: u32 = 0;
}
