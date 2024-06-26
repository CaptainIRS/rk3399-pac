#[doc = "Register `CH1_DFI_ACCESS_NUM` reader"]
pub type R = crate::R<Ch1DfiAccessNumSpec>;
#[doc = "Field `CH1_DFI_ACCESS_NUM` reader - DFI read and write command number in the statistics range of the\n\nchannel 1"]
pub type Ch1DfiAccessNumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DFI read and write command number in the statistics range of the\n\nchannel 1"]
    #[inline(always)]
    pub fn ch1_dfi_access_num(&self) -> Ch1DfiAccessNumR {
        Ch1DfiAccessNumR::new(self.bits)
    }
}
#[doc = "Channel 1 DFI Read and Write Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_dfi_access_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1DfiAccessNumSpec;
impl crate::RegisterSpec for Ch1DfiAccessNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1_dfi_access_num::R`](R) reader structure"]
impl crate::Readable for Ch1DfiAccessNumSpec {}
#[doc = "`reset()` method sets CH1_DFI_ACCESS_NUM to value 0"]
impl crate::Resettable for Ch1DfiAccessNumSpec {
    const RESET_VALUE: u32 = 0;
}
