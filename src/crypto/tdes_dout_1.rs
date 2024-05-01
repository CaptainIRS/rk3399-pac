#[doc = "Register `TDES_DOUT_1` reader"]
pub type R = crate::R<TdesDout1Spec>;
#[doc = "Field `TDES_DOUT_1` reader - Specifies TDES Output data \\[31:0\\]."]
pub type TdesDout1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies TDES Output data \\[31:0\\]."]
    #[inline(always)]
    pub fn tdes_dout_1(&self) -> TdesDout1R {
        TdesDout1R::new(self.bits)
    }
}
#[doc = "TDES Output Data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_dout_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdesDout1Spec;
impl crate::RegisterSpec for TdesDout1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdes_dout_1::R`](R) reader structure"]
impl crate::Readable for TdesDout1Spec {}
#[doc = "`reset()` method sets TDES_DOUT_1 to value 0"]
impl crate::Resettable for TdesDout1Spec {
    const RESET_VALUE: u32 = 0;
}
