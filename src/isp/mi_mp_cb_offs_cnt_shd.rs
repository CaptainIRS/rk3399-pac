#[doc = "Register `MI_MP_CB_OFFS_CNT_SHD` reader"]
pub type R = crate::R<MiMpCbOffsCntShdSpec>;
#[doc = "Field `mp_cb_offs_cnt` reader - Current offset counter of main picture Cb component\n\nring buffer for address generation\n\nNote: Soft reset will reset the contents to reset value."]
pub type MpCbOffsCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:27 - Current offset counter of main picture Cb component\n\nring buffer for address generation\n\nNote: Soft reset will reset the contents to reset value."]
    #[inline(always)]
    pub fn mp_cb_offs_cnt(&self) -> MpCbOffsCntR {
        MpCbOffsCntR::new((self.bits >> 3) & 0x01ff_ffff)
    }
}
#[doc = "Current offset counter of main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cb_offs_cnt_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiMpCbOffsCntShdSpec;
impl crate::RegisterSpec for MiMpCbOffsCntShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_mp_cb_offs_cnt_shd::R`](R) reader structure"]
impl crate::Readable for MiMpCbOffsCntShdSpec {}
#[doc = "`reset()` method sets MI_MP_CB_OFFS_CNT_SHD to value 0"]
impl crate::Resettable for MiMpCbOffsCntShdSpec {
    const RESET_VALUE: u32 = 0;
}
