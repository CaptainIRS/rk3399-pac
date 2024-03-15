#[doc = "Register `GRF_PCIE_PERF_CON0` reader"]
pub type R = crate::R<GrfPciePerfCon0Spec>;
#[doc = "Register `GRF_PCIE_PERF_CON0` writer"]
pub type W = crate::W<GrfPciePerfCon0Spec>;
#[doc = "axi_perf enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcieSwAxiPerfWork {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PcieSwAxiPerfWork> for bool {
    #[inline(always)]
    fn from(variant: PcieSwAxiPerfWork) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCIE_SW_AXI_PERF_WORK` reader - axi_perf enable bit"]
pub type PcieSwAxiPerfWorkR = crate::BitReader<PcieSwAxiPerfWork>;
impl PcieSwAxiPerfWorkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PcieSwAxiPerfWork {
        match self.bits {
            false => PcieSwAxiPerfWork::B0,
            true => PcieSwAxiPerfWork::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PcieSwAxiPerfWork::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PcieSwAxiPerfWork::B1
    }
}
#[doc = "Field `PCIE_SW_AXI_PERF_WORK` writer - axi_perf enable bit"]
pub type PcieSwAxiPerfWorkW<'a, REG> = crate::BitWriter<'a, REG, PcieSwAxiPerfWork>;
impl<'a, REG> PcieSwAxiPerfWorkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PcieSwAxiPerfWork::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PcieSwAxiPerfWork::B1)
    }
}
#[doc = "axi_perf clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcieSwAxiPerfClr {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<PcieSwAxiPerfClr> for bool {
    #[inline(always)]
    fn from(variant: PcieSwAxiPerfClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCIE_SW_AXI_PERF_CLR` reader - axi_perf clear bit"]
pub type PcieSwAxiPerfClrR = crate::BitReader<PcieSwAxiPerfClr>;
impl PcieSwAxiPerfClrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PcieSwAxiPerfClr {
        match self.bits {
            false => PcieSwAxiPerfClr::B0,
            true => PcieSwAxiPerfClr::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PcieSwAxiPerfClr::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PcieSwAxiPerfClr::B1
    }
}
#[doc = "Field `PCIE_SW_AXI_PERF_CLR` writer - axi_perf clear bit"]
pub type PcieSwAxiPerfClrW<'a, REG> = crate::BitWriter<'a, REG, PcieSwAxiPerfClr>;
impl<'a, REG> PcieSwAxiPerfClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PcieSwAxiPerfClr::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PcieSwAxiPerfClr::B1)
    }
}
#[doc = "axi_perf counter type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcieSwAxiCntType {
    #[doc = "0: ddr align transfer test"]
    B0 = 0,
    #[doc = "1: ddr align transfer test"]
    B1 = 1,
}
impl From<PcieSwAxiCntType> for bool {
    #[inline(always)]
    fn from(variant: PcieSwAxiCntType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCIE_SW_AXI_CNT_TYPE` reader - axi_perf counter type"]
pub type PcieSwAxiCntTypeR = crate::BitReader<PcieSwAxiCntType>;
impl PcieSwAxiCntTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PcieSwAxiCntType {
        match self.bits {
            false => PcieSwAxiCntType::B0,
            true => PcieSwAxiCntType::B1,
        }
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PcieSwAxiCntType::B0
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PcieSwAxiCntType::B1
    }
}
#[doc = "Field `PCIE_SW_AXI_CNT_TYPE` writer - axi_perf counter type"]
pub type PcieSwAxiCntTypeW<'a, REG> = crate::BitWriter<'a, REG, PcieSwAxiCntType>;
impl<'a, REG> PcieSwAxiCntTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PcieSwAxiCntType::B0)
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PcieSwAxiCntType::B1)
    }
}
#[doc = "axi_perf counter id control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcieSwArCntIdType {
    #[doc = "0: count sw_ar_count_id read channel only"]
    B0 = 0,
    #[doc = "1: count sw_ar_count_id read channel only"]
    B1 = 1,
}
impl From<PcieSwArCntIdType> for bool {
    #[inline(always)]
    fn from(variant: PcieSwArCntIdType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCIE_SW_AR_CNT_ID_TYPE` reader - axi_perf counter id control"]
pub type PcieSwArCntIdTypeR = crate::BitReader<PcieSwArCntIdType>;
impl PcieSwArCntIdTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PcieSwArCntIdType {
        match self.bits {
            false => PcieSwArCntIdType::B0,
            true => PcieSwArCntIdType::B1,
        }
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PcieSwArCntIdType::B0
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PcieSwArCntIdType::B1
    }
}
#[doc = "Field `PCIE_SW_AR_CNT_ID_TYPE` writer - axi_perf counter id control"]
pub type PcieSwArCntIdTypeW<'a, REG> = crate::BitWriter<'a, REG, PcieSwArCntIdType>;
impl<'a, REG> PcieSwArCntIdTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PcieSwArCntIdType::B0)
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PcieSwArCntIdType::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcieSwAwCntIdType {
    #[doc = "0: count sw_aw_count_id write channel only"]
    B0 = 0,
    #[doc = "1: count sw_aw_count_id write channel only"]
    B1 = 1,
}
impl From<PcieSwAwCntIdType> for bool {
    #[inline(always)]
    fn from(variant: PcieSwAwCntIdType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCIE_SW_AW_CNT_ID_TYPE` reader - "]
pub type PcieSwAwCntIdTypeR = crate::BitReader<PcieSwAwCntIdType>;
impl PcieSwAwCntIdTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PcieSwAwCntIdType {
        match self.bits {
            false => PcieSwAwCntIdType::B0,
            true => PcieSwAwCntIdType::B1,
        }
    }
    #[doc = "count sw_aw_count_id write channel only"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PcieSwAwCntIdType::B0
    }
    #[doc = "count sw_aw_count_id write channel only"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PcieSwAwCntIdType::B1
    }
}
#[doc = "Field `PCIE_SW_AW_CNT_ID_TYPE` writer - "]
pub type PcieSwAwCntIdTypeW<'a, REG> = crate::BitWriter<'a, REG, PcieSwAwCntIdType>;
impl<'a, REG> PcieSwAwCntIdTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "count sw_aw_count_id write channel only"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PcieSwAwCntIdType::B0)
    }
    #[doc = "count sw_aw_count_id write channel only"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PcieSwAwCntIdType::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PcieSwDdrAlignType {
    #[doc = "0: 128-Byte align"]
    D0 = 0,
    #[doc = "1: 128-Byte align"]
    D1 = 1,
    #[doc = "2: 128-Byte align"]
    D2 = 2,
    #[doc = "3: 128-Byte align"]
    D3 = 3,
}
impl From<PcieSwDdrAlignType> for u8 {
    #[inline(always)]
    fn from(variant: PcieSwDdrAlignType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PcieSwDdrAlignType {
    type Ux = u8;
}
#[doc = "Field `PCIE_SW_DDR_ALIGN_TYPE` reader - "]
pub type PcieSwDdrAlignTypeR = crate::FieldReader<PcieSwDdrAlignType>;
impl PcieSwDdrAlignTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PcieSwDdrAlignType {
        match self.bits {
            0 => PcieSwDdrAlignType::D0,
            1 => PcieSwDdrAlignType::D1,
            2 => PcieSwDdrAlignType::D2,
            3 => PcieSwDdrAlignType::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == PcieSwDdrAlignType::D0
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == PcieSwDdrAlignType::D1
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == PcieSwDdrAlignType::D2
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == PcieSwDdrAlignType::D3
    }
}
#[doc = "Field `PCIE_SW_DDR_ALIGN_TYPE` writer - "]
pub type PcieSwDdrAlignTypeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PcieSwDdrAlignType>;
impl<'a, REG> PcieSwDdrAlignTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(PcieSwDdrAlignType::D0)
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(PcieSwDdrAlignType::D1)
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(PcieSwDdrAlignType::D2)
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(PcieSwDdrAlignType::D3)
    }
}
#[doc = "Field `PCIE_SW_RD_LATENCY_ID` reader - Axi read channel id for latency AXI_PERFormance test"]
pub type PcieSwRdLatencyIdR = crate::FieldReader;
#[doc = "Field `PCIE_SW_RD_LATENCY_ID` writer - Axi read channel id for latency AXI_PERFormance test"]
pub type PcieSwRdLatencyIdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - axi_perf enable bit"]
    #[inline(always)]
    pub fn pcie_sw_axi_perf_work(&self) -> PcieSwAxiPerfWorkR {
        PcieSwAxiPerfWorkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - axi_perf clear bit"]
    #[inline(always)]
    pub fn pcie_sw_axi_perf_clr(&self) -> PcieSwAxiPerfClrR {
        PcieSwAxiPerfClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - axi_perf counter type"]
    #[inline(always)]
    pub fn pcie_sw_axi_cnt_type(&self) -> PcieSwAxiCntTypeR {
        PcieSwAxiCntTypeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - axi_perf counter id control"]
    #[inline(always)]
    pub fn pcie_sw_ar_cnt_id_type(&self) -> PcieSwArCntIdTypeR {
        PcieSwArCntIdTypeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pcie_sw_aw_cnt_id_type(&self) -> PcieSwAwCntIdTypeR {
        PcieSwAwCntIdTypeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn pcie_sw_ddr_align_type(&self) -> PcieSwDdrAlignTypeR {
        PcieSwDdrAlignTypeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Axi read channel id for latency AXI_PERFormance test"]
    #[inline(always)]
    pub fn pcie_sw_rd_latency_id(&self) -> PcieSwRdLatencyIdR {
        PcieSwRdLatencyIdR::new(((self.bits >> 8) & 0x1f) as u8)
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
    pub fn pcie_sw_axi_perf_work(&mut self) -> PcieSwAxiPerfWorkW<GrfPciePerfCon0Spec> {
        PcieSwAxiPerfWorkW::new(self, 0)
    }
    #[doc = "Bit 1 - axi_perf clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_sw_axi_perf_clr(&mut self) -> PcieSwAxiPerfClrW<GrfPciePerfCon0Spec> {
        PcieSwAxiPerfClrW::new(self, 1)
    }
    #[doc = "Bit 2 - axi_perf counter type"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_sw_axi_cnt_type(&mut self) -> PcieSwAxiCntTypeW<GrfPciePerfCon0Spec> {
        PcieSwAxiCntTypeW::new(self, 2)
    }
    #[doc = "Bit 3 - axi_perf counter id control"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_sw_ar_cnt_id_type(&mut self) -> PcieSwArCntIdTypeW<GrfPciePerfCon0Spec> {
        PcieSwArCntIdTypeW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_sw_aw_cnt_id_type(&mut self) -> PcieSwAwCntIdTypeW<GrfPciePerfCon0Spec> {
        PcieSwAwCntIdTypeW::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_sw_ddr_align_type(&mut self) -> PcieSwDdrAlignTypeW<GrfPciePerfCon0Spec> {
        PcieSwDdrAlignTypeW::new(self, 5)
    }
    #[doc = "Bits 8:12 - Axi read channel id for latency AXI_PERFormance test"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_sw_rd_latency_id(&mut self) -> PcieSwRdLatencyIdW<GrfPciePerfCon0Spec> {
        PcieSwRdLatencyIdW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfPciePerfCon0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "pcie performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_pcie_perf_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_pcie_perf_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfPciePerfCon0Spec;
impl crate::RegisterSpec for GrfPciePerfCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_pcie_perf_con0::R`](R) reader structure"]
impl crate::Readable for GrfPciePerfCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_pcie_perf_con0::W`](W) writer structure"]
impl crate::Writable for GrfPciePerfCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_PCIE_PERF_CON0 to value 0"]
impl crate::Resettable for GrfPciePerfCon0Spec {
    const RESET_VALUE: u32 = 0;
}
