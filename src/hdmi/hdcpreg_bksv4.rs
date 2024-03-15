#[doc = "Register `HDCPREG_BKSV4` reader"]
pub type R = crate::R<HdcpregBksv4Spec>;
#[doc = "Field `HDCPREG_BKSV4` reader - Contains the value of BKSV\\[39:32\\]."]
pub type HdcpregBksv4R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Contains the value of BKSV\\[39:32\\]."]
    #[inline(always)]
    pub fn hdcpreg_bksv4(&self) -> HdcpregBksv4R {
        HdcpregBksv4R::new(self.bits)
    }
}
#[doc = "Contains the value of BKSV\\[39:32\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_bksv4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregBksv4Spec;
impl crate::RegisterSpec for HdcpregBksv4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcpreg_bksv4::R`](R) reader structure"]
impl crate::Readable for HdcpregBksv4Spec {}
#[doc = "`reset()` method sets HDCPREG_BKSV4 to value 0"]
impl crate::Resettable for HdcpregBksv4Spec {
    const RESET_VALUE: u8 = 0;
}
