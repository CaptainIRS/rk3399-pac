#[doc = "Register `MI_MP_Y_OFFS_CNT_SHD` reader"]
pub type R = crate::R<MiMpYOffsCntShdSpec>;
#[doc = "Field `mp_y_offs_cnt` reader - Current offset counter of main picture Y component,\n\nJPEG or raw data ring buffer for address generation\n\nNote: Soft reset will reset the contents to reset value."]
pub type MpYOffsCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:28 - Current offset counter of main picture Y component,\n\nJPEG or raw data ring buffer for address generation\n\nNote: Soft reset will reset the contents to reset value."]
    #[inline(always)]
    pub fn mp_y_offs_cnt(&self) -> MpYOffsCntR {
        MpYOffsCntR::new((self.bits >> 3) & 0x03ff_ffff)
    }
}
#[doc = "Current offset counter of main picture Y component, JPEG \n\n\n\nor raw data ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_y_offs_cnt_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiMpYOffsCntShdSpec;
impl crate::RegisterSpec for MiMpYOffsCntShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_mp_y_offs_cnt_shd::R`](R) reader structure"]
impl crate::Readable for MiMpYOffsCntShdSpec {}
#[doc = "`reset()` method sets MI_MP_Y_OFFS_CNT_SHD to value 0"]
impl crate::Resettable for MiMpYOffsCntShdSpec {
    const RESET_VALUE: u32 = 0;
}
