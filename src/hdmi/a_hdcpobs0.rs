#[doc = "Register `A_HDCPOBS0` reader"]
pub type R = crate::R<AHdcpobs0Spec>;
#[doc = "Field `HDCPENGAGED` reader - Informs that the current HDMI link has the HDCP\n\nprotocol fully engaged."]
pub type HdcpengagedR = crate::BitReader;
#[doc = "Field `SUBSTATEA` reader - Observability register informs in which sub-state\n\nthe authentication is on."]
pub type SubstateaR = crate::FieldReader;
#[doc = "Field `STATEA` reader - Observability register informs in which state the\n\nauthentication machine is on."]
pub type StateaR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Informs that the current HDMI link has the HDCP\n\nprotocol fully engaged."]
    #[inline(always)]
    pub fn hdcpengaged(&self) -> HdcpengagedR {
        HdcpengagedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Observability register informs in which sub-state\n\nthe authentication is on."]
    #[inline(always)]
    pub fn substatea(&self) -> SubstateaR {
        SubstateaR::new((self.bits >> 1) & 7)
    }
    #[doc = "Bits 4:7 - Observability register informs in which state the\n\nauthentication machine is on."]
    #[inline(always)]
    pub fn statea(&self) -> StateaR {
        StateaR::new((self.bits >> 4) & 0x0f)
    }
}
#[doc = "HDCP Observation Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_hdcpobs0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHdcpobs0Spec;
impl crate::RegisterSpec for AHdcpobs0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`a_hdcpobs0::R`](R) reader structure"]
impl crate::Readable for AHdcpobs0Spec {}
#[doc = "`reset()` method sets A_HDCPOBS0 to value 0"]
impl crate::Resettable for AHdcpobs0Spec {
    const RESET_VALUE: u8 = 0;
}
