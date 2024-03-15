#[doc = "Register `GRF_HDCP22_PERF_CON0` reader"]
pub type R = crate::R<GrfHdcp22PerfCon0Spec>;
#[doc = "Register `GRF_HDCP22_PERF_CON0` writer"]
pub type W = crate::W<GrfHdcp22PerfCon0Spec>;
#[doc = "axi_perf enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdcp22SwAxiPerfWork {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Hdcp22SwAxiPerfWork> for bool {
    #[inline(always)]
    fn from(variant: Hdcp22SwAxiPerfWork) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCP22_SW_AXI_PERF_WORK` reader - axi_perf enable bit"]
pub type Hdcp22SwAxiPerfWorkR = crate::BitReader<Hdcp22SwAxiPerfWork>;
impl Hdcp22SwAxiPerfWorkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdcp22SwAxiPerfWork {
        match self.bits {
            false => Hdcp22SwAxiPerfWork::B0,
            true => Hdcp22SwAxiPerfWork::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hdcp22SwAxiPerfWork::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hdcp22SwAxiPerfWork::B1
    }
}
#[doc = "Field `HDCP22_SW_AXI_PERF_WORK` writer - axi_perf enable bit"]
pub type Hdcp22SwAxiPerfWorkW<'a, REG> = crate::BitWriter<'a, REG, Hdcp22SwAxiPerfWork>;
impl<'a, REG> Hdcp22SwAxiPerfWorkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22SwAxiPerfWork::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22SwAxiPerfWork::B1)
    }
}
#[doc = "axi_perf clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdcp22SwAxiPerfClr {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Hdcp22SwAxiPerfClr> for bool {
    #[inline(always)]
    fn from(variant: Hdcp22SwAxiPerfClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCP22_SW_AXI_PERF_CLR` reader - axi_perf clear bit"]
pub type Hdcp22SwAxiPerfClrR = crate::BitReader<Hdcp22SwAxiPerfClr>;
impl Hdcp22SwAxiPerfClrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdcp22SwAxiPerfClr {
        match self.bits {
            false => Hdcp22SwAxiPerfClr::B0,
            true => Hdcp22SwAxiPerfClr::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hdcp22SwAxiPerfClr::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hdcp22SwAxiPerfClr::B1
    }
}
#[doc = "Field `HDCP22_SW_AXI_PERF_CLR` writer - axi_perf clear bit"]
pub type Hdcp22SwAxiPerfClrW<'a, REG> = crate::BitWriter<'a, REG, Hdcp22SwAxiPerfClr>;
impl<'a, REG> Hdcp22SwAxiPerfClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22SwAxiPerfClr::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22SwAxiPerfClr::B1)
    }
}
#[doc = "axi_perf counter type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdcp22SwAxiCntType {
    #[doc = "0: ddr align transfer test"]
    B0 = 0,
    #[doc = "1: ddr align transfer test"]
    B1 = 1,
}
impl From<Hdcp22SwAxiCntType> for bool {
    #[inline(always)]
    fn from(variant: Hdcp22SwAxiCntType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCP22_SW_AXI_CNT_TYPE` reader - axi_perf counter type"]
pub type Hdcp22SwAxiCntTypeR = crate::BitReader<Hdcp22SwAxiCntType>;
impl Hdcp22SwAxiCntTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdcp22SwAxiCntType {
        match self.bits {
            false => Hdcp22SwAxiCntType::B0,
            true => Hdcp22SwAxiCntType::B1,
        }
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hdcp22SwAxiCntType::B0
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hdcp22SwAxiCntType::B1
    }
}
#[doc = "Field `HDCP22_SW_AXI_CNT_TYPE` writer - axi_perf counter type"]
pub type Hdcp22SwAxiCntTypeW<'a, REG> = crate::BitWriter<'a, REG, Hdcp22SwAxiCntType>;
impl<'a, REG> Hdcp22SwAxiCntTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22SwAxiCntType::B0)
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22SwAxiCntType::B1)
    }
}
#[doc = "axi_perf counter id control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdcp22SwArCntIdType {
    #[doc = "0: count sw_ar_count_id read channel only"]
    B0 = 0,
    #[doc = "1: count sw_ar_count_id read channel only"]
    B1 = 1,
}
impl From<Hdcp22SwArCntIdType> for bool {
    #[inline(always)]
    fn from(variant: Hdcp22SwArCntIdType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCP22_SW_AR_CNT_ID_TYPE` reader - axi_perf counter id control"]
pub type Hdcp22SwArCntIdTypeR = crate::BitReader<Hdcp22SwArCntIdType>;
impl Hdcp22SwArCntIdTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdcp22SwArCntIdType {
        match self.bits {
            false => Hdcp22SwArCntIdType::B0,
            true => Hdcp22SwArCntIdType::B1,
        }
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hdcp22SwArCntIdType::B0
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hdcp22SwArCntIdType::B1
    }
}
#[doc = "Field `HDCP22_SW_AR_CNT_ID_TYPE` writer - axi_perf counter id control"]
pub type Hdcp22SwArCntIdTypeW<'a, REG> = crate::BitWriter<'a, REG, Hdcp22SwArCntIdType>;
impl<'a, REG> Hdcp22SwArCntIdTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22SwArCntIdType::B0)
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22SwArCntIdType::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdcp22SwAwCntIdType {
    #[doc = "0: count sw_aw_count_id write channel only"]
    B0 = 0,
    #[doc = "1: count sw_aw_count_id write channel only"]
    B1 = 1,
}
impl From<Hdcp22SwAwCntIdType> for bool {
    #[inline(always)]
    fn from(variant: Hdcp22SwAwCntIdType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCP22_SW_AW_CNT_ID_TYPE` reader - "]
pub type Hdcp22SwAwCntIdTypeR = crate::BitReader<Hdcp22SwAwCntIdType>;
impl Hdcp22SwAwCntIdTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdcp22SwAwCntIdType {
        match self.bits {
            false => Hdcp22SwAwCntIdType::B0,
            true => Hdcp22SwAwCntIdType::B1,
        }
    }
    #[doc = "count sw_aw_count_id write channel only"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hdcp22SwAwCntIdType::B0
    }
    #[doc = "count sw_aw_count_id write channel only"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hdcp22SwAwCntIdType::B1
    }
}
#[doc = "Field `HDCP22_SW_AW_CNT_ID_TYPE` writer - "]
pub type Hdcp22SwAwCntIdTypeW<'a, REG> = crate::BitWriter<'a, REG, Hdcp22SwAwCntIdType>;
impl<'a, REG> Hdcp22SwAwCntIdTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "count sw_aw_count_id write channel only"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22SwAwCntIdType::B0)
    }
    #[doc = "count sw_aw_count_id write channel only"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22SwAwCntIdType::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hdcp22SwDdrAlignType {
    #[doc = "0: 128-Byte align"]
    D0 = 0,
    #[doc = "1: 128-Byte align"]
    D1 = 1,
    #[doc = "2: 128-Byte align"]
    D2 = 2,
    #[doc = "3: 128-Byte align"]
    D3 = 3,
}
impl From<Hdcp22SwDdrAlignType> for u8 {
    #[inline(always)]
    fn from(variant: Hdcp22SwDdrAlignType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hdcp22SwDdrAlignType {
    type Ux = u8;
}
#[doc = "Field `HDCP22_SW_DDR_ALIGN_TYPE` reader - "]
pub type Hdcp22SwDdrAlignTypeR = crate::FieldReader<Hdcp22SwDdrAlignType>;
impl Hdcp22SwDdrAlignTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdcp22SwDdrAlignType {
        match self.bits {
            0 => Hdcp22SwDdrAlignType::D0,
            1 => Hdcp22SwDdrAlignType::D1,
            2 => Hdcp22SwDdrAlignType::D2,
            3 => Hdcp22SwDdrAlignType::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Hdcp22SwDdrAlignType::D0
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Hdcp22SwDdrAlignType::D1
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Hdcp22SwDdrAlignType::D2
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Hdcp22SwDdrAlignType::D3
    }
}
#[doc = "Field `HDCP22_SW_DDR_ALIGN_TYPE` writer - "]
pub type Hdcp22SwDdrAlignTypeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Hdcp22SwDdrAlignType>;
impl<'a, REG> Hdcp22SwDdrAlignTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22SwDdrAlignType::D0)
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22SwDdrAlignType::D1)
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22SwDdrAlignType::D2)
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcp22SwDdrAlignType::D3)
    }
}
#[doc = "Field `HDCP22_SW_RD_LATENCY_ID` reader - Axi read channel id for latency AXI_PERFormance test"]
pub type Hdcp22SwRdLatencyIdR = crate::FieldReader;
#[doc = "Field `HDCP22_SW_RD_LATENCY_ID` writer - Axi read channel id for latency AXI_PERFormance test"]
pub type Hdcp22SwRdLatencyIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - axi_perf enable bit"]
    #[inline(always)]
    pub fn hdcp22_sw_axi_perf_work(&self) -> Hdcp22SwAxiPerfWorkR {
        Hdcp22SwAxiPerfWorkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - axi_perf clear bit"]
    #[inline(always)]
    pub fn hdcp22_sw_axi_perf_clr(&self) -> Hdcp22SwAxiPerfClrR {
        Hdcp22SwAxiPerfClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - axi_perf counter type"]
    #[inline(always)]
    pub fn hdcp22_sw_axi_cnt_type(&self) -> Hdcp22SwAxiCntTypeR {
        Hdcp22SwAxiCntTypeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - axi_perf counter id control"]
    #[inline(always)]
    pub fn hdcp22_sw_ar_cnt_id_type(&self) -> Hdcp22SwArCntIdTypeR {
        Hdcp22SwArCntIdTypeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn hdcp22_sw_aw_cnt_id_type(&self) -> Hdcp22SwAwCntIdTypeR {
        Hdcp22SwAwCntIdTypeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn hdcp22_sw_ddr_align_type(&self) -> Hdcp22SwDdrAlignTypeR {
        Hdcp22SwDdrAlignTypeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Axi read channel id for latency AXI_PERFormance test"]
    #[inline(always)]
    pub fn hdcp22_sw_rd_latency_id(&self) -> Hdcp22SwRdLatencyIdR {
        Hdcp22SwRdLatencyIdR::new(((self.bits >> 8) & 0x0f) as u8)
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
    pub fn hdcp22_sw_axi_perf_work(&mut self) -> Hdcp22SwAxiPerfWorkW<GrfHdcp22PerfCon0Spec> {
        Hdcp22SwAxiPerfWorkW::new(self, 0)
    }
    #[doc = "Bit 1 - axi_perf clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp22_sw_axi_perf_clr(&mut self) -> Hdcp22SwAxiPerfClrW<GrfHdcp22PerfCon0Spec> {
        Hdcp22SwAxiPerfClrW::new(self, 1)
    }
    #[doc = "Bit 2 - axi_perf counter type"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp22_sw_axi_cnt_type(&mut self) -> Hdcp22SwAxiCntTypeW<GrfHdcp22PerfCon0Spec> {
        Hdcp22SwAxiCntTypeW::new(self, 2)
    }
    #[doc = "Bit 3 - axi_perf counter id control"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp22_sw_ar_cnt_id_type(&mut self) -> Hdcp22SwArCntIdTypeW<GrfHdcp22PerfCon0Spec> {
        Hdcp22SwArCntIdTypeW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp22_sw_aw_cnt_id_type(&mut self) -> Hdcp22SwAwCntIdTypeW<GrfHdcp22PerfCon0Spec> {
        Hdcp22SwAwCntIdTypeW::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp22_sw_ddr_align_type(&mut self) -> Hdcp22SwDdrAlignTypeW<GrfHdcp22PerfCon0Spec> {
        Hdcp22SwDdrAlignTypeW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Axi read channel id for latency AXI_PERFormance test"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp22_sw_rd_latency_id(&mut self) -> Hdcp22SwRdLatencyIdW<GrfHdcp22PerfCon0Spec> {
        Hdcp22SwRdLatencyIdW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfHdcp22PerfCon0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "hdcp performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_hdcp22_perf_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_hdcp22_perf_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfHdcp22PerfCon0Spec;
impl crate::RegisterSpec for GrfHdcp22PerfCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_hdcp22_perf_con0::R`](R) reader structure"]
impl crate::Readable for GrfHdcp22PerfCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_hdcp22_perf_con0::W`](W) writer structure"]
impl crate::Writable for GrfHdcp22PerfCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_HDCP22_PERF_CON0 to value 0"]
impl crate::Resettable for GrfHdcp22PerfCon0Spec {
    const RESET_VALUE: u32 = 0;
}
