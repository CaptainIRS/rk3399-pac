#[doc = "Register `MI_SP_CB_SIZE_SHD` reader"]
pub type R = crate::R<MiSpCbSizeShdSpec>;
#[doc = "Field `sp_cb_size` reader - Size of self picture Cb component ring buffer."]
pub type SpCbSizeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:27 - Size of self picture Cb component ring buffer."]
    #[inline(always)]
    pub fn sp_cb_size(&self) -> SpCbSizeR {
        SpCbSizeR::new((self.bits >> 3) & 0x01ff_ffff)
    }
}
#[doc = "Size shadow register of self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cb_size_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiSpCbSizeShdSpec;
impl crate::RegisterSpec for MiSpCbSizeShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_sp_cb_size_shd::R`](R) reader structure"]
impl crate::Readable for MiSpCbSizeShdSpec {}
#[doc = "`reset()` method sets MI_SP_CB_SIZE_SHD to value 0"]
impl crate::Resettable for MiSpCbSizeShdSpec {
    const RESET_VALUE: u32 = 0;
}
