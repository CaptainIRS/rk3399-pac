#[doc = "Register `MI_MP_CB_SIZE_SHD` reader"]
pub type R = crate::R<MiMpCbSizeShdSpec>;
#[doc = "Field `mp_cb_size` reader - Size of main picture Cb component ring buffer."]
pub type MpCbSizeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:27 - Size of main picture Cb component ring buffer."]
    #[inline(always)]
    pub fn mp_cb_size(&self) -> MpCbSizeR {
        MpCbSizeR::new((self.bits >> 3) & 0x01ff_ffff)
    }
}
#[doc = "Size shadow register of main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cb_size_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiMpCbSizeShdSpec;
impl crate::RegisterSpec for MiMpCbSizeShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_mp_cb_size_shd::R`](R) reader structure"]
impl crate::Readable for MiMpCbSizeShdSpec {}
#[doc = "`reset()` method sets MI_MP_CB_SIZE_SHD to value 0"]
impl crate::Resettable for MiMpCbSizeShdSpec {
    const RESET_VALUE: u32 = 0;
}
