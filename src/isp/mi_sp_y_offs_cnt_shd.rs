#[doc = "Register `MI_SP_Y_OFFS_CNT_SHD` reader"]
pub type R = crate::R<MiSpYOffsCntShdSpec>;
#[doc = "Field `sp_y_offs_cnt` reader - Current offset counter of self picture Y component\n\nring buffer for address generation\n\nNote: Soft reset will reset the contents to reset value."]
pub type SpYOffsCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:28 - Current offset counter of self picture Y component\n\nring buffer for address generation\n\nNote: Soft reset will reset the contents to reset value."]
    #[inline(always)]
    pub fn sp_y_offs_cnt(&self) -> SpYOffsCntR {
        SpYOffsCntR::new((self.bits >> 3) & 0x03ff_ffff)
    }
}
#[doc = "Current offset counter of self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_offs_cnt_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiSpYOffsCntShdSpec;
impl crate::RegisterSpec for MiSpYOffsCntShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_sp_y_offs_cnt_shd::R`](R) reader structure"]
impl crate::Readable for MiSpYOffsCntShdSpec {}
#[doc = "`reset()` method sets MI_SP_Y_OFFS_CNT_SHD to value 0"]
impl crate::Resettable for MiSpYOffsCntShdSpec {
    const RESET_VALUE: u32 = 0;
}
