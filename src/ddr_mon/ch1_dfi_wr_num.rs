#[doc = "Register `CH1_DFI_WR_NUM` reader"]
pub type R = crate::R<Ch1DfiWrNumSpec>;
#[doc = "Field `CH1_DFI_WR_NUM` reader - DFI write command number in the statistics range of the channel 1"]
pub type Ch1DfiWrNumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DFI write command number in the statistics range of the channel 1"]
    #[inline(always)]
    pub fn ch1_dfi_wr_num(&self) -> Ch1DfiWrNumR {
        Ch1DfiWrNumR::new(self.bits)
    }
}
#[doc = "Channel 1 DFI write Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_dfi_wr_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1DfiWrNumSpec;
impl crate::RegisterSpec for Ch1DfiWrNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1_dfi_wr_num::R`](R) reader structure"]
impl crate::Readable for Ch1DfiWrNumSpec {}
#[doc = "`reset()` method sets CH1_DFI_WR_NUM to value 0"]
impl crate::Resettable for Ch1DfiWrNumSpec {
    const RESET_VALUE: u32 = 0;
}
