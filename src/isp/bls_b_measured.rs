#[doc = "Register `BLS_B_MEASURED` reader"]
pub type R = crate::R<BlsBMeasuredSpec>;
#[doc = "Field `BLS_B_MEASURED` reader - Measured black level for B pixels"]
pub type BlsBMeasuredR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Measured black level for B pixels"]
    #[inline(always)]
    pub fn bls_b_measured(&self) -> BlsBMeasuredR {
        BlsBMeasuredR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "measured black level B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_b_measured::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlsBMeasuredSpec;
impl crate::RegisterSpec for BlsBMeasuredSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bls_b_measured::R`](R) reader structure"]
impl crate::Readable for BlsBMeasuredSpec {}
#[doc = "`reset()` method sets BLS_B_MEASURED to value 0"]
impl crate::Resettable for BlsBMeasuredSpec {
    const RESET_VALUE: u32 = 0;
}
