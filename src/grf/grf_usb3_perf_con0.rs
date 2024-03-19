#[doc = "Register `GRF_USB3_PERF_CON0` reader"]
pub type R = crate::R<GrfUsb3PerfCon0Spec>;
#[doc = "Register `GRF_USB3_PERF_CON0` writer"]
pub type W = crate::W<GrfUsb3PerfCon0Spec>;
#[doc = "axi_perf enable bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb3SwAxiPerfWork {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Usb3SwAxiPerfWork> for bool {
    #[inline(always)]
    fn from(variant: Usb3SwAxiPerfWork) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB3_SW_AXI_PERF_WORK` reader - axi_perf enable bit"]
pub type Usb3SwAxiPerfWorkR = crate::BitReader<Usb3SwAxiPerfWork>;
impl Usb3SwAxiPerfWorkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb3SwAxiPerfWork {
        match self.bits {
            false => Usb3SwAxiPerfWork::B0,
            true => Usb3SwAxiPerfWork::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usb3SwAxiPerfWork::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usb3SwAxiPerfWork::B1
    }
}
#[doc = "Field `USB3_SW_AXI_PERF_WORK` writer - axi_perf enable bit"]
pub type Usb3SwAxiPerfWorkW<'a, REG> = crate::BitWriter<'a, REG, Usb3SwAxiPerfWork>;
impl<'a, REG> Usb3SwAxiPerfWorkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3SwAxiPerfWork::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3SwAxiPerfWork::B1)
    }
}
#[doc = "Fi\n\naxi_perf clear bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb3SwAxiPerfClr {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Usb3SwAxiPerfClr> for bool {
    #[inline(always)]
    fn from(variant: Usb3SwAxiPerfClr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB3_SW_AXI_PERF_CLR` reader - Fi\n\naxi_perf clear bit"]
pub type Usb3SwAxiPerfClrR = crate::BitReader<Usb3SwAxiPerfClr>;
impl Usb3SwAxiPerfClrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb3SwAxiPerfClr {
        match self.bits {
            false => Usb3SwAxiPerfClr::B0,
            true => Usb3SwAxiPerfClr::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usb3SwAxiPerfClr::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usb3SwAxiPerfClr::B1
    }
}
#[doc = "Field `USB3_SW_AXI_PERF_CLR` writer - Fi\n\naxi_perf clear bit"]
pub type Usb3SwAxiPerfClrW<'a, REG> = crate::BitWriter<'a, REG, Usb3SwAxiPerfClr>;
impl<'a, REG> Usb3SwAxiPerfClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3SwAxiPerfClr::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3SwAxiPerfClr::B1)
    }
}
#[doc = "axi_perf counter type\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb3SwAxiCntType {
    #[doc = "0: axi transfer test"]
    B0 = 0,
    #[doc = "1: ddr align transfer test"]
    B1 = 1,
}
impl From<Usb3SwAxiCntType> for bool {
    #[inline(always)]
    fn from(variant: Usb3SwAxiCntType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB3_SW_AXI_CNT_TYPE` reader - axi_perf counter type"]
pub type Usb3SwAxiCntTypeR = crate::BitReader<Usb3SwAxiCntType>;
impl Usb3SwAxiCntTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb3SwAxiCntType {
        match self.bits {
            false => Usb3SwAxiCntType::B0,
            true => Usb3SwAxiCntType::B1,
        }
    }
    #[doc = "axi transfer test"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usb3SwAxiCntType::B0
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usb3SwAxiCntType::B1
    }
}
#[doc = "Field `USB3_SW_AXI_CNT_TYPE` writer - axi_perf counter type"]
pub type Usb3SwAxiCntTypeW<'a, REG> = crate::BitWriter<'a, REG, Usb3SwAxiCntType>;
impl<'a, REG> Usb3SwAxiCntTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "axi transfer test"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3SwAxiCntType::B0)
    }
    #[doc = "ddr align transfer test"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3SwAxiCntType::B1)
    }
}
#[doc = "axi_perf counter id control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb3SwArCntIdType {
    #[doc = "0: count all read channel id"]
    B0 = 0,
    #[doc = "1: count sw_ar_count_id read channel only"]
    B1 = 1,
}
impl From<Usb3SwArCntIdType> for bool {
    #[inline(always)]
    fn from(variant: Usb3SwArCntIdType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB3_SW_AR_CNT_ID_TYPE` reader - axi_perf counter id control"]
pub type Usb3SwArCntIdTypeR = crate::BitReader<Usb3SwArCntIdType>;
impl Usb3SwArCntIdTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb3SwArCntIdType {
        match self.bits {
            false => Usb3SwArCntIdType::B0,
            true => Usb3SwArCntIdType::B1,
        }
    }
    #[doc = "count all read channel id"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usb3SwArCntIdType::B0
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usb3SwArCntIdType::B1
    }
}
#[doc = "Field `USB3_SW_AR_CNT_ID_TYPE` writer - axi_perf counter id control"]
pub type Usb3SwArCntIdTypeW<'a, REG> = crate::BitWriter<'a, REG, Usb3SwArCntIdType>;
impl<'a, REG> Usb3SwArCntIdTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "count all read channel id"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3SwArCntIdType::B0)
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3SwArCntIdType::B1)
    }
}
#[doc = "axi_perf counter id control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb3SwAwCntIdType {
    #[doc = "0: count all read channel id"]
    B0 = 0,
    #[doc = "1: count sw_ar_count_id read channel only"]
    B1 = 1,
}
impl From<Usb3SwAwCntIdType> for bool {
    #[inline(always)]
    fn from(variant: Usb3SwAwCntIdType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB3_SW_AW_CNT_ID_TYPE` reader - axi_perf counter id control"]
pub type Usb3SwAwCntIdTypeR = crate::BitReader<Usb3SwAwCntIdType>;
impl Usb3SwAwCntIdTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb3SwAwCntIdType {
        match self.bits {
            false => Usb3SwAwCntIdType::B0,
            true => Usb3SwAwCntIdType::B1,
        }
    }
    #[doc = "count all read channel id"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usb3SwAwCntIdType::B0
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usb3SwAwCntIdType::B1
    }
}
#[doc = "Field `USB3_SW_AW_CNT_ID_TYPE` writer - axi_perf counter id control"]
pub type Usb3SwAwCntIdTypeW<'a, REG> = crate::BitWriter<'a, REG, Usb3SwAwCntIdType>;
impl<'a, REG> Usb3SwAwCntIdTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "count all read channel id"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3SwAwCntIdType::B0)
    }
    #[doc = "count sw_ar_count_id read channel only"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3SwAwCntIdType::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usb3SwDdrAlignType {
    #[doc = "0: 16-Byte align"]
    D0 = 0,
    #[doc = "1: 32-Byte align"]
    D1 = 1,
    #[doc = "2: 64-Byte align"]
    D2 = 2,
    #[doc = "3: 128-Byte align"]
    D3 = 3,
}
impl From<Usb3SwDdrAlignType> for u8 {
    #[inline(always)]
    fn from(variant: Usb3SwDdrAlignType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usb3SwDdrAlignType {
    type Ux = u8;
}
#[doc = "Field `USB3_SW_DDR_ALIGN_TYPE` reader - "]
pub type Usb3SwDdrAlignTypeR = crate::FieldReader<Usb3SwDdrAlignType>;
impl Usb3SwDdrAlignTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb3SwDdrAlignType {
        match self.bits {
            0 => Usb3SwDdrAlignType::D0,
            1 => Usb3SwDdrAlignType::D1,
            2 => Usb3SwDdrAlignType::D2,
            3 => Usb3SwDdrAlignType::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "16-Byte align"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Usb3SwDdrAlignType::D0
    }
    #[doc = "32-Byte align"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Usb3SwDdrAlignType::D1
    }
    #[doc = "64-Byte align"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Usb3SwDdrAlignType::D2
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Usb3SwDdrAlignType::D3
    }
}
#[doc = "Field `USB3_SW_DDR_ALIGN_TYPE` writer - "]
pub type Usb3SwDdrAlignTypeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Usb3SwDdrAlignType>;
impl<'a, REG> Usb3SwDdrAlignTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-Byte align"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3SwDdrAlignType::D0)
    }
    #[doc = "32-Byte align"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3SwDdrAlignType::D1)
    }
    #[doc = "64-Byte align"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3SwDdrAlignType::D2)
    }
    #[doc = "128-Byte align"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3SwDdrAlignType::D3)
    }
}
#[doc = "Field `USB3_SW_RD_LATENCY_ID` reader - Axi read channel id for latency\n\nAXI_PERFormance test"]
pub type Usb3SwRdLatencyIdR = crate::FieldReader;
#[doc = "Field `USB3_SW_RD_LATENCY_ID` writer - Axi read channel id for latency\n\nAXI_PERFormance test"]
pub type Usb3SwRdLatencyIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb3RksocAxiPerfSel {
    #[doc = "0: usb3otg0"]
    B0 = 0,
    #[doc = "1: usb3otg1"]
    B1 = 1,
}
impl From<Usb3RksocAxiPerfSel> for bool {
    #[inline(always)]
    fn from(variant: Usb3RksocAxiPerfSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB3_RKSOC_AXI_PERF_SEL` reader - "]
pub type Usb3RksocAxiPerfSelR = crate::BitReader<Usb3RksocAxiPerfSel>;
impl Usb3RksocAxiPerfSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb3RksocAxiPerfSel {
        match self.bits {
            false => Usb3RksocAxiPerfSel::B0,
            true => Usb3RksocAxiPerfSel::B1,
        }
    }
    #[doc = "usb3otg0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usb3RksocAxiPerfSel::B0
    }
    #[doc = "usb3otg1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usb3RksocAxiPerfSel::B1
    }
}
#[doc = "Field `USB3_RKSOC_AXI_PERF_SEL` writer - "]
pub type Usb3RksocAxiPerfSelW<'a, REG> = crate::BitWriter<'a, REG, Usb3RksocAxiPerfSel>;
impl<'a, REG> Usb3RksocAxiPerfSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "usb3otg0"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3RksocAxiPerfSel::B0)
    }
    #[doc = "usb3otg1"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3RksocAxiPerfSel::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - axi_perf enable bit"]
    #[inline(always)]
    pub fn usb3_sw_axi_perf_work(&self) -> Usb3SwAxiPerfWorkR {
        Usb3SwAxiPerfWorkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fi\n\naxi_perf clear bit"]
    #[inline(always)]
    pub fn usb3_sw_axi_perf_clr(&self) -> Usb3SwAxiPerfClrR {
        Usb3SwAxiPerfClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - axi_perf counter type"]
    #[inline(always)]
    pub fn usb3_sw_axi_cnt_type(&self) -> Usb3SwAxiCntTypeR {
        Usb3SwAxiCntTypeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - axi_perf counter id control"]
    #[inline(always)]
    pub fn usb3_sw_ar_cnt_id_type(&self) -> Usb3SwArCntIdTypeR {
        Usb3SwArCntIdTypeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - axi_perf counter id control"]
    #[inline(always)]
    pub fn usb3_sw_aw_cnt_id_type(&self) -> Usb3SwAwCntIdTypeR {
        Usb3SwAwCntIdTypeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn usb3_sw_ddr_align_type(&self) -> Usb3SwDdrAlignTypeR {
        Usb3SwDdrAlignTypeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Axi read channel id for latency\n\nAXI_PERFormance test"]
    #[inline(always)]
    pub fn usb3_sw_rd_latency_id(&self) -> Usb3SwRdLatencyIdR {
        Usb3SwRdLatencyIdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn usb3_rksoc_axi_perf_sel(&self) -> Usb3RksocAxiPerfSelR {
        Usb3RksocAxiPerfSelR::new(((self.bits >> 15) & 1) != 0)
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
    pub fn usb3_sw_axi_perf_work(&mut self) -> Usb3SwAxiPerfWorkW<GrfUsb3PerfCon0Spec> {
        Usb3SwAxiPerfWorkW::new(self, 0)
    }
    #[doc = "Bit 1 - Fi\n\naxi_perf clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn usb3_sw_axi_perf_clr(&mut self) -> Usb3SwAxiPerfClrW<GrfUsb3PerfCon0Spec> {
        Usb3SwAxiPerfClrW::new(self, 1)
    }
    #[doc = "Bit 2 - axi_perf counter type"]
    #[inline(always)]
    #[must_use]
    pub fn usb3_sw_axi_cnt_type(&mut self) -> Usb3SwAxiCntTypeW<GrfUsb3PerfCon0Spec> {
        Usb3SwAxiCntTypeW::new(self, 2)
    }
    #[doc = "Bit 3 - axi_perf counter id control"]
    #[inline(always)]
    #[must_use]
    pub fn usb3_sw_ar_cnt_id_type(&mut self) -> Usb3SwArCntIdTypeW<GrfUsb3PerfCon0Spec> {
        Usb3SwArCntIdTypeW::new(self, 3)
    }
    #[doc = "Bit 4 - axi_perf counter id control"]
    #[inline(always)]
    #[must_use]
    pub fn usb3_sw_aw_cnt_id_type(&mut self) -> Usb3SwAwCntIdTypeW<GrfUsb3PerfCon0Spec> {
        Usb3SwAwCntIdTypeW::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn usb3_sw_ddr_align_type(&mut self) -> Usb3SwDdrAlignTypeW<GrfUsb3PerfCon0Spec> {
        Usb3SwDdrAlignTypeW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Axi read channel id for latency\n\nAXI_PERFormance test"]
    #[inline(always)]
    #[must_use]
    pub fn usb3_sw_rd_latency_id(&mut self) -> Usb3SwRdLatencyIdW<GrfUsb3PerfCon0Spec> {
        Usb3SwRdLatencyIdW::new(self, 8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn usb3_rksoc_axi_perf_sel(&mut self) -> Usb3RksocAxiPerfSelW<GrfUsb3PerfCon0Spec> {
        Usb3RksocAxiPerfSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfUsb3PerfCon0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "usb3 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3_perf_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3_perf_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfUsb3PerfCon0Spec;
impl crate::RegisterSpec for GrfUsb3PerfCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_usb3_perf_con0::R`](R) reader structure"]
impl crate::Readable for GrfUsb3PerfCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_usb3_perf_con0::W`](W) writer structure"]
impl crate::Writable for GrfUsb3PerfCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_USB3_PERF_CON0 to value 0"]
impl crate::Resettable for GrfUsb3PerfCon0Spec {
    const RESET_VALUE: u32 = 0;
}
