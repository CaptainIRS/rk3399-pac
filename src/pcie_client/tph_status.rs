#[doc = "Register `TPH_STATUS` reader"]
pub type R = crate::R<TphStatusSpec>;
#[doc = "Field `TPH_ST_MODE` reader - Physical function TPH steering tag mode\n\nBits \\[2:0\\]
of this output reflect the setting of the ST Mode Select\n\nbits in the TPH Requester Control Register of Physical Function 0.\n\nThese bits are active only in the Endpoint mode. They indicate\n\nthe allowed modes for generation of TPH Hints by the\n\ncorresponding Physical Function."]
pub type TphStModeR = crate::FieldReader;
#[doc = "Field `TPH_REQR_EN` reader - Physical function TPH requester enable\n\nBit 0 of this output is drives the TPH Requester Enable bit \\[8\\]
of\n\nthe TPH Requester Control Register in the TPH Requester\n\nCapability Structure of the Physical Function 0.\n\nThese bits are active only in the Endpoint mode.\n\nThey indicate whether the software has enabled the device to\n\ngenerate requests with TPH Hints from the associated Physical\n\nFunction."]
pub type TphReqrEnR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Physical function TPH steering tag mode\n\nBits \\[2:0\\]
of this output reflect the setting of the ST Mode Select\n\nbits in the TPH Requester Control Register of Physical Function 0.\n\nThese bits are active only in the Endpoint mode. They indicate\n\nthe allowed modes for generation of TPH Hints by the\n\ncorresponding Physical Function."]
    #[inline(always)]
    pub fn tph_st_mode(&self) -> TphStModeR {
        TphStModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Physical function TPH requester enable\n\nBit 0 of this output is drives the TPH Requester Enable bit \\[8\\]
of\n\nthe TPH Requester Control Register in the TPH Requester\n\nCapability Structure of the Physical Function 0.\n\nThese bits are active only in the Endpoint mode.\n\nThey indicate whether the software has enabled the device to\n\ngenerate requests with TPH Hints from the associated Physical\n\nFunction."]
    #[inline(always)]
    pub fn tph_reqr_en(&self) -> TphReqrEnR {
        TphReqrEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Physical TPH status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tph_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TphStatusSpec;
impl crate::RegisterSpec for TphStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tph_status::R`](R) reader structure"]
impl crate::Readable for TphStatusSpec {}
#[doc = "`reset()` method sets TPH_STATUS to value 0"]
impl crate::Resettable for TphStatusSpec {
    const RESET_VALUE: u32 = 0;
}
