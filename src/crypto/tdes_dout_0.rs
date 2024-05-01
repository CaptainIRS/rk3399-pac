#[doc = "Register `TDES_DOUT_0` reader"]
pub type R = crate::R<TdesDout0Spec>;
#[doc = "Field `TDES_DOUT_0` reader - Specifies TDES Output data \\[63:32\\]."]
pub type TdesDout0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies TDES Output data \\[63:32\\]."]
    #[inline(always)]
    pub fn tdes_dout_0(&self) -> TdesDout0R {
        TdesDout0R::new(self.bits)
    }
}
#[doc = "TDES Output Data 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_dout_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdesDout0Spec;
impl crate::RegisterSpec for TdesDout0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdes_dout_0::R`](R) reader structure"]
impl crate::Readable for TdesDout0Spec {}
#[doc = "`reset()` method sets TDES_DOUT_0 to value 0"]
impl crate::Resettable for TdesDout0Spec {
    const RESET_VALUE: u32 = 0;
}
