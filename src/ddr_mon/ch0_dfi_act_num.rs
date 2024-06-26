#[doc = "Register `CH0_DFI_ACT_NUM` reader"]
pub type R = crate::R<Ch0DfiActNumSpec>;
#[doc = "Field `CH0_DFI_ACT_NUM` reader - DFI active command number in the statistics range of the channel 0"]
pub type Ch0DfiActNumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DFI active command number in the statistics range of the channel 0"]
    #[inline(always)]
    pub fn ch0_dfi_act_num(&self) -> Ch0DfiActNumR {
        Ch0DfiActNumR::new(self.bits)
    }
}
#[doc = "Channel 0 DFI Active Command Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0_dfi_act_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0DfiActNumSpec;
impl crate::RegisterSpec for Ch0DfiActNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0_dfi_act_num::R`](R) reader structure"]
impl crate::Readable for Ch0DfiActNumSpec {}
#[doc = "`reset()` method sets CH0_DFI_ACT_NUM to value 0"]
impl crate::Resettable for Ch0DfiActNumSpec {
    const RESET_VALUE: u32 = 0;
}
