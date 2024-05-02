#[doc = "Register `MI_SP_CB_OFFS_CNT_SHD` reader"]
pub type R = crate::R<MiSpCbOffsCntShdSpec>;
#[doc = "Field `sp_cb_offs_cnt` reader - Current offset counter of self picture Cb component\n\nring buffer for address generation\n\nNote: Soft reset will reset the contents to reset value."]
pub type SpCbOffsCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:27 - Current offset counter of self picture Cb component\n\nring buffer for address generation\n\nNote: Soft reset will reset the contents to reset value."]
    #[inline(always)]
    pub fn sp_cb_offs_cnt(&self) -> SpCbOffsCntR {
        SpCbOffsCntR::new((self.bits >> 3) & 0x01ff_ffff)
    }
}
#[doc = "Current offset counter of self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cb_offs_cnt_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiSpCbOffsCntShdSpec;
impl crate::RegisterSpec for MiSpCbOffsCntShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_sp_cb_offs_cnt_shd::R`](R) reader structure"]
impl crate::Readable for MiSpCbOffsCntShdSpec {}
#[doc = "`reset()` method sets MI_SP_CB_OFFS_CNT_SHD to value 0"]
impl crate::Resettable for MiSpCbOffsCntShdSpec {
    const RESET_VALUE: u32 = 0;
}
