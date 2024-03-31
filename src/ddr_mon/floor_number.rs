#[doc = "Register `FLOOR_NUMBER` reader"]
pub type R = crate::R<FloorNumberSpec>;
#[doc = "Field `FLOOR_NUMBER` reader - The low threshold in the comparison of DDR access"]
pub type FloorNumberR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The low threshold in the comparison of DDR access"]
    #[inline(always)]
    pub fn floor_number(&self) -> FloorNumberR {
        FloorNumberR::new(self.bits)
    }
}
#[doc = "The Low Threshold in the Comparison of DDR Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`floor_number::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FloorNumberSpec;
impl crate::RegisterSpec for FloorNumberSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`floor_number::R`](R) reader structure"]
impl crate::Readable for FloorNumberSpec {}
#[doc = "`reset()` method sets FLOOR_NUMBER to value 0"]
impl crate::Resettable for FloorNumberSpec {
    const RESET_VALUE: u32 = 0;
}
