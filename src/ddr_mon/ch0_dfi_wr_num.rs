#[doc = "Register `CH0_DFI_WR_NUM` reader"]
pub type R = crate::R<Ch0DfiWrNumSpec>;
#[doc = "Field `CH0_DFI_WR_NUM` reader - DFI write command number in the statistics range of the channel 0"]
pub type Ch0DfiWrNumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DFI write command number in the statistics range of the channel 0"]
    #[inline(always)]
    pub fn ch0_dfi_wr_num(&self) -> Ch0DfiWrNumR {
        Ch0DfiWrNumR::new(self.bits)
    }
}
#[doc = "Channel 0 DFI write Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_dfi_wr_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0DfiWrNumSpec;
impl crate::RegisterSpec for Ch0DfiWrNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0_dfi_wr_num::R`](R) reader structure"]
impl crate::Readable for Ch0DfiWrNumSpec {}
#[doc = "`reset()` method sets CH0_DFI_WR_NUM to value 0"]
impl crate::Resettable for Ch0DfiWrNumSpec {
    const RESET_VALUE: u32 = 0;
}
