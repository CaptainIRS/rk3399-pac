#[doc = "Register `VF_TPH_STATUS` reader"]
pub type R = crate::R<VfTphStatusSpec>;
#[doc = "Field `VF_TPH_ST_MODE` reader - Virtual function TPH steering tag mode\n\nBits \\[2:0\\]
of this output reflect the setting of the ST Mode Select\n\nbits in the TPH Requester Control Register of Virtual Function 0.\n\nBits \\[5:3\\]
reflect the setting of the same register field of VF 1,\n\nand so on.\n\nThese bits are active only in the Endpoint mode. They indicate\n\nthe allowed modes for generation of TPH Hints by the\n\ncorresponding VF."]
pub type VfTphStModeR = crate::FieldReader<u32>;
#[doc = "Field `VF_TPH_REQR_EN` reader - Virtual function TPH requester enable\n\nEach of the 16 bits of this output is driven the TPH Requester\n\nEnable bit \\[8\\]
of the TPH Requester Control Register in the TPH\n\nRequester Capability Structure of the corresponding Virtual\n\nFunction.\n\nThese bits are active only in the Endpoint mode when SR-IOV is\n\nenabled. They indicate whether the software has enabled the\n\ndevice to generate requests with TPH Hints from the associated\n\nVirtual Function."]
pub type VfTphReqrEnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - Virtual function TPH steering tag mode\n\nBits \\[2:0\\]
of this output reflect the setting of the ST Mode Select\n\nbits in the TPH Requester Control Register of Virtual Function 0.\n\nBits \\[5:3\\]
reflect the setting of the same register field of VF 1,\n\nand so on.\n\nThese bits are active only in the Endpoint mode. They indicate\n\nthe allowed modes for generation of TPH Hints by the\n\ncorresponding VF."]
    #[inline(always)]
    pub fn vf_tph_st_mode(&self) -> VfTphStModeR {
        VfTphStModeR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - Virtual function TPH requester enable\n\nEach of the 16 bits of this output is driven the TPH Requester\n\nEnable bit \\[8\\]
of the TPH Requester Control Register in the TPH\n\nRequester Capability Structure of the corresponding Virtual\n\nFunction.\n\nThese bits are active only in the Endpoint mode when SR-IOV is\n\nenabled. They indicate whether the software has enabled the\n\ndevice to generate requests with TPH Hints from the associated\n\nVirtual Function."]
    #[inline(always)]
    pub fn vf_tph_reqr_en(&self) -> VfTphReqrEnR {
        VfTphReqrEnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Virtual function TPH status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vf_tph_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfTphStatusSpec;
impl crate::RegisterSpec for VfTphStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vf_tph_status::R`](R) reader structure"]
impl crate::Readable for VfTphStatusSpec {}
#[doc = "`reset()` method sets VF_TPH_STATUS to value 0"]
impl crate::Resettable for VfTphStatusSpec {
    const RESET_VALUE: u32 = 0;
}
