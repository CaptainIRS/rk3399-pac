#[doc = "Register `DDRMON_CH1_COUNT_NUM` reader"]
pub type R = crate::R<DdrmonCh1CountNumSpec>;
#[doc = "Field `CH1_DFI_COUNT_NUM` reader - Timer count number in the statistics range of the channel 1, the\n\nvalue should be divided by 2 as actual timer count."]
pub type Ch1DfiCountNumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timer count number in the statistics range of the channel 1, the\n\nvalue should be divided by 2 as actual timer count."]
    #[inline(always)]
    pub fn ch1_dfi_count_num(&self) -> Ch1DfiCountNumR {
        Ch1DfiCountNumR::new(self.bits)
    }
}
#[doc = "Channel 1 Timer Count Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch1_count_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrmonCh1CountNumSpec;
impl crate::RegisterSpec for DdrmonCh1CountNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrmon_ch1_count_num::R`](R) reader structure"]
impl crate::Readable for DdrmonCh1CountNumSpec {}
#[doc = "`reset()` method sets DDRMON_CH1_COUNT_NUM to value 0"]
impl crate::Resettable for DdrmonCh1CountNumSpec {
    const RESET_VALUE: u32 = 0;
}
