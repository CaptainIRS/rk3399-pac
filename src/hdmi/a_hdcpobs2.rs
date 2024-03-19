#[doc = "Register `A_HDCPOBS2` reader"]
pub type R = crate::R<AHdcpobs2Spec>;
#[doc = "Field `STATEEEG` reader - Observability register informs in which state the\n\nEESS machine is on."]
pub type StateeegR = crate::FieldReader;
#[doc = "Field `STATEE` reader - Observability register informs in which state the\n\ncipher machine is on."]
pub type StateeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Observability register informs in which state the\n\nEESS machine is on."]
    #[inline(always)]
    pub fn stateeeg(&self) -> StateeegR {
        StateeegR::new(self.bits & 7)
    }
    #[doc = "Bits 3:5 - Observability register informs in which state the\n\ncipher machine is on."]
    #[inline(always)]
    pub fn statee(&self) -> StateeR {
        StateeR::new((self.bits >> 3) & 7)
    }
}
#[doc = "HDCP Observation Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_hdcpobs2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHdcpobs2Spec;
impl crate::RegisterSpec for AHdcpobs2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`a_hdcpobs2::R`](R) reader structure"]
impl crate::Readable for AHdcpobs2Spec {}
#[doc = "`reset()` method sets A_HDCPOBS2 to value 0"]
impl crate::Resettable for AHdcpobs2Spec {
    const RESET_VALUE: u8 = 0;
}
