#[doc = "Register `GRF_GPU_PERF_CON2` reader"]
pub type R = crate::R<GrfGpuPerfCon2Spec>;
#[doc = "Register `GRF_GPU_PERF_CON2` writer"]
pub type W = crate::W<GrfGpuPerfCon2Spec>;
#[doc = "Field `GPU_SW_AR_COUNT_ID` reader - When sw_ar_cnt_id_type=1, only count\n\nthe id designated by sw_ar_count_id"]
pub type GpuSwArCountIdR = crate::FieldReader;
#[doc = "Field `GPU_SW_AR_COUNT_ID` writer - When sw_ar_cnt_id_type=1, only count\n\nthe id designated by sw_ar_count_id"]
pub type GpuSwArCountIdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `GPU_SW_AW_COUNT_ID` reader - When sw_aw_cnt_id_type=1, only count\n\nthe id designated by sw_aw_count_id"]
pub type GpuSwAwCountIdR = crate::FieldReader;
#[doc = "Field `GPU_SW_AW_COUNT_ID` writer - When sw_aw_cnt_id_type=1, only count\n\nthe id designated by sw_aw_count_id"]
pub type GpuSwAwCountIdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - When sw_ar_cnt_id_type=1, only count\n\nthe id designated by sw_ar_count_id"]
    #[inline(always)]
    pub fn gpu_sw_ar_count_id(&self) -> GpuSwArCountIdR {
        GpuSwArCountIdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - When sw_aw_cnt_id_type=1, only count\n\nthe id designated by sw_aw_count_id"]
    #[inline(always)]
    pub fn gpu_sw_aw_count_id(&self) -> GpuSwAwCountIdR {
        GpuSwAwCountIdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - When sw_ar_cnt_id_type=1, only count\n\nthe id designated by sw_ar_count_id"]
    #[inline(always)]
    #[must_use]
    pub fn gpu_sw_ar_count_id(&mut self) -> GpuSwArCountIdW<GrfGpuPerfCon2Spec> {
        GpuSwArCountIdW::new(self, 0)
    }
    #[doc = "Bits 8:12 - When sw_aw_cnt_id_type=1, only count\n\nthe id designated by sw_aw_count_id"]
    #[inline(always)]
    #[must_use]
    pub fn gpu_sw_aw_count_id(&mut self) -> GpuSwAwCountIdW<GrfGpuPerfCon2Spec> {
        GpuSwAwCountIdW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpuPerfCon2Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "gpu performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpu_perf_con2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpu_perf_con2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpuPerfCon2Spec;
impl crate::RegisterSpec for GrfGpuPerfCon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpu_perf_con2::R`](R) reader structure"]
impl crate::Readable for GrfGpuPerfCon2Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpu_perf_con2::W`](W) writer structure"]
impl crate::Writable for GrfGpuPerfCon2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPU_PERF_CON2 to value 0"]
impl crate::Resettable for GrfGpuPerfCon2Spec {
    const RESET_VALUE: u32 = 0;
}
