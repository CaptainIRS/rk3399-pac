#[doc = "Register `A_HDCPOBS1` reader"]
pub type R = crate::R<AHdcpobs1Spec>;
#[doc = "Field `STATER` reader - Observability register informs in which state the revocation machine is on."]
pub type StaterR = crate::FieldReader;
#[doc = "Field `STATEOEG` reader - Observability register informs in which state the OESS machine is on."]
pub type StateoegR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Observability register informs in which state the revocation machine is on."]
    #[inline(always)]
    pub fn stater(&self) -> StaterR {
        StaterR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - Observability register informs in which state the OESS machine is on."]
    #[inline(always)]
    pub fn stateoeg(&self) -> StateoegR {
        StateoegR::new((self.bits >> 4) & 7)
    }
}
#[doc = "Observability register informs in which state the revocation machine is on.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_hdcpobs1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHdcpobs1Spec;
impl crate::RegisterSpec for AHdcpobs1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`a_hdcpobs1::R`](R) reader structure"]
impl crate::Readable for AHdcpobs1Spec {}
#[doc = "`reset()` method sets A_HDCPOBS1 to value 0"]
impl crate::Resettable for AHdcpobs1Spec {
    const RESET_VALUE: u8 = 0;
}
