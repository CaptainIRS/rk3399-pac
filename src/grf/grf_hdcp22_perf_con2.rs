#[doc = "Register `GRF_HDCP22_PERF_CON2` reader"]
pub type R = crate::R<GrfHdcp22PerfCon2Spec>;
#[doc = "Register `GRF_HDCP22_PERF_CON2` writer"]
pub type W = crate::W<GrfHdcp22PerfCon2Spec>;
#[doc = "Field `HDCP22_SW_AR_COUNT_ID` reader - When sw_ar_cnt_id_type=1, only count\n\nthe id designated by sw_ar_count_id"]
pub type Hdcp22SwArCountIdR = crate::FieldReader;
#[doc = "Field `HDCP22_SW_AR_COUNT_ID` writer - When sw_ar_cnt_id_type=1, only count\n\nthe id designated by sw_ar_count_id"]
pub type Hdcp22SwArCountIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HDCP22_SW_AW_COUNT_ID` reader - When sw_aw_cnt_id_type=1, only count\n\nthe id designated by sw_aw_count_id"]
pub type Hdcp22SwAwCountIdR = crate::FieldReader;
#[doc = "Field `HDCP22_SW_AW_COUNT_ID` writer - When sw_aw_cnt_id_type=1, only count\n\nthe id designated by sw_aw_count_id"]
pub type Hdcp22SwAwCountIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - When sw_ar_cnt_id_type=1, only count\n\nthe id designated by sw_ar_count_id"]
    #[inline(always)]
    pub fn hdcp22_sw_ar_count_id(&self) -> Hdcp22SwArCountIdR {
        Hdcp22SwArCountIdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - When sw_aw_cnt_id_type=1, only count\n\nthe id designated by sw_aw_count_id"]
    #[inline(always)]
    pub fn hdcp22_sw_aw_count_id(&self) -> Hdcp22SwAwCountIdR {
        Hdcp22SwAwCountIdR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - When sw_ar_cnt_id_type=1, only count\n\nthe id designated by sw_ar_count_id"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp22_sw_ar_count_id(&mut self) -> Hdcp22SwArCountIdW<GrfHdcp22PerfCon2Spec> {
        Hdcp22SwArCountIdW::new(self, 0)
    }
    #[doc = "Bits 4:7 - When sw_aw_cnt_id_type=1, only count\n\nthe id designated by sw_aw_count_id"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp22_sw_aw_count_id(&mut self) -> Hdcp22SwAwCountIdW<GrfHdcp22PerfCon2Spec> {
        Hdcp22SwAwCountIdW::new(self, 4)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfHdcp22PerfCon2Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "hdcp performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_hdcp22_perf_con2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_hdcp22_perf_con2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfHdcp22PerfCon2Spec;
impl crate::RegisterSpec for GrfHdcp22PerfCon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_hdcp22_perf_con2::R`](R) reader structure"]
impl crate::Readable for GrfHdcp22PerfCon2Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_hdcp22_perf_con2::W`](W) writer structure"]
impl crate::Writable for GrfHdcp22PerfCon2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_HDCP22_PERF_CON2 to value 0"]
impl crate::Resettable for GrfHdcp22PerfCon2Spec {
    const RESET_VALUE: u32 = 0;
}
