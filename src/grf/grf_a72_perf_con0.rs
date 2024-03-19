#[doc = "Register `GRF_A72_PERF_CON0` reader"]
pub type R = crate::R<GrfA72PerfCon0Spec>;
#[doc = "Register `GRF_A72_PERF_CON0` writer"]
pub type W = crate::W<GrfA72PerfCon0Spec>;
#[doc = "axi_perf enable bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A72SwAxiPerfWork {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<A72SwAxiPerfWork> for bool {
    #[inline(always)]
    fn from(variant: A72SwAxiPerfWork) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A72_SW_AXI_PERF_WORK` reader - axi_perf enable bit"]
pub type A72SwAxiPerfWorkR = crate::BitReader<A72SwAxiPerfWork>;
impl A72SwAxiPerfWorkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A72SwAxiPerfWork {
        match self.bits {
            false => A72SwAxiPerfWork::B0,
            true => A72SwAxiPerfWork::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == A72SwAxiPerfWork::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == A72SwAxiPerfWork::B1
    }
}
#[doc = "Field `A72_SW_AXI_PERF_WORK` writer - axi_perf enable bit"]
pub type A72SwAxiPerfWorkW<'a, REG> = crate::BitWriter<'a, REG, A72SwAxiPerfWork>;
impl<'a, REG> A72SwAxiPerfWorkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(A72SwAxiPerfWork::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(A72SwAxiPerfWork::B1)
    }
}
#[doc = "axi_perf clear bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A72SwAxiPerfClr {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<A72SwAxiPerfClr> for bool {
    #[inline(always)]
    fn from(variant: A72SwAxiPerfClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A72_SW_AXI_PERF_CLR` reader - axi_perf clear bit"]
pub type A72SwAxiPerfClrR = crate::BitReader<A72SwAxiPerfClr>;
impl A72SwAxiPerfClrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A72SwAxiPerfClr {
        match self.bits {
            false => A72SwAxiPerfClr::B0,
            true => A72SwAxiPerfClr::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == A72SwAxiPerfClr::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == A72SwAxiPerfClr::B1
    }
}
#[doc = "Field `A72_SW_AXI_PERF_CLR` writer - axi_perf clear bit"]
pub type A72SwAxiPerfClrW<'a, REG> = crate::BitWriter<'a, REG, A72SwAxiPerfClr>;
impl<'a, REG> A72SwAxiPerfClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(A72SwAxiPerfClr::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(A72SwAxiPerfClr::B1)
    }
}
#[doc = "axi_perf counter type\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A72SwAxiCntType {
    #[doc = "0: axi transfer test"]
    B0 = 0,
    #[doc = "1: ddr align transfer test"]
    B1 = 1,
}
impl From<A72SwAxiCntType> for bool {
    #[inline(always)]
    fn from(variant: A72SwAxiCntType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A72_SW_AXI_CNT_TYPE` reader - axi_perf counter type"]
pub type A72SwAxiCntTypeR = crate::BitReader<A72SwAxiCntType>;
impl A72SwAxiCntTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A72SwAxiCntType {
        match self.bits {
            false => A72SwAxiCntType::B0,
            true => A72SwAxiCntType::B1,
        }
    }
    #[doc = "axi transfer test"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == A72SwAxiCntType::B0
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == A72SwAxiCntType::B1
    }
}
#[doc = "Field `A72_SW_AXI_CNT_TYPE` writer - axi_perf counter type"]
pub type A72SwAxiCntTypeW<'a, REG> = crate::BitWriter<'a, REG, A72SwAxiCntType>;
impl<'a, REG> A72SwAxiCntTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "axi transfer test"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(A72SwAxiCntType::B0)
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(A72SwAxiCntType::B1)
    }
}
#[doc = "axi_perf counter id control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A72SwArCntIdType {
    #[doc = "0: count all read channel id"]
    B0 = 0,
    #[doc = "1: count sw_ar_count_id read channel only"]
    B1 = 1,
}
impl From<A72SwArCntIdType> for bool {
    #[inline(always)]
    fn from(variant: A72SwArCntIdType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A72_SW_AR_CNT_ID_TYPE` reader - axi_perf counter id control"]
pub type A72SwArCntIdTypeR = crate::BitReader<A72SwArCntIdType>;
impl A72SwArCntIdTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A72SwArCntIdType {
        match self.bits {
            false => A72SwArCntIdType::B0,
            true => A72SwArCntIdType::B1,
        }
    }
    #[doc = "count all read channel id"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == A72SwArCntIdType::B0
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == A72SwArCntIdType::B1
    }
}
#[doc = "Field `A72_SW_AR_CNT_ID_TYPE` writer - axi_perf counter id control"]
pub type A72SwArCntIdTypeW<'a, REG> = crate::BitWriter<'a, REG, A72SwArCntIdType>;
impl<'a, REG> A72SwArCntIdTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "count all read channel id"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(A72SwArCntIdType::B0)
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(A72SwArCntIdType::B1)
    }
}
#[doc = "axi_perf counter id control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A72SwAwCntIdType {
    #[doc = "0: count all write channel id"]
    B0 = 0,
    #[doc = "1: count sw_ar_count_id write channel only"]
    B1 = 1,
}
impl From<A72SwAwCntIdType> for bool {
    #[inline(always)]
    fn from(variant: A72SwAwCntIdType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A72_SW_AW_CNT_ID_TYPE` reader - axi_perf counter id control"]
pub type A72SwAwCntIdTypeR = crate::BitReader<A72SwAwCntIdType>;
impl A72SwAwCntIdTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A72SwAwCntIdType {
        match self.bits {
            false => A72SwAwCntIdType::B0,
            true => A72SwAwCntIdType::B1,
        }
    }
    #[doc = "count all write channel id"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == A72SwAwCntIdType::B0
    }
    #[doc = "count sw_ar_count_id write channel only"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == A72SwAwCntIdType::B1
    }
}
#[doc = "Field `A72_SW_AW_CNT_ID_TYPE` writer - axi_perf counter id control"]
pub type A72SwAwCntIdTypeW<'a, REG> = crate::BitWriter<'a, REG, A72SwAwCntIdType>;
impl<'a, REG> A72SwAwCntIdTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "count all write channel id"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(A72SwAwCntIdType::B0)
    }
    #[doc = "count sw_ar_count_id write channel only"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(A72SwAwCntIdType::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum A72SwDdrAlignType {
    #[doc = "0: 16-Byte align"]
    D0 = 0,
    #[doc = "1: 32-Byte align"]
    D1 = 1,
    #[doc = "2: 64-Byte align"]
    D2 = 2,
    #[doc = "3: 128-Byte align"]
    D3 = 3,
}
impl From<A72SwDdrAlignType> for u8 {
    #[inline(always)]
    fn from(variant: A72SwDdrAlignType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for A72SwDdrAlignType {
    type Ux = u8;
}
#[doc = "Field `A72_SW_DDR_ALIGN_TYPE` reader - "]
pub type A72SwDdrAlignTypeR = crate::FieldReader<A72SwDdrAlignType>;
impl A72SwDdrAlignTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A72SwDdrAlignType {
        match self.bits {
            0 => A72SwDdrAlignType::D0,
            1 => A72SwDdrAlignType::D1,
            2 => A72SwDdrAlignType::D2,
            3 => A72SwDdrAlignType::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "16-Byte align"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == A72SwDdrAlignType::D0
    }
    #[doc = "32-Byte align"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == A72SwDdrAlignType::D1
    }
    #[doc = "64-Byte align"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == A72SwDdrAlignType::D2
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == A72SwDdrAlignType::D3
    }
}
#[doc = "Field `A72_SW_DDR_ALIGN_TYPE` writer - "]
pub type A72SwDdrAlignTypeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, A72SwDdrAlignType>;
impl<'a, REG> A72SwDdrAlignTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-Byte align"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(A72SwDdrAlignType::D0)
    }
    #[doc = "32-Byte align"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(A72SwDdrAlignType::D1)
    }
    #[doc = "64-Byte align"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(A72SwDdrAlignType::D2)
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(A72SwDdrAlignType::D3)
    }
}
#[doc = "Field `A72_SW_RD_LATENCY_ID` reader - Axi read channel id for latency\n\nAXI_PERFormance test"]
pub type A72SwRdLatencyIdR = crate::FieldReader;
#[doc = "Field `A72_SW_RD_LATENCY_ID` writer - Axi read channel id for latency\n\nAXI_PERFormance test"]
pub type A72SwRdLatencyIdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - axi_perf enable bit"]
    #[inline(always)]
    pub fn a72_sw_axi_perf_work(&self) -> A72SwAxiPerfWorkR {
        A72SwAxiPerfWorkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - axi_perf clear bit"]
    #[inline(always)]
    pub fn a72_sw_axi_perf_clr(&self) -> A72SwAxiPerfClrR {
        A72SwAxiPerfClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - axi_perf counter type"]
    #[inline(always)]
    pub fn a72_sw_axi_cnt_type(&self) -> A72SwAxiCntTypeR {
        A72SwAxiCntTypeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - axi_perf counter id control"]
    #[inline(always)]
    pub fn a72_sw_ar_cnt_id_type(&self) -> A72SwArCntIdTypeR {
        A72SwArCntIdTypeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - axi_perf counter id control"]
    #[inline(always)]
    pub fn a72_sw_aw_cnt_id_type(&self) -> A72SwAwCntIdTypeR {
        A72SwAwCntIdTypeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn a72_sw_ddr_align_type(&self) -> A72SwDdrAlignTypeR {
        A72SwDdrAlignTypeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:14 - Axi read channel id for latency\n\nAXI_PERFormance test"]
    #[inline(always)]
    pub fn a72_sw_rd_latency_id(&self) -> A72SwRdLatencyIdR {
        A72SwRdLatencyIdR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - axi_perf enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn a72_sw_axi_perf_work(&mut self) -> A72SwAxiPerfWorkW<GrfA72PerfCon0Spec> {
        A72SwAxiPerfWorkW::new(self, 0)
    }
    #[doc = "Bit 1 - axi_perf clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn a72_sw_axi_perf_clr(&mut self) -> A72SwAxiPerfClrW<GrfA72PerfCon0Spec> {
        A72SwAxiPerfClrW::new(self, 1)
    }
    #[doc = "Bit 2 - axi_perf counter type"]
    #[inline(always)]
    #[must_use]
    pub fn a72_sw_axi_cnt_type(&mut self) -> A72SwAxiCntTypeW<GrfA72PerfCon0Spec> {
        A72SwAxiCntTypeW::new(self, 2)
    }
    #[doc = "Bit 3 - axi_perf counter id control"]
    #[inline(always)]
    #[must_use]
    pub fn a72_sw_ar_cnt_id_type(&mut self) -> A72SwArCntIdTypeW<GrfA72PerfCon0Spec> {
        A72SwArCntIdTypeW::new(self, 3)
    }
    #[doc = "Bit 4 - axi_perf counter id control"]
    #[inline(always)]
    #[must_use]
    pub fn a72_sw_aw_cnt_id_type(&mut self) -> A72SwAwCntIdTypeW<GrfA72PerfCon0Spec> {
        A72SwAwCntIdTypeW::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn a72_sw_ddr_align_type(&mut self) -> A72SwDdrAlignTypeW<GrfA72PerfCon0Spec> {
        A72SwDdrAlignTypeW::new(self, 5)
    }
    #[doc = "Bits 8:14 - Axi read channel id for latency\n\nAXI_PERFormance test"]
    #[inline(always)]
    #[must_use]
    pub fn a72_sw_rd_latency_id(&mut self) -> A72SwRdLatencyIdW<GrfA72PerfCon0Spec> {
        A72SwRdLatencyIdW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfA72PerfCon0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "a72 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a72_perf_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a72_perf_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfA72PerfCon0Spec;
impl crate::RegisterSpec for GrfA72PerfCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_a72_perf_con0::R`](R) reader structure"]
impl crate::Readable for GrfA72PerfCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_a72_perf_con0::W`](W) writer structure"]
impl crate::Writable for GrfA72PerfCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_A72_PERF_CON0 to value 0"]
impl crate::Resettable for GrfA72PerfCon0Spec {
    const RESET_VALUE: u32 = 0;
}
