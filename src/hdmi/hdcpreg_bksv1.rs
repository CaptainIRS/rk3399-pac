#[doc = "Register `HDCPREG_BKSV1` reader"]
pub type R = crate::R<HdcpregBksv1Spec>;
#[doc = "Field `HDCPREG_BKSV1` reader - Description: Contains the value of BKSV\\[15:8\\]."]
pub type HdcpregBksv1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Description: Contains the value of BKSV\\[15:8\\]."]
    #[inline(always)]
    pub fn hdcpreg_bksv1(&self) -> HdcpregBksv1R {
        HdcpregBksv1R::new(self.bits)
    }
}
#[doc = "Description: Contains the value of BKSV\\[15:8\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_bksv1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregBksv1Spec;
impl crate::RegisterSpec for HdcpregBksv1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcpreg_bksv1::R`](R) reader structure"]
impl crate::Readable for HdcpregBksv1Spec {}
#[doc = "`reset()` method sets HDCPREG_BKSV1 to value 0"]
impl crate::Resettable for HdcpregBksv1Spec {
    const RESET_VALUE: u8 = 0;
}
