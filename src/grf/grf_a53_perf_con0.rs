#[doc = "Register `GRF_A53_PERF_CON0` reader"]
pub type R = crate::R<GrfA53PerfCon0Spec>;
#[doc = "Register `GRF_A53_PERF_CON0` writer"]
pub type W = crate::W<GrfA53PerfCon0Spec>;
#[doc = "a53 performance monitor control register axi_perf enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A53SwAxiPerfWork {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<A53SwAxiPerfWork> for bool {
    #[inline(always)]
    fn from(variant: A53SwAxiPerfWork) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A53_SW_AXI_PERF_WORK` reader - a53 performance monitor control register axi_perf enable bit"]
pub type A53SwAxiPerfWorkR = crate::BitReader<A53SwAxiPerfWork>;
impl A53SwAxiPerfWorkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A53SwAxiPerfWork {
        match self.bits {
            false => A53SwAxiPerfWork::B0,
            true => A53SwAxiPerfWork::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == A53SwAxiPerfWork::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == A53SwAxiPerfWork::B1
    }
}
#[doc = "Field `A53_SW_AXI_PERF_WORK` writer - a53 performance monitor control register axi_perf enable bit"]
pub type A53SwAxiPerfWorkW<'a, REG> = crate::BitWriter<'a, REG, A53SwAxiPerfWork>;
impl<'a, REG> A53SwAxiPerfWorkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(A53SwAxiPerfWork::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(A53SwAxiPerfWork::B1)
    }
}
#[doc = "axi_perf clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A53SwAxiPerfClr {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<A53SwAxiPerfClr> for bool {
    #[inline(always)]
    fn from(variant: A53SwAxiPerfClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A53_SW_AXI_PERF_CLR` reader - axi_perf clear bit"]
pub type A53SwAxiPerfClrR = crate::BitReader<A53SwAxiPerfClr>;
impl A53SwAxiPerfClrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A53SwAxiPerfClr {
        match self.bits {
            false => A53SwAxiPerfClr::B0,
            true => A53SwAxiPerfClr::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == A53SwAxiPerfClr::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == A53SwAxiPerfClr::B1
    }
}
#[doc = "Field `A53_SW_AXI_PERF_CLR` writer - axi_perf clear bit"]
pub type A53SwAxiPerfClrW<'a, REG> = crate::BitWriter<'a, REG, A53SwAxiPerfClr>;
impl<'a, REG> A53SwAxiPerfClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(A53SwAxiPerfClr::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(A53SwAxiPerfClr::B1)
    }
}
#[doc = "axi_perf counter type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A53SwAxiCntType {
    #[doc = "0: ddr align transfer test"]
    B0 = 0,
    #[doc = "1: ddr align transfer test"]
    B1 = 1,
}
impl From<A53SwAxiCntType> for bool {
    #[inline(always)]
    fn from(variant: A53SwAxiCntType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A53_SW_AXI_CNT_TYPE` reader - axi_perf counter type"]
pub type A53SwAxiCntTypeR = crate::BitReader<A53SwAxiCntType>;
impl A53SwAxiCntTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A53SwAxiCntType {
        match self.bits {
            false => A53SwAxiCntType::B0,
            true => A53SwAxiCntType::B1,
        }
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == A53SwAxiCntType::B0
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == A53SwAxiCntType::B1
    }
}
#[doc = "Field `A53_SW_AXI_CNT_TYPE` writer - axi_perf counter type"]
pub type A53SwAxiCntTypeW<'a, REG> = crate::BitWriter<'a, REG, A53SwAxiCntType>;
impl<'a, REG> A53SwAxiCntTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(A53SwAxiCntType::B0)
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(A53SwAxiCntType::B1)
    }
}
#[doc = "axi_perf counter id control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A53SwArCntIdType {
    #[doc = "0: count sw_ar_count_id read channel only"]
    B0 = 0,
    #[doc = "1: count sw_ar_count_id read channel only"]
    B1 = 1,
}
impl From<A53SwArCntIdType> for bool {
    #[inline(always)]
    fn from(variant: A53SwArCntIdType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A53_SW_AR_CNT_ID_TYPE` reader - axi_perf counter id control"]
pub type A53SwArCntIdTypeR = crate::BitReader<A53SwArCntIdType>;
impl A53SwArCntIdTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A53SwArCntIdType {
        match self.bits {
            false => A53SwArCntIdType::B0,
            true => A53SwArCntIdType::B1,
        }
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == A53SwArCntIdType::B0
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == A53SwArCntIdType::B1
    }
}
#[doc = "Field `A53_SW_AR_CNT_ID_TYPE` writer - axi_perf counter id control"]
pub type A53SwArCntIdTypeW<'a, REG> = crate::BitWriter<'a, REG, A53SwArCntIdType>;
impl<'a, REG> A53SwArCntIdTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(A53SwArCntIdType::B0)
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(A53SwArCntIdType::B1)
    }
}
#[doc = "axi_perf counter id control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A53SwAwCntIdType {
    #[doc = "0: count sw_ar_count_id write channel only"]
    B0 = 0,
    #[doc = "1: count sw_ar_count_id write channel only"]
    B1 = 1,
}
impl From<A53SwAwCntIdType> for bool {
    #[inline(always)]
    fn from(variant: A53SwAwCntIdType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A53_SW_AW_CNT_ID_TYPE` reader - axi_perf counter id control"]
pub type A53SwAwCntIdTypeR = crate::BitReader<A53SwAwCntIdType>;
impl A53SwAwCntIdTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A53SwAwCntIdType {
        match self.bits {
            false => A53SwAwCntIdType::B0,
            true => A53SwAwCntIdType::B1,
        }
    }
    #[doc = "count sw_ar_count_id write channel only"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == A53SwAwCntIdType::B0
    }
    #[doc = "count sw_ar_count_id write channel only"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == A53SwAwCntIdType::B1
    }
}
#[doc = "Field `A53_SW_AW_CNT_ID_TYPE` writer - axi_perf counter id control"]
pub type A53SwAwCntIdTypeW<'a, REG> = crate::BitWriter<'a, REG, A53SwAwCntIdType>;
impl<'a, REG> A53SwAwCntIdTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "count sw_ar_count_id write channel only"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(A53SwAwCntIdType::B0)
    }
    #[doc = "count sw_ar_count_id write channel only"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(A53SwAwCntIdType::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum A53SwDdrAlignType {
    #[doc = "0: 128-Byte align"]
    D0 = 0,
    #[doc = "1: 128-Byte align"]
    D1 = 1,
    #[doc = "2: 128-Byte align"]
    D2 = 2,
    #[doc = "3: 128-Byte align"]
    D3 = 3,
}
impl From<A53SwDdrAlignType> for u8 {
    #[inline(always)]
    fn from(variant: A53SwDdrAlignType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for A53SwDdrAlignType {
    type Ux = u8;
}
#[doc = "Field `A53_SW_DDR_ALIGN_TYPE` reader - "]
pub type A53SwDdrAlignTypeR = crate::FieldReader<A53SwDdrAlignType>;
impl A53SwDdrAlignTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A53SwDdrAlignType {
        match self.bits {
            0 => A53SwDdrAlignType::D0,
            1 => A53SwDdrAlignType::D1,
            2 => A53SwDdrAlignType::D2,
            3 => A53SwDdrAlignType::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == A53SwDdrAlignType::D0
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == A53SwDdrAlignType::D1
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == A53SwDdrAlignType::D2
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == A53SwDdrAlignType::D3
    }
}
#[doc = "Field `A53_SW_DDR_ALIGN_TYPE` writer - "]
pub type A53SwDdrAlignTypeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, A53SwDdrAlignType>;
impl<'a, REG> A53SwDdrAlignTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(A53SwDdrAlignType::D0)
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(A53SwDdrAlignType::D1)
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(A53SwDdrAlignType::D2)
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(A53SwDdrAlignType::D3)
    }
}
#[doc = "Field `A53_SW_RD_LATENCY_ID` reader - Axi read channel id for latency AXI_PERFormance test"]
pub type A53SwRdLatencyIdR = crate::FieldReader;
#[doc = "Field `A53_SW_RD_LATENCY_ID` writer - Axi read channel id for latency AXI_PERFormance test"]
pub type A53SwRdLatencyIdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - a53 performance monitor control register axi_perf enable bit"]
    #[inline(always)]
    pub fn a53_sw_axi_perf_work(&self) -> A53SwAxiPerfWorkR {
        A53SwAxiPerfWorkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - axi_perf clear bit"]
    #[inline(always)]
    pub fn a53_sw_axi_perf_clr(&self) -> A53SwAxiPerfClrR {
        A53SwAxiPerfClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - axi_perf counter type"]
    #[inline(always)]
    pub fn a53_sw_axi_cnt_type(&self) -> A53SwAxiCntTypeR {
        A53SwAxiCntTypeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - axi_perf counter id control"]
    #[inline(always)]
    pub fn a53_sw_ar_cnt_id_type(&self) -> A53SwArCntIdTypeR {
        A53SwArCntIdTypeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - axi_perf counter id control"]
    #[inline(always)]
    pub fn a53_sw_aw_cnt_id_type(&self) -> A53SwAwCntIdTypeR {
        A53SwAwCntIdTypeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn a53_sw_ddr_align_type(&self) -> A53SwDdrAlignTypeR {
        A53SwDdrAlignTypeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Axi read channel id for latency AXI_PERFormance test"]
    #[inline(always)]
    pub fn a53_sw_rd_latency_id(&self) -> A53SwRdLatencyIdR {
        A53SwRdLatencyIdR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - a53 performance monitor control register axi_perf enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn a53_sw_axi_perf_work(&mut self) -> A53SwAxiPerfWorkW<GrfA53PerfCon0Spec> {
        A53SwAxiPerfWorkW::new(self, 0)
    }
    #[doc = "Bit 1 - axi_perf clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn a53_sw_axi_perf_clr(&mut self) -> A53SwAxiPerfClrW<GrfA53PerfCon0Spec> {
        A53SwAxiPerfClrW::new(self, 1)
    }
    #[doc = "Bit 2 - axi_perf counter type"]
    #[inline(always)]
    #[must_use]
    pub fn a53_sw_axi_cnt_type(&mut self) -> A53SwAxiCntTypeW<GrfA53PerfCon0Spec> {
        A53SwAxiCntTypeW::new(self, 2)
    }
    #[doc = "Bit 3 - axi_perf counter id control"]
    #[inline(always)]
    #[must_use]
    pub fn a53_sw_ar_cnt_id_type(&mut self) -> A53SwArCntIdTypeW<GrfA53PerfCon0Spec> {
        A53SwArCntIdTypeW::new(self, 3)
    }
    #[doc = "Bit 4 - axi_perf counter id control"]
    #[inline(always)]
    #[must_use]
    pub fn a53_sw_aw_cnt_id_type(&mut self) -> A53SwAwCntIdTypeW<GrfA53PerfCon0Spec> {
        A53SwAwCntIdTypeW::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn a53_sw_ddr_align_type(&mut self) -> A53SwDdrAlignTypeW<GrfA53PerfCon0Spec> {
        A53SwDdrAlignTypeW::new(self, 5)
    }
    #[doc = "Bits 8:13 - Axi read channel id for latency AXI_PERFormance test"]
    #[inline(always)]
    #[must_use]
    pub fn a53_sw_rd_latency_id(&mut self) -> A53SwRdLatencyIdW<GrfA53PerfCon0Spec> {
        A53SwRdLatencyIdW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfA53PerfCon0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "a53 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a53_perf_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a53_perf_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfA53PerfCon0Spec;
impl crate::RegisterSpec for GrfA53PerfCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_a53_perf_con0::R`](R) reader structure"]
impl crate::Readable for GrfA53PerfCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_a53_perf_con0::W`](W) writer structure"]
impl crate::Writable for GrfA53PerfCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_A53_PERF_CON0 to value 0"]
impl crate::Resettable for GrfA53PerfCon0Spec {
    const RESET_VALUE: u32 = 0;
}
