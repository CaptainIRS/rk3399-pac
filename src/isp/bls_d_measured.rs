#[doc = "Register `BLS_D_MEASURED` reader"]
pub type R = crate::R<BlsDMeasuredSpec>;
#[doc = "Field `BLS_D_MEASURED` reader - Measured black level for D pixels\n\n"]
pub type BlsDMeasuredR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Measured black level for D pixels\n\n"]
    #[inline(always)]
    pub fn bls_d_measured(&self) -> BlsDMeasuredR {
        BlsDMeasuredR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "measured black level D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_d_measured::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlsDMeasuredSpec;
impl crate::RegisterSpec for BlsDMeasuredSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bls_d_measured::R`](R) reader structure"]
impl crate::Readable for BlsDMeasuredSpec {}
#[doc = "`reset()` method sets BLS_D_MEASURED to value 0"]
impl crate::Resettable for BlsDMeasuredSpec {
    const RESET_VALUE: u32 = 0;
}
