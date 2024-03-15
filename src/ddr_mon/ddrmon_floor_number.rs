#[doc = "Register `DDRMON_FLOOR_NUMBER` reader"]
pub type R = crate::R<DdrmonFloorNumberSpec>;
#[doc = "Field `FLOOR_NUMBER` reader - The low threshold in the comparison of DDR access"]
pub type FloorNumberR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The low threshold in the comparison of DDR access"]
    #[inline(always)]
    pub fn floor_number(&self) -> FloorNumberR {
        FloorNumberR::new(self.bits)
    }
}
#[doc = "The Low Threshold in the Comparison of DDR Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_floor_number::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrmonFloorNumberSpec;
impl crate::RegisterSpec for DdrmonFloorNumberSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrmon_floor_number::R`](R) reader structure"]
impl crate::Readable for DdrmonFloorNumberSpec {}
#[doc = "`reset()` method sets DDRMON_FLOOR_NUMBER to value 0"]
impl crate::Resettable for DdrmonFloorNumberSpec {
    const RESET_VALUE: u32 = 0;
}
