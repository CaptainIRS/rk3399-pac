#[doc = "Register `GRF_GMAC_PERF_CON0` reader"]
pub type R = crate::R<GrfGmacPerfCon0Spec>;
#[doc = "Register `GRF_GMAC_PERF_CON0` writer"]
pub type W = crate::W<GrfGmacPerfCon0Spec>;
#[doc = "axi_perf enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GmacSwAxiPerfWork {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<GmacSwAxiPerfWork> for bool {
    #[inline(always)]
    fn from(variant: GmacSwAxiPerfWork) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GMAC_SW_AXI_PERF_WORK` reader - axi_perf enable bit"]
pub type GmacSwAxiPerfWorkR = crate::BitReader<GmacSwAxiPerfWork>;
impl GmacSwAxiPerfWorkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GmacSwAxiPerfWork {
        match self.bits {
            false => GmacSwAxiPerfWork::B0,
            true => GmacSwAxiPerfWork::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GmacSwAxiPerfWork::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GmacSwAxiPerfWork::B1
    }
}
#[doc = "Field `GMAC_SW_AXI_PERF_WORK` writer - axi_perf enable bit"]
pub type GmacSwAxiPerfWorkW<'a, REG> = crate::BitWriter<'a, REG, GmacSwAxiPerfWork>;
impl<'a, REG> GmacSwAxiPerfWorkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GmacSwAxiPerfWork::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GmacSwAxiPerfWork::B1)
    }
}
#[doc = "axi_perf clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GmacSwAxiPerfClr {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<GmacSwAxiPerfClr> for bool {
    #[inline(always)]
    fn from(variant: GmacSwAxiPerfClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GMAC_SW_AXI_PERF_CLR` reader - axi_perf clear bit"]
pub type GmacSwAxiPerfClrR = crate::BitReader<GmacSwAxiPerfClr>;
impl GmacSwAxiPerfClrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GmacSwAxiPerfClr {
        match self.bits {
            false => GmacSwAxiPerfClr::B0,
            true => GmacSwAxiPerfClr::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GmacSwAxiPerfClr::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GmacSwAxiPerfClr::B1
    }
}
#[doc = "Field `GMAC_SW_AXI_PERF_CLR` writer - axi_perf clear bit"]
pub type GmacSwAxiPerfClrW<'a, REG> = crate::BitWriter<'a, REG, GmacSwAxiPerfClr>;
impl<'a, REG> GmacSwAxiPerfClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GmacSwAxiPerfClr::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GmacSwAxiPerfClr::B1)
    }
}
#[doc = "axi_perf counter type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GmacSwAxiCntType {
    #[doc = "0: ddr align transfer test"]
    B0 = 0,
    #[doc = "1: ddr align transfer test"]
    B1 = 1,
}
impl From<GmacSwAxiCntType> for bool {
    #[inline(always)]
    fn from(variant: GmacSwAxiCntType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GMAC_SW_AXI_CNT_TYPE` reader - axi_perf counter type"]
pub type GmacSwAxiCntTypeR = crate::BitReader<GmacSwAxiCntType>;
impl GmacSwAxiCntTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GmacSwAxiCntType {
        match self.bits {
            false => GmacSwAxiCntType::B0,
            true => GmacSwAxiCntType::B1,
        }
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GmacSwAxiCntType::B0
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GmacSwAxiCntType::B1
    }
}
#[doc = "Field `GMAC_SW_AXI_CNT_TYPE` writer - axi_perf counter type"]
pub type GmacSwAxiCntTypeW<'a, REG> = crate::BitWriter<'a, REG, GmacSwAxiCntType>;
impl<'a, REG> GmacSwAxiCntTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GmacSwAxiCntType::B0)
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GmacSwAxiCntType::B1)
    }
}
#[doc = "axi_perf counter id control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GmacSwArCntIdType {
    #[doc = "0: count sw_ar_count_id read channel only"]
    B0 = 0,
    #[doc = "1: count sw_ar_count_id read channel only"]
    B1 = 1,
}
impl From<GmacSwArCntIdType> for bool {
    #[inline(always)]
    fn from(variant: GmacSwArCntIdType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GMAC_SW_AR_CNT_ID_TYPE` reader - axi_perf counter id control"]
pub type GmacSwArCntIdTypeR = crate::BitReader<GmacSwArCntIdType>;
impl GmacSwArCntIdTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GmacSwArCntIdType {
        match self.bits {
            false => GmacSwArCntIdType::B0,
            true => GmacSwArCntIdType::B1,
        }
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GmacSwArCntIdType::B0
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GmacSwArCntIdType::B1
    }
}
#[doc = "Field `GMAC_SW_AR_CNT_ID_TYPE` writer - axi_perf counter id control"]
pub type GmacSwArCntIdTypeW<'a, REG> = crate::BitWriter<'a, REG, GmacSwArCntIdType>;
impl<'a, REG> GmacSwArCntIdTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GmacSwArCntIdType::B0)
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GmacSwArCntIdType::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GmacSwAwCntIdType {
    #[doc = "0: count sw_aw_count_id write channel only"]
    B0 = 0,
    #[doc = "1: count sw_aw_count_id write channel only"]
    B1 = 1,
}
impl From<GmacSwAwCntIdType> for bool {
    #[inline(always)]
    fn from(variant: GmacSwAwCntIdType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GMAC_SW_AW_CNT_ID_TYPE` reader - "]
pub type GmacSwAwCntIdTypeR = crate::BitReader<GmacSwAwCntIdType>;
impl GmacSwAwCntIdTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GmacSwAwCntIdType {
        match self.bits {
            false => GmacSwAwCntIdType::B0,
            true => GmacSwAwCntIdType::B1,
        }
    }
    #[doc = "count sw_aw_count_id write channel only"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GmacSwAwCntIdType::B0
    }
    #[doc = "count sw_aw_count_id write channel only"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GmacSwAwCntIdType::B1
    }
}
#[doc = "Field `GMAC_SW_AW_CNT_ID_TYPE` writer - "]
pub type GmacSwAwCntIdTypeW<'a, REG> = crate::BitWriter<'a, REG, GmacSwAwCntIdType>;
impl<'a, REG> GmacSwAwCntIdTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "count sw_aw_count_id write channel only"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GmacSwAwCntIdType::B0)
    }
    #[doc = "count sw_aw_count_id write channel only"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GmacSwAwCntIdType::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GmacSwDdrAlignType {
    #[doc = "0: 128-Byte align"]
    D0 = 0,
    #[doc = "1: 128-Byte align"]
    D1 = 1,
    #[doc = "2: 128-Byte align"]
    D2 = 2,
    #[doc = "3: 128-Byte align"]
    D3 = 3,
}
impl From<GmacSwDdrAlignType> for u8 {
    #[inline(always)]
    fn from(variant: GmacSwDdrAlignType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GmacSwDdrAlignType {
    type Ux = u8;
}
#[doc = "Field `GMAC_SW_DDR_ALIGN_TYPE` reader - "]
pub type GmacSwDdrAlignTypeR = crate::FieldReader<GmacSwDdrAlignType>;
impl GmacSwDdrAlignTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GmacSwDdrAlignType {
        match self.bits {
            0 => GmacSwDdrAlignType::D0,
            1 => GmacSwDdrAlignType::D1,
            2 => GmacSwDdrAlignType::D2,
            3 => GmacSwDdrAlignType::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == GmacSwDdrAlignType::D0
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == GmacSwDdrAlignType::D1
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == GmacSwDdrAlignType::D2
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == GmacSwDdrAlignType::D3
    }
}
#[doc = "Field `GMAC_SW_DDR_ALIGN_TYPE` writer - "]
pub type GmacSwDdrAlignTypeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, GmacSwDdrAlignType>;
impl<'a, REG> GmacSwDdrAlignTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(GmacSwDdrAlignType::D0)
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(GmacSwDdrAlignType::D1)
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(GmacSwDdrAlignType::D2)
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(GmacSwDdrAlignType::D3)
    }
}
#[doc = "Field `GMAC_SW_RD_LATENCY_ID` reader - Axi read channel id for latency AXI_PERFormance test"]
pub type GmacSwRdLatencyIdR = crate::FieldReader;
#[doc = "Field `GMAC_SW_RD_LATENCY_ID` writer - Axi read channel id for latency AXI_PERFormance test"]
pub type GmacSwRdLatencyIdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - axi_perf enable bit"]
    #[inline(always)]
    pub fn gmac_sw_axi_perf_work(&self) -> GmacSwAxiPerfWorkR {
        GmacSwAxiPerfWorkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - axi_perf clear bit"]
    #[inline(always)]
    pub fn gmac_sw_axi_perf_clr(&self) -> GmacSwAxiPerfClrR {
        GmacSwAxiPerfClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - axi_perf counter type"]
    #[inline(always)]
    pub fn gmac_sw_axi_cnt_type(&self) -> GmacSwAxiCntTypeR {
        GmacSwAxiCntTypeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - axi_perf counter id control"]
    #[inline(always)]
    pub fn gmac_sw_ar_cnt_id_type(&self) -> GmacSwArCntIdTypeR {
        GmacSwArCntIdTypeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gmac_sw_aw_cnt_id_type(&self) -> GmacSwAwCntIdTypeR {
        GmacSwAwCntIdTypeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn gmac_sw_ddr_align_type(&self) -> GmacSwDdrAlignTypeR {
        GmacSwDdrAlignTypeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Axi read channel id for latency AXI_PERFormance test"]
    #[inline(always)]
    pub fn gmac_sw_rd_latency_id(&self) -> GmacSwRdLatencyIdR {
        GmacSwRdLatencyIdR::new(((self.bits >> 8) & 0x1f) as u8)
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
    pub fn gmac_sw_axi_perf_work(&mut self) -> GmacSwAxiPerfWorkW<GrfGmacPerfCon0Spec> {
        GmacSwAxiPerfWorkW::new(self, 0)
    }
    #[doc = "Bit 1 - axi_perf clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_sw_axi_perf_clr(&mut self) -> GmacSwAxiPerfClrW<GrfGmacPerfCon0Spec> {
        GmacSwAxiPerfClrW::new(self, 1)
    }
    #[doc = "Bit 2 - axi_perf counter type"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_sw_axi_cnt_type(&mut self) -> GmacSwAxiCntTypeW<GrfGmacPerfCon0Spec> {
        GmacSwAxiCntTypeW::new(self, 2)
    }
    #[doc = "Bit 3 - axi_perf counter id control"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_sw_ar_cnt_id_type(&mut self) -> GmacSwArCntIdTypeW<GrfGmacPerfCon0Spec> {
        GmacSwArCntIdTypeW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_sw_aw_cnt_id_type(&mut self) -> GmacSwAwCntIdTypeW<GrfGmacPerfCon0Spec> {
        GmacSwAwCntIdTypeW::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_sw_ddr_align_type(&mut self) -> GmacSwDdrAlignTypeW<GrfGmacPerfCon0Spec> {
        GmacSwDdrAlignTypeW::new(self, 5)
    }
    #[doc = "Bits 8:12 - Axi read channel id for latency AXI_PERFormance test"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_sw_rd_latency_id(&mut self) -> GmacSwRdLatencyIdW<GrfGmacPerfCon0Spec> {
        GmacSwRdLatencyIdW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGmacPerfCon0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "gmac performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gmac_perf_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gmac_perf_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGmacPerfCon0Spec;
impl crate::RegisterSpec for GrfGmacPerfCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gmac_perf_con0::R`](R) reader structure"]
impl crate::Readable for GrfGmacPerfCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_gmac_perf_con0::W`](W) writer structure"]
impl crate::Writable for GrfGmacPerfCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GMAC_PERF_CON0 to value 0"]
impl crate::Resettable for GrfGmacPerfCon0Spec {
    const RESET_VALUE: u32 = 0;
}
