#[doc = "Register `CH1_DFI_RD_NUM` reader"]
pub type R = crate::R<Ch1DfiRdNumSpec>;
#[doc = "Field `CH1_DFI_RD_NUM` reader - DFI read command number in the statistics range of the channel 1"]
pub type Ch1DfiRdNumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DFI read command number in the statistics range of the channel 1"]
    #[inline(always)]
    pub fn ch1_dfi_rd_num(&self) -> Ch1DfiRdNumR {
        Ch1DfiRdNumR::new(self.bits)
    }
}
#[doc = "Channel 1 DFI read Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_dfi_rd_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1DfiRdNumSpec;
impl crate::RegisterSpec for Ch1DfiRdNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1_dfi_rd_num::R`](R) reader structure"]
impl crate::Readable for Ch1DfiRdNumSpec {}
#[doc = "`reset()` method sets CH1_DFI_RD_NUM to value 0"]
impl crate::Resettable for Ch1DfiRdNumSpec {
    const RESET_VALUE: u32 = 0;
}
