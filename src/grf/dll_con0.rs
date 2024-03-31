#[doc = "Register `DLL_CON0` reader"]
pub type R = crate::R<DllCon0Spec>;
#[doc = "Register `DLL_CON0` writer"]
pub type W = crate::W<DllCon0Spec>;
#[doc = "Field `PVTM_CORE_L_START` reader - pd_core_l PVT monitor start control"]
pub type PvtmCoreLStartR = crate::BitReader;
#[doc = "Field `PVTM_CORE_L_START` writer - pd_core_l PVT monitor start control"]
pub type PvtmCoreLStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "pd_core_l PVT monitor oscilator enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PvtmCoreLOscEn {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<PvtmCoreLOscEn> for bool {
    #[inline(always)]
    fn from(variant: PvtmCoreLOscEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVTM_CORE_L_OSC_EN` reader - pd_core_l PVT monitor oscilator enable"]
pub type PvtmCoreLOscEnR = crate::BitReader<PvtmCoreLOscEn>;
impl PvtmCoreLOscEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PvtmCoreLOscEn {
        match self.bits {
            true => PvtmCoreLOscEn::B1,
            false => PvtmCoreLOscEn::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PvtmCoreLOscEn::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PvtmCoreLOscEn::B0
    }
}
#[doc = "Field `PVTM_CORE_L_OSC_EN` writer - pd_core_l PVT monitor oscilator enable"]
pub type PvtmCoreLOscEnW<'a, REG> = crate::BitWriter<'a, REG, PvtmCoreLOscEn>;
impl<'a, REG> PvtmCoreLOscEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PvtmCoreLOscEn::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PvtmCoreLOscEn::B0)
    }
}
#[doc = "Field `PVTM_CORE_L_OSC_SEL` reader - pd_core_l PVT monitor oscilator select"]
pub type PvtmCoreLOscSelR = crate::FieldReader;
#[doc = "Field `PVTM_CORE_L_OSC_SEL` writer - pd_core_l PVT monitor oscilator select"]
pub type PvtmCoreLOscSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PVTM_CORE_B_START` reader - pd_core_b PVT monitor start control"]
pub type PvtmCoreBStartR = crate::BitReader;
#[doc = "Field `PVTM_CORE_B_START` writer - pd_core_b PVT monitor start control"]
pub type PvtmCoreBStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "pd_core_b PVT monitor oscilator enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PvtmCoreBOscEn {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<PvtmCoreBOscEn> for bool {
    #[inline(always)]
    fn from(variant: PvtmCoreBOscEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVTM_CORE_B_OSC_EN` reader - pd_core_b PVT monitor oscilator enable"]
pub type PvtmCoreBOscEnR = crate::BitReader<PvtmCoreBOscEn>;
impl PvtmCoreBOscEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PvtmCoreBOscEn {
        match self.bits {
            true => PvtmCoreBOscEn::B1,
            false => PvtmCoreBOscEn::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PvtmCoreBOscEn::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PvtmCoreBOscEn::B0
    }
}
#[doc = "Field `PVTM_CORE_B_OSC_EN` writer - pd_core_b PVT monitor oscilator enable"]
pub type PvtmCoreBOscEnW<'a, REG> = crate::BitWriter<'a, REG, PvtmCoreBOscEn>;
impl<'a, REG> PvtmCoreBOscEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PvtmCoreBOscEn::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PvtmCoreBOscEn::B0)
    }
}
#[doc = "Field `PVTM_CORE_B_OSC_SEL` reader - pd_core_l PVT monitor oscilator select\n\npvtm_core_b_osc_sel\\[1:0\\]"]
pub type PvtmCoreBOscSelR = crate::FieldReader;
#[doc = "Field `PVTM_CORE_B_OSC_SEL` writer - pd_core_l PVT monitor oscilator select\n\npvtm_core_b_osc_sel\\[1:0\\]"]
pub type PvtmCoreBOscSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PVTM_DDR_START` reader - ddr PVT monitor start control"]
pub type PvtmDdrStartR = crate::BitReader;
#[doc = "Field `PVTM_DDR_START` writer - ddr PVT monitor start control"]
pub type PvtmDdrStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ddr PVT monitor oscilator enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PvtmDdrOscEn {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<PvtmDdrOscEn> for bool {
    #[inline(always)]
    fn from(variant: PvtmDdrOscEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVTM_DDR_OSC_EN` reader - ddr PVT monitor oscilator enable"]
pub type PvtmDdrOscEnR = crate::BitReader<PvtmDdrOscEn>;
impl PvtmDdrOscEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PvtmDdrOscEn {
        match self.bits {
            true => PvtmDdrOscEn::B1,
            false => PvtmDdrOscEn::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PvtmDdrOscEn::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PvtmDdrOscEn::B0
    }
}
#[doc = "Field `PVTM_DDR_OSC_EN` writer - ddr PVT monitor oscilator enable"]
pub type PvtmDdrOscEnW<'a, REG> = crate::BitWriter<'a, REG, PvtmDdrOscEn>;
impl<'a, REG> PvtmDdrOscEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PvtmDdrOscEn::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PvtmDdrOscEn::B0)
    }
}
#[doc = "Field `PVTM_DDR_OSC_RING_SEL` reader - ddr PVT monitor oscilator ring select"]
pub type PvtmDdrOscRingSelR = crate::FieldReader;
#[doc = "Field `PVTM_DDR_OSC_RING_SEL` writer - ddr PVT monitor oscilator ring select"]
pub type PvtmDdrOscRingSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PVTM_GPU_START` reader - gpu PVT monitor start control"]
pub type PvtmGpuStartR = crate::BitReader;
#[doc = "Field `PVTM_GPU_START` writer - gpu PVT monitor start control"]
pub type PvtmGpuStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "gpu PVT monitor oscilator enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PvtmGpuOscEn {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<PvtmGpuOscEn> for bool {
    #[inline(always)]
    fn from(variant: PvtmGpuOscEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVTM_GPU_OSC_EN` reader - gpu PVT monitor oscilator enable"]
pub type PvtmGpuOscEnR = crate::BitReader<PvtmGpuOscEn>;
impl PvtmGpuOscEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PvtmGpuOscEn {
        match self.bits {
            true => PvtmGpuOscEn::B1,
            false => PvtmGpuOscEn::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PvtmGpuOscEn::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PvtmGpuOscEn::B0
    }
}
#[doc = "Field `PVTM_GPU_OSC_EN` writer - gpu PVT monitor oscilator enable"]
pub type PvtmGpuOscEnW<'a, REG> = crate::BitWriter<'a, REG, PvtmGpuOscEn>;
impl<'a, REG> PvtmGpuOscEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PvtmGpuOscEn::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PvtmGpuOscEn::B0)
    }
}
#[doc = "Field `PVTM_GPU_OSC_RING_SEL` reader - gpu PVT monitor oscilator ring select"]
pub type PvtmGpuOscRingSelR = crate::FieldReader;
#[doc = "Field `PVTM_GPU_OSC_RING_SEL` writer - gpu PVT monitor oscilator ring select"]
pub type PvtmGpuOscRingSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - pd_core_l PVT monitor start control"]
    #[inline(always)]
    pub fn pvtm_core_l_start(&self) -> PvtmCoreLStartR {
        PvtmCoreLStartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pd_core_l PVT monitor oscilator enable"]
    #[inline(always)]
    pub fn pvtm_core_l_osc_en(&self) -> PvtmCoreLOscEnR {
        PvtmCoreLOscEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - pd_core_l PVT monitor oscilator select"]
    #[inline(always)]
    pub fn pvtm_core_l_osc_sel(&self) -> PvtmCoreLOscSelR {
        PvtmCoreLOscSelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - pd_core_b PVT monitor start control"]
    #[inline(always)]
    pub fn pvtm_core_b_start(&self) -> PvtmCoreBStartR {
        PvtmCoreBStartR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - pd_core_b PVT monitor oscilator enable"]
    #[inline(always)]
    pub fn pvtm_core_b_osc_en(&self) -> PvtmCoreBOscEnR {
        PvtmCoreBOscEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - pd_core_l PVT monitor oscilator select\n\npvtm_core_b_osc_sel\\[1:0\\]"]
    #[inline(always)]
    pub fn pvtm_core_b_osc_sel(&self) -> PvtmCoreBOscSelR {
        PvtmCoreBOscSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - ddr PVT monitor start control"]
    #[inline(always)]
    pub fn pvtm_ddr_start(&self) -> PvtmDdrStartR {
        PvtmDdrStartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ddr PVT monitor oscilator enable"]
    #[inline(always)]
    pub fn pvtm_ddr_osc_en(&self) -> PvtmDdrOscEnR {
        PvtmDdrOscEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - ddr PVT monitor oscilator ring select"]
    #[inline(always)]
    pub fn pvtm_ddr_osc_ring_sel(&self) -> PvtmDdrOscRingSelR {
        PvtmDdrOscRingSelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - gpu PVT monitor start control"]
    #[inline(always)]
    pub fn pvtm_gpu_start(&self) -> PvtmGpuStartR {
        PvtmGpuStartR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - gpu PVT monitor oscilator enable"]
    #[inline(always)]
    pub fn pvtm_gpu_osc_en(&self) -> PvtmGpuOscEnR {
        PvtmGpuOscEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - gpu PVT monitor oscilator ring select"]
    #[inline(always)]
    pub fn pvtm_gpu_osc_ring_sel(&self) -> PvtmGpuOscRingSelR {
        PvtmGpuOscRingSelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - pd_core_l PVT monitor start control"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_core_l_start(&mut self) -> PvtmCoreLStartW<DllCon0Spec> {
        PvtmCoreLStartW::new(self, 0)
    }
    #[doc = "Bit 1 - pd_core_l PVT monitor oscilator enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_core_l_osc_en(&mut self) -> PvtmCoreLOscEnW<DllCon0Spec> {
        PvtmCoreLOscEnW::new(self, 1)
    }
    #[doc = "Bits 2:3 - pd_core_l PVT monitor oscilator select"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_core_l_osc_sel(&mut self) -> PvtmCoreLOscSelW<DllCon0Spec> {
        PvtmCoreLOscSelW::new(self, 2)
    }
    #[doc = "Bit 4 - pd_core_b PVT monitor start control"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_core_b_start(&mut self) -> PvtmCoreBStartW<DllCon0Spec> {
        PvtmCoreBStartW::new(self, 4)
    }
    #[doc = "Bit 5 - pd_core_b PVT monitor oscilator enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_core_b_osc_en(&mut self) -> PvtmCoreBOscEnW<DllCon0Spec> {
        PvtmCoreBOscEnW::new(self, 5)
    }
    #[doc = "Bits 6:7 - pd_core_l PVT monitor oscilator select\n\npvtm_core_b_osc_sel\\[1:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_core_b_osc_sel(&mut self) -> PvtmCoreBOscSelW<DllCon0Spec> {
        PvtmCoreBOscSelW::new(self, 6)
    }
    #[doc = "Bit 8 - ddr PVT monitor start control"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_ddr_start(&mut self) -> PvtmDdrStartW<DllCon0Spec> {
        PvtmDdrStartW::new(self, 8)
    }
    #[doc = "Bit 9 - ddr PVT monitor oscilator enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_ddr_osc_en(&mut self) -> PvtmDdrOscEnW<DllCon0Spec> {
        PvtmDdrOscEnW::new(self, 9)
    }
    #[doc = "Bits 10:11 - ddr PVT monitor oscilator ring select"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_ddr_osc_ring_sel(&mut self) -> PvtmDdrOscRingSelW<DllCon0Spec> {
        PvtmDdrOscRingSelW::new(self, 10)
    }
    #[doc = "Bit 12 - gpu PVT monitor start control"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_gpu_start(&mut self) -> PvtmGpuStartW<DllCon0Spec> {
        PvtmGpuStartW::new(self, 12)
    }
    #[doc = "Bit 13 - gpu PVT monitor oscilator enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_gpu_osc_en(&mut self) -> PvtmGpuOscEnW<DllCon0Spec> {
        PvtmGpuOscEnW::new(self, 13)
    }
    #[doc = "Bits 14:15 - gpu PVT monitor oscilator ring select"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_gpu_osc_ring_sel(&mut self) -> PvtmGpuOscRingSelW<DllCon0Spec> {
        PvtmGpuOscRingSelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<DllCon0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "pvtm control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DllCon0Spec;
impl crate::RegisterSpec for DllCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll_con0::R`](R) reader structure"]
impl crate::Readable for DllCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`dll_con0::W`](W) writer structure"]
impl crate::Writable for DllCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLL_CON0 to value 0"]
impl crate::Resettable for DllCon0Spec {
    const RESET_VALUE: u32 = 0;
}
