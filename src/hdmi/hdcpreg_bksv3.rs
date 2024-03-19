#[doc = "Register `HDCPREG_BKSV3` reader"]
pub type R = crate::R<HdcpregBksv3Spec>;
#[doc = "Field `HDCPREG_BKSV3` reader - Contains the value of BKSV\\[31:24\\]."]
pub type HdcpregBksv3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Contains the value of BKSV\\[31:24\\]."]
    #[inline(always)]
    pub fn hdcpreg_bksv3(&self) -> HdcpregBksv3R {
        HdcpregBksv3R::new(self.bits)
    }
}
#[doc = "HDCP KSV Status Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_bksv3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregBksv3Spec;
impl crate::RegisterSpec for HdcpregBksv3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcpreg_bksv3::R`](R) reader structure"]
impl crate::Readable for HdcpregBksv3Spec {}
#[doc = "`reset()` method sets HDCPREG_BKSV3 to value 0"]
impl crate::Resettable for HdcpregBksv3Spec {
    const RESET_VALUE: u8 = 0;
}
