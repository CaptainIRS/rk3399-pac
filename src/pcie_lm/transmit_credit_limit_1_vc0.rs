#[doc = "Register `TRANSMIT_CREDIT_LIMIT_1_VC0` reader"]
pub type R = crate::R<TransmitCreditLimit1Vc0Spec>;
#[doc = "Field `NPHC` reader - Non- Posted Header Credit VC0 \\[NPHC\\]
Non-Posted header credit limit received by the core for VC 0 (in number of packets)."]
pub type NphcR = crate::FieldReader;
#[doc = "Field `CPC` reader - Completion Payload Credit VC0 \\[CPC\\]
Completion payload credit limit received by the core for VC 0 (in units of 4 Dwords)."]
pub type CpcR = crate::FieldReader<u16>;
#[doc = "Field `R3` reader - Reserved \\[R3\\]
Reserved"]
pub type R3R = crate::FieldReader;
#[doc = "Field `CHC` reader - Completion Header Credit VC0 \\[CHC\\]
Completion header credit limit received by the core for VC 0 (in number of packets)."]
pub type ChcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Non- Posted Header Credit VC0 \\[NPHC\\]
Non-Posted header credit limit received by the core for VC 0 (in number of packets)."]
    #[inline(always)]
    pub fn nphc(&self) -> NphcR {
        NphcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:19 - Completion Payload Credit VC0 \\[CPC\\]
Completion payload credit limit received by the core for VC 0 (in units of 4 Dwords)."]
    #[inline(always)]
    pub fn cpc(&self) -> CpcR {
        CpcR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:23 - Reserved \\[R3\\]
Reserved"]
    #[inline(always)]
    pub fn r3(&self) -> R3R {
        R3R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Completion Header Credit VC0 \\[CHC\\]
Completion header credit limit received by the core for VC 0 (in number of packets)."]
    #[inline(always)]
    pub fn chc(&self) -> ChcR {
        ChcR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Transmit Credit Limit Register 1 VC0 Completion header credit limit received by the core for VC 0 (in number of packets).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_credit_limit_1_vc0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TransmitCreditLimit1Vc0Spec;
impl crate::RegisterSpec for TransmitCreditLimit1Vc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`transmit_credit_limit_1_vc0::R`](R) reader structure"]
impl crate::Readable for TransmitCreditLimit1Vc0Spec {}
#[doc = "`reset()` method sets TRANSMIT_CREDIT_LIMIT_1_VC0 to value 0"]
impl crate::Resettable for TransmitCreditLimit1Vc0Spec {
    const RESET_VALUE: u32 = 0;
}
