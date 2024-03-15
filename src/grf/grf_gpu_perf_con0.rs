#[doc = "Register `GRF_GPU_PERF_CON0` reader"]
pub type R = crate::R<GrfGpuPerfCon0Spec>;
#[doc = "Register `GRF_GPU_PERF_CON0` writer"]
pub type W = crate::W<GrfGpuPerfCon0Spec>;
#[doc = "axi_perf enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuSwAxiPerfWork {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<GpuSwAxiPerfWork> for bool {
    #[inline(always)]
    fn from(variant: GpuSwAxiPerfWork) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPU_SW_AXI_PERF_WORK` reader - axi_perf enable bit"]
pub type GpuSwAxiPerfWorkR = crate::BitReader<GpuSwAxiPerfWork>;
impl GpuSwAxiPerfWorkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpuSwAxiPerfWork {
        match self.bits {
            false => GpuSwAxiPerfWork::B0,
            true => GpuSwAxiPerfWork::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GpuSwAxiPerfWork::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GpuSwAxiPerfWork::B1
    }
}
#[doc = "Field `GPU_SW_AXI_PERF_WORK` writer - axi_perf enable bit"]
pub type GpuSwAxiPerfWorkW<'a, REG> = crate::BitWriter<'a, REG, GpuSwAxiPerfWork>;
impl<'a, REG> GpuSwAxiPerfWorkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GpuSwAxiPerfWork::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GpuSwAxiPerfWork::B1)
    }
}
#[doc = "axi_perf clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuSwAxiPerfClr {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<GpuSwAxiPerfClr> for bool {
    #[inline(always)]
    fn from(variant: GpuSwAxiPerfClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPU_SW_AXI_PERF_CLR` reader - axi_perf clear bit"]
pub type GpuSwAxiPerfClrR = crate::BitReader<GpuSwAxiPerfClr>;
impl GpuSwAxiPerfClrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpuSwAxiPerfClr {
        match self.bits {
            false => GpuSwAxiPerfClr::B0,
            true => GpuSwAxiPerfClr::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GpuSwAxiPerfClr::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GpuSwAxiPerfClr::B1
    }
}
#[doc = "Field `GPU_SW_AXI_PERF_CLR` writer - axi_perf clear bit"]
pub type GpuSwAxiPerfClrW<'a, REG> = crate::BitWriter<'a, REG, GpuSwAxiPerfClr>;
impl<'a, REG> GpuSwAxiPerfClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GpuSwAxiPerfClr::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GpuSwAxiPerfClr::B1)
    }
}
#[doc = "axi_perf counter type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuSwAxiCntType {
    #[doc = "0: ddr align transfer test"]
    B0 = 0,
    #[doc = "1: ddr align transfer test"]
    B1 = 1,
}
impl From<GpuSwAxiCntType> for bool {
    #[inline(always)]
    fn from(variant: GpuSwAxiCntType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPU_SW_AXI_CNT_TYPE` reader - axi_perf counter type"]
pub type GpuSwAxiCntTypeR = crate::BitReader<GpuSwAxiCntType>;
impl GpuSwAxiCntTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpuSwAxiCntType {
        match self.bits {
            false => GpuSwAxiCntType::B0,
            true => GpuSwAxiCntType::B1,
        }
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GpuSwAxiCntType::B0
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GpuSwAxiCntType::B1
    }
}
#[doc = "Field `GPU_SW_AXI_CNT_TYPE` writer - axi_perf counter type"]
pub type GpuSwAxiCntTypeW<'a, REG> = crate::BitWriter<'a, REG, GpuSwAxiCntType>;
impl<'a, REG> GpuSwAxiCntTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GpuSwAxiCntType::B0)
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GpuSwAxiCntType::B1)
    }
}
#[doc = "axi_perf counter id control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuSwArCntIdType {
    #[doc = "0: count sw_ar_count_id read channel only"]
    B0 = 0,
    #[doc = "1: count sw_ar_count_id read channel only"]
    B1 = 1,
}
impl From<GpuSwArCntIdType> for bool {
    #[inline(always)]
    fn from(variant: GpuSwArCntIdType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPU_SW_AR_CNT_ID_TYPE` reader - axi_perf counter id control"]
pub type GpuSwArCntIdTypeR = crate::BitReader<GpuSwArCntIdType>;
impl GpuSwArCntIdTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpuSwArCntIdType {
        match self.bits {
            false => GpuSwArCntIdType::B0,
            true => GpuSwArCntIdType::B1,
        }
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GpuSwArCntIdType::B0
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GpuSwArCntIdType::B1
    }
}
#[doc = "Field `GPU_SW_AR_CNT_ID_TYPE` writer - axi_perf counter id control"]
pub type GpuSwArCntIdTypeW<'a, REG> = crate::BitWriter<'a, REG, GpuSwArCntIdType>;
impl<'a, REG> GpuSwArCntIdTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GpuSwArCntIdType::B0)
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GpuSwArCntIdType::B1)
    }
}
#[doc = "axi_perf counter id control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuSwAwCntIdType {
    #[doc = "0: count sw_ar_count_id write channel only"]
    B0 = 0,
    #[doc = "1: count sw_ar_count_id write channel only"]
    B1 = 1,
}
impl From<GpuSwAwCntIdType> for bool {
    #[inline(always)]
    fn from(variant: GpuSwAwCntIdType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPU_SW_AW_CNT_ID_TYPE` reader - axi_perf counter id control"]
pub type GpuSwAwCntIdTypeR = crate::BitReader<GpuSwAwCntIdType>;
impl GpuSwAwCntIdTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpuSwAwCntIdType {
        match self.bits {
            false => GpuSwAwCntIdType::B0,
            true => GpuSwAwCntIdType::B1,
        }
    }
    #[doc = "count sw_ar_count_id write channel only"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GpuSwAwCntIdType::B0
    }
    #[doc = "count sw_ar_count_id write channel only"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GpuSwAwCntIdType::B1
    }
}
#[doc = "Field `GPU_SW_AW_CNT_ID_TYPE` writer - axi_perf counter id control"]
pub type GpuSwAwCntIdTypeW<'a, REG> = crate::BitWriter<'a, REG, GpuSwAwCntIdType>;
impl<'a, REG> GpuSwAwCntIdTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "count sw_ar_count_id write channel only"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GpuSwAwCntIdType::B0)
    }
    #[doc = "count sw_ar_count_id write channel only"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GpuSwAwCntIdType::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GpuSwDdrAlignType {
    #[doc = "0: 128-Byte align"]
    D0 = 0,
    #[doc = "1: 128-Byte align"]
    D1 = 1,
    #[doc = "2: 128-Byte align"]
    D2 = 2,
    #[doc = "3: 128-Byte align"]
    D3 = 3,
}
impl From<GpuSwDdrAlignType> for u8 {
    #[inline(always)]
    fn from(variant: GpuSwDdrAlignType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpuSwDdrAlignType {
    type Ux = u8;
}
#[doc = "Field `GPU_SW_DDR_ALIGN_TYPE` reader - "]
pub type GpuSwDdrAlignTypeR = crate::FieldReader<GpuSwDdrAlignType>;
impl GpuSwDdrAlignTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpuSwDdrAlignType {
        match self.bits {
            0 => GpuSwDdrAlignType::D0,
            1 => GpuSwDdrAlignType::D1,
            2 => GpuSwDdrAlignType::D2,
            3 => GpuSwDdrAlignType::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == GpuSwDdrAlignType::D0
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == GpuSwDdrAlignType::D1
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == GpuSwDdrAlignType::D2
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == GpuSwDdrAlignType::D3
    }
}
#[doc = "Field `GPU_SW_DDR_ALIGN_TYPE` writer - "]
pub type GpuSwDdrAlignTypeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, GpuSwDdrAlignType>;
impl<'a, REG> GpuSwDdrAlignTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(GpuSwDdrAlignType::D0)
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(GpuSwDdrAlignType::D1)
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(GpuSwDdrAlignType::D2)
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(GpuSwDdrAlignType::D3)
    }
}
#[doc = "Field `GPU_SW_RD_LATENCY_ID` reader - Axi read channel id for latency AXI_PERFormance test"]
pub type GpuSwRdLatencyIdR = crate::FieldReader;
#[doc = "Field `GPU_SW_RD_LATENCY_ID` writer - Axi read channel id for latency AXI_PERFormance test"]
pub type GpuSwRdLatencyIdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - axi_perf enable bit"]
    #[inline(always)]
    pub fn gpu_sw_axi_perf_work(&self) -> GpuSwAxiPerfWorkR {
        GpuSwAxiPerfWorkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - axi_perf clear bit"]
    #[inline(always)]
    pub fn gpu_sw_axi_perf_clr(&self) -> GpuSwAxiPerfClrR {
        GpuSwAxiPerfClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - axi_perf counter type"]
    #[inline(always)]
    pub fn gpu_sw_axi_cnt_type(&self) -> GpuSwAxiCntTypeR {
        GpuSwAxiCntTypeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - axi_perf counter id control"]
    #[inline(always)]
    pub fn gpu_sw_ar_cnt_id_type(&self) -> GpuSwArCntIdTypeR {
        GpuSwArCntIdTypeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - axi_perf counter id control"]
    #[inline(always)]
    pub fn gpu_sw_aw_cnt_id_type(&self) -> GpuSwAwCntIdTypeR {
        GpuSwAwCntIdTypeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn gpu_sw_ddr_align_type(&self) -> GpuSwDdrAlignTypeR {
        GpuSwDdrAlignTypeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Axi read channel id for latency AXI_PERFormance test"]
    #[inline(always)]
    pub fn gpu_sw_rd_latency_id(&self) -> GpuSwRdLatencyIdR {
        GpuSwRdLatencyIdR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - axi_perf enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn gpu_sw_axi_perf_work(&mut self) -> GpuSwAxiPerfWorkW<GrfGpuPerfCon0Spec> {
        GpuSwAxiPerfWorkW::new(self, 0)
    }
    #[doc = "Bit 1 - axi_perf clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn gpu_sw_axi_perf_clr(&mut self) -> GpuSwAxiPerfClrW<GrfGpuPerfCon0Spec> {
        GpuSwAxiPerfClrW::new(self, 1)
    }
    #[doc = "Bit 2 - axi_perf counter type"]
    #[inline(always)]
    #[must_use]
    pub fn gpu_sw_axi_cnt_type(&mut self) -> GpuSwAxiCntTypeW<GrfGpuPerfCon0Spec> {
        GpuSwAxiCntTypeW::new(self, 2)
    }
    #[doc = "Bit 3 - axi_perf counter id control"]
    #[inline(always)]
    #[must_use]
    pub fn gpu_sw_ar_cnt_id_type(&mut self) -> GpuSwArCntIdTypeW<GrfGpuPerfCon0Spec> {
        GpuSwArCntIdTypeW::new(self, 3)
    }
    #[doc = "Bit 4 - axi_perf counter id control"]
    #[inline(always)]
    #[must_use]
    pub fn gpu_sw_aw_cnt_id_type(&mut self) -> GpuSwAwCntIdTypeW<GrfGpuPerfCon0Spec> {
        GpuSwAwCntIdTypeW::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn gpu_sw_ddr_align_type(&mut self) -> GpuSwDdrAlignTypeW<GrfGpuPerfCon0Spec> {
        GpuSwDdrAlignTypeW::new(self, 5)
    }
    #[doc = "Bits 8:13 - Axi read channel id for latency AXI_PERFormance test"]
    #[inline(always)]
    #[must_use]
    pub fn gpu_sw_rd_latency_id(&mut self) -> GpuSwRdLatencyIdW<GrfGpuPerfCon0Spec> {
        GpuSwRdLatencyIdW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpuPerfCon0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "gpu performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpu_perf_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpu_perf_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpuPerfCon0Spec;
impl crate::RegisterSpec for GrfGpuPerfCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpu_perf_con0::R`](R) reader structure"]
impl crate::Readable for GrfGpuPerfCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpu_perf_con0::W`](W) writer structure"]
impl crate::Writable for GrfGpuPerfCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPU_PERF_CON0 to value 0"]
impl crate::Resettable for GrfGpuPerfCon0Spec {
    const RESET_VALUE: u32 = 0;
}
