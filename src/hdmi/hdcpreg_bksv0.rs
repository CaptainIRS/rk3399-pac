#[doc = "Register `HDCPREG_BKSV0` reader"]
pub type R = crate::R<HdcpregBksv0Spec>;
#[doc = "Field `HDCPREG_BKSV0` reader - Contains the value of BKSV\\[7:0\\]."]
pub type HdcpregBksv0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Contains the value of BKSV\\[7:0\\]."]
    #[inline(always)]
    pub fn hdcpreg_bksv0(&self) -> HdcpregBksv0R {
        HdcpregBksv0R::new(self.bits)
    }
}
#[doc = "Contains the value of BKSV\\[7:0\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_bksv0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregBksv0Spec;
impl crate::RegisterSpec for HdcpregBksv0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcpreg_bksv0::R`](R) reader structure"]
impl crate::Readable for HdcpregBksv0Spec {}
#[doc = "`reset()` method sets HDCPREG_BKSV0 to value 0"]
impl crate::Resettable for HdcpregBksv0Spec {
    const RESET_VALUE: u8 = 0;
}
