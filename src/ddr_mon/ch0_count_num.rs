#[doc = "Register `CH0_COUNT_NUM` reader"]
pub type R = crate::R<Ch0CountNumSpec>;
#[doc = "Field `CH0_DFI_COUNT_NUM` reader - Timer count number in the statistics range of the channel 0, the\n\nvalue should be divided by 2 as actual timer count."]
pub type Ch0DfiCountNumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timer count number in the statistics range of the channel 0, the\n\nvalue should be divided by 2 as actual timer count."]
    #[inline(always)]
    pub fn ch0_dfi_count_num(&self) -> Ch0DfiCountNumR {
        Ch0DfiCountNumR::new(self.bits)
    }
}
#[doc = "Channel 0 Timer Count Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_count_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0CountNumSpec;
impl crate::RegisterSpec for Ch0CountNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0_count_num::R`](R) reader structure"]
impl crate::Readable for Ch0CountNumSpec {}
#[doc = "`reset()` method sets CH0_COUNT_NUM to value 0"]
impl crate::Resettable for Ch0CountNumSpec {
    const RESET_VALUE: u32 = 0;
}
