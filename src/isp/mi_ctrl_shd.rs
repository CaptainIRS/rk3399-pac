#[doc = "Register `MI_CTRL_SHD` reader"]
pub type R = crate::R<MiCtrlShdSpec>;
#[doc = "Field `path_enable_in` reader - path_enable shadow register for module MI_IN\n\n(former raw_enable_in, jpeg_enable_in, sp_enable_in, mp_enable_in)"]
pub type PathEnableInR = crate::FieldReader;
#[doc = "Field `path_enable_out` reader - path_enable shadow register for module MI_OUT\n\n(former raw_enable_out, jpeg_enable_out, sp_enable_out, mp_enable_out)"]
pub type PathEnableOutR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - path_enable shadow register for module MI_IN\n\n(former raw_enable_in, jpeg_enable_in, sp_enable_in, mp_enable_in)"]
    #[inline(always)]
    pub fn path_enable_in(&self) -> PathEnableInR {
        PathEnableInR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - path_enable shadow register for module MI_OUT\n\n(former raw_enable_out, jpeg_enable_out, sp_enable_out, mp_enable_out)"]
    #[inline(always)]
    pub fn path_enable_out(&self) -> PathEnableOutR {
        PathEnableOutR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "global control internal shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_ctrl_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiCtrlShdSpec;
impl crate::RegisterSpec for MiCtrlShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_ctrl_shd::R`](R) reader structure"]
impl crate::Readable for MiCtrlShdSpec {}
#[doc = "`reset()` method sets MI_CTRL_SHD to value 0"]
impl crate::Resettable for MiCtrlShdSpec {
    const RESET_VALUE: u32 = 0;
}
