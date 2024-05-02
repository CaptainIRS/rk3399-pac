#[doc = "Register `BLS_A_MEASURED` reader"]
pub type R = crate::R<BlsAMeasuredSpec>;
#[doc = "Field `BLS_A_MEASURED` reader - Measured black level for A pixels\n\n"]
pub type BlsAMeasuredR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Measured black level for A pixels\n\n"]
    #[inline(always)]
    pub fn bls_a_measured(&self) -> BlsAMeasuredR {
        BlsAMeasuredR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "measured black level A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_a_measured::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlsAMeasuredSpec;
impl crate::RegisterSpec for BlsAMeasuredSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bls_a_measured::R`](R) reader structure"]
impl crate::Readable for BlsAMeasuredSpec {}
#[doc = "`reset()` method sets BLS_A_MEASURED to value 0"]
impl crate::Resettable for BlsAMeasuredSpec {
    const RESET_VALUE: u32 = 0;
}
