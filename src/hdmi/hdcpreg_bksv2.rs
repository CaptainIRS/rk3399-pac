#[doc = "Register `HDCPREG_BKSV2` reader"]
pub type R = crate::R<HdcpregBksv2Spec>;
#[doc = "Field `HDCPREG_BKSV2` reader - Contains the value of BKSV\\[23:16\\]."]
pub type HdcpregBksv2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Contains the value of BKSV\\[23:16\\]."]
    #[inline(always)]
    pub fn hdcpreg_bksv2(&self) -> HdcpregBksv2R {
        HdcpregBksv2R::new(self.bits)
    }
}
#[doc = "HDCP KSV Status Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_bksv2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregBksv2Spec;
impl crate::RegisterSpec for HdcpregBksv2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcpreg_bksv2::R`](R) reader structure"]
impl crate::Readable for HdcpregBksv2Spec {}
#[doc = "`reset()` method sets HDCPREG_BKSV2 to value 0"]
impl crate::Resettable for HdcpregBksv2Spec {
    const RESET_VALUE: u8 = 0;
}
