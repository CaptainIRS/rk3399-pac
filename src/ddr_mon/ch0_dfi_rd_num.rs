#[doc = "Register `CH0_DFI_RD_NUM` reader"]
pub type R = crate::R<Ch0DfiRdNumSpec>;
#[doc = "Field `CH0_DFI_RD_NUM` reader - DFI read command number in the statistics range of the channel 0"]
pub type Ch0DfiRdNumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DFI read command number in the statistics range of the channel 0"]
    #[inline(always)]
    pub fn ch0_dfi_rd_num(&self) -> Ch0DfiRdNumR {
        Ch0DfiRdNumR::new(self.bits)
    }
}
#[doc = "Channel 0 DFI read Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_dfi_rd_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0DfiRdNumSpec;
impl crate::RegisterSpec for Ch0DfiRdNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0_dfi_rd_num::R`](R) reader structure"]
impl crate::Readable for Ch0DfiRdNumSpec {}
#[doc = "`reset()` method sets CH0_DFI_RD_NUM to value 0"]
impl crate::Resettable for Ch0DfiRdNumSpec {
    const RESET_VALUE: u32 = 0;
}
