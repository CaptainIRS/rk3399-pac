#[doc = "Register `BLS_C_MEASURED` reader"]
pub type R = crate::R<BlsCMeasuredSpec>;
#[doc = "Field `BLS_C_MEASURED` reader - Measured black level for C pixels\n\n"]
pub type BlsCMeasuredR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Measured black level for C pixels\n\n"]
    #[inline(always)]
    pub fn bls_c_measured(&self) -> BlsCMeasuredR {
        BlsCMeasuredR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "measured black level C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_c_measured::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlsCMeasuredSpec;
impl crate::RegisterSpec for BlsCMeasuredSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bls_c_measured::R`](R) reader structure"]
impl crate::Readable for BlsCMeasuredSpec {}
#[doc = "`reset()` method sets BLS_C_MEASURED to value 0"]
impl crate::Resettable for BlsCMeasuredSpec {
    const RESET_VALUE: u32 = 0;
}
