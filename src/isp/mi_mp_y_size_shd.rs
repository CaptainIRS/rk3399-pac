#[doc = "Register `MI_MP_Y_SIZE_SHD` reader"]
pub type R = crate::R<MiMpYSizeShdSpec>;
#[doc = "Field `mp_y_size` reader - Size of main picture Y component ring buffer, JPEG\n\nring buffer or raw data ring buffer.\n\n\n\n"]
pub type MpYSizeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:28 - Size of main picture Y component ring buffer, JPEG\n\nring buffer or raw data ring buffer.\n\n\n\n"]
    #[inline(always)]
    pub fn mp_y_size(&self) -> MpYSizeR {
        MpYSizeR::new((self.bits >> 3) & 0x03ff_ffff)
    }
}
#[doc = "Size shadow register of main picture Y component, JPEG \n\n\n\nor raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_y_size_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiMpYSizeShdSpec;
impl crate::RegisterSpec for MiMpYSizeShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_mp_y_size_shd::R`](R) reader structure"]
impl crate::Readable for MiMpYSizeShdSpec {}
#[doc = "`reset()` method sets MI_MP_Y_SIZE_SHD to value 0"]
impl crate::Resettable for MiMpYSizeShdSpec {
    const RESET_VALUE: u32 = 0;
}
