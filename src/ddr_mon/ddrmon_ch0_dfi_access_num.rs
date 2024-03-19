#[doc = "Register `DDRMON_CH0_DFI_ACCESS_NUM` reader"]
pub type R = crate::R<DdrmonCh0DfiAccessNumSpec>;
#[doc = "Field `CH0_DFI_ACCESS_NUM` reader - DFI read and write command number in the statistics range of the\n\nchannel 0"]
pub type Ch0DfiAccessNumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DFI read and write command number in the statistics range of the\n\nchannel 0"]
    #[inline(always)]
    pub fn ch0_dfi_access_num(&self) -> Ch0DfiAccessNumR {
        Ch0DfiAccessNumR::new(self.bits)
    }
}
#[doc = "Channel 0 DFI Read and Write Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch0_dfi_access_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrmonCh0DfiAccessNumSpec;
impl crate::RegisterSpec for DdrmonCh0DfiAccessNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrmon_ch0_dfi_access_num::R`](R) reader structure"]
impl crate::Readable for DdrmonCh0DfiAccessNumSpec {}
#[doc = "`reset()` method sets DDRMON_CH0_DFI_ACCESS_NUM to value 0"]
impl crate::Resettable for DdrmonCh0DfiAccessNumSpec {
    const RESET_VALUE: u32 = 0;
}
