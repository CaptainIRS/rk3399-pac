#[doc = "Register `DDRMON_TOP_NUMBER` reader"]
pub type R = crate::R<DdrmonTopNumberSpec>;
#[doc = "Field `TOP_NUMBER` reader - The high threshold in the comparison of DDR access"]
pub type TopNumberR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The high threshold in the comparison of DDR access"]
    #[inline(always)]
    pub fn top_number(&self) -> TopNumberR {
        TopNumberR::new(self.bits)
    }
}
#[doc = "The High Threshold in the Comparison of DDR Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_top_number::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrmonTopNumberSpec;
impl crate::RegisterSpec for DdrmonTopNumberSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrmon_top_number::R`](R) reader structure"]
impl crate::Readable for DdrmonTopNumberSpec {}
#[doc = "`reset()` method sets DDRMON_TOP_NUMBER to value 0"]
impl crate::Resettable for DdrmonTopNumberSpec {
    const RESET_VALUE: u32 = 0;
}
