#[doc = "Register `ERRLOG_ErrLog6` reader"]
pub type R = crate::R<ErrlogErrLog6Spec>;
#[doc = "Field `REQUSERH` reader - High 8 LSB AXI user bits for cci_m0 and pcie master. It is read as 0 for the other master.Unused bits are read as 0."]
pub type RequserhR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - High 8 LSB AXI user bits for cci_m0 and pcie master. It is read as 0 for the other master.Unused bits are read as 0."]
    #[inline(always)]
    pub fn requserh(&self) -> RequserhR {
        RequserhR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "MSB user bits in transport protocol header\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errlog_err_log6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrlogErrLog6Spec;
impl crate::RegisterSpec for ErrlogErrLog6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errlog_err_log6::R`](R) reader structure"]
impl crate::Readable for ErrlogErrLog6Spec {}
#[doc = "`reset()` method sets ERRLOG_ErrLog6 to value 0"]
impl crate::Resettable for ErrlogErrLog6Spec {
    const RESET_VALUE: u32 = 0;
}
