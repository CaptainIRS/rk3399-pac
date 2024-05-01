#[doc = "Register `PERF_LATENCY_CTRL1` reader"]
pub type R = crate::R<PerfLatencyCtrl1Spec>;
#[doc = "Register `PERF_LATENCY_CTRL1` writer"]
pub type W = crate::W<PerfLatencyCtrl1Spec>;
#[doc = "Field `SW_ADDR_ALIGN_TYPE` reader - sw_addr_align_type"]
pub type SwAddrAlignTypeR = crate::FieldReader;
#[doc = "Field `SW_ADDR_ALIGN_TYPE` writer - sw_addr_align_type"]
pub type SwAddrAlignTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_AR_CNT_ID_TYPE` reader - sw_ar_cnt_id_type"]
pub type SwArCntIdTypeR = crate::BitReader;
#[doc = "Field `SW_AR_CNT_ID_TYPE` writer - sw_ar_cnt_id_type"]
pub type SwArCntIdTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_AW_CNT_ID_TYPE` reader - sw_aw_cnt_id_type"]
pub type SwAwCntIdTypeR = crate::BitReader;
#[doc = "Field `SW_AW_CNT_ID_TYPE` writer - sw_aw_cnt_id_type"]
pub type SwAwCntIdTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_AR_COUNT_ID` reader - sw_ar_count_id"]
pub type SwArCountIdR = crate::FieldReader;
#[doc = "Field `SW_AR_COUNT_ID` writer - sw_ar_count_id"]
pub type SwArCountIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SW_AW_COUNT_ID` reader - sw_aw_count_id"]
pub type SwAwCountIdR = crate::FieldReader;
#[doc = "Field `SW_AW_COUNT_ID` writer - sw_aw_count_id"]
pub type SwAwCountIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - sw_addr_align_type"]
    #[inline(always)]
    pub fn sw_addr_align_type(&self) -> SwAddrAlignTypeR {
        SwAddrAlignTypeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - sw_ar_cnt_id_type"]
    #[inline(always)]
    pub fn sw_ar_cnt_id_type(&self) -> SwArCntIdTypeR {
        SwArCntIdTypeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sw_aw_cnt_id_type"]
    #[inline(always)]
    pub fn sw_aw_cnt_id_type(&self) -> SwAwCntIdTypeR {
        SwAwCntIdTypeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - sw_ar_count_id"]
    #[inline(always)]
    pub fn sw_ar_count_id(&self) -> SwArCountIdR {
        SwArCountIdR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - sw_aw_count_id"]
    #[inline(always)]
    pub fn sw_aw_count_id(&self) -> SwAwCountIdR {
        SwAwCountIdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - sw_addr_align_type"]
    #[inline(always)]
    #[must_use]
    pub fn sw_addr_align_type(&mut self) -> SwAddrAlignTypeW<PerfLatencyCtrl1Spec> {
        SwAddrAlignTypeW::new(self, 0)
    }
    #[doc = "Bit 2 - sw_ar_cnt_id_type"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ar_cnt_id_type(&mut self) -> SwArCntIdTypeW<PerfLatencyCtrl1Spec> {
        SwArCntIdTypeW::new(self, 2)
    }
    #[doc = "Bit 3 - sw_aw_cnt_id_type"]
    #[inline(always)]
    #[must_use]
    pub fn sw_aw_cnt_id_type(&mut self) -> SwAwCntIdTypeW<PerfLatencyCtrl1Spec> {
        SwAwCntIdTypeW::new(self, 3)
    }
    #[doc = "Bits 4:7 - sw_ar_count_id"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ar_count_id(&mut self) -> SwArCountIdW<PerfLatencyCtrl1Spec> {
        SwArCountIdW::new(self, 4)
    }
    #[doc = "Bits 8:11 - sw_aw_count_id"]
    #[inline(always)]
    #[must_use]
    pub fn sw_aw_count_id(&mut self) -> SwAwCountIdW<PerfLatencyCtrl1Spec> {
        SwAwCountIdW::new(self, 8)
    }
}
#[doc = "PERF_LATENCY_CTRL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perf_latency_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perf_latency_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerfLatencyCtrl1Spec;
impl crate::RegisterSpec for PerfLatencyCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perf_latency_ctrl1::R`](R) reader structure"]
impl crate::Readable for PerfLatencyCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`perf_latency_ctrl1::W`](W) writer structure"]
impl crate::Writable for PerfLatencyCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERF_LATENCY_CTRL1 to value 0x11"]
impl crate::Resettable for PerfLatencyCtrl1Spec {
    const RESET_VALUE: u32 = 0x11;
}
